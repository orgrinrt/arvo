//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A claim that ran something, with no instrument behind it.
//!
//! The mark is what a later reader trusts. A row saying `measured` and citing
//! no probe is asking for a measurement's authority on an argument's evidence,
//! and the marking convention exists precisely because a canon stating a
//! theorem and a measurement in one voice loses the distinction.
//!
//! **A lint rather than a tool.** Two edits close it and both are edits to the
//! row: name the committed instrument, or mark the sentence as the argument it
//! is. Nothing here is a judgement somebody has to make, and nothing needs a
//! question.
//!
//! **`enumeration` is read alongside `measured` because this corpus calls its
//! sweeps enumerations**, which is what they are: somebody walked a bounded set
//! and reported what was in it. A walk over 4096 triples is an instrument
//! however its author labelled the sentence, and gating `measured` alone left
//! the check reaching almost nothing the corpus had actually run. Reported by
//! the seat that met it and found the gate had no purchase on its material.
//!
//! `theorem` is not read: a proof owes its route, not a run. Neither is
//! `argument`, which claims no run, nor `normative`, which claims nothing at
//! all.
//!
//! **The ceiling is the canon's own, not one invented here.** The `proposal`
//! registry's header says it in its own words: the rows that name no probe "sit
//! under a ratchet ... whose ceiling admits no new arrival rather than
//! tolerating them", and that "it currently holds with no headroom, so the next
//! `measured` row landing without a probe turns it red, which is the ratchet
//! working rather than a regression." So the refused state is a new arrival, and
//! the population under the ceiling is the six rows whose own documentation says
//! why each is stuck. A lint reporting all six at every gate would refuse a
//! state the canon describes as the current one, and lowering the severity to
//! get past that is the downgrade the workspace reserves for a human.
//!
//! **What the port loses, said plainly.** The crate this came from carried the
//! ceiling and a control asserting the finder still found the six, so a green
//! ceiling could not pass vacuously. The control does not travel into a unit
//! test here, because a `RegistryView` over the real registry needs a TOML
//! parser the generated pack may not depend on. What replaces it is that the
//! finding names every row it counted, so a ceiling that went green because the
//! predicate stopped finding anything reports zero of six rather than passing
//! quietly, and the ratchet itself is driven at a ceiling of zero in the tests
//! below.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(MeasuredClaimCitesNoProbe { ceiling: CEILING })
}

const LINT: &str = "measured-claim-cites-no-probe";

/// The namespace whose rows carry a sentence kind and an evidence edge.
const NAMESPACE: &str = "proposal";

/// The sentence kinds that ran an instrument, and therefore owe one.
const RAN_SOMETHING: [&str; 2] = ["measured", "enumeration"];

/// The rows standing unwired, measured over the committed canon.
///
/// **Lower it as rows are wired; never raise it.** The canon's own header calls
/// this a ratchet holding with no headroom, so the next `measured` row landing
/// without a probe turns it red, which is the ratchet working rather than a
/// regression.
const CEILING: usize = 6;

/// The lint, carrying the ceiling it grandfathers.
///
/// A field rather than a constant read inside the predicate, so a test can build
/// one at zero and drive the whole lint. A ceiling only ever exercised at the
/// number it was measured with is a ceiling nobody has seen fire.
struct MeasuredClaimCitesNoProbe {
    ceiling: usize,
}

impl Lint for MeasuredClaimCitesNoProbe {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for MeasuredClaimCitesNoProbe {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let unwired: Vec<String> = ctx
            .registry
            .rows_in(NAMESPACE)
            .iter()
            .filter_map(|q| {
                let kind = text(ctx.registry, q, "sentence_kind")?;
                if !RAN_SOMETHING.contains(&kind) {
                    return None;
                }
                if !list(ctx.registry, q, "evidence").is_empty() {
                    return None;
                }
                Some(format!("{q} ({kind})"))
            })
            .collect();
        if unwired.len() <= self.ceiling {
            return Vec::new();
        }
        vec![finding(
            LINT,
            None,
            format!(
                "{} rows have a `sentence_kind` that ran something and an empty `evidence`, \
                 against a ceiling of {}. Name the committed instrument on the one that just \
                 landed, or mark its sentence as the argument it is. A measurement with no \
                 instrument is an argument wearing a number, and a sweep is a measurement \
                 whatever its author called the sentence. Raising the number is not the fix. \
                 The rows: {unwired:?}",
                unwired.len(),
                self.ceiling
            ),
        )]
    }
}
#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use super::MeasuredClaimCitesNoProbe as Unwired;
    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx, view,
    };

    /// The messages one planted registry produces at a given ceiling.
    fn found(rows: &[(&str, &[(&str, &str)])], ceiling: usize) -> Vec<String> {
        let v = view(rows, &[]);
        Unwired { ceiling }
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
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
    fn a_measurement_with_no_instrument_is_counted_and_an_argument_is_not() {
        let named = names(&[
            ("proposal::a_bare_number", &[("sentence_kind", "measured")]),
            (
                "proposal::a_reasoned_claim",
                &[("sentence_kind", "argument")],
            ),
            (
                "proposal::a_real_measurement",
                &[("sentence_kind", "measured"), ("evidence", "the_sweep")],
            ),
        ]);
        assert!(named.contains("proposal::a_bare_number"), "{named}");
        assert!(!named.contains("a_reasoned_claim"), "{named}");
        assert!(!named.contains("a_real_measurement"), "{named}");
        assert!(named.contains("1 rows"), "the count is the population: {named}");
    }

    #[test]
    fn an_enumeration_owes_an_instrument_and_a_theorem_does_not() {
        // A sweep is a measurement whatever the sentence was called. Gating
        // `measured` alone leaves the arm with no purchase on most of what this
        // corpus ran, and every other arm here would still pass.
        let named = names(&[
            (
                "proposal::a_sweep_with_nothing_behind_it",
                &[("sentence_kind", "enumeration")],
            ),
            (
                "proposal::a_sweep_that_names_its_run",
                &[("sentence_kind", "enumeration"), ("evidence", "the_walk")],
            ),
            ("proposal::a_proof", &[("sentence_kind", "theorem")]),
            (
                "proposal::an_imposed_one",
                &[("sentence_kind", "normative")],
            ),
        ]);
        assert!(named.contains("a_sweep_with_nothing_behind_it"), "{named}");
        assert!(!named.contains("a_sweep_that_names_its_run"), "{named}");
        assert!(!named.contains("a_proof"), "{named}");
        assert!(!named.contains("an_imposed_one"), "{named}");
        assert!(
            named.contains("enumeration"),
            "the report names the kind, or a reader fixing it looks for the wrong field: {named}"
        );
    }

    #[test]
    fn an_evidence_field_present_and_holding_nothing_is_no_instrument() {
        // The engine writes an empty array as the empty string, so a row that
        // carries the edge and fills it with nothing arrives looking exactly
        // like one that never wrote it. Both are the state this counts.
        for blank in ["", " ", ", ", "  ,  "] {
            let named = names(&[(
                "proposal::p",
                &[("sentence_kind", "measured"), ("evidence", blank)],
            )]);
            assert!(
                named.contains("proposal::p"),
                "{blank:?} passed as an instrument: {named}"
            );
        }
    }

    #[test]
    fn a_row_with_no_sentence_kind_at_all_owes_nothing() {
        assert!(found(&[("proposal::p", &[("says", "something")])], 0).is_empty());
    }

    #[test]
    fn a_ruling_is_not_asked_for_an_instrument() {
        // Only `proposal` declares `sentence_kind` and `evidence`. A lint
        // reading every namespace would be deciding a rule for namespaces it
        // was not written for, and the arms above cannot tell, because each
        // plants one namespace.
        assert!(found(&[("ruling::r", &[("sentence_kind", "measured")])], 0).is_empty());
    }

    #[test]
    fn every_offending_row_is_counted_rather_than_the_first() {
        let named = names(&[
            ("proposal::a", &[("sentence_kind", "measured")]),
            ("proposal::b", &[("sentence_kind", "enumeration")]),
        ]);
        assert!(named.contains("2 rows"), "{named}");
        assert!(named.contains("proposal::a"), "{named}");
        assert!(named.contains("proposal::b"), "{named}");
    }

    #[test]
    fn the_population_under_the_ceiling_is_silent_and_one_above_it_is_not() {
        // The ratchet, driven at both sides of its own edge. The canon's header
        // calls this a ceiling admitting no new arrival, so the population it
        // grandfathers must pass and one more must not.
        let two = [
            ("proposal::a", &[("sentence_kind", "measured")] as &[(&str, &str)]),
            ("proposal::b", &[("sentence_kind", "measured")]),
        ];
        assert!(found(&two, 2).is_empty(), "two under a ceiling of two");
        let over = found(&two, 1);
        assert_eq!(over.len(), 1, "{over:?}");
        assert!(over[0].contains("ceiling of 1"), "{}", over[0]);
    }

    #[test]
    fn a_ceiling_reached_exactly_is_the_grandfathered_state_rather_than_a_finding() {
        // The off-by-one that decides whether the current corpus commits at
        // all. The canon says the ceiling holds with no headroom, so equal has
        // to pass and greater has to fail.
        let one = [("proposal::a", &[("sentence_kind", "measured")] as &[(&str, &str)])];
        assert!(found(&one, 1).is_empty());
        assert_eq!(found(&one, 0).len(), 1);
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[], 0).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("proposal::p", &[("sentence_kind", "measured")])], &[]);
        assert_findings_block_at(&Unwired { ceiling: 0 }, &ctx(&v));
    }

    #[test]
    fn the_committed_ceiling_is_what_the_registered_lint_carries() {
        assert_eq!(super::CEILING, 6);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&Unwired {
            ceiling: super::CEILING,
        });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            Unwired {
                ceiling: super::CEILING
            }
            .name(),
            "measured-claim-cites-no-probe"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("measured-claim-cites-no-probe");
    }
}
