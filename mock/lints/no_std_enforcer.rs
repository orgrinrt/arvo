//! Lint: arvo is `#![no_std]`. Any `use std::` import is a violation.
//!
//! arvo is a numeric + analysis substrate with zero platform
//! dependency. Crates must not import `std::*`. Consumers that need
//! platform integration pull in their own `std`-bearing crates.
//!
//! Scope: every arvo crate under `mock/crates/`. The lint scans the
//! file's code (the source as `code_only_source` returns it, with every
//! comment and string literal blanked out of the one parse the engine
//! holds) to catch the path before it reaches the compile pipeline,
//! which gives a clear arvo-specific message rather than a generic
//! `unresolved import` error from `no_std` mode.
//!
//! The path is forbidden wherever it is written: in a `use`, and written
//! out in full in an expression or a type, as `std::process::exit(0);` is.
//! A `std` that is not the crate root does not match: `my_std::x` ends in
//! an identifier byte before it, and `facade::std::x` has a path segment
//! before it. A leading `::std::` is the crate root and does.

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

        let code = crate::code_only_source::code_only(ctx);
        for (idx, (line, code_line)) in ctx.source().lines().zip(code.lines()).enumerate() {
            // Match a `std::` path wherever it is written, and `extern crate std`.
            let hit = names_the_std_crate(code_line)
                || code_line.trim_start().starts_with("extern crate std");

            if hit {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-std-enforcer",
                    format!(
                        "arvo is `#![no_std]`; a `std::` path is forbidden, in a `use` or written \
                         out in full: {}",
                        line.trim()
                    ),
                    Severity::HARD_ERROR,
                ));
            }
        }

        errors
    }
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Whether a line of code writes a path rooted at the `std` crate.
///
/// `std::` counts when no identifier byte stands before it (so `my_std::x` and
/// `ministd::x` do not), and when it is not a segment of a longer path: a `::`
/// before it is the crate root only where nothing that is part of a path stands
/// before that (`::std::fmt` does, `facade::std::fmt` does not).
fn names_the_std_crate(code_line: &str) -> bool {
    let bytes = code_line.as_bytes();
    let mut from = 0;
    while let Some(rel) = code_line[from ..].find("std::") {
        let start = from + rel;
        from = start + 1;
        if start > 0 && is_ident_byte(bytes[start - 1]) {
            continue;
        }
        if start >= 2 && &bytes[start - 2 .. start] == b"::" {
            let before_colons = start.checked_sub(3).map(|i| bytes[i]);
            if before_colons.is_some_and(is_ident_byte) {
                continue;
            }
        }
        return true;
    }
    false
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
        assert!(hits("let x = 1; // see use std::fmt for the analogue\n").is_empty());
        assert!(hits("let x = 1; // std::process::exit(0) is not allowed\n").is_empty());
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

    // --- what only the parse can tell apart --------------------------------

    /// `mock/crates/` is currently empty (arvo's canon is being written),
    /// so there is nothing for the engine's own crate walk to discover.
    /// That is a fact about the engine's crate-discovery step, not about
    /// this lint: `LintFixture` builds a `LintContext` directly, with no
    /// dependency on any crate existing on disk, and the lint reads only
    /// `ctx.source()` and `ctx.tree()`. This lint is fully operative today;
    /// its "Scope" line is aspirational until the first crate exists to walk.
    #[test]
    fn the_lint_itself_needs_no_crate_tree_on_disk() {
        assert_eq!(hits("use std::fmt;\n").len(), 1);
    }

    /// `std::process::exit(0);` is real platform-dependent code this lint
    /// exists to forbid, and it never imports the path. The controls are the
    /// shapes that look like it and are not the crate root.
    #[test]
    fn a_fully_qualified_std_path_outside_a_use_statement_fires() {
        let errors = hits("pub fn abort() {\n    std::process::exit(0);\n}\n");
        assert_eq!(
            errors.len(),
            1,
            "a direct std:: call is exactly the platform dependency this lint exists to forbid"
        );
        assert_eq!(errors[0].line, 2);
    }

    #[test]
    fn a_fully_qualified_std_path_in_a_type_or_a_leading_colon_path_fires() {
        assert_eq!(hits("fn f(x: std::fmt::Arguments) {}\n").len(), 1);
        assert_eq!(hits("let x = ::std::mem::size_of::<T>();\n").len(), 1);
        assert_eq!(hits("let x = (1, std::cmp::max(a, b));\n").len(), 1);
    }

    #[test]
    fn a_path_that_only_has_std_inside_it_is_silent() {
        assert!(hits("let x = my_std::helper();\n").is_empty());
        assert!(hits("let x = facade::std::helper();\n").is_empty());
        assert!(hits("let x = ministd::helper();\n").is_empty());
        assert!(hits("fn std_like() {}\n").is_empty());
    }

    #[test]
    fn a_std_path_in_a_string_or_a_comment_is_silent() {
        assert!(hits("let m = \"call std::process::exit here\";\n").is_empty());
        assert!(hits("// std::process::exit(0) is forbidden\n").is_empty());
    }

    #[test]
    fn a_comment_does_not_hide_the_std_path_before_it() {
        assert_eq!(hits("std::process::exit(0); // gone\n").len(), 1);
    }

    #[test]
    fn an_interior_line_of_a_block_comment_is_silent() {
        assert!(hits("/*\nuse std::fmt;\n*/\n").is_empty());
    }

    #[test]
    fn an_interior_line_of_a_raw_string_literal_is_silent() {
        assert!(hits("let example = r#\"\nuse std::fmt;\n\"#;\n").is_empty());
    }
}
