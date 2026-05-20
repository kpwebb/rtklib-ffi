//! Post-processed kinematic positioning.
//!
//! ```no_run
//! use rtklib_ffi::ppk::{PrcOpt, SolOpt, FilOpt, postpos};
//!
//! let popt = PrcOpt::kinematic();
//! let sopt = SolOpt::default();
//! let fopt = FilOpt::default();
//!
//! postpos(
//!     "rover.obs", "base.obs",
//!     &["nav.nav"], "output.pos",
//!     &popt, &sopt, &fopt,
//! ).unwrap();
//! ```

use crate::{util::CStringArray, NavSys};
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;
use std::ffi::{CString, OsStr, OsString};

/// Positioning mode.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum PosMode {
    /// Single point positioning. From PMODE_SINGLE.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_SINGLE"))]
    Single = ffi::PMODE_SINGLE,
    /// Differential GPS / DGNSS. From PMODE_DGPS.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_DGPS"))]
    Dgps = ffi::PMODE_DGPS,
    /// Kinematic positioning. From PMODE_KINEMA.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_KINEMA"))]
    Kinematic = ffi::PMODE_KINEMA,
    /// Static positioning. From PMODE_STATIC.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_STATIC"))]
    Static = ffi::PMODE_STATIC,
    /// Static positioning starting from a known position. From PMODE_STATIC_START.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_STATIC_START"))]
    StaticStart = ffi::PMODE_STATIC_START,
    /// Moving base station. From PMODE_MOVEB.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_MOVEB"))]
    MovingBase = ffi::PMODE_MOVEB,
    /// Fixed position. From PMODE_FIXED.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_FIXED"))]
    Fixed = ffi::PMODE_FIXED,
    /// Precise Point Positioning, kinematic. From PMODE_PPP_KINEMA.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_PPP_KINEMA"))]
    PppKinematic = ffi::PMODE_PPP_KINEMA,
    /// Precise Point Positioning, static. From PMODE_PPP_STATIC.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_PPP_STATIC"))]
    PppStatic = ffi::PMODE_PPP_STATIC,
    /// Precise Point Positioning, fixed. From PMODE_PPP_FIXED.
    #[cfg_attr(feature = "strum", strum(to_string = "PMODE_PPP_FIXED"))]
    PppFixed = ffi::PMODE_PPP_FIXED,
}

/// Time format for solution output.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum TimeFormat {
    /// GPS seconds of week: `sssss.s`.
    GpsSeconds = 0,
    /// Calendar date and time: `yyyy/mm/dd hh:mm:ss.s`.
    Calendar = 1,
}

/// Solution output format.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum SolFormat {
    /// Latitude, longitude, and height. From SOLF_LLH.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLF_LLH"))]
    Llh = ffi::SOLF_LLH,
    /// X, Y, Z in ECEF coordinates. From SOLF_XYZ.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLF_XYZ"))]
    Xyz = ffi::SOLF_XYZ,
    /// East, north, up baseline components. From SOLF_ENU.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLF_ENU"))]
    Enu = ffi::SOLF_ENU,
    /// NMEA-0183 sentences. From SOLF_NMEA.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLF_NMEA"))]
    Nmea = ffi::SOLF_NMEA,
}

/// Ionosphere correction option.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum IonoOpt {
    /// Ionosphere correction disabled. From IONOOPT_OFF.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_OFF"))]
    Off = ffi::IONOOPT_OFF,
    /// Klobuchar broadcast model. From IONOOPT_BRDC.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_BRDC"))]
    Broadcast = ffi::IONOOPT_BRDC,
    /// SBAS ionosphere model. From IONOOPT_SBAS.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_SBAS"))]
    Sbas = ffi::IONOOPT_SBAS,
    /// Iono-free linear combination of L1/L2 or L1/L5. From IONOOPT_IFLC.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_IFLC"))]
    IonFreeLc = ffi::IONOOPT_IFLC,
    /// Ionosphere delay estimation. From IONOOPT_EST.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_EST"))]
    Estimation = ffi::IONOOPT_EST,
    /// IONEX TEC grid model. From IONOOPT_TEC.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_TEC"))]
    Tec = ffi::IONOOPT_TEC,
    /// QZSS broadcast ionosphere model. From IONOOPT_QZS.
    #[cfg_attr(feature = "strum", strum(to_string = "IONOOPT_QZS"))]
    Qzs = ffi::IONOOPT_QZS,
}

/// Troposphere correction option.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum TropOpt {
    /// Troposphere correction disabled. From TROPOPT_OFF.
    #[cfg_attr(feature = "strum", strum(to_string = "TROPOPT_OFF"))]
    Off = ffi::TROPOPT_OFF,
    /// Saastamoinen model. From TROPOPT_SAAS.
    #[cfg_attr(feature = "strum", strum(to_string = "TROPOPT_SAAS"))]
    Saastamoinen = ffi::TROPOPT_SAAS,
    /// SBAS troposphere model. From TROPOPT_SBAS.
    #[cfg_attr(feature = "strum", strum(to_string = "TROPOPT_SBAS"))]
    Sbas = ffi::TROPOPT_SBAS,
    /// Zenith total delay estimation. From TROPOPT_EST.
    #[cfg_attr(feature = "strum", strum(to_string = "TROPOPT_EST"))]
    Estimation = ffi::TROPOPT_EST,
    /// Zenith total delay plus horizontal gradient estimation. From TROPOPT_ESTG.
    #[cfg_attr(feature = "strum", strum(to_string = "TROPOPT_ESTG"))]
    EstimationGrad = ffi::TROPOPT_ESTG,
}

/// Ambiguity resolution mode.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum ArMode {
    /// Ambiguity resolution disabled.
    #[cfg_attr(feature = "strum", strum(to_string = "ARMODE_OFF"))]
    Off = ffi::ARMODE_OFF,
    /// Continuous ambiguity resolution.
    #[cfg_attr(feature = "strum", strum(to_string = "ARMODE_CONT"))]
    Continuous = ffi::ARMODE_CONT,
    /// Instantaneous ambiguity resolution.
    #[cfg_attr(feature = "strum", strum(to_string = "ARMODE_INST"))]
    Instantaneous = ffi::ARMODE_INST,
    /// Fix-and-hold ambiguity resolution.
    #[cfg_attr(feature = "strum", strum(to_string = "ARMODE_FIXHOLD"))]
    FixAndHold = ffi::ARMODE_FIXHOLD,
}

/// Filter solution type.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum SolutionType {
    /// Forward filter only. From SOLTYPE_FORWARD.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLTYPE_FORWARD"))]
    Forward = ffi::SOLTYPE_FORWARD,
    /// Backward filter only. From SOLTYPE_BACKWARD.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLTYPE_BACKWARD"))]
    Backward = ffi::SOLTYPE_BACKWARD,
    /// Combined forward+backward. From SOLTYPE_COMBINED.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLTYPE_COMBINED"))]
    Combined = ffi::SOLTYPE_COMBINED,
    /// Combined forward+backward without phase reset. From SOLTYPE_COMBINED_NORESET.
    #[cfg_attr(feature = "strum", strum(to_string = "SOLTYPE_COMBINED_NORESET"))]
    CombinedNoReset = ffi::SOLTYPE_COMBINED_NORESET,
}

/// Processing options wrapper around `prcopt_t`.
pub struct PrcOpt(ffi::prcopt_t);

impl Default for PrcOpt {
    fn default() -> Self {
        Self(unsafe { ffi::prcopt_default })
    }
}

impl PrcOpt {
    /// Kinematic mode with combined forward+backward solution.
    pub fn kinematic() -> Self {
        let mut opt = Self::default();
        opt.0.mode = PosMode::Kinematic as i32;
        opt.0.soltype = SolutionType::Combined as i32;
        opt.0.modear = ArMode::Continuous as i32;
        opt.0.nf = 2;
        opt
    }

    /// Static mode with combined forward+backward solution.
    pub fn static_mode() -> Self {
        let mut opt = Self::default();
        opt.0.mode = PosMode::Static as i32;
        opt.0.soltype = SolutionType::Combined as i32;
        opt.0.modear = ArMode::Continuous as i32;
        opt.0.nf = 2;
        opt
    }

    /// Positioning mode.
    pub fn with_mode(mut self, mode: PosMode) -> Self {
        self.0.mode = mode as i32;
        self
    }

    /// Get positioning mode.
    pub fn mode(&self) -> PosMode {
        // Only set via typed setters or RTKLIB defaults; an invalid value is an unreachable bug.
        PosMode::try_from(self.0.mode as u32).unwrap()
    }

    /// Solution type.
    pub fn with_solution_type(mut self, sol: SolutionType) -> Self {
        self.0.soltype = sol as i32;
        self
    }

    /// Get solution type.
    pub fn solution_type(&self) -> SolutionType {
        // Only set via typed setters or RTKLIB defaults; an invalid value is an unreachable bug.
        SolutionType::try_from(self.0.soltype as u32).unwrap()
    }

    /// Enabled navigation systems.
    pub fn with_navsys(mut self, sys: NavSys) -> Self {
        self.0.navsys = sys.bits() as i32;
        self
    }

    /// Get enabled navigation systems.
    pub fn navsys(&self) -> NavSys {
        NavSys::from_bits_truncate(self.0.navsys as u32)
    }

    /// Number of frequencies. 1=L1, 2=L1+L2, 3=L1+L2+L5.
    pub fn with_frequencies(mut self, nf: i32) -> Self {
        self.0.nf = nf;
        self
    }

    /// Get number of frequencies.
    pub fn frequencies(&self) -> i32 {
        self.0.nf
    }

    /// Elevation mask angle in degrees.
    pub fn with_elevation_mask(mut self, deg: f64) -> Self {
        self.0.elmin = deg.to_radians();
        self
    }

    /// Get elevation mask angle in radians.
    pub fn elevation_mask(&self) -> f64 {
        self.0.elmin
    }

    /// Ambiguity resolution mode.
    pub fn with_ar_mode(mut self, mode: ArMode) -> Self {
        self.0.modear = mode as i32;
        self
    }

    /// Get ambiguity resolution mode.
    pub fn ar_mode(&self) -> ArMode {
        // Only set via typed setters or RTKLIB defaults; an invalid value is an unreachable bug.
        ArMode::try_from(self.0.modear as u32).unwrap()
    }

    /// Ionosphere correction option.
    pub fn with_ionosphere(mut self, opt: IonoOpt) -> Self {
        self.0.ionoopt = opt as i32;
        self
    }

    /// Get ionosphere correction option.
    pub fn ionosphere(&self) -> IonoOpt {
        // Only set via typed setters or RTKLIB defaults; an invalid value is an unreachable bug.
        IonoOpt::try_from(self.0.ionoopt as u32).unwrap()
    }

    /// Base station position in ECEF coordinates (meters).
    ///
    /// Equivalent to the `-r` flag in `rnx2rtkp`.
    pub fn with_base_position_ecef(mut self, x: f64, y: f64, z: f64) -> Self {
        self.0.refpos = ffi::POSOPT_POS_XYZ as i32;
        self.0.rb = [x, y, z];
        self
    }

    /// Base station position in geodetic coordinates
    /// (latitude and longitude in degrees, height in meters).
    ///
    /// Equivalent to the `-l` flag in `rnx2rtkp`.
    pub fn with_base_position_llh(mut self, lat_deg: f64, lon_deg: f64, height: f64) -> Self {
        self.0.refpos = ffi::POSOPT_POS_LLH as i32;
        let pos = [lat_deg.to_radians(), lon_deg.to_radians(), height];
        unsafe { ffi::pos2ecef(pos.as_ptr(), self.0.rb.as_mut_ptr()) };
        self
    }

    /// Get base station position in ECEF coordinates (meters).
    pub fn base_position_ecef(&self) -> [f64; 3] {
        self.0.rb
    }

    /// Troposphere correction option.
    pub fn with_troposphere(mut self, opt: TropOpt) -> Self {
        self.0.tropopt = opt as i32;
        self
    }

    /// Get troposphere correction option.
    pub fn troposphere(&self) -> TropOpt {
        // Only set via typed setters or RTKLIB defaults; an invalid value is an unreachable bug.
        TropOpt::try_from(self.0.tropopt as u32).unwrap()
    }

    pub(crate) fn as_ffi(&self) -> &ffi::prcopt_t {
        &self.0
    }
}

/// Solution output options wrapper around `solopt_t`.
pub struct SolOpt(ffi::solopt_t);

impl Default for SolOpt {
    fn default() -> Self {
        Self(unsafe { ffi::solopt_default })
    }
}

impl SolOpt {
    /// Solution output format.
    pub fn with_format(mut self, format: SolFormat) -> Self {
        self.0.posf = format as i32;
        self
    }

    /// Get solution output format.
    pub fn format(&self) -> SolFormat {
        // Only set via typed setters or RTKLIB defaults; an invalid value is an unreachable bug.
        SolFormat::try_from(self.0.posf as u32).unwrap()
    }

    /// Time format.
    pub fn with_time_format(mut self, timef: TimeFormat) -> Self {
        self.0.timef = timef as i32;
        self
    }

    /// Get time format.
    pub fn time_format(&self) -> TimeFormat {
        TimeFormat::try_from(self.0.timef as u32).unwrap()
    }

    /// Number of decimal places for time output.
    pub fn with_time_decimals(mut self, timeu: i32) -> Self {
        self.0.timeu = timeu;
        self
    }

    /// Get number of decimal places for time output.
    pub fn time_decimals(&self) -> i32 {
        self.0.timeu
    }

    /// Output header.
    pub fn with_output_header(mut self, enable: bool) -> Self {
        self.0.outhead = enable as i32;
        self
    }

    /// Get whether output header is enabled.
    pub fn output_header(&self) -> bool {
        self.0.outhead != 0
    }

    pub(crate) fn as_ffi(&self) -> &ffi::solopt_t {
        &self.0
    }
}

/// File options wrapper around `filopt_t`.
pub struct FilOpt(ffi::filopt_t);

impl Default for FilOpt {
    fn default() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }
}

impl FilOpt {
    pub(crate) fn as_ffi(&self) -> &ffi::filopt_t {
        &self.0
    }
}

/// Error from PPK post-processing.
#[derive(Debug, thiserror::Error)]
pub enum PostposError {
    /// A file path contained an interior null byte.
    #[error("path contains null byte: {0:?}")]
    NulByte(OsString),
    /// Too many input files for RTKLIB's fixed-size array.
    #[error("{count} > {max} input files")]
    TooManyInputFiles { count: usize, max: usize },
    /// RTKLIB `postpos()` returned an error code.
    #[error("postpos processing failed with code {0}")]
    ProcessingFailed(i32),
}

impl From<&OsStr> for PostposError {
    fn from(s: &OsStr) -> Self {
        Self::NulByte(s.to_owned())
    }
}

/// Run PPK post-processing on RINEX observation and navigation files.
///
/// Returns `Ok(())` on success. Results are written to the output file.
pub fn postpos<T: AsRef<OsStr>>(
    rover_obs: impl AsRef<OsStr>,
    base_obs: impl AsRef<OsStr>,
    nav_files: &[T],
    output: impl AsRef<OsStr>,
    popt: &PrcOpt,
    sopt: &SolOpt,
    fopt: &FilOpt,
) -> Result<(), PostposError> {
    const MAX_INFILE: usize = 1000;

    let total = 2 + nav_files.len();
    if total > MAX_INFILE {
        return Err(PostposError::TooManyInputFiles {
            count: total,
            max: MAX_INFILE,
        });
    }

    let mut all_inputs: Vec<&OsStr> = vec![rover_obs.as_ref(), base_obs.as_ref()];
    all_inputs.extend(nav_files.iter().map(|f| f.as_ref()));

    // C signature is `const char **infile` — the strings are const but the
    // pointer to the array is not, so bindgen generates `*mut *const c_char`.
    // The function does not actually mutate the array.
    let mut infile_arr = CStringArray::try_new(&all_inputs)?;
    let out_arr = CStringArray::try_new(&[output.as_ref()])?;
    let rov = CString::new("").unwrap();
    let base = CString::new("").unwrap();

    let ts = ffi::gtime_t { time: 0, sec: 0.0 };
    let te = ffi::gtime_t { time: 0, sec: 0.0 };

    let ret = unsafe {
        ffi::postpos(
            ts,
            te,
            0.0, // processing interval (0 = all)
            0.0, // processing unit time (0 = all)
            popt.as_ffi(),
            sopt.as_ffi(),
            fopt.as_ffi(),
            infile_arr.as_mut_ptr(),
            infile_arr.len() as i32,
            out_arr.first(),
            rov.as_ptr(),
            base.as_ptr(),
        )
    };

    if ret == 0 {
        Ok(())
    } else {
        Err(PostposError::ProcessingFailed(ret))
    }
}
