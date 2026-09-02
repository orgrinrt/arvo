//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the primitive is wrapped in, which is the axis that says whether arvo
//! owes a numeral at the position at all.

use crate::kinds::Carrier;
use crate::walk::walk;

const SHAPES: &str = r"
pub fn scalar(count: usize) {}
pub fn reference(count: &u32) {}
pub fn slice(bytes: &[u8]) {}
pub fn array(fixed: [u16; 4]) {}
pub fn pointer(raw: *mut u8) {}
pub fn const_pointer(raw: *const u8) {}
pub fn argument(held: Option<u64>) {}
pub fn nested_slice(rows: &[&[i8]]) {}
pub fn slice_of_pointers(p: &[*mut f32]) {}
";

fn carrier_named(name: &str) -> Carrier {
    walk("t", "f.rs", SHAPES)
        .into_iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("no position named `{name}` was found at all"))
        .carrier
}

#[test]
fn a_bare_primitive_is_a_scalar() {
    assert_eq!(carrier_named("count"), Carrier::Scalar);
}

#[test]
fn a_slice_element_and_a_pointer_target_are_not_numbers() {
    assert_eq!(carrier_named("bytes"), Carrier::Element);
    assert_eq!(carrier_named("raw"), Carrier::Pointer);
    assert!(!Carrier::Element.is_a_number());
    assert!(!Carrier::Pointer.is_a_number());
}

#[test]
fn the_control_a_scalar_a_reference_and_an_argument_all_are_numbers() {
    // The claim above is only worth anything if the carrier axis does not
    // simply excuse everything. Three of the five must come back as numbers.
    assert!(Carrier::Scalar.is_a_number());
    assert!(Carrier::Reference.is_a_number());
    assert!(Carrier::Argument.is_a_number());
    assert_eq!(carrier_named("held"), Carrier::Argument);
}

#[test]
fn the_innermost_wrapper_wins_so_a_reference_to_a_slice_is_an_element() {
    // `&[u8]`: what the `u8` is the unit of is the slice. The reference around
    // the slice says nothing about the `u8`, and reading the outermost node
    // would call every byte buffer in the stack a reference to a number.
    assert_eq!(carrier_named("bytes"), Carrier::Element);
    assert_eq!(carrier_named("rows"), Carrier::Element);
}

#[test]
fn a_fixed_array_element_is_an_element_and_not_a_scalar() {
    assert_eq!(carrier_named("fixed"), Carrier::Element);
}

#[test]
fn a_pointer_inside_a_slice_is_still_a_pointer() {
    assert_eq!(carrier_named("p"), Carrier::Pointer);
}

#[test]
fn the_control_the_carrier_axis_actually_discriminates() {
    let found = walk("t", "f.rs", SHAPES);
    let mut seen: Vec<Carrier> = found.iter().map(|f| f.carrier).collect();
    seen.sort_unstable();
    seen.dedup();
    assert!(
        seen.len() >= 4,
        "the carrier read returned {} distinct answers over nine deliberately \
         different shapes, which is close enough to one answer to be none: {seen:?}",
        seen.len()
    );
}

#[test]
fn the_control_the_carrier_moves_when_the_wrapper_moves() {
    // The mutation: one character of difference between the two sources, and
    // the answer has to change.
    let scalar = walk("t", "f.rs", "pub fn f(x: u8) {}");
    let pointer = walk("t", "f.rs", "pub fn f(x: *mut u8) {}");
    assert_eq!(scalar[0].carrier, Carrier::Scalar);
    assert_eq!(pointer[0].carrier, Carrier::Pointer);
}

#[test]
fn a_const_generic_parameters_type_is_a_scalar_rather_than_an_argument() {
    // `<const N: usize>` sits inside `type_parameters`, which is not
    // `type_arguments`. Confusing the two would file every const generic
    // parameter in the stack as a type argument.
    let found = walk("t", "f.rs", "pub fn f<const N: usize>() {}");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].carrier, Carrier::Scalar);
    assert_eq!(found[0].position, crate::kinds::Position::ConstGenericParam);
}

#[test]
fn a_design_fence_reads_a_slice_as_an_element() {
    use crate::report::design_positions;
    let text = "```rust\npub fn load(path: &[u8]) -> usize;\n```\n";
    let rows = design_positions("t", "d.md.tmpl", text);
    assert_eq!(rows.len(), 2, "{rows:?}");
    // The fence is parsed, so the two positions on one line get their own
    // carriers rather than one between them. An earlier line-scanning version
    // could not separate them and gave both `element`, which excused the
    // `usize` return along with the byte slice.
    let path = rows
        .iter()
        .find(|r| r.name == "path")
        .expect("the slice was lost");
    let ret = rows
        .iter()
        .find(|r| r.primitive == "usize")
        .expect("the return was lost");
    assert_eq!(path.carrier, Carrier::Element);
    assert_eq!(ret.carrier, Carrier::Scalar);
}
