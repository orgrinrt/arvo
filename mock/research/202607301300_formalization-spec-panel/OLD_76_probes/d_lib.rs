//! D (library half). Route Y's last candidate expression of the predicate:
//! the declaration macro emits the `HasOne` impl only for numerals whose
//! integer part is nonzero. The table then exists but is O(declared numerals)
//! rather than O(ceiling squared), and it refuses at type-check like a bound
//! should, which is what `ys`'s const-block witness fails to do.
//!
//! The question this file's companion asks is whether a CONSUMER can emit that
//! impl, because arvo cannot know which numerals a consumer will declare.
#![no_std]
#![feature(adt_const_params)]
use core::marker::PhantomData;
pub struct Hot;
pub trait Strategy {}
impl Strategy for Hot {}
pub struct UFixed<const I: u16, const F: u16, S>(PhantomData<S>);
pub trait HasOne {}
#[macro_export]
macro_rules! declare_numeral {
    ($name:ident, $i:literal, $f:literal, $s:ty, yes) => {
        pub type $name = $crate::UFixed<$i, $f, $s>;
        impl $crate::HasOne for $crate::UFixed<$i, $f, $s> {}
    };
    ($name:ident, $i:literal, $f:literal, $s:ty, no) => {
        pub type $name = $crate::UFixed<$i, $f, $s>;
    };
}
