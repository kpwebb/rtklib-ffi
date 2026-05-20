//! RTCM3 message decoding.
//!
//! ```no_run
//! use rtklib_ffi::rtcm::{DecodeResult, MsgType, RtcmDecoder};
//!
//! let mut decoder = RtcmDecoder::try_new().unwrap();
//! # let rtcm_bytes: Vec<u8> = vec![];
//!
//! for &byte in &rtcm_bytes {
//!     let Some(status) = decoder.decode(byte) else { continue; };
//!     match status {
//!         DecodeResult::Observation => {
//!             let obs = decoder.observations();
//!             // process observations...
//!         }
//!         DecodeResult::Ephemeris => {
//!             let msg_type = decoder.message_type().unwrap();
//!             // handle ephemeris...
//!         }
//!         _ => {}
//!     }
//! }
//! ```

use crate::{meas::ObsData, DecoderInitError};
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;
use std::convert::TryFrom;

/// Outcome of feeding a byte into the RTCM3 decoder.
///
/// Maps the return codes documented in `rtcm.c`:
/// `-1`=error, `0`=no message, `1`=observation, `2`=ephemeris,
/// `5`=station info, `6`=time params, `7`=DGPS corrections,
/// `9`=special message, `20`=SSR corrections.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum DecodeResult {
    /// Incomplete frame; feed more bytes.
    Incomplete = 0,
    /// Observation data decoded.
    Observation = 1,
    /// Ephemeris decoded.
    Ephemeris = 2,
    /// Station position/antenna parameters decoded.
    StationInfo = 5,
    /// Time parameters decoded.
    TimeParams = 6,
    /// DGPS corrections decoded.
    DgpsCorrections = 7,
    /// Special/text message decoded.
    SpecialMessage = 9,
    /// SSR corrections decoded.
    SsrCorrections = 20,
}

/// RTCM3 message decoder.
///
/// Wraps the RTKLIB `rtcm_t` struct. Call [`try_new`](Self::try_new) to create,
/// then feed bytes via [`decode`](Self::decode). When `decode` returns
/// [`Some(DecodeResult::Observation)`], read the observations with
/// [`observations`](Self::observations).
pub struct RtcmDecoder(Box<ffi::rtcm_t>);

impl RtcmDecoder {
    /// Create a new RTCM decoder.
    ///
    /// Returns `Err` if RTKLIB cannot allocate internal buffers.
    pub fn try_new() -> Result<Self, DecoderInitError> {
        unsafe {
            // rtcm_t is ~886KB, too large for the stack. Allocate zeroed
            // memory directly on the heap to avoid stack overflow.
            let layout = std::alloc::Layout::new::<ffi::rtcm_t>();
            let ptr = std::alloc::alloc_zeroed(layout) as *mut ffi::rtcm_t;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            let mut rtcm = Box::from_raw(ptr);
            if ffi::init_rtcm(rtcm.as_mut()) == 0 {
                return Err(DecoderInitError);
            }
            Ok(Self(rtcm))
        }
    }

    /// Feed one byte into the RTCM3 decoder.
    ///
    /// Returns `None` if the byte did not complete a recognized message.
    pub fn decode(&mut self, byte: u8) -> Option<DecodeResult> {
        let ret = unsafe { ffi::input_rtcm3(self.0.as_mut(), byte) };
        DecodeResult::try_from(ret).ok()
    }

    /// The RTCM3 message type of the last decoded message.
    ///
    /// Returns `None` if the type number is not recognized.
    /// Only meaningful after `decode` returns `Some`.
    pub fn message_type(&self) -> Option<MsgType> {
        let raw = unsafe { ffi::getbitu(self.0.buff.as_ptr(), 24, 12) as u16 };
        MsgType::try_from(raw).ok()
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
        // SAFETY: ObsData is #[repr(transparent)] over obsd_t.
        // obs.data is a contiguous array allocated by init_rtcm,
        // valid for the lifetime of &self.
        unsafe { std::slice::from_raw_parts(self.0.obs.data as *const ObsData, n) }
    }

    /// Station ID from the last decoded message.
    pub fn station_id(&self) -> i32 {
        self.0.staid
    }
}

impl Drop for RtcmDecoder {
    fn drop(&mut self) {
        unsafe {
            ffi::free_rtcm(self.0.as_mut());
        }
    }
}

/// RTCM3 message type numbers handled by RTKLIB's `decode_rtcm3`.
///
/// Reference: [RTCM 3 Message List](https://www.use-snip.com/kb/knowledge-base/rtcm-3-message-list/)
/// and RTCM Standard 10403.x.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum MsgType {
    // GPS legacy observations (L1 only / L1+L2)
    GpsL1Obs = 1001,
    GpsL1ObsFull = 1002,
    GpsL1L2Obs = 1003,
    GpsL1L2ObsFull = 1004,

    // Station and antenna
    StationArpEcef = 1005,
    StationArpEcefHeight = 1006,
    AntennaDescriptor = 1007,
    AntennaDescriptorSerial = 1008,

    // GLONASS legacy observations (L1 only / L1+L2)
    GloL1Obs = 1009,
    GloL1ObsFull = 1010,
    GloL1L2Obs = 1011,
    GloL1L2ObsFull = 1012,

    // System parameters
    SystemParameters = 1013,

    // GPS ephemeris
    GpsEphemeris = 1019,

    // GLONASS ephemeris
    GloEphemeris = 1020,

    // Coordinate transformation (Helmert/Molodensky)
    HelmertAbridgedMolodensky = 1021,
    MolodenskyBadekas = 1022,
    Residuals = 1023,
    PlaneCoordProjection = 1024,
    ProjectionExceptLcc2sp = 1025,
    ProjectionParametersLcc2sp = 1026,
    ProjectionParametersOm = 1027,

    // Unicode text string
    UnicodeTextString = 1029,

    // Network RTK residuals
    GpsNetworkRtkResidual = 1030,
    GloNetworkRtkResidual = 1031,
    PhysicalReferenceStation = 1032,

    // Receiver and antenna descriptors
    ReceiverAntennaDescriptor = 1033,

    // GPS network FKP gradient
    GpsNetworkFkpGradient = 1034,
    GloNetworkFkpGradient = 1035,

    // GLONASS network RTK ionosphere
    GloNetworkRtkIonoCorrection = 1037,
    GloNetworkRtkGeometricCorrection = 1038,
    GloNetworkRtkCombinedCorrection = 1039,

    // NavIC/IRNSS ephemeris
    NavicEphemeris = 1041,

    // BeiDou ephemeris
    BdsEphemeris = 1042,

    // QZSS ephemeris
    QzsEphemeris = 1044,

    // Galileo F/NAV ephemeris
    GalFnavEphemeris = 1045,
    // Galileo I/NAV ephemeris
    GalInavEphemeris = 1046,

    // SSR GPS
    GpsSsrOrbitCorrection = 1057,
    GpsSsrClockCorrection = 1058,
    GpsSsrCodeBias = 1059,
    GpsSsrCombinedCorrection = 1060,
    GpsSsrUra = 1061,
    GpsSsrHighRateClockCorrection = 1062,

    // SSR GLONASS
    GloSsrOrbitCorrection = 1063,
    GloSsrClockCorrection = 1064,
    GloSsrCodeBias = 1065,
    GloSsrCombinedCorrection = 1066,
    GloSsrUra = 1067,
    GloSsrHighRateClockCorrection = 1068,

    // GPS MSM1–MSM7
    GpsMsm1 = 1071,
    GpsMsm2 = 1072,
    GpsMsm3 = 1073,
    GpsMsm4 = 1074,
    GpsMsm5 = 1075,
    GpsMsm6 = 1076,
    GpsMsm7 = 1077,

    // GLONASS MSM1–MSM7
    GloMsm1 = 1081,
    GloMsm2 = 1082,
    GloMsm3 = 1083,
    GloMsm4 = 1084,
    GloMsm5 = 1085,
    GloMsm6 = 1086,
    GloMsm7 = 1087,

    // Galileo MSM1–MSM7
    GalMsm1 = 1091,
    GalMsm2 = 1092,
    GalMsm3 = 1093,
    GalMsm4 = 1094,
    GalMsm5 = 1095,
    GalMsm6 = 1096,
    GalMsm7 = 1097,

    // SBAS MSM1–MSM7
    SbasMsm1 = 1101,
    SbasMsm2 = 1102,
    SbasMsm3 = 1103,
    SbasMsm4 = 1104,
    SbasMsm5 = 1105,
    SbasMsm6 = 1106,
    SbasMsm7 = 1107,

    // QZSS MSM1–MSM7
    QzsMsm1 = 1111,
    QzsMsm2 = 1112,
    QzsMsm3 = 1113,
    QzsMsm4 = 1114,
    QzsMsm5 = 1115,
    QzsMsm6 = 1116,
    QzsMsm7 = 1117,

    // BeiDou MSM1–MSM7
    BdsMsm1 = 1121,
    BdsMsm2 = 1122,
    BdsMsm3 = 1123,
    BdsMsm4 = 1124,
    BdsMsm5 = 1125,
    BdsMsm6 = 1126,
    BdsMsm7 = 1127,

    // NavIC/IRNSS MSM1–MSM7
    NavicMsm1 = 1131,
    NavicMsm2 = 1132,
    NavicMsm3 = 1133,
    NavicMsm4 = 1134,
    NavicMsm5 = 1135,
    NavicMsm6 = 1136,
    NavicMsm7 = 1137,

    // GLONASS L1/L2 code-phase biases
    GloL1L2CodePhaseBias = 1230,

    // SSR Galileo (draft)
    GalSsrOrbitCorrection = 1240,
    GalSsrClockCorrection = 1241,
    GalSsrCodeBias = 1242,
    GalSsrCombinedCorrection = 1243,
    GalSsrUra = 1244,
    GalSsrHighRateClockCorrection = 1245,

    // SSR QZSS (draft)
    QzsSsrOrbitCorrection = 1246,
    QzsSsrClockCorrection = 1247,
    QzsSsrCodeBias = 1248,
    QzsSsrCombinedCorrection = 1249,
    QzsSsrUra = 1250,
    QzsSsrHighRateClockCorrection = 1251,

    // SSR SBAS (draft)
    SbasSsrOrbitCorrection = 1252,
    SbasSsrClockCorrection = 1253,
    SbasSsrCodeBias = 1254,
    SbasSsrCombinedCorrection = 1255,
    SbasSsrUra = 1256,
    SbasSsrHighRateClockCorrection = 1257,

    // SSR BeiDou (draft)
    BdsSsrOrbitCorrection = 1258,
    BdsSsrClockCorrection = 1259,
    BdsSsrCodeBias = 1260,
    BdsSsrCombinedCorrection = 1261,
    BdsSsrUra = 1262,
    BdsSsrHighRateClockCorrection = 1263,

    // Proprietary messages
    ProprietaryNonmea = 4073,
    ProprietaryIgssSsr = 4076,
}
