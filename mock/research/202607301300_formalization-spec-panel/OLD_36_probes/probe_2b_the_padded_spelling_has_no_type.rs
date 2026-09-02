//! Probe 2b (the refusing half of probe 2): under the value-unique encoding
//! the spellings probe 1 exhibits cannot reach any position the design
//! bounds. `O<Z>` (a zero digit above an empty chain, the shape of
//! `UInt<UTerm, B0>`) is a well-formed type and is not a `Pos`, because
//! `O<P>` implements `Pos` only for `P: Pos` and `Z` is not one. Same for
//! `Pz<Z>` against `Nat`.
//!
//! This is the perimeter argument made mechanical rather than asserted: the
//! guarantee holds over the operations through which the type is observed,
//! the observation is the bound `Pos`, and the non-canonical spelling fails
//! it. No normalisation pass runs anywhere in probe 2 and none is needed.
//!
//! Committed refusing, on purpose. Do not "fix" this file: the two E0277s
//! below are the statement that the illegal state is unrepresentable in the
//! position that matters.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2b_the_padded_spelling_has_no_type.rs
//! Outcome: FAILS WITH two E0277s, verbatim in OUTCOMES.md, against
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

pub trait Pos {
    const VAL: u64;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

impl Pos for H {
    const VAL: u64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub trait Nat {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

/// Any generic width position in the design carries this bound. It is the
/// whole observation surface.
pub fn takes_a_width<N: Nat>() {}
pub fn takes_a_positive<P: Pos>() {}

/// The analogue of probe 1's `UInt<UTerm, B0>`: a zero digit above the
/// terminator. It has no `Pos` impl, so it cannot be a width.
pub fn the_padded_zero_is_not_a_positive() {
    takes_a_positive::<O<Z>>();
}

/// And zero has exactly one `Nat` spelling: `Pz<Z>` is not one.
pub fn zero_has_no_second_nat_spelling() {
    takes_a_width::<Pz<Z>>();
}
