//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A canon that loaded nothing reads exactly like a canon with nothing wrong.
//!
//! Every other canon lint here sweeps the registry and reports what it finds.
//! **A registry that failed to load hands each of them an empty population, and
//! an empty population produces an empty report**, which the gate renders as a
//! pass. So one broken loader turns the whole set green at once, silently, and
//! in the flattering direction.
//!
//! It has happened. The crate these lints came from returned an empty registry
//! for a path that was not a directory, and an expert measured that 21 of its
//! 30 arms said nothing on an empty input and nothing on the real one. The fix
//! there was to refuse an empty load rather than return one. This is that fix
//! carried onto the other side of the migration, because the defect does not
//! belong to a crate: it belongs to the shape of a check that reports what it
//! finds.
//!
//! **Two states, and the second is the one worth having.** A repository that
//! declares a canon and whose view holds no rows is the first. A repository
//! whose registry directory holds TOML files while the view is still empty is
//! the second, and that is a load failure rather than an absence: the rows are
//! on disk and the check that was going to read them cannot see them.
//!
//! **A lint rather than a tool.** There is no state of this repository in which
//! an empty canon is correct, and nothing downstream of the gate can act on a
//! report saying so. It has to refuse, and it has to refuse before the rest of
//! the pack reports clean over nothing.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ACanonThatLoadsNothing)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "a-canon-that-loads-nothing-is-not-a-clean-canon";

/// Where the rows sit on disk, relative to the mock directory.
///
/// The engine's own loader reads `<mock>/registry`, so this is that path and
/// not a second convention. Read here to tell an absent canon from one that did
/// not load, which is a distinction the view alone cannot make.
const REGISTRY: &str = "registry";

struct ACanonThatLoadsNothing;
impl Lint for ACanonThatLoadsNothing {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ACanonThatLoadsNothing {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(
            !ctx.registry.is_empty(),
            ctx.canon_paths,
            &files_on_disk(&ctx.mock_dir.join(REGISTRY)),
        )
    }
}

/// The verdict, over the three facts it rests on.
///
/// Split out so a test can drive every combination without a tree, and so the
/// tree-reading half is one function with one job.
fn check(view_has_rows: bool, canon_paths: &[String], on_disk: &[String]) -> Vec<LintError> {
    if view_has_rows {
        return Vec::new();
    }
    if !on_disk.is_empty() {
        let mut named: Vec<&str> = on_disk.iter().map(String::as_str).collect();
        named.sort_unstable();
        return vec![finding(
            NAME,
            Some("the-registry-did-not-load"),
            format!(
                "`mock/{REGISTRY}` holds {} TOML file(s) and the registry the lints are \
                 handed is empty: {named:?}. Every canon lint sweeps that registry and \
                 reports what it finds, so an empty one turns all of them green at once and \
                 the gate renders it as a pass. This is a defect in the load rather than in \
                 the canon.",
                on_disk.len()
            ),
        )];
    }
    if !canon_paths.is_empty() {
        return vec![finding(
            NAME,
            Some("the-canon-is-empty"),
            format!(
                "`canon_paths` declares {canon_paths:?} and no row was loaded from any of \
                 them. Every canon lint is about to report clean over nothing, which is what \
                 a canon with nothing wrong in it also looks like."
            ),
        )];
    }
    // No canon declared and nothing on disk. A project without a registry is a
    // legitimate state and this lint has nothing to say about it, which is what
    // lets the same pack ship to a repository that has not written one.
    Vec::new()
}

/// The names of the TOML files under the registry directory, walked.
///
/// Names rather than paths, because the finding is read by somebody looking at
/// the directory and the absolute path of a worktree tells them nothing. An
/// unreadable directory answers empty, which is the same answer an absent one
/// gives: both mean this half cannot distinguish anything and the declaration
/// half decides.
fn files_on_disk(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(files_on_disk(&path));
        } else if path.extension().is_some_and(|e| e == "toml") {
            out.push(
                path.file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            );
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };

    fn kinds(found: &[mockspace::LintError]) -> Vec<Option<&'static str>> {
        found.iter().map(|e| e.finding_kind).collect()
    }

    #[test]
    fn a_registry_holding_rows_is_silent() {
        let declared = ["mock/registry/*.toml".to_string()];
        assert!(super::check(true, &declared, &["ruling.toml".to_string()]).is_empty());
    }

    #[test]
    fn files_on_disk_and_an_empty_view_is_a_load_failure_and_says_so() {
        // The state that turns the whole pack green. The rows are there, the
        // view is empty, and every other lint is about to report nothing.
        let declared = ["mock/registry/*.toml".to_string()];
        let found = super::check(false, &declared, &["ruling.toml".to_string()]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(kinds(&found), [Some("the-registry-did-not-load")]);
        assert!(
            found[0].message.contains("ruling.toml"),
            "{}",
            found[0].message
        );
    }

    #[test]
    fn a_declared_canon_with_nothing_anywhere_is_a_different_finding() {
        // Not the same defect and not the same fix, so not the same kind. This
        // one says nobody has written the canon yet, or the declaration points
        // somewhere nothing lives.
        let declared = ["mock/registry/*.toml".to_string()];
        let found = super::check(false, &declared, &[]);
        assert_eq!(kinds(&found), [Some("the-canon-is-empty")], "{found:?}");
    }

    #[test]
    fn control_a_project_declaring_no_canon_is_left_alone() {
        // The control that keeps this from being a lint refusing every
        // repository that has no registry. Without it the pack could not ship
        // anywhere else, and every arm above would still pass.
        assert!(super::check(false, &[], &[]).is_empty());
    }

    #[test]
    fn control_a_declared_canon_that_loaded_is_silent_whatever_is_on_disk() {
        // The other direction: rows loaded is the whole test, and the disk walk
        // must not turn a working registry into a finding.
        let declared = ["mock/registry/*.toml".to_string()];
        assert!(super::check(true, &declared, &[]).is_empty());
    }

    #[test]
    fn the_disk_walk_finds_a_nested_file_and_reports_a_missing_directory_as_empty() {
        let dir = planted_tree("canon-load");
        assert!(
            super::files_on_disk(&dir.join("registry")).is_empty(),
            "a directory that is not there is not a load failure"
        );
        plant(&dir, "registry/ruling.toml", "[[ruling]]\nid = \"a\"\n");
        plant(&dir, "registry/nested/law.toml", "[[law]]\nid = \"b\"\n");
        plant(&dir, "registry/notes.md", "not toml\n");
        let mut found = super::files_on_disk(&dir.join("registry"));
        found.sort();
        assert_eq!(
            found,
            ["law.toml", "ruling.toml"],
            "nested files count and a non-TOML file does not"
        );
    }

    #[test]
    fn it_reads_the_registry_directory_under_the_mock_directory_it_is_handed() {
        // The wiring, which the split-out predicate cannot cover: a lint that
        // computed the right verdict from the wrong directory would pass every
        // arm above.
        let dir = planted_tree("canon-load-wiring");
        plant(&dir, "registry/ruling.toml", "[[ruling]]\nid = \"a\"\n");
        let empty = view(&[], &[]);
        let found = super::ACanonThatLoadsNothing.check_repo(&ctx_at(&dir, &empty));
        assert_eq!(
            kinds(&found),
            [Some("the-registry-did-not-load")],
            "{found:?}"
        );
    }

    #[test]
    fn its_findings_block_every_gate() {
        // A finding here that did not block would leave the pack reporting
        // clean over nothing with a warning nobody reads beside it, which is
        // the state this lint exists to refuse.
        let dir = planted_tree("canon-load-severity");
        plant(&dir, "registry/ruling.toml", "[[ruling]]\nid = \"a\"\n");
        let empty = view(&[], &[]);
        assert_findings_block_at(&super::ACanonThatLoadsNothing, &ctx_at(&dir, &empty));
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ACanonThatLoadsNothing);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::ACanonThatLoadsNothing.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
