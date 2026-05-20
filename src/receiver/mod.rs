//! Hardware receiver data decoding.

use crate::{
    meas::{Nav, ObsData},
    DecoderInitError,
};
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;
use std::{
    alloc::{alloc_zeroed, handle_alloc_error, Layout},
    slice::from_raw_parts,
};

pub mod septentrio;
pub use septentrio::*;

/// Shared raw receiver state wrapping the RTKLIB `raw_t` struct.
pub struct RawReceiver(pub(in crate::receiver) Box<ffi::raw_t>);

impl RawReceiver {
    /// Allocate and initialize a `raw_t` with the given format code.
    pub(crate) fn init(format: i32) -> Result<Self, DecoderInitError> {
        unsafe {
            // raw_t is ~673KB, too large for the stack.
            let layout = Layout::new::<ffi::raw_t>();
            let ptr = alloc_zeroed(layout) as *mut ffi::raw_t;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            let mut raw = Box::from_raw(ptr);
            // init_raw sets raw->format and allocates the obs/nav buffers.
            if ffi::init_raw(raw.as_mut(), format) == 0 {
                return Err(DecoderInitError);
            }
            Ok(Self(raw))
        }
    }

    /// Number of observation records in the current message.
    pub fn observation_count(&self) -> usize {
        self.0.obs.n as usize
    }

    /// Observation data from the last decoded observation message.
    pub fn observations(&self) -> &[ObsData] {
        let n = self.0.obs.n as usize;
        if n == 0 || self.0.obs.data.is_null() {
            return &[];
        }
        unsafe { from_raw_parts(self.0.obs.data as *const ObsData, n) }
    }

    /// Navigation data updated as ephemeris messages arrive.
    pub fn nav(&self) -> &Nav {
        unsafe { &*(&self.0.nav as *const ffi::nav_t as *const Nav) }
    }

    /// Satellite number of the most recently decoded ephemeris.
    pub fn ephemeris_sat(&self) -> i32 {
        self.0.ephsat
    }
}

impl Drop for RawReceiver {
    fn drop(&mut self) {
        // free_raw frees the obs/nav buffers and calls the format-specific free.
        unsafe { ffi::free_raw(self.0.as_mut()); }
    }
}

/// Outcome of feeding a byte into a receiver decoder.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum DecodeStatus {
    /// Incomplete frame; feed more bytes.
    Incomplete = 0,
    /// Observation data decoded.
    Observation = 1,
    /// Ephemeris decoded.
    Ephemeris = 2,
    /// SBAS corrections decoded.
    SbasCorrections = 3,
    /// Station position/antenna parameters decoded.
    StationInfo = 5,
    /// Ionosphere/UTC parameters decoded.
    IonOrUtc = 9,
}
