//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A `note` claiming one of its own row's fields is empty, where it is not.
//!
//! **The commonest way a row goes stale, and the quietest.** A note is written
//! when a field genuinely was empty; a later pass fills the field; nothing
//! re-reads the note. It then reads as a caveat a reader should honour, and
//! what it actually says is what the row looked like some rounds ago. Nothing
//! in a schema can catch it: both halves are valid and only their disagreement
//! is wrong.
//!
//! Fifteen rows here carried "`evidence` is empty and the measured-implies-
//! evidence check is red on this row", written truthfully when the instruments
//! namespace did not exist. It exists now and the edges were wired, so thirteen
//! of the fifteen notes said the opposite of their own row.
//!
//! **A lint rather than a tool.** The refused state is a row disagreeing with
//! itself, and the repair is one edit to the note. There is no reading under
//! which it is fine and no judgement anybody has to be asked for.
//!
//! **Matched on the row's own field names**, never on a word list, so it works
//! for any field a note might claim is empty and needs no editing when a
//! namespace gains one.
//!
//! # What the view's shape changed, and what it costs
//!
//! The crate this came from walked a row's **array** fields only, because its
//! own loader kept scalars and arrays apart. A lint is handed a `RegistryView`,
//! whose row is one flat map of field name to string with a `string[]` already
//! joined by `", "`, so **that separation is not available here and this walks
//! every field the row carries.**
//!
//! Two consequences, both stated rather than left to be discovered.
//!
//! **It reports more.** A note claiming a scalar field is empty, where that
//! field holds text, is now a finding. That is the same defect one field type
//! over, and reporting it is the widening working rather than a side effect to
//! be sorry about.
//!
//! **The count is over the joined field.** How far a note has drifted is worth
//! reporting, and the only count available is how many `", "`-separated pieces
//! the value has. For an array field that is exactly the entry count the row
//! wrote. For a scalar it is one, unless the prose itself carries the
//! separator, in which case the number is larger than the one thing the field
//! holds. The residue is real, it is confined to the count rather than to
//! whether the row is reported, and a reader who opens the row sees the truth
//! in one look.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ANoteDescribesTheRowItSitsOn)
}

const LINT: &str = "a-note-describes-the-row-it-sits-on";

/// The ways this corpus says a field holds nothing.
///
/// A phrase, never a keyword: `empty` alone matches "the empty region" and
/// every other legitimate use of the word, which in this corpus is most of
/// them.
const SAYS_EMPTY: [&str; 4] = ["is empty", "are empty", "carries none", "carries nothing"];

/// How far past the field name the phrase may sit and still be about it.
///
/// Deliberately short. A note mentioning a field in one clause and saying
/// something is empty three sentences later is talking about two things, and
/// joining them would make this fire on prose that is correct.
const WINDOW: usize = 24;

struct ANoteDescribesTheRowItSitsOn;
impl Lint for ANoteDescribesTheRowItSitsOn {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ANoteDescribesTheRowItSitsOn {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry;
        let mut out = Vec::new();
        for namespace in reg.namespaces() {
            for q in reg.rows_in(namespace) {
                let Some(note) = text(reg, q, "note") else {
                    continue;
                };
                let Some(row) = reg.row(q) else {
                    continue;
                };
                for field in row.keys() {
                    let held = list(reg, q, field).len();
                    if held > 0 && claims_it_is_empty(note, field) {
                        out.push(stale(q, field, held));
                    }
                }
            }
        }
        out
    }
}

/// Whether the note claims this field holds nothing.
///
/// The claim is written as a backticked field name followed closely by one of
/// the phrases, and every occurrence of the name is looked at rather than the
/// first, because a note naming a field twice says the thing at one of them.
fn claims_it_is_empty(note: &str, field: &str) -> bool {
    let needle = format!("`{field}`");
    let mut from = 0;
    while let Some(at) = note[from..].find(&needle) {
        let start = from + at + needle.len();
        let end = ceiling(note, start + WINDOW);
        if SAYS_EMPTY.iter().any(|p| note[start..end].contains(p)) {
            return true;
        }
        from = start;
    }
    false
}

/// The nearest character boundary at or below `at`, so a window landing inside
/// a multi-byte character does not panic.
///
/// The corpus is prose and carries them. `str::floor_char_boundary` would say
/// this in one call and is unstable, which a lint pack cannot reach for.
fn ceiling(s: &str, at: usize) -> usize {
    let mut at = at.min(s.len());
    while !s.is_char_boundary(at) {
        at -= 1;
    }
    at
}

fn stale(q: &str, field: &str, held: usize) -> LintError {
    finding(
        LINT,
        None,
        format!(
            "`{q}` has a `note` saying `{field}` is empty, and `{field}` holds {held} \
             entr{}. The note was true when it was written and a later pass filled the field, \
             so it now reads as a caveat about the row it sits on and describes a different \
             one.",
            if held == 1 { "y" } else { "ies" }
        ),
    )
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ANoteDescribesTheRowItSitsOn, &view(rows, &[]))
    }

    #[test]
    fn a_false_claim_fires_and_a_true_one_does_not() {
        // Both ways round in one input. Without the second row the arm is a ban
        // on notes mentioning a field at all.
        let f = found(&[
            (
                "proposal::the_note_is_stale",
                &[
                    ("says", "Something."),
                    ("evidence", "a_probe"),
                    ("note", "`evidence` is empty, so nobody can check this."),
                ],
            ),
            (
                "proposal::the_note_is_true",
                &[
                    ("says", "Something else."),
                    ("note", "`evidence` is empty, so nobody can check this."),
                ],
            ),
        ]);
        assert_eq!(f.len(), 1, "exactly the stale one: {f:?}");
        assert!(f[0].contains("the_note_is_stale"), "{}", f[0]);
        assert!(
            f[0].contains("holds 1 entry"),
            "the count is reported, so a reader can see how far the note has drifted: {}",
            f[0]
        );
    }

    #[test]
    fn the_word_empty_about_something_else_is_left_alone() {
        // Without this the arm fires on every note that mentions a field and
        // somewhere says the word, which in this corpus is most of them: an
        // empty region, an empty intersection, an empty result being the
        // finding.
        let f = found(&[(
            "proposal::a_row_about_empty_regions",
            &[
                ("says", "Something."),
                ("evidence", "a_probe"),
                (
                    "note",
                    "The `evidence` here rests on an instrument whose covered set is empty at \
                      width two, and the intersection of the two predicates is empty.",
                ),
            ],
        )]);
        assert!(
            f.is_empty(),
            "a note may say `empty` about the thing a field describes: {f:?}"
        );
    }

    #[test]
    fn a_field_named_far_from_the_phrase_is_not_a_claim_about_it() {
        let f = found(&[(
            "proposal::two_separate_clauses",
            &[
                ("says", "Something."),
                ("evidence", "a_probe"),
                (
                    "note",
                    "The `evidence` is one instrument and its author bounded it exactly, \
                      measured at the widths in its table and no further. The region it \
                      establishes is empty above that.",
                ),
            ],
        )]);
        assert!(
            f.is_empty(),
            "two clauses about two things are not one claim about one: {f:?}"
        );
    }

    #[test]
    fn it_works_on_a_field_the_lint_never_heard_of() {
        // It reads the row's own field names, so a namespace gaining a field
        // needs no edit here. A word list would have had to gain it too,
        // silently missing it until somebody noticed.
        let f = found(&[(
            "retirement::a_route",
            &[
                ("claim", "A way."),
                ("obligation", "a_thing, another_thing"),
                ("note", "`obligation` is empty on this row."),
            ],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("holds 2 entries"), "{}", f[0]);
    }

    #[test]
    fn a_note_claiming_a_scalar_field_is_empty_is_reported_too() {
        // The widening the view's shape forces, planted rather than left to be
        // discovered. The crate this came from walked array fields only, so a
        // note lying about a scalar passed. It is the same defect and it is
        // reported, and the count is over the joined value, which for a scalar
        // holding no separator is one.
        let f = found(&[(
            "proposal::p",
            &[
                ("says", "Something."),
                ("gap", "the register does not yet carry the scope"),
                ("note", "`gap` is empty on this row."),
            ],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("holds 1 entry"), "{}", f[0]);
    }

    #[test]
    fn a_note_naming_a_field_the_row_does_not_carry_is_not_a_finding() {
        // The other half of the same claim. A note saying a field is empty,
        // about a field that really holds nothing or is not there at all, is
        // accurate and stays.
        let f = found(&[(
            "proposal::p",
            &[
                ("says", "Something."),
                ("note", "`evidence` is empty and `predicate` carries none."),
            ],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_stale_clause_in_one_note_is_reported_rather_than_the_first_field() {
        // The walk is over every field the row carries. One that stopped at the
        // first match would report half of a note lying about two fields, and
        // every arm above plants one field so none of them can tell.
        let f = found(&[(
            "proposal::p",
            &[
                ("says", "Something."),
                ("evidence", "a_probe"),
                ("predicate", "threads: 1"),
                ("note", "`evidence` is empty and `predicate` is empty."),
            ],
        )]);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().any(|m| m.contains("`evidence`")), "{f:?}");
        assert!(f.iter().any(|m| m.contains("`predicate`")), "{f:?}");
    }

    #[test]
    fn a_second_mention_of_one_field_is_read_when_the_first_says_nothing() {
        // The scan walks every occurrence of the name. One that looked only at
        // the first would miss a note that mentions a field in passing and then
        // lies about it in the next sentence.
        let f = found(&[(
            "proposal::p",
            &[
                ("says", "Something."),
                ("evidence", "a_probe"),
                (
                    "note",
                    "The `evidence` was chosen for the width it reached rather than for its \
                      arity. `evidence` is empty here.",
                ),
            ],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn all_four_phrasings_are_read() {
        // Four spellings, one meaning, and a list carrying only the first would
        // pass every arm above.
        for phrase in ["is empty", "are empty", "carries none", "carries nothing"] {
            let note = format!("`evidence` {phrase} on this row.");
            let f = found(&[(
                "proposal::p",
                &[
                    ("says", "Something."),
                    ("evidence", "a_probe"),
                    ("note", &note),
                ],
            )]);
            assert_eq!(f.len(), 1, "{phrase:?} was not read: {f:?}");
        }
    }

    #[test]
    fn a_note_carrying_a_multibyte_character_at_the_window_edge_does_not_panic() {
        // The window is a byte count into prose, and this corpus carries
        // characters wider than one byte. A slice landing inside one panics,
        // which on a gate is a lint that reports nothing at all.
        let note = "`evidence` ".to_string() + &"é".repeat(40) + " is empty";
        let f = found(&[(
            "proposal::p",
            &[
                ("says", "Something."),
                ("evidence", "a_probe"),
                ("note", &note),
            ],
        )]);
        assert!(f.is_empty(), "the phrase is far past the window: {f:?}");
    }

    #[test]
    fn a_row_with_no_note_at_all_is_not_read() {
        let f = found(&[("proposal::p", &[("evidence", "a_probe")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_namespace_is_read_rather_than_a_written_list() {
        // A note goes stale wherever one is written, so there is no namespace
        // to leave out and none is.
        let f = found(&[
            (
                "ruling::r",
                &[("ratifies", "x"), ("note", "`ratifies` is empty.")],
            ),
            (
                "probe::p",
                &[("standing", "sound"), ("note", "`standing` is empty.")],
            ),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[(
                "proposal::p",
                &[("evidence", "a_probe"), ("note", "`evidence` is empty.")],
            )],
            &[],
        );
        assert_findings_block(&super::ANoteDescribesTheRowItSitsOn, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ANoteDescribesTheRowItSitsOn);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ANoteDescribesTheRowItSitsOn.name(),
            "a-note-describes-the-row-it-sits-on"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-note-describes-the-row-it-sits-on");
    }
}
