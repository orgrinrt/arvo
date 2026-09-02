//! Lint: arvo is `#![no_std]`. Any `use std::` import is a violation.
//!
//! arvo is a numeric + analysis substrate with zero platform
//! dependency. Crates must not import `std::*`. Consumers that need
//! platform integration pull in their own `std`-bearing crates.
//!
//! Scope: every arvo crate under `mock/crates/`. The lint scans raw
//! source (line by line) to catch imports before they reach the
//! compile pipeline, which gives a clear arvo-specific message
//! rather than a generic `unresolved import` error from `no_std` mode.
//!
//! **Known gap**: the match is anchored on `use` / `pub use` / `extern
//! crate`, so a fully qualified call that never imports the path, such
//! as `std::process::exit(0);`, is invisible to it. See the catalogued
//! test below.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(NoStdEnforcer)
}

struct NoStdEnforcer;

impl Lint for NoStdEnforcer {
    fn name(&self) -> &'static str {
        "no-std-enforcer"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl CrateLint for NoStdEnforcer {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        // `is_proc_macro_crate` does not consider the `lint_proc_macro_source`
        // opt-in (its own doc says so); this is the "should I skip this
        // source lint" decision, which is what that method exists for.
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }

        let mut errors = Vec::new();

        for (idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim_start();

            if trimmed.starts_with("//") {
                continue;
            }

            // Match `use std::`, `use ::std::`, `extern crate std`.
            let hit = trimmed.starts_with("use std::")
                || trimmed.starts_with("use ::std::")
                || trimmed.starts_with("pub use std::")
                || trimmed.starts_with("pub use ::std::")
                || trimmed.starts_with("extern crate std");

            if hit {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-std-enforcer",
                    format!(
                        "arvo is `#![no_std]`; `use std::` is forbidden: {}",
                        trimmed.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    fn hits(source: &str) -> Vec<LintError> {
        let fixture = LintFixture::new(source);
        NoStdEnforcer.check(&fixture.ctx())
    }

    // --- registration and severity -----------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(NoStdEnforcer.name(), "no-std-enforcer");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&NoStdEnforcer);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered("no-std-enforcer");
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors = hits("use std::fmt;\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &NoStdEnforcer,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    // --- the four forbidden shapes, one line each ---------------------

    #[test]
    fn a_bare_use_std_fires() {
        let errors = hits("use std::fmt;\n");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 1);
    }

    #[test]
    fn a_leading_double_colon_use_std_fires() {
        let errors = hits("use ::std::fmt;\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_pub_use_std_fires() {
        let errors = hits("pub use std::fmt;\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_pub_use_leading_double_colon_std_fires() {
        let errors = hits("pub use ::std::fmt;\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn an_extern_crate_std_fires() {
        let errors = hits("extern crate std;\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn every_hit_names_its_own_line() {
        let errors = hits("fn a() {}\nuse std::fmt;\nfn b() {}\nuse std::io;\n");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].line, 2);
        assert_eq!(errors[1].line, 4);
    }

    // --- what must stay silent ----------------------------------------

    #[test]
    fn core_and_alloc_imports_are_silent() {
        assert!(hits("use core::fmt;\nuse alloc::vec::Vec;\n").is_empty());
    }

    #[test]
    fn an_import_of_a_crate_merely_containing_std_as_a_substring_is_silent() {
        // `stdlib_shim` starts with `std` but the match requires the exact
        // `use std::` / `use ::std::` prefix, so a crate merely named
        // similarly is not caught by accident.
        assert!(hits("use stdlib_shim::helpers;\n").is_empty());
    }

    #[test]
    fn a_full_line_comment_mentioning_use_std_is_silent() {
        assert!(hits("// use std::fmt;\n").is_empty());
    }

    #[test]
    fn a_doc_comment_mentioning_use_std_is_silent() {
        assert!(hits("/// See `use std::fmt` for the std-side equivalent.\n").is_empty());
        assert!(hits("//! Consumers reach for `use std::fmt` on their own side.\n").is_empty());
    }

    #[test]
    fn a_trailing_comment_after_real_code_is_silent() {
        // The match is anchored on the trimmed line start, so `use std::`
        // appearing only after real code on the same line never matches:
        // this is the one lint of the five that a same-line trailing
        // comment cannot fool.
        assert!(hits("let x = 1; // see use std::fmt for the analogue\n").is_empty());
    }

    // --- proc-macro exemption ------------------------------------------

    #[test]
    fn a_declared_proc_macro_crate_is_skipped() {
        let fixture = LintFixture::new("use std::fmt;\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"]);
        assert!(NoStdEnforcer.check(&fixture.ctx()).is_empty());
    }

    #[test]
    fn a_crate_not_named_as_a_proc_macro_crate_is_not_exempt() {
        let fixture = LintFixture::new("use std::fmt;\n")
            .with_crate_name("acme-core", "core")
            .with_proc_macro_crates(&["acme-macros"]);
        assert_eq!(NoStdEnforcer.check(&fixture.ctx()).len(), 1);
    }

    /// `mockspace.toml`'s `lint_proc_macro_source = true` opts a repo back
    /// into running source lints against its declared proc-macro crates.
    /// `LintContext::is_proc_macro_crate` explicitly does not consider that
    /// preference (its own doc says so: callers wanting the skip decision
    /// should call `should_skip_proc_macro_source_lint`), so a lint built on
    /// the wrong method ignores the opt-in and keeps skipping regardless.
    /// This is the construction that caught it before the fix above: with
    /// `is_proc_macro_crate` this asserted `left: 0, right: 1`.
    #[test]
    fn the_opt_in_override_is_honoured_rather_than_ignored() {
        let fixture = LintFixture::new("use std::fmt;\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"])
            .with_lint_proc_macro_source(true);
        assert_eq!(
            NoStdEnforcer.check(&fixture.ctx()).len(),
            1,
            "the opt-in override should make this crate's source lintable again"
        );
    }

    // --- catalogued gaps: line-based scanning has no notion of context -

    /// `mock/crates/` is currently empty (arvo's canon is being written),
    /// so there is nothing for the engine's own crate walk to discover.
    /// That is a fact about the engine's crate-discovery step, not about
    /// this lint: `LintFixture` builds a `LintContext` directly, with no
    /// dependency on any crate existing on disk, and the lint reads only
    /// `ctx.source`. This lint is fully operative today; its "Scope" line
    /// is aspirational until the first crate exists to walk.
    #[test]
    fn the_lint_itself_needs_no_crate_tree_on_disk() {
        assert_eq!(hits("use std::fmt;\n").len(), 1);
    }

    /// Measured independently against a committed probe and control in the
    /// design round that closed alongside this suite: the match is anchored
    /// on `use` / `pub use` / `extern crate`, so a fully qualified call that
    /// never imports the path is invisible to it. `std::process::exit(0);`
    /// is real platform-dependent code this lint exists to forbid, and it
    /// passes silently. Catalogued rather than fixed: broadening the match
    /// to catch a bare `std::` occurrence anywhere on a line reopens the
    /// string-literal and trailing-comment false-positive classes the other
    /// four lints already carry, which is a real design change rather than
    /// a one-line correction.
    #[test]
    #[ignore = "catalogue: a fully qualified `std::` path used outside a \
                `use` statement (e.g. `std::process::exit(0);`) is not \
                matched at all; needs a broader, false-positive-aware match \
                or tree-sitter-based scanning to fix"]
    fn a_fully_qualified_std_path_outside_a_use_statement_escapes_detection() {
        let errors = hits("pub fn abort() {\n    std::process::exit(0);\n}\n");
        assert_eq!(
            errors.len(),
            1,
            "a direct std:: call is exactly the platform dependency this lint exists to forbid"
        );
    }

    /// A block comment is never recognised as a comment at all: the skip
    /// only tests `trimmed.starts_with("//")`. An interior line of a
    /// `/* ... */` block that happens to start with `use std::` once
    /// trimmed reads as real source. Catalogued rather than fixed: making
    /// this lint block-comment-aware needs the `ctx.tree` tree-sitter AST
    /// this lint does not use today, which is a real redesign rather than
    /// a one-line correction.
    #[test]
    #[ignore = "catalogue: block comments are not recognised, so an interior \
                line of one starting with `use std::` false-positives; needs \
                tree-sitter-aware scanning to fix"]
    fn an_interior_line_of_a_block_comment_is_wrongly_flagged() {
        let errors = hits("/*\nuse std::fmt;\n*/\n");
        assert!(
            errors.is_empty(),
            "a block-comment interior line should not read as real source"
        );
    }

    /// The line-anchored match has no notion of "inside a string literal":
    /// a raw multi-line string whose interior line happens to read exactly
    /// `use std::fmt;` once trimmed is indistinguishable, to a line scan,
    /// from the same text as real source. Catalogued rather than fixed for
    /// the same reason as the block comment case above.
    #[test]
    #[ignore = "catalogue: a raw multi-line string literal whose interior \
                line reads `use std::...;` once trimmed false-positives; \
                needs tree-sitter-aware scanning to fix"]
    fn an_interior_line_of_a_raw_string_literal_is_wrongly_flagged() {
        let errors = hits("let example = r#\"\nuse std::fmt;\n\"#;\n");
        assert!(
            errors.is_empty(),
            "text inside a raw string literal is not an import"
        );
    }
}
