//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An axis that was declared and is no longer there.
//!
//! `dimension.toml`'s own header says the set is append-only and that a check
//! pins it rather than the sentence asking. **It said that twice and no check
//! existed**, which is the shape a claim of totality takes when nobody names
//! the mechanism behind it.
//!
//! What is at stake is narrow and worth stating exactly. Adding an axis reaches
//! nobody: a predicate's absence quantifies over the world rather than over
//! this file, so a row written before an axis was declared was always as narrow
//! as it now reads. **Deleting or renaming one is what rewrites meaning**,
//! because a span written over that axis becomes unparseable and a written
//! absence becomes nonsense, and both fail silently: the corpus still reads,
//! and the sentence it now says is a different one.
//!
//! **The record is git, and only git.** Every id the namespace has carried in
//! `dimension.toml` across that file's history must still resolve to a row.
//! History is the one record a deletion cannot edit in the commit that deletes,
//! which is the whole reason it is the record.
//!
//! # What a roster cost, since one stood here and is gone
//!
//! A hand-written list of every id ever declared sat beside this, read in both
//! directions: every id on it had to resolve, and every row had to be on it.
//! **Together those two force the list to equal the row ids exactly**, so it
//! held no record the registry did not, and deleting a row together with its
//! entry passed every arm. It read as an independent historical record and was
//! a duplicate of the current state.
//!
//! It was walked state by state before being removed, and there is no state
//! where it earned its place. A one-sided deletion fires the history arm too. A
//! two-sided deletion is the case it was rebuilt for and the case it failed. An
//! unrostered new row protected nothing, because history protects an axis from
//! the commit that declares it whether or not anybody wrote it down. And it was
//! never a fallback for an unreadable history, because that already blocks.
//!
//! # A history that cannot be trusted is reported, never passed
//!
//! **A shallow clone is the case that makes this more than a formality.**
//! `git log` on a truncated history succeeds and returns fewer commits, so the
//! walk comes back with a narrower answer and nothing says it narrowed. The arm
//! that a single commit cannot defeat is then defeated by one commit plus a
//! clone depth: measured at 1 commit against 6, exit status zero, with the
//! two-sided deletion passing clean.
//!
//! So the repository is asked whether it is shallow before the walk is
//! believed, and **an answer that cannot be obtained counts as shallow**, since
//! a check that cannot establish it read the whole history must not claim it
//! did. Both routes land in the same report.
//!
//! # The premise is checked rather than assumed
//!
//! The walk reads one path, so a second file declaring the namespace would
//! leave its axes unprotected with no arm noticing. `law.toml` has already been
//! split exactly that way. The third arm reports it, and the repair is to widen
//! the scan rather than to keep the premise.
//!
//! **Widening it now would be the wrong trade.** Reading every path the
//! registry directory has ever held measured 7.3 seconds against 0.2 for the
//! one file, over 131 commits and 12 paths, and this runs at every commit. That
//! is a cost paid forever against a case that does not exist yet, so the arm
//! makes the premise fail loudly and the scan gets widened when something
//! actually needs it.
//!
//! # What the unit tests here cannot ask
//!
//! That the walk reads the real repository, because a planted temporary
//! directory is not a git checkout, and that the shipped registry satisfies it,
//! because building a `RegistryView` from `mock/registry/` needs a TOML parser
//! the generated pack has no route to depend on. So the predicate is a pure
//! function over its inputs and every arm drives it directly; reading git and
//! reading the registry directory are the thin parts left over, and the string
//! handling that decides shallowness is split out so it can be pinned.
//!
//! **Both were driven against the real repository, deliberately.** Deleting
//! `leaf_aliasing` from the registry produces this arm naming it. Doing that in
//! a `--depth 1` clone produced `all lints passed` before the shallow check
//! existed, which is the defect that check was added for.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(TheAxisSetIsAppendOnly)
}

const LINT: &str = "the-axis-set-is-append-only";

/// The namespace whose rows are the axes.
const NAMESPACE: &str = "dimension";

/// The registry file the axes are declared in, from the repository root.
const FILE: &str = "mock/registry/dimension.toml";

struct TheAxisSetIsAppendOnly;
impl Lint for TheAxisSetIsAppendOnly {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for TheAxisSetIsAppendOnly {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(
            &ids_of(ctx.registry.rows_in(NAMESPACE)),
            ids_ever_declared(ctx.repo_root).as_deref(),
            &files_declaring(&ctx.mock_dir.join("registry")),
        )
    }
}

/// The bare ids of qualified rows in this namespace, and nothing else.
///
/// Its own function so the discrimination stays testable: a row spelled
/// `ruling::signedness` is a different row from `dimension::signedness`, and a
/// filter written on the slug alone counts it as the axis and passes.
fn ids_of(rows: &[String]) -> Vec<String> {
    rows.iter()
        .filter_map(|q| q.strip_prefix(NAMESPACE)?.strip_prefix("::"))
        .map(str::to_string)
        .collect()
}

/// The whole predicate, over ids rather than qualified rows so a test can state
/// its fixtures the way the registry states them.
///
/// `ever` is `None` where the history could not be read or could not be
/// trusted, which is reported rather than passed.
fn check(live: &[String], ever: Option<&[String]>, declaring: &[String]) -> Vec<LintError> {
    let rows: Vec<&str> = live.iter().map(String::as_str).collect();
    let mut out = Vec::new();

    match ever {
        Some(ever) => {
            let lost: Vec<&str> = ever
                .iter()
                .map(String::as_str)
                .filter(|id| !rows.contains(id))
                .collect();
            if !lost.is_empty() {
                out.push(finding(
                    LINT,
                    None,
                    format!(
                        "{} axes were declared in this file's history and resolve to no row \
                         now: {lost:?}. A span written over an axis becomes unparseable when \
                         the axis goes and a written absence becomes nonsense, and both fail \
                         silently. Restore the row under its original id. A rename is a \
                         deletion and needs the old id kept, or every predicate written in it \
                         stops parsing.",
                        lost.len()
                    ),
                ));
            }
        }
        None => out.push(finding(
            LINT,
            None,
            format!(
                "the history of `{FILE}` could not be read, or could not be established as \
                 complete, so the only check on the axis set did not run. A shallow or \
                 partial clone reaches this: `git log` on a truncated history succeeds and \
                 returns fewer commits, so believing it would report a deleted axis as fine. \
                 Run this in a full checkout."
            ),
        )),
    }

    if declaring.len() > 1 || (declaring.len() == 1 && !FILE.ends_with(&declaring[0])) {
        out.push(finding(
            LINT,
            None,
            format!(
                "`{NAMESPACE}` rows are declared in {declaring:?} and the history walk reads \
                 only `{FILE}`, so an axis declared in another of them is unprotected. Widen \
                 the walk to every file that declares the namespace, and mind that reading \
                 every path the registry directory has ever held measured 7.3 seconds against \
                 0.2 for the one file, so the walk is the part to make cheaper rather than \
                 the premise to keep."
            ),
        ));
    }
    out
}

/// Every `id` the namespace has carried in `FILE` across that file's history.
///
/// `None` where git is unavailable, where the file has no history, or where the
/// repository is shallow and the walk would therefore be narrower than it
/// reads. Six commits touch this file in a full checkout, so the walk is one
/// `git log` and six `git show`, measured at 0.2 seconds.
fn ids_ever_declared(repo_root: &Path) -> Option<Vec<String>> {
    if is_shallow(git(repo_root, &["rev-parse", "--is-shallow-repository"]).as_deref()) {
        return None;
    }
    let commits = git(repo_root, &["log", "--follow", "--format=%H", "--", FILE])?;
    if commits.split_whitespace().next().is_none() {
        // No commit touches the file, so the walk established nothing rather
        // than establishing that nothing was lost. `Some(vec![])` would read as
        // the second and is the same false pass a shallow clone produces.
        return None;
    }
    let mut out: Vec<String> = Vec::new();
    for commit in commits.split_whitespace() {
        let Some(text) = git(repo_root, &["show", &format!("{commit}:{FILE}")]) else {
            continue;
        };
        out.extend(ids_in(&text));
    }
    out.sort();
    out.dedup();
    Some(out)
}

/// Whether the walk must be treated as truncated.
///
/// **An answer that could not be obtained counts as shallow.** A check unable
/// to establish that it read the whole history may not claim it did, and the
/// two routes are reported as one thing because they mean one thing.
fn is_shallow(answer: Option<&str>) -> bool {
    match answer {
        Some(text) => text.trim() != "false",
        None => true,
    }
}

/// Every id declared under this namespace in one file's text.
///
/// The table header decides the namespace, never the path, because that is what
/// the registry itself does: `law.toml` and `law-the-later-topics.toml` are one
/// namespace in two files.
fn ids_in(text: &str) -> Vec<String> {
    let mut here = String::new();
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if let Some(ns) = t.strip_prefix("[[").and_then(|r| r.strip_suffix("]]")) {
            here = ns.to_string();
        } else if here == NAMESPACE {
            if let Some(v) = t.strip_prefix("id = \"").and_then(|r| r.strip_suffix('"')) {
                out.push(v.to_string());
            }
        }
    }
    out
}

/// The registry files that declare this namespace, by filename.
fn files_declaring(registry: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(registry) else {
        return Vec::new();
    };
    let mut out: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|t| !ids_in(&t).is_empty()))
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .collect();
    out.sort();
    out
}

/// One git invocation, or `None` where it could not be run or did not succeed.
fn git(repo_root: &Path, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, view,
    };

    /// The one file the walk reads, as `files_declaring` reports it.
    const ONE_FILE: &[&str] = &["dimension.toml"];

    fn run(live: &[&str], ever: Option<&[&str]>, files: &[&str]) -> Vec<String> {
        let live: Vec<String> = live.iter().map(|s| (*s).to_string()).collect();
        let ever: Option<Vec<String>> = ever.map(|e| e.iter().map(|s| (*s).to_string()).collect());
        let files: Vec<String> = files.iter().map(|s| (*s).to_string()).collect();
        super::check(&live, ever.as_deref(), &files)
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn a_history_agreeing_with_the_rows_is_silent() {
        assert!(run(&["radix", "signedness"], Some(&["radix", "signedness"]), ONE_FILE).is_empty());
    }

    #[test]
    fn an_id_in_history_that_no_longer_resolves_is_named() {
        let f = run(&["radix"], Some(&["radix", "signedness"]), ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(
            !f[0].contains("radix"),
            "the surviving axis was named: {}",
            f[0]
        );
    }

    #[test]
    fn a_deletion_that_edits_every_file_in_the_tree_is_still_caught() {
        // The case a roster could not reach, and the reason history is the
        // record. Nothing in the working tree mentions the axis any more.
        let f = run(&["radix"], Some(&["radix", "leaf_aliasing"]), ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("leaf_aliasing"), "{}", f[0]);
    }

    #[test]
    fn every_lost_id_is_named_rather_than_the_first() {
        let f = run(&[], Some(&["radix", "signedness"]), ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("radix"), "{}", f[0]);
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(f[0].contains('2'), "the count is missing: {}", f[0]);
    }

    #[test]
    fn an_axis_declared_since_the_history_was_read_is_not_a_finding() {
        // Growth is free. A row present now and absent from the walk is an
        // addition, and the arm is one-directional on purpose.
        assert!(run(&["radix", "a_new_axis"], Some(&["radix"]), ONE_FILE).is_empty());
    }

    #[test]
    fn an_id_whose_name_merely_contains_a_lost_one_does_not_satisfy_it() {
        let f = run(
            &["radix", "signedness_of_the_accumulator"],
            Some(&["radix", "signedness"]),
            ONE_FILE,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("\"signedness\""), "{}", f[0]);
    }

    #[test]
    fn a_history_that_cannot_be_read_is_reported_rather_than_passed() {
        let f = run(&["radix", "signedness"], None, ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("could not be read"), "{}", f[0]);
        assert!(f[0].contains("shallow"), "{}", f[0]);
    }

    #[test]
    fn a_shallow_repository_is_treated_as_unreadable_and_an_unknown_answer_too() {
        // The blocker this lint shipped with once. `git log` on a truncated
        // history exits zero and returns fewer commits, so a narrow `Some`
        // reads exactly like a complete one. An answer nobody could obtain
        // counts as shallow for the same reason: a check that cannot establish
        // it read the whole history may not claim it did.
        assert!(super::is_shallow(Some("true\n")), "shallow was not detected");
        assert!(super::is_shallow(None), "an unobtainable answer was trusted");
        assert!(
            super::is_shallow(Some("something unexpected")),
            "an answer that is not `false` was trusted"
        );
        assert!(
            !super::is_shallow(Some("false\n")),
            "a full checkout was refused"
        );
    }

    #[test]
    fn a_second_file_declaring_the_namespace_is_reported() {
        // The premise the walk rests on, checked rather than assumed.
        // `law.toml` has already been split this way once.
        let f = run(
            &["radix"],
            Some(&["radix"]),
            &["dimension-the-later-axes.toml", "dimension.toml"],
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("dimension-the-later-axes.toml"), "{}", f[0]);
        assert!(f[0].contains("unprotected"), "{}", f[0]);
    }

    #[test]
    fn a_file_that_is_not_the_walked_one_is_reported_even_when_it_is_the_only_one() {
        // One file is not enough; it has to be the file the walk reads.
        let f = run(&["radix"], Some(&["radix"]), &["axes.toml"]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("axes.toml"), "{}", f[0]);
    }

    #[test]
    fn control_nothing_declared_and_nothing_in_history_is_the_one_silent_vacuum() {
        assert!(run(&[], Some(&[]), ONE_FILE).is_empty());
    }

    #[test]
    fn a_row_in_another_namespace_is_not_an_axis() {
        // The discrimination `ids_of` rests on. A filter written on the slug
        // alone counts `ruling::signedness` as the axis and passes.
        let rows = vec![
            "dimension::radix".to_string(),
            "ruling::signedness".to_string(),
        ];
        assert_eq!(super::ids_of(&rows), vec!["radix".to_string()]);
    }

    #[test]
    fn ids_are_read_from_the_table_header_rather_than_the_filename() {
        // What lets one namespace live in two files, which is why the
        // declaring-file arm is a report rather than a parse failure.
        let text = "[[dimension]]\nid = \"radix\"\n\n[[ruling]]\nid = \"not_an_axis\"\n";
        assert_eq!(super::ids_in(text), vec!["radix".to_string()]);
    }

    #[test]
    fn control_a_file_declaring_no_axis_yields_no_ids() {
        assert!(super::ids_in("[[ruling]]\nid = \"a_ruling\"\n").is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        // This runs the real `check_repo` against a tree where no commit
        // touches the file, which is the third route to a walk that
        // established nothing, alongside a shallow clone and an unavailable
        // git. It found nothing at all until that route returned `None`, so
        // this arm is what pins it as well as pinning the severity.
        assert_findings_block(&super::TheAxisSetIsAppendOnly, &view(&[], &[]));
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::TheAxisSetIsAppendOnly);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::TheAxisSetIsAppendOnly.name(), super::LINT);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::LINT);
    }
}
