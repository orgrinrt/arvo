//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A ruling at the ratified tier that does not say which route ratified it.
//!
//! The schema calls `ratified_by` required on a `ratified` row and meaningless
//! elsewhere, and a field declaration can only say required or optional for the
//! whole namespace, so the field is declared optional and the condition was
//! held by nobody. Eighteen ratified rows went without it, and nothing could
//! tell a reader whether op had stated them or two experts had converged on
//! them, which is the first thing worth knowing about a row that turns out
//! wrong.
//!
//! A lint rather than a tool, because there is no legitimate reading of the
//! state. Either the row is ratified and somebody ratified it, or it is not
//! ratified and `rung` says so. Both are one edit to one row.
//!
//! Only the required half is checked. A `ratified_by` on a row at another tier
//! is what the schema calls meaningless, which is a weaker word than wrong, so
//! this does not report it. Which values the field takes is the schema's own
//! closed list and is not repeated here.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ARatifiedRulingNamesItsRatifier)
}

const LINT: &str = "a-ratified-ruling-names-its-ratifier";

/// The namespace carrying `rung` and `ratified_by`.
const NAMESPACE: &str = "ruling";

/// The one tier the field is required at.
const RATIFIED: &str = "ratified";

struct ARatifiedRulingNamesItsRatifier;
impl Lint for ARatifiedRulingNamesItsRatifier {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ARatifiedRulingNamesItsRatifier {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        ctx.registry()
            .rows_in(NAMESPACE)
            .iter()
            .filter(|q| text(ctx.registry(), q, "rung") == Some(RATIFIED))
            .filter(|q| {
                text(ctx.registry(), q, "ratified_by")
                    .is_none_or(|by| by.trim().is_empty())
            })
            .map(|q| {
                finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` is at `rung = \"ratified\"` and carries no `ratified_by`. Say \
                         which route made it canon, `op`, `experts` or `both`, from the record \
                         of how it was ratified; where no record says, the row is not shown to \
                         be ratified and its `rung` is what is wrong."
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
        assert_findings_block,
        assert_not_declared_off,
        assert_registered,
        findings,
        view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ARatifiedRulingNamesItsRatifier, &view(rows, &[]))
    }

    #[test]
    fn a_ratified_ruling_with_no_ratifier_is_reported() {
        let f = found(&[
            ("ruling::stamped_by_nobody", &[("rung", "ratified")]),
            ("ruling::stamped_by_him", &[
                ("rung", "ratified"),
                ("ratified_by", "op"),
            ]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("stamped_by_nobody"), "{}", f[0]);
        assert!(
            f[0].contains("ratified_by"),
            "the report names the field, or a reader cannot tell what to add: {}",
            f[0]
        );
    }

    #[test]
    fn a_ratifier_present_and_blank_names_nobody() {
        for blank in ["", " ", "\n"] {
            let f = found(&[("ruling::r", &[
                ("rung", "ratified"),
                ("ratified_by", blank),
            ])]);
            assert_eq!(f.len(), 1, "{blank:?} passed as a ratifier: {f:?}");
        }
    }

    #[test]
    fn control_every_route_the_schema_names_passes() {
        // An arm keyed on one value would report the rows the experts
        // promoted, which are ratified by a route of their own.
        for by in ["op", "experts", "both"] {
            let f = found(&[("ruling::r", &[
                ("rung", "ratified"),
                ("ratified_by", by),
            ])]);
            assert!(f.is_empty(), "{by:?} was not read as a ratifier: {f:?}");
        }
    }

    #[test]
    fn control_a_ruling_at_another_tier_owes_no_ratifier() {
        // Most of the namespace is direction and not canon. An arm keyed on
        // the field alone, with no look at the tier, would report all of it.
        for rung in ["stated", "in_force", "open"] {
            let f = found(&[("ruling::r", &[("rung", rung)])]);
            assert!(f.is_empty(), "{rung:?} was asked for a ratifier: {f:?}");
        }
    }

    #[test]
    fn control_a_ruling_with_no_rung_at_all_is_not_this_lints() {
        // A missing tier is the schema's to refuse, since the field is
        // required. Reporting it here would say the row is ratified.
        assert!(found(&[("ruling::r", &[("says", "something")])]).is_empty());
    }

    #[test]
    fn a_ratifier_on_a_row_at_another_tier_is_left_alone() {
        // The schema's word for it is meaningless, and this lint holds the
        // required half only.
        let f = found(&[("ruling::r", &[
            ("rung", "stated"),
            ("ratified_by", "op"),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_proposal_is_not_read_whatever_it_carries() {
        // Only `ruling` carries `rung`. Every arm above plants one
        // namespace, so none of them can tell whether the others are read.
        let f = found(&[("proposal::p", &[("rung", "ratified")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_offending_ruling_is_reported_rather_than_the_first() {
        let f = found(&[
            ("ruling::a", &[("rung", "ratified")]),
            ("ruling::b", &[("rung", "ratified"), ("ratified_by", " ")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("ruling::r", &[("rung", "ratified")])], &[]);
        assert_findings_block(&super::ARatifiedRulingNamesItsRatifier, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ARatifiedRulingNamesItsRatifier);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ARatifiedRulingNamesItsRatifier.name(),
            "a-ratified-ruling-names-its-ratifier"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-ratified-ruling-names-its-ratifier");
    }
}
