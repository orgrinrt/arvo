//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The design side: a position that would sit, written in a template.
//!
//! Two consumers in this stack carry designs and no source at all. A walk over
//! `.rs` answers those at zero, and the obligation covers a position that "sits
//! **or would sit**", so a zero there would be the enumeration missing the half
//! of the demand that has not been built yet.

use crate::kinds::Position;
use crate::report::design_positions;

const TEMPLATE: &str = r#"
Some prose that mentions u32 and usize and bool outside any fence at all.

```rust
pub fn measure(count: u32, width: u8) -> usize;
pub const RADIX: u16 = 10;
pub struct Held {
    pub extent: u64,
}
```

More prose, this time naming f64.

```toml
value = 32
width = "u32"
```

```
pub fn fenced_without_a_language(n: usize);
```
"#;

fn found() -> Vec<(String, String, Position)> {
    design_positions("t", "d.md.tmpl", TEMPLATE)
        .into_iter()
        .map(|f| (f.primitive, f.name, f.position))
        .collect()
}

#[test]
fn a_signature_inside_a_rust_fence_is_a_position_and_carries_its_own_identifier() {
    let rows = found();
    // The parameters are named for themselves rather than for the function
    // that declares them, which the line scan this replaced could not do and
    // which is what the role reading needs.
    assert!(
        rows.contains(&("u32".into(), "count".into(), Position::FnParam)),
        "{rows:?}"
    );
    assert!(
        rows.contains(&("u8".into(), "width".into(), Position::FnParam)),
        "{rows:?}"
    );
    assert!(
        rows.contains(&("usize".into(), "measure".into(), Position::FnReturn)),
        "{rows:?}"
    );
    assert!(
        rows.contains(&("u64".into(), "extent".into(), Position::StructField)),
        "{rows:?}"
    );
}

#[test]
fn a_free_constant_in_a_fence_is_a_free_constant() {
    // The line scan called this a trait constant, from the word the line began
    // with. It is not in a trait and nobody outside implements it.
    let rows = found();
    assert!(
        rows.contains(&("u16".into(), "RADIX".into(), Position::ItemConst)),
        "{rows:?}"
    );
}

#[test]
fn an_unlabelled_fence_is_read_as_rust_because_that_is_what_they_are_here() {
    let rows = found();
    assert!(
        rows.contains(&("usize".into(), "n".into(), Position::FnParam)),
        "{rows:?}"
    );
}

#[test]
fn the_control_prose_outside_a_fence_is_not_a_position() {
    let rows = found();
    // The prose names `u32`, `usize`, `bool` and `f64`. Only `bool` and `f64`
    // appear nowhere inside a fence, so they are the two that must be absent.
    assert!(
        !rows.iter().any(|(p, _, _)| p == "bool"),
        "prose naming `bool` was counted as a position: {rows:?}"
    );
    assert!(
        !rows.iter().any(|(p, _, _)| p == "f64"),
        "prose naming `f64` was counted as a position: {rows:?}"
    );
}

#[test]
fn the_control_a_fence_in_another_language_is_not_read() {
    // The toml block declares `width = "u32"` on its own line. The rust fence
    // also has a parameter called `width`, so the check is the count: reading
    // the toml block as Rust would produce a second one.
    let rows = found();
    assert_eq!(
        rows.iter().filter(|(_, n, _)| n == "width").count(),
        1,
        "a toml fence was parsed as Rust: {rows:?}"
    );
}

#[test]
fn the_control_a_non_rust_fence_does_not_desynchronise_the_ones_after_it() {
    // The failure this pins: tracking only the rust fences makes the *closing*
    // marker of a toml block read as an opening marker with an empty language,
    // which counts as rust, and every fence after it is inverted. The fixture
    // has an unlabelled rust fence sitting after the toml one for this reason.
    let rows = found();
    assert!(
        rows.iter().any(|(_, n, _)| n == "n"),
        "the fence after the toml block was lost: {rows:?}"
    );
}

#[test]
fn a_const_generic_parameter_in_a_fence_is_the_excepted_position() {
    // The defect that retired the line scan. Both of these are const generic
    // parameters, which op excepted in his own words, and the line scan filed
    // both as struct fields because the line began with `pub ` and held a colon.
    let text = "```rust\npub struct PoolFrame<const MAX_CORES: usize, const MAX_PHASES: usize> {\n    pub arrived: u32,\n}\n```\n";
    let rows = design_positions("t", "d.md.tmpl", text);
    let excepted = rows
        .iter()
        .filter(|r| r.position == Position::ConstGenericParam)
        .count();
    let fields = rows
        .iter()
        .filter(|r| r.position == Position::StructField)
        .count();
    assert_eq!(excepted, 2, "{rows:?}");
    assert_eq!(fields, 1, "{rows:?}");
}

#[test]
fn a_supertrait_bounds_type_argument_in_a_fence_is_a_bound_and_not_a_field() {
    // The second defect from the same corpus read.
    let text = "```rust\npub trait ByteEmitter: Push<u8> + BulkPush<u8> {}\n```\n";
    let rows = design_positions("t", "d.md.tmpl", text);
    assert!(
        rows.iter().all(|r| r.position != Position::StructField),
        "a supertrait bound was filed as a struct field: {rows:?}"
    );
}

#[test]
fn the_control_a_fence_with_no_type_position_yields_nothing() {
    let text = "```rust\nsome_call(u32_helper());\n```\n";
    assert!(design_positions("t", "d.md.tmpl", text).is_empty());
}

#[test]
fn the_control_a_literal_suffix_in_a_fence_is_not_a_position() {
    let text = "```rust\npub const X: Width = 32u32;\n```\n";
    // `Width` is arvo's own and lands on the supply side; what must be absent
    // is the literal's suffix on the host side.
    let rows: Vec<_> = design_positions("t", "d.md.tmpl", text)
        .into_iter()
        .filter(|r| r.supplier.is_none())
        .collect();
    assert!(
        rows.is_empty(),
        "`32u32` was read as a type position: {:?}",
        rows.iter()
            .map(|r| (&r.primitive, &r.name))
            .collect::<Vec<_>>()
    );
}

#[test]
fn the_control_the_scanner_moves_when_the_fence_moves() {
    // The mutation the controls above need: take the same text, put the prose
    // line inside the fence, and the count has to rise.
    let outside = "prose naming u32 here\n\n```rust\npub fn f(a: u8);\n```\n";
    let inside = "```rust\npub fn g(b: u32);\npub fn f(a: u8);\n```\n";
    assert_eq!(design_positions("t", "d.md.tmpl", outside).len(), 1);
    assert_eq!(design_positions("t", "d.md.tmpl", inside).len(), 2);
}

#[test]
fn a_commented_out_signature_inside_a_fence_is_not_a_position() {
    let text = "```rust\n// pub fn f(a: u8);\npub fn g(b: u32);\n```\n";
    let rows = design_positions("t", "d.md.tmpl", text);
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(rows[0].primitive, "u32");
}
