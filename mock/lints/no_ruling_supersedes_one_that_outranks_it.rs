//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Only op retires what op ratified.
//!
//! `supersedes` says the named row is dead. A row he never stamped cannot carry
//! that edge into one he did, and a graph saying otherwise reports a ratified
//! decision as retired on the authority of an unratified one. That is the
//! provenance ladder inverted inside the namespace built to hold it.
//!
//! Chronology is not the test and deliberately so. Both instances that produced
//! this check were the later row of their pair, which is what made them look
//! ordinary: a newer statement replacing an older one is the shape `supersedes`
//! is for. What neither of them did was contradict its target. One confirmed
//! its target, when op was asked the same question a second time and said it
//! was already answered, and the other extended its target's horizon from one
//! night to a hundred stretches. An edge is earned by disagreeing, and both of
//! those agreed.
//!
//! **A lint rather than a tool, and a hard zero.** There is no legitimate
//! reading of the edge: either the row genuinely disagrees, which is a finding
//! for op rather than an edge, or it confirms and extends, which is the usual
//! case and the edge comes out. Both repairs are one edit.
//!
//! # Why the reverse edges are not the instrument here
//!
//! `supersedes` is typed `ruling[]`, so the engine does compute a reverse edge
//! for it and `referrers` would name the rows pointing at a given ruling. It
//! does not say through which field, and `corrects` is `ruling[]` on the same
//! namespace, so the two arrive as one undifferentiated list. Reading the
//! forward field keeps the two apart, and `corrects` says something else
//! entirely: it says the target was wrong about a detail rather than dead.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon carries no such edge. A unit test cannot build a
//! `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoRulingSupersedesOneThatOutranksIt)
}

const LINT: &str = "no-ruling-supersedes-one-that-outranks-it";

/// The tiers a `ruling` may sit at, weakest first.
///
/// `open` is a question wearing a ruling's clothes and `in_force` is a process
/// call nobody stamped, so both sit below the two that matter. The order exists
/// for the one comparison below and ranks nothing else.
const TIERS: [&str; 4] = ["open", "in_force", "stated", "ratified"];

/// Where a tier sits, or `None` for a spelling the schema has not seen.
///
/// An unknown spelling is left alone rather than guessed at. The schema refuses
/// it by name already, and ranking it here would mean this lint deciding an
/// order for a value nobody declared.
fn rank(tier: &str) -> Option<usize> {
    TIERS.iter().position(|known| *known == tier)
}

struct NoRulingSupersedesOneThatOutranksIt;
impl Lint for NoRulingSupersedesOneThatOutranksIt {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoRulingSupersedesOneThatOutranksIt {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for q in ctx.registry.rows_in("ruling") {
            let Some(mine) = text(ctx.registry, q, "rung").and_then(rank) else {
                continue;
            };
            for target in list(ctx.registry, q, "supersedes") {
                let dead = format!("ruling::{target}");
                // A target naming no ruling row is the engine's
                // `unknown-row-reference` to report, and answering a question
                // about a row that does not exist would be this lint
                // disagreeing with the engine about the same data.
                let Some(theirs_word) = text(ctx.registry, &dead, "rung") else {
                    continue;
                };
                let Some(theirs) = rank(theirs_word) else {
                    continue;
                };
                if theirs <= mine {
                    continue;
                }
                let mine_word = text(ctx.registry, q, "rung").unwrap_or("");
                out.push(finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` sets `rung` to `{mine_word}` and `supersedes` names \
                         `{target}`, whose `rung` is `{theirs_word}`. This row therefore \
                         reports a decision he stamped as retired, on the authority of one \
                         he did not. Only he retires what he ratified. If this row \
                         genuinely disagrees with that one, the disagreement is a finding \
                         for him rather than an edge; if it confirms or extends it, which \
                         is the usual case, drop the edge."
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
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::NoRulingSupersedesOneThatOutranksIt
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    /// A `stated` row superseding a `ratified` one is the exact shape that was
    /// found twice, so it is the shape asserted to fire.
    #[test]
    fn a_stated_ruling_superseding_a_ratified_one_is_reported() {
        let f = findings(&[
            (
                "ruling::he_never_stamped_this",
                &[("rung", "stated"), ("supersedes", "he_stamped_this")],
            ),
            ("ruling::he_stamped_this", &[("rung", "ratified")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("he_stamped_this"), "{}", f[0]);
        assert!(f[0].contains("ratified"), "{}", f[0]);
    }

    /// The other direction is the ordinary correct shape and must not fire.
    ///
    /// A ratified row retiring a stated one is op replacing his own earlier
    /// wording, which is what `supersedes` exists for. So is an edge between
    /// two rows at the same tier. A lint refusing either would be refusing the
    /// namespace rather than checking it.
    #[test]
    fn control_a_ratified_ruling_superseding_a_stated_one_is_silent() {
        let f = findings(&[
            (
                "ruling::he_stamped_this_later",
                &[("rung", "ratified"), ("supersedes", "he_only_said_this")],
            ),
            ("ruling::he_only_said_this", &[("rung", "stated")]),
            (
                "ruling::one_stated_row",
                &[("rung", "stated"), ("supersedes", "another_stated_row")],
            ),
            ("ruling::another_stated_row", &[("rung", "stated")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Every tier below the target's is reported, rather than the adjacent one.
    ///
    /// A comparison written as "one step below" would pass the arm above and
    /// miss an `open` row retiring a `ratified` one, which is the worst
    /// instance of the class.
    #[test]
    fn every_lower_tier_is_reported_against_a_ratified_target() {
        for weaker in ["open", "in_force", "stated"] {
            let f = findings(&[
                (
                    "ruling::the_weaker_one",
                    &[("rung", weaker), ("supersedes", "he_stamped_this")],
                ),
                ("ruling::he_stamped_this", &[("rung", "ratified")]),
            ]);
            assert_eq!(f.len(), 1, "`{weaker}` was not compared: {f:?}");
        }
    }

    /// Each offending target is reported rather than the first in the row.
    #[test]
    fn every_offending_edge_in_one_row_is_reported() {
        let f = findings(&[
            (
                "ruling::the_weaker_one",
                &[
                    ("rung", "in_force"),
                    ("supersedes", "one_ratified, another_ratified"),
                ],
            ),
            ("ruling::one_ratified", &[("rung", "ratified")]),
            ("ruling::another_ratified", &[("rung", "ratified")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    /// A `corrects` edge is a different claim and is not read.
    ///
    /// It says the target was wrong about a detail rather than dead, and both
    /// fields are typed `ruling[]`, so the reverse edges cannot tell them
    /// apart. Reading the forward field is what keeps them separate, and this
    /// is the arm that says so.
    #[test]
    fn control_a_corrects_edge_is_not_a_supersession() {
        let f = findings(&[
            (
                "ruling::he_never_stamped_this",
                &[("rung", "stated"), ("corrects", "he_stamped_this")],
            ),
            ("ruling::he_stamped_this", &[("rung", "ratified")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// A target naming no ruling row is the engine's report to make.
    #[test]
    fn control_an_edge_into_nothing_is_left_to_the_engine() {
        let f = findings(&[(
            "ruling::a_call",
            &[("rung", "stated"), ("supersedes", "no_such_ruling")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// A tier spelling the schema has not seen is left alone at either end.
    ///
    /// Ranking it would be this lint deciding an order for a value nobody
    /// declared, and the schema refuses the spelling by name already.
    #[test]
    fn control_an_unknown_tier_is_not_ranked() {
        let f = findings(&[
            (
                "ruling::the_source",
                &[("rung", "invented"), ("supersedes", "the_target")],
            ),
            ("ruling::the_target", &[("rung", "ratified")]),
            (
                "ruling::the_other_source",
                &[("rung", "stated"), ("supersedes", "the_other_target")],
            ),
            ("ruling::the_other_target", &[("rung", "invented")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Only `ruling` is read. `proposal` carries a `supersedes` of its own,
    /// typed `proposal[]`, and it carries no `rung`, so the comparison this
    /// lint makes has no meaning there.
    #[test]
    fn control_a_proposal_supersession_is_not_read() {
        let f = findings(&[
            (
                "proposal::a_claim",
                &[("rung", "stated"), ("supersedes", "an_older_claim")],
            ),
            ("proposal::an_older_claim", &[("rung", "ratified")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_ruling_with_no_supersedes_owes_nothing() {
        let f = findings(&[("ruling::a_call", &[("rung", "stated")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[
                (
                    "ruling::weak",
                    &[("rung", "stated"), ("supersedes", "strong")],
                ),
                ("ruling::strong", &[("rung", "ratified")]),
            ],
            &[],
        );
        assert_findings_block(&super::NoRulingSupersedesOneThatOutranksIt, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoRulingSupersedesOneThatOutranksIt);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::NoRulingSupersedesOneThatOutranksIt.name(),
            "no-ruling-supersedes-one-that-outranks-it"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("no-ruling-supersedes-one-that-outranks-it");
    }
}
