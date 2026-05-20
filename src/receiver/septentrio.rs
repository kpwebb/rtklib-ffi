//! Septentrio SBF receiver decoder.
//!
//! ```no_run
//! use rtklib_ffi::receiver::{DecodeStatus, SbfDecoder};
//!
//! let mut decoder = SbfDecoder::try_new().unwrap();
//! # let sbf_bytes: Vec<u8> = vec![];
//!
//! for &byte in &sbf_bytes {
//!     let Some(status) = decoder.decode(byte) else { continue; };
//!     match status {
//!         DecodeStatus::Observation => {
//!             let obs = decoder.observations();
//!             // process observations...
//!         }
//!         DecodeStatus::Ephemeris => {
//!             let sat = decoder.ephemeris_sat();
//!             // handle ephemeris update for satellite sat...
//!         }
//!         _ => {}
//!     }
//! }
//! ```

use super::{DecodeStatus, RawReceiver};
use crate::DecoderInitError;
use rtklib_sys::rtklib as ffi;
use std::{convert::TryFrom, ops::Deref};

/// Septentrio SBF receiver data decoder.
pub struct SbfDecoder(RawReceiver);

impl SbfDecoder {
    /// Create a new SBF decoder.
    ///
    /// Returns `Err` if RTKLIB cannot allocate internal buffers.
    pub fn try_new() -> Result<Self, DecoderInitError> {
        // init_raw sets raw->format = STRFMT_SEPT and allocates the obs/nav
        // buffers. init_sbf checks raw->format and returns 0 if it is not
        // set, so init_sbf alone cannot be used here.
        RawReceiver::init(ffi::STRFMT_SEPT as i32).map(Self)
    }

    /// Feed one byte into the SBF decoder.
    ///
    /// Returns `None` if the byte did not complete a recognized message.
    pub fn decode(&mut self, byte: u8) -> Option<DecodeStatus> {
        let ret = unsafe { ffi::input_sbf(self.0.0.as_mut(), byte) };
        DecodeStatus::try_from(ret).ok()
    }
}

impl Deref for SbfDecoder {
    type Target = RawReceiver;

    fn deref(&self) -> &RawReceiver { &self.0 }
}
