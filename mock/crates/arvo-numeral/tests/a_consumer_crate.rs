//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A crate outside this workspace, depending on `arvo-numeral` with no feature
//! line, builds and runs while reaching every public item.
//!
//! It is built by its own cargo run into this run's scratch directory inside
//! the build directory, because the run that executes this test holds the
//! build directory's lock for as long as the test lasts.
//!
//! Both paths are taken when the test runs. A build directory shared between
//! checkouts of this repository hands one test binary to all of them, so a
//! path read at build time would name the checkout that built it first.

use std::path::PathBuf;
use std::process::Command;

/// The crate's directory in the checkout this run was started from.
fn the_crate_directory() -> PathBuf {
    let dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .expect("cargo sets `CARGO_MANIFEST_DIR` for the tests it runs");
    PathBuf::from(dir)
}

/// The scratch directory of the build directory this test binary sits under.
///
/// Cargo writes `CACHEDIR.TAG` at a build directory's root and nowhere below
/// it, so the nearest ancestor holding one is the root whether or not the
/// build named a target.
fn the_scratch_directory() -> PathBuf {
    let binary = std::env::current_exe().expect("the test binary has a path");
    let root = binary
        .ancestors()
        .find(|dir| dir.join("CACHEDIR.TAG").is_file())
        .expect("a test binary sits under a build directory cargo has tagged");
    root.join("tmp")
}

/// The line `the_crate_directory_is_read_when_the_test_runs` reads back.
const RESOLVED: &str = "resolved crate directory: ";

/// Prints the crate directory as this process resolves it. Run on its own by
/// the test below, in a process whose environment names another directory.
#[test]
fn the_crate_directory_as_resolved() {
    println!("{RESOLVED}{}", the_crate_directory().display());
}

/// Runs this binary again with another crate directory in its environment and
/// reads back the one it resolved. A path fixed when the binary was built would
/// come back as this checkout's, whatever the environment says.
#[test]
fn the_crate_directory_is_read_when_the_test_runs() {
    let elsewhere = the_scratch_directory().join("a-checkout-that-is-not-this-one");
    let binary = std::env::current_exe().expect("the test binary has a path");
    let out = Command::new(binary)
        .args(["--exact", "the_crate_directory_as_resolved", "--nocapture"])
        .env("CARGO_MANIFEST_DIR", &elsewhere)
        .output()
        .expect("the test binary could not be started again");
    assert!(out.status.success(), "the second run failed");
    let printed = String::from_utf8_lossy(&out.stdout);
    let resolved = printed
        .lines()
        .find_map(|line| line.strip_prefix(RESOLVED))
        .expect("the second run printed no resolved directory");
    assert_eq!(PathBuf::from(resolved), elsewhere);
    assert_ne!(elsewhere, the_crate_directory());
}

#[test]
fn a_consumer_with_no_feature_line_builds_and_runs() {
    let manifest = the_crate_directory().join("tests/consumer/Cargo.toml");
    assert!(manifest.is_file(), "no consumer crate at {manifest:?}");
    let target = the_scratch_directory().join("consumer");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let out = Command::new(cargo)
        .args(["run", "--quiet", "--locked", "--offline", "--manifest-path"])
        .arg(&manifest)
        .arg("--target-dir")
        .arg(&target)
        .output()
        .expect("cargo could not be started");
    assert!(
        out.status.success(),
        "the consumer crate did not build and run:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
