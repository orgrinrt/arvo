//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Two live definitions of one term.
//!
//! A definition stipulates what a term means, so two rows defining the same
//! term are either a supersession, in which case the later one says so, or a
//! disagreement, in which case somebody has to resolve it. Sitting side by
//! side, both are cited and each reader gets whichever they found first.
//!
//! **A lint rather than a tool.** The state has one of two repairs and neither
//! is a judgement call somebody has to be asked for: record the supersession,
//! or resolve the disagreement. Nothing here is a ranking and there is no
//! threshold to invent.
//!
//! **A supersession only excuses a rival where it replaces the same term.** The
//! first version of this skipped any row carrying a `supersedes` at all, which
//! is the whole edge in one word: a definition that supersedes something
//! unrelated stopped being a definition for this check's purposes, so a genuine
//! rival pair vanished. It was found by a control that refused to fire, which
//! is worth more than one that fires, and the arm was green over a selection
//! that one line had emptied.
//!
//! **A definition with no `defines` is not reported here.** It has no term to
//! collide on, and `a-definition-names-its-term` is the check that names it.
//! Reporting it twice would be two reports of one defect in different words.
//!
//! **What the port loses.** The crate this came from also asserted the
//! committed canon defines no term twice. A unit test cannot build a
//! `RegistryView` from the real registry, because that needs a TOML parser the
//! generated pack may not depend on, so that arm is `cargo mock --lint-only`
//! now, over the real rows, blocking.
use std::collections::BTreeMap;

use mockspace::{Lint, LintError, RegistryView, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, slug, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ATermIsDefinedTwice)
}

const LINT: &str = "a-term-is-defined-twice";

/// The namespace whose rows carry `sentence_kind` and `defines`.
const NAMESPACE: &str = "proposal";

/// The kind of sentence that stipulates a term.
const DEFINITION: &str = "definition";

struct ATermIsDefinedTwice;
impl Lint for ATermIsDefinedTwice {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ATermIsDefinedTwice {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry;
        let definitions: Vec<&str> = reg
            .rows_in(NAMESPACE)
            .iter()
            .map(String::as_str)
            .filter(|q| text(reg, q, "sentence_kind") == Some(DEFINITION))
            .collect();

        let mut seen: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for q in &definitions {
            let Some(term) = text(reg, q, "defines") else {
                continue; // reported by `a-definition-names-its-term`
            };
            if replaces_a_rival(reg, q, term, &definitions) {
                continue;
            }
            seen.entry(term).or_default().push(q);
        }

        seen.into_iter()
            .filter(|(_, rows)| rows.len() > 1)
            .map(|(term, rows)| {
                finding(
                    LINT,
                    None,
                    format!(
                        "`{term}` is defined by {} live rows and none supersedes another: {}. \
                         Two stipulations of one term is a supersession somebody did not record \
                         or a disagreement somebody did not resolve.",
                        rows.len(),
                        rows.join(", ")
                    ),
                )
            })
            .collect()
    }
}

/// Whether this row supersedes another definition of the same term.
///
/// Both halves matter. A `supersedes` naming a row that is not a definition, or
/// one defining something else, excuses nothing, and reading the field alone is
/// the defect a control caught here once already.
fn replaces_a_rival(reg: &RegistryView, q: &str, term: &str, definitions: &[&str]) -> bool {
    list(reg, q, "supersedes").iter().any(|named| {
        definitions
            .iter()
            .any(|other| slug(other) == *named && text(reg, other, "defines") == Some(term))
    })
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ATermIsDefinedTwice, &view(rows, &[]))
    }

    #[test]
    fn one_term_stipulated_twice_is_reported_and_a_supersession_is_not() {
        let f = found(&[
            (
                "proposal::the_first_reading",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::a_rival_reading",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::an_older_reading_of_something_else",
                &[("sentence_kind", "definition"), ("defines", "stretch")],
            ),
            (
                "proposal::the_reading_that_replaced_it",
                &[
                    ("sentence_kind", "definition"),
                    ("defines", "stretch"),
                    ("supersedes", "an_older_reading_of_something_else"),
                ],
            ),
        ]);
        assert_eq!(
            f.len(),
            1,
            "two live readings of `chain` are a finding; a replaced reading of `stretch` is the \
             mechanism working: {f:?}"
        );
        assert!(f[0].contains("chain"), "{}", f[0]);
        assert!(f[0].contains("a_rival_reading"), "{}", f[0]);
        assert!(f[0].contains("the_first_reading"), "{}", f[0]);
    }

    #[test]
    fn superseding_something_unrelated_does_not_hide_a_rival_definition() {
        // The first version of this arm skipped any row carrying a `supersedes`
        // at all, so a definition that replaced some unrelated claim stopped
        // counting as a definition and its rival vanished with it. The arm then
        // read green over a selection one line had emptied, which is the shape
        // a checker cannot see from inside: it reports nothing, and nothing is
        // what a clean corpus reports too.
        let f = found(&[
            (
                "proposal::the_first_reading",
                &[("sentence_kind", "definition"), ("defines", "strategy")],
            ),
            (
                "proposal::a_rival_that_replaced_something_else",
                &[
                    ("sentence_kind", "definition"),
                    ("defines", "strategy"),
                    ("supersedes", "an_unrelated_claim"),
                ],
            ),
            (
                "proposal::an_unrelated_claim",
                &[("sentence_kind", "definition"), ("defines", "container")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("strategy"), "{}", f[0]);
    }

    #[test]
    fn superseding_a_row_that_is_not_a_definition_excuses_nothing() {
        // The other half of the same discrimination, and one the crate this
        // came from never planted. A `supersedes` naming a live row that is not
        // a definition would satisfy a check that only asked whether the named
        // slug exists.
        let f = found(&[
            (
                "proposal::the_first_reading",
                &[("sentence_kind", "definition"), ("defines", "strategy")],
            ),
            (
                "proposal::a_rival",
                &[
                    ("sentence_kind", "definition"),
                    ("defines", "strategy"),
                    ("supersedes", "a_measured_claim"),
                ],
            ),
            (
                "proposal::a_measured_claim",
                &[("sentence_kind", "measured"), ("defines", "strategy")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_rival"), "{}", f[0]);
    }

    #[test]
    fn a_supersedes_naming_several_rows_is_read_past_the_first() {
        // The entry list is read whole. One that stopped at the first entry
        // would report a pair whose supersession is recorded second, and every
        // arm above plants a single-entry list so none of them can tell.
        let f = found(&[
            (
                "proposal::an_older_reading",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::the_replacement",
                &[
                    ("sentence_kind", "definition"),
                    ("defines", "chain"),
                    ("supersedes", "something_else, an_older_reading"),
                ],
            ),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_definition_with_no_term_is_left_to_the_check_that_names_it() {
        // Two rows defining nothing collide on nothing, and reporting them here
        // would be a second report of a defect `a-definition-names-its-term`
        // already makes, in different words.
        let f = found(&[
            ("proposal::a", &[("sentence_kind", "definition")]),
            ("proposal::b", &[("sentence_kind", "definition")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn two_rows_defining_one_term_where_only_one_is_a_definition_is_not_a_collision() {
        // The discrimination on `sentence_kind`. A `measured` row carrying a
        // `defines` is not a stipulation, and a check keyed on the field alone
        // would report the pair.
        let f = found(&[
            (
                "proposal::a_stipulation",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::a_measurement_about_chains",
                &[("sentence_kind", "measured"), ("defines", "chain")],
            ),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn three_readings_of_one_term_are_one_finding_naming_all_three() {
        let f = found(&[
            (
                "proposal::a",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::b",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::c",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains('3'), "the count is reported: {}", f[0]);
        for who in ["proposal::a", "proposal::b", "proposal::c"] {
            assert!(f[0].contains(who), "{who} is not named: {}", f[0]);
        }
    }

    #[test]
    fn two_terms_each_defined_twice_are_two_findings() {
        let f = found(&[
            (
                "proposal::a",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::b",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::c",
                &[("sentence_kind", "definition"), ("defines", "stretch")],
            ),
            (
                "proposal::d",
                &[("sentence_kind", "definition"), ("defines", "stretch")],
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
            &[
                (
                    "proposal::a",
                    &[("sentence_kind", "definition"), ("defines", "chain")],
                ),
                (
                    "proposal::b",
                    &[("sentence_kind", "definition"), ("defines", "chain")],
                ),
            ],
            &[],
        );
        assert_findings_block(&super::ATermIsDefinedTwice, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ATermIsDefinedTwice);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::ATermIsDefinedTwice.name(), "a-term-is-defined-twice");
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-term-is-defined-twice");
    }
}
