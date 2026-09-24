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
//! It warns and does not block, at every gate. The reader under it is wrong in
//! both directions, on sentences a writer here does write, and a gate that
//! blocks on a reader known to be wrong refuses true sentences at hard error.
//! What it gets wrong is stated next, and every known case is a known-red pair
//! in `half_up_is_not_a_magnitude_rule/sentences/catalogue.rs` rather than a
//! sentence in this paragraph. The demotion does not rest on the ruling's own
//! prose: `check_registry` skips every ratified ruling outright, so the gate
//! never reads a field of it, the governing ruling's `promotion` included. It
//! rests on the catalogue's refused-true entries, which are shapes the gate
//! does read: a clause naming the alias under a name that is also a reading,
//! and a relative or a clause whose subject is a rule or a seat named in
//! words.
//!
//! The gate is incomplete in both directions.
//!
//! - It lets through a clause that gives the mode the reading where the clause
//!   opens on a capitalised word that is also a method's name, `Round`, `Fix`,
//!   `Ceiling` and the rest, used as an imperative or an ordinary word. It reads
//!   the word as another rule, because the design documents write those method
//!   names bare at the head of sentences and nothing lexical tells the two apart.
//! - It lets through a clause opening on a code span that is not a name, such as
//!   a number, since a code span opening a clause is the only subject it can
//!   recognise without a word for it.
//! - It refuses a clause whose reading belongs to a rule named in words it has
//!   no vocabulary for, "Java's rule" or "seat 270", because nothing named there
//!   takes the reading away from the mode named before it. The governing
//!   ruling's own `promotion` field is one of these.
//! - It refuses a clause naming the alias beside the mode, where the alias's
//!   own name is also one of the readings, `roundTiesToAway` and the
//!   `ties-away` spellings, and no negator says the alias is not the mode.
//!   "`half_up` is nearest, whereas `roundTiesToAway` goes away from zero" is
//!   true and is refused.
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
//! What lets a clause through, each with a test and a control:
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
//!   last and the pronoun after it points there, and a sentence denying that
//!   something is the mode leaves that something as what the pronoun means;
//! - a reading whose own segment is about another rule, by naming it ahead of
//!   the reading or by pointing back with a relative or a pronoun where another
//!   rule stood last;
//! - the settled denotation stated ahead of the reading, with no conjunction in
//!   the reading's own segment joining the two, which is a contrast rather than
//!   a second reading;
//! - a reading quoted as a term, in a code span with a word beside it naming a
//!   piece of a corpus.
//!
//! A table row is read by its first cell: a reading in any other cell is paired
//! with the name the first cell carries. An outer doc block is read as naming
//! the mode wherever the item under it names the mode, since rustdoc prints it
//! under that name.
//!
//! What it does not read: string literals, which are data rather than prose, so
//! a test planting a violation does not trip it; the research tree and the
//! design rounds, which are the record of how the question was argued and say
//! the other reading on purpose; `target/`; `lints/`, this lint's own source
//! and its siblings', which explain a reading in order to refuse it; the
//! `retirement` namespace, whose
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
//! repair any writer can make, and it reads prose, which the tool does not
//! open. That it warns rather than gates is the reader's limit, not a judgement
//! that the state is acceptable.

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

/// What every finding carries: a warning at every gate, blocking none.
///
/// The reader is incomplete in both directions, as the module doc says, and
/// refuses true sentences it has no vocabulary for: a clause naming the alias
/// under a name that is also a reading, and a relative or a clause whose
/// subject is a rule or a seat named in words, catalogued in
/// `half_up_is_not_a_magnitude_rule/sentences/catalogue.rs`. A hard error
/// would refuse those sentences at every gate. The lint's own default and the
/// severity its findings carry are this one constant, so the two cannot
/// disagree.
const SEVERITY: Severity = Severity::ADVISORY;

/// Directories under the mock directory that are the record rather than the
/// description, or not source at all.
///
/// `lints` is among them for the same argument as `research` and
/// `design_rounds`: this lint's own source, and its siblings', explain the
/// readings a sentence can carry in order to refuse them, which reads as
/// asserting every one. Reading it here fired on the module doc's own
/// examples, at every gate, on a clean tree.
const NOT_READ: &[&str] = &["target", "research", "design_rounds", "lints"];

/// Registry fields that hold identifiers, citations, tiers or verbatim words
/// rather than prose somebody here writes.
///
/// `answered` is among them because a question is answered by naming both
/// readings and saying which one was taken, so that field carries the other
/// reading on purpose. The rest of an answered question is not exempt: `asks`,
/// `note` and `because` are written here like any other sentence, and skipping
/// the whole row put every one of them outside the gate.
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
        SEVERITY
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

/// One finding, saying what the ruling settled, what to write instead, and why
/// it is a warning.
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
             from zero, name that as the alias and say it is not the mode. This is a warning: the \
             reader behind it is wrong in both directions on known sentences, listed in the \
             module doc of `mock/lints/half_up_is_not_a_magnitude_rule.rs`, so a clause that is \
             true and still refused is one of those rather than something to rewrite.",
            hit.name, hit.reading
        ),
        SEVERITY,
    );
    e.path = Some(at.to_string());
    e
}

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/segment_tests.rs"]
mod segment_tests;

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/carry_tests.rs"]
mod carry_tests;

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/corpus_tests.rs"]
mod corpus_tests;

#[cfg(test)]
#[path = "half_up_is_not_a_magnitude_rule/sentences.rs"]
mod sentences;

#[cfg(test)]
mod reaches_the_gate {
    use mockspace::{Lint, RepoLint, Severity};

    use super::{HalfUpIsNotAMagnitudeRule, NAME};
    use crate::canon_lint_testkit::{
        assert_findings_carry_at,
        assert_not_declared_off,
        assert_registered,
        ctx_at,
        plant,
        planted_tree,
        repo_root,
        view,
    };

    #[test]
    fn its_findings_warn_at_every_gate_and_block_none() {
        // The literal rather than the constant the lint reads, so moving the
        // constant back to a blocking severity is a failure here.
        let dir = planted_tree("half-up-severity");
        plant(
            &dir,
            "mock/crates/a/src/lib.rs",
            "// `half_up` takes a tie away from zero.\n",
        );
        let empty = view(&[], &[]);
        assert_findings_carry_at(
            &HalfUpIsNotAMagnitudeRule,
            &ctx_at(&dir.join("mock"), &empty),
            Severity::ADVISORY,
        );
        assert_eq!(
            HalfUpIsNotAMagnitudeRule.default_severity(),
            Severity::ADVISORY
        );
    }

    #[test]
    fn its_message_says_why_it_warns_and_where_the_reason_is() {
        let dir = planted_tree("half-up-message");
        plant(
            &dir,
            "mock/crates/a/src/lib.rs",
            "// `half_up` takes a tie away from zero.\n",
        );
        let empty = view(&[], &[]);
        let found = HalfUpIsNotAMagnitudeRule.check_repo(&ctx_at(&dir.join("mock"), &empty));
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("This is a warning"));
        assert!(
            found[0]
                .message
                .contains("mock/lints/half_up_is_not_a_magnitude_rule.rs")
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

    #[test]
    fn no_severity_override_pins_it_to_the_pack_default() {
        // The `[lints]` snapshot in `catalogues.md` reports this lint at
        // "pack default", never "set here". A `[lints.half-up-is-not-a-magnitude-rule]`
        // table in `mockspace.toml` would give this lint a severity this repo
        // chose, which nothing here asks for: the module doc's whole point is
        // that ADVISORY is right because the reader is known incomplete, and a
        // repo override is the shape by which that could quietly drift.
        let toml = std::fs::read_to_string(repo_root().join("mockspace.toml"))
            .expect("mockspace.toml at the repo root");
        assert!(
            !toml.contains("[lints.half-up-is-not-a-magnitude-rule]"),
            "mockspace.toml now overrides this lint's severity; the module doc \
             and this test both assume the pack default governs it"
        );
    }
}
