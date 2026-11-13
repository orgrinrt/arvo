//! Lint: arvo is `#![no_std]`. Any `use std::` import is a violation.
//!
//! arvo is a numeric + analysis substrate with zero platform
//! dependency. Crates must not import `std::*`. Consumers that need
//! platform integration pull in their own `std`-bearing crates.
//!
//! Scope: every arvo crate under `mock/crates/`. The question is asked of
//! the one parse the engine holds (`crate_root_paths`), which names a `std`
//! path, an `extern crate std` and a path through an alias of it by what
//! the nodes are and not by how a line begins. That gives a clear
//! arvo-specific message rather than a generic `unresolved import` error
//! from `no_std` mode.
//!
//! The path is forbidden wherever it is written: in a `use`, written out
//! in full in an expression or a type, as `std::process::exit(0);` is,
//! inside a macro's arguments, and as an `extern crate` declaration
//! whatever precedes it (`pub extern crate std;`) and whatever it is
//! aliased to. A `std` that is not the crate root does not match:
//! `my_std::x` and `facade::std::x` have other roots, and
//! `extern crate stdx;` names another crate. A leading `::std::` is the
//! crate root and does.

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

        let lines: Vec<&str> = ctx.source().lines().collect();
        crate::crate_root_paths::lines_naming(ctx, "std")
            .into_iter()
            .map(|line| {
                LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line,
                    "no-std-enforcer",
                    format!(
                        "arvo is `#![no_std]`; a `std` path is forbidden, in a `use`, written \
                         out in full, in a macro's arguments or as `extern crate`: {}",
                        lines.get(line - 1).map_or("", |l| l.trim())
                    ),
                    Severity::HARD_ERROR,
                )
            })
            .collect()
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

    // --- the class, not the instance: what a line scan keyed on how a line begins missed ---

    #[test]
    fn a_pub_extern_crate_std_fires() {
        assert_eq!(hits("pub extern crate std;\n").len(), 1);
        assert_eq!(hits("pub(crate) extern crate std;\n").len(), 1);
    }

    #[test]
    fn an_alias_of_std_fires_at_the_declaration_and_at_a_path_through_it() {
        let errors = hits("pub extern crate std as s;\nfn f() { s::process::exit(0); }\n");
        assert_eq!(errors.len(), 2, "{errors:?}");
        assert_eq!(errors[0].line, 1);
        assert_eq!(errors[1].line, 2);
    }

    #[test]
    fn a_path_in_a_macros_arguments_fires() {
        assert_eq!(hits("fn f() { m!(std::process::id()); }\n").len(), 1);
    }

    #[test]
    fn control_extern_crate_of_a_longer_name_is_silent() {
        assert!(hits("extern crate stdx;\n").is_empty());
        assert!(hits("fn f() { facade::std::x(); }\n").is_empty());
    }
}
