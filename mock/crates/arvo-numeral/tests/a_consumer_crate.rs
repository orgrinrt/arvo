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

use std::path::Path;
use std::process::Command;

#[test]
fn a_consumer_with_no_feature_line_builds_and_runs() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/consumer/Cargo.toml");
    let target = Path::new(env!("CARGO_TARGET_TMPDIR")).join("consumer");
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
