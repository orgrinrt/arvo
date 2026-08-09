// If the adjudicator cannot be a free key member (p2), can it be a derived one: an
// associated type computed from the source and target that are already in the key?
//
// EXPECTED: compiles.  And that is the finding, not a rescue of the column: a value
// derivable from members already present is not a new member.  Under 143b, which op
// called settled canon, a function whose value is derivable from its inputs is not
// data.
//
// rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p3_adjudicator_derived.rs

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

/// The adjudication rule, stated once, as a total function of the pair.
pub trait Adjudicate<Src> {
    type Governs: Strategy;
}

impl<SN, SS, TN, TS> Adjudicate<Num<SN, SS>> for Num<TN, TS>
where
    SN: Numeral,
    SS: Strategy,
    TN: Numeral,
    TS: Strategy,
{
    type Governs = TS;
}

pub trait Quantise<Src> {
    fn quantise(v: i128) -> i128;
}

impl<SN, SS, TN, TS> Quantise<Num<SN, SS>> for Num<TN, TS>
where
    SN: Numeral,
    SS: Strategy,
    TN: Numeral,
    TS: Strategy,
{
    fn quantise(v: i128) -> i128 {
        if <Self as Adjudicate<Num<SN, SS>>>::Governs::CLAMPS && v > TN::HI {
            TN::HI
        } else {
            v
        }
    }
}
