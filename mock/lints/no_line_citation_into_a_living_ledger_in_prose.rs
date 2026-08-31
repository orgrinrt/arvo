//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A line citation into a ledger that is still being written, from the panel's
//! own prose.
//!
//! The largest citation-rot class here, and nothing was watching it. A registry
//! row may not cite a living ledger by line and a separate lint refuses that.
//! Nothing looked at the prose those rows are drawn from, which is where nearly
//! all of the citations are: a ledger is rewritten as the panel moves, so a line
//! number into one names different text after any edit above it, **and the
//! citation keeps resolving the whole way**.
//!
//! **Found by repair rather than by reading.** Fifteen probes were pointed at
//! the tree they sit in instead of at a clone where the old line numbers still
//! matched. Three independently written citation checkers immediately reported
//! the same ledger's citations failing. None of them could see it before and
//! neither could any committed check.
//!
//! Four sat in ledgers, which are edited and so repairable, and opening them is
//! what shows why the class matters. One pointed at a blank line. One pointed at
//! a passage about a different subject. **One pointed at text that still
//! supported the claim it was cited for**, which is the failure this whole
//! discipline exists to prevent: the reader lands somewhere real, checks it, and
//! finds agreement that nobody wrote.
//!
//! **Two populations**, on the same split the registry-line lint uses and for
//! the same reason. A ledger is edited, so a citation in one is repaired. A
//! member file is written once, so its citations are counted rather than
//! repointed, and the repair is upstream: a seat that cites headings writes none
//! of these.
//!
//! A heading anchor is what to write instead. A rename stops it resolving, which
//! is a report rather than a lie.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{
    finding, ledger_stems, line_citations, markdown, panel_dir, shown, LIVING_LEDGERS,
};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoLineCitationIntoALivingLedgerInProse)
}

const NAME: &str = "no-line-citation-into-a-living-ledger-in-prose";

/// The standing count in files that are the record, measured over the committed
/// tree.
///
/// 676 when the class was first measured, across 98 files. It falls only when a
/// file is superseded. **Do not raise it.**
const CEILING: usize = 673;

struct NoLineCitationIntoALivingLedgerInProse;
impl Lint for NoLineCitationIntoALivingLedgerInProse {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoLineCitationIntoALivingLedgerInProse {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(&panel_dir(ctx.mock_dir), CEILING)
    }
}

fn check(dir: &Path, ceiling: usize) -> Vec<LintError> {
    let found = citations(dir);
    let (in_ledgers, in_the_record): (
        Vec<&(String, usize, String)>,
        Vec<&(String, usize, String)>,
    ) = found.iter().partition(|(at, _, _)| is_a_living_ledger(at));

    let mut out: Vec<LintError> = in_ledgers
        .iter()
        .map(|(at, line, cited)| {
            finding(
                NAME,
                at,
                *line,
                format!(
                    "cites `{cited}`. That ledger is still being written, so an edit above \
                     the line moves the target and the citation still resolves, naming text \
                     nobody wrote there. This file is edited too, so cite the heading, which \
                     fails loudly when renamed."
                ),
            )
        })
        .collect();

    if in_the_record.len() > ceiling {
        let mut per_file: Vec<String> = in_the_record.iter().map(|(at, _, _)| at.clone()).collect();
        per_file.sort();
        per_file.dedup();
        out.push(finding(
            NAME,
            "the panel's landed files",
            0,
            format!(
                "{} line citations into a living ledger sit in files that are the record, \
                 against a ceiling of {ceiling}. Repointing one would be editing history to \
                 make a checker green, so what this refuses is a new one. Brief the next seat \
                 to cite headings; do not raise the number. Across {} file(s).",
                in_the_record.len(),
                per_file.len()
            ),
        ));
    }
    out
}

/// Every `<LEDGER>.md:<line>` the panel's prose carries.
fn citations(dir: &Path) -> Vec<(String, usize, String)> {
    let stems = ledger_stems();
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            for cited in line_citations(line, ".md", &stems) {
                out.push((at.clone(), n + 1, cited));
            }
        }
    }
    out
}

/// Whether a file is one a reader treats as current, and may therefore edit.
///
/// The seat that reported this class caught the same shape in its own
/// classifier on the same day, matching a repository name as a substring and
/// getting the flattering answer. A classifier that partitions by name gets the
/// cases nobody thought of wrong, and it gets them wrong quietly.
fn is_a_living_ledger(at: &str) -> bool {
    !at.contains('/') && LIVING_LEDGERS.contains(&at)
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };
    use crate::panel_corpus::PANEL;

    fn findings(what: &str, files: &[(&str, &str)], ceiling: usize) -> Vec<String> {
        let dir = planted_tree(what);
        for (at, text) in files {
            plant(&dir, at, text);
        }
        super::check(&dir, ceiling)
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn a_ledger_line_fires_and_a_heading_a_member_file_and_an_archive_do_not() {
        let f = findings(
            "ledger-prose-mix",
            &[(
                "RULES.md",
                "See `RULES.md:163` and `OPTIONS.md:1113-1115`.\n\
                 Under \"Panel mechanics\" in `RULES.md`, which is a heading.\n\
                 And `109_bellard_the_primitive_derived_cold.md:156`, a member file.\n\
                 And `seed/OLD_SETTLED.md:44`, archived and frozen.\n",
            )],
            usize::MAX,
        );
        assert_eq!(
            f.len(),
            2,
            "both ledger lines fire; the heading, the member file and the archive do not. A \
             member file is written once and an archive is frozen, so a line into either is \
             honest: {f:?}"
        );
        assert!(f.iter().any(|m| m.contains("RULES.md:163")), "{f:?}");
        assert!(f.iter().any(|m| m.contains("OPTIONS.md:1113")), "{f:?}");
    }

    #[test]
    fn control_naming_a_ledger_without_a_line_is_left_alone() {
        assert!(
            findings(
                "ledger-prose-noline",
                &[(
                    "RULES.md",
                    "The register is `OPTIONS.md` and the droplist is `DROPLIST.md`, cited by \
                     neither line.\n",
                )],
                usize::MAX,
            )
            .is_empty(),
            "naming the ledger is not citing into it"
        );
    }

    #[test]
    fn a_member_file_is_counted_rather_than_refused_and_the_ratchet_fires_above_the_ceiling() {
        let files = [("42_member.md", "See `RULES.md:163`.\n")];
        assert!(
            findings("ledger-prose-under", &files, 1).is_empty(),
            "one citation under a ceiling of one is the grandfathered population"
        );
        let f = findings("ledger-prose-over", &files, 0);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ceiling of 0"), "{}", f[0]);
    }

    #[test]
    fn a_probes_own_findings_file_is_the_record_rather_than_a_ledger() {
        let files = [("214_probes/run/FINDINGS.md", "See `RULES.md:163`.\n")];
        assert!(
            findings("ledger-prose-probe", &files, 1).is_empty(),
            "a file inside a landed probe directory is not a ledger"
        );
        assert_eq!(findings("ledger-prose-probe2", &files, 0).len(), 1);
    }

    #[test]
    fn every_ledger_is_covered_rather_than_the_one_the_fixture_names() {
        // The coverage arm. A fixture citing `RULES.md` alone passes for as
        // long as any other ledger is invisible to the finder, and keeps
        // passing.
        for stem in crate::panel_corpus::ledger_stems() {
            let f = findings(
                &format!("ledger-prose-{stem}"),
                &[("RULES.md", &format!("See `{stem}.md:12`.\n"))],
                usize::MAX,
            );
            assert_eq!(f.len(), 1, "`{stem}` is on the ledger list and is not seen");
        }
    }

    #[test]
    fn a_panel_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("ledger-prose-absent");
        assert!(super::check(&dir.join("nothing"), 0).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("ledger-prose-severity");
        plant(
            &dir,
            &format!("mock/{PANEL}/RULES.md"),
            "See `OPTIONS.md:1113`.\n",
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(
            &super::NoLineCitationIntoALivingLedgerInProse,
            &ctx_at(&dir.join("mock"), &empty),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoLineCitationIntoALivingLedgerInProse);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::NoLineCitationIntoALivingLedgerInProse.name(),
            super::NAME
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
