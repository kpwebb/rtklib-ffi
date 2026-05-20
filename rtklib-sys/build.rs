use std::path::PathBuf;
use std::{env, fs};

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!("The `{name}` directory is empty, did you forget to pull the submodules?");
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn main() {
    fail_on_empty_directory("rtklib/");

    let mut build = cc::Build::new();

    // Enable all constellations
    build.define("ENAGLO", None);
    build.define("ENAGAL", None);
    build.define("ENAQZS", None);
    build.define("ENACMP", None);
    build.define("ENAIRN", None);
    build.define("TRACE", None);
    // Activate the built-in stub implementations of GUI callbacks
    // (showmsg, settspan, settime) in rtkcmn.c
    build.define("DLL", None);

    // Core files (always compiled)
    build.file("rtklib/src/rtkcmn.c");
    build.file("rtklib/src/trace.c");
    build.file("rtklib/src/geoid.c");
    build.file("rtklib/src/datum.c");

    #[cfg(feature = "conv")]
    {
        build.file("rtklib/src/convrnx.c");
        build.file("rtklib/src/convkml.c");
        build.file("rtklib/src/convgpx.c");
    }

    #[cfg(feature = "gis")]
    {
        build.file("rtklib/src/gis.c");
    }

    #[cfg(feature = "net")]
    {
        build.file("rtklib/src/stream.c");
        build.file("rtklib/src/rtksvr.c");
        build.file("rtklib/src/streamsvr.c");
        build.file("rtklib/src/download.c");
        println!("cargo:rustc-link-lib=pthread");
    }

    // These files are needed by both ppk and conv. convrnx.c calls pntpos for
    // auto-position estimation; pntpos.c calls into preceph.c and ionex.c;
    // rinex.c and sbas.c are referenced unconditionally by convrnx.c.
    #[cfg(any(feature = "ppk", feature = "conv"))]
    {
        build.file("rtklib/src/rinex.c");
        build.file("rtklib/src/ephemeris.c");
        build.file("rtklib/src/sbas.c");
        build.file("rtklib/src/pntpos.c");
        build.file("rtklib/src/preceph.c");
        build.file("rtklib/src/ionex.c");
    }

    #[cfg(feature = "ppk")]
    {
        build.file("rtklib/src/postpos.c");
        build.file("rtklib/src/rtkpos.c");
        build.file("rtklib/src/lambda.c");
        build.file("rtklib/src/solution.c");
        build.file("rtklib/src/options.c");
        build.file("rtklib/src/ppp.c");
        build.file("rtklib/src/ppp_ar.c");
        build.file("rtklib/src/tides.c");
    }

    // All receiver format files must be compiled together with rcvraw.c.
    // rcvraw.c calls init, free, and input functions for every supported
    // format unconditionally - there are no #ifdef guards - so all format
    // .c files must be present in the same link unit whenever rcvraw.c is.
    #[cfg(feature = "receivers")]
    {
        build.file("rtklib/src/rcvraw.c");
        build.include("rtklib/src");
        build.file("rtklib/src/rcv/binex.c");
        build.file("rtklib/src/rcv/crescent.c");
        build.file("rtklib/src/rcv/javad.c");
        build.file("rtklib/src/rcv/novatel.c");
        build.file("rtklib/src/rcv/nvs.c");
        build.file("rtklib/src/rcv/rt17.c");
        build.file("rtklib/src/rcv/septentrio.c");
        build.file("rtklib/src/rcv/skytraq.c");
        build.file("rtklib/src/rcv/swiftnav.c");
        build.file("rtklib/src/rcv/ublox.c");
        build.file("rtklib/src/rcv/unicore.c");
        // comnav.c and tersus.c use satwavelen() and lam_carr[] which were
        // removed upstream; excluded until updated to use sat2freq().
        // build.file("rtklib/src/rcv/comnav.c");
        // build.file("rtklib/src/rcv/tersus.c");
    }

    // RTCM files are needed by ppk, rtcm, and conv.
    // PPK uses them for SSR corrections; conv uses them for RTCM input.
    #[cfg(any(feature = "ppk", feature = "rtcm", feature = "conv"))]
    {
        build.file("rtklib/src/rtcm.c");
        build.file("rtklib/src/rtcm2.c");
        build.file("rtklib/src/rtcm3.c");
        build.file("rtklib/src/rtcm3e.c");
    }

    #[cfg(feature = "tle")]
    {
        build.file("rtklib/src/tle.c");
    }

    #[cfg(unix)]
    println!("cargo:rustc-link-lib=m");

    build.opt_level_str(&env::var("OPT_LEVEL").unwrap());
    build.warnings(false);
    build.compile("rtklib");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=rtklib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("rtklib/src/rtklib.h")
        .clang_arg("-DENAGLO")
        .clang_arg("-DENAGAL")
        .clang_arg("-DENAQZS")
        .clang_arg("-DENACMP")
        .clang_arg("-DENAIRN")
        .clang_arg("-DTRACE")
        // Block constants that get duplicate definitions from math.h
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!(
        "cargo:cargo_manifest_dir={}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!("cargo:out_dir={}", env::var("OUT_DIR").unwrap());
}
