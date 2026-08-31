//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A line citation into a registry file names a different row every time
//! somebody adds one.
//!
//! The panel's living ledgers were the obvious moving target and a separate lint
//! refuses line citations into those from inside a row. **The registry is worse
//! and nobody was watching it**: it is the working surface, every seat writes
//! rows into it, and a row inserted anywhere shifts every line below it in that
//! file.
//!
//! It has already happened, and the seat that caused it reported it against
//! itself: a member file cites `proposal.toml:211` and four more like it, a
//! later pass inserted fourteen lines into that file, and those citations now
//! name different rows while still looking exactly like citations. **The reader
//! lands on a real row**, which is what makes this worse than a citation that
//! fails to resolve.
//!
//! A row has a slug and the slug survives every insertion. That is the whole
//! repair: name the row rather than the line it happens to sit on.
//!
//! **Two populations.** A living ledger is edited by definition, so a line
//! citation in one is repairable and is refused outright. A numbered member file
//! is the record, and repointing its citations would be editing history to make
//! a checker green, so that half is a ratchet against a measured count and the
//! repair is upstream of the file: a seat briefed to name slugs writes none of
//! these, and the number stops growing because nobody adds to it rather than
//! because anybody cleaned it.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{
    finding, line_citations, markdown, panel_dir, shown, LIVING_LEDGERS, REGISTRY_FILES,
};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoLineCitationIntoARegistryFile)
}

const NAME: &str = "no-line-citation-into-a-registry-file";

/// The standing count in files that are the record, measured over the committed
/// tree.
///
/// Every one is unrepairable by construction, so this falls only when a file is
/// superseded. **Do not raise it.** A citation above it was written after the
/// check landed, and the fix is to name that row's slug.
const CEILING: usize = 45;

struct NoLineCitationIntoARegistryFile;
impl Lint for NoLineCitationIntoARegistryFile {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoLineCitationIntoARegistryFile {
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
                    "cites `{cited}`. A registry file gains rows constantly and every \
                     insertion shifts the lines under it, so this names a different row from \
                     the one it meant and still reads as a citation. This file is edited, so \
                     name the row's slug, which survives every insertion."
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
                "{} line citations into a registry file sit in files that are the record, \
                 against a ceiling of {ceiling}. Repointing one would be editing history to \
                 make a checker green, so what this refuses is a new one. Brief the next seat \
                 to write slugs and repoint the citation that was just added; do not raise \
                 the number. In: {per_file:?}",
                in_the_record.len()
            ),
        ));
    }
    out
}

/// Every `<registry file>.toml:<line>` the panel's prose carries.
fn citations(dir: &Path) -> Vec<(String, usize, String)> {
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            for cited in line_citations(line, ".toml", REGISTRY_FILES) {
                out.push((at.clone(), n + 1, cited));
            }
        }
    }
    out
}

/// Whether a file is one a reader treats as current, and may therefore edit.
///
/// **Classified by what the file is, not by what it is called.** An earlier
/// version asked whether the basename began with digits, and a probe's own
/// `FINDINGS.md` two directories down inside a landed probe directory then
/// counted as editable, where it is exactly as much the record as the member
/// file that produced it. The only editable files are the ledgers and they sit
/// at the panel root.
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
    fn a_ledger_citing_a_registry_line_is_refused_and_a_slug_is_not() {
        let f = findings(
            "registry-line-ledger",
            &[(
                "RULES.md",
                "See `proposal.toml:211` and `proposal-the-later-topics.toml:454`.\n\
                 The row is `proposal::composition_contracts_above_the_numeral`.\n\
                 Built from `Cargo.toml:17` and `mockspace.toml:311`.\n",
            )],
            usize::MAX,
        );
        assert_eq!(
            f.len(),
            2,
            "both registry citations are reported, the slug is not, and neither a manifest \
             nor the project config is a registry file: {f:?}"
        );
        assert!(f.iter().any(|m| m.contains("proposal.toml:211")), "{f:?}");
        assert!(
            f.iter()
                .any(|m| m.contains("proposal-the-later-topics.toml:454")),
            "a registry file filed by subject carries a hyphenated name and is still a \
             registry file: {f:?}"
        );
    }

    #[test]
    fn control_naming_a_registry_file_without_a_line_is_left_alone() {
        // Without this the lint is a ban on mentioning the registry at all, and
        // every arm above still passes.
        assert!(findings(
            "registry-line-noline",
            &[(
                "RULES.md",
                "Every row lives in `proposal.toml` or `ruling.toml`, and neither is \
                     cited by line.\n",
            )],
            usize::MAX,
        )
        .is_empty());
    }

    #[test]
    fn a_member_file_is_counted_rather_than_refused_and_the_ratchet_fires_above_the_ceiling() {
        let files = [("42_member.md", "See `proposal.toml:211`.\n")];
        assert!(
            findings("registry-line-under", &files, 1).is_empty(),
            "one citation under a ceiling of one is the grandfathered population"
        );
        let f = findings("registry-line-over", &files, 0);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ceiling of 0"), "{}", f[0]);
        assert!(f[0].contains("42_member.md"), "{}", f[0]);
    }

    #[test]
    fn a_probes_own_findings_file_is_the_record_rather_than_a_ledger() {
        // The classification defect this arm exists for. Asking whether the
        // basename begins with digits puts a probe's own `FINDINGS.md`, two
        // directories down inside a landed probe directory, on the repairable
        // side, where it is exactly as much the record as the member file that
        // produced it.
        let files = [("214_probes/run/FINDINGS.md", "See `ruling.toml:9`.\n")];
        assert!(
            findings("registry-line-probe", &files, 1).is_empty(),
            "a file inside a landed probe directory is not a ledger"
        );
        assert_eq!(findings("registry-line-probe2", &files, 0).len(), 1);
    }

    #[test]
    fn a_ledger_citation_is_refused_whatever_the_ceiling_says() {
        let f = findings(
            "registry-line-not-grandfathered",
            &[("OPTIONS.md", "See `ruling.toml:9`.\n")],
            usize::MAX,
        );
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn a_panel_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("registry-line-absent");
        assert!(super::check(&dir.join("nothing"), 0).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("registry-line-severity");
        plant(
            &dir,
            &format!("mock/{PANEL}/RULES.md"),
            "See `proposal.toml:211`.\n",
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(
            &super::NoLineCitationIntoARegistryFile,
            &ctx_at(&dir.join("mock"), &empty),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoLineCitationIntoARegistryFile);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::NoLineCitationIntoARegistryFile.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
