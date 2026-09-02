// Question two's other remedy: the schema gains a column naming the adjudicating
// strategy.  What does that column cost when it is expressed as a key member rather
// than as prose?
//
// A key member is something a law instance is identified by.  Expressed in the type
// system, a free key member is a parameter.  This file adds exactly that and nothing
// else, on top of the shape p1 proved compiles.
//
// EXPECTED: refusal.  If the adjudicator is free, it is unconstrained, and a rule the
// implementation cannot recover from its inputs is not a rule.
//
// rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p2_adjudicator_as_free_key.rs

#![no_std]

use core::marker::PhantomData;

pub trait Strategy {
    const CLAMPS: bool;
}
pub struct Hot;
pub struct Warm;
impl Strategy for Hot {
    const CLAMPS: bool = false;
}
impl Strategy for Warm {
    const CLAMPS: bool = true;
}

pub trait Numeral {
    const LO: i128;
    const HI: i128;
}
pub struct Num<N, S>(PhantomData<(N, S)>);

pub trait Quantise<Src> {
    fn quantise(v: i128) -> i128;
}

// `A` is the adjudicating strategy, named as its own key member.
impl<SN, SS, TN, TS, A> Quantise<Num<SN, SS>> for Num<TN, TS>
where
    SN: Numeral,
    SS: Strategy,
    TN: Numeral,
    TS: Strategy,
    A: Strategy,
{
    fn quantise(v: i128) -> i128 {
        if A::CLAMPS && v > TN::HI {
            TN::HI
        } else {
            v
        }
    }
}
