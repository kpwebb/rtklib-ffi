//! Tests for RTCM3 decoding via the safe `RtcmDecoder` wrapper.
//!
//! Feeds raw RTCM3 bytes from `tests/debug.rtcm` through `input_rtcm3`
//! one byte at a time. The file contains GPS and Galileo ephemeris messages
//! alongside GPS and Galileo MSM7 observation messages.
#![cfg(feature = "rtcm")]

use rtklib_ffi::rtcm::{DecodeResult, MsgType, RtcmDecoder};
use rtklib_ffi::{satno, NavSys};

#[test]
fn decode_rtcm3_ephemeris() {
    let data = std::fs::read("tests/debug.rtcm").expect("failed to read test file");
    let mut decoder = RtcmDecoder::try_new().expect("failed to init RTCM decoder");

    let mut ephemeris_count = 0u32;
    let mut msg_types: Vec<MsgType> = Vec::new();
    let mut gps_sats: Vec<u8> = Vec::new();
    let mut gal_sats: Vec<u8> = Vec::new();
    let mut prev_obs_n = 0usize;

    for &byte in &data {
        let Some(status) = decoder.decode(byte) else { continue; };
        match status {
            DecodeResult::Incomplete => {
                // MSM7 messages with sync=1 return Incomplete but still
                // store observations in the internal buffer. Detect this
                // by watching `observation_count` change.
                let n = decoder.observation_count();
                if n != prev_obs_n && n > 0 {
                    prev_obs_n = n;
                    let sats: Vec<u8> = decoder.observations().iter().map(|o| o.sat()).collect();
                    match decoder.message_type() {
                        Some(MsgType::GpsMsm7) => gps_sats = sats,
                        Some(MsgType::GalMsm7) => gal_sats = sats,
                        _ => {}
                    }
                }
            }
            DecodeResult::Ephemeris => {
                let mt = decoder.message_type().expect("unknown message type");
                ephemeris_count += 1;
                msg_types.push(mt);
            }
            _ => {
                if let Some(mt) = decoder.message_type() {
                    msg_types.push(mt);
                }
            }
        }
    }

    // The file contains GPS and Galileo ephemeris messages alongside
    // GPS and Galileo MSM7 messages with sync=1 that don't produce
    // a complete observation epoch.
    assert!(
        ephemeris_count >= 2,
        "expected at least 2 ephemeris messages"
    );
    assert!(
        msg_types.contains(&MsgType::GpsEphemeris),
        "expected GPS ephemeris"
    );
    assert!(
        msg_types.contains(&MsgType::GalInavEphemeris),
        "expected Galileo ephemeris"
    );

    assert_eq!(gps_sats.len(), 7, "expected 7 GPS observations");
    assert_eq!(
        gps_sats[0],
        satno(NavSys::Gps, 10).expect("GPS PRN 10 out of range"),
        "expected first GPS sat to be PRN 10"
    );
    assert_eq!(gal_sats.len(), 6, "expected 6 Galileo observations");
    assert_eq!(
        gal_sats[0],
        satno(NavSys::Gal, 4).expect("Galileo PRN 4 out of range"),
        "expected first Galileo sat to be PRN 4"
    );
}
