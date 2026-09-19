//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for the `core`-binding arm of
//! `the-platform-width-points-read-the-pointer-width`.
//!
//! Every spelling that fires is paired with a silent control that differs from
//! it in the one thing the arm reads, so a matcher that fired on everything
//! near the word `core` fails the control and one that fired on nothing fails
//! the spelling.

use mockspace::testkit::LintFixture;

use super::core_bindings::{Form, Kind, core_bindings_in, message, tokens};
use super::tests::{SHIPPED, hits};
use super::*;

/// A source with the shipped aliases plus one extra declaration spliced in
/// before them.
fn with_declaration(decl: &str) -> String {
    format!("{decl}\n{SHIPPED}")
}

/// Asserts `decl` gives exactly one finding, of `form`, and that `control`
/// gives none.
fn fires_and_its_control_does_not(decl: &str, control: &str, form: Form) {
    let errors = hits(&with_declaration(decl));
    assert_eq!(errors.len(), 1, "{decl:?}: {errors:?}");
    assert_eq!(errors[0].message, message(form), "{decl:?}");
    let quiet = hits(&with_declaration(control));
    assert!(quiet.is_empty(), "control {control:?}: {quiet:?}");
}

#[test]
fn a_module_named_core_fires_in_every_spelling() {
    for (decl, control) in [
        (
            "mod core {\n    pub const X: u32 = 1;\n}",
            "mod corelib {\n    pub const X: u32 = 1;\n}",
        ),
        ("mod core;", "mod cores;"),
        ("mod r#core {}", "mod r#corex {}"),
        ("pub(crate) mod core;", "pub(crate) mod score;"),
        ("mod\ncore;", "mod\nother;"),
        ("mod /* x */ core;", "mod /* core */ other;"),
        ("mod    core {}", "fn core() {}"),
    ] {
        fires_and_its_control_does_not(decl, control, Form::Mod);
    }
}

#[test]
fn a_use_alias_to_core_fires_in_every_spelling() {
    for (decl, control) in [
        ("use crate::fake as core;", "use crate::fake as corelib;"),
        ("use crate::fake as r#core;", "use crate::fake as r#corex;"),
        (
            "use crate::{fake as core, fake as other};",
            "use crate::{fake as other, fake as another};",
        ),
        (
            "use crate::{other, fake as core};",
            "use crate::{other, fake as _};",
        ),
        ("use a::{self as core};", "use a::{self as other};"),
        (
            "use crate::fake\n    as\n    core;",
            "use crate::fake\n    as\n    other;",
        ),
        (
            "use crate::fake as /* x */ core;",
            "use crate::fake as /* core */ other;",
        ),
    ] {
        fires_and_its_control_does_not(decl, control, Form::Use);
    }
}

#[test]
fn a_use_binding_core_as_a_last_segment_fires_in_every_spelling() {
    for (decl, control) in [
        ("use crate::fake::core;", "use crate::core::mem;"),
        ("use core;", "use core::mem;"),
        ("use a::{core};", "use a::{core::mem};"),
        ("use a::{x, core};", "use a::{x, corelib};"),
        ("use a::r#core;", "use a::r#core as other;"),
        ("use crate::fake::r#core;", "use crate::fake::r#core::x;"),
        ("use a::{b::{c, core}};", "use a::{b::{c, core::d}};"),
        (
            "use a::{\n    x,\n    core,\n};",
            "use a::{\n    x,\n    core as c,\n};",
        ),
        ("use a :: core ;", "use a :: core :: x ;"),
        ("use\na::core;", "use\na::core::x;"),
        ("use a::/* x */core;", "use a::/* core */other;"),
        ("pub use a::core;", "pub use a::core as other;"),
        ("#[cfg(unix)]\nuse a::core;", "#[cfg(unix)]\nuse a::other;"),
    ] {
        fires_and_its_control_does_not(decl, control, Form::Use);
    }
}

#[test]
fn a_use_binding_core_through_self_fires() {
    for (decl, control) in [
        ("use a::core::{self};", "use a::core::{self as c};"),
        ("use a::core::{self, mem};", "use a::core::{mem};"),
        ("use a::{core::{self}};", "use a::{core::{mem}};"),
    ] {
        fires_and_its_control_does_not(decl, control, Form::Use);
    }
}

#[test]
fn an_extern_crate_alias_to_core_fires_in_every_spelling() {
    for (decl, control) in [
        (
            "extern crate self as core;",
            "extern crate self as corelib;",
        ),
        ("extern crate somewhere as core;", "extern crate core;"),
        (
            "extern crate self as r#core;",
            "extern crate core as other;",
        ),
        (
            "extern  crate self as core;",
            "extern  crate self as other;",
        ),
        (
            "extern\ncrate self as core;",
            "extern\ncrate self as other;",
        ),
        (
            "extern /* x */ crate self as core;",
            "extern /* core */ crate self as other;",
        ),
        (
            "extern crate self\n    as\n    core;",
            "extern crate self\n    as\n    other;",
        ),
        (
            "pub(crate) extern crate self as core;",
            "pub(crate) extern crate self as other;",
        ),
        (
            "#[cfg(target_pointer_width = \"32\")]\nextern crate self as r#core;\n",
            "#[cfg(target_pointer_width = \"32\")]\nextern crate self as other;\n",
        ),
    ] {
        fires_and_its_control_does_not(decl, control, Form::ExternCrate);
    }
}

#[test]
fn a_path_ending_in_core_outside_a_use_item_is_silent() {
    // The last-segment reading belongs to a `use` item's tree and nothing else:
    // a path ending in `core` in an expression, a pattern or a type binds
    // nothing.
    for decl in [
        "static S: u32 = m::core;",
        "const fn f() -> u32 { g(m::core, 1) }",
        "const X: S = S { f: m::core };",
        "const fn f(x: i32) -> i32 { x as core::ffi::c_int as i32 }",
        "fn f() -> impl Sized + use<> { m::core }",
    ] {
        let errors = hits(&with_declaration(decl));
        assert!(errors.is_empty(), "{decl:?}: {errors:?}");
    }
}

#[test]
fn a_capture_bound_hides_nothing_after_it() {
    // `use<..>` in a return type is not a `use` item; the scan goes on past it.
    let errors = hits(&with_declaration(
        "fn f() -> impl Sized + use<> { 0 }\nmod core {}",
    ));
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].message, message(Form::Mod));
}

#[test]
fn a_mention_of_core_in_a_comment_or_literal_is_silent() {
    for decl in [
        "// mod core {}",
        "const S: &str = \"mod core {}\";",
        "// use fake as core;",
        "/* extern crate self as core; */",
        "const S: &str = r#\"use a::core;\"#;",
        "const C: char = 'c'; // extern crate self as core;",
    ] {
        let errors = hits(&with_declaration(decl));
        assert!(errors.is_empty(), "{decl:?}: {errors:?}");
    }
}

#[test]
fn a_raw_keyword_is_a_name_and_opens_nothing() {
    for decl in [
        "fn r#mod() {}\nconst C: u32 = r#use(core);",
        "let r#extern = crate_self_as(core);",
    ] {
        let errors = hits(&with_declaration(decl));
        assert!(errors.is_empty(), "{decl:?}: {errors:?}");
    }
}

#[test]
fn every_binding_in_one_item_is_its_own_finding_at_its_own_line() {
    let errors = hits(&with_declaration(
        "use a::{\n    core,\n    b as r#core,\n    c::core::{self},\n};",
    ));
    let mut lines: Vec<usize> = errors.iter().map(|e| e.line).collect();
    lines.sort_unstable();
    assert_eq!(lines, [2, 3, 4], "{errors:?}");
    assert!(errors.iter().all(|e| e.message == message(Form::Use)));
}

#[test]
fn each_form_says_what_it_reaches() {
    // The two forms the leading `::` resists say they do not reach the aliases
    // and say a `use` binds in its own module; the one it does not resist says
    // it reaches them.
    let module = message(Form::Mod);
    let import = message(Form::Use);
    let external = message(Form::ExternCrate);
    assert!(module.contains("does not reach them"), "{module}");
    assert!(import.contains("does not reach"), "{import}");
    assert!(
        import.contains("binds only in the module that holds it"),
        "{import}"
    );
    assert!(external.contains("so it reaches"), "{external}");
    assert!(!external.contains("does not reach"), "{external}");
    assert!(module != import && import != external && module != external);
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
        errors.iter().any(|e| e.message == message(Form::Mod)),
        "{errors:?}"
    );
}

#[test]
fn the_tokens_are_words_raw_words_and_punctuation() {
    let toks = tokens("use a::r#core as  x;\nr#\"   \"#");
    let seen: Vec<(&str, Kind)> = toks.iter().map(|t| (t.text, t.kind)).collect();
    assert_eq!(seen, [
        ("use", Kind::Word),
        ("a", Kind::Word),
        ("::", Kind::Punct),
        ("core", Kind::Raw),
        ("as", Kind::Word),
        ("x", Kind::Word),
        (";", Kind::Punct),
        ("r", Kind::Word),
        ("#", Kind::Punct),
        ("\"", Kind::Punct),
        ("\"", Kind::Punct),
        ("#", Kind::Punct),
    ]);
    assert_eq!(toks[3].at, 7);
}

#[test]
fn the_forms_are_read_from_the_stripped_text() {
    let (_, found) = core_bindings_in("mod core; // use a::core;\nextern crate x as core;");
    let forms: Vec<Form> = found.iter().map(|&(form, _)| form).collect();
    assert_eq!(forms, [Form::Mod, Form::ExternCrate]);
}
