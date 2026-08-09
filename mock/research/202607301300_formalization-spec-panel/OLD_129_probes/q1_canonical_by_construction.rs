#![no_std]
#![allow(dead_code)]

// The precision IS the const parameter. Nothing derives it, nothing computes it
// in type position. Canonicity is by construction: one type per value.
pub struct Prec<const P: u32>;

pub struct Warm;
pub struct Number<const P: u32, S>(u128, core::marker::PhantomData<S>);

// concrete arithmetic in a const argument at a CONCRETE site: no generic params
// involved, so this is ordinary const eval and needs no feature at all.
pub type A = Number<{ 13 + 3 }, Warm>;
pub type B = Number<{ 8 + 8 }, Warm>;
pub type C = Number<16, Warm>;

pub fn takes(_: Number<16, Warm>) {}

pub fn agree(a: A, b: B, c: C) {
    takes(a);
    takes(b);
    takes(c);
}

// and the three are literally interchangeable in both directions
pub fn round_trip(x: A) -> B {
    x
}
pub fn round_trip2(x: C) -> A {
    x
}
