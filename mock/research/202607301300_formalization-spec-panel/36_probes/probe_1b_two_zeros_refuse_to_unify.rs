//! Probe 1b (the refusing half of probe 1): the two `Width` inhabitants of
//! zero are distinct types, so a signature naming one refuses the other.
//! This is probe 5b's E0308 (`34:320`) reproduced one layer down, on the
//! width chain rather than on the rational adjustment, which is the part
//! file 34 believed was already safe.
//!
//! Committed refusing, on purpose, per
//! `a-test-that-cannot-compile-is-the-finding.md`. Do not "fix" this file.
//! The fix is the encoding change in probe 2, after which the second zero
//! cannot be spelled at all and this file has nothing to say.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1b_two_zeros_refuse_to_unify.rs
//! Outcome: FAILS WITH E0308, verbatim in OUTCOMES.md, against
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

pub trait Bit {
    const VAL: u16;
}
pub struct B0;
pub struct B1;
impl Bit for B0 {
    const VAL: u16 = 0;
}
impl Bit for B1 {
    const VAL: u16 = 1;
}

pub trait Width {
    const VALUE: u16;
}
pub struct UTerm;
pub struct UInt<Hi, Lo>(PhantomData<(Hi, Lo)>);
impl Width for UTerm {
    const VALUE: u16 = 0;
}
impl<Hi: Width, Lo: Bit> Width for UInt<Hi, Lo> {
    const VALUE: u16 = Hi::VALUE * 2 + Lo::VAL;
}

/// The same type-equality demand probe 5 used, which is the cheapest one
/// available without unstable machinery.
pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

/// Both denote the width zero (probe 1 asserts it). They are not the same
/// type, so this call does not type-check.
pub fn the_two_zeros_are_one_width_and_two_types() {
    same_type(PhantomData::<UTerm>, PhantomData::<UInt<UTerm, B0>>);
}
