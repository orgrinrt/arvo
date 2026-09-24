//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A published page says what the thing is, not what this project used to do.
//!
//! The shape is a true statement about the design with a clause bolted on
//! saying how the design got there. "the rule this crate shipped before the
//! canon settled", and after that was taken out, "is why the crate once
//! recorded a gap the alias does not have", three paragraphs further down the
//! same document in the round that removed the first one. The statement always
//! stands without the clause: what a spelling cannot form is a fact about the
//! spelling, and that this crate once recorded a gap because of it is a fact
//! about last week.
//!
//! It came back because nothing refused it. A reviewer found it twice and a
//! sweep found it a third time, and a sweep leaves nothing behind that catches
//! the fourth.
//!
//! What it reads is the published surface, which is what
//! `half-up-carries-its-note` reads and is defined once beside that lint: the
//! root `README.md`, every `.md.tmpl` that renders into `docs/`, and rustdoc on
//! a crate's own source. A plain comment is a note to whoever opens the file
//! and rustdoc does not print it. The design rounds, the research tree, the
//! registry and the agent surfaces are the record of how the project got here,
//! which is exactly where this belongs.
//!
//! Four sentences on the surface match the phrase list and are not this class.
//! They are excused by name in `EXCUSED` below rather than by weakening the
//! list, and an arm asserts each is still in the file it names, so a carve-out
//! that has stopped matching its sentence is a finding here rather than a
//! silent widening. The number is asserted as well as the sentences: it is
//! written in this paragraph and over the array as well as being the array's
//! length, and three copies of one number disagree without anything saying so
//! unless one of them is checked against the others.

use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::half_up_carries_its_note::{collect, line_of};
use crate::half_up_is_not_a_magnitude_rule::comments::passages;
use crate::half_up_is_not_a_magnitude_rule::reading::bounded;

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoProjectHistoryInPublishedProse)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
pub(crate) const NAME: &str = "no-project-history-in-published-prose";

/// What a sentence reaches for when it dates a rule or names a past state of
/// this project. Matched without case and bounded as a word.
///
/// The bound is load-bearing. Each phrase is several words, but its head can
/// still sit inside an ordinary word: `used to` is the tail of `refused to`,
/// `caused to` and `paused to`, and the first is ordinary English in a doc
/// comment, so an unbounded match refuses a page that says nothing about this
/// project's past, at hard error. The bound takes that class out without
/// touching the list.
///
/// What a bound cannot answer is the sense of a phrase standing as its own
/// words, which is what the carve-outs below are for and what they cost.
pub(crate) const PHRASES: &[&str] = &[
    "used to",
    "formerly",
    "previously",
    "before the ruling",
    "before the canon settled",
    "once recorded",
    "once carried",
    "once said",
    "an earlier draft",
    "an earlier version",
    "shipped before",
    "stood before",
    "no longer",
];

/// A sentence on the published surface that matches the list above and is not
/// this class.
pub(crate) struct Excused {
    /// The file, named as the lint names it: relative to the mock directory, or
    /// `README.md` for the repository root's.
    pub(crate) file:     &'static str,
    /// Enough of the sentence to find it, on the line the phrase sits on.
    pub(crate) sentence: &'static str,
    /// Why it stands.
    pub(crate) because:  &'static str,
}

/// Every sentence excused, with the reason. Two of the four are one document.
pub(crate) const EXCUSED: &[Excused] = &[
    Excused {
        file:     "crates/arvo-format/DESIGN.md.tmpl",
        sentence: "used to be the join",
        because:  "It names a state of the lattice over one kind of point rather than a past \
                   state of this codebase. Putting floating points into the space is what stops \
                   that point being the join, and the result is measured.",
    },
    Excused {
        file:     "DESIGN.md.tmpl",
        sentence: "used crates that no longer exist",
        because:  "Here the history is the statement rather than a clause on one. The paragraph \
                   exists to correct what earlier versions of this document promised a downstream \
                   reader, and dropping the history deletes the correction.",
    },
    Excused {
        file:     "DESIGN.md.tmpl",
        sentence: "removed crates used to answer",
        because:  "The same paragraph, saying which piece of what was removed the new crate \
                   answers. It is what a reader pinning this branch needs and is not available \
                   anywhere else.",
    },
    Excused {
        file:     "crates/arvo-format/src/lib.rs",
        sentence: "cannot be\n//! used to define themselves",
        because:  "A different sense of the words: the primitives are employed to define \
                   themselves, rather than having once done so. Nothing separates the two \
                   lexically, which is why this is named rather than filtered.",
    },
];

struct NoProjectHistoryInPublishedProse;

impl Lint for NoProjectHistoryInPublishedProse {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for NoProjectHistoryInPublishedProse {
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

/// Every phrase in a whole document, outside what is excused for that file.
fn markdown(shown: &str, text: &str) -> Vec<LintError> {
    found(text)
        .into_iter()
        .map(|(at, phrase)| (line_of(text, at), phrase))
        .filter(|&(line, _)| !excused(shown, text, line))
        .map(|(line, phrase)| finding(shown, line, phrase))
        .collect()
}

/// Every phrase in a doc comment. A plain comment is a note to whoever opens
/// the file, and rustdoc does not print it.
fn rustdoc(shown: &str, text: &str) -> Vec<LintError> {
    let mut out = Vec::new();
    for passage in passages(text) {
        if !passage.doc {
            continue;
        }
        for (at, phrase) in found(&passage.text) {
            let line = passage.line_of(at);
            if !excused(shown, text, line) {
                out.push(finding(shown, line, phrase));
            }
        }
    }
    out
}

/// Where every phrase in `text` starts, with the phrase, matched without case
/// and bounded, so a phrase sitting inside a longer word is not one.
fn found(text: &str) -> Vec<(usize, &'static str)> {
    let lower = text.to_ascii_lowercase();
    let mut out: Vec<(usize, &'static str)> = PHRASES
        .iter()
        .flat_map(|p| bounded(&lower, p).into_iter().map(move |at| (at, *p)))
        .collect();
    out.sort();
    out
}

/// Whether `line` of this file lies inside a sentence excused for it.
///
/// Taken by line rather than by offset, because a doc comment's offsets are
/// into the passage and a document's are into the file, and the line is the one
/// coordinate both report. A carve-out covers the lines its own sentence spans
/// and no others, so the same phrase elsewhere in the same file is reported.
fn excused(shown: &str, text: &str, line: usize) -> bool {
    EXCUSED
        .iter()
        .filter(|e| e.file == shown)
        .filter_map(|e| text.find(e.sentence).map(|from| (from, e.sentence.len())))
        .any(|(from, len)| (line_of(text, from) ..= line_of(text, from + len)).contains(&line))
}

/// One finding, saying what to write instead.
fn finding(at: &str, line: usize, phrase: &'static str) -> LintError {
    let mut e = LintError::with_severity(
        "mock".to_string(),
        line,
        NAME,
        format!(
            "says `{phrase}` on a page a stranger reads, which dates a rule or names a past \
             state of this project. A published document says what the thing is. Say what the \
             design does and what it cannot do, and stop there: how it came to be that way is \
             the design rounds' and the registry's, which is where a reader who wants it looks. \
             Where the history genuinely is the statement rather than a clause on one, name the \
             sentence in `EXCUSED` in `mock/lints/{}.rs` with the reason.",
            NAME.replace('-', "_")
        ),
        Severity::HARD_ERROR,
    );
    e.path = Some(at.to_string());
    e
}

#[cfg(test)]
#[path = "no_project_history_in_published_prose/tests.rs"]
mod tests;
