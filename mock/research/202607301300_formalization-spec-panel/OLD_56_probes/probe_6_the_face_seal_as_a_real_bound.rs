//! Probe 6: probe 4 found the face's reducedness check is EVALUATIVE, not
//! STRUCTURAL: `NFace<BAD>` is a well-formed type that compiles and can be
//! passed around freely; only a call to `.checked()` panics, and nothing
//! forces that call. That is a weaker guarantee than the internal tower's
//! own seal, where a malformed `Ratio<N, D>` simply never satisfies
//! `Adjustment`'s bound at all (`E0271` at the bound, never a panic at
//! use), which is exactly the perimeter
//! `what-you-can-observe-is-what-you-guaranteed.md` asks about: is there a
//! way to reach a `NFace<S>` for which reducedness does not hold. Probe 4
//! answered yes.
//!
//! This probe tests the standard bridge from a boolean const condition to
//! a real trait bound (`Assert<{cond}>: True`), so a bad spec fails at the
//! BOUND, matching the tower's own seal shape, rather than only when code
//! happens to call a specific constructor.
//!
//! EXPECTED: unknown going in.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   probe_6_the_face_seal_as_a_real_bound.rs

#![allow(dead_code)]
#![feature(adt_const_params)]
use core::marker::ConstParamTy;
use core::marker::PhantomData;

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Spec {
    pub precision: u16,
    pub bias_num: u64,
    pub bias_den: u64,
}
impl Spec {
    const fn is_reduced(&self) -> bool {
        gcd(self.bias_num, self.bias_den) == 1
    }
}

pub trait True {}
pub struct Assert<const B: bool>;
impl True for Assert<true> {}

pub struct NFace<const S: Spec>(PhantomData<()>);

// The bound: every consuming position that wants a WELL-FORMED face names
// this, exactly the shape the internal tower's own `Adjustment` bound uses
// (`N: Pos + Gcd<D, Out = H>`), structural refusal rather than a forced
// evaluation.
pub fn declare<const S: Spec>(_: NFace<S>)
where
    Assert<{ S.is_reduced() }>: True,
{
}

pub const GOOD: Spec = Spec {
    precision: 15,
    bias_num: 1,
    bias_den: 2,
};
pub const BAD: Spec = Spec {
    precision: 15,
    bias_num: 6,
    bias_den: 12,
};

pub fn uses_good(x: NFace<GOOD>) {
    declare(x);
}
pub fn uses_bad(x: NFace<BAD>) {
    declare(x);
}
