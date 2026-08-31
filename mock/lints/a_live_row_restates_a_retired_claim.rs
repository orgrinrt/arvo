//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A live claim restating something the corpus retired.
//!
//! Every retirement holds the struck-out sentence in the words a later reader
//! would search for, which is what makes this checkable at all. **Nothing was
//! checking it.** The seat that wired the first answering edges named the gap
//! itself: it had not read the retirements, so a claim it wired to a question
//! could be one somebody had already struck out, and nothing in its process
//! would have caught that.
//!
//! The failure is worse than a stale row. A retired claim wired to a question
//! reports that question **settled**, by a sentence the corpus has said must
//! not be cited, and the reader who follows the edge meets an answer where a
//! retirement should have stopped them.
//!
//! **A lint rather than a tool.** A hit is one of exactly two states, both
//! wrong and both repairable in the rows: if the live row is right the
//! retirement is wrong and says so, and if the retirement is right the live row
//! answers nothing. There is no third reading and no ranking.
//!
//! **Deliberately high precision and low recall.** It matches a long verbatim
//! run rather than a paraphrase, so it finds a claim carried over wholesale and
//! misses one reworded. A fuzzy version would report a shared subject as a
//! restatement, and a check nobody believes is a check nobody runs.
//!
//! **What the port loses.** The crate this came from asserted the committed
//! canon carries none of these. A unit test cannot build a `RegistryView` from
//! the real registry, because that needs a TOML parser the generated pack may
//! not depend on, so that arm is `cargo mock --lint-only` over the real rows,
//! blocking, and the sweep across every retirement times every live row runs
//! there rather than here.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, normalise, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ALiveRowRestatesARetiredClaim)
}

const LINT: &str = "a-live-row-restates-a-retired-claim";

/// The namespace holding the struck-out sentences.
const RETIREMENTS: &str = "retirement";

/// The namespaces a live claim is written in.
///
/// A `ruling` is read as well as a `proposal` because op restating a retired
/// claim is the same defect and a worse one: it would carry his authority.
const LIVE: [&str; 2] = ["proposal", "ruling"];

/// How many consecutive words make a match distinctive rather than a
/// coincidence of vocabulary.
///
/// Eight is long enough that two authors do not write it twice by accident on
/// this subject, and short enough to survive a claim quoted with its edges
/// trimmed.
const RUN: usize = 8;

/// Below this a claim identifies nothing, so no live row is reported against it
/// and the retirement itself is the finding.
///
/// Five, because the schema asks a `claim` to be the sentence a reader would
/// search for or quote, and a phrase of four words or fewer is a subject rather
/// than a sentence. `a-retirement-claim-can-be-found` reports the row.
const FLOOR: usize = 5;

struct ALiveRowRestatesARetiredClaim;
impl Lint for ALiveRowRestatesARetiredClaim {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ALiveRowRestatesARetiredClaim {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry;
        let retired: Vec<(&str, &str, Vec<String>)> = reg
            .rows_in(RETIREMENTS)
            .iter()
            .filter_map(|q| {
                let claim = text(reg, q, "claim")?;
                Some((q.as_str(), claim, normalise(claim)))
            })
            .filter(|(_, _, words)| words.len() >= FLOOR)
            .collect();

        let mut out = Vec::new();
        for namespace in LIVE {
            for q in reg.rows_in(namespace) {
                let Some(says) = text(reg, q, "says") else {
                    continue;
                };
                let haystack = normalise(says).join(" ");
                for (at, claim, words) in &retired {
                    if carries(&haystack, words) {
                        out.push(hit(q, at, claim));
                    }
                }
            }
        }
        out
    }
}

/// Whether the live text carries a distinctive run of the retired claim.
///
/// A claim shorter than a distinctive run has none, so the whole of it has to
/// appear or nothing is reported. Anything looser on a five-word sentence
/// matches the subject rather than the claim.
fn carries(haystack: &str, words: &[String]) -> bool {
    if words.len() < RUN {
        return haystack.contains(&words.join(" "));
    }
    words.windows(RUN).any(|w| haystack.contains(&w.join(" ")))
}

fn hit(q: &str, at: &str, claim: &str) -> LintError {
    let shown: String = claim.chars().take(120).collect();
    finding(
        LINT,
        None,
        format!(
            "`{q}` carries in `says` a run of `{at}`, whose whole purpose is that the sentence \
             is not cited again: \"{shown}\". If the row is right the retirement is wrong and \
             says so; if the retirement is right this row answers nothing."
        ),
    )
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ALiveRowRestatesARetiredClaim, &view(rows, &[]))
    }

    #[test]
    fn a_verbatim_carry_fires_and_a_shared_subject_does_not() {
        // Both directions, because an arm reporting every row that mentions a
        // retired subject would be reporting the subject and would be switched
        // off within a day.
        let f = found(&[
            (
                "retirement::the_figure_that_matched_no_sweep",
                &[(
                    "claim",
                    "the sweep separates twenty one thousand two hundred and four of thirty two \
                 thousand seven hundred and sixty eight cells",
                )],
            ),
            (
                "proposal::carries_it_wholesale",
                &[(
                    "says",
                    "As established, the sweep separates twenty one thousand two hundred and four \
                 of thirty two thousand seven hundred and sixty eight cells, so the split is \
                 real.",
                )],
            ),
            (
                "proposal::about_the_same_subject_in_its_own_words",
                &[(
                    "says",
                    "The separating cells were counted directly and the count is far smaller than \
                 the one reported.",
                )],
            ),
            (
                "ruling::an_unrelated_call",
                &[("says", "The strategy set is not closed at exactly four.")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("carries_it_wholesale"), "{}", f[0]);
        assert!(
            f[0].contains("the_figure_that_matched_no_sweep"),
            "the report names which retirement, so a reader can decide which of the two is \
             wrong: {}",
            f[0]
        );
    }

    #[test]
    fn a_ruling_is_read_as_well_as_a_proposal() {
        let f = found(&[
            (
                "retirement::a_struck_sentence",
                &[(
                    "claim",
                    "the container premise is localised to a single clause of the candidate",
                )],
            ),
            (
                "ruling::a_call_carrying_it",
                &[(
                    "says",
                    "Given that the container premise is localised to a single clause of the \
                 candidate, proceed.",
                )],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ruling::a_call_carrying_it"), "{}", f[0]);
    }

    #[test]
    fn a_short_claim_matches_only_in_full() {
        // Below the distinctive-run length there is no run to be distinctive,
        // so anything looser reports a shared subject. The boundary is a test
        // rather than something left to whoever next reads the constant.
        let f = found(&[
            (
                "retirement::a_short_one",
                &[("claim", "the strategies are partially ordered")],
            ),
            (
                "proposal::carries_all_of_it",
                &[(
                    "says",
                    "It follows that the strategies are partially ordered by how many chain laws \
                 each honours.",
                )],
            ),
            (
                "proposal::carries_part_of_it",
                &[(
                    "says",
                    "The strategies are compared on how many chain laws each honours.",
                )],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("carries_all_of_it"), "{}", f[0]);
    }

    #[test]
    fn a_carry_survives_a_difference_in_punctuation_and_case() {
        // A quotation lifted into prose picks up neither reliably.
        let f = found(&[
            (
                "retirement::a_struck_sentence",
                &[(
                    "claim",
                    "A law verdict is established at the widths the instrument reached and no \
                 further",
                )],
            ),
            (
                "proposal::carries_it_reformatted",
                &[(
                    "says",
                    "**a law verdict is established at the widths the instrument reached, and no \
                 further.**",
                )],
            ),
        ]);
        assert_eq!(
            f.len(),
            1,
            "a carry that gained a comma and lost its capital is the same carry: {f:?}"
        );
    }

    #[test]
    fn a_claim_below_the_floor_reports_no_live_row() {
        // The case that had to fail and did: before the floor, the whole
        // two-word claim was the match, so a live row using the ordinary phrase
        // was reported as restating a retired one. The reported phrase is the
        // **restated** form the same retirement endorses.
        let f = found(&[
            ("retirement::an_abbreviation", &[("claim", "No repair.")]),
            ("proposal::a_row_using_the_ordinary_phrase", &[(
                "says",
                "A missed merge costs nothing at a monomorphic site, one threaded parameter at \
                 a polymorphic signature, and no repair at a homogeneous container.",
            )]),
        ]);
        assert!(
            f.is_empty(),
            "a two-word claim matches the subject rather than the claim: {f:?}"
        );
    }

    #[test]
    fn a_claim_exactly_at_the_floor_still_reports_a_full_carry() {
        // The other side of the boundary, which the crate this came from never
        // planted. A floor written as `>` rather than `>=` would silence a real
        // five-word retirement and every arm above would still pass.
        let f = found(&[
            (
                "retirement::exactly_five_words",
                &[("claim", "The strategy set is closed.")],
            ),
            (
                "proposal::carries_it",
                &[(
                    "says",
                    "It was assumed that the strategy set is closed, which op reopened.",
                )],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn a_genuine_restatement_of_a_full_sentence_is_still_reported_through_a_reworded_tail() {
        let f = found(&[
            ("retirement::a_sentence", &[(
                "claim",
                "A component is an output of the derivation when the consumer did not write it, \
                 the machine needs it, and a downstream site that holds the other components \
                 cannot recover it.",
            )]),
            ("proposal::a_row_restating_it", &[(
                "says",
                "A component is an output of the derivation when the consumer did not write it, \
                 the machine needs it, and a downstream site holding the other components \
                 cannot recover it.",
            )]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_row_restating_it"), "{}", f[0]);
    }

    #[test]
    fn a_retirement_with_no_claim_at_all_reports_nothing() {
        let f = found(&[
            ("retirement::r", &[("why", "it was wrong")]),
            (
                "proposal::p",
                &[("says", "anything at all, at some length, here")],
            ),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_row_with_no_says_is_not_read() {
        let f = found(&[
            (
                "retirement::r",
                &[("claim", "the strategies are partially ordered by law count")],
            ),
            (
                "proposal::p",
                &[(
                    "because",
                    "the strategies are partially ordered by law count",
                )],
            ),
        ]);
        assert!(
            f.is_empty(),
            "only `says` carries the claim; reading every field would report a row quoting a \
             retirement in order to argue against it: {f:?}"
        );
    }

    #[test]
    fn one_row_carrying_two_retired_claims_is_reported_against_each() {
        // The inner loop runs the whole retirement list. One that stopped at
        // the first hit would report half of a row carrying two, and every arm
        // above plants one retirement so none of them can tell.
        let f = found(&[
            (
                "retirement::a",
                &[("claim", "the strategies are partially ordered")],
            ),
            (
                "retirement::b",
                &[("claim", "the container premise is one clause")],
            ),
            (
                "proposal::p",
                &[(
                    "says",
                    "the strategies are partially ordered and the container premise is one clause",
                )],
            ),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().any(|m| m.contains("retirement::a")), "{f:?}");
        assert!(f.iter().any(|m| m.contains("retirement::b")), "{f:?}");
    }

    #[test]
    fn a_long_claim_reported_in_a_message_is_cut_rather_than_dumped_whole() {
        // The message carries the first 120 characters of the claim, so a
        // report over a corpus of long retirements stays readable.
        let long = "a".repeat(40) + " " + &["word"; 60].join(" ");
        let f = found(&[
            ("retirement::r", &[("claim", &long)]),
            ("proposal::p", &[("says", &long)]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(
            !f[0].contains(&long),
            "the whole claim was pasted into the report: {}",
            f[0]
        );
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
                    "retirement::r",
                    &[("claim", "the strategies are partially ordered")],
                ),
                (
                    "proposal::p",
                    &[("says", "and the strategies are partially ordered, as above")],
                ),
            ],
            &[],
        );
        assert_findings_block(&super::ALiveRowRestatesARetiredClaim, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ALiveRowRestatesARetiredClaim);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ALiveRowRestatesARetiredClaim.name(),
            "a-live-row-restates-a-retired-claim"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-live-row-restates-a-retired-claim");
    }
}
