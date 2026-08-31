//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A ruling that stamps a proposal without itself being ratified.
//!
//! Ratification is what a stamp is. A row at any other tier claiming to promote
//! something is promoting it on nobody's authority, and the proposal it names
//! then reads as canon on the strength of an ack, which op's own correction
//! says an ack is not.
//!
//! **A lint rather than a tool.** There is no legitimate reading of the state:
//! either the ruling really is a ratification, in which case `rung` says so, or
//! it is not, in which case the `ratifies` edge should not be there. Both are
//! one edit to one row.
//!
//! **The check is the pair, never either half alone.** A ruling at a lower tier
//! is ordinary and most of the namespace is one; an edge from a ratified ruling
//! is what the field exists for. Only their conjunction is wrong, which is
//! exactly the shape a schema cannot state.
//!
//! **What the port loses.** The crate this came from asserted the committed
//! canon stamps nothing on an ack. A unit test here cannot build a
//! `RegistryView` from the real registry, because that needs a TOML parser the
//! generated pack may not depend on, so that assertion is now
//! `cargo mock --lint-only` over the real rows.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AnUnratifiedRulingStampsAProposal)
}

const LINT: &str = "an-unratified-ruling-stamps-a-proposal";

/// The namespace carrying `ratifies` and `rung`.
const NAMESPACE: &str = "ruling";

/// The one tier at which a stamp is what it claims to be.
const RATIFIED: &str = "ratified";

struct AnUnratifiedRulingStampsAProposal;
impl Lint for AnUnratifiedRulingStampsAProposal {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AnUnratifiedRulingStampsAProposal {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        ctx.registry
            .rows_in(NAMESPACE)
            .iter()
            .filter(|q| !list(ctx.registry, q, "ratifies").is_empty())
            .filter(|q| text(ctx.registry, q, "rung") != Some(RATIFIED))
            .map(|q| {
                let rung = text(ctx.registry, q, "rung").unwrap_or("(absent)");
                finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` sets `ratifies` and its `rung` is `{rung}`. A stamp is a \
                         ratification; at any other tier the proposal it names becomes canon on \
                         the strength of an ack, which op's own correction says an ack is not."
                    ),
                )
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
        findings(&super::AnUnratifiedRulingStampsAProposal, &view(rows, &[]))
    }

    #[test]
    fn a_stamp_from_anything_but_a_ratification_is_reported() {
        let f = found(&[
            (
                "ruling::an_ack_that_stamps",
                &[("rung", "stated"), ("ratifies", "some_claim")],
            ),
            (
                "ruling::a_real_ratification",
                &[("rung", "ratified"), ("ratifies", "some_other_claim")],
            ),
            ("ruling::an_ack_that_stamps_nothing", &[("rung", "stated")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("an_ack_that_stamps"), "{}", f[0]);
        assert!(
            f[0].contains("stated"),
            "the report names the tier, or a reader cannot tell which half to fix: {}",
            f[0]
        );
    }

    #[test]
    fn a_stamp_from_a_ruling_with_no_rung_at_all_is_reported_and_says_so() {
        // An absent tier is not the ratified one, and a reader has to be able
        // to tell the two apart, since the repairs differ: one is a wrong value
        // and the other is a missing one.
        let f = found(&[("ruling::r", &[("ratifies", "some_claim")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("(absent)"), "{}", f[0]);
    }

    #[test]
    fn a_rung_present_and_blank_is_not_the_ratified_one() {
        for blank in ["", " ", "\n"] {
            let f = found(&[("ruling::r", &[("rung", blank), ("ratifies", "some_claim")])]);
            assert_eq!(f.len(), 1, "{blank:?} passed as a ratification: {f:?}");
        }
    }

    #[test]
    fn control_a_ratifies_field_present_and_holding_nothing_stamps_nothing() {
        // A row that carries the edge and fills it with nothing has promoted
        // no proposal, so there is nothing standing on its authority. An arm
        // keyed on the field being present would report most of the namespace.
        for blank in ["", " ", ", "] {
            let f = found(&[("ruling::r", &[("rung", "stated"), ("ratifies", blank)])]);
            assert!(f.is_empty(), "{blank:?} was read as a stamp: {f:?}");
        }
    }

    #[test]
    fn a_proposal_is_not_read_whatever_it_carries() {
        // Only `ruling` carries `ratifies`. A lint reading every namespace
        // would be deciding a rule for namespaces it was not written for, and
        // every arm above plants one namespace so none of them can tell.
        let f = found(&[(
            "proposal::p",
            &[("rung", "stated"), ("ratifies", "some_claim")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_offending_ruling_is_reported_rather_than_the_first() {
        let f = found(&[
            ("ruling::a", &[("rung", "stated"), ("ratifies", "x")]),
            ("ruling::b", &[("rung", "acked"), ("ratifies", "y")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[("ruling::r", &[("rung", "stated"), ("ratifies", "x")])],
            &[],
        );
        assert_findings_block(&super::AnUnratifiedRulingStampsAProposal, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AnUnratifiedRulingStampsAProposal);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AnUnratifiedRulingStampsAProposal.name(),
            "an-unratified-ruling-stamps-a-proposal"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("an-unratified-ruling-stamps-a-proposal");
    }
}
