//! Lint: arvo does not allocate. No `alloc`, `Vec<`, `String`, `Box<`.
//!
//! arvo crates are `#![no_std]` with no allocator. Dynamic
//! collections and heap boxes are forbidden. Use:
//!   * `[T; N]` or custom const-generic `Seq<T, N>` for arrays.
//!   * Consumer-owned buffers passed in by reference.
//!
//! The lint reads the file's code and nothing else. The `alloc` crate is asked
//! of the one parse the engine holds (`crate_root_paths`): a `use`, a path
//! written out in full (`alloc::rc::Rc::new`, `alloc::vec![1]`), an
//! `extern crate` whatever precedes it or is aliased to, and a path in a
//! macro's arguments, so `extern crate allocator;` is not a hit. The type
//! tokens are read from the source as `code_only_source` returns it, with
//! every comment and string literal blanked. Over that it scans
//! every line for tokens, matched at an identifier boundary so
//! `SmallVec<T, N>` does not read as `Vec<T>` and `String::from` at
//! column zero (no leading whitespace) is not missed the way a bare
//! `" String"` substring search would miss it.
//!
//! A construction that names no generic argument (`Vec::new()`,
//! `Box::new(x)`) is the forbidden shape as much as `Vec<u8>` is, so the
//! path `Vec::` and `Box::` reads as a hit too. That match is safe only
//! because comments and strings are already blanked: a comment or a
//! message that spells the path is not code.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

use crate::code_only_source::{contains_after_identifier_boundary, contains_whole_word};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(NoAllocEnforcer)
}

struct NoAllocEnforcer;

impl Lint for NoAllocEnforcer {
    fn name(&self) -> &'static str {
        "no-alloc-enforcer"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl CrateLint for NoAllocEnforcer {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        // `is_proc_macro_crate` does not consider the `lint_proc_macro_source`
        // opt-in (its own doc says so); this is the "should I skip this
        // source lint" decision, which is what that method exists for.
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }

        let mut errors = Vec::new();

        let code = crate::code_only_source::code_only(ctx);
        // The `alloc` crate, asked of the parse: a `use`, a path written out in
        // full (`alloc::rc::Rc::new`, `alloc::vec![1]`), an `extern crate`
        // whatever precedes it or is aliased to, and a path in a macro's
        // arguments. A line naming it is reported once and skips the token scan.
        let alloc_lines = crate::crate_root_paths::lines_naming(ctx, "alloc");
        for (idx, (line, code_line)) in ctx.source().lines().zip(code.lines()).enumerate() {
            if alloc_lines.binary_search(&(idx + 1)).is_ok() {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-alloc-enforcer",
                    format!(
                        "arvo forbids `alloc::*`; use const-sized containers instead: {}",
                        line.trim()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            // Type-position Vec / String / Box, and the path a construction
            // with no generic argument is written with. Hits both imports
            // (`use core::vec::Vec`) and field/return types (`Vec<`).
            let bad_tokens: [(bool, &str); 3] = [
                (
                    contains_after_identifier_boundary(code_line, "Vec<")
                        || contains_after_identifier_boundary(code_line, "Vec::"),
                    "Vec<T>",
                ),
                (contains_whole_word(code_line, "String"), "String"),
                (
                    contains_after_identifier_boundary(code_line, "Box<")
                        || contains_after_identifier_boundary(code_line, "Box::"),
                    "Box<T>",
                ),
            ];
            for (matched, display) in bad_tokens {
                if matched {
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        idx + 1,
                        "no-alloc-enforcer",
                        format!(
                            "arvo forbids `{}` (no alloc); use `[T; N]` or a consumer-owned buffer: {}",
                            display,
                            line.trim_end()
                        ),
                        Severity::HARD_ERROR,
                    ));
                    break;
                }
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
        NoAllocEnforcer.check(&fixture.ctx())
    }

    // --- registration and severity -----------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(NoAllocEnforcer.name(), "no-alloc-enforcer");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&NoAllocEnforcer);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered("no-alloc-enforcer");
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors = hits("use alloc::vec::Vec;\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &NoAllocEnforcer,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    // --- the alloc import shapes ---------------------------------------

    #[test]
    fn a_bare_use_alloc_fires() {
        assert_eq!(hits("use alloc::vec::Vec;\n").len(), 1);
    }

    #[test]
    fn a_leading_double_colon_use_alloc_fires() {
        // Missing in the original: only `use alloc::` was matched, so
        // `use ::alloc::...` (the leading-`::` spelling `no-std-enforcer`
        // already handles for `std`) escaped detection entirely.
        assert_eq!(hits("use ::alloc::boxed::Box;\n").len(), 1);
    }

    #[test]
    fn a_pub_use_alloc_fires() {
        assert_eq!(hits("pub use alloc::string::String;\n").len(), 1);
    }

    #[test]
    fn a_pub_use_leading_double_colon_alloc_fires() {
        assert_eq!(hits("pub use ::alloc::vec::Vec;\n").len(), 1);
    }

    #[test]
    fn an_extern_crate_alloc_fires() {
        assert_eq!(hits("extern crate alloc;\n").len(), 1);
    }

    // --- type-position Vec / String / Box -------------------------------

    #[test]
    fn a_vec_type_position_fires() {
        let errors = hits("let v: Vec<u8> = buf;\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("Vec<T>"));
    }

    #[test]
    fn a_box_type_position_fires() {
        let errors = hits("fn f() -> Box<dyn Trait> {}\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("Box<T>"));
    }

    #[test]
    fn a_string_type_position_fires() {
        let errors = hits("let s: String = owned;\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("String"));
    }

    #[test]
    fn only_the_first_bad_token_on_a_line_is_reported() {
        // Vec and String on the same line: the loop breaks on the first
        // match, so this asserts one finding rather than two.
        let errors = hits("fn f(v: Vec<String>) {}\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn every_hit_names_its_own_line() {
        let errors = hits("fn a() {}\nlet v: Vec<u8> = x;\nfn b() {}\nlet s: String = y;\n");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].line, 2);
        assert_eq!(errors[1].line, 4);
    }

    /// Measured independently against a committed probe and control in
    /// the design round that closed alongside this suite: the original
    /// token was the literal `" String"` (with a leading space), so
    /// `String::from(...)` sitting at column zero, with no leading
    /// whitespace on the line at all, had no space to find and passed
    /// silently. The word-boundary rewrite above closes it: the check is
    /// "not preceded by an identifier byte", and there being no
    /// character at all before column zero satisfies that.
    #[test]
    fn a_column_zero_string_construction_is_caught() {
        let errors = hits("String::from(\"x\");\n");
        assert_eq!(
            errors.len(),
            1,
            "a String construction with no leading whitespace must still be caught"
        );
    }

    // --- what must stay silent ------------------------------------------

    #[test]
    fn core_and_fixed_size_arrays_are_silent() {
        assert!(hits("use core::fmt;\nlet a: [u8; 4] = [0; 4];\n").is_empty());
    }

    #[test]
    fn a_full_line_comment_mentioning_the_tokens_is_silent() {
        assert!(hits("// Vec<T>, String and Box<T> are all forbidden here\n").is_empty());
    }

    #[test]
    fn a_doc_comment_mentioning_the_tokens_is_silent() {
        assert!(hits("/// Returns a `Vec<T>` when the caller owns the buffer.\n").is_empty());
        assert!(hits("//! A `String` here would defeat the whole point.\n").is_empty());
    }

    /// A type merely starting with one of the forbidden words, at either
    /// end, is not the forbidden type: `SmallVec<T, N>` is not `Vec<T>`,
    /// `ToolBox<T>` is not `Box<T>`, and `StringId` / `OwnedString` are
    /// not `String`. This is exactly what the identifier-boundary rewrite
    /// above exists to fix, over the bare substring search that used to
    /// be here.
    #[test]
    fn identifiers_merely_containing_a_forbidden_word_are_silent() {
        assert!(hits("struct SmallVec<T, const N: usize>([T; N]);\n").is_empty());
        assert!(hits("struct ToolBox<T>(T);\n").is_empty());
        assert!(hits("type StringId = u32;\n").is_empty());
        assert!(hits("struct OwnedString;\n").is_empty());
    }

    // --- proc-macro exemption --------------------------------------------

    #[test]
    fn a_declared_proc_macro_crate_is_skipped() {
        let fixture = LintFixture::new("let v: Vec<u8> = x;\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"]);
        assert!(NoAllocEnforcer.check(&fixture.ctx()).is_empty());
    }

    #[test]
    fn the_opt_in_override_is_honoured_rather_than_ignored() {
        let fixture = LintFixture::new("let v: Vec<u8> = x;\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"])
            .with_lint_proc_macro_source(true);
        assert_eq!(
            NoAllocEnforcer.check(&fixture.ctx()).len(),
            1,
            "the opt-in override should make this crate's source lintable again"
        );
    }

    // --- what only the parse can tell apart --------------------------------

    /// A trailing comment on a line of real code names the type without using
    /// it. The control below is the same line with the type written in the
    /// code part, which has to keep firing, so a lint that went quiet on every
    /// line carrying a comment would not pass.
    #[test]
    fn a_trailing_comment_after_real_code_is_silent() {
        assert!(hits("let n = 1; // returns a Box<T> eventually\n").is_empty());
    }

    #[test]
    fn a_trailing_comment_does_not_hide_the_code_before_it() {
        assert_eq!(hits("let v: Vec<u8> = x; // fine\n").len(), 1);
    }

    /// Text inside a string literal is not a use of the type. The control is
    /// the type written in code on the same line as a string.
    #[test]
    fn a_string_literal_naming_a_forbidden_type_is_silent() {
        assert!(hits("let msg = \"expected a Vec<T> here\";\n").is_empty());
    }

    #[test]
    fn a_string_literal_does_not_hide_the_code_beside_it() {
        assert_eq!(hits("let v: Vec<u8> = f(\"a\");\n").len(), 1);
    }

    #[test]
    fn a_raw_multi_line_string_naming_a_forbidden_type_is_silent() {
        assert!(hits("let doc = r#\"\nlet v: Vec<u8> = x;\n\"#;\n").is_empty());
    }

    #[test]
    fn a_block_comment_interior_line_naming_a_forbidden_type_is_silent() {
        assert!(hits("/*\nlet v: Vec<u8> = x;\n*/\n").is_empty());
    }

    /// A construction with no generic argument carries neither `Vec<` nor
    /// `Box<`, so a match on those two alone never saw it. `let v = Vec::new();
    /// v.push(1);` never writes the type out.
    #[test]
    fn a_type_inferred_construction_with_no_annotation_fires() {
        let errors = hits("let v = Vec::new();\n");
        assert_eq!(
            errors.len(),
            1,
            "Vec::new() is exactly the heap allocation this lint exists to forbid"
        );
        assert!(errors[0].message.contains("Vec<T>"));
        let errors = hits("let b = Box::new(x);\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("Box<T>"));
    }

    /// The controls for the path match: a name that merely ends in a forbidden
    /// word, and a comment or a message that spells the path.
    #[test]
    fn a_path_through_a_lookalike_type_is_silent() {
        assert!(hits("let v = SmallVec::new();\n").is_empty());
        assert!(hits("let t = ToolBox::new(x);\n").is_empty());
        assert!(hits("// build it with Vec::new() when the caller owns it\n").is_empty());
        assert!(hits("let m = \"call Box::new here\";\n").is_empty());
    }

    // --- the class, not the instance: what a line scan keyed on how a line begins missed ---

    #[test]
    fn a_pub_extern_crate_alloc_fires_and_an_alias_of_it_does_too() {
        assert_eq!(hits("pub extern crate alloc;\n").len(), 1);
        assert_eq!(hits("pub extern crate alloc as a;\n").len(), 1);
    }

    #[test]
    fn a_path_written_out_in_full_fires_without_any_use() {
        assert_eq!(hits("fn f() { let _ = alloc::rc::Rc::new(1); }\n").len(), 1);
        assert_eq!(
            hits("fn f() { let _ = alloc::sync::Arc::new(1); }\n").len(),
            1
        );
        assert_eq!(hits("fn f() { let _ = alloc::vec![1]; }\n").len(), 1);
    }

    #[test]
    fn a_path_through_an_alias_fires() {
        let errors = hits("extern crate alloc as a;\nfn f() { let _ = a::rc::Rc::new(1); }\n");
        assert_eq!(errors.len(), 2, "the declaration and the path through it");
    }

    #[test]
    fn control_a_different_crate_with_the_prefix_is_silent() {
        assert!(hits("extern crate allocator;\nuse allocator::Pool;\n").is_empty());
        assert!(hits("fn f() { facade::alloc::x(); my_alloc::y(); }\n").is_empty());
    }
}
