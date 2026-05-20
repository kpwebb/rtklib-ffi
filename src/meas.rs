//! GNSS measurement and navigation data types.
//!
//! These types wrap the FFI structs produced by format decoders such as
//! [`RtcmDecoder`](crate::rtcm::RtcmDecoder) and [`SbfDecoder`](crate::raw::SbfDecoder).

use crate::GpsTime;
use num_enum::TryFromPrimitive;
use rtklib_sys::rtklib as ffi;
use std::{convert::TryFrom, slice::from_raw_parts};

/// A single GNSS observation record.
///
/// Transparent wrapper around the FFI `obsd_t` struct. Produced by format decoders.
#[repr(transparent)]
pub struct ObsData(ffi::obsd_t);

impl ObsData {
    /// Satellite number in RTKLIB internal numbering.
    pub fn sat(&self) -> u8 {
        self.0.sat
    }

    /// Carrier phase measurements for up to 3 frequencies, in cycles.
    pub fn carrier_phase(&self) -> &[f64; 3] {
        &self.0.L
    }

    /// Pseudorange measurements for up to 3 frequencies, in meters.
    pub fn pseudorange(&self) -> &[f64; 3] {
        &self.0.P
    }

    /// Doppler measurements for up to 3 frequencies, in Hz.
    pub fn doppler(&self) -> &[f32; 3] {
        &self.0.D
    }

    /// Signal-to-noise ratio for up to 3 frequencies, in dB-Hz.
    pub fn snr(&self) -> &[f32; 3] {
        &self.0.SNR
    }

    /// Signal code identifiers for up to 3 frequencies.
    pub fn code(&self) -> &[u8; 3] {
        &self.0.code
    }

    /// Loss-of-lock indicators for up to 3 frequencies.
    pub fn lli(&self) -> &[u8; 3] {
        &self.0.LLI
    }
}

/// GPS/Galileo/BeiDou/QZSS broadcast ephemeris record.
///
/// Transparent wrapper around the FFI `eph_t` struct.
#[repr(transparent)]
pub struct Eph(ffi::eph_t);

impl Eph {
    /// Satellite number in RTKLIB internal numbering.
    pub fn sat(&self) -> i32 { self.0.sat }
    /// Issue of data, ephemeris.
    pub fn iode(&self) -> i32 { self.0.iode }
    /// Issue of data, clock.
    pub fn iodc(&self) -> i32 { self.0.iodc }
    /// SV accuracy index (URA).
    pub fn sva(&self) -> i32 { self.0.sva }
    /// Raw SV health field. Use [`is_healthy`](Self::is_healthy) for a simple check.
    pub fn svh(&self) -> i32 { self.0.svh }
    /// Returns true if the SV health field indicates no issues.
    pub fn is_healthy(&self) -> bool { self.0.svh == 0 }
    /// GPS/QZS week number; Galileo week number for GAL.
    pub fn week(&self) -> i32 { self.0.week }
    /// Signal codes: GPS/QZS L2 code type; GAL/BDS data source bitmask.
    pub fn code(&self) -> i32 { self.0.code }
    /// Flags: GPS/QZS L2 P data flag; BDS nav message type.
    pub fn flag(&self) -> i32 { self.0.flag }
    /// Time of ephemeris.
    pub fn toe(&self) -> GpsTime { GpsTime(self.0.toe) }
    /// Time of clock.
    pub fn toc(&self) -> GpsTime { GpsTime(self.0.toc) }
    /// Signal transmission time.
    pub fn ttr(&self) -> GpsTime { GpsTime(self.0.ttr) }
    /// Semi-major axis, in meters.
    pub fn semi_major_axis(&self) -> f64 { self.0.A }
    /// Eccentricity.
    pub fn eccentricity(&self) -> f64 { self.0.e }
    /// Inclination at reference time, in radians.
    pub fn inclination(&self) -> f64 { self.0.i0 }
    /// Longitude of ascending node at weekly epoch, in radians.
    pub fn right_ascension(&self) -> f64 { self.0.OMG0 }
    /// Argument of perigee, in radians.
    pub fn argument_of_perigee(&self) -> f64 { self.0.omg }
    /// Mean anomaly at reference time, in radians.
    pub fn mean_anomaly(&self) -> f64 { self.0.M0 }
    /// Mean motion difference from computed value, in rad/s.
    pub fn mean_motion_diff(&self) -> f64 { self.0.deln }
    /// Rate of right ascension, in rad/s.
    pub fn right_ascension_rate(&self) -> f64 { self.0.OMGd }
    /// Rate of inclination, in rad/s.
    pub fn inclination_rate(&self) -> f64 { self.0.idot }
    /// Orbit radius harmonic sine correction, in meters.
    pub fn crs(&self) -> f64 { self.0.crs }
    /// Orbit radius harmonic cosine correction, in meters.
    pub fn crc(&self) -> f64 { self.0.crc }
    /// Argument of latitude harmonic sine correction, in radians.
    pub fn cus(&self) -> f64 { self.0.cus }
    /// Argument of latitude harmonic cosine correction, in radians.
    pub fn cuc(&self) -> f64 { self.0.cuc }
    /// Inclination harmonic sine correction, in radians.
    pub fn cis(&self) -> f64 { self.0.cis }
    /// Inclination harmonic cosine correction, in radians.
    pub fn cic(&self) -> f64 { self.0.cic }
    /// Time of ephemeris within the week, in seconds.
    pub fn toes(&self) -> f64 { self.0.toes }
    /// Fit interval, in hours.
    pub fn fit_interval(&self) -> f64 { self.0.fit }
    /// Clock bias, in seconds.
    pub fn clock_bias(&self) -> f64 { self.0.f0 }
    /// Clock drift, in s/s.
    pub fn clock_drift(&self) -> f64 { self.0.f1 }
    /// Clock drift rate, in s/s².
    pub fn clock_drift_rate(&self) -> f64 { self.0.f2 }
    /// Group delay parameters, in seconds.
    /// GPS/QZS index 0 is TGD. GAL indices 0-1 are BGD_E1E5a and BGD_E1E5b.
    /// BDS indices 0-5 are TGD_B1I, TGD_B2I, TGD_B1Cp, TGD_B2ap, ISC_B1Cd, ISC_B2ad.
    pub fn group_delay(&self) -> &[f64; 6] { &self.0.tgd }
    /// Semi-major axis rate of change for CNAV, in m/s.
    pub fn semi_major_axis_rate(&self) -> f64 { self.0.Adot }
    /// Mean motion rate for CNAV, in rad/s².
    pub fn mean_motion_rate(&self) -> f64 { self.0.ndot }
}

/// GLONASS satellite generation, from the M field of the status flags.
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum GloSatelliteType {
    Glonass = 0,
    GlonassM = 1,
    GlonassK1 = 3,
}

/// GLONASS broadcast ephemeris record.
///
/// Transparent wrapper around the FFI `geph_t` struct.
#[repr(transparent)]
pub struct GloEph(ffi::geph_t);

impl GloEph {
    /// Satellite number in RTKLIB internal numbering.
    pub fn sat(&self) -> i32 { self.0.sat }
    /// Issue of data.
    pub fn iode(&self) -> i32 { self.0.iode }
    /// Frequency channel number.
    pub fn frequency_number(&self) -> i32 { self.0.frq }
    /// Raw SV health field. Use [`is_healthy`](Self::is_healthy) for a simple check.
    pub fn svh(&self) -> i32 { self.0.svh }
    /// Returns true if the SV health field indicates no issues.
    pub fn is_healthy(&self) -> bool { self.0.svh == 0 }
    /// Raw status flags field.
    pub fn raw_flags(&self) -> i32 { self.0.flags }
    /// String type identifier. Flags P field in bits 0-1.
    pub fn p_string_type(&self) -> u8 { (self.0.flags & 0x3) as u8 }
    /// Time interval between adjacent tb values, in minutes. Flags P1 field in bits 2-3.
    pub fn p1_interval_minutes(&self) -> u8 {
        match (self.0.flags >> 2) & 0x3 {
            0 => 0,
            1 => 30,
            2 => 45,
            _ => 60,
        }
    }
    /// Odd/even flag. Flags P2 field in bit 4.
    pub fn p2_odd(&self) -> bool { (self.0.flags >> 4) & 1 != 0 }
    /// True if the current frame contains 5 almanac satellites. Flags P3 field in bit 5.
    pub fn p3_flag(&self) -> bool { (self.0.flags >> 5) & 1 != 0 }
    /// True if ephemeris has been updated. Flags P4 field in bit 6.
    pub fn p4_updated(&self) -> bool { (self.0.flags >> 6) & 1 != 0 }
    /// Satellite generation. Flags M field in bits 7-8.
    pub fn satellite_type(&self) -> Option<GloSatelliteType> {
        GloSatelliteType::try_from((self.0.flags >> 7) as u8 & 0x3).ok()
    }
    /// SV accuracy index.
    pub fn sva(&self) -> i32 { self.0.sva }
    /// Age of operation, in days.
    pub fn age(&self) -> i32 { self.0.age }
    /// Epoch of ephemeris.
    pub fn toe(&self) -> GpsTime { GpsTime(self.0.toe) }
    /// Message frame time.
    pub fn tof(&self) -> GpsTime { GpsTime(self.0.tof) }
    /// Satellite position in ECEF, in meters.
    pub fn position(&self) -> &[f64; 3] { &self.0.pos }
    /// Satellite velocity in ECEF, in m/s.
    pub fn velocity(&self) -> &[f64; 3] { &self.0.vel }
    /// Satellite acceleration in ECEF, in m/s².
    pub fn acceleration(&self) -> &[f64; 3] { &self.0.acc }
    /// SV clock bias, in seconds.
    pub fn clock_bias(&self) -> f64 { self.0.taun }
    /// Relative frequency bias.
    pub fn freq_bias(&self) -> f64 { self.0.gamn }
    /// L1/L2 inter-frequency delay, in seconds.
    pub fn l1_l2_delay(&self) -> f64 { self.0.dtaun }
}

/// SBAS satellite ephemeris record.
///
/// Transparent wrapper around the FFI `seph_t` struct.
#[repr(transparent)]
pub struct SbasEph(ffi::seph_t);

impl SbasEph {
    /// Satellite number in RTKLIB internal numbering.
    pub fn sat(&self) -> i32 { self.0.sat }
    /// Reference epoch time.
    pub fn t0(&self) -> GpsTime { GpsTime(self.0.t0) }
    /// Message frame time.
    pub fn tof(&self) -> GpsTime { GpsTime(self.0.tof) }
    /// SV accuracy index (URA).
    pub fn sva(&self) -> i32 { self.0.sva }
    /// Raw SV health field. Use [`is_healthy`](Self::is_healthy) for a simple check.
    pub fn svh(&self) -> i32 { self.0.svh }
    /// Returns true if the SV health field indicates no issues.
    pub fn is_healthy(&self) -> bool { self.0.svh == 0 }
    /// Satellite position in ECEF, in meters.
    pub fn position(&self) -> &[f64; 3] { &self.0.pos }
    /// Satellite velocity in ECEF, in m/s.
    pub fn velocity(&self) -> &[f64; 3] { &self.0.vel }
    /// Satellite acceleration in ECEF, in m/s².
    pub fn acceleration(&self) -> &[f64; 3] { &self.0.acc }
    /// Clock offset, in seconds.
    pub fn clock_offset(&self) -> f64 { self.0.af0 }
    /// Clock drift, in s/s.
    pub fn clock_drift(&self) -> f64 { self.0.af1 }
}

/// Navigation data store.
///
/// Wraps the FFI `nav_t` struct. Holds ephemeris and correction tables
/// populated by format decoders.
#[repr(transparent)]
pub struct Nav(ffi::nav_t);

impl Nav {
    /// GPS/Galileo/BeiDou/QZSS ephemeris records.
    pub fn eph(&self) -> &[Eph] {
        let n = self.0.n as usize;
        if n == 0 || self.0.eph.is_null() { return &[]; }
        unsafe { from_raw_parts(self.0.eph as *const Eph, n) }
    }

    /// GLONASS ephemeris records.
    pub fn glo_eph(&self) -> &[GloEph] {
        let n = self.0.ng as usize;
        if n == 0 || self.0.geph.is_null() { return &[]; }
        unsafe { from_raw_parts(self.0.geph as *const GloEph, n) }
    }

    /// SBAS ephemeris records.
    pub fn sbas_eph(&self) -> &[SbasEph] {
        let n = self.0.ns as usize;
        if n == 0 || self.0.seph.is_null() { return &[]; }
        unsafe { from_raw_parts(self.0.seph as *const SbasEph, n) }
    }

    /// GPS ionospheric correction parameters, 8 values.
    pub fn ion_gps(&self) -> &[f64; 8] { &self.0.ion_gps }

    /// Galileo ionospheric correction parameters, 4 values.
    pub fn ion_gal(&self) -> &[f64; 4] { &self.0.ion_gal }

    /// QZSS ionospheric correction parameters, 8 values.
    pub fn ion_qzs(&self) -> &[f64; 8] { &self.0.ion_qzs }

    /// BeiDou ionospheric correction parameters, 8 values.
    pub fn ion_cmp(&self) -> &[f64; 8] { &self.0.ion_cmp }

    /// NavIC ionospheric correction parameters, 8 values.
    pub fn ion_irn(&self) -> &[f64; 8] { &self.0.ion_irn }

    /// GLONASS frequency channel assignments, indexed by slot number.
    pub fn glo_fcn(&self) -> &[i32; 32] { &self.0.glo_fcn }
}
