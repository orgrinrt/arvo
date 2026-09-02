//! Lint: arvo never pays for dynamic dispatch.
//!
//! No `dyn Trait`, no `TypeId`, no `std::any::*`. Monomorphisation
//! IS the dispatch. Generic code is instantiated at the use site;
//! the compiler proves devirtualisation and we never pay for runtime
//! lookup.
//!
//! This catches three shapes:
//!   * ` dyn ` (with surrounding whitespace to avoid matching
//!     identifiers like `dyndns` or `already_done`).
//!   * `TypeId`, matched at an identifier boundary so `NodeTypeId`
//!     does not read as a use of `core::any::TypeId`.
//!   * `std::any` and `core::any` paths, matched so what follows
//!     `any` is not itself an identifier character, so `core::anything`
//!     does not read as `core::any`.
//!
//! **Known gaps**, catalogued as red tests below rather than fixed: a
//! trailing comment, a string literal, and a block comment interior
//! line all still false-positive on all three shapes, the same class
//! of gap `no-std-enforcer` and `no-alloc-enforcer` carry, for the same
//! reason: no notion of "inside a comment" or "inside a string" beyond
//! a whole-line `//` prefix.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(NoDynamicDispatch)
}

struct NoDynamicDispatch;

impl Lint for NoDynamicDispatch {
    fn name(&self) -> &'static str {
        "no-dynamic-dispatch"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Whether `line` contains `token` as a whole identifier or path: not
/// preceded and not followed by an identifier byte.
///
/// `NodeTypeId` does not match `TypeId`, and `core::anything` does not
/// match `core::any`, because the byte immediately after the match is
/// an identifier byte in both cases.
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

impl CrateLint for NoDynamicDispatch {
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

            // Skip comments wholesale.
            if trimmed.starts_with("//") {
                continue;
            }

            // ` dyn ` (space-delimited to avoid false positives on `dyn`
            // as part of an identifier).
            let has_dyn = line.contains(" dyn ")
                || line.contains("&dyn ")
                || line.contains("&'static dyn ")
                || line.contains("<dyn ")
                || line.contains("(dyn ")
                || line.ends_with(" dyn")
                || line.trim() == "dyn";

            if has_dyn {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "arvo forbids `dyn Trait`; make the function generic over the trait bound instead: {}",
                        line.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            if contains_whole_word(line, "TypeId") {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "arvo forbids `TypeId`; the monomorphised function IS the type identity: {}",
                        line.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            if contains_whole_word(line, "std::any") || contains_whole_word(line, "core::any") {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "arvo forbids `std::any` / `core::any`; no runtime type erasure: {}",
                        line.trim_end()
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
        NoDynamicDispatch.check(&fixture.ctx())
    }

    // --- registration and severity -----------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(NoDynamicDispatch.name(), "no-dynamic-dispatch");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&NoDynamicDispatch);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered("no-dynamic-dispatch");
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors = hits("fn f(t: &dyn Trait) {}\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &NoDynamicDispatch,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    // --- the dyn shapes --------------------------------------------------

    #[test]
    fn a_reference_to_dyn_fires() {
        assert_eq!(hits("fn f(t: &dyn Trait) {}\n").len(), 1);
    }

    #[test]
    fn a_static_dyn_reference_fires() {
        assert_eq!(hits("fn f(t: &'static dyn Trait) {}\n").len(), 1);
    }

    #[test]
    fn a_dyn_in_a_generic_position_fires() {
        assert_eq!(hits("let b: Box<dyn Trait>;\n").len(), 1);
    }

    #[test]
    fn a_dyn_in_a_paren_position_fires() {
        assert_eq!(hits("fn f() -> (dyn Trait) {}\n").len(), 1);
    }

    #[test]
    fn a_trailing_bare_dyn_fires() {
        assert_eq!(hits("impl dyn\n").len(), 1);
    }

    #[test]
    fn a_lone_dyn_line_fires() {
        assert_eq!(hits("dyn\n").len(), 1);
    }

    // --- TypeId and std::any / core::any -----------------------------

    #[test]
    fn a_bare_typeid_fires() {
        let errors = hits("let id = core::any::TypeId::of::<u8>();\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("TypeId"));
    }

    #[test]
    fn a_std_any_path_fires() {
        assert_eq!(hits("use std::any::Any;\n").len(), 1);
    }

    #[test]
    fn a_core_any_path_fires() {
        assert_eq!(hits("use core::any::Any;\n").len(), 1);
    }

    #[test]
    fn only_one_finding_per_line_even_with_two_shapes() {
        // dyn is checked first and `continue`s, so a line carrying both
        // `dyn` and `TypeId` reports once.
        let errors = hits("fn f(t: &dyn core::any::Any) {}\n");
        assert_eq!(errors.len(), 1);
    }

    // --- what must stay silent ----------------------------------------

    #[test]
    fn generic_bounds_are_silent() {
        assert!(hits("fn f<T: Trait>(t: &T) {}\n").is_empty());
    }

    #[test]
    fn dyn_as_part_of_an_identifier_is_silent() {
        // The design's own stated exemption: `dyndns` and
        // `already_done` must not match the space-delimited `dyn`.
        assert!(hits("fn dyndns_lookup() {}\n").is_empty());
        assert!(hits("let already_done = true;\n").is_empty());
    }

    #[test]
    fn a_full_line_comment_mentioning_the_forbidden_shapes_is_silent() {
        assert!(hits("// no dyn Trait, no TypeId, no std::any here\n").is_empty());
    }

    /// A type merely containing `TypeId` or `any` as a substring of a
    /// longer identifier or path is not the forbidden shape:
    /// `NodeTypeId` is not `core::any::TypeId`, and `core::anything` is
    /// not `core::any`. This is exactly what the identifier-boundary
    /// rewrite above exists to fix, over the bare substring search that
    /// used to be here.
    #[test]
    fn identifiers_merely_containing_a_forbidden_word_are_silent() {
        assert!(hits("struct NodeTypeId;\n").is_empty());
        assert!(hits("let x: core::anything::Foo = default();\n").is_empty());
    }

    #[test]
    fn the_forbidden_shapes_still_fire_inside_a_macro_body() {
        // No cfg(test) or macro exemption is documented anywhere, and
        // none is implemented: the arvo constraints are absolute, and a
        // macro body is still source the compiler emits.
        let errors = hits("macro_rules! m {\n    () => { let x: &dyn Trait = y; };\n}\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn the_forbidden_shapes_still_fire_inside_a_cfg_test_module() {
        let errors = hits("#[cfg(test)]\nmod tests {\n    use core::any::TypeId;\n}\n");
        assert_eq!(errors.len(), 1);
    }

    // --- proc-macro exemption --------------------------------------------

    #[test]
    fn a_declared_proc_macro_crate_is_skipped() {
        let fixture = LintFixture::new("fn f(t: &dyn Trait) {}\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"]);
        assert!(NoDynamicDispatch.check(&fixture.ctx()).is_empty());
    }

    #[test]
    fn the_opt_in_override_is_honoured_rather_than_ignored() {
        let fixture = LintFixture::new("fn f(t: &dyn Trait) {}\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"])
            .with_lint_proc_macro_source(true);
        assert_eq!(
            NoDynamicDispatch.check(&fixture.ctx()).len(),
            1,
            "the opt-in override should make this crate's source lintable again"
        );
    }

    // --- catalogued gaps ---------------------------------------------------

    #[test]
    #[ignore = "catalogue: a trailing `//` comment on a real line of code \
                is scanned as code; `let x = 5; // avoid dyn dispatch \
                here` false-positives; needs tree-sitter-aware scanning \
                to fix"]
    fn a_trailing_comment_after_real_code_is_wrongly_flagged() {
        let errors = hits("let x = 5; // avoid dyn dispatch here\n");
        assert!(
            errors.is_empty(),
            "a trailing comment mentioning dyn is not a use of it"
        );
    }

    #[test]
    #[ignore = "catalogue: a string literal naming one of the forbidden \
                shapes false-positives; needs tree-sitter-aware scanning \
                to fix"]
    fn a_string_literal_naming_a_forbidden_shape_is_wrongly_flagged() {
        let errors = hits("let msg = \"expected dyn Trait here\";\n");
        assert!(
            errors.is_empty(),
            "text inside a string literal is not a use of the shape"
        );
    }

    #[test]
    #[ignore = "catalogue: a block-comment interior line naming one of \
                the forbidden shapes false-positives; needs \
                tree-sitter-aware scanning to fix"]
    fn an_interior_line_of_a_block_comment_is_wrongly_flagged() {
        let errors = hits("/*\nlet t: &dyn Trait = x;\n*/\n");
        assert!(
            errors.is_empty(),
            "a block-comment interior line should not read as real source"
        );
    }
}
