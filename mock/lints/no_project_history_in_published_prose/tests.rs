//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the history lint fires on, what it lets through, and whether the
//! carve-outs it ships still name sentences that are in the tree.
//!
//! The last of those is the arm that matters most. A carve-out is a claim about
//! one sentence in one file, and a claim about live data that nothing checks is
//! how the sibling lint's registry argument came to be true only by accident.
//! So the sentences are asserted present rather than trusted, and a carve-out
//! whose sentence has been edited away is a failure here rather than a silent
//! widening of what the lint permits.

use std::path::{Path, PathBuf};

use super::{EXCUSED, NAME, NoProjectHistoryInPublishedProse, PHRASES, check};
use crate::canon_lint_testkit::{
    assert_findings_block_at,
    assert_not_declared_off,
    assert_registered,
    ctx_at,
    plant,
    planted_tree,
    view,
};

/// What `check` says about one planted tree, as the paths and lines it names.
fn over(dir: &Path) -> Vec<String> {
    check(dir, &dir.join("mock"))
        .into_iter()
        .map(|e| format!("{}:{}", e.path.unwrap_or_default(), e.line))
        .collect()
}

/// The repository this lint actually guards, found from this file's own path so
/// the arms below read the real tree rather than a planted one.
fn repo_root() -> PathBuf {
    let here = Path::new(file!());
    let mut dir = if here.is_absolute() {
        here.to_path_buf()
    } else {
        std::env::current_dir()
            .expect("a working directory")
            .join(here)
    };
    // `<root>/mock/lints/<lint>/tests.rs` is four components down.
    for _ in 0 .. 4 {
        dir.pop();
    }
    dir
}

#[test]
fn every_carve_out_still_names_a_sentence_that_is_in_the_file_it_names() {
    // The arm this lint exists to make possible, run against the real tree. A
    // carve-out is a claim about live data, and the reason it is written out in
    // full beside the sentence is so a reader meeting this failure can decide
    // whether the sentence moved or the exemption stopped being earned.
    let root = repo_root();
    let mut missing = Vec::new();
    for e in EXCUSED {
        let path = if e.file == "README.md" {
            root.join("README.md")
        } else {
            root.join("mock").join(e.file)
        };
        match std::fs::read_to_string(&path) {
            Ok(text) if text.contains(e.sentence) => {},
            Ok(_) => missing.push(format!("{}: {:?} is not in the file", e.file, e.sentence)),
            Err(err) => missing.push(format!("{}: {err}", e.file)),
        }
    }
    assert!(
        missing.is_empty(),
        "a carve-out names a sentence the tree no longer has, so it now excuses \
         nothing and hides whatever else is on those lines:\n{}",
        missing.join("\n")
    );
}

#[test]
fn every_carve_out_carries_a_reason_a_reader_can_weigh() {
    // A carve-out with no reason is an exemption nobody can argue with, which is
    // the shape this lint was written to refuse in prose.
    for e in EXCUSED {
        assert!(
            e.because.split_whitespace().count() >= 20,
            "{}: {:?} is excused in {} words",
            e.file,
            e.sentence,
            e.because.split_whitespace().count()
        );
    }
}

#[test]
fn the_real_tree_is_clean_of_everything_that_is_not_excused() {
    // The lint over the repository it guards. Green here says the two instances
    // the review found are gone and nothing else on the surface carries the
    // shape, which is a claim about the tree rather than about a fixture.
    let root = repo_root();
    let found = check(&root, &root.join("mock"));
    let shown: Vec<String> = found
        .iter()
        .map(|e| format!("{}:{}", e.path.clone().unwrap_or_default(), e.line))
        .collect();
    assert!(shown.is_empty(), "{}", shown.join("\n"));
}

#[test]
fn every_phrase_fires_in_a_document_and_in_rustdoc() {
    // Walked whole rather than sampled: a phrase in the list that nothing
    // matches is a phrase that was never going to catch anything.
    for p in PHRASES {
        let dir = planted_tree("history-phrase");
        plant(
            &dir,
            "mock/crates/a/DESIGN.md.tmpl",
            &format!("# A\n\nThe map is total, and it {p} be partial.\n"),
        );
        plant(
            &dir,
            "mock/crates/a/src/lib.rs",
            &format!("//! The map is total, and it {p} be partial.\n"),
        );
        assert_eq!(
            over(&dir),
            vec!["crates/a/DESIGN.md.tmpl:3".to_string(), "crates/a/src/lib.rs:1".to_string(),],
            "{p}"
        );
    }
}

#[test]
fn control_the_same_sentences_without_the_phrase_are_silent() {
    let dir = planted_tree("history-clean");
    plant(
        &dir,
        "mock/crates/a/DESIGN.md.tmpl",
        "# A\n\nThe map is total, and it cannot be partial.\n",
    );
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        "//! The map is total, and it cannot be partial.\n",
    );
    assert!(over(&dir).is_empty());
}

#[test]
fn the_root_readme_is_read() {
    let dir = planted_tree("history-readme");
    plant(
        &dir,
        "README.md",
        "# arvo\n\nIt previously did otherwise.\n",
    );
    assert_eq!(over(&dir), vec!["README.md:3".to_string()]);
}

#[test]
fn a_plain_comment_is_not_a_published_surface_and_a_doc_comment_is() {
    // Rustdoc prints one and not the other, and a note to whoever opens the
    // file is where this material is allowed to live.
    let dir = planted_tree("history-plain");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        "// This previously did otherwise.\nfn f() {}\n",
    );
    assert!(over(&dir).is_empty());

    let doc = planted_tree("history-doc");
    plant(
        &doc,
        "mock/crates/a/src/lib.rs",
        "/// This previously did otherwise.\nfn f() {}\n",
    );
    assert_eq!(over(&doc), vec!["crates/a/src/lib.rs:1".to_string()]);
}

#[test]
fn the_record_is_not_read_at_all() {
    // The design rounds, the research tree and the agent surfaces are where the
    // project's history belongs, so a phrase in one of them is not a finding.
    for at in [
        "mock/design_rounds/a.md",
        "mock/research/a.md",
        "mock/agent/a.md.tmpl",
        "mock/target/a.md.tmpl",
    ] {
        let dir = planted_tree("history-record");
        plant(&dir, at, "It previously did otherwise.\n");
        assert!(over(&dir).is_empty(), "{at}");
    }
}

#[test]
fn a_carve_out_covers_its_own_sentence_and_no_other_line_of_the_same_file() {
    // The property that keeps a carve-out from turning into a file-wide
    // exemption, which is what a phrase-list weakening would have been.
    let dir = planted_tree("history-carve");
    plant(
        &dir,
        "mock/DESIGN.md.tmpl",
        "# A\n\nThe consumers named there used crates that no longer exist.\n\nAnd it \
         previously did otherwise.\n",
    );
    assert_eq!(
        over(&dir),
        vec!["DESIGN.md.tmpl:5".to_string()],
        "the excused sentence is excused and the second one is not"
    );
}

#[test]
fn its_findings_block_every_gate() {
    let dir = planted_tree("history-severity");
    plant(&dir, "mock/crates/a/DESIGN.md.tmpl", "It previously did.\n");
    let empty = view(&[], &[]);
    assert_findings_block_at(
        &NoProjectHistoryInPublishedProse,
        &ctx_at(&dir.join("mock"), &empty),
    );
}

#[test]
fn it_is_not_declared_off_so_it_runs_at_all() {
    assert_not_declared_off(&NoProjectHistoryInPublishedProse);
}

#[test]
fn it_reaches_the_pack_the_engine_is_handed() {
    assert_registered(NAME);
}
