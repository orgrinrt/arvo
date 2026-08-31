//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A retirement whose `claim` is too short to find.
//!
//! The schema asks a `claim` to be the sentence itself, or close enough that a
//! search finds it, and a claim of a few words is neither. **It cannot be
//! enforced and it cannot fail honestly**: too short to match the sentence it
//! retires, and long enough to match ordinary prose that happens to share the
//! vocabulary.
//!
//! Both halves were observed on one row. `No repair.` is how its source
//! abbreviates a claim, and against it the restatement check reported a live
//! row saying "no repair at a homogeneous container", which is the **restated**
//! form that same retirement endorses. So the retirement pinned nothing it
//! meant to pin and reported the one thing it meant to protect.
//!
//! **A lint rather than a tool.** The repair is to write the sentence, which is
//! usually recoverable from the provenance the row already carries, and there
//! is no reading under which an unfindable claim is fine.
//!
//! **The floor is five words and it is shared with
//! `a-live-row-restates-a-retired-claim`**, which refuses to report against a
//! claim below it. Between the two, a short claim produces exactly one finding
//! and it lands on the retirement, which is where the repair is.
//!
//! **What the port loses.** The crate this came from asserted every retirement
//! in the committed canon carries a findable claim. A unit test cannot build a
//! `RegistryView` from the real registry, because that needs a TOML parser the
//! generated pack may not depend on, so that sweep is `cargo mock --lint-only`
//! over the real rows now, blocking.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, normalise, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ARetirementClaimCanBeFound)
}

const LINT: &str = "a-retirement-claim-can-be-found";

/// The namespace holding the struck-out sentences.
const NAMESPACE: &str = "retirement";

/// The fewest words a claim can carry and still name a sentence rather than a
/// subject.
///
/// Five, because the schema asks a `claim` to be the sentence a later reader
/// would quote, and a phrase of four words or fewer is a subject.
const FLOOR: usize = 5;

struct ARetirementClaimCanBeFound;
impl Lint for ARetirementClaimCanBeFound {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ARetirementClaimCanBeFound {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        ctx.registry
            .rows_in(NAMESPACE)
            .iter()
            .filter_map(|q| {
                // `claim` is required, so a row carrying none is the schema's
                // report and not this one.
                let claim = text(ctx.registry, q, "claim")?;
                let words = normalise(claim).len();
                if words >= FLOOR {
                    return None;
                }
                Some(finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` holds a `claim` of {words} word(s), \"{claim}\", which no search \
                         can distinguish from ordinary prose on the same subject. A claim is the \
                         sentence a later reader would quote, so nothing can be pinned to this \
                         one and a live row restating it goes unreported. Write the sentence; \
                         the provenance usually still holds it."
                    ),
                ))
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ARetirementClaimCanBeFound, &view(rows, &[]))
    }

    #[test]
    fn a_short_claim_is_reported_and_a_sentence_is_not() {
        let f = found(&[
            ("retirement::an_abbreviation", &[("claim", "No repair.")]),
            (
                "retirement::a_sentence",
                &[(
                    "claim",
                    "Two names for one primitive is a compile error with no in-language repair.",
                )],
            ),
        ]);
        assert_eq!(f.len(), 1, "only the abbreviation: {f:?}");
        assert!(f[0].contains("an_abbreviation"), "{}", f[0]);
        assert!(f[0].contains("2 word"), "the count is reported: {}", f[0]);
    }

    #[test]
    fn a_claim_at_the_floor_is_left_alone_and_one_below_it_is_not() {
        // The boundary both ways, because a floor written as `>` rather than
        // `>=` reports a findable five-word claim and every other arm here
        // still passes.
        let f = found(&[
            (
                "retirement::exactly_five_words",
                &[("claim", "The strategy set is closed.")],
            ),
            (
                "retirement::four_words",
                &[("claim", "The strategy set moved.")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("four_words"), "{}", f[0]);
    }

    #[test]
    fn punctuation_and_emphasis_are_not_counted_as_words() {
        // The count is over normalised words, so a claim padded with markup is
        // exactly as short as it reads. A split on whitespace alone would let
        // `**No** *repair.*` through at four.
        let f = found(&[("retirement::r", &[("claim", "**No** ,,, *repair.*")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("2 word"), "{}", f[0]);
    }

    #[test]
    fn a_row_with_no_claim_at_all_is_the_schemas_report_rather_than_this_one() {
        // `claim` is required. A second report here in different words would be
        // two findings for one defect, and the repairs differ: one is a missing
        // field and the other is a field holding too little.
        let f = found(&[("retirement::r", &[("why", "it was wrong")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_claim_present_and_blank_is_read_as_absent_rather_than_as_zero_words() {
        // A field holding nothing tells a reader exactly what a missing one
        // does, and the schema owns both. Reporting it here would put a
        // required-field finding under a lint about claim length.
        for blank in ["", " ", "\n"] {
            let f = found(&[("retirement::r", &[("claim", blank)])]);
            assert!(f.is_empty(), "{blank:?} was reported: {f:?}");
        }
    }

    #[test]
    fn a_proposal_is_not_read_whatever_it_carries() {
        // Only `retirement` declares `claim`. A lint reading every namespace
        // would report a row in some other namespace that happens to use the
        // word, and every arm above plants one namespace so none can tell.
        let f = found(&[("proposal::p", &[("claim", "No repair.")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_short_retirement_is_reported_rather_than_the_first() {
        let f = found(&[
            ("retirement::a", &[("claim", "No repair.")]),
            ("retirement::b", &[("claim", "Not a definition.")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("retirement::r", &[("claim", "No repair.")])], &[]);
        assert_findings_block(&super::ARetirementClaimCanBeFound, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ARetirementClaimCanBeFound);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ARetirementClaimCanBeFound.name(),
            "a-retirement-claim-can-be-found"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-retirement-claim-can-be-found");
    }
}
