//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A crate outside this workspace, depending on `arvo-capacity` with no feature
//! line, builds and runs while sizing arrays the way hilavitkutin does.
//!
//! It is built by its own cargo run into this run's scratch directory inside
//! the build directory, because the run that executes this test holds the
//! build directory's lock for as long as the test lasts.
//!
//! Both paths are taken when the test runs. A build directory shared between
//! checkouts of this repository hands one test binary to all of them, so a
//! path read at build time would name the checkout that built it first.
//!
//! The harness is `arvo-numeral`'s, whose own test proves the crate directory
//! is read at run time; it is not run twice.
// FIXME(extract_the_consumer_crate_test_harness_shared_by_arvo_numer): one
// harness for both crates, once it can move out of `arvo-numeral`.

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

#[test]
fn a_consumer_with_no_feature_line_builds_and_runs() {
    let manifest = the_crate_directory().join("tests/consumer/Cargo.toml");
    assert!(manifest.is_file(), "no consumer crate at {manifest:?}");
    let target = the_scratch_directory().join("arvo-capacity-consumer");
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

/// The `source` line of the package `name` in a lockfile's text.
fn locked_source(lock: &str, name: &str) -> Option<String> {
    let header = format!("name = \"{name}\"");
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line == header {
            return lines
                .take_while(|l| !l.is_empty())
                .find_map(|l| l.strip_prefix("source = "))
                .map(str::to_owned);
        }
    }
    None
}

/// The consumer builds `--locked`, so its notko must be the one the workspace
/// builds, or the test checks this crate against a notko nothing else uses.
#[test]
fn the_consumer_locks_the_notko_the_workspace_builds() {
    let read = |path: PathBuf| {
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path:?} unreadable: {e}"))
    };
    let consumer = read(the_crate_directory().join("tests/consumer/Cargo.lock"));
    let workspace = read(the_crate_directory().join("../../Cargo.lock"));
    let pinned = locked_source(&consumer, "notko").expect("the consumer locks no notko");
    assert_eq!(Some(pinned), locked_source(&workspace, "notko"));
    assert_eq!(locked_source(&consumer, "no-such-crate"), None);
}
