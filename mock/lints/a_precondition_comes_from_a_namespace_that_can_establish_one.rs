//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The control on the precondition walk, standing outside it.
//!
//! A precondition says an obligation has a dependency, which leaves it further
//! from met rather than nearer. Only a result can establish one, so the walk
//! that collects them reads `law` and `proposal` and nothing else. An edge from
//! anywhere else contributes nothing, so the dependency it states is invisible
//! and the obligation reads less encumbered than it is.
//!
//! **The flattering direction again, and therefore the one to guard.** The
//! arithmetic instinct already runs the wrong way here: an obligation with four
//! established preconditions looks better attended than one with none, and is
//! worse off. An edge nobody reads makes that worse in the same direction, and
//! nothing about a clean run distinguishes it.
//!
//! **A lint rather than a tool, and a hard zero.** A `retirement` is a closed
//! route rather than a result, so an edge from one is not a dependency it could
//! have established. The repair is to move the edge to whatever result actually
//! establishes it, or to drop it.
//!
//! # The two lists this now guards across a boundary
//!
//! The walk that collects preconditions is a tool, because a report of what
//! depends on what has no pass line. So the list of namespaces lives twice:
//! once there and once here. **Nothing checks that the two agree**, and in the
//! crate this came from they were one const. What survives the split is that a
//! namespace missing from *either* list is reported here; what is lost is that
//! a namespace added here and forgotten there is a silent under-count again.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon carries no such edge. A unit test cannot build a
//! `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, has};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(APreconditionComesFromANamespaceThatCanEstablishOne)
}

const LINT: &str = "a-precondition-comes-from-a-namespace-that-can-establish-one";

/// The namespaces that can establish a precondition, and therefore carry the
/// edge.
///
/// A law may establish one as much as a proposal may, and the commonest shape
/// needs it: a reassociation measurement is a proposal and the associativity it
/// rests on is a law, so a list of proposals alone would leave the ordinary
/// case nowhere to go.
const SOURCES: [&str; 2] = ["law", "proposal"];

struct APreconditionComesFromANamespaceThatCanEstablishOne;
impl Lint for APreconditionComesFromANamespaceThatCanEstablishOne {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for APreconditionComesFromANamespaceThatCanEstablishOne {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for ns in ctx.registry.namespaces() {
            if SOURCES.contains(&ns) {
                continue;
            }
            for q in ctx.registry.rows_in(ns) {
                // Presence rather than content, for the same reason as the
                // untiered-edge lint: a `precondition_for` written and left
                // empty is exactly as unread as one carrying slugs.
                if !has(ctx.registry, q, "precondition_for") {
                    continue;
                }
                out.push(finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` carries a `precondition_for` edge from `{ns}`, which the walk \
                         does not read, so the dependency it states is invisible and the \
                         obligation reads less encumbered than it is."
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
        super::APreconditionComesFromANamespaceThatCanEstablishOne
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    /// A `retirement` is the plausible source: it is about a route to an
    /// obligation already, so writing this edge on one is the way it happens.
    #[test]
    fn an_edge_from_a_namespace_that_cannot_establish_one_is_reported() {
        let f = findings(&[
            ("obligation::a_thing", &[("need", "A thing.")]),
            (
                "retirement::a_dead_route",
                &[("claim", "A way."), ("precondition_for", "a_thing")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("retirement"), "{}", f[0]);
        assert!(f[0].contains("retirement::a_dead_route"), "{}", f[0]);
    }

    /// The control, and it is what separates this from a lint refusing the
    /// field outright. Both sources pass, and the `law` half matters: the
    /// reassociation case is a measurement and the associativity under it is a
    /// law, so a list of proposals alone would report the commonest shape.
    #[test]
    fn control_both_establishing_namespaces_are_silent() {
        for ns in super::SOURCES {
            let source = format!("{ns}::a_result");
            let f = findings(&[
                ("obligation::a_thing", &[("need", "A thing.")]),
                (&source, &[("precondition_for", "a_thing")]),
            ]);
            assert!(
                f.is_empty(),
                "`{ns}` can establish one and must pass: {f:?}"
            );
        }
    }

    /// An edge written and left empty is still unread by the walk.
    #[test]
    fn an_empty_edge_from_an_unreadable_namespace_is_still_reported() {
        let f = findings(&[("ruling::a_call", &[("precondition_for", "")])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// One finding per row rather than one per slug. The defect is the
    /// namespace, and the repair is one decision about the row.
    #[test]
    fn a_row_with_several_edges_is_one_finding() {
        let f = findings(&[(
            "retirement::a_dead_route",
            &[("precondition_for", "one_thing, another_thing")],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// Every unreadable namespace is reported rather than the first found.
    #[test]
    fn every_unreadable_namespace_carrying_the_edge_is_reported() {
        let f = findings(&[
            (
                "retirement::a_dead_route",
                &[("precondition_for", "a_thing")],
            ),
            ("ruling::a_call", &[("precondition_for", "a_thing")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    /// The `obligation` field is a different edge and is not read here.
    ///
    /// A `retirement` carrying `obligation` is the correct shape the schema
    /// declares, and confusing the two would report every closed route in the
    /// canon.
    #[test]
    fn control_an_obligation_edge_is_not_a_precondition() {
        let f = findings(&[(
            "retirement::a_dead_route",
            &[("claim", "A way."), ("obligation", "a_thing")],
        )]);
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
                "retirement::a_dead_route",
                &[("precondition_for", "a_thing")],
            )],
            &[],
        );
        assert_findings_block(
            &super::APreconditionComesFromANamespaceThatCanEstablishOne,
            &v,
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::APreconditionComesFromANamespaceThatCanEstablishOne);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::APreconditionComesFromANamespaceThatCanEstablishOne.name(),
            "a-precondition-comes-from-a-namespace-that-can-establish-one"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-precondition-comes-from-a-namespace-that-can-establish-one");
    }
}
