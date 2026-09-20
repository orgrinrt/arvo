//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: a published surface naming `half_up` carries the note that travels
//! with the name.
//!
//! `ruling::half_up_denotes_ties_toward_positive_infinity` settles the
//! denotation and then says what has to accompany it: a note travels with the
//! name, as the floor note travels with the retired word, because the reader
//! hazard is real in both directions. Java and Python spell the other operation
//! `HALF_UP`, MATLAB spells this one `Nearest`, and a reader arriving from
//! either language reads the name as the one they know. The ruling answers that
//! hazard with a note rather than with a different name, so the note is not
//! decoration: it is the other half of the decision, and a page carrying the
//! name without it delivers half of what was ratified.
//!
//! The note's mark is the word `HALF_UP`. Naming the constant is what makes the
//! contrast checkable, and a page saying `half_up` is not Java's or Python's
//! `HALF_UP` has said the thing however it words the rest.
//!
//! What it reads, which is the surface a stranger lands on:
//!
//! - the repository's root `README.md`, which is where a reader who has never
//!   seen this project arrives, and which is exactly where the note was missing
//!   while three other surfaces carried it;
//! - every `.md.tmpl` under the mock directory outside the record, since those
//!   render into the published documents;
//! - every doc comment in a crate's own source that states the denotation,
//!   since rustdoc prints each of those as a page of its own and a reader
//!   reaches it without passing anything else.
//!
//! A doc comment states the denotation where it names the mode and says where a
//! tie goes, in the words the canon states it in. That is a narrower test than
//! naming the mode at all, on purpose: a passage mentioning `half_up` while
//! discussing something else owes nothing, and a passage telling a reader what
//! the name means owes the note in the same breath.
//!
//! What it does not read: the agent instructions under `agent/`, which are not
//! a published surface; plain comments, which rustdoc does not print; a crate's
//! `tests/` and `benches/`, which rustdoc does not print either; the research
//! tree and the design rounds, which are the record of how the question was
//! argued.
//!
//! It does not read the registry, and the documents generated under `docs/` are
//! where that shows. Most of them render from the templates above, so reading
//! the template covers them. One per namespace renders from the registry
//! instead, and several of those name the mode, in predicate axis values and in
//! the question rows the ruling was the answer to. None of them is telling a
//! reader what the name means: an axis value is a mention, and the question
//! rows that do state the denotation cite the ruling by slug in the same
//! sentence. A row that named the mode and explained it wrongly would be past
//! this lint, and it would be the canon correcting itself rather than a surface
//! being fixed, which is the sibling lint's carve-out for a ratified ruling in
//! its other half.

use std::path::{Path, PathBuf};

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::half_up_is_not_a_magnitude_rule::comments::passages;
use crate::half_up_is_not_a_magnitude_rule::reading::{DENOTES, SPELLINGS, spellings_in};

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(HalfUpCarriesItsNote)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "half-up-carries-its-note";

/// The mark the note leaves. The other operation's name in the two languages
/// that spell it this way, which is what the contrast is against.
const MARK: &str = "HALF_UP";

/// Directories under the mock directory that are the record, the build, or a
/// surface nobody outside this repository reads.
const NOT_READ: &[&str] = &["target", "research", "design_rounds", "agent"];

struct HalfUpCarriesItsNote;

impl Lint for HalfUpCarriesItsNote {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for HalfUpCarriesItsNote {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(ctx.repo_root, ctx.mock_dir)
    }
}

/// Every finding over the published surface.
pub(crate) fn check(repo_root: &Path, mock_dir: &Path) -> Vec<LintError> {
    let mut out = Vec::new();
    let readme = repo_root.join("README.md");
    if let Ok(text) = std::fs::read_to_string(&readme) {
        out.extend(markdown("README.md", &text));
    }
    let mut files = Vec::new();
    collect(mock_dir, &mut files);
    files.sort();
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .strip_prefix(mock_dir)
            .unwrap_or(&path)
            .display()
            .to_string();
        if shown.ends_with(".rs") {
            out.extend(rustdoc(&shown, &text));
        } else {
            out.extend(markdown(&shown, &text));
        }
    }
    out
}

/// A markdown document owes the note where it names the mode at all: a reader
/// arrives at the document rather than at a paragraph of it.
fn markdown(shown: &str, text: &str) -> Vec<LintError> {
    if text.contains(MARK) {
        return Vec::new();
    }
    let Some(&(at, _)) = spellings_in(text).first() else {
        return Vec::new();
    };
    vec![finding(shown, line_of(text, at), "names the mode")]
}

/// A doc comment owes the note where it states the denotation. Every other
/// passage in the file may name the mode freely.
fn rustdoc(shown: &str, text: &str) -> Vec<LintError> {
    let mut out = Vec::new();
    for passage in passages(text) {
        if !passage.doc || passage.text.contains(MARK) {
            continue;
        }
        let lower = passage.text.to_ascii_lowercase();
        let names = !spellings_in(&passage.text).is_empty()
            || passage.subject.is_some_and(|s| SPELLINGS.contains(&s));
        let Some(at) = DENOTES.iter().find_map(|d| lower.find(d)) else {
            continue;
        };
        if names {
            out.push(finding(
                shown,
                passage.line_of(at),
                "states what the name denotes",
            ));
        }
    }
    out
}

/// Every `.rs` under a crate's own source, and every `.md.tmpl`, outside the
/// directories this does not read and outside any hidden one.
pub(crate) fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if !(name.starts_with('.') || NOT_READ.contains(&name.as_str())) {
                collect(&path, out);
            }
        } else if name.ends_with(".md.tmpl") || is_crate_source(&path, &name) {
            out.push(path);
        }
    }
}

/// Whether a file is a crate's own library source, which is what rustdoc
/// prints. A lint, a tool or a bench is a program this repository runs rather
/// than a page anybody reads, and a file under a crate's `tests/` is read by
/// whoever opens it: rustdoc renders neither, so a reader never arrives at one
/// without already being here.
///
/// A crate's tests live under `src/` here as well as beside it, as `tests.rs`
/// or under a `tests/` directory, and they sit behind `#[cfg(test)]` wherever
/// they are. Rustdoc does not print those either, so they are excluded on the
/// same argument rather than on a different one.
fn is_crate_source(path: &Path, name: &str) -> bool {
    let mut in_crates = false;
    let mut in_source = false;
    let mut in_tests = false;
    for part in path.components() {
        let part = part.as_os_str().to_string_lossy();
        if in_source && part == "tests" {
            in_tests = true;
        }
        if in_crates && part == "src" {
            in_source = true;
        }
        if part == "crates" {
            in_crates = true;
        }
    }
    name.ends_with(".rs") && in_source && !in_tests && name != "tests.rs"
}

/// The line an offset lies on, counting from one.
pub(crate) fn line_of(text: &str, at: usize) -> usize {
    1 + text[.. at.min(text.len())].matches('\n').count()
}

/// One finding, saying what the ruling asks for and what to write.
fn finding(at: &str, line: usize, what: &str) -> LintError {
    let mut e = LintError::with_severity(
        "mock".to_string(),
        line,
        NAME,
        format!(
            "{what} and does not carry the note that travels with it. \
             `ruling::half_up_denotes_ties_toward_positive_infinity` settles `half_up` as a tie \
             toward positive infinity at every sign and asks that a note travel with the name, \
             because Java and Python spell the other operation `HALF_UP` and a reader arriving \
             from either reads this name as that one. Say that `half_up` is not the `HALF_UP` of \
             Java or Python, and that a tie at -2.5 goes to -2."
        ),
        Severity::HARD_ERROR,
    );
    e.path = Some(at.to_string());
    e
}

#[cfg(test)]
#[path = "half_up_carries_its_note/tests.rs"]
mod tests;
