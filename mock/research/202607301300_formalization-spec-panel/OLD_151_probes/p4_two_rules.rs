// Could the adjudication vary per numeral pair, so that some conversions are governed
// by the source and others by the target?  That is what a genuine key COLUMN would
// mean: a member whose value differs from instance to instance.
//
// EXPECTED: refusal.  Two adjudication rules over the same pair overlap, and the
// overlap is at the head constructor rather than under a substitution, so it is
// structural rather than a coincidence of the chosen shapes.
//
// rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p4_two_rules.rs

#![no_std]

use core::marker::PhantomData;

pub trait Strategy {}
pub struct Hot;
pub struct Warm;
impl Strategy for Hot {}
impl Strategy for Warm {}

pub trait Numeral {}
pub struct Num<N, S>(PhantomData<(N, S)>);

pub trait Adjudicate<Src> {
    type Governs: Strategy;
}

// Rule one: the target governs.
impl<SN, SS, TN, TS> Adjudicate<Num<SN, SS>> for Num<TN, TS>
where
    SN: Numeral,
    SS: Strategy,
    TN: Numeral,
    TS: Strategy,
{
    type Governs = TS;
}

// Rule two: the source governs.
impl<SN, SS, TN, TS> Adjudicate<Num<SN, SS>> for Num<TN, TS>
where
    SN: Numeral,
    SS: Strategy,
    TN: Numeral,
    TS: Strategy,
{
    type Governs = SS;
}
