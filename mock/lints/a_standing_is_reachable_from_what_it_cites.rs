//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A standing asserting several arrivals cites more than one file.
//!
//! `standing` is the field that decides whether a proposal is close to canon,
//! and the schema constrains its spelling and nothing else. A row may say
//! `cross_topic`, the strongest tier the panel produces, while citing one file
//! once, and every gate stays green. That was measured rather than assumed: the
//! transcript is in `214_probes/standing_is_unconstrained.txt`, together with
//! the positive control setting the same field to `seventeen_experts` and
//! watching the schema refuse it by name, so the green is a fact about what the
//! gate reads rather than about whether it runs.
//!
//! Independence is between authors, and a numbered member file has one author,
//! so one citation cannot exhibit two arrivals however the note describes them.
//! The row may still be right. What it cannot be is checked, and a claim nobody
//! can locate is one nobody can overturn.
//!
//! **A necessary condition rather than a sufficient one.** Nothing here can
//! tell whether two authors really arrived separately, and it does not pretend
//! to. What it establishes is that a reader could go and find out.
//!
//! **A lint rather than a tool.** There is a state it refuses: a row claiming
//! more arrivals than its citations can carry. The fix is per row and it is
//! additive, so the answer is not an inventory somebody ranks.
//!
//! # What the corpus carries, and why the finding blocks a new one only
//!
//! Twenty-nine committed rows are in this state, and the crate this came from
//! held them under a ratchet with its reasoning written out: the list is not
//! asserted empty because it is not empty, and the work of emptying it is per
//! row and per topic rather than mechanical. What was pinned is that it does not
//! grow, because a row porting a claim out of a consolidation without its
//! establishing files is added one row at a time by whoever is porting that
//! topic.
//!
//! **So the ceiling is carried across and the severity is not touched.** The
//! alternative considered was declaring the lint at `warn` in `mockspace.toml`,
//! which states the true thing about the backlog and reports every offending row
//! at every gate. It was refused on one ground: a warning refuses nothing, so
//! the twenty-ninth row and the thirtieth are treated alike, and the thirtieth
//! is the whole point. Severity is also the one dial this workspace reserves for
//! a human, and a ratchet needs nobody to turn it.
//!
//! The contract's objection to a threshold is real and is answered by where this
//! one came from: it is a measurement of the committed corpus rather than a
//! number somebody chose, it only ever falls, and the finding names every row it
//! counted, so a ceiling that went green because the predicate broke reports
//! zero of twenty-nine rather than passing quietly.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon agrees with this predicate. A unit test cannot
//! build a `RegistryView` from `mock/registry/`, because that needs a TOML
//! parser the generated pack has no route to depend on. `cargo mock --lint-only`
//! is where the predicate meets the real rows, and it runs this over all of
//! them at every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_citations::files_cited;
use crate::canon_rows::{finding, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AStandingIsReachableFromWhatItCites { ceiling: CEILING })
}

/// The rows in this state, measured over the committed canon.
///
/// **Lower it as topics are second-read; never raise it.** Under the rule that
/// two agreeing experts ratify, these are exactly the rows eligible for
/// promotion, and not one of them names a second source.
const CEILING: usize = 29;

const LINT: &str = "a-standing-is-reachable-from-what-it-cites";

/// The standings that assert more than one independent arrival.
///
/// `contested` is absent on purpose: it says somebody disagreed, which is not a
/// count of arrivals and is not inflated by resting on one file.
const MULTI_ARRIVAL: [&str; 3] = ["two_experts", "three_or_more", "cross_topic"];

/// The lint, carrying the ceiling it grandfathers.
///
/// A field rather than a constant read inside the predicate, so a test can build
/// one at zero and drive the whole lint. A ceiling only ever exercised at the
/// number it was measured with is a ceiling nobody has seen fire.
struct AStandingIsReachableFromWhatItCites {
    ceiling: usize,
}

impl Lint for AStandingIsReachableFromWhatItCites {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AStandingIsReachableFromWhatItCites {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let found: Vec<LintError> = ctx
            .registry
            .rows_in("proposal")
            .iter()
            .filter_map(|q| check(ctx.registry, q))
            .collect();
        if found.len() <= self.ceiling {
            return Vec::new();
        }
        found
    }
}

/// One proposal, against the citations its standing would need.
fn check(reg: &mockspace::RegistryView, q: &str) -> Option<LintError> {
    let standing = text(reg, q, "standing")?;
    if !MULTI_ARRIVAL.contains(&standing) {
        return None;
    }
    let files = files_cited(reg, q);
    if files.len() >= 2 {
        return None;
    }
    let named = files.iter().copied().next().unwrap_or("nothing at all");
    Some(finding(
        LINT,
        None,
        format!(
            "`{q}` sets `standing` to `{standing}`, which asserts more than one independent \
             arrival, and `provenance` names `{named}` and no second file. One file has one \
             author, so a reader cannot reach the second arrival from this row. Cite the \
             files that reached it, or state the standing the citations support."
        ),
    ))
}

#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };
    use crate::canon_rows::JOIN;

    const ROOT: &str = "panel::202608072330_the-numeral-canon-panel";

    /// A `provenance` field as the row wrote it, joined the way the engine
    /// joins one before a lint ever sees it.
    fn cited(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        at_a_ceiling_of(0, rows)
    }

    /// The same, with the ratchet set where a test wants it.
    fn at_a_ceiling_of(ceiling: usize, rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::AStandingIsReachableFromWhatItCites { ceiling }
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
        let p = cited(&[&format!("{ROOT}::74_giesen_consolidation::614")]);
        let two: [(&str, &[(&str, &str)]); 2] = [
            (
                "proposal::a",
                &[("standing", "two_experts"), ("provenance", &p)],
            ),
            (
                "proposal::b",
                &[("standing", "cross_topic"), ("provenance", &p)],
            ),
        ];
        assert!(at_a_ceiling_of(2, &two).is_empty(), "two under a ceiling of two");
        let over = at_a_ceiling_of(1, &two);
        assert_eq!(over.len(), 2, "every row is named, not the surplus: {over:?}");
        assert!(over.iter().any(|m| m.contains("proposal::a")), "{over:?}");
        assert!(over.iter().any(|m| m.contains("proposal::b")), "{over:?}");
    }

    #[test]
    fn a_two_expert_row_citing_one_file_is_reported() {
        let p = cited(&[&format!(
            "{ROOT}::74_giesen_consolidation_the_number_system::614"
        )]);
        let f = findings(&[(
            "proposal::a_claim",
            &[("standing", "two_experts"), ("provenance", &p)],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("two_experts"), "{}", f[0]);
        assert!(f[0].contains("proposal::a_claim"), "{}", f[0]);
    }

    /// The control that makes the arm above mean something: two files pass.
    ///
    /// An arm refusing every multi-arrival standing would be refusing the tier
    /// rather than checking it, and would read exactly the same on a clean
    /// canon.
    #[test]
    fn control_a_two_expert_row_citing_two_files_is_silent() {
        let p = cited(&[
            &format!("{ROOT}::65_knuth_number_systems_derived_cold::519"),
            &format!("{ROOT}::66_dolan_number_systems_derived_cold::255"),
        ]);
        let f = findings(&[(
            "proposal::a_claim",
            &[("standing", "two_experts"), ("provenance", &p)],
        )]);
        assert!(
            f.is_empty(),
            "two files is what a two-expert standing names: {f:?}"
        );
    }

    /// Two citations into one file are one file, which is the case a count of
    /// `provenance` entries gets wrong and the case the registry contains.
    #[test]
    fn two_anchors_into_one_file_are_one_source() {
        let one = format!("{ROOT}::74_giesen_consolidation_the_number_system");
        let p = cited(&[&format!("{one}::543"), &format!("{one}::251")]);
        let f = findings(&[(
            "proposal::a_claim",
            &[("standing", "two_experts"), ("provenance", &p)],
        )]);
        assert_eq!(f.len(), 1, "two anchors into one file is one author: {f:?}");
    }

    #[test]
    fn control_a_one_expert_row_citing_one_file_is_silent() {
        let p = cited(&[&format!(
            "{ROOT}::67_kiselyov_which_prefix_earns_the_word::613"
        )]);
        let f = findings(&[(
            "proposal::a_claim",
            &[("standing", "one_expert"), ("provenance", &p)],
        )]);
        assert!(f.is_empty(), "the ordinary honest row: {f:?}");
    }

    /// `contested` says somebody disagreed rather than that several arrived, so
    /// it is not inflated by resting on one file and must not be reported.
    #[test]
    fn control_a_contested_row_citing_one_file_is_silent() {
        let p = cited(&[&format!(
            "{ROOT}::67_kiselyov_which_prefix_earns_the_word::613"
        )]);
        let f = findings(&[(
            "proposal::a_claim",
            &[("standing", "contested"), ("provenance", &p)],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Every one of the three multi-arrival standings is read, rather than the
    /// first of them. A list written and then half-consulted reads exactly like
    /// a complete one.
    #[test]
    fn all_three_multi_arrival_standings_are_read() {
        let p = cited(&[&format!("{ROOT}::74_giesen_consolidation::614")]);
        for standing in super::MULTI_ARRIVAL {
            let f = findings(&[(
                "proposal::a_claim",
                &[("standing", standing), ("provenance", &p)],
            )]);
            assert_eq!(f.len(), 1, "`{standing}` was not read: {f:?}");
        }
    }

    /// A row with no `provenance` at all is reported, and says so.
    ///
    /// The schema requires the field, so this is the shape a row takes while it
    /// is being written rather than one the canon carries. It is still the
    /// defect the lint names, and reporting `nothing at all` is more use than
    /// silence.
    #[test]
    fn a_row_asserting_arrivals_and_citing_nothing_is_reported() {
        let f = findings(&[("proposal::a_claim", &[("standing", "three_or_more")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("nothing at all"), "{}", f[0]);
    }

    /// Only `proposal` is read.
    ///
    /// A ruling cites op's own file and a probe's `lives` is not the provenance
    /// of a claim. A walk over every row would report both and mean nothing by
    /// it.
    #[test]
    fn control_a_ruling_is_not_read_as_a_proposal() {
        let p = cited(&[&format!("{ROOT}::74_giesen_consolidation::614")]);
        let f = findings(&[(
            "ruling::a_call",
            &[("standing", "two_experts"), ("provenance", &p)],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_proposal_with_no_standing_owes_nothing() {
        let f = findings(&[("proposal::a_claim", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        // The severity that decides a refusal is the one the FINDING carries,
        // not the one `default_severity` returns. That `mockspace.toml`
        // currently restamps these to a warning is the repository's call about
        // a backlog and does not change what the finding is worth.
        let v = view(
            &[(
                "proposal::a",
                &[
                    ("standing", "two_experts"),
                    ("provenance", "panel::one_file"),
                ],
            )],
            &[],
        );
        assert_findings_block(&super::AStandingIsReachableFromWhatItCites { ceiling: 0 }, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AStandingIsReachableFromWhatItCites { ceiling: super::CEILING });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AStandingIsReachableFromWhatItCites { ceiling: 0 }.name(),
            "a-standing-is-reachable-from-what-it-cites"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-standing-is-reachable-from-what-it-cites");
    }
}
