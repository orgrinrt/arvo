//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A measurement resting on an instrument nobody should quote.
//!
//! **The gate beside this one was accidentally strong while the `probe`
//! namespace was empty**: a `measured` row could name no probe and was
//! reported, and it could name no usable one either, because there were none of
//! any kind. The seat that filled the namespace predicted, before its own run,
//! that finishing the job would weaken that check, and it was right. Naming a
//! probe is now enough to pass it, and a probe can be defective, withdrawn, or
//! one whose own control admits nothing was run.
//!
//! So citing is not the bar. **What a measurement owes is an instrument whose
//! figures may be used**, and the three states reported here are the three ways
//! one cannot be:
//!
//! - `defective`, where a known defect means the numbers are wrong.
//! - `withdrawn`, where the author retracted it.
//! - `uncontrolled`, where no case that had to fail was run, which is the
//!   quietest of the three and the one this corpus is full of: a probe that
//!   cannot come out any other way produces a number and is not an instrument.
//!
//! **Two finding kinds, because the two repairs differ.** A defective or
//! withdrawn instrument wants a different citation; an uncontrolled one wants
//! the control built, or the sentence remarked as the argument it now is. A
//! severity can be set per kind, which one bucket would not allow.
//!
//! **`standing` is data and no sentence is parsed here.** The crate this came
//! from carried three versions of a prose matcher over the `control` field and
//! deleted all three: a word list caught one admission in five, then six in
//! nine, and failed in both directions for reasons no longer list fixes, since
//! it cannot tell a report from a counterfactual and cannot see a negation
//! sitting on a different noun. Every probe row now carries a standing somebody
//! set by reading its field in full.
//!
//! **A cited slug naming no probe row is not this lint's report.** The engine
//! resolves typed references and refuses a dangling one, so reporting it here
//! would be a second opinion on a question already answered, in different
//! words.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AMeasurementOwesAUsableInstrument)
}

const LINT: &str = "a-measurement-owes-a-usable-instrument";

/// The namespace whose rows carry the claim, and the one holding the
/// instruments they cite.
const CLAIMS: &str = "proposal";
const INSTRUMENTS: &str = "probe";

/// The sentence kinds that ran an instrument, and therefore rest on one.
///
/// The same pair `measured-claim-cites-no-probe` reads, for the same reason: a
/// sweep is a measurement whatever its author called the sentence. An
/// `argument` citing a defective probe is not a measurement resting on one, so
/// it is not read here.
const RAN_SOMETHING: [&str; 2] = ["measured", "enumeration"];

/// A standing whose figures are not to be used, and the kind each reports under.
const UNUSABLE: [(&str, &str); 3] = [
    ("defective", "measurement-rests-on-an-unusable-instrument"),
    ("withdrawn", "measurement-rests-on-an-unusable-instrument"),
    (
        "uncontrolled",
        "measurement-rests-on-an-uncontrolled-instrument",
    ),
];

struct AMeasurementOwesAUsableInstrument;
impl Lint for AMeasurementOwesAUsableInstrument {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AMeasurementOwesAUsableInstrument {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for q in ctx.registry.rows_in(CLAIMS) {
            let Some(kind) = text(ctx.registry, q, "sentence_kind") else {
                continue;
            };
            if !RAN_SOMETHING.contains(&kind) {
                continue;
            }
            for cited in list(ctx.registry, q, "evidence") {
                let probe = format!("{INSTRUMENTS}::{cited}");
                if ctx.registry.row(&probe).is_none() {
                    continue; // a slug naming no row is the engine's report
                }
                let standing = text(ctx.registry, &probe, "standing").unwrap_or("");
                if let Some(e) = report(q, cited, standing) {
                    out.push(e);
                }
            }
        }
        out
    }
}

/// One cited instrument, against the three standings that disqualify it.
fn report(q: &str, cited: &str, standing: &str) -> Option<LintError> {
    let kind = UNUSABLE
        .iter()
        .find(|(value, _)| *value == standing)
        .map(|(_, kind)| *kind)?;
    let says = if standing == "uncontrolled" {
        format!(
            "`{q}` names `probe::{cited}` in `evidence`, whose own control says no case that \
             had to fail was run. An instrument that cannot come out any other way produces a \
             number and is not an instrument, so what this row has is a figure rather than a \
             measurement."
        )
    } else {
        format!(
            "`{q}` names `probe::{cited}` in `evidence`, whose `standing` is `{standing}`. Its \
             figures are not to be used, so a claim resting on it is not a measurement. Cite a \
             sound instrument, or mark the sentence as the argument it now is."
        )
    };
    Some(finding(LINT, Some(kind), says))
}
#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, findings, view,
    };

    /// The findings with the kind each carries, so an arm can assert which of
    /// the two refusals fired rather than only that something did.
    fn kinds(rows: &[(&str, &[(&str, &str)])]) -> Vec<(Option<&'static str>, String)> {
        let v = view(rows, &[]);
        super::AMeasurementOwesAUsableInstrument
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| (e.finding_kind, e.message))
            .collect()
    }

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::AMeasurementOwesAUsableInstrument, &view(rows, &[]))
    }

    /// The four instruments a corpus holds, planted once and used by the arms
    /// that need them.
    const PROBES: [(&str, &[(&str, &str)]); 4] = [
        (
            "probe::the_sound_one",
            &[
                ("standing", "sound"),
                (
                    "control",
                    "the wrap arm had to disagree with clamp at width 5, and it fired",
                ),
            ],
        ),
        (
            "probe::the_defective_one",
            &[
                ("standing", "defective"),
                (
                    "control",
                    "the arm compared two placements of sixteen; no control caught it",
                ),
            ],
        ),
        (
            "probe::the_withdrawn_one",
            &[
                ("standing", "withdrawn"),
                ("control", "the author retracted it before reporting"),
            ],
        ),
        (
            "probe::the_uncontrolled_one",
            &[
                ("standing", "uncontrolled"),
                ("control", "no control was run"),
            ],
        ),
    ];

    fn with_probes(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let mut all: Vec<(&str, &[(&str, &str)])> = PROBES.to_vec();
        all.extend_from_slice(rows);
        findings(&super::AMeasurementOwesAUsableInstrument, &view(&all, &[]))
    }

    #[test]
    fn a_measurement_may_not_rest_on_a_defective_withdrawn_or_uncontrolled_probe() {
        // The sound one must pass, and an argument citing a defective probe is
        // not a measurement resting on one. Without the last row an arm keyed
        // on the edge alone would pass every other assertion here.
        let f = with_probes(&[
            (
                "proposal::rests_on_the_sound_one",
                &[("sentence_kind", "measured"), ("evidence", "the_sound_one")],
            ),
            (
                "proposal::rests_on_the_defective_one",
                &[
                    ("sentence_kind", "measured"),
                    ("evidence", "the_defective_one"),
                ],
            ),
            (
                "proposal::rests_on_the_withdrawn_one",
                &[
                    ("sentence_kind", "enumeration"),
                    ("evidence", "the_withdrawn_one"),
                ],
            ),
            (
                "proposal::rests_on_the_uncontrolled_one",
                &[
                    ("sentence_kind", "measured"),
                    ("evidence", "the_uncontrolled_one"),
                ],
            ),
            (
                "proposal::an_argument_citing_a_defective_one",
                &[
                    ("sentence_kind", "argument"),
                    ("evidence", "the_defective_one"),
                ],
            ),
        ]);
        assert_eq!(f.len(), 3, "{f:?}");
        for who in [
            "rests_on_the_defective_one",
            "rests_on_the_withdrawn_one",
            "rests_on_the_uncontrolled_one",
        ] {
            assert!(
                f.iter().any(|m| m.contains(who)),
                "{who} was not reported: {f:?}"
            );
        }
        assert!(
            !f.iter().any(|m| m.contains("rests_on_the_sound_one")),
            "{f:?}"
        );
        assert!(
            !f.iter().any(|m| m.contains("an_argument_citing")),
            "an argument citing a defective probe is not a measurement resting on one: {f:?}"
        );
    }

    #[test]
    fn the_uncontrolled_case_is_reported_under_its_own_kind() {
        // The fix differs, so the bucket does. A severity set on one of these
        // must not reach the other.
        let f = kinds(&[
            ("probe::defective", &[("standing", "defective")]),
            ("probe::uncontrolled", &[("standing", "uncontrolled")]),
            (
                "proposal::a",
                &[("sentence_kind", "measured"), ("evidence", "defective")],
            ),
            (
                "proposal::b",
                &[("sentence_kind", "measured"), ("evidence", "uncontrolled")],
            ),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(
            f.iter().any(
                |(k, m)| *k == Some("measurement-rests-on-an-uncontrolled-instrument")
                    && m.contains("proposal::b")
            ),
            "{f:?}"
        );
        assert!(
            f.iter().any(
                |(k, m)| *k == Some("measurement-rests-on-an-unusable-instrument")
                    && m.contains("proposal::a")
            ),
            "{f:?}"
        );
    }

    #[test]
    fn the_standing_decides_and_the_control_prose_does_not() {
        // Two probes carrying the identical control text and differing only in
        // `standing`, which is the whole point: three prose matchers over that
        // field were built and deleted, and this arm fails if one comes back.
        const SAME: &str = "None was run as a case that had to fail. Had the arms disagreed it \
                            would have been reported, and they did not.";
        let f = found(&[
            (
                "probe::declared_uncontrolled",
                &[("standing", "uncontrolled"), ("control", SAME)],
            ),
            (
                "probe::declared_sound",
                &[("standing", "sound"), ("control", SAME)],
            ),
            (
                "proposal::rests_on_the_declared_one",
                &[
                    ("sentence_kind", "measured"),
                    ("evidence", "declared_uncontrolled"),
                ],
            ),
            (
                "proposal::rests_on_the_undeclared_one",
                &[
                    ("sentence_kind", "measured"),
                    ("evidence", "declared_sound"),
                ],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("rests_on_the_declared_one"), "{}", f[0]);
    }

    #[test]
    fn a_slug_naming_no_probe_row_is_the_engines_report_rather_than_this_one() {
        // The engine resolves typed references and refuses a dangling one. A
        // second opinion here in different words would be two reports of one
        // defect, and a lint that reported an absent row as unusable would fire
        // on every corpus mid-edit.
        let f = found(&[(
            "proposal::p",
            &[
                ("sentence_kind", "measured"),
                ("evidence", "nothing_declares_this"),
            ],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_probe_carrying_no_standing_at_all_is_not_reported() {
        // Absent is not one of the three disqualifying values. The schema owns
        // whether the field is required, and inventing a fourth state here
        // would report a row for a defect a different check names.
        let f = found(&[
            ("probe::p", &[("control", "something was run")]),
            (
                "proposal::q",
                &[("sentence_kind", "measured"), ("evidence", "p")],
            ),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_cited_instrument_is_read_rather_than_the_first() {
        // The inner loop is over the whole `evidence` list, and one that
        // stopped at the first entry passes every arm above, since each plants
        // one edge per row.
        let f = found(&[
            ("probe::a", &[("standing", "defective")]),
            ("probe::b", &[("standing", "withdrawn")]),
            (
                "proposal::p",
                &[("sentence_kind", "measured"), ("evidence", "a, b")],
            ),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().any(|m| m.contains("probe::a")), "{f:?}");
        assert!(f.iter().any(|m| m.contains("probe::b")), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[
                ("probe::p", &[("standing", "defective")]),
                (
                    "proposal::q",
                    &[("sentence_kind", "measured"), ("evidence", "p")],
                ),
            ],
            &[],
        );
        assert_findings_block(&super::AMeasurementOwesAUsableInstrument, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AMeasurementOwesAUsableInstrument);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AMeasurementOwesAUsableInstrument.name(),
            "a-measurement-owes-a-usable-instrument"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-measurement-owes-a-usable-instrument");
    }
}
