//! p07d. Following rustc's own suggestion from p07b: declare the nat's value as
//! a `type const`, which is `min_generic_const_args`'s form and is allowed.
//!
//! The ladder is inlined rather than included, because the change is inside it:
//! `const V: u32` becomes `type const V: u32`. Only the value-carrying part of
//! the ladder is reproduced, since that is all the pinning question needs.
//!
//! Two sub-questions, and both have to hold:
//!   (a) may a `type const` be DEFINED structurally, off a generic parameter?
//!   (b) may it then be USED in const-argument position?
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p07d.meta p07d_pin_const_to_nat_type_const.rs 2> out/p07d.log
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

pub struct Term;
pub struct D0<T>(PhantomData<T>);
pub struct D1<T>(PhantomData<T>);

pub trait Nat {
    type const V: u32;
}
impl Nat for Term {
    type const V: u32 = 0;
}
// (a) the structural definition. `T::V` is a generic parameter's associated
// const, on the RHS of a `type const`.
impl<T: Nat> Nat for D0<T> {
    type const V: u32 = const { 2 * T::V };
}
impl<T: Nat> Nat for D1<T> {
    type const V: u32 = const { 2 * T::V + 1 };
}

// (b) the use in const-argument position.
pub trait NatIs<const N: u32> {}
impl<W: Nat> NatIs<{ <W as Nat>::V }> for W {}

pub struct Fixed<const I: u32, S, WI>(PhantomData<(S, WI)>)
where
    WI: NatIs<I>;
