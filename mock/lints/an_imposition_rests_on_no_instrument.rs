//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Nothing is imposed and established at once.
//!
//! The registry's own header states the test: a claim that could be measured
//! false is not `normative` however definitional its grammar, and it carries
//! the region it was established in or it is not here at all. An `evidence`
//! entry names a probe, a probe is an instrument, and an instrument that could
//! have returned the other answer is what `measured` means. So the two fields
//! contradict each other.
//!
//! The contradiction matters because `normative` is one of the two kinds that
//! carry no region. Filing a measured claim as an imposition silently widens it
//! from the model width it was established at to everywhere, and it does that
//! without touching the predicate, which is where the widening would have shown
//! up.
//!
//! **A lint rather than a tool, and a hard zero rather than a backlog.** This
//! is not a population to work down: it is a contradiction between two fields,
//! and a row carrying it is wrong at the moment it is written. Both repairs are
//! one edit. Either the claim is measured and owes the region its instrument
//! ran at, or it is imposed and owes no evidence.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon carries no such row. A unit test cannot build a
//! `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AnImpositionRestsOnNoInstrument)
}

const LINT: &str = "an-imposition-rests-on-no-instrument";

/// The two `sentence_kind` values that carry no region, and therefore owe no
/// instrument.
///
/// The others say the claim was established, which is what `evidence` is for.
const IMPOSED: [&str; 2] = ["normative", "definition"];

struct AnImpositionRestsOnNoInstrument;
impl Lint for AnImpositionRestsOnNoInstrument {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AnImpositionRestsOnNoInstrument {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        ctx.registry
            .rows_in("proposal")
            .iter()
            .filter_map(|q| {
                let kind = text(ctx.registry, q, "sentence_kind")?;
                if !IMPOSED.contains(&kind) {
                    return None;
                }
                let evidence = list(ctx.registry, q, "evidence");
                if evidence.is_empty() {
                    return None;
                }
                Some(finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` sets `sentence_kind` to `{kind}`, which says the claim is \
                         imposed rather than established and is why it carries no region, \
                         and `evidence` names {evidence:?}. An instrument that could have \
                         returned the other answer is what `measured` means. Either the \
                         claim is measured and owes the region its instrument was run at, \
                         or it is imposed and owes no evidence."
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

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::AnImpositionRestsOnNoInstrument
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn a_normative_row_with_evidence_is_reported() {
        let f = findings(&[(
            "proposal::a_claim",
            &[
                ("sentence_kind", "normative"),
                ("evidence", "an_instrument"),
            ],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("normative"), "{}", f[0]);
        assert!(f[0].contains("an_instrument"), "{}", f[0]);
    }

    /// A `definition` is the other region-free kind and is caught the same way.
    ///
    /// Written as its own arm rather than folded into the one above, because a
    /// predicate reading only the first entry of the list would pass that one
    /// and fail this.
    #[test]
    fn a_definition_with_evidence_is_reported() {
        let f = findings(&[(
            "proposal::a_claim",
            &[
                ("sentence_kind", "definition"),
                ("evidence", "an_instrument"),
            ],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("definition"), "{}", f[0]);
    }

    /// A measured row with evidence is the ordinary correct shape.
    ///
    /// A lint refusing this would be refusing the schema rather than checking
    /// it, and it would read the same on a clean canon.
    #[test]
    fn control_a_measured_row_with_evidence_is_silent() {
        let f = findings(&[(
            "proposal::a_claim",
            &[("sentence_kind", "measured"), ("evidence", "an_instrument")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// A normative row without evidence is the other correct shape.
    #[test]
    fn control_a_normative_row_without_evidence_is_silent() {
        let f = findings(&[("proposal::a_claim", &[("sentence_kind", "normative")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// An `evidence` field written and left empty is not evidence.
    ///
    /// The engine writes an empty array as the empty string, so a presence test
    /// would report a row that names no instrument at all. That row is a
    /// different defect and not this one.
    #[test]
    fn control_an_empty_evidence_list_is_not_an_instrument() {
        let f = findings(&[(
            "proposal::a_claim",
            &[("sentence_kind", "normative"), ("evidence", "")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Every entry is named in the report rather than the first.
    ///
    /// A reader repairing the row has to see all of them, since the choice
    /// between the two repairs depends on what the instruments actually are.
    #[test]
    fn every_named_instrument_reaches_the_report() {
        let f = findings(&[(
            "proposal::a_claim",
            &[
                ("sentence_kind", "normative"),
                ("evidence", "first_probe, second_probe"),
            ],
        )]);
        assert_eq!(f.len(), 1, "one row is one finding: {f:?}");
        assert!(f[0].contains("first_probe"), "{}", f[0]);
        assert!(f[0].contains("second_probe"), "{}", f[0]);
    }

    /// Only `proposal` is read. `law` carries `evidence` too, and it carries no
    /// `sentence_kind`, so a walk over every row would ask a question the law
    /// namespace never answers.
    #[test]
    fn control_a_law_is_not_read_as_a_proposal() {
        let f = findings(&[(
            "law::associativity",
            &[
                ("sentence_kind", "normative"),
                ("evidence", "an_instrument"),
            ],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_row_with_no_sentence_kind_owes_nothing() {
        let f = findings(&[("proposal::a_claim", &[("evidence", "an_instrument")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[(
                "proposal::a",
                &[("sentence_kind", "normative"), ("evidence", "a_probe")],
            )],
            &[],
        );
        assert_findings_block(&super::AnImpositionRestsOnNoInstrument, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AnImpositionRestsOnNoInstrument);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AnImpositionRestsOnNoInstrument.name(),
            "an-imposition-rests-on-no-instrument"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("an-imposition-rests-on-no-instrument");
    }
}
