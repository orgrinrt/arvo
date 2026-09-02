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
    // `Width` is one of arvo's own, so it comes back on the supply side. What
    // must be empty is the host side, and asserting over the whole return would
    // be asserting that the supply walk does nothing.
    let found: Vec<_> = walk("t", "f.rs", clean)
        .into_iter()
        .filter(|f| f.supplier.is_none())
        .collect();
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
    let found: Vec<_> = walk("t", "f.rs", mutated)
        .into_iter()
        .filter(|f| f.supplier.is_none())
        .collect();
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

#[test]
fn a_stack_type_at_the_same_position_lands_on_the_supply_side() {
    // The denominator. Without it, a count of host primitives says nothing
    // about whether the obligation is nearly met or barely started.
    let source = "pub fn ask(w: Width, n: usize) -> Outcome<Bool, E> { todo!() }";
    let found = walk("t", "f.rs", source);
    let host: Vec<_> = found.iter().filter(|f| f.supplier.is_none()).collect();
    let arvo: Vec<_> = found
        .iter()
        .filter(|f| f.supplier == Some("arvo"))
        .collect();
    let notko: Vec<_> = found
        .iter()
        .filter(|f| f.supplier == Some("notko"))
        .collect();
    assert_eq!(host.len(), 1, "{found:?}");
    assert_eq!(host[0].primitive, "usize");
    assert_eq!(
        arvo.len(),
        2,
        "`Width` and `Bool` are both arvo's: {found:?}"
    );
    assert_eq!(notko.len(), 1, "`Outcome` is notko's: {found:?}");
}

#[test]
fn the_control_an_ordinary_consumer_type_is_on_neither_side() {
    // The supply list is hand-written, so it has to be a list rather than "any
    // capitalised identifier". A type nobody in this stack ships must not enter
    // the count in either direction.
    let found = walk(
        "t",
        "f.rs",
        "pub fn ask(x: SomeConsumerType, y: Vec<Whatever>) {}",
    );
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_stack_type_carries_the_same_position_and_carrier_reading_as_a_host_one() {
    // The supply walk has to be the *same* classification, or the two halves of
    // the fraction are over different populations and the ratio means nothing.
    let host = walk("t", "f.rs", "pub struct S { pub f: *mut u8 }");
    let stack = walk("t", "f.rs", "pub struct S { pub f: *mut Width }");
    assert_eq!(host.len(), 1);
    assert_eq!(stack.len(), 1);
    assert_eq!(host[0].position, stack[0].position);
    assert_eq!(host[0].carrier, stack[0].carrier);
    assert_eq!(host[0].name, stack[0].name);
}

#[test]
fn a_name_arvo_no_longer_exports_is_its_own_answer_and_not_supply() {
    use crate::supply::supplier;
    // The finding the three-way split exists to make visible. `USize` is the
    // single commonest stack type at an API position in this corpus and arvo
    // does not export it: the crate tree holding it was deleted and the
    // consumers were not changed. Filing it as supply reports an obligation as
    // nearly met against types that are not there.
    assert_eq!(supplier("USize"), Some("gone"));
    assert_eq!(supplier("Cap"), Some("gone"));
    assert_eq!(supplier("Bits"), Some("gone"));
    assert_eq!(supplier("BitAccess"), Some("gone"));
}

#[test]
fn the_control_the_names_arvo_does_export_are_supply() {
    use crate::supply::supplier;
    // Read off the three shipped `lib.rs` files. If these came back `gone` the
    // split would be a claim that arvo exports nothing.
    for name in [
        "Width",
        "Bool",
        "Format",
        "Quantum",
        "Placement",
        "Strategy",
        "Hot",
    ] {
        assert_eq!(supplier(name), Some("arvo"), "`{name}` is exported today");
    }
}

#[test]
fn the_control_no_name_is_on_two_lists_at_once() {
    use crate::supply::{ARVO_GONE, ARVO_SHIPPED, NOTKO};
    // A name on both lists resolves by whichever branch runs first, silently,
    // and the two counts then disagree with each other by exactly that name.
    for name in ARVO_SHIPPED {
        assert!(!ARVO_GONE.contains(name), "`{name}` is on both arvo lists");
        assert!(
            !NOTKO.contains(name),
            "`{name}` is on the arvo and the notko list"
        );
    }
    for name in ARVO_GONE {
        assert!(
            !NOTKO.contains(name),
            "`{name}` is on the gone and the notko list"
        );
    }
}

#[test]
fn the_control_a_host_primitive_is_on_no_supply_list() {
    use crate::supply::supplier;
    for name in ["u8", "u32", "usize", "bool", "f64", "str", "char"] {
        assert_eq!(supplier(name), None, "`{name}` is the host's");
    }
}

#[test]
fn an_opaque_newtype_does_not_leak_the_integer_it_wraps() {
    // `pub struct Width(u32)` is the shape the whole obligation is aiming at:
    // the integer is inside and no caller can name it. Counting it as a public
    // position reports arvo's own answer as nine more violations, and turns the
    // thing that satisfies the rule into evidence against it.
    let found = walk("t", "src/lib.rs", "pub struct Width(u32);");
    assert_eq!(found.len(), 1, "the field was not found at all: {found:?}");
    assert!(
        !found[0].public,
        "an opaque newtype's inner field was called public"
    );
    assert!(!found[0].is_demand());
}

#[test]
fn the_control_a_newtype_that_does_leak_it_is_counted() {
    // The mutation: one `pub` of difference. `vehje`'s `NodeId(pub u32)` and
    // `hilavitkutin`'s `BitWidth(pub u32)` are written this way and each
    // carries a `lint:allow(no-public-raw-field)` beside it, so the distinction
    // is one the stack already makes by hand.
    let found = walk("t", "src/lib.rs", "pub struct NodeId(pub u32);");
    assert_eq!(found.len(), 1);
    assert!(found[0].public, "a `pub` inner field was called private");
    assert!(found[0].is_demand());
}

#[test]
fn a_mixed_tuple_struct_reports_one_of_its_two_fields() {
    // Both fields in one list, one public and one not, so the read has to be
    // per field rather than per list. A list-wide check gives the same answer
    // for both and cannot be told from a correct one on a struct where the two
    // agree.
    let found = walk("t", "src/lib.rs", "pub struct Pair(pub i8, u128);");
    let public: Vec<&str> = found
        .iter()
        .filter(|f| f.public)
        .map(|f| f.primitive.as_str())
        .collect();
    assert_eq!(public, vec!["i8"], "{found:?}");
}

#[test]
fn an_enum_variants_tuple_field_stays_as_public_as_the_enum() {
    // A variant's field carries no visibility of its own, so the per-field rule
    // above must not apply to it.
    let found = walk("t", "src/lib.rs", "pub enum E { V(u32) }");
    assert_eq!(found.len(), 1);
    assert!(
        found[0].public,
        "a public enum's variant field was called private"
    );
}

#[test]
fn the_name_an_item_declares_is_not_a_use_of_that_name() {
    // Found on the fixture for the newtype test above: `pub struct Width(u32)`
    // was reporting a `Width` position, so every declaration of one of arvo's
    // own types entered the supply count as a consumer of itself.
    let found = walk("t", "src/lib.rs", "pub struct Width(u32);");
    assert_eq!(
        found.len(),
        1,
        "the declaration was counted as a use: {found:?}"
    );
    assert_eq!(found[0].primitive, "u32");
}

#[test]
fn the_control_a_real_use_of_the_same_name_is_still_counted() {
    // The distinction has to cut one way only. Three uses here and no
    // declaration of `Width` at all.
    let found = walk(
        "t",
        "src/lib.rs",
        "pub fn f(w: Width) -> Width { w }\npub struct S { pub w: Width }",
    );
    let widths = found.iter().filter(|f| f.primitive == "Width").count();
    assert_eq!(widths, 3, "{found:?}");
}

#[test]
fn a_declaration_and_a_use_in_one_file_separate_correctly() {
    let found = walk(
        "t",
        "src/lib.rs",
        "pub struct Width(u32);\npub fn f(w: Width) {}",
    );
    let widths: Vec<&crate::kinds::Found> =
        found.iter().filter(|f| f.primitive == "Width").collect();
    assert_eq!(widths.len(), 1, "{found:?}");
    assert_eq!(widths[0].name, "w");
}

#[test]
fn the_control_an_alias_separates_its_own_name_from_its_target() {
    // `type A = Width;` has two `type_identifier` nodes under one parent and
    // only the second is a use. An exclusion by parent kind rather than by
    // field name would drop both.
    let found = walk("t", "src/lib.rs", "pub type A = Width;");
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].primitive, "Width");
}
#[test]
fn an_associated_path_is_found_and_is_always_interior() {
    // `u32::MAX` puts the primitive in a slot the grammar spells `identifier`,
    // so the leaf walk cannot see it. The stack's own lint lists associated
    // paths among what it refuses, and the two instruments disagreed about 691
    // suppressed lines until this was closed.
    let found = walk(
        "t",
        "src/lib.rs",
        "pub fn f() { let m = u32::MAX; let n = usize::BITS; }",
    );
    let names: Vec<&str> = found.iter().map(|f| f.primitive.as_str()).collect();
    assert_eq!(names, vec!["u32", "usize"], "{found:?}");
    assert!(
        found
            .iter()
            .all(|f| f.position == crate::kinds::Position::Interior),
        "an expression cannot be an API position: {found:?}"
    );
    assert!(found.iter().all(|f| !f.is_demand()));
}

#[test]
fn the_control_an_ordinary_scoped_path_is_not_a_primitive() {
    let found = walk(
        "t",
        "src/lib.rs",
        "pub fn f() { let x = Width::NONE; let y = Vec::new(); }",
    );
    assert!(found.iter().all(|f| f.primitive != "u32"), "{found:?}");
}

#[test]
fn the_control_the_member_half_of_a_path_is_not_taken() {
    // `Thing::u32` is not a primitive position, and matching on the text alone
    // rather than on the path slot would take it.
    let found = walk("t", "src/lib.rs", "pub fn f() { let x = Thing::u32; }");
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_type_annotation_beside_an_associated_path_is_counted_once_as_each() {
    let found = walk("t", "src/lib.rs", "pub const X: u32 = u32::MAX;");
    assert_eq!(found.len(), 2, "{found:?}");
    let kinds: Vec<crate::kinds::Position> = found.iter().map(|f| f.position).collect();
    assert!(
        kinds.contains(&crate::kinds::Position::ItemConst),
        "{found:?}"
    );
    assert!(
        kinds.contains(&crate::kinds::Position::Interior),
        "{found:?}"
    );
}
