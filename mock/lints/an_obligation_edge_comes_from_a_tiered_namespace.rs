//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The control on the coverage measurement, standing outside it.
//!
//! What reaches each obligation is the bar the canon is measured against, and
//! the walk that measures it partitions by namespace from a written list: a
//! `ruling` meets an obligation, a `proposal` proposes against it, a
//! `retirement` closes one route to it. A namespace gaining the field and not
//! appearing on that list contributes nothing, so the coverage reads better
//! than it is, silently, and in the flattering direction.
//!
//! **This is the case that has to fail.** An instrument whose blind spot makes
//! the answer look better is the one shape a clean run cannot distinguish from
//! a real one.
//!
//! **A lint rather than a tool, and a hard zero.** There is a state it refuses:
//! an edge the measurement cannot read. The repair is to decide what a row in
//! that namespace means for an obligation and add it, which is a decision
//! rather than a ranking, and the row stays wrong until somebody makes it.
//!
//! # The two lists this now guards across a boundary
//!
//! The tiering itself is a tool, because a coverage report has no pass line. So
//! the list of namespaces lives twice: once there, as the tiering, and once
//! here, as what this refuses anything outside of. **Nothing checks that the
//! two agree**, and in the crate this came from they were one const with the
//! tier attached, which could not disagree with itself. What survives the split
//! is that a namespace missing from *either* list is reported by this lint, so
//! the failure is loud rather than silent; what is lost is that a namespace
//! added to this list and forgotten in the tool's is a silent under-count
//! again. Said plainly rather than left for somebody to find.
//!
//! # Why the reverse edges are not the instrument
//!
//! `referrers` is built from the fields the configuration types as row
//! references, per namespace. A namespace carrying an `obligation` the schema
//! does not declare there produces no typed edge, so it is invisible in the
//! reverse direction for the same reason the measurement cannot see it. That is
//! the defect, not the instrument for finding it. The forward field is where an
//! undeclared edge still exists, because the loader stores every key a row
//! writes whether the schema declares it or not.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon carries no untiered edge. A unit test cannot build
//! a `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, has};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AnObligationEdgeComesFromATieredNamespace)
}

const LINT: &str = "an-obligation-edge-comes-from-a-tiered-namespace";

/// The namespaces whose rows the coverage measurement knows how to tier.
const TIERED: [&str; 3] = ["ruling", "proposal", "retirement"];

struct AnObligationEdgeComesFromATieredNamespace;
impl Lint for AnObligationEdgeComesFromATieredNamespace {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AnObligationEdgeComesFromATieredNamespace {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for ns in ctx.registry.namespaces() {
            if TIERED.contains(&ns) {
                continue;
            }
            for q in ctx.registry.rows_in(ns) {
                // Presence rather than content. A row carrying an `obligation`
                // written and left empty is exactly as invisible to the
                // measurement as one carrying slugs, and it is the state a
                // content test would miss.
                if !has(ctx.registry, q, "obligation") {
                    continue;
                }
                out.push(finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` carries an `obligation` edge from `{ns}`, which the coverage \
                         measurement does not tier. It would be counted as nothing at all, \
                         so coverage would read better than it is. Decide what a row there \
                         means for an obligation and add it."
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
        super::AnObligationEdgeComesFromATieredNamespace
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    /// A `law` is the plausible fourth: it carries `precondition_for` already,
    /// so somebody adding `obligation` to it is the way this happens.
    #[test]
    fn an_edge_from_an_untiered_namespace_is_reported() {
        let f = findings(&[
            ("obligation::a_thing", &[("need", "A thing.")]),
            ("law::some_law", &[("obligation", "a_thing")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("law"), "{}", f[0]);
        assert!(f[0].contains("law::some_law"), "{}", f[0]);
    }

    /// The control, and it is the whole of what separates this from a lint that
    /// refuses the field outright: all three tiered namespaces pass.
    #[test]
    fn control_every_tiered_namespace_is_silent() {
        for ns in super::TIERED {
            let source = format!("{ns}::a_row");
            let f = findings(&[
                ("obligation::a_thing", &[("need", "A thing.")]),
                (&source, &[("obligation", "a_thing")]),
            ]);
            assert!(f.is_empty(), "`{ns}` is tiered and must pass: {f:?}");
        }
    }

    /// An edge written and left empty is still invisible to the measurement.
    ///
    /// The distinction the presence test turns on. A predicate asking what the
    /// edge says rather than whether it is there would let this through, and
    /// the row is exactly as uncounted either way.
    #[test]
    fn an_empty_edge_from_an_untiered_namespace_is_still_reported() {
        let f = findings(&[("law::some_law", &[("obligation", "")])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// One finding per row rather than one per slug.
    ///
    /// The defect is the namespace rather than the target, and the repair is
    /// one decision about that namespace however many edges it carries.
    #[test]
    fn a_row_with_several_edges_is_one_finding() {
        let f = findings(&[
            ("obligation::one_thing", &[("need", "A thing.")]),
            ("obligation::another_thing", &[("need", "Another.")]),
            (
                "law::some_law",
                &[("obligation", "one_thing, another_thing")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// Every untiered namespace is reported rather than the first found.
    #[test]
    fn every_untiered_namespace_carrying_the_edge_is_reported() {
        let f = findings(&[
            ("law::some_law", &[("obligation", "a_thing")]),
            ("probe::an_instrument", &[("obligation", "a_thing")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    /// The `obligation` namespace itself is untiered and carries no such field,
    /// so an ordinary obligation row is silent. A walk keyed on the field name
    /// alone rather than on presence in the row would have to say something
    /// about it.
    #[test]
    fn control_an_obligation_row_is_silent() {
        let f = findings(&[(
            "obligation::a_thing",
            &[("need", "A thing."), ("consumer", "any")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("law::some_law", &[("obligation", "a_thing")])], &[]);
        assert_findings_block(&super::AnObligationEdgeComesFromATieredNamespace, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AnObligationEdgeComesFromATieredNamespace);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AnObligationEdgeComesFromATieredNamespace.name(),
            "an-obligation-edge-comes-from-a-tiered-namespace"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("an-obligation-edge-comes-from-a-tiered-namespace");
    }
}
