//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An `obligation` edge names an obligation that exists.
//!
//! An obligation slug is an address. Landed prose cites it, the coverage
//! measurement keys on it, and a rename orphans every citation to it and this
//! edge with them. The row then reads as reaching something and reaches
//! nothing.
//!
//! **A lint rather than a tool, and a hard zero.** A typo or a stale rename is
//! wrong at the moment it is written and the repair is one edit.
//!
//! # This overlaps the engine deliberately, and the reason is the suite
//!
//! The engine reports a dangling row reference as `unknown-row-reference`, so
//! the gate would refuse this anyway. What the engine cannot give is a control
//! a test can plant: a check only the engine performs is one this pack cannot
//! exercise on a planted registry, so a rename would have nowhere to fail that
//! the suite can see. Having the predicate here costs one walk and buys the
//! arms below.
//!
//! # Why the reverse edges cannot answer this
//!
//! Not merely a worse instrument: a structurally blind one. `obligation` is
//! typed `obligation[]`, so the engine does build reverse edges for it, and
//! `for_each_edge` emits an edge **only where the target is a row**. A slug
//! naming no obligation therefore produces no edge at all, so `referrers`
//! reports exactly the same thing for a row nothing points at and for one whose
//! every pointer is broken. The forward field is the only place the broken
//! pointer still exists.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon carries no such edge. A unit test cannot build a
//! `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AnObligationEdgeNamesARow)
}

const LINT: &str = "an-obligation-edge-names-a-row";

/// The namespaces the schema gives an `obligation` field.
///
/// The same three the coverage tool tiers by, and nothing ties the two lists
/// together across the pack and tool boundary. What holds them together is
/// `an-obligation-edge-comes-from-a-tiered-namespace`, which refuses an edge
/// from anywhere else, so a fourth namespace gaining the field is reported
/// rather than silently unread by either.
const BEARING: [&str; 3] = ["ruling", "proposal", "retirement"];

struct AnObligationEdgeNamesARow;
impl Lint for AnObligationEdgeNamesARow {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AnObligationEdgeNamesARow {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for ns in BEARING {
            for q in ctx.registry.rows_in(ns) {
                for slug in list(ctx.registry, q, "obligation") {
                    if ctx.registry.row(&format!("obligation::{slug}")).is_some() {
                        continue;
                    }
                    out.push(finding(
                        LINT,
                        None,
                        format!(
                            "`{q}` names obligation `{slug}`, which is not a row. An \
                             obligation slug is an address and landed prose cites it, so a \
                             rename orphans every citation to it and this edge with them."
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
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::AnObligationEdgeNamesARow
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn an_edge_naming_nothing_is_reported_and_the_real_one_beside_it_is_not() {
        let f = findings(&[
            ("obligation::the_real_one", &[("need", "A thing.")]),
            (
                "proposal::a_claim",
                &[("obligation", "the_real_one, a_slug_that_was_renamed")],
            ),
        ]);
        assert_eq!(f.len(), 1, "only the orphan: {f:?}");
        assert!(f[0].contains("a_slug_that_was_renamed"), "{}", f[0]);
        assert!(f[0].contains("proposal::a_claim"), "{}", f[0]);
    }

    /// The control. An edge naming a row that exists is the ordinary shape, and
    /// a lint refusing it would refuse the field itself.
    #[test]
    fn control_an_edge_naming_a_row_is_silent() {
        let f = findings(&[
            ("obligation::a_thing", &[("need", "A thing.")]),
            ("proposal::a_claim", &[("obligation", "a_thing")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// All three bearing namespaces are read rather than the first of them.
    #[test]
    fn every_bearing_namespace_is_read() {
        for ns in super::BEARING {
            let source = format!("{ns}::a_row");
            let f = findings(&[
                ("obligation::a_thing", &[("need", "A thing.")]),
                (&source, &[("obligation", "no_such_obligation")]),
            ]);
            assert_eq!(f.len(), 1, "`{ns}` was not read: {f:?}");
        }
    }

    /// The lookup is inside the `obligation` namespace and not anywhere.
    ///
    /// A slug that happens to name a row in some other namespace still names no
    /// obligation. A check written over every slug in the registry would pass
    /// this and mean nothing.
    #[test]
    fn a_slug_naming_a_row_in_another_namespace_is_still_an_orphan() {
        let f = findings(&[
            ("proposal::a_thing", &[("says", "Something.")]),
            ("ruling::a_call", &[("obligation", "a_thing")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// Every orphan in one row is reported rather than the first.
    #[test]
    fn every_orphan_in_one_row_is_reported() {
        let f = findings(&[(
            "retirement::a_dead_route",
            &[("obligation", "one_ghost, another_ghost")],
        )]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    /// A field written and left empty names no orphan.
    #[test]
    fn control_an_empty_obligation_field_names_nothing() {
        let f = findings(&[("proposal::a_claim", &[("obligation", "")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_row_with_no_obligation_edge_owes_nothing() {
        let f = findings(&[("proposal::a_claim", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[("proposal::a_claim", &[("obligation", "no_such_obligation")])],
            &[],
        );
        assert_findings_block(&super::AnObligationEdgeNamesARow, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AnObligationEdgeNamesARow);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AnObligationEdgeNamesARow.name(),
            "an-obligation-edge-names-a-row"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("an-obligation-edge-names-a-row");
    }
}
