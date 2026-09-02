//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The cases that must fail.
//!
//! A walk reporting nothing and a corpus containing nothing are the same output.
//! Everything here plants an input where the instrument has to say something
//! different, so a zero elsewhere in the suite is a zero the instrument earned.

use crate::kinds::Position;
use crate::tests::EVERY_POSITION;
use crate::walk::walk;

#[test]
fn the_control_a_file_with_no_host_primitive_reports_nothing() {
    let clean = r"
        pub struct Held { pub width: Width }
        pub fn ask(count: Count) -> Extent { Extent }
        pub trait Contract { const BASE: Radix; }
    ";
    let found = walk("t", "f.rs", clean);
    assert!(
        found.is_empty(),
        "a file naming only the stack's own types reported {} positions: {:?}",
        found.len(),
        found
            .iter()
            .map(|f| (&f.primitive, &f.name))
            .collect::<Vec<_>>()
    );
}

#[test]
fn the_control_the_same_file_with_one_primitive_put_back_reports_exactly_one() {
    // The mutation the control above needs to be worth anything: change one
    // token and the count has to move by one.
    let mutated = r"
        pub struct Held { pub width: u8 }
        pub fn ask(count: Count) -> Extent { Extent }
        pub trait Contract { const BASE: Radix; }
    ";
    let found = walk("t", "f.rs", mutated);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].primitive, "u8");
    assert_eq!(found[0].position, Position::StructField);
}

#[test]
fn the_control_a_type_named_like_a_primitive_is_not_one() {
    // `U32` and `Usize` are ordinary identifiers. A scan matching on text would
    // take them; the parse does not, because they are `type_identifier` rather
    // than `primitive_type`.
    let source = r"
        pub struct Held { pub a: U32, pub b: Usize, pub c: MyBool }
        pub fn ask(x: Ubool) {}
    ";
    assert!(walk("t", "f.rs", source).is_empty());
}

#[test]
fn the_control_a_primitive_inside_a_string_or_a_comment_is_not_a_position() {
    let source = r#"
        // pub fn commented(x: u32) {}
        /// A doc comment naming u64 and bool.
        pub fn real() { let s = "u32 usize f64"; }
    "#;
    let found = walk("t", "f.rs", source);
    assert!(
        found.is_empty(),
        "a comment or a literal was read as a position: {:?}",
        found
            .iter()
            .map(|f| (&f.primitive, f.line))
            .collect::<Vec<_>>()
    );
}

#[test]
fn the_control_a_literal_suffix_is_not_a_type_position() {
    // `2u32` is one token to the parser and is not a `primitive_type`. This
    // matters because the shipped line-scanning lint says in its own header
    // that it cannot reach a suffix; the parse can distinguish the two and the
    // enumeration must not inherit the scan's blind spot as a false positive
    // in the other direction.
    let source = "pub fn f() { let x = 2u32 + 3usize as u64; }";
    let found = walk("t", "f.rs", source);
    let kinds: Vec<&str> = found.iter().map(|f| f.primitive.as_str()).collect();
    assert_eq!(
        kinds,
        vec!["u64"],
        "only the cast is a type position; the two suffixes are not: {found:?}"
    );
}

#[test]
fn the_control_visibility_actually_discriminates() {
    let found = walk("t", "f.rs", EVERY_POSITION);
    let public = found.iter().filter(|f| f.public).count();
    let private = found.iter().filter(|f| !f.public).count();
    assert!(
        public > 0 && private > 0,
        "the visibility judgement returned one answer for every position, \
         which is the same as not judging: {public} public, {private} private"
    );
}

#[test]
fn a_public_item_inside_a_private_module_is_not_reachable() {
    let found = walk("t", "f.rs", EVERY_POSITION);
    let row = found
        .iter()
        .find(|f| f.name == "x" && f.owner == "unreachable")
        .expect("the parameter inside the private module was not found at all");
    assert!(
        !row.public,
        "a `pub fn` inside a private `mod` was called reachable"
    );
}

#[test]
fn a_cfg_test_module_is_not_part_of_the_public_surface() {
    let found = walk("t", "f.rs", EVERY_POSITION);
    let row = found
        .iter()
        .find(|f| f.name == "y")
        .expect("the parameter inside the test module was not found at all");
    assert!(
        !row.public,
        "a `#[cfg(test)]` module was counted as shipped surface"
    );
}

#[test]
fn a_private_function_and_a_private_field_are_not_public() {
    let found = walk("t", "f.rs", EVERY_POSITION);
    let hidden = found
        .iter()
        .find(|f| f.owner == "hidden" && f.name == "n")
        .expect("the private inherent method's parameter was not found");
    assert!(!hidden.public);
    let field = found
        .iter()
        .find(|f| f.name == "private")
        .expect("the private field was not found");
    assert!(!field.public);
}

#[test]
fn a_trait_members_position_is_public_without_carrying_pub_itself() {
    let found = walk("t", "f.rs", EVERY_POSITION);
    let base = found
        .iter()
        .find(|f| f.name == "BASE" && f.position == Position::TraitConst)
        .expect("the trait constant was not found");
    assert!(
        base.public,
        "a trait member carries no visibility of its own and is as public as the trait"
    );
}

#[test]
fn the_control_a_private_trait_hides_its_members() {
    let source = "trait Hidden { const BASE: u32; fn ask(n: usize); }";
    let found = walk("t", "f.rs", source);
    assert!(!found.is_empty(), "the members were not found at all");
    assert!(
        found.iter().all(|f| !f.public),
        "members of a private trait were called public: {found:?}"
    );
}

#[test]
fn the_control_str_and_char_are_found_and_filed_outside_the_numeric_family() {
    use crate::kinds::family;
    let source = "pub fn f(s: &str, c: char) {}";
    let found = walk("t", "f.rs", source);
    assert_eq!(found.len(), 2, "{found:?}");
    for row in &found {
        assert_eq!(
            family(&row.primitive),
            Some("textual"),
            "`{}` was filed as something the obligation names, and it does not",
            row.primitive
        );
    }
}
