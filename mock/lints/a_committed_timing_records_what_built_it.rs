//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A timing with no build profile is a measurement of nothing.
//!
//! The profile is a dimension like any other, and under a notation where an
//! unstated dimension claims nothing, a timing that does not name it holds
//! nowhere. The workspace rule that says so was written after a true finding was
//! retired as unreproducible: a crate measured at about 107 seconds, three
//! refutations at four to five, everybody right, and nobody had written down
//! that the first was a debug build and the rest were release. Measured back to
//! back afterwards: **a factor of 29.**
//!
//! The same rule asks for the tree the artifact came from. A meta recording a
//! dirty worktree cannot be tied to a commit, so the variant code behind the
//! number is whatever happened to be on disk when somebody ran it.
//!
//! **The harness records both when the tool runs it, and the committed corpus
//! predates that or was run around it.** So the population under the ceilings is
//! not wrong, it is unciteable as magnitudes, and nothing but re-running under
//! the tool changes that. What the ceilings pin is that the corpus does not grow
//! another one.
//!
//! **A lint rather than a tool.** A new timing that cannot say what built it is
//! a refused state: it enters the record looking exactly like a citable one, and
//! the cost of that is a true finding being retired as unreproducible, which has
//! happened. The ceilings grandfather a population nobody can repair by editing,
//! only by re-running, and every artifact above them is refused.
use std::path::{Path, PathBuf};

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ACommittedTimingRecordsWhatBuiltIt {
        without_a_profile: WITHOUT_A_PROFILE,
        from_a_dirty_tree: FROM_A_DIRTY_TREE,
    })
}

const NAME: &str = "a-committed-timing-records-what-built-it";

/// Where the harness leaves its artifacts, relative to the mock directory.
const BENCHES: &str = "benches";

/// The suffix the harness gives a timing's metadata.
const META: &str = ".meta.json";

/// The key a meta carries when the tool recorded the profile.
const PROFILE: &str = "\"build_profile\"";

/// What a commit hash carries when the worktree was not clean.
const DIRTY: &str = "-dirty";

/// Committed timings naming no build profile, measured over the committed tree.
///
/// **Lower it by re-running under the tool; never raise it.**
const WITHOUT_A_PROFILE: usize = 254;

/// Committed timings taken from a dirty worktree, measured over the committed
/// tree.
const FROM_A_DIRTY_TREE: usize = 253;

/// The lint, carrying the two ceilings it grandfathers.
///
/// Fields rather than constants read inside the predicate, so a test can build
/// one at zero and drive the whole lint against a planted tree. A ceiling only
/// ever exercised at the number it was measured with is a ceiling nobody has
/// seen fire.
struct ACommittedTimingRecordsWhatBuiltIt {
    without_a_profile: usize,
    from_a_dirty_tree: usize,
}

impl Lint for ACommittedTimingRecordsWhatBuiltIt {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ACommittedTimingRecordsWhatBuiltIt {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        self.check(&ctx.mock_dir.join(BENCHES))
    }
}

impl ACommittedTimingRecordsWhatBuiltIt {
    fn check(&self, dir: &Path) -> Vec<LintError> {
        if !dir.is_dir() {
            // A project with no bench tree has no timings, which is a
            // legitimate state and not this lint's business. Reporting it would
            // make the pack refuse every repository that has not run a bench.
            return Vec::new();
        }
        let all = metas(dir);
        if all.is_empty() {
            return vec![finding(
                NAME,
                Some("no-artifacts-to-read"),
                format!(
                    "`mock/{BENCHES}` exists and holds no `*{META}` artifact, so both counts \
                     below are over an empty population and certify nothing. Either the \
                     harness has never run here, or the walk is reading the wrong tree."
                ),
            )];
        }
        let mut out = Vec::new();
        let bare = named(&all, dir, |text| !text.contains(PROFILE));
        if bare.len() > self.without_a_profile {
            out.push(finding(
                NAME,
                Some("no-build-profile"),
                format!(
                    "{} of {} committed timings name no build profile, against a ceiling of \
                     {}. A timing that does not say what built it cannot be compared with \
                     anything: the same crate measured 109s under one profile and 3.78s \
                     under another, a factor of 29, and a true finding was retired as \
                     unreproducible over exactly that. Run it through the tool rather than \
                     raising this number. Newest offenders: {:?}",
                    bare.len(),
                    all.len(),
                    self.without_a_profile,
                    &bare[..bare.len().min(8)]
                ),
            ));
        }
        let dirty = named(&all, dir, |text| text.contains(DIRTY));
        if dirty.len() > self.from_a_dirty_tree {
            out.push(finding(
                NAME,
                Some("from-a-dirty-tree"),
                format!(
                    "{} of {} committed timings were taken from a dirty worktree, against a \
                     ceiling of {}. Nothing ties such a number to the variant code that \
                     produced it, so the arm it measures is whatever happened to be on disk. \
                     Commit first, then measure. Newest offenders: {:?}",
                    dirty.len(),
                    all.len(),
                    self.from_a_dirty_tree,
                    &dirty[..dirty.len().min(8)]
                ),
            ));
        }
        out
    }
}

/// The names of the artifacts whose text satisfies a predicate.
fn named(all: &[PathBuf], dir: &Path, wanted: impl Fn(&str) -> bool) -> Vec<String> {
    all.iter()
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|t| wanted(&t)))
        .map(|p| crate::panel_corpus::shown(p, dir))
        .collect()
}

/// Every timing metadata artifact under the bench tree, sorted.
fn metas(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect(dir, &mut out);
    out.sort();
    out
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.to_string_lossy().ends_with(META) {
            out.push(path);
        }
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use super::ACommittedTimingRecordsWhatBuiltIt as Timings;
    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };

    /// A bench tree carrying the artifacts named, and the finding kinds over it.
    fn kinds(
        what: &str,
        files: &[(&str, &str)],
        without_a_profile: usize,
        from_a_dirty_tree: usize,
    ) -> Vec<Option<&'static str>> {
        let dir = planted_tree(what);
        std::fs::create_dir_all(dir.join("benches")).expect("a bench tree");
        for (at, text) in files {
            plant(&dir, &format!("benches/{at}"), text);
        }
        Timings {
            without_a_profile,
            from_a_dirty_tree,
        }
        .check(&dir.join("benches"))
        .into_iter()
        .map(|e| e.finding_kind)
        .collect()
    }

    /// A meta the tool wrote: it names the profile and a clean commit.
    const COMPLETE: &str =
        r#"{"cpu":"x","git_commit":"abc1234","build_profile":"opt-level=3,lto=\"fat\""}"#;
    /// A meta from before the tool recorded either.
    const BARE: &str = r#"{"cpu":"x","git_commit":"abc1234-dirty"}"#;

    #[test]
    fn the_two_properties_are_read_apart_rather_than_together() {
        // The discrimination. One artifact carries both defects and one carries
        // neither, so a walk that conflated them would report the complete one
        // under one of the two kinds. The arms this replaced asserted that a
        // string literal contained a substring visible in the literal, which
        // cannot fail and established nothing about the walk.
        let files = [("a/1.meta.json", COMPLETE), ("a/2.meta.json", BARE)];
        assert_eq!(
            kinds("timings-apart", &files, 0, 1),
            [Some("no-build-profile")],
            "one of the two names no profile, and one of the two is from a dirty tree"
        );
        assert_eq!(
            kinds("timings-apart2", &files, 1, 0),
            [Some("from-a-dirty-tree")]
        );
    }

    #[test]
    fn control_a_complete_artifact_is_reported_under_neither() {
        assert!(kinds("timings-complete", &[("a/1.meta.json", COMPLETE)], 0, 0).is_empty());
    }

    #[test]
    fn both_ceilings_fire_together_where_both_are_exceeded() {
        let files = [("a/1.meta.json", BARE), ("a/2.meta.json", BARE)];
        let k = kinds("timings-both", &files, 0, 0);
        assert_eq!(k.len(), 2, "{k:?}");
        assert!(k.contains(&Some("no-build-profile")), "{k:?}");
        assert!(k.contains(&Some("from-a-dirty-tree")), "{k:?}");
    }

    #[test]
    fn a_bench_tree_holding_no_artifact_is_the_finding_rather_than_a_pass() {
        // Both counts are over the population, so an empty one makes both
        // ceilings vacuous while every other arm here still passes. That is the
        // shape the whole migration exists to refuse.
        assert_eq!(
            kinds(
                "timings-empty",
                &[("a/notes.md", "no artifacts here")],
                0,
                0
            ),
            [Some("no-artifacts-to-read")]
        );
    }

    #[test]
    fn control_a_project_with_no_bench_tree_at_all_is_silent() {
        // The other direction, and the one that keeps the pack shippable to a
        // repository that has never run a bench.
        let dir = planted_tree("timings-nobenches");
        assert!(Timings {
            without_a_profile: 0,
            from_a_dirty_tree: 0,
        }
        .check(&dir.join("benches"))
        .is_empty());
    }

    #[test]
    fn a_nested_artifact_is_walked_and_a_neighbouring_file_is_not() {
        assert_eq!(
            kinds(
                "timings-nested",
                &[
                    ("deep/down/1.meta.json", BARE),
                    ("deep/down/1.csv", "not a meta"),
                ],
                0,
                usize::MAX,
            ),
            [Some("no-build-profile")]
        );
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("timings-severity");
        plant(&dir, "mock/benches/a/1.meta.json", BARE);
        let empty = view(&[], &[]);
        assert_findings_block_at(
            &Timings {
                without_a_profile: 0,
                from_a_dirty_tree: 0,
            },
            &ctx_at(&dir.join("mock"), &empty),
        );
    }

    #[test]
    fn the_committed_ceilings_are_what_the_registered_lint_carries() {
        assert_eq!(super::WITHOUT_A_PROFILE, 254);
        assert_eq!(super::FROM_A_DIRTY_TREE, 253);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&Timings {
            without_a_profile: super::WITHOUT_A_PROFILE,
            from_a_dirty_tree: super::FROM_A_DIRTY_TREE,
        });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            Timings {
                without_a_profile: 0,
                from_a_dirty_tree: 0,
            }
            .name(),
            super::NAME
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
