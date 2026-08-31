//! What a `CrateLint`'s own tests are built out of.
//!
//! **This file declares no lint.** The engine scans each `mock/lints/*.rs` for
//! `lint()`, `cross_lint()`, `repo_lint()` and `message_lint()`, includes the
//! file as a module either way, and registers only what it found. So a module
//! that defines none of them compiles into the cdylib alongside the lints and
//! reaches nothing, which is what lets every `CrateLint` here share one set of
//! helpers without any of them depending on another's internals.
//!
//! Everything here is `#[cfg(test)]`, so a release build of the pack carries an
//! empty module.
//!
//! `canon_lint_testkit` already carries [`crate::canon_lint_testkit::assert_not_declared_off`],
//! which is generic over any [`Lint`] and applies to a `CrateLint` unchanged;
//! it is not repeated here. What is added here is the crate-lint half: pulling
//! `LintPack::crate_lints` rather than `repo_lints`, and stating what a
//! finding's severity is rather than assuming `HARD_ERROR`, because one of the
//! five lints under test here is `ADVISORY` on purpose.

#![cfg(test)]

use std::collections::HashMap;

use mockspace::{CrateLint, LintConfig, LintContext, LintError, LintPack, Severity};

/// The names of every crate lint in the pack the engine is handed.
///
/// Mirrors `canon_lint_testkit::registered_repo_lints`, over
/// `LintPack::crate_lints` instead of `repo_lints`: the same
/// `__mockspace_collect_lints` scan, read for the other trait object.
pub fn registered_crate_lints() -> Vec<String> {
    let mut pack = LintPack::default();
    crate::__mockspace_collect_lints(&mut pack);
    pack.crate_lints
        .iter()
        .map(|l| l.name().to_string())
        .collect()
}

/// Assert this lint is in the pack, under the name it answers to.
///
/// The name matters as much as the presence: `[lints.<name>]` in
/// `mockspace.toml` is keyed on it, and a lint whose declared `name()`
/// disagrees with the pack's registration is one `mockspace.toml` can never
/// reach.
pub fn assert_registered(name: &str) {
    let names = registered_crate_lints();
    assert!(
        names.iter().any(|n| n == name),
        "`{name}` is not in the pack the engine is handed, so nothing runs it \
         however well it works. The pack carries: {names:?}"
    );
}

/// Every finding this lint produced carries the severity its own
/// `default_severity` declares.
///
/// Not the same claim as "blocks every gate": `no-runtime-grow` declares
/// itself `ADVISORY`, and asserting `HARD_ERROR` against it would be wrong
/// rather than merely stricter. A caller states the severity it actually
/// expects, and this checks every finding agrees with it and with what the
/// lint's own `default_severity()` returns, which is the invariant
/// `canon_lint_testkit::assert_findings_block`'s own comment names as
/// independent of the declared default: nothing here assumes the two must
/// agree, this asserts that for these five they do.
pub fn assert_findings_carry(lint: &dyn CrateLint, findings: &[LintError], expected: Severity) {
    assert!(
        !findings.is_empty(),
        "nothing was found, so this says nothing about what a finding carries"
    );
    assert_eq!(
        lint.default_severity(),
        expected,
        "`{}` declares a default severity other than what this test expects",
        lint.name()
    );
    for e in findings {
        assert_eq!(
            e.severity, expected,
            "`{}` reported `{}` at a severity other than its own declared default",
            e.lint_name, e.message
        );
    }
}

/// Run one `CrateLint` against a fixture with the given `[lints.<name>]`
/// base-severity override applied, the way `mockspace.toml` applies one.
///
/// `mockspace-lint-rules` does not expose its TOML loader to a consumer's
/// generated cdylib (only the parsed [`LintConfig`] shape is public), so this
/// is the same override this repo's `mockspace.toml` would produce, built by
/// hand from the same values rather than read from the file: `from_base`
/// with the one entry `[lints.<name>]` would resolve to. It exercises the
/// same `run_with_overrides` restamping path the real gate does, through the
/// same public [`mockspace::check_crate_with_extra`] entry point.
pub fn findings_under_override(
    ctx: &LintContext,
    lint: Box<dyn CrateLint>,
    lint_name: &str,
    over: Severity,
) -> Vec<LintError> {
    let cfg = LintConfig::from_base(HashMap::from([(lint_name.to_string(), over)]));
    mockspace::check_crate_with_extra(ctx, false, Some(&cfg), std::slice::from_ref(&lint))
        .into_iter()
        .filter(|e| e.lint_name == lint_name)
        .collect()
}
