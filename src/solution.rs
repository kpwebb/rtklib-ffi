//! Solution file I/O.
//!
//! Wraps RTKLIB's `sol_t` / `solbuf_t` types. [`read_sol`] wraps `readsol`,
//! [`read_solt`] wraps `readsolt`, and [`write_sol`] wraps `outsolheads` and
//! `outsols` from `solution.c`.

use crate::{ppk::SolOpt, util::CStringArray, GpsTime, Llh, SolStatus};
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{Error as IoError, Write},
};
use thiserror::Error;

/// Coordinate type stored in a [`Solution`].
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum CoordType {
    /// ECEF X/Y/Z coordinates.
    Ecef = 0,
    /// ENU baseline coordinates.
    EnuBaseline = 1,
}

/// Error from solution I/O.
#[derive(Debug, Error)]
pub enum SolError {
    /// A file path contained an interior null byte.
    #[error("path contains null byte: {0:?}")]
    NulByte(OsString),
    /// Reading the solution file failed or returned no data.
    #[error("read failed")]
    ReadFailed,
    /// Unrecognised coordinate type field.
    #[error("unrecognized coordinate type: {0}")]
    InvalidCoordType(u8),
    /// I/O error.
    #[error(transparent)]
    Io(#[from] IoError),
}

/// A single solution record, wrapping `sol_t`.
#[repr(transparent)]
pub struct Solution(ffi::sol_t);

impl Solution {
    /// Time of the solution.
    pub fn time(&self) -> GpsTime {
        GpsTime(self.0.time)
    }

    /// Time of the triggering event.
    pub fn event_time(&self) -> GpsTime {
        GpsTime(self.0.eventime)
    }

    /// Position in ECEF coordinates (meters).
    pub fn position_ecef(&self) -> &[f64] {
        &self.0.rr[..3]
    }

    /// Velocity in ECEF or ENU (m/s).
    pub fn velocity(&self) -> &[f64] {
        &self.0.rr[3..]
    }

    /// Position variance/covariance (m²) as a symmetric 3×3 covariance matrix.
    pub fn position_variance(&self) -> [f32; 9] {
        let q = self.0.qr;
        [q[0], q[3], q[5], q[3], q[1], q[4], q[5], q[4], q[2]]
    }

    /// Raw position variance/covariance (m²): `{c_xx,c_yy,c_zz,c_xy,c_yz,c_zx}` or
    /// `{c_ee,c_nn,c_uu,c_en,c_nu,c_ue}`.
    pub fn position_variance_raw(&self) -> [f32; 6] {
        self.0.qr
    }

    /// Velocity variance/covariance (m²/s²) as a symmetric 3×3 covariance matrix.
    pub fn velocity_variance(&self) -> [f32; 9] {
        let q = self.0.qv;
        [q[0], q[3], q[5], q[3], q[1], q[4], q[5], q[4], q[2]]
    }

    /// Raw velocity variance/covariance (m²/s²): `{c_xx,c_yy,c_zz,c_xy,c_yz,c_zx}` or
    /// `{c_ee,c_nn,c_uu,c_en,c_nu,c_ue}`.
    pub fn velocity_variance_raw(&self) -> [f32; 6] {
        self.0.qv
    }

    /// Receiver clock bias to each time system (s).
    pub fn clock_bias(&self) -> &[f64] {
        &self.0.dtr
    }

    /// Coordinate type of the position/velocity fields.
    pub fn coord_type(&self) -> CoordType {
        // Validated at SolBuf construction; an invalid value is an unreachable bug.
        CoordType::try_from(self.0.type_).unwrap()
    }

    /// Solution quality status.
    pub fn status(&self) -> SolStatus {
        SolStatus::try_from(self.0.stat as u32).unwrap_or(SolStatus::None)
    }

    /// Number of valid satellites used.
    pub fn num_satellites(&self) -> u8 {
        self.0.ns
    }

    /// Age of differential correction (s).
    pub fn age(&self) -> f32 {
        self.0.age
    }

    /// AR ratio factor for ambiguity validation.
    pub fn ratio(&self) -> f32 {
        self.0.ratio
    }

    /// Previous initial AR ratio factor.
    pub fn prev_ratio1(&self) -> f32 {
        self.0.prev_ratio1
    }

    /// Previous final AR ratio factor.
    pub fn prev_ratio2(&self) -> f32 {
        self.0.prev_ratio2
    }

    /// AR ratio threshold.
    pub fn threshold(&self) -> f32 {
        self.0.thres
    }

    /// Reference station ID.
    pub fn ref_station_id(&self) -> i32 {
        self.0.refstationid
    }

    /// Position converted to geodetic coordinates (lat/lon in radians, height in meters).
    pub fn position_llh(&self) -> Llh {
        let mut pos = Llh::default();
        unsafe { ffi::ecef2pos(self.0.rr.as_ptr(), &mut pos as *mut Llh as *mut f64) };
        pos
    }
}

/// Buffer of solution records read from one or more `.pos` files, wrapping `solbuf_t`.
pub struct SolBuf(ffi::solbuf_t);

impl Drop for SolBuf {
    fn drop(&mut self) {
        unsafe { ffi::freesolbuf(&mut self.0) };
    }
}

impl SolBuf {
    /// Number of solutions in the buffer.
    pub fn len(&self) -> usize {
        self.0.n as usize
    }

    /// Whether the buffer contains no solutions.
    pub fn is_empty(&self) -> bool {
        self.0.n == 0
    }

    /// Reference station position in ECEF coordinates (meters).
    pub fn ref_position_ecef(&self) -> [f64; 3] {
        self.0.rb
    }

    /// All solutions as a slice.
    pub fn as_slice(&self) -> &[Solution] {
        if self.0.data.is_null() || self.0.n == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.0.data as *const Solution, self.0.n as usize) }
    }
}

fn validate_coord_types(buf: SolBuf) -> Result<SolBuf, SolError> {
    if let Some(sol) = buf
        .as_slice()
        .iter()
        .find(|sol| CoordType::try_from(sol.0.type_).is_err())
    {
        return Err(SolError::InvalidCoordType(sol.0.type_));
    }
    Ok(buf)
}

impl From<&OsStr> for SolError {
    fn from(s: &OsStr) -> Self {
        Self::NulByte(s.to_owned())
    }
}

/// Read solution records from one or more `.pos` files.
pub fn read_sol<T: AsRef<OsStr>>(paths: &[T]) -> Result<SolBuf, SolError> {
    let mut arr = CStringArray::try_new(paths)?;
    let mut buf = unsafe { std::mem::zeroed::<ffi::solbuf_t>() };
    let ret = unsafe { ffi::readsol(arr.as_mut_ptr(), arr.len() as i32, &mut buf) };
    if ret == 0 {
        return Err(SolError::ReadFailed);
    }
    validate_coord_types(SolBuf(buf))
}

/// Read solution records within a time window from one or more `.pos` files.
pub fn read_solt<T: AsRef<OsStr>>(
    paths: &[T],
    ts: GpsTime,
    te: GpsTime,
    tint: f64,
    qflag: i32,
    mean: bool,
) -> Result<SolBuf, SolError> {
    let mut arr = CStringArray::try_new(paths)?;
    let mut buf = unsafe { std::mem::zeroed::<ffi::solbuf_t>() };
    let ret = unsafe {
        ffi::readsolt(
            arr.as_mut_ptr(),
            arr.len() as i32,
            ts.0,
            te.0,
            tint,
            qflag,
            mean as i32,
            &mut buf,
        )
    };
    if ret == 0 {
        return Err(SolError::ReadFailed);
    }
    validate_coord_types(SolBuf(buf))
}

/// Write solution records to a file using the given output options.
pub fn write_sol(path: impl AsRef<OsStr>, buf: &SolBuf, opt: &SolOpt) -> Result<(), SolError> {
    let mut file = File::create(path.as_ref())?;
    let mut scratch = [0u8; 513];
    let n = unsafe { ffi::outsolheads(scratch.as_mut_ptr(), opt.as_ffi()) };
    file.write_all(&scratch[..n as usize])?;
    let rb = buf.0.rb.as_ptr();
    for sol in buf.as_slice() {
        let n = unsafe { ffi::outsols(scratch.as_mut_ptr(), &sol.0, rb, opt.as_ffi()) };
        file.write_all(&scratch[..n as usize])?;
    }
    Ok(())
}
