// Probe C for panel file 10. The union's lie, attempted under the
// one-definition signatures.
//
// In `08_probes/a_union.rs:692-698`, `add` computed `C::over(max)` and
// every `Deliver::refuse(nearest)` received a caller-chosen value it
// could return as the answer. That is how a wrap preset clamped: the
// substitute value was IN SCOPE at the point of the lie.
//
// Under the one-definition shape, the refusal constructor receives no
// payload. This file attempts the same lie two ways and both are refused
// by name resolution and by the type system, not by a check:
//
//   1. `refused()` tries to return the clamp bound `max`: E0425, the name
//      does not exist in this scope. The signature is the enforcement.
//   2. `refused()` tries to fabricate any T at all from thin air with
//      only `T: Copy` in scope: there is no expression of type T to
//      write. The attempt below reaches for Default and gets E0599.
//
// (What remains writable is the branch lie, and probe B is the check
// that catches that. The two mechanisms partition the carrier's function
// space between them.)
//
// EXPECTED OUTCOME: does not compile. E0425 and E0599.
//
// rustc +nightly-2026-05-28 --edition 2024 c_substitution_unwritable.rs
#![feature(const_trait_impl)]
#![allow(dead_code)]

#[derive(Copy, Clone)]
pub enum Rec<T: Copy> {
    At(T),
    Refused,
}

pub const trait CarrierC<T: Copy>: Copy {
    fn from_output(v: T) -> Self;
    fn refused() -> Self;
    fn observe(self) -> Rec<T>;
}

#[derive(Copy, Clone)]
pub struct Clamping<T: Copy>(T);

const impl<T: Copy> CarrierC<T> for Clamping<T> {
    fn from_output(v: T) -> Self {
        Clamping(v)
    }
    fn refused() -> Self {
        // attempt 1: return the clamp bound, as the union's AsSum did.
        Clamping(max) // E0425: cannot find value `max` in this scope
    }
    fn observe(self) -> Rec<T> {
        Rec::At(self.0)
    }
}

#[derive(Copy, Clone)]
pub struct Fabricating<T: Copy>(T);

const impl<T: Copy> CarrierC<T> for Fabricating<T> {
    fn from_output(v: T) -> Self {
        Fabricating(v)
    }
    fn refused() -> Self {
        // attempt 2: fabricate a T. Only `T: Copy` is in scope; there is
        // no constructor, no Default, no value. E0599.
        Fabricating(T::default())
    }
    fn observe(self) -> Rec<T> {
        Rec::At(self.0)
    }
}

fn main() {}
