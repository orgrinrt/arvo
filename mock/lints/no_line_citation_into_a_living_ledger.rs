//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A line citation into a file that moves is a lie that reads as a citation.
//!
//! The `panel` root is declared frozen, which is what makes a line number
//! honest for the numbered member files: each is written once and never edited.
//! The same root holds the panel's living ledgers, and freezing is per root
//! rather than per file, so the declaration permits a line citation into those
//! too. It should not, and this is the refusal the declaration cannot make.
//!
//! The failure is the worst shape there is. An edit anywhere above a cited line
//! shifts it, the citation still resolves, the engine reports nothing, and the
//! anchor now points at different text. The only case that fails loudly is a
//! line past the end of the file, and that is the case that matters least.
//!
//! A heading anchor into the same file is fine and is what to write instead: a
//! rename stops it resolving, which is a report rather than a lie.
//!
//! **A lint rather than a tool, and a hard zero.** There is no legitimate
//! reading of a line number into a file still being written, and the repair is
//! one edit: name the heading the line sits under.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon cites no moving line. A unit test cannot build a
//! `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_citations::{is_line_anchor, ledger_named, CITATION_FIELDS};
use crate::canon_rows::{finding, list};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoLineCitationIntoALivingLedger)
}

const LINT: &str = "no-line-citation-into-a-living-ledger";

struct NoLineCitationIntoALivingLedger;
impl Lint for NoLineCitationIntoALivingLedger {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoLineCitationIntoALivingLedger {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        // Every row rather than a named list of namespaces. A citation is a
        // citation whichever namespace wrote it, and the fields are the same
        // two everywhere they appear.
        for ns in ctx.registry.namespaces() {
            for q in ctx.registry.rows_in(ns) {
                for field in CITATION_FIELDS {
                    for citation in list(ctx.registry, q, field) {
                        if !is_line_anchor(citation) {
                            continue;
                        }
                        let Some(ledger) = ledger_named(citation) else {
                            continue;
                        };
                        out.push(finding(
                            LINT,
                            None,
                            format!(
                                "`{q}` cites `{citation}` in `{field}`, which is a line \
                                 number into `{ledger}.md`. That file is still being \
                                 written, so the line moves and the citation keeps \
                                 resolving to whatever ends up there. Cite a heading \
                                 instead: a rename fails loudly, a shifted line does not."
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
        super::NoLineCitationIntoALivingLedger
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn a_line_into_a_ledger_is_reported() {
        let p = cited(&[&format!("{ROOT}::OPTIONS::2656")]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("OPTIONS"), "{}", f[0]);
        assert!(f[0].contains("ruling::a_call"), "{}", f[0]);
    }

    /// The first half of the control: a heading into the same file is the fix,
    /// so it must pass. A lint refusing both would be refusing every citation
    /// into the ledgers, which is not the rule and would push authors to cite
    /// nothing.
    #[test]
    fn control_a_heading_into_the_same_ledger_is_silent() {
        let p = cited(&[&format!(
            "{ROOT}::OPTIONS::#q41-whether-an-arms-predicate-may-read-data"
        )]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert!(
            f.is_empty(),
            "a heading anchor fails loudly when renamed and is what to write instead: {f:?}"
        );
    }

    /// The second half: a line into a numbered member file is honest and must
    /// pass, because those are written once. A lint refusing every line number
    /// would be refusing the corpus's own citation style, and the corpus is
    /// right about it.
    #[test]
    fn control_a_line_into_a_numbered_member_file_is_silent() {
        let p = cited(&[&format!(
            "{ROOT}::109_bellard_the_primitive_derived_cold::156"
        )]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// A ledger named with its extension still resolves to the ledger.
    ///
    /// Citations may omit the extension and the engine finds the file either
    /// way, so a lint matching on the bare stem alone would miss half of them.
    #[test]
    fn a_ledger_named_with_its_extension_is_still_caught() {
        let p = cited(&[&format!("{ROOT}::AGREEMENTS.md::468")]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("AGREEMENTS"), "{}", f[0]);
    }

    /// A probe's `lives` is a citation too, and reading only `provenance` would
    /// leave out the one namespace whose whole subject is where the evidence
    /// sits.
    #[test]
    fn a_probes_location_is_read_as_a_citation() {
        let p = cited(&[&format!("{ROOT}::RULES::12")]);
        let f = findings(&[("probe::an_instrument", &[("lives", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("lives"), "{}", f[0]);
    }

    /// Every ledger in the list is read rather than the first of them.
    ///
    /// A list written and then half-consulted reads exactly like a complete
    /// one, and nine names is enough that a partial walk would go unnoticed.
    #[test]
    fn every_declared_ledger_is_read() {
        for ledger in crate::canon_citations::LIVING_LEDGERS {
            let p = cited(&[&format!("{ROOT}::{ledger}::10")]);
            let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
            assert_eq!(f.len(), 1, "`{ledger}` was not read: {f:?}");
        }
    }

    /// Every offending citation is reported rather than the first in the row.
    #[test]
    fn every_offending_citation_in_one_row_is_reported() {
        let p = cited(&[
            &format!("{ROOT}::OPTIONS::2656"),
            &format!("{ROOT}::AGREEMENTS::468"),
            &format!("{ROOT}::109_bellard_the_primitive_derived_cold::156"),
        ]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert_eq!(
            f.len(),
            2,
            "the member file is honest and the two ledgers are not: {f:?}"
        );
    }

    /// A citation naming a ledger with no anchor at all is not a line citation.
    #[test]
    fn control_a_ledger_cited_without_an_anchor_is_silent() {
        let p = cited(&[&format!("{ROOT}::OPTIONS")]);
        let f = findings(&[("ruling::a_call", &[("provenance", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_row_with_no_citation_fields_owes_nothing() {
        let f = findings(&[("ruling::a_call", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let p = format!("{ROOT}::OPTIONS::2656");
        let v = view(&[("ruling::a_call", &[("provenance", &p)])], &[]);
        assert_findings_block(&super::NoLineCitationIntoALivingLedger, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoLineCitationIntoALivingLedger);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::NoLineCitationIntoALivingLedger.name(),
            "no-line-citation-into-a-living-ledger"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("no-line-citation-into-a-living-ledger");
    }
}
