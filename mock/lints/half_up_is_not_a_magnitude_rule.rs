//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: prose never gives the mode `half_up` a magnitude reading.
//!
//! `ruling::half_up_denotes_ties_toward_positive_infinity` settles what the name
//! denotes: `floor(x + q/2)`, a tie going toward positive infinity at every
//! sign. The reading this crate once shipped put a tie by its magnitude instead,
//! and it got in through prose first. A doc comment said it, a test pinned what
//! the doc comment said, the oracle was written from both, and every check
//! agreed with every other because each had copied the sentence rather than
//! the ruling. So the sentence is what this refuses.
//!
//! It reads every comment and doc comment in a `.rs` file under the mock
//! directory, every `.md.tmpl` there, and the registry's prose fields, clause by
//! clause. A clause fires where it names the mode, in any spelling in
//! `reading::SPELLINGS`, and in the same clause gives one of the readings in
//! `reading::READINGS`: a tie sent by the sign or by the magnitude, or a claim
//! that the mode commutes with reflection. The readings are matched as words, so
//! a longer identifier carrying one is not one, and `in magnitude` counts only
//! behind a direction word, since the same two words bound an error in prose
//! that is about something else entirely.
//!
//! Five things let a clause through, and each has a test and a control:
//!
//! - a negator bound to the reading or to the name, which means one sitting in
//!   that term's own segment of the clause, ahead of it, with no conjunction
//!   between the two turning the sentence back to what it asserts;
//! - a marker saying the clause is about another rule, a planted one, a wrong
//!   one, the one that stood before, or no tie at all;
//! - a list of three items or more, with the name and the reading in different
//!   items and each of those items holding nothing but its own term;
//! - the name and the reading in different clauses;
//! - a clause opening with a pronoun whose antecedent is something else, since
//!   a sentence contrasting the mode with Java's rule leaves Java's rule named
//!   last and the pronoun after it points there.
//!
//! A table row is read by its first cell: a reading in any other cell is paired
//! with the name the first cell carries. An outer doc block is read as naming
//! the mode wherever the item under it names the mode, since rustdoc prints it
//! under that name.
//!
//! What it does not read: string literals, which are data rather than prose, so
//! a test planting a violation does not trip it; the research tree and the
//! design rounds, which are the record of how the question was argued and say
//! the other reading on purpose; `target/`; the `retirement` namespace, whose
//! rows quote a retired claim in the words it was written in; a ratified
//! `ruling`, which is the canon this defends and is not corrected from below;
//! the `answered` field of a question, which names both readings in order to say
//! which one was taken; the `options` of any question, and a row's `quote`,
//! which is op's verbatim. The rest of an answered question is prose somebody
//! here wrote and is read like any other. A column table, whose name sits in a
//! header rather than in the row's first cell, is not read as a pairing.
//!
//! A lint rather than a tool, though `mock/tools/rounding-vocabulary` reads the
//! same vocabulary. That tool declares itself not a lint, since the repair for
//! a row outside the six is an edit nobody can make mechanically, and it reads
//! the predicates only. This has a state the project refuses to be in and a
//! repair any writer can make, so it gates, and it reads prose, which the tool
//! does not open.

use std::path::{Path, PathBuf};

use mockspace::{Lint, LintError, RegistryView, RepoContext, RepoLint, Severity};

#[path = "half_up_is_not_a_magnitude_rule/comments.rs"]
pub(crate) mod comments;
#[path = "half_up_is_not_a_magnitude_rule/reading.rs"]
pub(crate) mod reading;

use comments::{Passage, passages};
use reading::{Hit, hits};

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(HalfUpIsNotAMagnitudeRule)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "half-up-is-not-a-magnitude-rule";

/// Directories under the mock directory that are the record rather than the
/// description, or not source at all.
const NOT_READ: &[&str] = &["target", "research", "design_rounds"];

/// Registry fields that hold identifiers, citations, tiers or verbatim words
/// rather than prose somebody here writes.
///
/// `answered` is among them because a question is answered by naming both
/// readings and saying which one was taken, so that field carries the other
/// reading on purpose. The rest of an answered question is not exempt: `asks`,
/// `note` and `because` are written here like any other sentence, and skipping
/// the whole row, which is what stood before, put every one of them outside the
/// gate.
const NOT_PROSE: &[&str] = &[
    "id",
    "keywords",
    "provenance",
    "topic",
    "kind",
    "standing",
    "sentence_kind",
    "lives",
    "decider",
    "rung",
    "key",
    "answers",
    "answered",
    "evidence",
    "law",
    "ratified_by",
    "obligation",
    "ratifies",
    "declines",
    "supersedes",
    "corrects",
    "precondition_for",
    "gate",
    "name",
    "unit",
    "grammar",
    "quote",
    "options",
];

struct HalfUpIsNotAMagnitudeRule;

impl Lint for HalfUpIsNotAMagnitudeRule {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for HalfUpIsNotAMagnitudeRule {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = check_tree(ctx.mock_dir);
        out.extend(check_registry(ctx.registry));
        out
    }
}

/// Every finding in the files under `mock_dir`.
fn check_tree(mock_dir: &Path) -> Vec<LintError> {
    let mut files = Vec::new();
    collect(mock_dir, &mut files);
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .strip_prefix(mock_dir)
            .unwrap_or(&path)
            .display()
            .to_string();
        let read = if shown.ends_with(".rs") {
            passages(&text)
        } else {
            vec![Passage {
                text,
                line: 1,
                subject: None,
                doc: false,
            }]
        };
        for passage in read {
            for hit in hits(&passage.text, passage.subject) {
                out.push(finding("mock", &shown, passage.line_of(hit.at), &hit));
            }
        }
    }
    out
}

/// Every `.rs` and `.md.tmpl` file under `dir`, outside the directories this
/// does not read and outside any hidden one.
fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
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
        } else if name.ends_with(".rs") || name.ends_with(".md.tmpl") {
            out.push(path);
        }
    }
}

/// Every finding in the registry's prose fields.
fn check_registry(registry: &RegistryView) -> Vec<LintError> {
    let mut out = Vec::new();
    for ns in registry.namespaces() {
        if ns == "retirement" {
            continue;
        }
        for q in registry.rows_in(ns) {
            if ns == "ruling" && registry.field(q, "rung") == Some("ratified") {
                continue;
            }
            let Some(row) = registry.row(q) else {
                continue;
            };
            for (field, value) in row {
                if NOT_PROSE.contains(&field.as_str()) {
                    continue;
                }
                for hit in hits(value, None) {
                    let at = format!("`{q}`, field `{field}`");
                    out.push(finding("registry", &at, 0, &hit));
                }
            }
        }
    }
    out
}

/// One finding, saying what the ruling settled and what to write instead.
fn finding(unit: &str, at: &str, line: usize, hit: &Hit) -> LintError {
    let mut e = LintError::with_severity(
        unit.to_string(),
        line,
        NAME,
        format!(
            "names `{}` and reads it as `{}` in one clause. \
             `ruling::half_up_denotes_ties_toward_positive_infinity` settles `half_up` as \
             `floor(x + q/2)`, a tie going toward positive infinity at every sign, so it reads \
             nothing of the sign or the magnitude. Say that, or where the clause means ties away \
             from zero, name that as the alias and say it is not the mode.",
            hit.name, hit.reading
        ),
        Severity::HARD_ERROR,
    );
    e.path = Some(at.to_string());
    e
}

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/corpus_tests.rs"]
mod corpus_tests;

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/sentences.rs"]
mod sentences;

#[cfg(test)]
mod reaches_the_gate {
    use super::{HalfUpIsNotAMagnitudeRule, NAME};
    use crate::canon_lint_testkit::{
        assert_findings_block_at,
        assert_not_declared_off,
        assert_registered,
        ctx_at,
        plant,
        planted_tree,
        view,
    };

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("half-up-severity");
        plant(
            &dir,
            "mock/crates/a/src/lib.rs",
            "// `half_up` takes a tie away from zero.\n",
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(
            &HalfUpIsNotAMagnitudeRule,
            &ctx_at(&dir.join("mock"), &empty),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&HalfUpIsNotAMagnitudeRule);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        // The literal the configuration keys on. Comparing the accessor with the
        // constant it returns is a sentence about nothing and was one.
        assert_eq!(NAME, "half-up-is-not-a-magnitude-rule");
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(NAME);
    }
}
