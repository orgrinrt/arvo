//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the note lint fires on and what it lets through.
//!
//! Every arm here is a pair: the surface without the note, which has to fire,
//! and the same surface with the note, which has to be silent. A lint that only
//! ever fires is as useless as one that never does, and the second half is what
//! says which of the two this is.

use std::path::Path;

use super::{HalfUpCarriesItsNote, NAME, check};
use crate::canon_lint_testkit::{
    assert_findings_block_at,
    assert_not_declared_off,
    assert_registered,
    ctx_at,
    plant,
    planted_tree,
    view,
};

/// The note as a page would carry it, in the shortest form that says the thing.
const NOTE: &str = "This is not the `HALF_UP` of Java or Python: a tie at -2.5 \
                    goes to -2.";

/// A sentence stating the denotation, which is what a doc comment owes the note
/// for.
const DENOTES: &str = "`half_up` sends a tie toward positive infinity.";

/// What `check` says about one planted tree, as the paths and lines it names.
fn over(dir: &Path) -> Vec<String> {
    check(dir, &dir.join("mock"))
        .into_iter()
        .map(|e| format!("{}:{}", e.path.unwrap_or_default(), e.line))
        .collect()
}

#[test]
fn the_root_readme_owes_the_note_wherever_it_names_the_mode() {
    let dir = planted_tree("note-readme-bare");
    plant(
        &dir,
        "README.md",
        "# arvo\n\nThe modes include `half_up`.\n",
    );
    assert_eq!(
        over(&dir),
        vec!["README.md:3".to_string()],
        "the root README is where a reader who has never seen this project \
         arrives, and it is where the note was missing"
    );
}

#[test]
fn the_root_readme_carrying_the_note_is_silent() {
    let dir = planted_tree("note-readme-noted");
    plant(
        &dir,
        "README.md",
        &format!("# arvo\n\nThe modes include `half_up`. {NOTE}\n"),
    );
    assert!(
        over(&dir).is_empty(),
        "the note is what is asked for, so carrying it is the whole obligation"
    );
}

#[test]
fn a_readme_naming_no_mode_is_silent_so_the_fire_above_is_about_the_name() {
    let dir = planted_tree("note-readme-unrelated");
    plant(
        &dir,
        "README.md",
        "# arvo\n\nThe modes include `floor` and `ceil`.\n",
    );
    assert!(
        over(&dir).is_empty(),
        "a page that never names the mode owes nothing about it, and without \
         this the first arm would only be saying that the file exists"
    );
}

#[test]
fn a_template_naming_the_mode_owes_the_note_and_carrying_it_clears() {
    let bare = planted_tree("note-tmpl-bare");
    plant(
        &bare,
        "mock/crates/a/README.md.tmpl",
        "Rounding is `half_up` by default.\n",
    );
    assert_eq!(over(&bare), vec!["crates/a/README.md.tmpl:1".to_string()]);

    let noted = planted_tree("note-tmpl-noted");
    plant(
        &noted,
        "mock/crates/a/README.md.tmpl",
        &format!("Rounding is `half_up` by default. {NOTE}\n"),
    );
    assert!(over(&noted).is_empty());
}

#[test]
fn a_doc_comment_stating_the_denotation_owes_the_note() {
    let dir = planted_tree("note-doc-bare");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        &format!("/// {DENOTES}\npub fn f() {{}}\n"),
    );
    assert_eq!(
        over(&dir),
        vec!["crates/a/src/lib.rs:1".to_string()],
        "rustdoc prints each doc comment as a page of its own, and a reader \
         reaches one without passing the crate's front page"
    );
}

#[test]
fn a_doc_comment_carrying_the_note_is_silent() {
    let dir = planted_tree("note-doc-noted");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        &format!("/// {DENOTES}\n///\n/// {NOTE}\npub fn f() {{}}\n"),
    );
    assert!(over(&dir).is_empty());
}

#[test]
fn a_doc_comment_naming_the_mode_without_stating_what_it_denotes_is_silent() {
    let dir = planted_tree("note-doc-mention");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        "/// Builds the table `half_up` reads its carry out of.\n\
         pub fn f() {}\n",
    );
    assert!(
        over(&dir).is_empty(),
        "a passage mentioning the mode while discussing something else is not \
         telling anybody what the name means, and demanding the note there \
         would put it on every line that says the word"
    );
}

#[test]
fn a_doc_comment_stating_the_denotation_of_something_else_is_silent() {
    let dir = planted_tree("note-doc-other-mode");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        "/// `ceil` sends every value toward positive infinity.\npub fn f() {}\n",
    );
    assert!(
        over(&dir).is_empty(),
        "the words that state this denotation state another mode's whole \
         behaviour, so the name has to be there too or the lint fires on ceil"
    );
}

#[test]
fn the_item_under_an_outer_block_names_the_mode_for_it() {
    let dir = planted_tree("note-doc-subject");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        "/// Sends a tie toward positive infinity.\npub fn half_up() {}\n",
    );
    assert_eq!(
        over(&dir),
        vec!["crates/a/src/lib.rs:1".to_string()],
        "rustdoc prints the block under the item's name, so the page names the \
         mode even where the prose does not"
    );
}

#[test]
fn a_plain_comment_is_not_a_page_and_owes_nothing() {
    let dir = planted_tree("note-plain-comment");
    plant(
        &dir,
        "mock/crates/a/src/lib.rs",
        &format!("// {DENOTES}\npub fn f() {{}}\n"),
    );
    assert!(
        over(&dir).is_empty(),
        "rustdoc does not print it, so nobody arrives at it from outside; the \
         magnitude lint is what reads a plain comment"
    );
}

#[test]
fn the_record_and_the_build_are_not_a_published_surface() {
    for at in [
        "mock/research/a.md.tmpl",
        "mock/design_rounds/a.md.tmpl",
        "mock/target/a.md.tmpl",
        "mock/agent/a.md.tmpl",
        "mock/.hidden/a.md.tmpl",
    ] {
        let dir = planted_tree(&format!("note-skips-{}", at.replace('/', "-")));
        plant(&dir, at, "Rounding is `half_up` by default.\n");
        assert!(
            over(&dir).is_empty(),
            "{at} is the record, the build, or an agent surface, and none of \
             those is a page a stranger lands on"
        );
    }
}

#[test]
fn a_program_this_repository_runs_is_not_a_page_either() {
    let dir = planted_tree("note-lint-source");
    plant(
        &dir,
        "mock/lints/a.rs",
        &format!("//! {DENOTES}\npub fn f() {{}}\n"),
    );
    assert!(
        over(&dir).is_empty(),
        "a lint, a tool or a bench is a program rather than a published crate, \
         and its own prose is read by whoever opens the file"
    );
}

#[test]
fn a_crates_own_test_file_is_not_a_page_either() {
    let dir = planted_tree("note-crate-test");
    plant(
        &dir,
        "mock/crates/a/tests/a.rs",
        &format!("/// {DENOTES}\nfn f() {{}}\n"),
    );
    assert!(
        over(&dir).is_empty(),
        "rustdoc renders a crate's `tests/` no more than it renders a lint, so \
         a doc comment on a test helper is read by whoever opens the file and \
         by nobody arriving from outside"
    );
}

#[test]
fn every_published_surface_in_this_repository_carries_it() {
    let root = crate::canon_lint_testkit::repo_root();
    let found = check(&root, &root.join("mock"));
    assert!(
        found.is_empty(),
        "a surface here names the mode or states the denotation without the \
         note: {:?}",
        found
            .iter()
            .map(|e| format!("{}:{}", e.path.clone().unwrap_or_default(), e.line))
            .collect::<Vec<_>>()
    );
}

#[test]
fn its_findings_block_every_gate() {
    let dir = planted_tree("note-severity");
    plant(&dir, "mock/crates/a/README.md.tmpl", "It is `half_up`.\n");
    let empty = view(&[], &[]);
    assert_findings_block_at(&HalfUpCarriesItsNote, &ctx_at(&dir.join("mock"), &empty));
}

#[test]
fn it_is_not_declared_off_so_it_runs_at_all() {
    assert_not_declared_off(&HalfUpCarriesItsNote);
}

#[test]
fn it_reaches_the_pack_the_engine_is_handed() {
    assert_registered(NAME);
}
