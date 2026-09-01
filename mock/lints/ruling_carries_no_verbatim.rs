//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A ruling recording op's authority with no words of his behind it.
//!
//! `says` is required and `quote` is not, which inverts the trust order: a row
//! can carry somebody's restatement of him, pass every schema check, and be
//! mechanically indistinguishable from one that was invented. The first port of
//! this corpus landed four of exactly that shape, and one of them governs when
//! anything becomes canon at all, its only record being an agent's sentence
//! reporting which option he took.
//!
//! **A lint rather than a tool.** The state refused is a row claiming his
//! authority with nothing checkable behind it, and the repair is one of two
//! edits to that row: quote the source, or write into `note` that the corpus
//! holds no verbatim and what it holds instead. Neither needs a question from
//! anybody and neither wants a threshold.
//!
//! **A ruling the experts ratified is out of scope by construction.** Until
//! ratification widened, every ruling was his and a missing quote meant a
//! missing source. `ratified_by = "experts"` is now a route of its own: the
//! experts propose, the coordinator gates, and he is not in it, so such a row
//! carries a `promotion` recording the judgement and there is no verbatim to
//! have lost. Filtered rather than reported, because a finding is something
//! somebody should go and fix and this is not one.
//!
//! **The rows already standing sit under a ratchet rather than under a report.**
//! Seven of them do, and each is a ratified ruling whose only record of him is
//! an agent's sentence. A lint reporting all seven at every gate would refuse a
//! state that is simply the current one, and lowering the severity to get past
//! that is the downgrade this workspace reserves for a human. So the ceiling
//! admits what stands and refuses the next arrival, which is the property worth
//! gating: a new ruling claiming his authority with nothing checkable behind it.
//!
//! **Lower it as rows are quoted, and never raise it.** The repair per row is
//! one edit: find his words in the corpus and put them in `quote`, or write into
//! `note` that no verbatim survives and what stands in its place. Raising the
//! number instead is the thing the ratchet exists to make visible.
//!
//! **What the ceiling cannot check here.** The crate this came from pinned the
//! standing rows by name in a test, which a lint's own unit tests cannot do: a
//! `RegistryView` over the real registry needs a TOML parser the generated pack
//! may not depend on. What replaces it is that the finding names every row it
//! counted, so a ceiling that went green because the predicate stopped matching
//! reports zero of seven rather than passing quietly, and the ratchet itself is
//! driven at a ceiling of zero in the tests below.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(RulingCarriesNoVerbatim { ceiling: CEILING })
}

const LINT: &str = "ruling-carries-no-verbatim";

/// The namespace whose rows claim his authority.
///
/// A `proposal` carries no `quote` by construction, because there are no words
/// but the panel's and `says` holds them. Reading it here would report the
/// namespace rather than a defect in any row of it.
const NAMESPACE: &str = "ruling";

/// The ratification route that never passed through him.
const NOT_HIS: &str = "experts";

/// The rows standing without a verbatim, counted over the committed canon.
///
/// **Lower it as rows are quoted; never raise it.** Every one of these is a
/// ratified ruling resting on somebody's restatement of him, so the number going
/// down is the corpus getting its provenance back and the number going up is the
/// defect arriving again.
const CEILING: usize = 7;

/// The lint, carrying the ceiling it grandfathers.
///
/// A field rather than a constant read inside the predicate, so a test can build
/// one at zero and drive the whole lint. A ceiling only ever exercised at the
/// number it was measured with is a ceiling nobody has seen fire.
struct RulingCarriesNoVerbatim {
    ceiling: usize,
}
impl Lint for RulingCarriesNoVerbatim {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for RulingCarriesNoVerbatim {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let unquoted: Vec<String> = ctx
            .registry
            .rows_in(NAMESPACE)
            .iter()
            .filter(|q| ctx.registry.field(q, "ratified_by") != Some(NOT_HIS))
            .filter(|q| text(ctx.registry, q, "quote").is_none())
            .map(|q| q.to_string())
            .collect();
        if unquoted.len() <= self.ceiling {
            return Vec::new();
        }
        vec![finding(
            LINT,
            None,
            format!(
                "{} rulings set `says` and carry no `quote`, against a ceiling of {}. What \
                 stands behind each is somebody's restatement of him, which passes every \
                 schema check and is mechanically indistinguishable from an invention. Quote \
                 the source on the one that just landed, or record in `note` that the corpus \
                 holds no verbatim and what it holds instead. Raising the number is not the \
                 fix. The rows: {unquoted:?}",
                unquoted.len(),
                self.ceiling
            ),
        )]
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use super::RulingCarriesNoVerbatim as Unquoted;
    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    /// The messages one planted registry produces at a given ceiling.
    fn found(rows: &[(&str, &[(&str, &str)])], ceiling: usize) -> Vec<String> {
        findings(&Unquoted { ceiling }, &view(rows, &[]))
    }

    /// The rows the report named, recovered from its own text.
    ///
    /// A ratchet reports one finding over a population, so an arm asserting the
    /// count of findings says nothing about which rows were counted. This is
    /// what lets the discriminations below be about the predicate.
    fn names(rows: &[(&str, &[(&str, &str)])]) -> String {
        found(rows, 0).join(" ")
    }

    #[test]
    fn a_ruling_with_a_quote_is_not_reported_and_one_without_is() {
        let f = found(
            &[
                (
                    "ruling::his_own_words",
                    &[
                        ("says", "the strategy set is not closed at four"),
                        ("quote", "the strategy set is not closed at exactly four"),
                    ],
                ),
                (
                    "ruling::somebody_elses_words",
                    &[("says", "he took the third option")],
                ),
            ],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ruling::somebody_elses_words"), "{}", f[0]);
        assert!(f[0].contains("quote"), "{}", f[0]);
    }

    #[test]
    fn a_quote_that_is_only_whitespace_is_no_verbatim_and_fires() {
        // The field being present is not what was asked for. A row carrying a
        // blank `quote` satisfies a predicate written as "has the field" and
        // tells a reader exactly what a missing one does.
        for blank in ["", " ", "\n", "\t  \n"] {
            let f = found(
                &[("ruling::r", &[("says", "something"), ("quote", blank)])],
                0,
            );
            assert_eq!(f.len(), 1, "{blank:?} passed as a verbatim");
        }
    }

    #[test]
    fn control_a_ruling_the_experts_ratified_is_not_asked_for_one() {
        // The carve-out, which nothing exercised in the crate this came from.
        // A row the experts promoted was never a call of his, so there is no
        // verbatim to have lost and reporting it would be the old model still
        // running.
        let f = found(
            &[(
                "ruling::the_experts_promoted_it",
                &[("says", "something"), ("ratified_by", "experts")],
            )],
            0,
        );
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_ratified_by_that_is_not_experts_is_still_asked() {
        // The discrimination the carve-out rests on. A filter written as
        // "skip anything carrying `ratified_by`" passes the arm above and
        // silently excuses every row that names him in that field.
        for who in ["op", "orgrinrt", ""] {
            let f = found(
                &[("ruling::r", &[("says", "something"), ("ratified_by", who)])],
                0,
            );
            assert_eq!(f.len(), 1, "`ratified_by = {who:?}` was excused: {f:?}");
        }
    }

    #[test]
    fn a_proposal_is_not_asked_for_a_verbatim() {
        // A proposal has no `quote` by construction: there are no words but the
        // panel's, which is what `says` holds. A lint reading every namespace
        // would report the whole of that one.
        let f = found(
            &[(
                "proposal::a_claim",
                &[(
                    "says",
                    "the partition is derivable without the observability rule",
                )],
            )],
            0,
        );
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_offending_ruling_is_named_rather_than_the_first() {
        let rows: &[(&str, &[(&str, &str)])] = &[
            ("ruling::a", &[("says", "one")]),
            ("ruling::b", &[("says", "two")]),
            ("ruling::c", &[("says", "three"), ("quote", "his words")]),
        ];
        let n = names(rows);
        assert!(n.contains("ruling::a"), "{n}");
        assert!(n.contains("ruling::b"), "{n}");
        assert!(!n.contains("ruling::c"), "{n}");
        assert!(
            n.contains('2'),
            "the count of what it found is missing: {n}"
        );
    }

    #[test]
    fn the_ratchet_admits_at_its_ceiling_and_refuses_the_next_arrival() {
        // The whole point of the ceiling, and the arm the number it ships with
        // never exercises. Two standing rows pass at a ceiling of two; a third
        // arriving turns it red.
        let two: &[(&str, &[(&str, &str)])] = &[
            ("ruling::a", &[("says", "one")]),
            ("ruling::b", &[("says", "two")]),
        ];
        assert!(found(two, 2).is_empty(), "the standing rows were refused");
        let three: &[(&str, &[(&str, &str)])] = &[
            ("ruling::a", &[("says", "one")]),
            ("ruling::b", &[("says", "two")]),
            ("ruling::c", &[("says", "three")]),
        ];
        let f = found(three, 2);
        assert_eq!(f.len(), 1, "the arrival was admitted: {f:?}");
        assert!(f[0].contains("ceiling of 2"), "{}", f[0]);
        assert!(f[0].contains("ruling::c"), "{}", f[0]);
    }

    #[test]
    fn a_ceiling_going_green_because_nothing_matched_still_reports_the_count() {
        // A ratchet that stops finding anything passes exactly like one whose
        // population went to zero, so the honest reading is that the report
        // names what it counted. At a ceiling of zero an empty registry is
        // silent, which is the same shape, and the discrimination is that a
        // populated one is not.
        assert!(found(&[], 0).is_empty());
        assert!(!names(&[("ruling::a", &[("says", "one")])]).is_empty());
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[], 0).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        // The severity that decides a refusal is the one the FINDING carries,
        // not the one `default_severity` returns.
        let v = view(&[("ruling::r", &[("says", "something")])], &[]);
        assert_findings_block(&Unquoted { ceiling: 0 }, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&Unquoted {
            ceiling: super::CEILING,
        });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            Unquoted {
                ceiling: super::CEILING
            }
            .name(),
            "ruling-carries-no-verbatim"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("ruling-carries-no-verbatim");
    }
}
