//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A region on a sentence that states none inverts it, and its absence anywhere
//! else hides one.
//!
//! Two kinds state no region, for opposite reasons. A `normative` sentence
//! imposes rather than establishes, so a region on it says the design may
//! violate it everywhere the region does not reach, which is the opposite of
//! what it means. A `definition` stipulates what a term means: it is not a
//! claim about where anything holds, so a region on one is a category error
//! rather than a narrowing. Fifteen of the first seventeen rows written were
//! marked `normative` and about half were stipulations, which is what earned
//! the second value.
//!
//! Every other kind is established somewhere and nowhere else, and leaving the
//! region out claims the whole space. Under the predicate notation an absent
//! axis says the claim holds in no situation where that axis exists, so an
//! absent region entirely is a sentence with nothing to gate on at all.
//!
//! **A lint rather than a tool.** The two halves point opposite ways and both
//! are refusals of a state the row can be edited out of: drop the region, or
//! write the one it was established in, however narrow. `threads = 1` is a real
//! region and a real finding.
//!
//! **Two finding kinds**, because a severity set on one must not reach the
//! other and the repairs are opposites.
//!
//! **What the port loses.** The crate this came from also asserted the
//! committed canon agrees with itself here. A unit test cannot build a
//! `RegistryView` from the real registry, since that needs a TOML parser the
//! generated pack may not depend on, so that arm is `cargo mock --lint-only`
//! now, over the real rows, blocking.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, predicate_entries, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ARegionAgreesWithTheSentenceKind)
}

const LINT: &str = "a-region-agrees-with-the-sentence-kind";

/// The namespace declaring both `sentence_kind` and `predicate`.
const NAMESPACE: &str = "proposal";

/// The two kinds that state no region.
const REGIONLESS: [&str; 2] = ["normative", "definition"];

struct ARegionAgreesWithTheSentenceKind;
impl Lint for ARegionAgreesWithTheSentenceKind {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ARegionAgreesWithTheSentenceKind {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for q in ctx.registry.rows_in(NAMESPACE) {
            let kind = text(ctx.registry, q, "sentence_kind").unwrap_or("");
            let regionless = REGIONLESS.contains(&kind);
            let has_region = !predicate_entries(ctx.registry, q, "predicate").is_empty();
            if regionless && has_region {
                out.push(finding(
                    LINT,
                    Some("an-imposed-proposition-carries-a-region"),
                    format!(
                        "`{q}` has `sentence_kind` `{kind}` and carries a `predicate`. Neither \
                         kind states a region: an imposed proposition would be saying the \
                         design may violate it everywhere the region does not reach, and a \
                         definition is not a claim about where anything holds."
                    ),
                ));
            }
            if !regionless && !has_region {
                out.push(finding(
                    LINT,
                    Some("an-established-claim-carries-no-region"),
                    format!(
                        "`{q}` has `sentence_kind` `{kind}` and an empty `predicate`, so the \
                         claim reads as holding everywhere while having been established \
                         somewhere. Write the region it was established in, however narrow: \
                         `threads = 1` is a real region and a real finding."
                    ),
                ));
            }
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, findings, view,
    };
    use crate::canon_rows::JOIN;

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ARegionAgreesWithTheSentenceKind, &view(rows, &[]))
    }

    fn kinds(rows: &[(&str, &[(&str, &str)])]) -> Vec<Option<&'static str>> {
        let v = view(rows, &[]);
        super::ARegionAgreesWithTheSentenceKind
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.finding_kind)
            .collect()
    }

    #[test]
    fn an_imposed_proposition_carrying_a_region_is_reported() {
        let k = kinds(&[(
            "proposal::a_firewall",
            &[
                ("sentence_kind", "normative"),
                ("predicate", "fraction_width: 0"),
            ],
        )]);
        assert_eq!(
            k,
            [Some("an-imposed-proposition-carries-a-region")],
            "{k:?}"
        );
    }

    #[test]
    fn an_established_claim_with_no_region_is_reported() {
        let k = kinds(&[(
            "proposal::a_theorem_with_no_region",
            &[("sentence_kind", "theorem")],
        )]);
        assert_eq!(k, [Some("an-established-claim-carries-no-region")], "{k:?}");
    }

    #[test]
    fn a_definition_carries_no_region_and_neither_does_an_imposition() {
        // A definition states no region for a different reason than an
        // imposition does, and both must be silent while the definition
        // carrying one is not.
        let f = found(&[
            (
                "proposal::a_stipulation",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
            (
                "proposal::a_stipulation_with_a_region",
                &[
                    ("sentence_kind", "definition"),
                    ("defines", "stretch"),
                    ("predicate", "fraction_width: 0"),
                ],
            ),
            (
                "proposal::an_imposed_one",
                &[("sentence_kind", "normative")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_stipulation_with_a_region"), "{}", f[0]);
        assert!(
            f[0].contains("definition"),
            "the report names which of the two kinds it was: {}",
            f[0]
        );
    }

    #[test]
    fn the_two_correct_shapes_are_both_silent() {
        // The two halves point opposite ways, so the pair satisfying both must
        // be silent or the arm is reporting the rule rather than a breach of it.
        let region = ["fraction_width: 0", "threads: 1"].join(JOIN);
        let f = found(&[
            (
                "proposal::an_imposed_one",
                &[("sentence_kind", "normative")],
            ),
            (
                "proposal::an_established_one",
                &[("sentence_kind", "theorem"), ("predicate", &region)],
            ),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_predicate_present_and_holding_nothing_reads_as_no_region() {
        // The engine writes an empty array as the empty string, so a row that
        // carries the field and fills it with nothing arrives looking exactly
        // like one that never wrote it, and an established claim with either is
        // claiming the whole space.
        //
        // A lone separator is not in the list, and the omission is the point:
        // `list` reads `", "` as no entries and the predicate reader reads it
        // as one entry spelled `,`, because the reader hands a malformed entry
        // on whole so the arm that reports malformed entries can see it. The
        // engine writes an empty array as the empty string, so neither reading
        // describes a state a real run produces.
        for blank in ["", " ", "\t", "\n  "] {
            let f = found(&[(
                "proposal::p",
                &[("sentence_kind", "theorem"), ("predicate", blank)],
            )]);
            assert_eq!(f.len(), 1, "{blank:?} passed as a region: {f:?}");
        }
    }

    #[test]
    fn a_row_with_no_sentence_kind_at_all_is_read_as_establishing_something() {
        // Absent is not one of the two regionless kinds, so a row carrying no
        // kind and no region is reported. That is the finder's own reading,
        // carried over deliberately: the schema owns whether the field is
        // required, and a row missing both says nothing about where it holds.
        let f = found(&[("proposal::p", &[("says", "something")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        let k = kinds(&[("proposal::p", &[("says", "something")])]);
        assert_eq!(k, [Some("an-established-claim-carries-no-region")], "{k:?}");
    }

    #[test]
    fn a_predicate_whose_values_carry_the_separator_still_reads_as_a_region() {
        // The joined field is read through the predicate reader rather than a
        // plain split, so an entry carrying `", "` inside its values is one
        // entry. Emptiness is the only question this lint asks, so the two
        // readers agree here; the reader is used anyway, because the day this
        // arm asks anything finer a plain split would be wrong and silent.
        let f = found(&[(
            "proposal::p",
            &[
                ("sentence_kind", "theorem"),
                ("predicate", "signedness: signedness in {unsigned, signed}"),
            ],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_ruling_is_not_read_whatever_it_carries() {
        // Only `proposal` declares both fields. A lint reading every namespace
        // would report every `ruling` in the corpus as an established claim
        // with no region, and every arm above would still pass.
        let f = found(&[("ruling::r", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_offending_row_is_reported_rather_than_the_first() {
        let f = found(&[
            ("proposal::a", &[("sentence_kind", "theorem")]),
            (
                "proposal::b",
                &[("sentence_kind", "normative"), ("predicate", "threads: 1")],
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
        let v = view(&[("proposal::p", &[("sentence_kind", "theorem")])], &[]);
        assert_findings_block(&super::ARegionAgreesWithTheSentenceKind, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ARegionAgreesWithTheSentenceKind);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ARegionAgreesWithTheSentenceKind.name(),
            "a-region-agrees-with-the-sentence-kind"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-region-agrees-with-the-sentence-kind");
    }
}
