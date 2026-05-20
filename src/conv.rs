//! RINEX file format conversion.
//!
//! ```no_run
//! use rtklib_ffi::conv::{RnxOpt, RnxOutputFiles, StreamFmt, convrnx};
//!
//! let mut opt = RnxOpt::default();
//! let ofiles = RnxOutputFiles::new("out.obs").with_nav("out.nav");
//! convrnx(StreamFmt::Rinex, &mut opt, "input.rnx", &ofiles).unwrap();
//! ```

use crate::{
    util::{copy_osstr, CStringArray},
    GpsTime, NavSys,
};
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;
use std::ffi::{OsStr, OsString};
use thiserror::Error;

/// Input stream format for [`convrnx`].
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum StreamFmt {
    /// RTCM 2.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_RTCM2"))]
    Rtcm2 = ffi::STRFMT_RTCM2,
    /// RTCM 3.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_RTCM3"))]
    Rtcm3 = ffi::STRFMT_RTCM3,
    /// NovAtel OEM4/6/7.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_OEM4"))]
    Oem4 = ffi::STRFMT_OEM4,
    /// u-blox UBX.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_UBX"))]
    Ubx = ffi::STRFMT_UBX,
    /// Swift Navigation SBP.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_SBP"))]
    Sbp = ffi::STRFMT_SBP,
    /// Hemisphere Crescent.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_CRES"))]
    Crescent = ffi::STRFMT_CRES,
    /// SkyTraq.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_STQ"))]
    SkyTraq = ffi::STRFMT_STQ,
    /// Javad/Topcon GRIL/GREIS.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_JAVAD"))]
    Javad = ffi::STRFMT_JAVAD,
    /// NVS NVC08C.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_NVS"))]
    Nvs = ffi::STRFMT_NVS,
    /// BINEX.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_BINEX"))]
    Binex = ffi::STRFMT_BINEX,
    /// Trimble RT17.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_RT17"))]
    Rt17 = ffi::STRFMT_RT17,
    /// Septentrio SBF.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_SEPT"))]
    Sbf = ffi::STRFMT_SEPT,
    /// Unicore.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_UNICORE"))]
    Unicore = ffi::STRFMT_UNICORE,
    /// RINEX observation or navigation file.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_RINEX"))]
    Rinex = ffi::STRFMT_RINEX,
    /// SP3 precise ephemeris.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_SP3"))]
    Sp3 = ffi::STRFMT_SP3,
    /// RINEX clock file.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_RNXCLK"))]
    RinexClk = ffi::STRFMT_RNXCLK,
    /// SBAS log.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_SBAS"))]
    Sbas = ffi::STRFMT_SBAS,
    /// NMEA 0183.
    #[cfg_attr(feature = "strum", strum(to_string = "STRFMT_NMEA"))]
    Nmea = ffi::STRFMT_NMEA,
}

bitflags::bitflags! {
    /// Observation types to include in the output RINEX.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ObsType: u32 {
        /// Pseudorange.
        const Pr = ffi::OBSTYPE_PR;
        /// Carrier phase.
        const Cp = ffi::OBSTYPE_CP;
        /// Doppler frequency.
        const Dop = ffi::OBSTYPE_DOP;
        /// Signal-to-noise ratio.
        const Snr = ffi::OBSTYPE_SNR;
        /// All observation types.
        const All = ffi::OBSTYPE_ALL;
    }
}

bitflags::bitflags! {
    /// Frequency bands to include in the output RINEX.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FreqType: u32 {
        /// L1/G1/E1/B1.
        const L1 = ffi::FREQTYPE_L1;
        /// L2/G2/E5b/B2.
        const L2 = ffi::FREQTYPE_L2;
        /// L5/G3/E5a/B2a.
        const L3 = ffi::FREQTYPE_L3;
        /// L6/E6/B3.
        const L4 = ffi::FREQTYPE_L4;
        /// E5ab/B1C/B1A.
        const L5 = ffi::FREQTYPE_L5;
        /// B2ab.
        const L6 = ffi::FREQTYPE_L6;
        /// All frequency bands.
        const All = ffi::FREQTYPE_ALL;
    }
}

/// RINEX format version.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum RinexVersion {
    /// RINEX 2.10.
    V210 = 210,
    /// RINEX 2.11.
    V211 = 211,
    /// RINEX 2.12.
    V212 = 212,
    /// RINEX 3.03.
    V303 = 303,
    /// RINEX 3.04.
    V304 = 304,
    /// RINEX 3.05.
    V305 = 305,
}

/// RINEX conversion options.
#[derive(Clone, Copy)]
pub struct RnxOpt(ffi::rnxopt_t);

impl Default for RnxOpt {
    /// Creates options defaulting to RINEX 3.03, all navigation systems,
    /// all observation types, all frequency bands, and no time window.
    fn default() -> Self {
        let mut inner = unsafe { std::mem::zeroed::<ffi::rnxopt_t>() };
        inner.rnxver = RinexVersion::V303 as i32;
        inner.navsys = ffi::SYS_ALL as i32;
        inner.obstype = ffi::OBSTYPE_ALL as i32;
        inner.freqtype = ffi::FREQTYPE_ALL as i32;
        Self(inner)
    }
}

impl RnxOpt {
    /// Set the RINEX output version.
    pub fn with_version(mut self, ver: RinexVersion) -> Self {
        self.0.rnxver = ver as i32;
        self
    }

    /// The RINEX output version. Returns `None` if the stored value is not a
    /// recognized version.
    pub fn version(&self) -> Option<RinexVersion> {
        RinexVersion::try_from(self.0.rnxver).ok()
    }

    /// Set the navigation systems to include in the output.
    pub fn with_navsys(mut self, sys: NavSys) -> Self {
        self.0.navsys = sys.bits() as i32;
        self
    }

    /// The navigation systems included in the output.
    pub fn navsys(&self) -> NavSys {
        NavSys::from_bits_truncate(self.0.navsys as u32)
    }

    /// Set the observation types to include in the output.
    pub fn with_obstype(mut self, obs: ObsType) -> Self {
        self.0.obstype = obs.bits() as i32;
        self
    }

    /// The observation types included in the output.
    pub fn obstype(&self) -> ObsType {
        ObsType::from_bits_truncate(self.0.obstype as u32)
    }

    /// Set the frequency bands to include in the output.
    pub fn with_freqtype(mut self, freq: FreqType) -> Self {
        self.0.freqtype = freq.bits() as i32;
        self
    }

    /// The frequency bands included in the output.
    pub fn freqtype(&self) -> FreqType {
        FreqType::from_bits_truncate(self.0.freqtype as u32)
    }

    /// Set the output sampling interval in seconds. Zero outputs all epochs.
    pub fn with_interval(mut self, tint: f64) -> Self {
        self.0.tint = tint;
        self
    }

    /// The output sampling interval in seconds.
    pub fn interval(&self) -> f64 {
        self.0.tint
    }

    /// Set the time tolerance for matching epochs in seconds.
    pub fn with_time_tolerance(mut self, ttol: f64) -> Self {
        self.0.ttol = ttol;
        self
    }

    /// The time tolerance for matching epochs in seconds.
    pub fn time_tolerance(&self) -> f64 {
        self.0.ttol
    }

    /// Set the session length for multi-file output in seconds. Zero produces
    /// a single output file.
    pub fn with_time_unit(mut self, tunit: f64) -> Self {
        self.0.tunit = tunit;
        self
    }

    /// The session length for multi-file output in seconds.
    pub fn time_unit(&self) -> f64 {
        self.0.tunit
    }

    /// Restrict conversion to a time window. Both times zero means no
    /// restriction.
    pub fn with_time_window(mut self, start: GpsTime, end: GpsTime) -> Self {
        self.0.ts = start.0;
        self.0.te = end.0;
        self
    }

    /// Include ionosphere correction parameters in the navigation output.
    pub fn with_outiono(mut self, enable: bool) -> Self {
        self.0.outiono = enable as i32;
        self
    }

    /// Whether ionosphere correction parameters are included in the output.
    pub fn outiono(&self) -> bool {
        self.0.outiono != 0
    }

    /// Include time system correction parameters in the navigation output.
    pub fn with_outtime(mut self, enable: bool) -> Self {
        self.0.outtime = enable as i32;
        self
    }

    /// Whether time system correction parameters are included in the output.
    pub fn outtime(&self) -> bool {
        self.0.outtime != 0
    }

    /// Include leap second parameters in the navigation output.
    pub fn with_outleaps(mut self, enable: bool) -> Self {
        self.0.outleaps = enable as i32;
        self
    }

    /// Whether leap second parameters are included in the output.
    pub fn outleaps(&self) -> bool {
        self.0.outleaps != 0
    }

    /// Detect the approximate receiver position automatically from the data.
    pub fn with_autopos(mut self, enable: bool) -> Self {
        self.0.autopos = enable as i32;
        self
    }

    /// Whether approximate receiver position is detected automatically.
    pub fn autopos(&self) -> bool {
        self.0.autopos != 0
    }

    /// Write separate navigation files for each constellation.
    pub fn with_sep_nav(mut self, enable: bool) -> Self {
        self.0.sep_nav = enable as i32;
        self
    }

    /// Whether navigation files are written separately per constellation.
    pub fn sep_nav(&self) -> bool {
        self.0.sep_nav != 0
    }

    /// Set the station ID written to the RINEX header.
    pub fn with_station_id(mut self, id: impl AsRef<OsStr>) -> Self {
        copy_osstr(&mut self.0.staid, id.as_ref());
        self
    }

    /// Set the marker name written to the RINEX header.
    pub fn with_marker_name(mut self, name: impl AsRef<OsStr>) -> Self {
        copy_osstr(&mut self.0.marker, name.as_ref());
        self
    }

    /// Set the marker number written to the RINEX header.
    pub fn with_marker_number(mut self, number: impl AsRef<OsStr>) -> Self {
        copy_osstr(&mut self.0.markerno, number.as_ref());
        self
    }

    /// Set the marker type written to the RINEX header.
    pub fn with_marker_type(mut self, t: impl AsRef<OsStr>) -> Self {
        copy_osstr(&mut self.0.markertype, t.as_ref());
        self
    }

    /// Set the observer name written to the RINEX header.
    pub fn with_observer(mut self, observer: impl AsRef<OsStr>) -> Self {
        copy_osstr(&mut self.0.name[0], observer.as_ref());
        self
    }

    /// Set the agency name written to the RINEX header.
    pub fn with_agency(mut self, agency: impl AsRef<OsStr>) -> Self {
        copy_osstr(&mut self.0.name[1], agency.as_ref());
        self
    }
}

/// Output file paths for [`convrnx`]. An empty path disables the corresponding output.
#[repr(C)]
#[derive(Default)]
pub struct RnxOutputFiles {
    obs: OsString,
    nav: OsString,
    gnav: OsString,
    hnav: OsString,
    qnav: OsString,
    lnav: OsString,
    cnav: OsString,
    inav: OsString,
    sbas: OsString,
}

impl RnxOutputFiles {
    /// Create output file paths with only the observation file set. All other outputs are
    /// disabled.
    pub fn new(obs: impl AsRef<OsStr>) -> Self {
        Self {
            obs: obs.as_ref().to_owned(),
            ..Self::default()
        }
    }

    pub(crate) fn as_slice(&self) -> &[OsString] {
        // SAFETY: #[repr(C)] with 9 contiguous same-type fields is layout-equivalent to
        // [OsString; 9].
        unsafe { std::slice::from_raw_parts(self as *const Self as *const OsString, 9) }
    }

    /// Set the GPS navigation output file.
    pub fn with_nav(mut self, nav: impl AsRef<OsStr>) -> Self {
        self.nav = nav.as_ref().to_owned();
        self
    }

    /// Set the GLONASS navigation output file.
    pub fn with_gnav(mut self, gnav: impl AsRef<OsStr>) -> Self {
        self.gnav = gnav.as_ref().to_owned();
        self
    }

    /// Set the GEO/SBAS navigation output file.
    pub fn with_hnav(mut self, hnav: impl AsRef<OsStr>) -> Self {
        self.hnav = hnav.as_ref().to_owned();
        self
    }

    /// Set the QZSS navigation output file.
    pub fn with_qnav(mut self, qnav: impl AsRef<OsStr>) -> Self {
        self.qnav = qnav.as_ref().to_owned();
        self
    }

    /// Set the LEX navigation output file.
    pub fn with_lnav(mut self, lnav: impl AsRef<OsStr>) -> Self {
        self.lnav = lnav.as_ref().to_owned();
        self
    }

    /// Set the BeiDou navigation output file.
    pub fn with_cnav(mut self, cnav: impl AsRef<OsStr>) -> Self {
        self.cnav = cnav.as_ref().to_owned();
        self
    }

    /// Set the IRNSS navigation output file.
    pub fn with_inav(mut self, inav: impl AsRef<OsStr>) -> Self {
        self.inav = inav.as_ref().to_owned();
        self
    }

    /// Set the SBAS log output file.
    pub fn with_sbas(mut self, sbas: impl AsRef<OsStr>) -> Self {
        self.sbas = sbas.as_ref().to_owned();
        self
    }
}

/// Error from [`convrnx`].
#[derive(Debug, Error)]
pub enum ConvrnxError {
    /// A path contains a nul byte.
    #[error("path contains a nul byte: {0:?}")]
    NulByte(OsString),
    /// The conversion failed.
    #[error("conversion failed")]
    Failed,
    /// The conversion was aborted by the progress callback.
    #[error("conversion aborted")]
    Aborted,
    /// RTKLIB returned an unrecognized status code.
    #[error("unknown return code: {0}")]
    Unknown(i32),
}

/// Convert a raw receiver data file to RINEX.
///
/// Returns `Ok(())` on success. Returns [`ConvrnxError::Failed`] if RTKLIB reports failure, or
/// [`ConvrnxError::Aborted`] if the conversion was interrupted.
pub fn convrnx(
    format: StreamFmt,
    opt: &mut RnxOpt,
    file: impl AsRef<OsStr>,
    ofile: &RnxOutputFiles,
) -> Result<(), ConvrnxError> {
    let file_arr = CStringArray::try_single(file.as_ref()).map_err(|e| ConvrnxError::NulByte(e.to_owned()))?;
    let mut ofile_arr = CStringArray::try_new(ofile.as_slice()).map_err(|e| ConvrnxError::NulByte(e.to_owned()))?;

    let ret = unsafe {
        ffi::convrnx(
            format as i32,
            &mut opt.0,
            file_arr.first(),
            ofile_arr.as_mut_ptr() as *mut *mut i8,
        )
    };

    match ret {
        1 => Ok(()),
        0 => Err(ConvrnxError::Failed),
        -1 => Err(ConvrnxError::Aborted),
        _ => Err(ConvrnxError::Unknown(ret)),
    }
}
