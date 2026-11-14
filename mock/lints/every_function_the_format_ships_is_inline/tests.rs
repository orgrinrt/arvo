//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `every-function-the-format-ships-is-inline`.

use mockspace::testkit::LintFixture;

use super::*;

/// The shapes the crate ships, each marked as the lint wants.
const SHIPPED: &str = "\
impl Width {
    /// How many bits.
    #[must_use]
    #[inline]
    pub const fn count(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub(crate) const fn at(at: i128) -> Self {
        Self(at)
    }
}

#[must_use]
#[inline]
pub const fn cancelling_slot<F: Format>(m: Magnitude) -> Maybe<Slot> {
    Maybe::Isnt
}

#[inline]
pub const fn wide() -> UFixed<{ ::core::primitive::usize::BITS }, 0> {
    todo()
}

#[inline]
const fn helper(a: Slot) -> Slot {
    a
}

impl Clone for Divisor {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

pub trait Format {
    fn required(&self) -> Slot;
}

#[cfg(test)]
pub(crate) const fn only_for_tests() -> Slot {
    Slot::ZERO
}

pub type Pointer = fn(Slot) -> Slot;
";

fn hits_in(path: &str, source: &str) -> Vec<LintError> {
    let fixture = LintFixture::new(source)
        .with_crate_name("arvo-format", "format")
        .with_sources(&[(path, source)]);
    EveryFunctionTheFormatShipsIsInline.check(&fixture.ctx())
}

fn hits(source: &str) -> Vec<LintError> {
    hits_in("src/lib.rs", source)
}

/// The lines the findings name, in order.
fn lines(source: &str) -> Vec<usize> {
    hits(source).iter().map(|e| e.line).collect()
}

#[test]
fn it_answers_to_the_name_the_gate_and_the_config_use() {
    assert_eq!(EveryFunctionTheFormatShipsIsInline.name(), NAME);
    assert_eq!(NAME, "every-function-the-format-ships-is-inline");
}

#[test]
fn it_is_not_declared_off_so_it_runs_at_all() {
    crate::canon_lint_testkit::assert_not_declared_off(&EveryFunctionTheFormatShipsIsInline);
}

#[test]
fn it_reaches_the_pack_the_engine_is_handed() {
    crate::crate_lint_testkit::assert_registered(NAME);
}

#[test]
fn its_findings_carry_its_own_declared_severity() {
    let errors = hits("pub const fn count() -> u32 {\n    1\n}\n");
    crate::crate_lint_testkit::assert_findings_carry(
        &EveryFunctionTheFormatShipsIsInline,
        &errors,
        Severity::HARD_ERROR,
    );
}

#[test]
fn the_control_the_shipped_spelling_is_silent() {
    assert!(hits(SHIPPED).is_empty(), "{:?}", hits(SHIPPED));
}

#[test]
fn a_function_with_no_mark_fires_at_the_line_its_item_starts() {
    let source = "\
/// Doc.
#[must_use]
pub(crate) const fn count(self) -> u32 {
    self.0
}
";
    assert_eq!(lines(source), vec![3]);
    assert!(hits(source)[0].message.contains("`fn count`"));
}

#[test]
fn each_qualifier_form_fires_without_its_mark() {
    for source in [
        "fn a() {}\n",
        "pub fn a() {}\n",
        "pub(crate) fn a() {}\n",
        "pub(super) const fn a() {}\n",
        "const unsafe fn a() {}\n",
        "pub extern \"C\" fn a() {}\n",
        "impl X {\n    pub const fn a(self) -> Self {\n        self\n    }\n}\n",
        "impl Clone for X {\n    fn clone(&self) -> Self {\n        *self\n    }\n}\n",
        "pub trait T {\n    fn provided(&self) -> u32 {\n        1\n    }\n}\n",
    ] {
        assert_eq!(hits(source).len(), 1, "{source}");
    }
}

#[test]
fn inline_never_fires_and_says_so() {
    let errors = hits("#[inline(never)]\npub const fn a() {}\n");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("inline(never)"));
}

#[test]
fn a_mark_on_another_item_does_not_count() {
    let source = "#[inline]\npub const fn a() {}\n\npub const fn b() {}\n";
    assert_eq!(lines(source), vec![4]);
}

#[test]
fn a_mark_in_a_comment_is_not_a_mark() {
    let source = "// #[inline]\npub const fn a() {}\n/* #[inline] */\npub const fn b() {}\n";
    assert_eq!(lines(source), vec![2, 4]);
}

#[test]
fn a_signature_with_a_const_generic_argument_still_has_a_body() {
    let source = "pub const fn a() -> UFixed<{ W }, 0> {\n    todo()\n}\n";
    assert_eq!(lines(source), vec![1]);
}

#[test]
fn a_where_clause_does_not_hide_the_body() {
    let source = "pub const fn a<F>() -> Slot\nwhere\n    F: Format<X = Y>,\n{\n    todo()\n}\n";
    assert_eq!(lines(source), vec![1]);
}

#[test]
fn a_required_trait_method_and_a_function_pointer_are_not_functions() {
    let source = "pub trait T {\n    fn required(&self) -> u32;\n}\npub type P = fn(u32) -> u32;\n";
    assert!(hits(source).is_empty(), "{:?}", hits(source));
}

#[test]
fn a_function_under_cfg_test_is_skipped() {
    let source = "#[cfg(test)]\n#[must_use]\npub(crate) const fn a() {}\n";
    assert!(hits(source).is_empty(), "{:?}", hits(source));
}

#[test]
fn a_test_module_is_not_read() {
    let source = "pub const fn a() {}\n";
    assert!(hits_in("src/apply/tests/mod.rs", source).is_empty());
    assert!(hits_in("src/apply/tests/the_edges.rs", source).is_empty());
    assert!(hits_in("src/the_platform/tests.rs", source).is_empty());
    assert_eq!(hits_in("src/tests_of_nothing.rs", source).len(), 1);
    assert_eq!(hits_in("src/apply.rs", source).len(), 1);
}

#[test]
fn another_crate_is_not_read() {
    let source = "pub const fn a() {}\n";
    let fixture = LintFixture::new(source)
        .with_crate_name("arvo-numeral", "numeral")
        .with_sources(&[("src/lib.rs", source)]);
    assert!(
        EveryFunctionTheFormatShipsIsInline
            .check(&fixture.ctx())
            .is_empty()
    );
}

#[test]
fn a_finding_names_its_file() {
    let errors = hits_in("src/apply.rs", "pub const fn a() {}\n");
    assert_eq!(errors[0].path.as_deref(), Some("src/apply.rs"));
}
