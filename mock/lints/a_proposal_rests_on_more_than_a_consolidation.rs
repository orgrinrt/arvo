//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A proposal cites what established the claim, not only the compression of it.
//!
//! `ruling::the_canon_is_written_once_at_the_end` states it: a consolidation is
//! the topic's best available compression and has no standing beyond that, so
//! it is input rather than canon in miniature. A consolidation names, per
//! sentence, the member files that established each claim. A row citing only
//! the consolidation has replaced those with a pointer at the compression, so
//! the claim survives and the ability to check it does not.
//!
//! **The repair is additive and cheap**, which is why this refuses rather than
//! ranks: keep the consolidation citation, since that is where the wording came
//! from, and add the establishing files it names beside it. It has been done by
//! hand once already, to
//! `proposal::conversion_and_resolution_are_one_obligation_at_two_arities`,
//! whose own note records the repair.
//!
//! **A lint rather than a tool.** There is a state it refuses, and the fix is
//! per row rather than a judgement somebody makes off a ranking.
//!
//! # What the corpus carries, and why the finding blocks a new one only
//!
//! Forty-three committed rows are in this state, and the crate this came from
//! held them under a ratchet whose reasoning it wrote out: the list is not
//! asserted empty because it is not empty, and the work of emptying it is per
//! row and per topic rather than mechanical. One row is deliberately in it and
//! stays, the consolidator's own contribution, where the consolidation genuinely
//! is the source and the flag is honest signal rather than a defect to repair.
//!
//! **So the ceiling is carried across and the severity is not touched.** The
//! alternative considered was declaring the lint at `warn` in `mockspace.toml`,
//! which states the true thing about the backlog and reports every offending row
//! at every gate. It was refused on one ground: a warning refuses nothing, so
//! the forty-third row and the forty-fourth are treated alike, and the
//! forty-fourth is the whole point. Severity is also the one dial this workspace
//! reserves for a human, and a ratchet needs nobody to turn it.
//!
//! The contract's objection to a threshold is real and is answered by where this
//! one came from: it is a measurement of the committed corpus rather than a
//! number somebody chose, it only ever falls, and the finding names every row it
//! counted, so a ceiling that went green because the predicate broke reports
//! zero of forty-three rather than passing quietly.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon agrees with this predicate. A unit test cannot
//! build a `RegistryView` from `mock/registry/`, because that needs a TOML
//! parser the generated pack has no route to depend on. `cargo mock --lint-only`
//! is where the predicate meets the real rows, and it runs this over all of
//! them at every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_citations::{files_cited, is_a_consolidation};
use crate::canon_rows::finding;
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AProposalRestsOnMoreThanAConsolidation { ceiling: CEILING })
}

const LINT: &str = "a-proposal-rests-on-more-than-a-consolidation";

/// The rows in this state, measured over the committed canon.
///
/// **Lower it as topics are second-read; never raise it.** The number moved from
/// 65 to 48 when the number-system topic was second-read, and to this when the
/// registry grew past what that measurement covered.
const CEILING: usize = 43;

/// The lint, carrying the ceiling it grandfathers.
///
/// A field rather than a constant read inside the predicate, so a test can build
/// one at zero and drive the whole lint. A ceiling only ever exercised at the
/// number it was measured with is a ceiling nobody has seen fire.
struct AProposalRestsOnMoreThanAConsolidation {
    ceiling: usize,
}

impl Lint for AProposalRestsOnMoreThanAConsolidation {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AProposalRestsOnMoreThanAConsolidation {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let found = self.every_offending_row(ctx);
        if found.len() <= self.ceiling {
            return Vec::new();
        }
        found
    }
}

impl AProposalRestsOnMoreThanAConsolidation {
    fn every_offending_row(&self, ctx: &RepoContext) -> Vec<LintError> {
        ctx.registry
            .rows_in("proposal")
            .iter()
            .filter_map(|q| {
                let files = files_cited(ctx.registry, q);
                // A row citing nothing is the schema's report to make, and an
                // `all` over an empty set is true, so the emptiness is checked
                // first rather than left to decide it silently.
                if files.is_empty() || !files.iter().all(|f| is_a_consolidation(f)) {
                    return None;
                }
                let named: Vec<&str> = files.iter().copied().collect();
                Some(finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` names {named:?} in `provenance` and nothing else, and every one \
                         of those is a consolidation. A consolidation is the topic's best \
                         available compression and has no standing beyond that, so this row \
                         rests on a restatement rather than on what established the claim. \
                         The consolidation names the establishing files per sentence; add \
                         them beside it."
                    ),
                ))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };
    use crate::canon_rows::JOIN;

    const ROOT: &str = "panel::202608072330_the-numeral-canon-panel";
    const CONSOLIDATION: &str = "74_giesen_consolidation_the_number_system_concept";
    const MEMBER: &str = "71_orchard_what_crosses_between_two_systems";

    fn cited(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        at_a_ceiling_of(0, rows)
    }

    /// The same, with the ratchet set where a test wants it.
    fn at_a_ceiling_of(ceiling: usize, rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::AProposalRestsOnMoreThanAConsolidation { ceiling }
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn the_population_under_the_ceiling_is_silent_and_one_above_it_names_all_of_them() {
        // The ratchet at both sides of its own edge. Under it the backlog is
        // grandfathered; one above it every row is named, so the one that just
        // landed is in the list rather than being a count somebody has to diff.
        let p = cited(&[&format!("{ROOT}::{CONSOLIDATION}::475")]);
        let two: [(&str, &[(&str, &str)]); 2] = [
            ("proposal::a", &[("provenance", &p)]),
            ("proposal::b", &[("provenance", &p)]),
        ];
        assert!(at_a_ceiling_of(2, &two).is_empty(), "two under a ceiling of two");
        let over = at_a_ceiling_of(1, &two);
        assert_eq!(over.len(), 2, "every row is named, not the surplus: {over:?}");
        assert!(over.iter().any(|m| m.contains("proposal::a")), "{over:?}");
        assert!(over.iter().any(|m| m.contains("proposal::b")), "{over:?}");
    }

    #[test]
    fn a_row_resting_only_on_a_consolidation_is_reported() {
        let p = cited(&[&format!("{ROOT}::{CONSOLIDATION}::475")]);
        let f = findings(&[("proposal::a_claim", &[("provenance", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains(CONSOLIDATION), "{}", f[0]);
    }

    /// The repair passes: the consolidation stays and an establishing file
    /// joins it.
    ///
    /// Without this the lint would be telling authors to drop the consolidation
    /// citation, which is the wrong fix. The wording came from the
    /// consolidation and that is worth recording; what was missing is where the
    /// claim came from.
    #[test]
    fn control_a_consolidation_beside_its_source_is_silent() {
        let p = cited(&[
            &format!("{ROOT}::{CONSOLIDATION}::526"),
            &format!("{ROOT}::{MEMBER}::399"),
        ]);
        let f = findings(&[("proposal::a_claim", &[("provenance", &p)])]);
        assert!(
            f.is_empty(),
            "the repair is additive and must not fire: {f:?}"
        );
    }

    /// Two consolidations are still only consolidations.
    ///
    /// The arm a naive "cites more than one file" reading gets wrong, and it is
    /// the reason the test is over what the files are rather than how many.
    #[test]
    fn two_consolidations_and_nothing_else_is_still_reported() {
        let p = cited(&[
            &format!("{ROOT}::{CONSOLIDATION}::526"),
            &format!("{ROOT}::63_spj_consolidation_the_format_concept::120"),
        ]);
        let f = findings(&[("proposal::a_claim", &[("provenance", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn control_a_row_citing_only_member_files_is_silent() {
        let p = cited(&[&format!(
            "{ROOT}::68_leroy_what_the_pipeline_certifies::226"
        )]);
        let f = findings(&[("proposal::a_claim", &[("provenance", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// An entailment check of a consolidation is an independent read and is the
    /// discrimination this lint would be worthless without. It is pinned in
    /// full beside the predicate, in `canon_citations.rs`; this arm is that
    /// predicate reached through the lint rather than directly.
    #[test]
    fn control_an_entailment_check_on_a_consolidation_is_not_one() {
        let p = cited(&[&format!(
            "{ROOT}::75_arntzen_entailment_check_on_the_number_system_consolidation::40"
        )]);
        let f = findings(&[("proposal::a_claim", &[("provenance", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// A row with no `provenance` at all is left to the schema.
    ///
    /// The schema requires the field, so an absent one is already refused, and
    /// firing here would report the same defect twice under two names. It is
    /// also what an `all` over an empty set would do on its own, which is why
    /// the emptiness is tested rather than assumed.
    #[test]
    fn control_a_row_with_no_provenance_is_left_to_the_schema() {
        let f = findings(&[("proposal::a_claim", &[("standing", "one_expert")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_ruling_is_not_read_as_a_proposal() {
        let p = cited(&[&format!("{ROOT}::{CONSOLIDATION}::614")]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let p = format!("{ROOT}::{CONSOLIDATION}::475");
        let v = view(&[("proposal::a", &[("provenance", &p)])], &[]);
        assert_findings_block(&super::AProposalRestsOnMoreThanAConsolidation { ceiling: 0 }, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AProposalRestsOnMoreThanAConsolidation { ceiling: super::CEILING });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AProposalRestsOnMoreThanAConsolidation { ceiling: 0 }.name(),
            "a-proposal-rests-on-more-than-a-consolidation"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-proposal-rests-on-more-than-a-consolidation");
    }
}
