//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every grammatical position the vocabulary names, planted and found.

use crate::kinds::Position;
use crate::tests::EVERY_POSITION;
use crate::walk::walk;

fn at(kind: Position) -> Vec<(String, String)> {
    walk("t", "f.rs", EVERY_POSITION)
        .into_iter()
        .filter(|f| f.position == kind)
        .map(|f| (f.primitive, f.name))
        .collect()
}

#[test]
fn a_parameter_of_a_public_function_is_a_parameter_position() {
    let rows = at(Position::FnParam);
    assert!(rows.contains(&("u32".into(), "count".into())), "{rows:?}");
    assert!(rows.contains(&("bool".into(), "flag".into())), "{rows:?}");
}

#[test]
fn a_return_type_is_a_return_position_and_not_a_parameter_one() {
    let rows = at(Position::FnReturn);
    assert!(
        rows.contains(&("usize".into(), "free_param".into())),
        "{rows:?}"
    );
    assert!(rows.contains(&("f64".into(), "ask".into())), "{rows:?}");
    let params = at(Position::FnParam);
    assert!(
        !params.iter().any(|(p, n)| p == "f64" && n == "ask"),
        "the return of `ask` was counted as one of its parameters: {params:?}"
    );
}

#[test]
fn a_named_field_is_a_struct_field_whether_or_not_it_is_public() {
    let rows = at(Position::StructField);
    assert!(rows.contains(&("u8".into(), "width".into())), "{rows:?}");
    assert!(rows.contains(&("u16".into(), "private".into())), "{rows:?}");
}

#[test]
fn a_type_nested_inside_a_field_is_still_that_field_and_not_an_interior_one() {
    let rows = at(Position::StructField);
    assert!(
        rows.contains(&("u64".into(), "nested".into())),
        "`Option<[u64; 4]>` did not resolve to the field it is the type of: {rows:?}"
    );
}

#[test]
fn a_tuple_struct_field_is_a_struct_field() {
    let rows = at(Position::StructField);
    assert!(rows.iter().any(|(p, _)| p == "i8"), "{rows:?}");
    assert!(rows.iter().any(|(p, _)| p == "u128"), "{rows:?}");
}

#[test]
fn an_enum_variants_fields_are_variant_fields_both_named_and_ordered() {
    let rows = at(Position::VariantField);
    assert!(rows.contains(&("u32".into(), "extent".into())), "{rows:?}");
    assert!(rows.iter().any(|(p, _)| p == "f32"), "{rows:?}");
    let struct_fields = at(Position::StructField);
    assert!(
        !struct_fields.iter().any(|(_, n)| n == "extent"),
        "a variant's field was filed as a struct's: {struct_fields:?}"
    );
}

#[test]
fn a_trait_constant_is_a_trait_constant_and_the_impls_copy_is_not() {
    let trait_side = at(Position::TraitConst);
    assert!(
        trait_side.contains(&("i32".into(), "BASE".into())),
        "{trait_side:?}"
    );
    assert!(
        trait_side.contains(&("bool".into(), "SIGNED".into())),
        "{trait_side:?}"
    );
    let impl_side = at(Position::ImplConst);
    assert!(
        impl_side.contains(&("i32".into(), "BASE".into())),
        "{impl_side:?}"
    );
    assert!(
        impl_side.contains(&("u32".into(), "INHERENT".into())),
        "{impl_side:?}"
    );
    assert_eq!(
        trait_side.iter().filter(|(_, n)| n == "BASE").count(),
        1,
        "the trait's declaration and the impl's definition are one position each, \
         not two of the same kind: {trait_side:?}"
    );
}

#[test]
fn a_free_constant_and_a_static_are_item_constants() {
    let rows = at(Position::ItemConst);
    assert!(rows.contains(&("u64".into(), "TOP".into())), "{rows:?}");
    assert!(rows.contains(&("i16".into(), "ALSO".into())), "{rows:?}");
}

#[test]
fn an_alias_target_is_its_own_position() {
    let rows = at(Position::AliasTarget);
    assert!(rows.contains(&("u32".into(), "Alias".into())), "{rows:?}");
}

#[test]
fn an_associated_types_binding_is_an_associated_type_position() {
    let rows = at(Position::AssocType);
    assert!(rows.contains(&("u16".into(), "Carrier".into())), "{rows:?}");
}

#[test]
fn the_type_of_a_const_generic_parameter_is_the_excepted_position() {
    let rows = at(Position::ConstGenericParam);
    assert!(rows.contains(&("usize".into(), "N".into())), "{rows:?}");
    assert!(rows.contains(&("u8".into(), "W".into())), "{rows:?}");
    assert!(
        !Position::ConstGenericParam.is_api(),
        "op excepted this position in his own words; the vocabulary must agree"
    );
}

#[test]
fn a_const_generic_parameter_is_not_confused_with_the_array_that_uses_it() {
    // `[u8; N]` in the return: the `u8` is the return position and the `usize`
    // of `N` is the excepted one. Two different positions on one line.
    let returns = at(Position::FnReturn);
    assert!(
        returns.contains(&("u8".into(), "generic".into())),
        "the element type of the returned array was lost: {returns:?}"
    );
    let excepted = at(Position::ConstGenericParam);
    assert!(
        !excepted.iter().any(|(_, n)| n == "generic"),
        "the return type was filed under the exception: {excepted:?}"
    );
}

#[test]
fn a_local_and_a_cast_are_interior_and_never_api() {
    let rows = at(Position::Interior);
    assert!(
        rows.iter().any(|(p, _)| p == "u32"),
        "the let binding was missed: {rows:?}"
    );
    assert!(
        rows.iter().any(|(p, _)| p == "u8"),
        "the cast was missed: {rows:?}"
    );
    assert!(!Position::Interior.is_api());
}

#[test]
fn every_kind_the_vocabulary_names_is_reachable_or_named_as_not() {
    // The whole matrix rather than the kinds this fixture happens to exercise.
    // Two kinds are not exercised here and are named, so the gap is a statement
    // rather than an omission nobody noticed.
    let found = walk("t", "f.rs", EVERY_POSITION);
    let unreached: Vec<&'static str> = Position::all()
        .iter()
        .filter(|k| !found.iter().any(|f| f.position == **k))
        .map(|k| k.token())
        .collect();
    assert_eq!(
        unreached,
        vec!["impl-trait-arg", "bound-arg"],
        "the set of kinds this fixture does not reach changed; either a kind \
         became reachable or one stopped being. Both are results and both want \
         the fixture updated deliberately."
    );
}
