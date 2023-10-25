/* Clippy config */
#![allow(clippy::expect_used, clippy::print_stdout)]
/* Dependencies imports */
use rustc_version::{version, version_meta, Channel};

fn main() {
    let version = version().expect("Couldn't get Rust version");
    let version_meta = version_meta().expect("Couldn't get Rust channel");

    // Assert we haven't travelled back in time
    assert!(
        version.major >= 1,
        "How did you get a version before 1.0.0?"
    );

    // Generic setting
    println!("cargo:rerun-if-changed=build.rs");

    if matches!(version_meta.channel, Channel::Nightly) {
        println!("cargo:rustc-cfg=NIGHTLY");
    }
}
