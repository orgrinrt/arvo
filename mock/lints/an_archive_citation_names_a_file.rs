//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An `OLD_` citation naming a file the archive does not hold.
//!
//! The other half of a sentence in `RULES.md` that asserted a totality over the
//! whole tree and had checked five files. One half is that nothing still cites
//! an archive by its dead spelling; this is the half that would fail if an
//! archive were tidied again.
//!
//! **Walked over the whole panel rather than over the ledgers**, because an
//! `OLD_` citation is written deliberately. Somebody repointed it on purpose, so
//! a dangling one is a rename nobody followed rather than a leftover, and it is
//! wrong wherever it sits. That is also why this half carries no ceiling: unlike
//! the dead-spelling residue, there is no population here that was correct when
//! it was written.
//!
//! **A lint rather than a tool.** A citation resolving to nothing is refused,
//! and the repair is one path.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{archive_citations, finding, markdown, panel_dir, shown, ARCHIVES};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AnArchiveCitationNamesAFile)
}

const NAME: &str = "an-archive-citation-names-a-file";

struct AnArchiveCitationNamesAFile;
impl Lint for AnArchiveCitationNamesAFile {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AnArchiveCitationNamesAFile {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(&panel_dir(ctx.mock_dir))
    }
}

fn check(dir: &Path) -> Vec<LintError> {
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            for archive in ARCHIVES {
                for cited in archive_citations(line, archive.prefix) {
                    let file = cited.trim_start_matches(archive.prefix);
                    if !file.starts_with("OLD_") {
                        continue; // the dead-spelling lint owns those
                    }
                    if dir.join(archive.at).join(file).exists() {
                        continue;
                    }
                    out.push(finding(
                        NAME,
                        &at,
                        n + 1,
                        format!(
                            "cites `{cited}`, and the {} archive holds no such file. An \
                             `OLD_` citation is written deliberately, so a dangling one is a \
                             rename nobody followed rather than a typo.",
                            archive.name
                        ),
                    ));
                }
            }
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };
    use crate::panel_corpus::PANEL;

    /// A panel tree one directory down, so the closed archive's `..` address
    /// lands inside the fixture rather than beside it.
    fn findings(what: &str, files: &[(&str, &str)]) -> Vec<String> {
        let panel = planted_tree(what).join("panel");
        std::fs::create_dir_all(&panel).expect("a planted panel");
        for (at, text) in files {
            plant(&panel, at, text);
        }
        super::check(&panel).into_iter().map(|e| e.message).collect()
    }

    /// One file present in each archive, so the arm has a case that must not
    /// fire beside every case that must.
    const PRESENT: [(&str, &str); 2] = [
        ("seed/OLD_SETTLED_laws.md", "# laws\n"),
        (
            "../202607301300_formalization-spec-panel/OLD_143b_op.md",
            "# ruling\n",
        ),
    ];

    #[test]
    fn a_dangling_prefixed_citation_is_reported_and_a_present_one_is_not() {
        let mut files = PRESENT.to_vec();
        files.push((
            "42_member.md",
            "Present: `seed/OLD_SETTLED_laws.md`. Absent: `seed/OLD_SETTLED_container.md`.\n",
        ));
        let f = findings("dangling-one", &files);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("OLD_SETTLED_container.md"), "{}", f[0]);
    }

    #[test]
    fn both_archives_are_covered_by_the_same_arm() {
        // A fixture planting only into `seed/` passes for as long as the second
        // archive is invisible, and keeps passing.
        let mut files = PRESENT.to_vec();
        files.push((
            "42_member.md",
            "Present: `seed/OLD_SETTLED_laws.md`. Absent: \
             `mock/research/202607301300_formalization-spec-panel/OLD_999_nothing.md`.\n",
        ));
        let f = findings("dangling-both", &files);
        assert_eq!(
            f.len(),
            1,
            "the absent closed-panel file dangles and the present seed file does not: {f:?}"
        );
        assert!(f[0].contains("OLD_999_nothing.md"), "{}", f[0]);
        assert!(f[0].contains("closed formalization panel"), "{}", f[0]);
    }

    #[test]
    fn control_an_unprefixed_citation_belongs_to_the_other_lint() {
        // The discrimination. Reporting both here would report every dead
        // spelling twice under two names, and the two have different repairs
        // and different populations.
        let mut files = PRESENT.to_vec();
        files.push(("42_member.md", "At `seed/SETTLED_nothing_here.md`.\n"));
        assert!(findings("dangling-unprefixed", &files).is_empty());
    }

    #[test]
    fn control_naming_the_archive_directory_is_not_a_citation() {
        let mut files = PRESENT.to_vec();
        files.push((
            "42_member.md",
            "The panel at `mock/research/202607301300_formalization-spec-panel/` holds it.\n",
        ));
        assert!(
            findings("dangling-directory", &files).is_empty(),
            "the directory was never renamed, so naming it is a live reference"
        );
    }

    #[test]
    fn a_citation_in_a_ledger_is_reported_like_one_anywhere_else() {
        // No ceiling and no partition here, deliberately: an `OLD_` citation was
        // written on purpose, so there is no population that was correct when it
        // was written and nothing to grandfather.
        let mut files = PRESENT.to_vec();
        files.push(("RULES.md", "At `seed/OLD_SETTLED_gone.md`.\n"));
        assert_eq!(findings("dangling-ledger", &files).len(), 1);
    }

    #[test]
    fn a_panel_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("dangling-absent");
        assert!(super::check(&dir.join("nothing")).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("dangling-severity");
        plant(
            &dir,
            &format!("mock/{PANEL}/42_member.md"),
            "At `seed/OLD_SETTLED_gone.md`.\n",
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(
            &super::AnArchiveCitationNamesAFile,
            &ctx_at(&dir.join("mock"), &empty),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AnArchiveCitationNamesAFile);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::AnArchiveCitationNamesAFile.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
