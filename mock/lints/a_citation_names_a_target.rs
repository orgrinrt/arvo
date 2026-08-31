//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A citation with one segment names a root and nothing in it.
//!
//! The engine reports a citation whose path does not resolve. It cannot report
//! one that is merely a root, or a bare filename with no root, because both
//! fail earlier and less legibly than a wrong path does: there is no path to
//! look for, so there is nothing to say did not resolve.
//!
//! **A lint rather than a tool, and a hard zero.** A citation naming nothing is
//! wrong at the moment it is written, whatever else the row says, and the
//! repair is to write the rest of the address.
//!
//! # One thing this can no longer see
//!
//! An empty string sitting in a citation array. The old crate parsed the TOML
//! itself and saw `["", "panel::x"]` as two entries, one of them empty, and
//! reported the empty one. A lint is handed the field already joined with
//! `", "`, so those two arrive as `", panel::x"` and the empty entry is not
//! recoverable from the text: it is indistinguishable from the separator that
//! joined it. Nothing in the committed canon carries one, the schema's own
//! pattern on a `ref[]` entry refuses it, and there is no route around the
//! join from inside the pack.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon carries no such citation. A unit test cannot build
//! a `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_citations::CITATION_FIELDS;
use crate::canon_rows::{finding, list};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ACitationNamesATarget)
}

const LINT: &str = "a-citation-names-a-target";

/// The separator between a citation's root, path and anchor.
const SEP: &str = "::";

struct ACitationNamesATarget;
impl Lint for ACitationNamesATarget {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ACitationNamesATarget {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for ns in ctx.registry.namespaces() {
            for q in ctx.registry.rows_in(ns) {
                for field in CITATION_FIELDS {
                    for citation in list(ctx.registry, q, field) {
                        if citation.split(SEP).count() >= 2 {
                            continue;
                        }
                        out.push(finding(
                            LINT,
                            None,
                            format!(
                                "`{q}` carries `{citation}` in `{field}`, which has no \
                                 `root::path` split. A citation with one segment names a \
                                 root and nothing in it."
                            ),
                        ));
                    }
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
    use crate::canon_rows::JOIN;

    const ROOT: &str = "panel::202608072330_the-numeral-canon-panel";

    fn cited(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::ACitationNamesATarget
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn a_citation_that_is_only_a_root_is_reported() {
        let f = findings(&[("ruling::a_call", &[("provenance", "panel")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ruling::a_call"), "{}", f[0]);
        assert!(f[0].contains("provenance"), "{}", f[0]);
    }

    /// A bare filename with no root is the other half of the same defect, and
    /// it is the one that looks most like a citation.
    #[test]
    fn a_bare_filename_with_no_root_is_reported() {
        let f = findings(&[(
            "ruling::a_call",
            &[(
                "provenance",
                "74_giesen_consolidation_the_number_system_concept",
            )],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// The control that makes the arm above mean something: an ordinary
    /// citation passes. A lint refusing every citation would read identically
    /// on a clean canon, since the canon would then simply carry none.
    #[test]
    fn control_a_root_and_a_path_is_silent() {
        let f = findings(&[(
            "ruling::a_call",
            &[(
                "provenance",
                &cited(&[&format!("{ROOT}::109_bellard::156")]),
            )],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// Two segments is the floor rather than three, so a citation ending at its
    /// file with no anchor passes.
    #[test]
    fn control_two_segments_is_enough() {
        let f = findings(&[("ruling::a_call", &[("provenance", "panel::AGREEMENTS")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// A probe's `lives` is read as well as `provenance`.
    #[test]
    fn a_probes_location_is_read_as_a_citation() {
        let f = findings(&[("probe::an_instrument", &[("lives", "panel")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("lives"), "{}", f[0]);
    }

    /// Every offending entry is reported rather than the first in the row.
    #[test]
    fn every_offending_citation_in_one_row_is_reported() {
        let p = cited(&["panel", "research", &format!("{ROOT}::109_bellard::156")]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    /// A field written and left empty names nothing and is not reported.
    ///
    /// The engine writes an empty array as the empty string, so an absent field
    /// and an empty one arrive here the same way, and neither carries a
    /// citation to complain about. A row owing a `provenance` it does not have
    /// is the schema's report.
    #[test]
    fn control_an_empty_citation_field_is_left_to_the_schema() {
        let f = findings(&[("ruling::a_call", &[("provenance", "")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("ruling::a_call", &[("provenance", "panel")])], &[]);
        assert_findings_block(&super::ACitationNamesATarget, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ACitationNamesATarget);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ACitationNamesATarget.name(),
            "a-citation-names-a-target"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-citation-names-a-target");
    }
}
