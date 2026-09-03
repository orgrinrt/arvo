//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A lint that scopes itself to one crate names a crate this workspace ships,
//! or it can never fire and nothing says so.
//!
//! A `CrateLint` runs once per crate, so one that governs a single crate opens
//! by returning early everywhere else:
//!
//! ```ignore
//! if ctx.crate_name != "arvo-bits" {
//!     return Vec::new();
//! }
//! ```
//!
//! When that crate leaves the tree the lint stays, compiles, registers, is
//! configured at `error` in `mockspace.toml`, and returns early on every crate
//! there is. Every gate reports it as passing, because passing and never
//! running produce the same output.
//!
//! **The tier it guards is the thing that makes this worth refusing.** arvo's
//! crate layout above the bottom three was removed on purpose, and a lint
//! enforcing that layout's rules is the removed tier surviving in the one place
//! nobody reads: it is not a document mentioning a dead crate, it is a rule
//! still nominally in force over one.
//!
//! It has happened. `arvo-bits-traits-only` was 348 lines gated on a crate that
//! had not existed for as long as the canon work has been running, with a
//! `[lints.arvo-bits-traits-only]` block setting it to `error` at all three
//! gates. Three surfaces naming crates arvo does not have had already been
//! fixed in a separate change, and this one survived that sweep because a lint
//! is source rather than prose and the sweep was over prose.
//!
//! **What this reads is the gate, not every mention of a crate name.** A lint's
//! module doc names crates freely, its tests plant fixtures with
//! `with_crate_name("arvo-bits", ...)`, and neither is a claim that the crate is
//! in the tree. Only the early-return comparison against `ctx.crate_name` says
//! the lint governs that crate and nothing else, so only that is read.
//!
//! **Known gap**, catalogued as a red test below rather than fixed: the match
//! is over raw source with no notion of being inside a comment or a string
//! beyond a whole-line `//` prefix, so a commented-out gate on a dead crate is
//! reported and a gate written inside a block comment is too. That is the same
//! class of gap the source-side scanners in this directory carry, for the same
//! reason.
//!
//! **A lint rather than a tool.** There is no state of this repository in which
//! a lint may guard a crate that does not exist, so this refuses rather than
//! reports.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ALintGuardsACrateThatExists)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "a-lint-guards-a-crate-that-exists";

struct ALintGuardsACrateThatExists;
impl Lint for ALintGuardsACrateThatExists {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ALintGuardsACrateThatExists {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(&ctx.mock_dir, ctx.all_crates)
    }
}

/// Every crate name one lint file gates itself on.
///
/// Both directions are read. `!=` is the early-return form and is what every
/// scoped lint in this directory uses; `==` is the same claim written the other
/// way round, and a lint using it governs that crate exactly as narrowly.
///
/// The comparison has to be against `ctx.crate_name` specifically. A test
/// helper taking a crate name as an argument is not a gate, and a module doc
/// naming a crate is not one either, so neither is read.
fn gated_crates(source: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for line in source.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        for operator in ["crate_name != \"", "crate_name == \""] {
            let mut rest = line;
            while let Some((_, after)) = rest.split_once(operator) {
                if let Some((name, tail)) = after.split_once('"') {
                    found.insert(name.to_string());
                    rest = tail;
                } else {
                    break;
                }
            }
        }
    }
    found
}

/// The verdict, over a mock directory and the crates the workspace resolved.
///
/// Split from the trait impl so a test can point it at a tree it built with a
/// crate set it chose, and so the one place that decides is one function.
///
/// **An empty crate set returns nothing rather than refusing everything.** A
/// context that resolved no crates is a context that cannot answer the
/// question, and reading its silence as "no crate exists" would turn every
/// scoped lint in the directory into a finding at once. That is the shape a
/// false alarm takes here, and it would arrive on every gate in one run.
fn check(mock_dir: &Path, crates: &BTreeSet<String>) -> Vec<LintError> {
    if crates.is_empty() {
        return Vec::new();
    }

    let Ok(entries) = std::fs::read_dir(mock_dir.join("lints")) else {
        return Vec::new();
    };

    let mut by_file: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let missing: BTreeSet<String> = gated_crates(&text)
            .into_iter()
            .filter(|name| !crates.contains(name))
            .collect();
        if missing.is_empty() {
            continue;
        }
        by_file.insert(entry.file_name().to_string_lossy().to_string(), missing);
    }

    by_file
        .into_iter()
        .map(|(file, missing)| finding(&file, &missing))
        .collect()
}

/// One finding per lint file, naming every crate that file gates on and the
/// workspace does not ship.
///
/// Per file rather than per name, because the remedy is the same for all of
/// them and it is about the file: either the lint goes with the tier it was
/// guarding, or the gate is rewritten against a crate that is here.
fn finding(file: &str, missing: &BTreeSet<String>) -> LintError {
    let named = missing.iter().cloned().collect::<Vec<_>>().join(", ");
    // The location and `path` are joined by the renderer rather than being two
    // spellings of one answer, so the location is the mock directory and the
    // path is where inside it to look. Setting both to the file doubles it.
    let mut error = LintError::error(
        "mock".to_string(),
        0,
        NAME,
        format!(
            "gates itself on {named}, which this workspace does not ship, so it \
             returns early on every crate and cannot fire. Delete it with the tier \
             it guarded, or point the gate at a crate that is here."
        ),
    );
    error.finding_kind = Some("a-lint-guards-a-crate-that-is-gone");
    error.path = Some(format!("lints/{file}"));
    error
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::canon_lint_testkit::{
        assert_findings_block_at,
        assert_not_declared_off,
        assert_registered,
        ctx_at,
        plant,
        planted_tree,
        view,
    };

    /// The three crates arvo ships, as the resolved set a context carries.
    fn shipped() -> BTreeSet<String> {
        ["arvo-format", "arvo-placement", "arvo-strategy"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    /// A mock directory carrying a `lints/` built from file name and body.
    ///
    /// `planted_tree` is the testkit's, keyed on this name plus the process and
    /// thread, so two arms cannot plant into each other's tree.
    fn planted_mock(what: &str, files: &[(&str, &str)]) -> PathBuf {
        let dir = planted_tree(what);
        std::fs::create_dir_all(dir.join("lints")).expect("a planted lints directory");
        for (name, body) in files {
            plant(&dir, &format!("lints/{name}"), body);
        }
        dir
    }

    /// A scoped lint's early return, in the shape every one in the directory
    /// writes it.
    fn gate(crate_name: &str) -> String {
        format!(
            "use mockspace::*;\n\
             impl CrateLint for X {{\n\
                 fn check(&self, ctx: &LintContext) -> Vec<LintError> {{\n\
                     if ctx.crate_name != \"{crate_name}\" {{\n\
                         return Vec::new();\n\
                     }}\n\
                     Vec::new()\n\
                 }}\n\
             }}\n"
        )
    }

    #[test]
    fn a_gate_on_a_crate_that_is_gone_is_a_finding() {
        let dir = planted_mock("gone", &[("dead.rs", &gate("arvo-bits"))]);
        let found = check(&dir, &shipped());
        assert_eq!(
            found.len(),
            1,
            "one file gates on a crate that is not shipped"
        );
        assert!(
            found[0].message.contains("arvo-bits"),
            "the finding names the crate rather than only the file: {}",
            found[0].message
        );
    }

    #[test]
    fn a_gate_on_a_crate_that_is_here_is_not_a_finding() {
        let dir = planted_mock("here", &[("live.rs", &gate("arvo-format"))]);
        assert!(
            check(&dir, &shipped()).is_empty(),
            "a lint scoped to a crate the workspace ships is doing its job"
        );
    }

    #[test]
    fn the_equality_form_is_read_as_the_same_claim() {
        let body = "impl X { fn f(ctx: &C) { if ctx.crate_name == \"arvo-bits\" { g(); } } }\n";
        let dir = planted_mock("equality", &[("dead.rs", body)]);
        assert_eq!(
            check(&dir, &shipped()).len(),
            1,
            "`==` scopes a lint exactly as narrowly as `!=` and is read the same way"
        );
    }

    #[test]
    fn a_lint_that_scopes_itself_to_nothing_is_not_a_finding() {
        let body = "impl RepoLint for X { fn check_repo(&self, _: &RepoContext) -> Vec<LintError> \
                    { Vec::new() } }\n";
        let dir = planted_mock("unscoped", &[("wide.rs", body)]);
        assert!(
            check(&dir, &shipped()).is_empty(),
            "a lint with no crate gate governs every crate and names none"
        );
    }

    #[test]
    fn a_test_fixture_naming_a_dead_crate_is_not_a_gate() {
        let body = "#[cfg(test)]\nmod tests {\n    fn f() {\n        let fixture = \
                    LintFixture::new(s).with_crate_name(\"arvo-bits\", \"bits\");\n    }\n}\n";
        let dir = planted_mock("fixture", &[("scoped.rs", body)]);
        assert!(
            check(&dir, &shipped()).is_empty(),
            "planting a fixture under a name is not a claim that the crate is in the tree"
        );
    }

    #[test]
    fn a_module_doc_naming_a_dead_crate_is_not_a_gate() {
        let body = "//! `arvo-bits` used to live here, and this explains why it does not.\n\
                    //! if ctx.crate_name != \"arvo-bits\" was the shape it had.\n\
                    pub fn f() {}\n";
        let dir = planted_mock("doc", &[("prose.rs", body)]);
        assert!(
            check(&dir, &shipped()).is_empty(),
            "prose about a removed crate is a record of it, not a rule over it"
        );
    }

    #[test]
    fn several_dead_crates_in_one_file_are_one_finding_naming_all_of_them() {
        let body = format!("{}{}", gate("arvo-bits"), gate("arvo-mask"));
        let dir = planted_mock("several", &[("dead.rs", &body)]);
        let found = check(&dir, &shipped());
        assert_eq!(
            found.len(),
            1,
            "the remedy is about the file, so the finding is too"
        );
        assert!(
            found[0].message.contains("arvo-bits") && found[0].message.contains("arvo-mask"),
            "both names reach the reader: {}",
            found[0].message
        );
    }

    #[test]
    fn a_context_that_resolved_no_crates_reports_nothing() {
        let dir = planted_mock("empty", &[("dead.rs", &gate("arvo-bits"))]);
        assert!(
            check(&dir, &BTreeSet::new()).is_empty(),
            "an empty crate set cannot answer the question, and reading it as an answer \
             would refuse every scoped lint at once"
        );
    }

    #[test]
    fn a_tree_with_no_lints_directory_reports_nothing() {
        let dir = planted_tree("no-lints");
        std::fs::create_dir_all(&dir).expect("a planted tree");
        assert!(
            check(&dir, &shipped()).is_empty(),
            "a repository carrying no lints has none that could guard a dead crate"
        );
    }

    #[test]
    #[ignore = "catalogue: a commented-out gate is read as a gate; tracked in the module doc"]
    fn a_commented_out_gate_is_not_a_rule_and_should_not_be_read_as_one() {
        let body = "impl X { fn f(ctx: &C) {\n    /* if ctx.crate_name != \"arvo-bits\" { \
                    return; } */\n} }\n";
        let dir = planted_mock("commented", &[("dead.rs", body)]);
        assert!(
            check(&dir, &shipped()).is_empty(),
            "a gate inside a block comment governs nothing, so it is not a finding"
        );
    }

    #[test]
    fn the_finding_blocks_every_gate() {
        // Through `check_repo` rather than through `finding`, because the
        // severity a gate reads is the one that survives the trait call. The
        // testkit's `ctx_at` resolves no crates, which is the arm that reports
        // nothing, so this builds a context carrying the crates arvo ships.
        let dir = planted_mock("blocks", &[("dead.rs", &gate("arvo-bits"))]);
        let registry = view(&[], &[]);
        let crates = shipped();
        let mut ctx = ctx_at(&dir, &registry);
        ctx.all_crates = &crates;
        assert_findings_block_at(&ALintGuardsACrateThatExists, &ctx);
    }

    #[test]
    fn the_location_and_the_path_are_not_two_spellings_of_the_same_answer() {
        // The renderer joins them, so naming the file in both printed
        // `lints/dead.rs/lints/dead.rs` at a reader who then cannot open it.
        // Measured on the real corpus before it was fixed.
        let error = finding("dead.rs", &["arvo-bits".to_string()].into_iter().collect());
        assert_eq!(error.path.as_deref(), Some("lints/dead.rs"));
        assert!(
            !error
                .path
                .as_deref()
                .unwrap_or_default()
                .starts_with("lints/lints/"),
            "the path is where inside the mock directory to look, once"
        );
    }

    #[test]
    fn it_is_registered_and_not_declared_off() {
        assert_registered(NAME);
        assert_not_declared_off(&ALintGuardsACrateThatExists);
    }
}
