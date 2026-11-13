//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A proposal superseding one that a ratified ruling stamps.
//!
//! A proposal is not canon, and the proposal a ratified ruling names under
//! `ratifies` is. A `supersedes` edge from the first to the second says the
//! canon row is dead on the authority of a row nobody stamped, and leaves the
//! ruling standing on a proposal the graph reports as retired. Either reading
//! is wrong: the ruling is then ratifying nothing live, or the edge is.
//!
//! The ruling-side twin is `no-ruling-supersedes-one-that-outranks-it`. This
//! one reads proposals, which carry no `rung` of their own, so what ranks a
//! proposal is whether a ruling at `ratified` stamps it.
//!
//! Repairs, one edit each: where the newer proposal replaces nothing the older
//! one says, the edge comes out; where it does replace it, the disagreement is
//! a finding for a ruling to settle, which re-points `ratifies` or ratifies the
//! successor in a row of its own.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, slug, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoProposalSupersedesARatifiedOne)
}

const LINT: &str = "no-proposal-supersedes-a-ratified-one";

struct NoProposalSupersedesARatifiedOne;
impl Lint for NoProposalSupersedesARatifiedOne {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoProposalSupersedesARatifiedOne {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry();
        // Each proposal a ratified ruling stamps, with the ruling stamping it.
        let stamped: Vec<(&str, &str)> = reg
            .rows_in("ruling")
            .iter()
            .filter(|r| text(reg, r, "rung") == Some("ratified"))
            .flat_map(|r| {
                list(reg, r, "ratifies")
                    .into_iter()
                    .map(move |p| (p, slug(r)))
            })
            .collect();
        let mut out = Vec::new();
        for q in reg.rows_in("proposal") {
            // A proposal a ratified ruling stamps is canon itself, and may retire
            // what it replaces: that is the repair the finding names.
            if stamped.iter().any(|(p, _)| *p == slug(q)) {
                continue;
            }
            for target in list(reg, q, "supersedes") {
                for (_, ruling) in stamped.iter().filter(|(p, _)| *p == target) {
                    out.push(finding(
                        LINT,
                        None,
                        format!(
                            "`{q}` supersedes `{target}`, which `ruling::{ruling}` ratifies. \
                             A proposal does not retire canon. Where `{q}` replaces nothing \
                             `{target}` says, take the edge out; where it does, a ruling \
                             settles it, re-pointing `ratifies` or ratifying `{q}` itself."
                        ),
                    ));
                }
            }
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block,
        assert_not_declared_off,
        assert_registered,
        findings,
        view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::NoProposalSupersedesARatifiedOne, &view(rows, &[]))
    }

    const STAMP: (&str, &[(&str, &str)]) = ("ruling::stamp", &[
        ("rung", "ratified"),
        ("ratifies", "old"),
    ]);

    #[test]
    fn a_proposal_superseding_a_ratified_one_is_reported() {
        // The planted case, the shape the canon carried on 12 November.
        let f =
            found(&[STAMP, ("proposal::old", &[]), ("proposal::new", &[("supersedes", "old")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(
            f[0].contains("proposal::new") && f[0].contains("ruling::stamp"),
            "{}",
            f[0]
        );
    }

    #[test]
    fn control_a_proposal_superseding_an_unstamped_one_is_silent() {
        // Panels iterate on each other's work, which is what the edge is for.
        let f = found(&[("proposal::old", &[]), ("proposal::new", &[("supersedes", "old")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_stamp_below_ratified_does_not_rank_the_proposal() {
        let f = found(&[
            ("ruling::stamp", &[("rung", "stated"), ("ratifies", "old")]),
            ("proposal::new", &[("supersedes", "old")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_stamp_on_the_target_is_named() {
        let f = found(&[
            STAMP,
            ("ruling::again", &[
                ("rung", "ratified"),
                ("ratifies", "old"),
            ]),
            ("proposal::new", &[("supersedes", "old")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn control_a_ratified_proposal_may_supersede_a_ratified_one() {
        // The repair the finding names: a ruling ratifies the new row too.
        let f = found(&[
            STAMP,
            ("ruling::later", &[
                ("rung", "ratified"),
                ("ratifies", "new"),
            ]),
            ("proposal::old", &[]),
            ("proposal::new", &[("supersedes", "old")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[STAMP, ("proposal::new", &[("supersedes", "old")])], &[]);
        assert_findings_block(&super::NoProposalSupersedesARatifiedOne, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoProposalSupersedesARatifiedOne);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::NoProposalSupersedesARatifiedOne.name(),
            "no-proposal-supersedes-a-ratified-one"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("no-proposal-supersedes-a-ratified-one");
    }
}
