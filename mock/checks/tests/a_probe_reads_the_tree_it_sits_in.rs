//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The tenth instrument-defect class, and the quietest of the ten.
//!
//! A committed probe that names an absolute path to somebody's checkout does
//! not fail elsewhere. If the checkout exists on the host it succeeds and
//! reports about a different tree, which is what makes this worse than a
//! broken probe: the probe keeps working, on the wrong subject, and no output
//! says which tree was read.
//!
//! Measured when the class was found: 31 committed probe scripts reaching into
//! another clone of this repository, 20 of them citation checkers, and that
//! clone exists on this host.
//!
//! **Measured again by this arm, which looks for any home-anchored path rather
//! than for that one clone: 69 files.** Every occurrence names the same
//! workspace root, so the wider pattern is the same defect reaching further
//! than the pattern that found it, which is the ordinary shape here and the
//! reason a class is measured rather than counted from the report that raised
//! it.
//!
//! There is a second thing wrong with those 31 and it is not about
//! reproducibility. The clone they read belongs to somebody else, and a session
//! works in a workspace of its own; a committed probe that reaches into another
//! one does it every time anybody runs it, without deciding to.

use std::fs;

use arvo_checks::corpus;

/// The catalogue, and it is red on purpose.
///
/// The 31 sit in landed probe directories, which are the record and are not
/// edited. Repairing them means re-running each against the tree it should
/// have read, which changes what they establish and is a dispatch rather than
/// an edit. What this pins in the meantime is that nobody writes a new one:
/// the ceiling is the measurement, and raising it is not the fix for a failure.
#[test]
fn no_new_probe_reads_another_tree() {
    /// Measured by this arm over the committed tree. Lower it as probes are
    /// repaired; never raise it.
    const CEILING: usize = 69;

    let found = corpus::probes_reading_another_tree(&corpus::panel_dir());
    let mut files: Vec<&str> = found.iter().map(|f| f.at.as_str()).collect();
    files.sort();
    files.dedup();
    assert!(
        files.len() <= CEILING,
        "{} probe scripts name a path outside this repository, against a ceiling of \
         {CEILING}. A new one has been written: resolve its root from its own location \
         rather than raising this number. Per file: {files:#?}",
        files.len()
    );
}

/// The control, and the reason the ceiling test is not a way of saying the
/// directory exists.
#[test]
fn the_arm_fires_on_a_planted_script_and_not_on_a_clean_one() {
    let tmp = tempdir();
    fs::create_dir_all(tmp.join("42_probes")).unwrap();
    fs::write(
        tmp.join("42_probes/reads_elsewhere.sh"),
        "#!/bin/sh\nroot=/Users/somebody/Dev/other-clone/arvo\ngrep -r x \"$root\"\n",
    )
    .unwrap();
    fs::write(
        tmp.join("42_probes/reads_itself.sh"),
        "#!/bin/sh\nroot=$(cd \"$(dirname \"$0\")/../../..\" && pwd)\ngrep -r x \"$root\"\n",
    )
    .unwrap();
    // Prose naming a path is not a script and is not this arm's business: the
    // corpus records what it ran, and a finding there would be a finding about
    // the audit trail doing its job.
    fs::write(
        tmp.join("42_notes.md"),
        "It was run against `/Users/somebody/Dev/other-clone/arvo`.\n",
    )
    .unwrap();

    let found = corpus::probes_reading_another_tree(&tmp);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("reads_elsewhere.sh"),
        "the script resolving its own root must not be reported: {}",
        found[0].at
    );
    assert!(
        found[0].says.contains("/Users/somebody/Dev/other-clone/arvo"),
        "the report names the path so it can be fixed without opening the file: {}",
        found[0].says
    );

    fs::remove_dir_all(&tmp).ok();
}

/// A relative path is a different and much louder failure, so it stays out.
#[test]
fn a_relative_path_is_not_reported() {
    let tmp = tempdir();
    fs::create_dir_all(tmp.join("43_probes")).unwrap();
    fs::write(
        tmp.join("43_probes/relative.py"),
        "open('../../../mock/registry/dimension.toml')\n",
    )
    .unwrap();
    assert!(
        corpus::probes_reading_another_tree(&tmp).is_empty(),
        "a relative path fails loudly where it is wrong, which is the opposite problem"
    );
    fs::remove_dir_all(&tmp).ok();
}

fn tempdir() -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "arvo-checks-tree-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    fs::create_dir_all(&base).unwrap();
    base
}
