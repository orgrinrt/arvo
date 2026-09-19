//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `the-platform-width-points-read-the-pointer-width`, split out of
//! the lint file along the 500-line guideline. The alias-spelling arm and the
//! `core`-binding arm share this fixture and this file.

use mockspace::testkit::LintFixture;

use super::*;

/// The shipped spelling, as `points` carries it.
const SHIPPED: &str = "\
    pub type USize = UFixed<{ ::core::primitive::usize::BITS }, 0>;
    pub type ISize = Integer<{ ::core::primitive::usize::BITS }>;
";

fn hits(source: &str) -> Vec<LintError> {
    let fixture = LintFixture::new(source).with_crate_name("arvo-format", "format");
    ThePlatformWidthPointsReadThePointerWidth.check(&fixture.ctx())
}

#[test]
fn it_answers_to_the_name_the_gate_and_the_config_use() {
    assert_eq!(ThePlatformWidthPointsReadThePointerWidth.name(), NAME);
    assert_eq!(NAME, "the-platform-width-points-read-the-pointer-width");
}

#[test]
fn it_is_not_declared_off_so_it_runs_at_all() {
    crate::canon_lint_testkit::assert_not_declared_off(&ThePlatformWidthPointsReadThePointerWidth);
}

#[test]
fn it_reaches_the_pack_the_engine_is_handed() {
    crate::crate_lint_testkit::assert_registered(NAME);
}

#[test]
fn its_findings_carry_its_own_declared_severity() {
    let errors = hits(
        "pub type USize = UFixed<64, 0>;\npub type ISize = Integer<{ \
         ::core::primitive::usize::BITS }>;\n",
    );
    crate::crate_lint_testkit::assert_findings_carry(
        &ThePlatformWidthPointsReadThePointerWidth,
        &errors,
        Severity::HARD_ERROR,
    );
}

#[test]
fn the_control_the_shipped_spelling_is_silent() {
    assert!(hits(SHIPPED).is_empty(), "{:?}", hits(SHIPPED));
}

#[test]
fn a_literal_width_fires_for_each_alias() {
    for (source, alias) in [
        (
            "pub type USize = UFixed<64, 0>;\npub type ISize = Integer<{ \
             ::core::primitive::usize::BITS }>;\n",
            "USize",
        ),
        (
            "pub type USize = UFixed<{ ::core::primitive::usize::BITS }, 0>;\npub type ISize = \
             Integer<32>;\n",
            "ISize",
        ),
    ] {
        let errors = hits(source);
        assert_eq!(errors.len(), 1, "{alias}: {errors:?}");
        assert!(
            errors[0].message.contains(alias),
            "{alias}: {}",
            errors[0].message
        );
    }
    assert_eq!(
        hits("pub type USize = UFixed<16, 0>;\npub type ISize = Integer<16>;\n").len(),
        2
    );
}

#[test]
fn a_width_read_from_somewhere_else_fires() {
    // `u64::BITS` reads a width and is still a fixed one.
    let errors = hits(
        "pub type USize = UFixed<{ u64::BITS }, 0>;\npub type ISize = Integer<{ \
         ::core::primitive::usize::BITS }>;\n",
    );
    assert_eq!(errors.len(), 1);
}

#[test]
fn a_missing_alias_fires() {
    let errors = hits("pub type USize = UFixed<{ ::core::primitive::usize::BITS }, 0>;\n");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("ISize"));
    assert_eq!(hits("").len(), 2);
}

#[test]
fn a_commented_out_alias_is_not_the_alias() {
    let errors = hits(
        "// pub type USize = UFixed<{ usize::BITS }, 0>;\npub type ISize = Integer<{ \
         ::core::primitive::usize::BITS }>;\n",
    );
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("not declared"));
}

#[test]
fn a_longer_name_is_not_the_alias() {
    let errors = hits(
        "pub type USizeLike = UFixed<8, 0>;\npub type ISize = Integer<{ \
         ::core::primitive::usize::BITS }>;\n",
    );
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("not declared"));
}

/// The findings for a source whose `ISize` is the shipped one and whose
/// `USize` is `usize_decl`, or the other way round when `on_isize`.
fn one_bad(decl: &str, on_isize: bool) -> Vec<LintError> {
    if on_isize {
        hits(&format!(
            "pub type USize = UFixed<{{ ::core::primitive::usize::BITS }}, 0>;\n{decl}\n"
        ))
    } else {
        hits(&format!(
            "{decl}\npub type ISize = Integer<{{ ::core::primitive::usize::BITS }}>;\n"
        ))
    }
}

#[test]
fn a_literal_width_beside_a_comment_naming_the_pointer_width_fires() {
    for (decl, on_isize) in [
        ("pub type USize = UFixed<64, 0>; // usize::BITS", false),
        ("pub type USize = UFixed<64, 0>; /* usize::BITS */", false),
        ("pub type ISize = Integer</* usize::BITS */ 64>;", true),
        ("pub type ISize = Integer<32>; // was { usize::BITS }", true),
    ] {
        assert_eq!(one_bad(decl, on_isize).len(), 1, "{decl}");
    }
}

#[test]
fn a_width_that_only_mentions_the_pointer_width_fires() {
    for (decl, on_isize) in [
        (
            "pub type USize = UFixed<{ if usize::BITS == 64 { 64 } else { 32 } }, 0>;",
            false,
        ),
        ("pub type USize = UFixed<{ usize::BITS * 2 }, 0>;", false),
        ("pub type USize = UFixed<{ usize::BITS }, 1>;", false),
        ("pub type ISize = Integer<{ usize::BITS - 1 }>;", true),
        (
            "pub type ISize = Integer<{ core::cmp::min(usize::BITS, 32) }>;",
            true,
        ),
    ] {
        assert_eq!(one_bad(decl, on_isize).len(), 1, "{decl}");
    }
}

#[test]
fn an_alias_spelled_with_the_other_point_fires() {
    // Each alias reads the pointer width and names the wrong point: `USize`
    // signed, `ISize` unsigned.
    assert_eq!(
        one_bad("pub type USize = Integer<{ usize::BITS }>;", false).len(),
        1
    );
    assert_eq!(
        one_bad("pub type ISize = UFixed<{ usize::BITS }, 0>;", true).len(),
        1
    );
    assert_eq!(
        hits(
            "pub type USize = Integer<{ usize::BITS }>;\npub type ISize = UFixed<{ usize::BITS }, \
             0>;\n"
        )
        .len(),
        2
    );
}

#[test]
fn a_correct_alias_across_lines_and_around_comments_is_silent() {
    for source in [
        "pub type USize =\n    UFixed<{ ::core::primitive::usize::BITS }, 0>;\npub type ISize \
         =\n    Integer<{ ::core::primitive::usize::BITS }>;\n",
        "pub type USize = UFixed<\n    { ::core::primitive::usize::BITS },\n    0,\n>;\npub type \
         ISize = Integer<\n    { ::core::primitive::usize::BITS },\n>;\n",
        "pub type USize = UFixed<{ ::core::primitive::usize::BITS }, 0>; // the pointer \
         width\npub type ISize = Integer<{::core::primitive::usize::BITS}>;\n",
        "pub type USize = UFixed<{ ::core::primitive::usize::BITS } /* read */, 0>;\npub type \
         ISize = Integer<\n    // the pointer width\n    { ::core::primitive::usize::BITS \
         }\n>;\n",
    ] {
        assert!(hits(source).is_empty(), "{source}: {:?}", hits(source));
    }
}

#[test]
fn a_wrong_alias_across_lines_is_found_at_its_opening_line() {
    let errors = hits(
        "pub type USize = UFixed<{ ::core::primitive::usize::BITS }, 0>;\n\npub type ISize \
         =\n    Integer<64>;\n",
    );
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].line, 3);
}

#[test]
fn a_declaration_inside_a_literal_is_not_a_declaration() {
    for source in [
        "const S: &str = \"pub type USize = UFixed<64, 0>;\";\n",
        "const S: &str = r#\"pub type USize = UFixed<64, 0>;\"#;\n",
        "const S: &[u8] = br\"pub type USize = UFixed<64, 0>;\";\n",
    ] {
        let errors = hits(&format!(
            "{source}pub type ISize = Integer<{{ ::core::primitive::usize::BITS }}>;\n"
        ));
        assert_eq!(errors.len(), 1, "{source}: {errors:?}");
        assert!(errors[0].message.contains("not declared"), "{source}");
    }
}

#[test]
fn a_comment_marker_inside_a_literal_opens_nothing() {
    // Were the marker read, the declaration after it would vanish and the
    // alias would be reported missing; were the quote character read as a
    // string, the same.
    for opener in [
        "const S: &str = \"/*\";",
        "const S: &str = \"// \\\" /*\";",
        "const C: char = '\"';",
        "const C: char = '\\'';",
        "fn f<'a>(x: &'a str) -> &'a str { x }",
    ] {
        let source = format!(
            "{opener}\npub type USize = UFixed<{{ ::core::primitive::usize::BITS }}, 0>;\npub \
             type ISize = Integer<{{ ::core::primitive::usize::BITS }}>; /* */\n"
        );
        assert!(hits(&source).is_empty(), "{opener}: {:?}", hits(&source));
        let wrong = source.replace(
            "UFixed<{ ::core::primitive::usize::BITS }, 0>",
            "UFixed<64, 0>",
        );
        assert_eq!(hits(&wrong).len(), 1, "{opener}");
    }
}

#[test]
fn a_nested_block_comment_is_dropped_whole() {
    let errors = hits(
        "/* outer /* inner */ pub type USize = UFixed<64, 0>; */\npub type USize = \
         UFixed<{ ::core::primitive::usize::BITS }, 0>;\npub type ISize = Integer<{ \
         ::core::primitive::usize::BITS }>;\n",
    );
    assert!(errors.is_empty(), "{errors:?}");
}

#[test]
fn stripping_keeps_the_lines_where_they_were() {
    let text = "a /* x\ny */ b // z\n\"q\nr\" 'c'\n";
    let stripped = without_comments(text);
    assert_eq!(stripped.lines().count(), text.lines().count());
    assert_eq!(stripped.matches('\n').count(), text.matches('\n').count());
    assert!(stripped.contains(" b "));
    assert!(!stripped.contains('x') && !stripped.contains('z') && !stripped.contains('q'));
}

#[test]
fn it_reads_the_crate_once_rather_than_once_per_file() {
    assert!(!ThePlatformWidthPointsReadThePointerWidth.per_file());
}

#[test]
fn another_crate_is_not_read() {
    let fixture = LintFixture::new("pub type USize = UFixed<64, 0>;\n")
        .with_crate_name("arvo-placement", "placement");
    assert!(
        ThePlatformWidthPointsReadThePointerWidth
            .check(&fixture.ctx())
            .is_empty()
    );
}

#[test]
fn a_module_named_usize_in_scope_still_fires_on_the_bare_spelling() {
    // A module or item literally named `usize`, declared or brought into scope
    // in the same file, would take precedence over the primitive type at a bare
    // `usize::BITS` path, so a build under that shadow could read a `BITS` this
    // lint never sees. The lint cannot resolve names, but it does not need to
    // for this particular shadow: comparing the text against the leading-`::`
    // `::core::primitive::usize::BITS` exactly finds a bare `usize::BITS`
    // regardless of whether anything in scope shadows it, closing this one hole
    // by construction. It is not the general answer to shadowing, which is why
    // `core` itself needs the separate arm below rather than a stronger reading
    // of this same comparison: an item can still bind the name `core`, and the
    // spelling comparison alone cannot see that, which is what
    // `a_declared_core_module_fires` and its siblings exist to close.
    let errors = hits(
        "mod usize {\n    pub const BITS: u32 = 99;\n}\npub type USize = UFixed<{ usize::BITS \
         }, 0>;\npub type ISize = Integer<{ ::core::primitive::usize::BITS }>;\n",
    );
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("USize"));
}

// --- the core-binding arm ----------------------------------------------------

/// A source with the shipped aliases plus one extra declaration spliced in
/// before them.
fn with_declaration(decl: &str) -> String {
    format!("{decl}\n{SHIPPED}")
}

#[test]
fn a_declared_core_module_fires() {
    let errors = hits(&with_declaration(
        "mod core {\n    pub const X: u32 = 1;\n}",
    ));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].message.contains("mod core"));
}

#[test]
fn a_core_module_referencing_an_external_file_fires() {
    // `mod core;` with no body is the same declaration, spelled for a file this
    // lint never has to see.
    let errors = hits(&with_declaration("mod core;"));
    assert_eq!(errors.len(), 1, "{errors:?}");
}

#[test]
fn a_use_alias_to_core_fires() {
    let errors = hits(&with_declaration("use crate::fake as core;"));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].message.contains("use"));
}

#[test]
fn an_extern_crate_alias_to_core_fires() {
    let errors = hits(&with_declaration("extern crate self as core;"));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].message.contains("extern crate"));
}

#[test]
fn an_extern_crate_alias_to_core_from_a_named_crate_fires() {
    let errors = hits(&with_declaration("extern crate somewhere as core;"));
    assert_eq!(errors.len(), 1, "{errors:?}");
}

#[test]
fn a_core_binding_is_found_regardless_of_which_file_carries_the_aliases() {
    // The crate-scoped arm reads every file, so a `mod core` in one file and
    // the aliases in another still meet.
    let fixture = LintFixture::new(SHIPPED)
        .with_crate_name("arvo-format", "format")
        .with_module("other.rs", "mod core {\n    pub const X: u32 = 1;\n}\n");
    let errors = ThePlatformWidthPointsReadThePointerWidth.check(&fixture.ctx());
    assert!(
        errors.iter().any(|e| e.message.contains("mod core")),
        "{errors:?}"
    );
}

#[test]
fn the_control_no_core_binding_present_is_silent() {
    // The shipped source alone, with neither arm's fixture spliced in, fires
    // neither: this is the positive control for the alias arm (already covered
    // above) and the silent control for this arm.
    assert!(hits(SHIPPED).is_empty());
}

#[test]
fn a_module_named_something_else_is_silent() {
    // The word `core` has to be the whole word bound, not a prefix or suffix of
    // one, and `corelib` is neither `core` the module nor the alias to it.
    let errors = hits(&with_declaration(
        "mod corelib {\n    pub const X: u32 = 1;\n}",
    ));
    assert!(errors.is_empty(), "{errors:?}");
}

#[test]
fn a_use_alias_to_something_else_is_silent() {
    let errors = hits(&with_declaration("use crate::fake as corelib;"));
    assert!(errors.is_empty(), "{errors:?}");
}

#[test]
fn a_cast_to_a_path_starting_with_core_is_silent() {
    // `x as core::ffi::c_int` is an ordinary cast to a path that happens to
    // start with `core`, not a binding of the name `core` to anything, and the
    // next character after `core` is `:` rather than a statement's `;`, which is
    // what the terminator check on this arm is for.
    let errors = hits(&with_declaration(
        "const fn f(x: i32) -> i32 { x as core::ffi::c_int as i32 }",
    ));
    assert!(errors.is_empty(), "{errors:?}");
}

#[test]
fn a_mention_of_core_in_a_comment_or_literal_is_silent() {
    for decl in ["// mod core {}", "const S: &str = \"mod core {}\";", "// use fake as core;"] {
        let errors = hits(&with_declaration(decl));
        assert!(errors.is_empty(), "{decl}: {errors:?}");
    }
}
