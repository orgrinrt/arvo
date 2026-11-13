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

use crate::canon_citations::{
    Consolidation,
    arrivals,
    consolidation_numbers,
    files_cited,
    is_a_consolidation,
    members_named,
    number_of,
};
use crate::canon_rows::{finding, text};
use crate::panel_corpus::{markdown, panel_dir};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AStandingIsReachableFromWhatItCites {
        ceiling: CEILING,
    })
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
        let panel = markdown(ctx.view(), &panel_dir(ctx.mock_dir));
        let names: Vec<&str> = panel
            .iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
            .collect();
        let numbers = consolidation_numbers(names.iter().copied());
        let consolidations: Vec<Consolidation> = panel
            .iter()
            .filter_map(|p| {
                let name = p.file_name()?.to_str()?;
                if !is_a_consolidation(name) {
                    return None;
                }
                let number = number_of(name)?;
                let text = ctx.view().text(p)?;
                Some(Consolidation {
                    number,
                    members: members_named(&text, number, &numbers),
                })
            })
            .collect();
        let found: Vec<LintError> = ctx
            .registry()
            .rows_in("proposal")
            .iter()
            .filter_map(|q| check(ctx.registry(), q, &consolidations))
            .collect();
        if found.len() <= self.ceiling {
            return Vec::new();
        }
        found
    }
}

/// One proposal, against the citations its standing would need.
fn check(
    reg: &mockspace::RegistryView,
    q: &str,
    consolidations: &[Consolidation],
) -> Option<LintError> {
    let standing = text(reg, q, "standing")?;
    if !MULTI_ARRIVAL.contains(&standing) {
        return None;
    }
    let files = files_cited(reg, q);
    if arrivals(&files, consolidations) >= 2 {
        return None;
    }
    let named = files.iter().copied().collect::<Vec<_>>().join("`, `");
    let named = if named.is_empty() { "nothing at all".to_string() } else { named };
    Some(finding(
        LINT,
        None,
        format!(
            "`{q}` sets `standing` to `{standing}`, which asserts more than one independent \
             arrival, and `provenance` names `{named}`, which is one arrival: one file has one \
             author, and a member file with the consolidation over it is one author's account \
             twice. A reader cannot reach a second arrival from this row. Cite the files that \
             reached it, or state the standing the citations support."
        ),
    ))
}

#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        Judge,
        assert_findings_block,
        assert_not_declared_off,
        assert_registered,
        ctx,
        ctx_at,
        plant,
        planted_tree,
        view,
    };
    use crate::canon_rows::JOIN;
    use crate::panel_corpus::PANEL;

    const ROOT: &str = "panel::202608072330_the-numeral-canon-panel";

    /// A `provenance` field as the row wrote it, joined the way the engine
    /// joins one before a lint ever sees it.
    fn cited(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        at_a_ceiling_of(0, rows)
    }

    /// The panel files the sitting-shaped tests plant, each with the text that
    /// matters here: two units, each closing on a consolidation whose preamble
    /// names its members, a later check of one of them that no consolidation
    /// names, and a ledger that belongs to neither.
    const SITTING: [(&str, &str); 9] = [
        (
            "63_spj_consolidation_the_format_concept.md",
            "# 63. Consolidation\n\n**Position:** members `55`, `56`.\n\n## 1. Body\n",
        ),
        (
            "79_fallin_attacking_the_two_cold_derivations_on_derived_laws.md",
            "",
        ),
        (
            "90_giesen_consolidation_derived_algebraic_laws.md",
            "# 90. Consolidation\n\n**Position:** compressed: `76`, `77`, `79`, `80`, and the \
             consolidation `63` read as input.\n\n## 1. Body\n\nLater `91` is cited here.\n",
        ),
        (
            "91_ringer_entailment_check_on_the_derived_laws_consolidation.md",
            "",
        ),
        ("98_second_read.md", ""),
        (
            "106_giesen_consolidation_the_strategy_axis.md",
            "# 106\n\n**What it compresses.** `93` and `94`, the pair, and `97`, `98`.\n\n## 1. \
             Body\n",
        ),
        ("215_kiselyov_second_read_the_algebraic_laws.md", ""),
        (
            "244_orchard_consolidation_admission_and_the_number_system.md",
            "# 244\n\nThe sitting on admission: `241`, `242` and `243`.\n\n## Gates\n",
        ),
        ("AGREEMENTS.md", ""),
    ];

    /// The same as `findings`, over a panel directory holding `panel`.
    fn findings_over(panel: &[(&str, &str)], rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let dir = planted_tree("standing-panel");
        for (file, text) in panel {
            plant(&dir, &format!("{PANEL}/{file}"), text);
        }
        let v = view(rows, &[]);
        super::AStandingIsReachableFromWhatItCites {
            ceiling: 0,
        }
        .judge(ctx_at(&dir, &v))
        .into_iter()
        .map(|e| e.message)
        .collect()
    }

    /// The same, with the ratchet set where a test wants it.
    fn at_a_ceiling_of(ceiling: usize, rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::AStandingIsReachableFromWhatItCites {
            ceiling,
        }
        .judge(ctx(&v))
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
            ("proposal::a", &[
                ("standing", "two_experts"),
                ("provenance", &p),
            ]),
            ("proposal::b", &[
                ("standing", "cross_topic"),
                ("provenance", &p),
            ]),
        ];
        assert!(
            at_a_ceiling_of(2, &two).is_empty(),
            "two under a ceiling of two"
        );
        let over = at_a_ceiling_of(1, &two);
        assert_eq!(
            over.len(),
            2,
            "every row is named, not the surplus: {over:?}"
        );
        assert!(over.iter().any(|m| m.contains("proposal::a")), "{over:?}");
        assert!(over.iter().any(|m| m.contains("proposal::b")), "{over:?}");
    }

    #[test]
    fn a_two_expert_row_citing_one_file_is_reported() {
        let p = cited(&[&format!("{ROOT}::74_giesen_consolidation_the_number_system::614")]);
        let f = findings(&[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
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
        let f = findings(&[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
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
        let f = findings(&[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
        assert_eq!(f.len(), 1, "two anchors into one file is one author: {f:?}");
    }

    /// A member file plus the consolidation over that member is one arrival.
    ///
    /// Two distinct files, so a count of files passes it, and the second is the
    /// first's own compression, so the two agree by construction rather than by
    /// arriving separately. Telling that from two topics arriving apart needs
    /// the members a consolidation says it compresses, which the lint reads
    /// from its own text.
    #[test]
    fn a_file_and_the_consolidation_over_it_are_one_arrival() {
        let p = cited(&[
            &format!("{ROOT}::79_fallin_attacking_the_two_cold_derivations_on_derived_laws::27"),
            &format!("{ROOT}::90_giesen_consolidation_derived_algebraic_laws::127"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
        assert_eq!(f.len(), 1, "a compression is not a second arrival: {f:?}");
        assert!(f[0].contains("one arrival"), "{}", f[0]);
    }

    /// The control that keeps the arm above honest.
    ///
    /// Two consolidations of two different topics are two arrivals, so whatever
    /// closes the arm above must leave this silent. Without it the cheapest fix
    /// passes the arm and refuses the strongest tier the panel produces.
    #[test]
    fn control_two_consolidations_of_different_topics_stay_two_arrivals() {
        let p = cited(&[
            &format!("{ROOT}::63_spj_consolidation_the_format_concept::120"),
            &format!("{ROOT}::106_giesen_consolidation_the_strategy_axis::360"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "cross_topic"),
            ("provenance", &p),
        ])]);
        assert!(
            f.is_empty(),
            "two topics arriving separately is what cross_topic names: {f:?}"
        );
    }

    /// A member of one topic and the consolidation of another did not cite each
    /// other, so they are two arrivals.
    #[test]
    fn control_a_member_and_the_consolidation_of_another_topic_stay_two_arrivals() {
        let p = cited(&[
            &format!("{ROOT}::79_fallin_attacking_the_two_cold_derivations_on_derived_laws::27"),
            &format!("{ROOT}::106_giesen_consolidation_the_strategy_axis::360"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "cross_topic"),
            ("provenance", &p),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// The first pair the numbering-only reading refused. A check of a
    /// consolidation is numbered after it, so the numbering filed it under the
    /// next topic's consolidation and called the pair one arrival. No consolidation
    /// names the check, so it is a second reading.
    #[test]
    fn control_a_later_check_and_the_consolidation_of_the_next_topic_are_two_arrivals() {
        let p = cited(&[
            &format!("{ROOT}::91_ringer_entailment_check_on_the_derived_laws_consolidation::40"),
            &format!("{ROOT}::106_giesen_consolidation_the_strategy_axis::360"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
        assert!(
            f.is_empty(),
            "a check no consolidation names is a second arrival: {f:?}"
        );
    }

    /// The second pair: a second read numbered before a consolidation that does
    /// not name it. `215` is absent from what `244` says it compresses.
    #[test]
    fn control_a_second_read_the_consolidation_does_not_name_is_a_second_arrival() {
        let p = cited(&[
            &format!("{ROOT}::215_kiselyov_second_read_the_algebraic_laws::442"),
            &format!("{ROOT}::244_orchard_consolidation_admission_and_the_number_system::300"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// The arm that keeps the two above honest: a member the consolidation does
    /// name is absorbed, in a unit the numbering alone would also have found and
    /// in one it would not (`98` closes on `106` by numbering, and `106` names it).
    #[test]
    fn a_member_the_consolidation_names_is_absorbed() {
        let p = cited(&[
            &format!("{ROOT}::98_second_read::12"),
            &format!("{ROOT}::106_giesen_consolidation_the_strategy_axis::360"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// A consolidation reads another as input without compressing it, so a pair
    /// of two consolidations stays two arrivals even where one names the other.
    #[test]
    fn a_consolidation_naming_another_does_not_absorb_it() {
        let p = cited(&[
            &format!("{ROOT}::63_spj_consolidation_the_format_concept::120"),
            &format!("{ROOT}::90_giesen_consolidation_derived_algebraic_laws::127"),
        ]);
        let f = findings_over(&SITTING, &[("proposal::a_claim", &[
            ("standing", "cross_topic"),
            ("provenance", &p),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Without the panel on disk nothing is known to compress anything, so the
    /// pair is two files and the lint stays silent rather than guessing.
    #[test]
    fn with_no_panel_on_disk_a_pair_is_two_arrivals() {
        let p = cited(&[
            &format!("{ROOT}::79_fallin_attacking_the_two_cold_derivations_on_derived_laws::27"),
            &format!("{ROOT}::90_giesen_consolidation_derived_algebraic_laws::127"),
        ]);
        let f = findings(&[("proposal::a_claim", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_one_expert_row_citing_one_file_is_silent() {
        let p = cited(&[&format!("{ROOT}::67_kiselyov_which_prefix_earns_the_word::613")]);
        let f = findings(&[("proposal::a_claim", &[
            ("standing", "one_expert"),
            ("provenance", &p),
        ])]);
        assert!(f.is_empty(), "the ordinary honest row: {f:?}");
    }

    /// `contested` says somebody disagreed rather than that several arrived, so
    /// it is not inflated by resting on one file and must not be reported.
    #[test]
    fn control_a_contested_row_citing_one_file_is_silent() {
        let p = cited(&[&format!("{ROOT}::67_kiselyov_which_prefix_earns_the_word::613")]);
        let f = findings(&[("proposal::a_claim", &[
            ("standing", "contested"),
            ("provenance", &p),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Every one of the three multi-arrival standings is read, rather than the
    /// first of them. A list written and then half-consulted reads exactly like
    /// a complete one.
    #[test]
    fn all_three_multi_arrival_standings_are_read() {
        let p = cited(&[&format!("{ROOT}::74_giesen_consolidation::614")]);
        for standing in super::MULTI_ARRIVAL {
            let f = findings(&[("proposal::a_claim", &[
                ("standing", standing),
                ("provenance", &p),
            ])]);
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
        let f = findings(&[("ruling::a_call", &[
            ("standing", "two_experts"),
            ("provenance", &p),
        ])]);
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
            &[("proposal::a", &[
                ("standing", "two_experts"),
                ("provenance", "panel::one_file"),
            ])],
            &[],
        );
        assert_findings_block(
            &super::AStandingIsReachableFromWhatItCites {
                ceiling: 0,
            },
            &v,
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AStandingIsReachableFromWhatItCites {
            ceiling: super::CEILING,
        });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AStandingIsReachableFromWhatItCites {
                ceiling: 0,
            }
            .name(),
            "a-standing-is-reachable-from-what-it-cites"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-standing-is-reachable-from-what-it-cites");
    }
}
