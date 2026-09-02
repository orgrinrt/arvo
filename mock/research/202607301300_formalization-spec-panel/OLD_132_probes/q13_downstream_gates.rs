//! The features hilavitkutin-api actually ships, under -Znext-solver=globally.
#![no_std]
#![crate_type = "lib"]
#![feature(
    min_specialization,
    marker_trait_attr,
    negative_impls,
    impl_trait_in_assoc_type,
    adt_const_params,
    const_trait_impl,
    associated_type_defaults
)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

// min_specialization: the column_value.rs shape.
pub trait ColumnValue {
    fn tag(&self) -> u32;
}
impl<T> ColumnValue for T {
    default fn tag(&self) -> u32 {
        0
    }
}
impl ColumnValue for u8 {
    fn tag(&self) -> u32 {
        1
    }
}

// marker_trait_attr: the overlapping-membership shape.
#[marker]
pub trait Member {}
pub struct A;
pub struct B;
impl Member for A {}
impl Member for B {}

// negative_impls.
pub struct NotSync(*const u8);
impl !Sync for NotSync {}

// impl_trait_in_assoc_type.
pub trait Src {
    type It: Iterator<Item = u32>;
    fn it(&self) -> Self::It;
}
pub struct Three;
impl Src for Three {
    type It = impl Iterator<Item = u32>;
    fn it(&self) -> Self::It {
        [1u32, 2, 3].into_iter()
    }
}

// associated_type_defaults: the BuilderInput shape.
pub trait BuilderInput: Sized {
    type Init = Self;
}
pub struct Plain;
impl BuilderInput for Plain {}

// const_trait_impl + adt_const_params.
#[derive(PartialEq, Eq, core::marker::ConstParamTy)]
pub struct W(pub u16);
pub struct Sized2<const N: W>(PhantomData<[(); 0]>);
pub fn holds(_: Sized2<{ W(13) }>) {}
