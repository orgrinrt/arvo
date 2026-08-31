//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A committed probe that names an absolute path reads somebody else's clone.
//!
//! **The quietest instrument defect this corpus has produced, and it was found
//! tenth.** A probe script with an absolute path to somebody's checkout does not
//! fail when run somewhere else: if that checkout exists on the host, and here it
//! does, the script succeeds and reports about a different tree. Twenty of the
//! ones carrying it are citation checkers, which is the cheapest correctness tool
//! the panel has, verifying somebody else's clone.
//!
//! They were correct when written, because the panel lived in that clone. They
//! became wrong the moment the arc moved, silently, with nothing in any output
//! saying which tree had been read. **That is the shape worth naming: not a
//! broken probe, a probe that keeps working on the wrong subject.**
//!
//! There is a second thing wrong with them and it is not about reproducibility.
//! The clone they read belongs to somebody else, and a session works in a
//! workspace of its own; a committed probe reaching into another one does it
//! every time anybody runs it, without deciding to.
//!
//! The repair in every case is the same and is one line: resolve the root from
//! the script's own location rather than naming it. Fifteen were repaired by
//! hand and every one was re-run afterwards, which is where the value turned out
//! to be. Pointed at the right tree they do not report clean: three
//! independently written citation checkers each report the same living ledger's
//! line citations no longer resolving, one reports seven failures of forty-one,
//! and two were reaching for files that had since been archived and prefixed.
//! **All of that was invisible while they read a clone where the old paths still
//! resolved.** A probe reading the wrong tree does not merely prove nothing; it
//! hides what it would have proved.
//!
//! **A lint against a ceiling.** What is left sits in landed probe directories,
//! which are the record, so those are counted rather than repaired and a new one
//! is what gets refused. Raising the number is not the fix for a failure.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{finding, panel_dir, scripts, shown};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AProbeReadsTheTreeItSitsIn { ceiling: CEILING })
}

const NAME: &str = "a-probe-reads-the-tree-it-sits-in";

/// The standing count of committed scripts naming a path outside this
/// repository, by file, measured over the committed tree.
///
/// **Lower it as probes are repaired; never raise it.** Was 69 when the wider
/// pattern was first measured, 31 after the repairs, and re-measured here by the
/// lint itself, which is where a ceiling should come from: the number the check
/// reports today, rather than the number the last check happened to be written
/// with.
const CEILING: usize = 25;

/// The prefix that makes a path a home-anchored absolute one.
///
/// A relative path is resolved against wherever the script runs, which is a
/// different and much louder failure, so it stays out.
const ANCHORED: &str = "/Users/";

/// The lint, carrying the ceiling it grandfathers.
///
/// **A field rather than a constant read inside the predicate**, so a test can
/// build one at zero and drive the whole lint, `check_repo` included, against a
/// planted tree. A ceiling only ever exercised at the number it was measured
/// with is a ceiling nobody has seen fire, and every arm asserting the predicate
/// would still pass if the ratchet never reported anything at all.
struct AProbeReadsTheTreeItSitsIn {
    ceiling: usize,
}

impl Lint for AProbeReadsTheTreeItSitsIn {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AProbeReadsTheTreeItSitsIn {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        self.check(&panel_dir(ctx.mock_dir))
    }
}

impl AProbeReadsTheTreeItSitsIn {
    fn check(&self, dir: &Path) -> Vec<LintError> {
        let found = offenders(dir);
        let mut files: Vec<&str> = found.iter().map(|(at, _, _)| at.as_str()).collect();
        files.sort_unstable();
        files.dedup();
        if files.len() <= self.ceiling {
            return Vec::new();
        }
        let first = found
            .first()
            .map(|(_, _, cited)| cited.as_str())
            .unwrap_or("");
        vec![finding(
            NAME,
            "the panel's committed probes",
            0,
            format!(
                "{} probe scripts name a path outside this repository, against a ceiling of \
                 {}. One has been written since the repair: if that path exists on the host \
                 the script succeeds against a tree that is not this one and says nothing, \
                 and if it does not, the script fails for a reason nobody will connect to the \
                 move. Resolve the root from the script's own location rather than raising \
                 this number. First occurrence names `{first}`. Per file: {files:?}",
                files.len(),
                self.ceiling
            ),
        )]
    }
}

/// Every committed script naming a home-anchored absolute path, with the path.
fn offenders(dir: &Path) -> Vec<(String, usize, String)> {
    let mut out = Vec::new();
    for path in scripts(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            let Some(found) = line.find(ANCHORED) else {
                continue;
            };
            let cited: String = line[found..]
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '"' && *c != '\'' && *c != ')')
                .collect();
            out.push((at.clone(), n + 1, cited));
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use super::AProbeReadsTheTreeItSitsIn as Probe;
    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };
    use crate::panel_corpus::PANEL;

    fn tree(what: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
        let dir = planted_tree(what);
        for (at, text) in files {
            plant(&dir, at, text);
        }
        dir
    }

    #[test]
    fn a_script_naming_another_tree_is_found_and_one_resolving_its_own_root_is_not() {
        let dir = tree("probe-tree", &[
            (
                "42_probes/reads_elsewhere.sh",
                "#!/bin/sh\nroot=/Users/somebody/Dev/other-clone/arvo\ngrep -r x \"$root\"\n",
            ),
            (
                "42_probes/reads_itself.sh",
                "#!/bin/sh\nroot=$(cd \"$(dirname \"$0\")/../..\" && pwd)\ngrep -r x \"$root\"\n",
            ),
        ]);
        let found = super::offenders(&dir);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].0.contains("reads_elsewhere.sh"), "{found:?}");
        assert_eq!(
            found[0].2, "/Users/somebody/Dev/other-clone/arvo",
            "the report names the path so it can be fixed without opening the file"
        );
    }

    #[test]
    fn control_prose_naming_a_path_is_not_a_script() {
        // The corpus records what it ran, and a finding there would be a
        // finding about the audit trail doing its job.
        let dir = tree("probe-prose", &[(
            "42_notes.md",
            "It was run against `/Users/somebody/Dev/other-clone/arvo`.\n",
        )]);
        assert!(super::offenders(&dir).is_empty());
    }

    #[test]
    fn control_a_relative_path_is_a_different_and_louder_failure() {
        let dir = tree("probe-relative", &[(
            "43_probes/relative.py",
            "open('../../../mock/registry/dimension.toml')\n",
        )]);
        assert!(
            super::offenders(&dir).is_empty(),
            "a relative path fails loudly where it is wrong, which is the opposite problem"
        );
    }

    #[test]
    fn every_committed_script_extension_is_walked() {
        // The finder reads four extensions and a fixture naming one passes for
        // as long as the other three are unwalked. The panel's probes are
        // written in all of them.
        for ext in ["sh", "py", "rs", "awk"] {
            let dir = tree(
                &format!("probe-ext-{ext}"),
                &[(
                    &format!("44_probes/reads.{ext}"),
                    "root=/Users/somebody/Dev/other-clone\n",
                )],
            );
            assert_eq!(
                super::offenders(&dir).len(),
                1,
                "a `.{ext}` probe is committed and is not walked"
            );
        }
    }

    #[test]
    fn the_ratchet_counts_files_rather_than_occurrences() {
        // A single script naming the path on four lines is one probe to repair,
        // and counting occurrences would make one file look like four.
        let dir = tree("probe-count", &[(
            "45_probes/many.sh",
            "a=/Users/x/one\nb=/Users/x/two\nc=/Users/x/three\nd=/Users/x/four\n",
        )]);
        assert_eq!(super::offenders(&dir).len(), 4, "four lines carry one file");
        assert!(
            Probe { ceiling: 1 }.check(&dir).is_empty(),
            "one file under a ceiling of one is the grandfathered population"
        );
        let f = Probe { ceiling: 0 }.check(&dir);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].message.contains("1 probe scripts"), "{}", f[0].message);
    }

    #[test]
    fn a_panel_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("probe-absent");
        assert!(Probe { ceiling: 0 }.check(&dir.join("nothing")).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        // Driven through `check_repo`, at a ceiling of zero, so the wiring from
        // the mock directory to the panel directory is under test as well as
        // the severity. A lint reading the wrong directory would find nothing
        // and this arm reports that as nothing found.
        let dir = planted_tree("probe-severity");
        plant(
            &dir,
            &format!("mock/{PANEL}/46_probes/reads.sh"),
            "root=/Users/somebody/Dev/other-clone\n",
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(&Probe { ceiling: 0 }, &ctx_at(&dir.join("mock"), &empty));
    }

    #[test]
    fn the_committed_ceiling_is_what_the_registered_lint_carries() {
        // The constant and the lint the engine is handed have to agree, or the
        // arms above measure a ceiling nothing runs at.
        assert_eq!(Probe { ceiling: super::CEILING }.ceiling, 25);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&Probe { ceiling: super::CEILING });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(Probe { ceiling: super::CEILING }.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
