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

use std::path::{Path, PathBuf};
use std::process::Command;

/// The crate's directory in the checkout this run was started from.
fn the_crate_directory() -> PathBuf {
    let dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .expect("cargo sets `CARGO_MANIFEST_DIR` for the tests it runs");
    PathBuf::from(dir)
}

/// The scratch directory of the build directory this test binary sits in.
fn the_scratch_directory() -> PathBuf {
    let binary = std::env::current_exe().expect("the test binary has a path");
    let build = Path::ancestors(&binary)
        .nth(3)
        .expect("a test binary sits in `<build directory>/<profile>/deps`");
    build.join("tmp")
}

#[test]
fn the_paths_are_the_ones_this_run_was_given() {
    let dir = the_crate_directory();
    let cwd = std::env::current_dir().expect("the run has a working directory");
    assert_eq!(dir, cwd, "cargo runs a test in its crate's directory");
    assert!(dir.join("tests/consumer/Cargo.toml").is_file());
    assert!(the_scratch_directory().is_dir());
}

#[test]
fn a_consumer_with_no_feature_line_builds_and_runs() {
    let manifest = the_crate_directory().join("tests/consumer/Cargo.toml");
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
