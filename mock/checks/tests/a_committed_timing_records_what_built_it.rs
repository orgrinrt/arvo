//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A timing with no build profile is a measurement of nothing.
//!
//! The profile is a dimension like any other, and under a notation where an
//! unstated dimension claims nothing, a timing that does not name it holds
//! nowhere. The workspace rule that says so was written after a true finding
//! was retired as unreproducible: a crate measured at about 107 seconds, three
//! refutations at four to five, everybody right, and nobody had written down
//! that the first was a debug build and the rest were release. Measured back to
//! back afterwards: **a factor of 29.**
//!
//! The same rule asks for the tree the artifact came from. A meta recording a
//! dirty worktree cannot be tied to a commit, so the variant code behind the
//! number is whatever happened to be on disk.
//!
//! The harness records both when the tool runs it, and the committed corpus
//! predates that or was run around it. **So this is a catalogue rather than a
//! gate**: the figures are not wrong, they are unciteable as magnitudes, and
//! nothing but re-running them under the tool changes that. What the ceilings
//! pin is that the corpus does not grow another one.

use std::fs;
use std::path::PathBuf;

use arvo_checks::repo;

fn metas() -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(&repo().join("mock/benches"), &mut out);
    out.sort();
    out
}

fn walk(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.to_string_lossy().ends_with(".meta.json") {
            out.push(path);
        }
    }
}

/// Every committed timing artifact names the profile that produced it.
///
/// Zero of them do today, which is the whole corpus of measured cost in this
/// project. The ceiling is the count, so a new run under the tool lowers it and
/// a new run without the tool does not raise it.
#[test]
fn no_new_timing_lands_without_its_build_profile() {
    /// Measured over the committed tree. Lower it by re-running under the
    /// tool; never raise it.
    const WITHOUT_A_PROFILE: usize = 254;

    let all = metas();
    assert!(!all.is_empty(), "control: the bench tree holds artifacts to check");
    let bare: Vec<String> = all
        .iter()
        .filter(|p| {
            fs::read_to_string(p).is_ok_and(|t| !t.contains("\"build_profile\""))
        })
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
        .collect();
    assert!(
        bare.len() <= WITHOUT_A_PROFILE,
        "{} of {} committed timings name no build profile, against a ceiling of \
         {WITHOUT_A_PROFILE}. A timing that does not say what built it cannot be compared \
         with anything, so run it through the tool rather than raising this number.",
        bare.len(),
        all.len()
    );
}

/// Every committed timing names the commit it was taken at.
#[test]
fn no_new_timing_lands_from_a_dirty_tree() {
    /// Measured over the committed tree. Lower it; never raise it.
    const FROM_A_DIRTY_TREE: usize = 253;

    let all = metas();
    assert!(!all.is_empty(), "control: the bench tree holds artifacts to check");
    let dirty: Vec<String> = all
        .iter()
        .filter(|p| fs::read_to_string(p).is_ok_and(|t| t.contains("-dirty")))
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
        .collect();
    assert!(
        dirty.len() <= FROM_A_DIRTY_TREE,
        "{} of {} committed timings were taken from a dirty worktree, against a ceiling of \
         {FROM_A_DIRTY_TREE}. Nothing ties such a number to the variant code that produced \
         it. Commit first, then measure.",
        dirty.len(),
        all.len()
    );
}

/// The control, and the reason neither ceiling is a way of saying the directory
/// is empty.
///
/// Each arm counts a property, so each has to be shown able to count zero of it
/// as well as many. A synthetic meta carrying both properties, and one carrying
/// neither, are classified apart.
#[test]
fn the_two_properties_are_read_apart_on_planted_artifacts() {
    let complete = r#"{"cpu":"x","git_commit":"abc1234","build_profile":"opt-level=3,lto=\"fat\""}"#;
    let bare = r#"{"cpu":"x","git_commit":"abc1234-dirty"}"#;

    assert!(complete.contains("\"build_profile\""));
    assert!(!complete.contains("-dirty"));
    assert!(!bare.contains("\"build_profile\""));
    assert!(bare.contains("-dirty"));

    // And the arms are reading real files rather than an empty list, which is
    // the failure that would make both ceilings vacuous.
    let all = metas();
    assert!(
        all.len() > 100,
        "the arms above are counting over {} artifacts; a collapse here makes both ceilings \
         meaningless while both tests still pass",
        all.len()
    );
}
