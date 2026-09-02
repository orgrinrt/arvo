//! Lint: arvo does not allocate. No `alloc`, `Vec<`, `String`, `Box<`.
//!
//! arvo crates are `#![no_std]` with no allocator. Dynamic
//! collections and heap boxes are forbidden. Use:
//!   * `[T; N]` or custom const-generic `Seq<T, N>` for arrays.
//!   * Consumer-owned buffers passed in by reference.
//!
//! The lint scans every line for tokens, matched at an identifier
//! boundary so `SmallVec<T, N>` does not read as `Vec<T>` and
//! `String::from` at column zero (no leading whitespace) is not
//! missed the way a bare `" String"` substring search would miss it.
//!
//! **Known gaps**, catalogued as red tests below rather than fixed:
//! a trailing comment on a line of real code and a string literal
//! naming one of these types both still false-positive, because the
//! scanner has no notion of "inside a comment" or "inside a string"
//! beyond a whole-line `//` prefix; and a value-position construction
//! with no type annotation (`Vec::new()`, `Box::new(x)`) still escapes
//! detection, because nothing after `Vec` or `Box` other than `<`
//! is read as the forbidden shape.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

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

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Whether `line` contains `token` at an identifier boundary on its
/// left side, with nothing checked on its right.
///
/// Correct for a token that already ends on a non-identifier character
/// (`Vec<`, `Box<`): `SmallVec<T, N>` does not match `Vec<` because the
/// byte before the match (`c` of `Small`) is an identifier byte, while
/// `let v: Vec<u8>` does match because the byte before it is a space.
/// Nothing needs checking on the right, because the token's own
/// trailing `<` is already not an identifier byte.
fn contains_after_identifier_boundary(line: &str, token: &str) -> bool {
    let bytes = line.as_bytes();
    let mut search_from = 0;
    while let Some(rel) = line[search_from..].find(token) {
        let start = search_from + rel;
        let before_is_ident = start > 0 && is_ident_byte(bytes[start - 1]);
        if !before_is_ident {
            return true;
        }
        search_from = start + 1;
    }
    false
}

/// Whether `line` contains `token` as a whole identifier: not preceded
/// and not followed by an identifier byte.
///
/// `StringBuilder` and `OwnedString` do not match `String`, because one
/// side or the other is an identifier byte. `String::from(...)` sitting
/// at column zero, with nothing before it on the line at all, does
/// match, which a bare `.contains(" String")` search misses because
/// there is no leading space to find.
fn contains_whole_word(line: &str, token: &str) -> bool {
    let bytes = line.as_bytes();
    let mut search_from = 0;
    while let Some(rel) = line[search_from..].find(token) {
        let start = search_from + rel;
        let end = start + token.len();
        let before_is_ident = start > 0 && is_ident_byte(bytes[start - 1]);
        let after_is_ident = end < bytes.len() && is_ident_byte(bytes[end]);
        if !before_is_ident && !after_is_ident {
            return true;
        }
        search_from = start + 1;
    }
    false
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

        for (idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim_start();

            if trimmed.starts_with("//") {
                continue;
            }

            // `use alloc::*` / `use ::alloc::*` / `extern crate alloc`.
            if trimmed.starts_with("use alloc::")
                || trimmed.starts_with("use ::alloc::")
                || trimmed.starts_with("pub use alloc::")
                || trimmed.starts_with("pub use ::alloc::")
                || trimmed.starts_with("extern crate alloc")
            {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-alloc-enforcer",
                    format!(
                        "arvo forbids `alloc::*`; use const-sized containers instead: {}",
                        trimmed.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            // Type-position Vec / String / Box. Hits both imports
            // (`use core::vec::Vec`) and field/return types (`Vec<`).
            let line_body = line;
            let bad_tokens: [(bool, &str); 3] = [
                (
                    contains_after_identifier_boundary(line_body, "Vec<"),
                    "Vec<T>",
                ),
                (contains_whole_word(line_body, "String"), "String"),
                (
                    contains_after_identifier_boundary(line_body, "Box<"),
                    "Box<T>",
                ),
            ];
            for (matched, display) in bad_tokens {
                if matched {
                    // Skip doc comments that only mention the type.
                    if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                        continue;
                    }
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        idx + 1,
                        "no-alloc-enforcer",
                        format!(
                            "arvo forbids `{}` (no alloc); use `[T; N]` or a consumer-owned buffer: {}",
                            display,
                            line_body.trim_end()
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

    // --- catalogued gaps ---------------------------------------------------

    /// A trailing comment on a line of real code: the scanner has no
    /// notion of "the rest of this line is a comment", only "the whole
    /// trimmed line starts with `//`". Catalogued rather than fixed: it
    /// needs the `ctx.tree` tree-sitter AST this lint does not use today.
    #[test]
    #[ignore = "catalogue: a trailing `//` comment on a real line of code \
                is scanned as code; `let n = 1; // returns a Box<T> \
                eventually` false-positives; needs tree-sitter-aware \
                scanning to fix"]
    fn a_trailing_comment_after_real_code_is_wrongly_flagged() {
        let errors = hits("let n = 1; // returns a Box<T> eventually\n");
        assert!(
            errors.is_empty(),
            "a trailing comment mentioning the type is not a use of it"
        );
    }

    /// The module doc used to claim string literals are tolerated
    /// "because the scan only fires on import-like or type-position
    /// usage"; the code never implemented that, it scanned the whole
    /// line unconditionally. Catalogued rather than fixed for the same
    /// reason as the trailing comment above.
    #[test]
    #[ignore = "catalogue: a string literal naming one of the forbidden \
                types false-positives; `let msg = \"expected a Vec<T> \
                here\";` fires; needs tree-sitter-aware scanning to fix"]
    fn a_string_literal_naming_a_forbidden_type_is_wrongly_flagged() {
        let errors = hits("let msg = \"expected a Vec<T> here\";\n");
        assert!(
            errors.is_empty(),
            "text inside a string literal is not a use of the type"
        );
    }

    /// A value-position construction with no type annotation carries
    /// neither `Vec<` nor `Box<` anywhere on the line, so it is invisible
    /// to a lint whose whole match is those two literal substrings. This
    /// is a real, if narrower, escape: `let v = Vec::new(); v.push(1);`
    /// never writes the type out. Catalogued rather than fixed: closing
    /// it means matching `Vec::` / `Box::` too, which reopens the same
    /// string-literal and trailing-comment classes above for an even
    /// more common token.
    #[test]
    #[ignore = "catalogue: a type-inferred construction with no explicit \
                generic annotation (`Vec::new()`, `Box::new(x)`) is \
                invisible to this lint; needs a broader match or \
                tree-sitter-aware scanning to fix"]
    fn a_type_inferred_construction_with_no_annotation_escapes_detection() {
        let errors = hits("let v = Vec::new();\n");
        assert_eq!(
            errors.len(),
            1,
            "Vec::new() is exactly the heap allocation this lint exists to forbid"
        );
    }
}
