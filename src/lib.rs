//! Safe Rust bindings for [RTKLIB](https://github.com/rtklibexplorer/RTKLIB).
//!
//! # Features
//!
//! Enable functionality via Cargo features:
//!
//! - **`conv`**: File format conversion. Implies `receivers` - `convrnx` calls
//!   `init_raw` at link time regardless of input format, which requires all
//!   receiver format files to be present.
//! - **`gis`**: GIS data support.
//! - **`hifitime`**: Conversions between [`GpsTime`] and [`hifitime::Epoch`].
//! - **`net`**: Network streaming.
//! - **`ppk`**: Post-processed kinematic positioning via [`postpos()`].
//! - **`receivers`**: All supported hardware receiver decoders: BINEX, Hemisphere
//!   Crescent, Javad/Topcon, NovAtel OEM, NVS, Septentrio SBF, SkyTraq, Swift
//!   Navigation SBP, Trimble RT17, u-blox UBX, and Unicore. ComNav and Tersus are
//!   not included; their source files use APIs removed in the current upstream.
//! - **`rtcm`**: RTCM3 message decoding via [`RtcmDecoder`].
//! - **`strum`**: Adds [`std::fmt::Display`] support for enums via the optional [`strum`](https://docs.rs/strum) dependency.
//! - **`tle`**: TLE satellite tracking.

#[cfg(feature = "hifitime")]
use hifitime::Epoch;
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;

#[cfg(feature = "ppk")]
pub mod ppk;
#[cfg(feature = "ppk")]
pub use ppk::*;

#[cfg(feature = "ppk")]
pub mod solution;
#[cfg(feature = "ppk")]
pub use solution::*;

pub mod meas;
pub use meas::*;

mod util;

/// Error returned when a decoder fails to initialize.
#[derive(Debug, thiserror::Error)]
#[error("failed to initialize decoder")]
pub struct DecoderInitError;

#[cfg(feature = "receivers")]
pub mod receiver;
#[cfg(feature = "receivers")]
pub use receiver::*;

#[cfg(feature = "conv")]
pub mod conv;
#[cfg(feature = "conv")]
pub use conv::*;

#[cfg(feature = "rtcm")]
pub mod rtcm;
#[cfg(feature = "rtcm")]
pub use rtcm::*;

bitflags::bitflags! {
    /// GNSS navigation system bitmask.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct NavSys: u32 {
        /// Global Positioning System. From SYS_GPS.
        const Gps = ffi::SYS_GPS;
        /// Satellite-Based Augmentation System. From SYS_SBS.
        const Sbs = ffi::SYS_SBS;
        /// GLONASS. From SYS_GLO.
        const Glo = ffi::SYS_GLO;
        /// Galileo. From SYS_GAL.
        const Gal = ffi::SYS_GAL;
        /// Quasi-Zenith Satellite System. From SYS_QZS.
        const Qzs = ffi::SYS_QZS;
        /// BeiDou Navigation Satellite System. From SYS_CMP.
        const Cmp = ffi::SYS_CMP;
        /// Indian Regional Navigation Satellite System / NavIC. From SYS_IRN.
        const Irn = ffi::SYS_IRN;
        /// Low Earth Orbit satellites. From SYS_LEO.
        const Leo = ffi::SYS_LEO;
        /// All navigation systems. From SYS_ALL.
        const All = ffi::SYS_ALL;
    }
}

/// Convert a constellation and PRN/slot number to an internal satellite number.
///
/// Returns `None` if the PRN is out of range for the constellation.
pub fn satno(sys: NavSys, prn: i32) -> Option<u8> {
    let n = unsafe { ffi::satno(sys.bits() as i32, prn) };
    if n == 0 {
        None
    } else {
        Some(n as u8)
    }
}

/// Geodetic position: latitude and longitude in radians, height in meters.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Llh {
    pub lat_rad: f64,
    pub lon_rad: f64,
    pub height_m: f64,
}

impl Llh {
    /// Construct from latitude and longitude in degrees, height in meters.
    pub fn from_deg(lat_deg: f64, lon_deg: f64, height_m: f64) -> Self {
        Self {
            lat_rad: lat_deg.to_radians(),
            lon_rad: lon_deg.to_radians(),
            height_m,
        }
    }

    /// Convert to ECEF coordinates (meters).
    pub fn to_ecef(&self) -> [f64; 3] {
        let mut r = [0.0f64; 3];
        unsafe { ffi::pos2ecef(self as *const Llh as *const f64, r.as_mut_ptr()) };
        r
    }
}

impl From<[f64; 3]> for Llh {
    fn from(a: [f64; 3]) -> Self {
        Self {
            lat_rad: a[0],
            lon_rad: a[1],
            height_m: a[2],
        }
    }
}

impl From<Llh> for [f64; 3] {
    fn from(l: Llh) -> Self {
        [l.lat_rad, l.lon_rad, l.height_m]
    }
}

/// Convert ECEF position (meters) to geodetic coordinates.
///
/// Returns latitude and longitude in radians, height in meters.
pub fn ecef2pos(ecef: [f64; 3]) -> Llh {
    let mut pos = Llh::default();
    unsafe { ffi::ecef2pos(ecef.as_ptr(), &mut pos as *mut Llh as *mut f64) };
    pos
}

/// GPS time: seconds since Unix epoch plus sub-second fraction.
#[derive(Clone, Copy, Debug)]
pub struct GpsTime(pub(crate) ffi::gtime_t);

impl GpsTime {
    /// Time zero (Unix epoch).
    pub fn zero() -> Self {
        Self(ffi::gtime_t { time: 0, sec: 0.0 })
    }

    /// Construct from Unix seconds and a fractional part in [0, 1).
    pub fn from_unix(secs: i64, frac: f64) -> Self {
        Self(ffi::gtime_t {
            time: secs,
            sec: frac,
        })
    }

    /// Construct from GPS week number and time-of-week (seconds).
    pub fn from_gps(week: i32, tow: f64) -> Self {
        Self(unsafe { ffi::gpst2time(week, tow) })
    }

    /// Unix seconds component (integer part).
    pub fn unix_secs(&self) -> i64 {
        self.0.time
    }

    /// Sub-second fractional part.
    pub fn frac_secs(&self) -> f64 {
        self.0.sec
    }
}

#[cfg(feature = "hifitime")]
impl From<Epoch> for GpsTime {
    fn from(e: Epoch) -> Self {
        let unix_s = e.to_unix_seconds();
        let secs = unix_s.floor() as i64;
        let frac = unix_s - secs as f64;
        GpsTime(ffi::gtime_t {
            time: secs,
            sec: frac,
        })
    }
}

#[cfg(feature = "hifitime")]
impl From<GpsTime> for Epoch {
    fn from(t: GpsTime) -> Self {
        Epoch::from_unix_seconds(t.0.time as f64 + t.0.sec)
    }
}

/// Solution quality status.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[cfg_attr(feature = "strum", strum(serialize_all = "SCREAMING_SNAKE_CASE"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum SolStatus {
    /// No solution available. From SOLQ_NONE.
    None = ffi::SOLQ_NONE,
    /// Integer ambiguity resolved. From SOLQ_FIX.
    Fix = ffi::SOLQ_FIX,
    /// Ambiguity not resolved, float solution. From SOLQ_FLOAT.
    Float = ffi::SOLQ_FLOAT,
    /// SBAS corrected solution. From SOLQ_SBAS.
    Sbas = ffi::SOLQ_SBAS,
    /// DGPS/DGNSS corrected solution. From SOLQ_DGPS.
    Dgps = ffi::SOLQ_DGPS,
    /// Single point solution. From SOLQ_SINGLE.
    Single = ffi::SOLQ_SINGLE,
    /// Precise Point Positioning converged solution. From SOLQ_PPP.
    Ppp = ffi::SOLQ_PPP,
    /// Dead reckoning solution. From SOLQ_DR.
    DeadReckoning = ffi::SOLQ_DR,
}
