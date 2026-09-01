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
//! What is actually at stake is narrow and worth stating exactly. Adding an
//! axis reaches nobody: a predicate's absence quantifies over the world rather
//! than over this file, so a row written before an axis was declared was always
//! as narrow as it now reads. **Deleting or renaming one is what rewrites
//! meaning**, because a span written over that axis becomes unparseable and a
//! written absence becomes nonsense, and both fail silently: the corpus still
//! reads, and the sentence it now says is a different one.
//!
//! **The record that decides it is git**, because that is the only record a
//! deletion cannot edit in the commit that deletes. Every id the namespace has
//! carried in `dimension.toml` across that file's history must still resolve to
//! a row. Six commits touch the file, so the walk is one `git log` and six
//! `git show`, measured at 0.2 seconds.
//!
//! **A roster is kept beside it**, read in both directions: every id on it must
//! resolve, and every row must be on it. That pair is forgeable, and knowing
//! exactly how is the point of keeping it. An author who deletes an axis is
//! told a roster entry no longer resolves, and the obvious way to silence that
//! is to delete the entry, which is a two-line edit in a file already open.
//! What the roster buys instead is a check that needs no git at all, and a
//! record of the axis set a reader can see without running anything.
//!
//! **So the arms fail differently on purpose.** The history arm is unforgeable
//! and needs a git checkout. The roster arms need nothing and can be defeated
//! in one commit. **Where the history cannot be read, that is reported rather
//! than passed**, because a check that could not run is not a check that
//! agreed.
//!
//! **The premise is checked rather than assumed.** The history arm reads one
//! path, so a second file declaring the namespace would leave its axes
//! unprotected without any arm noticing. `law.toml` has already been split
//! exactly that way, so the third arm reports it, and the fix is to widen the
//! scan rather than to keep the premise. Reading every path the registry
//! directory has ever held measured 7.3 seconds against 0.2 for the one file,
//! which is why the scan is narrow and the premise is guarded.
//!
//! **No ceiling and no threshold.** The refused state is not a population that
//! grew, it is one id that stopped resolving, and there is no number to
//! grandfather because the committed registry satisfies every arm exactly
//! today: 24 rows, 24 roster entries, and 24 ids across the file's whole
//! history.
//!
//! # What the unit tests here cannot ask
//!
//! That the shipped roster matches the committed registry, and that the history
//! walk reads the real repository. A unit test cannot build a `RegistryView`
//! from `mock/registry/` (that needs a TOML parser the generated pack has no
//! route to depend on) and a planted temporary directory is not a git checkout.
//! So the predicate is a pure function over the three inputs and every arm below
//! drives it directly, the reading of git and of the registry directory being
//! the thin part left over.
//!
//! **All three were driven against the real repository, deliberately.** Renaming
//! `radix` produces the roster arm naming it gone, plus two
//! `every-predicate-names-a-declared-axis` errors in the same run, which is the
//! harm this describes arriving on cue. Adding an unrostered row produces the
//! second arm. **Deleting `leaf_aliasing` from the registry and the roster
//! together produces the history arm**, and produced nothing at all before that
//! arm existed, which is the defect it was added for.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(TheAxisSetIsAppendOnly { roster: ROSTER })
}

const LINT: &str = "the-axis-set-is-append-only";

/// The namespace whose rows are the axes.
const NAMESPACE: &str = "dimension";

/// Every axis id that has ever been declared.
///
/// **Append only.** A new axis adds a line here in the same commit that adds
/// the row. Removing a line is removing the check, not satisfying it.
const ROSTER: &[&str] = &[
    "integer_width",
    "fraction_width",
    "total_width",
    "signedness",
    "overflow_policy",
    "rounding",
    "operation",
    "arity",
    "chain_length",
    "target_features",
    "threads",
    "container",
    "alignment",
    "access_pattern",
    "strategy",
    "radix",
    "ambient_domain",
    "accumulator_width",
    "build_profile",
    "toolchain",
    "operand_window",
    "occupancy",
    "association",
    "leaf_aliasing",
];

/// The lint, carrying the roster it pins.
///
/// A field rather than the constant read inside the predicate, so a test can
/// drive it with a planted roster against a planted registry. A roster only
/// ever exercised against the rows it was copied from is a roster nobody has
/// seen fire.
struct TheAxisSetIsAppendOnly {
    roster: &'static [&'static str],
}
impl Lint for TheAxisSetIsAppendOnly {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
/// The registry file the axes are declared in, from the repository root.
const FILE: &str = "mock/registry/dimension.toml";

impl RepoLint for TheAxisSetIsAppendOnly {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let live = ids_of(ctx.registry.rows_in(NAMESPACE));
        check(
            &live,
            self.roster,
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
/// `ever` is `None` where the history could not be read at all, which is itself
/// reported: a check that cannot run is not a check that passed.
fn check(
    live: &[String],
    roster: &[&str],
    ever: Option<&[String]>,
    declaring: &[String],
) -> Vec<LintError> {
    let rows: Vec<&str> = live.iter().map(String::as_str).collect();
    let gone: Vec<&str> = roster
        .iter()
        .copied()
        .filter(|id| !rows.contains(id))
        .collect();
    let unrostered: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|id| !roster.contains(id))
        .collect();

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
                         now: {lost:?}. This arm reads git rather than the roster below, so \
                         deleting the row and its roster entry in one commit does not reach \
                         it. Restore the row under its original id, or rewrite history, which \
                         is not something this repository does.",
                        lost.len()
                    ),
                ));
            }
        }
        None => out.push(finding(
            LINT,
            None,
            format!(
                "the history of `{FILE}` could not be read, so the arm that a single commit \
                 cannot defeat did not run and only the roster below applied. That roster \
                 lives in the working tree and can be edited in the same commit as a \
                 deletion, so this is a weaker check rather than a passing one. Run this in a \
                 git checkout of the repository."
            ),
        )),
    }
    if declaring.len() > 1 || (declaring.len() == 1 && !FILE.ends_with(&declaring[0])) {
        out.push(finding(
            LINT,
            None,
            format!(
                "`{NAMESPACE}` rows are declared in {declaring:?} and the history arm reads \
                 only `{FILE}`, so an axis declared in another of them is unprotected. Widen \
                 the scan to every file that declares the namespace, and mind that reading \
                 every path the registry directory has ever held measured 7.3 seconds against \
                 0.2 for the one file, so the scan is the part to make cheaper rather than \
                 the premise to keep."
            ),
        ));
    }
    if !gone.is_empty() {
            out.push(finding(
                LINT,
                None,
                format!(
                    "{} declared axes no longer resolve to a row: {gone:?}. The set is \
                     append-only, because a span written over an axis becomes unparseable when \
                     the axis goes and a written absence becomes nonsense, and both fail \
                     silently. Restore the row under its original id. A rename is a deletion and \
                     needs the old id kept, or every predicate written in it stops parsing. \
                     Deleting the roster entry instead removes the check rather than satisfying \
                     it.",
                    gone.len()
                ),
            ));
        }
        if !unrostered.is_empty() {
            out.push(finding(
                LINT,
                None,
                format!(
                    "{} axes are declared and not rostered: {unrostered:?}. Nothing would catch \
                     their deletion, so the arm above cannot see them and the set is append-only \
                     only over the part somebody remembered to list. Add each id to `ROSTER` in \
                     the commit that declares it, which is the one moment nobody is thinking \
                     about deletion.",
                unrostered.len()
            ),
        ));
    }
    out
}

/// Every `id` the namespace has carried in `FILE` across that file's history.
///
/// `None` where git is not available or the file has no history, which the
/// caller reports rather than reading as nothing lost. Six commits touch this
/// file, so the walk is one `git log` and six `git show`, measured at 0.2
/// seconds.
fn ids_ever_declared(repo_root: &Path) -> Option<Vec<String>> {
    let commits = git(repo_root, &["log", "--format=%H", "--", FILE])?;
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
        .filter(|p| {
            std::fs::read_to_string(p).is_ok_and(|t| !ids_in(&t).is_empty())
        })
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

    const ROSTER_TWO: &[&str] = &["radix", "signedness"];

    /// The one file the history arm reads, as `files_declaring` reports it.
    const ONE_FILE: &[&str] = &["dimension.toml"];

    /// The whole predicate on planted inputs.
    ///
    /// `ever` is stated per arm rather than defaulted, because defaulting it to
    /// the live rows would make the history arm silent everywhere and no arm
    /// would be about it.
    fn run(live: &[&str], roster: &[&str], ever: Option<&[&str]>, files: &[&str]) -> Vec<String> {
        let live: Vec<String> = live.iter().map(|s| (*s).to_string()).collect();
        let ever: Option<Vec<String>> =
            ever.map(|e| e.iter().map(|s| (*s).to_string()).collect());
        let files: Vec<String> = files.iter().map(|s| (*s).to_string()).collect();
        super::check(&live, roster, ever.as_deref(), &files)
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    /// The ordinary case: history agrees with the rows, one declaring file.
    fn plant(live: &[&str]) -> Vec<String> {
        run(live, ROSTER_TWO, Some(live), ONE_FILE)
    }

    #[test]
    fn a_registry_holding_every_rostered_axis_is_silent() {
        assert!(plant(&["radix", "signedness"]).is_empty());
    }

    #[test]
    fn a_rostered_axis_that_is_gone_is_named() {
        let f = plant(&["radix"]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(!f[0].contains("radix"), "the surviving axis was named: {}", f[0]);
    }

    #[test]
    fn every_missing_axis_is_named_rather_than_the_first() {
        let f = plant(&[]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("radix"), "{}", f[0]);
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(f[0].contains('2'), "the count is missing: {}", f[0]);
    }

    #[test]
    fn an_axis_declared_and_not_rostered_is_reported_as_unprotected() {
        // Growth is still free in the sense that matters: an addition is not
        // refused and nothing has to be undone. What it is not is invisible.
        let f = plant(&["radix", "signedness", "an_axis_declared_later"]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("an_axis_declared_later"), "{}", f[0]);
        assert!(f[0].contains("not rostered"), "{}", f[0]);
        assert!(!f[0].contains("no longer resolve"), "an addition was reported as a deletion: {}", f[0]);
    }

    #[test]
    fn a_rename_fires_on_both_halves_and_says_which_is_which() {
        let f = plant(&["radix", "signedness_but_renamed"]);
        assert_eq!(f.len(), 2, "both halves are reported: {f:?}");
        let gone = f.iter().find(|m| m.contains("no longer resolve"))
            .unwrap_or_else(|| panic!("no deletion finding: {f:?}"));
        assert!(gone.contains("signedness"), "{gone}");
        assert!(gone.contains("rename"), "the repair is not stated: {gone}");
        let added = f.iter().find(|m| m.contains("not rostered"))
            .unwrap_or_else(|| panic!("no unprotected finding: {f:?}"));
        assert!(added.contains("signedness_but_renamed"), "{added}");
    }

    #[test]
    fn an_axis_whose_id_merely_contains_a_rostered_one_does_not_satisfy_it() {
        let f = plant(&["radix", "signedness_of_the_accumulator"]);
        assert_eq!(f.len(), 2, "{f:?}");
        let gone = f.iter().find(|m| m.contains("no longer resolve"))
            .unwrap_or_else(|| panic!("the longer id was read as the rostered axis: {f:?}"));
        assert!(gone.contains("\"signedness\""), "{gone}");
        let added = f.iter().find(|m| m.contains("not rostered"))
            .unwrap_or_else(|| panic!("the rostered id was read as covering the longer one: {f:?}"));
        assert!(added.contains("signedness_of_the_accumulator"), "{added}");
    }

    #[test]
    fn a_deletion_from_both_the_registry_and_the_roster_is_still_caught() {
        // The hole the roster alone had, and the reason the history arm exists.
        // Both roster arms are satisfied exactly, because the roster was edited
        // to match, and the axis is still gone.
        let f = run(&["radix"], &["radix"], Some(&["radix", "signedness"]), ONE_FILE);
        assert_eq!(f.len(), 1, "a both-sides deletion was silent: {f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(f[0].contains("history"), "the arm that fired is not the history one: {}", f[0]);
        assert!(!f[0].contains("not rostered"), "{}", f[0]);
    }

    #[test]
    fn control_a_history_agreeing_with_the_rows_says_nothing() {
        // The negative control for the arm above. Without it that arm passes
        // for as long as the history arm fires on anything at all.
        assert!(run(&["radix", "signedness"], ROSTER_TWO, Some(&["radix", "signedness"]), ONE_FILE).is_empty());
    }

    #[test]
    fn a_history_that_cannot_be_read_is_reported_rather_than_passed() {
        // A check that could not run is not a check that passed, and the
        // roster that remains is the forgeable one.
        let f = run(&["radix", "signedness"], ROSTER_TWO, None, ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("could not be read"), "{}", f[0]);
        assert!(f[0].contains("weaker check rather than a passing one"), "{}", f[0]);
    }

    #[test]
    fn a_second_file_declaring_the_namespace_is_reported() {
        // The premise the history arm rests on, checked rather than assumed.
        // `law.toml` has already been split this way once.
        let f = run(
            &["radix", "signedness"],
            ROSTER_TWO,
            Some(&["radix", "signedness"]),
            &["dimension-the-later-axes.toml", "dimension.toml"],
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("dimension-the-later-axes.toml"), "{}", f[0]);
        assert!(f[0].contains("unprotected"), "{}", f[0]);
    }

    #[test]
    fn a_file_that_is_not_the_scanned_one_is_reported_even_when_it_is_the_only_one() {
        // The rename case for the file itself. One file is not enough; it has
        // to be the file the history arm reads.
        let f = run(
            &["radix", "signedness"],
            ROSTER_TWO,
            Some(&["radix", "signedness"]),
            &["axes.toml"],
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("axes.toml"), "{}", f[0]);
    }

    #[test]
    fn control_an_empty_registry_names_the_whole_roster_rather_than_panicking() {
        let f = run(&[], ROSTER_TWO, Some(&[]), ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("radix"), "{}", f[0]);
    }

    #[test]
    fn an_empty_roster_is_loud_rather_than_vacuously_silent() {
        // This arm used to assert the opposite, and pinning that was the
        // defect: with only the deletion arm, an empty roster passed over any
        // registry at all, so the emptier the roster the quieter the lint.
        let f = run(&["radix"], &[], Some(&["radix"]), ONE_FILE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("radix"), "{}", f[0]);
        assert!(f[0].contains("not rostered"), "{}", f[0]);
    }

    #[test]
    fn control_nothing_declared_and_nothing_rostered_is_the_one_silent_vacuum() {
        assert!(run(&[], &[], Some(&[]), ONE_FILE).is_empty());
    }

    #[test]
    fn a_row_in_another_namespace_does_not_satisfy_the_roster() {
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
    fn the_shipped_roster_pins_something_and_holds_no_duplicate() {
        assert!(super::ROSTER.len() >= 24, "{}", super::ROSTER.len());
        let mut sorted = super::ROSTER.to_vec();
        sorted.sort_unstable();
        let mut deduped = sorted.clone();
        deduped.dedup();
        assert_eq!(sorted, deduped, "the roster pins an id twice");
    }

    #[test]
    fn its_findings_block_every_gate() {
        assert_findings_block(&super::TheAxisSetIsAppendOnly { roster: ROSTER_TWO }, &view(&[], &[]));
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::TheAxisSetIsAppendOnly { roster: super::ROSTER });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::TheAxisSetIsAppendOnly { roster: super::ROSTER }.name(),
            super::LINT
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::LINT);
    }
}
