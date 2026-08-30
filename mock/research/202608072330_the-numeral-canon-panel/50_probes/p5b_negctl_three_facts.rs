// p5b: the negative controls for p5. This file is EXPECTED TO FAIL TO COMPILE, and its diagnostic
// is committed beside it as p5b_negctl_three_facts.err.
//
// Two refusals are wanted, and they are wanted for different reasons.
//
//   N1  the compute types of Warm and Precise are asserted equal. They are u16 and u32. A refusal
//       here is what makes p5's must-not-refuse assertion mean something: it shows the SameType
//       bridge is not vacuous, which is the shape 47_probes/p1b and p4b use and which the-test-gate
//       names as the difference between a control and a decoration.
//
//   N2  under M1 the slot holds the compute type, and the at-rest type is reached from the stride,
//       which is a const. This is the second half of 48:227-229's claim, and it should be refused
//       naming the forbidden generic_const_exprs, from a third starting point after 16_probes/p5b
//       and 47_probes/p2 and p3.
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p5b_negctl_three_facts.rs
//
// No #![feature] gate, deliberately: adding one is what the refusal is about.

#![no_std]

pub struct Warm;
pub struct Precise;
pub struct W13;

pub trait SameType<T> {}
impl<T> SameType<T> for T {}
pub const fn assert_same<A: SameType<B>, B>() {}

pub trait Facts<S> {
    type AtRest: Copy;
    type Compute: Copy;
    const STRIDE: u32;
}
impl Facts<Warm> for W13 {
    type AtRest = u16;
    type Compute = u16;
    const STRIDE: u32 = 16;
}
impl Facts<Precise> for W13 {
    type AtRest = u16;
    type Compute = u32;
    const STRIDE: u32 = 16;
}

// ---- N1: the two compute types are NOT the same, and asserting they are must be refused ----
const _: () = assert_same::<<W13 as Facts<Warm>>::Compute, <W13 as Facts<Precise>>::Compute>();

// ---- N2: reaching the at-rest TYPE from the stride CONST, generically ----

pub struct Bytes<const B: u32>;
pub trait NativeFor {
    type T: Copy;
}
impl NativeFor for Bytes<1> {
    type T = u8;
}
impl NativeFor for Bytes<2> {
    type T = u16;
}
impl NativeFor for Bytes<4> {
    type T = u32;
}

/// under M1 the derivation's slots are (compute type, stride). the at-rest type is a function of
/// the stride, and this is what a site would have to write to get it.
pub type AtRestFromStride<S> = <Bytes<{ <W13 as Facts<S>>::STRIDE / 8 }> as NativeFor>::T;

/// the same in return position, so the refusal is about the direction and not about type aliases.
pub fn at_rest_of<S>() -> <Bytes<{ <W13 as Facts<S>>::STRIDE / 8 }> as NativeFor>::T
where
    W13: Facts<S>,
    Bytes<{ <W13 as Facts<S>>::STRIDE / 8 }>: NativeFor,
{
    unimplemented!()
}
