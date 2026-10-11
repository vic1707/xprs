//! This file is used to configure the build process of XPRS.
/* Clippy config */
#![allow(
    clippy::expect_used,
    clippy::print_stdout,
    reason = "build scripts emit Cargo directives and must fail if rustc cannot be queried"
)]
/* Dependencies imports */
use rustc_version::{Channel, version_meta};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(nightly)");
    if version_meta().expect("Couldn't get Rust channel").channel
        == Channel::Nightly
    {
        println!("cargo::rustc-cfg=nightly");
    }
}
