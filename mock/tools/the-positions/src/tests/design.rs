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
fn a_signature_inside_a_rust_fence_is_a_position() {
    let rows = found();
    assert!(
        rows.iter().any(|(p, n, _)| p == "u32" && n == "measure"),
        "{rows:?}"
    );
    assert!(
        rows.iter().any(|(p, n, _)| p == "u8" && n == "measure"),
        "{rows:?}"
    );
    assert!(
        rows.iter().any(|(p, n, _)| p == "u16" && n == "RADIX"),
        "{rows:?}"
    );
    assert!(
        rows.iter().any(|(p, n, _)| p == "u64" && n == "extent"),
        "{rows:?}"
    );
}

#[test]
fn an_unlabelled_fence_is_read_as_rust_because_that_is_what_they_are_here() {
    let rows = found();
    assert!(
        rows.iter()
            .any(|(p, n, _)| p == "usize" && n == "fenced_without_a_language"),
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
    let rows = found();
    assert!(
        !rows.iter().any(|(_, n, _)| n == "width"),
        "a toml fence was parsed as Rust: {rows:?}"
    );
}

#[test]
fn the_control_a_fence_with_no_signature_line_yields_nothing() {
    let text = "```rust\nsome_call(u32_helper());\n```\n";
    assert!(design_positions("t", "d.md.tmpl", text).is_empty());
}

#[test]
fn the_control_a_literal_suffix_in_a_fence_is_not_a_position() {
    let text = "```rust\npub const X: Width = 32u32;\n```\n";
    let rows = design_positions("t", "d.md.tmpl", text);
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
