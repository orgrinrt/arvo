//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every tool lockfile pins the same mockspace revision, or the lint pack does
//! not compile and nobody can run the suite.
//!
//! Each tool under `mock/tools/` carries its own tracked `Cargo.lock`, and each
//! one pins `mockspace-lint-rules` by git revision. The generated lint crate
//! pins its own. When those disagree, two `LintPack` types land in one
//! dependency graph, `lint_pack!` expands against the one it did not come from,
//! and the build fails with a type mismatch between two structs of the same
//! name from two checkouts of the same repository.
//!
//! It has happened, and it was invisible until somebody ran the whole suite.
//! Seven lockfiles pinned three different revisions: five at one, one at
//! another, one at a third, the third being the revision the generated crate
//! actually used. `cargo mock test` reported two of ten trees failing, and
//! nothing in the ordinary commit path said anything, because the lint pass
//! runs against an already-built pack and a lockfile edit looks like noise in a
//! diff.
//!
//! **What this asserts is agreement, not a particular revision.** Naming the
//! revision here would be a second copy of something the lockfiles already say,
//! and it would need editing on every legitimate bump. What cannot be right is
//! two of them disagreeing, so that is what refuses.
//!
//! **The generated crate is read as a participant rather than assumed to
//! follow.** Comparing the tools only against each other passes the case where
//! all of them agree and all of them are stale, which is the likelier one:
//! `mockspace_branch = "dev"` has the launcher re-resolve the head from time to
//! time, and nothing moves a tool lockfile when it does. Its manifest lives
//! under `target/`, so it is absent in a clone nobody has built in, and the
//! check falls back to the pairwise comparison there.
//!
//! **A lint rather than a tool.** There is no state of this repository in which
//! two tool lockfiles may pin different mockspace revisions, so this refuses
//! rather than reports.
use std::collections::BTreeMap;
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(TheToolLocksPinOneMockspace)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "the-tool-locks-pin-one-mockspace";

/// The package whose revision has to agree across every lockfile.
const PACKAGE: &str = "mockspace-lint-rules";

struct TheToolLocksPinOneMockspace;
impl Lint for TheToolLocksPinOneMockspace {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for TheToolLocksPinOneMockspace {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(&ctx.mock_dir)
    }
}

/// The revision one lockfile pins, if it pins this package at all.
///
/// A `source` line for a git dependency ends in `#<40 hex>`, and the package it
/// belongs to is named two lines above it. Reading the pair rather than
/// grepping the whole file is what keeps a revision belonging to some other git
/// dependency out of the answer.
fn revision_in(lock: &str) -> Option<String> {
    let mut named = false;
    for line in lock.lines() {
        let line = line.trim();
        if line.starts_with("name = ") {
            named = line == format!("name = \"{PACKAGE}\"");
        } else if named && line.starts_with("source = ") {
            return line.rsplit_once('#').map(|(_, rev)| {
                rev.trim_end_matches('"')
                    .chars()
                    .take(7)
                    .collect::<String>()
            });
        }
    }
    None
}

/// The revision the generated lint crate is built against, if its manifest is
/// on disk.
///
/// The launcher writes `target/mockspace-lints/Cargo.toml` with the engine
/// pinned by `rev`, so this is the participant the tools have to agree with
/// rather than a fourth opinion. Read from the manifest and not from the
/// lockfile beside it, because a `[patch]` there resolves the package to a
/// checkout path and leaves it with no `source` line at all.
///
/// **Returns `None` where the manifest is absent, and that is the ordinary
/// state of a fresh clone**, since `target/` is not tracked. The check falls
/// back to comparing the tools against each other, which is what it did before
/// this arm existed, so a clone nobody has built in is not held in violation of
/// something it cannot yet know.
fn engine_revision(mock_dir: &Path) -> Option<String> {
    let manifest =
        std::fs::read_to_string(mock_dir.join("target/mockspace-lints/Cargo.toml")).ok()?;
    manifest
        .lines()
        .find(|line| line.contains(&format!("package = \"{PACKAGE}\"")))
        .and_then(|line| line.split_once("rev = \""))
        .map(|(_, rest)| rest.chars().take(7).collect())
}

/// The verdict, over a mock directory.
///
/// Split from the trait impl so a test can point it at a tree it built, and so
/// the one place that decides is one function.
///
/// **The generated lint crate is one of the participants**, which is what makes
/// this more than a pairwise check between tools. Seven tools agreeing with
/// each other and disagreeing with the engine is the same build failure as two
/// tools disagreeing, and it is the likelier shape: `mockspace_branch = "dev"`
/// means the launcher re-resolves the head periodically, and when it moves no
/// tool lockfile moves with it, so they stay uniform and stale together.
fn check(mock_dir: &Path) -> Vec<LintError> {
    let mut by_revision: BTreeMap<String, Vec<String>> = BTreeMap::new();
    if let Some(rev) = engine_revision(mock_dir) {
        by_revision
            .entry(rev)
            .or_default()
            .push("the generated lint crate".to_string());
    }

    let Ok(entries) = std::fs::read_dir(mock_dir.join("tools")) else {
        return verdict(by_revision);
    };

    for entry in entries.flatten() {
        let lock = entry.path().join("Cargo.lock");
        let Ok(text) = std::fs::read_to_string(&lock) else {
            continue;
        };
        let Some(rev) = revision_in(&text) else {
            continue;
        };
        let tool = entry.file_name().to_string_lossy().to_string();
        by_revision.entry(rev).or_default().push(tool);
    }

    verdict(by_revision)
}

/// One finding where the participants disagree, nothing where they do not.
///
/// Separate from the gathering so the early return above lands here too: a mock
/// directory with no `tools/` still has an engine revision worth reporting
/// against, and returning an empty vector there would have skipped it.
fn verdict(by_revision: BTreeMap<String, Vec<String>>) -> Vec<LintError> {
    if by_revision.len() < 2 {
        return Vec::new();
    }

    let spread = by_revision
        .iter()
        .map(|(rev, tools)| {
            let mut tools = tools.clone();
            tools.sort();
            format!("{rev}: {}", tools.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ");

    vec![finding(
        NAME,
        Some("the-tool-locks-disagree"),
        format!(
            "{} different revisions of `{PACKAGE}` are pinned across the tools and the generated \
             lint crate, and two of its `LintPack` types in one dependency graph is a build \
             failure rather than a warning. {spread}. Run `cargo update -p {PACKAGE}` in each tool \
             directory that lags; where the generated crate is the one standing alone, it is the \
             engine pin that moved and every tool lags it.",
            by_revision.len()
        ),
    )]
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };

    /// A lockfile body pinning `PACKAGE` at one revision.
    fn lock_at(rev: &str) -> String {
        format!(
            "version = 4\n\n\
             [[package]]\n\
             name = \"serde\"\n\
             version = \"1.0.0\"\n\
             source = \"registry+https://github.com/rust-lang/crates.io-index\"\n\n\
             [[package]]\n\
             name = \"{PACKAGE}\"\n\
             version = \"0.1.0\"\n\
             source = \"git+ssh://git@github.com/hiisi-digital/mockspace.git?branch=dev#{rev}\"\n"
        )
    }

    const A: &str = "b4e0c7ad219a24051952c4abd797a8a68b2e7908";
    const B: &str = "cfa89a497f0e91accf7817de8f7dc05ca8a4ebdc";
    const C: &str = "bd879613170fdf9d70aed1c1b22b168860f98548";

    /// A mock directory carrying a `tools/` built from tool name and lockfile.
    ///
    /// `planted_tree` is the testkit's, keyed on this name plus the process and
    /// thread, so two arms cannot plant into each other's tree. The returned
    /// path is a mock directory rather than the tools directory itself, because
    /// that is what `check_repo` is handed and what `ctx_at` wants.
    fn planted_mock(what: &str, tools: &[(&str, Option<String>)]) -> PathBuf {
        let dir = planted_tree(what);
        std::fs::create_dir_all(dir.join("tools")).expect("a planted tools directory");
        for (name, body) in tools {
            let tool = dir.join("tools").join(name);
            std::fs::create_dir_all(&tool).expect("a planted tool");
            if let Some(body) = body {
                plant(&dir, &format!("tools/{name}/Cargo.lock"), body);
            }
        }
        dir
    }

    /// A planted mock directory carrying a generated lint crate pinned at one
    /// revision, so an arm can make the engine a participant.
    ///
    /// The manifest line is the shape the launcher writes, package name and
    /// `rev` on one line, and nothing else in the file matters to the reader.
    fn plant_engine(dir: &Path, rev: &str) {
        plant(
            dir,
            "target/mockspace-lints/Cargo.toml",
            &format!(
                "[package]\nname = \"mockspace-lints\"\n\n\
                 [dependencies]\n\
                 mockspace = {{ package = \"{PACKAGE}\", git = \
                 \"ssh://git@github.com/hiisi-digital/mockspace.git\", rev = \"{rev}\" }}\n"
            ),
        );
    }

    #[test]
    fn every_lock_on_one_revision_passes() {
        let d = planted_mock(
            "locks-agree",
            &[
                ("alpha", Some(lock_at(A))),
                ("beta", Some(lock_at(A))),
                ("gamma", Some(lock_at(A))),
            ],
        );
        assert!(
            check(&d).is_empty(),
            "three lockfiles agreeing is the state this lint exists to permit"
        );
    }

    #[test]
    fn two_revisions_are_caught() {
        let d = planted_mock(
            "locks-two",
            &[("alpha", Some(lock_at(A))), ("beta", Some(lock_at(B)))],
        );
        let f = check(&d);
        assert_eq!(f.len(), 1, "one finding for the whole disagreement: {f:?}");
        let m = &f[0].message;
        assert!(
            m.contains("2 different revisions"),
            "the count is named: {m}"
        );
        assert!(m.contains("alpha"), "the lagging tool is named: {m}");
        assert!(m.contains("beta"), "the other tool is named: {m}");
    }

    /// The shape actually found in the tree: three revisions, unevenly spread.
    #[test]
    fn three_revisions_name_every_tool_on_every_one() {
        let d = planted_mock(
            "locks-three",
            &[
                ("alpha", Some(lock_at(A))),
                ("beta", Some(lock_at(A))),
                ("gamma", Some(lock_at(C))),
                ("delta", Some(lock_at(B))),
            ],
        );
        let f = check(&d);
        assert_eq!(f.len(), 1, "still one finding: {f:?}");
        let m = &f[0].message;
        assert!(m.contains("3 different revisions"), "{m}");
        for tool in ["alpha", "beta", "gamma", "delta"] {
            assert!(m.contains(tool), "`{tool}` is not named: {m}");
        }
    }

    /// Every tool agreeing with every other and disagreeing with the engine.
    ///
    /// **The arm the pairwise check could not have.** Three lockfiles on one
    /// revision is exactly the shape `every_lock_on_one_revision_passes` plants
    /// and passes; what makes this a violation is the fourth participant, so an
    /// implementation that reads only `tools/` returns empty here and fails.
    #[test]
    fn tools_agreeing_with_each_other_and_not_with_the_engine_are_caught() {
        let d = planted_mock(
            "locks-engine-disagrees",
            &[
                ("alpha", Some(lock_at(A))),
                ("beta", Some(lock_at(A))),
                ("gamma", Some(lock_at(A))),
            ],
        );
        plant_engine(&d, B);
        let f = check(&d);
        assert_eq!(
            f.len(),
            1,
            "seven tools uniformly stale against a moved engine pin is the same build failure as \
             two tools disagreeing: {f:?}"
        );
        let message = format!("{:?}", f[0]);
        assert!(
            message.contains("the generated lint crate"),
            "the finding has to name the participant standing alone, or nobody knows which side \
             to move: {message}"
        );
    }

    /// The same tree with the engine on the revision the tools carry.
    ///
    /// The negative half of the arm above: without it, a `check` that reported
    /// a finding whenever an engine manifest existed would pass that one.
    #[test]
    fn tools_agreeing_with_the_engine_pass() {
        let d = planted_mock(
            "locks-engine-agrees",
            &[("alpha", Some(lock_at(A))), ("beta", Some(lock_at(A)))],
        );
        plant_engine(&d, A);
        assert!(
            check(&d).is_empty(),
            "agreement across every participant is the state this lint exists to protect"
        );
    }

    /// A tree with no generated crate on disk, which is every fresh clone.
    ///
    /// `target/` is not tracked, so the manifest is absent until somebody
    /// builds. The engine arm has to be skipped there rather than counted as a
    /// participant pinning nothing.
    #[test]
    fn a_missing_generated_crate_falls_back_to_the_pairwise_check() {
        let agree = planted_mock(
            "locks-no-engine-agree",
            &[("alpha", Some(lock_at(A))), ("beta", Some(lock_at(A)))],
        );
        assert!(
            check(&agree).is_empty(),
            "a clone nobody has built in cannot be in violation of a pin it has not resolved"
        );

        let disagree = planted_mock(
            "locks-no-engine-disagree",
            &[("alpha", Some(lock_at(A))), ("beta", Some(lock_at(B)))],
        );
        assert_eq!(
            check(&disagree).len(),
            1,
            "the pairwise check still has to refuse when the engine is the part that is missing"
        );
    }

    /// The engine's revision read off a manifest and nothing else.
    ///
    /// The lockfile beside it carries the package with no `source` line at all,
    /// because a `[patch]` resolves it to a checkout path, so a reader built on
    /// `revision_in` would return `None` and the whole arm would go quiet.
    #[test]
    fn the_engine_revision_comes_from_the_manifest_not_the_lockfile() {
        let d = planted_tree("locks-engine-reader");
        assert_eq!(
            engine_revision(&d),
            None,
            "no manifest is no answer, not a wrong one"
        );
        plant_engine(&d, C);
        assert_eq!(
            engine_revision(&d),
            Some(C.chars().take(7).collect::<String>()),
            "the short form has to match what `revision_in` produces, or the two never compare \
             equal and the arm refuses every tree"
        );
    }

    #[test]
    fn a_single_tool_cannot_disagree_with_itself() {
        let d = planted_mock("locks-single", &[("alpha", Some(lock_at(A)))]);
        assert!(check(&d).is_empty(), "one lockfile is agreement");
    }

    #[test]
    fn no_tools_at_all_passes() {
        let d = planted_mock("locks-none", &[]);
        assert!(
            check(&d).is_empty(),
            "an empty tools directory pins nothing"
        );
    }

    #[test]
    fn a_missing_tools_directory_passes() {
        let d = planted_tree("locks-absent");
        assert!(
            check(&d.join("nothing-here")).is_empty(),
            "a repository with no tools directory is not in violation of this"
        );
    }

    #[test]
    fn a_tool_with_no_lockfile_is_skipped_rather_than_counted() {
        let d = planted_mock(
            "locks-unpinned",
            &[
                ("alpha", Some(lock_at(A))),
                ("beta", None),
                ("gamma", Some(lock_at(A))),
            ],
        );
        assert!(
            check(&d).is_empty(),
            "a tool that pins nothing cannot disagree with one that does"
        );
    }

    #[test]
    fn a_lockfile_not_naming_the_package_is_skipped() {
        let unrelated = "version = 4\n\n\
                         [[package]]\n\
                         name = \"serde\"\n\
                         version = \"1.0.0\"\n\
                         source = \"registry+https://github.com/rust-lang/crates.io-index\"\n";
        let d = planted_mock(
            "locks-unrelated",
            &[
                ("alpha", Some(lock_at(A))),
                ("beta", Some(unrelated.to_string())),
            ],
        );
        assert!(
            check(&d).is_empty(),
            "a lockfile without the package contributes no revision"
        );
    }

    /// A second git dependency's revision must not be read as this one's.
    #[test]
    fn another_git_dependency_revision_is_not_mistaken_for_this_one() {
        let two_deps = format!(
            "version = 4\n\n\
             [[package]]\n\
             name = \"some-other-git-dep\"\n\
             version = \"0.1.0\"\n\
             source = \"git+ssh://git@github.com/orgrinrt/notko.git?branch=dev#{C}\"\n\n\
             [[package]]\n\
             name = \"{PACKAGE}\"\n\
             version = \"0.1.0\"\n\
             source = \"git+ssh://git@github.com/hiisi-digital/mockspace.git?branch=dev#{A}\"\n"
        );
        let d = planted_mock(
            "locks-other-dep",
            &[("alpha", Some(two_deps)), ("beta", Some(lock_at(A)))],
        );
        assert!(
            check(&d).is_empty(),
            "the other dependency's revision was read as this package's"
        );
    }

    #[test]
    fn the_revision_reader_takes_the_short_form_of_the_right_package() {
        assert_eq!(revision_in(&lock_at(A)).as_deref(), Some("b4e0c7a"));
        assert_eq!(revision_in(&lock_at(B)).as_deref(), Some("cfa89a4"));
        assert_eq!(revision_in("version = 4\n"), None);
    }

    /// The wiring, which every arm above is blind to.
    ///
    /// `check` is a free function and each arm calls it directly, so all of
    /// them would pass on a lint whose `check_repo` reads the wrong directory,
    /// or on one the engine never runs at all. These three ask the questions
    /// the predicate cannot.
    #[test]
    fn it_reads_the_tools_directory_under_the_mock_directory_it_is_handed() {
        let d = planted_mock(
            "locks-wiring",
            &[("alpha", Some(lock_at(A))), ("beta", Some(lock_at(B)))],
        );
        let empty = view(&[], &[]);
        let found = TheToolLocksPinOneMockspace.check_repo(&ctx_at(&d, &empty));
        assert_eq!(
            found.len(),
            1,
            "the lint found nothing through a context, so it is reading somewhere \
             other than `mock_dir/tools`: {found:?}"
        );
    }

    #[test]
    fn its_findings_block_every_gate() {
        // A finding here that did not block would leave the pack green over a
        // dependency graph that cannot compile, which is the state this lint
        // exists to refuse and the one it was written after.
        let d = planted_mock(
            "locks-severity",
            &[("alpha", Some(lock_at(A))), ("beta", Some(lock_at(B)))],
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(&TheToolLocksPinOneMockspace, &ctx_at(&d, &empty));
    }

    #[test]
    fn it_is_not_declared_off_and_it_reaches_the_pack() {
        assert_not_declared_off(&TheToolLocksPinOneMockspace);
        assert_registered(NAME);
    }
}
