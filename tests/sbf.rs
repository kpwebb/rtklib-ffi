//! Smoke tests for the Septentrio SBF decoder.
#![cfg(feature = "receivers")]

#[cfg(feature = "conv")]
use rtklib_ffi::conv::{convrnx, RnxOpt, RnxOutputFiles, StreamFmt};
use rtklib_ffi::receiver::{DecodeStatus, SbfDecoder};

#[test]
fn decode_sbf_all_blocks() {
    let data = std::fs::read("tests/all_blocks_0000.sbf").expect("failed to read test file");
    let mut decoder = SbfDecoder::try_new().expect("failed to init SBF decoder");

    // all_blocks_0000.sbf has empty MeasEpoch blocks (n1=0), so no observations
    // are decoded. It does contain GPSNav and GALNav blocks.
    let mut eph_count = 0u32;

    for &byte in &data {
        let Some(status) = decoder.decode(byte) else {
            continue;
        };
        match status {
            DecodeStatus::Ephemeris => eph_count += 1,
            _ => {}
        }
    }

    assert!(eph_count > 0, "expected at least one ephemeris message");
}

#[cfg(feature = "conv")]
#[test]
fn convrnx_sbf_to_rinex() {
    let obs_path = std::env::temp_dir().join("rtklib_ffi_test_sbf.obs");
    let ofiles = RnxOutputFiles::new(&obs_path);
    let result = convrnx(StreamFmt::Sbf, &mut RnxOpt::default(), "tests/log_0000.sbf", &ofiles);

    assert!(result.is_ok(), "convrnx failed: {result:?}");
    let meta = std::fs::metadata(&obs_path).expect("output file not created");
    assert!(meta.len() > 0, "output file is empty");
    let _ = std::fs::remove_file(&obs_path);
}
