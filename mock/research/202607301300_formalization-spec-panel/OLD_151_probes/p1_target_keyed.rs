// Can a conversion be keyed on the target's strategy alone, with the source's strategy
// free, under the pin and with no feature gate at all?
//
// This is the compiled half of the answer to question two.  If the target-keyed shape
// needs no gate, then the sentence "a conversion's resolutions and direction are the
// target's" is implementable, and nothing in the type system is pushing toward a key
// column that names an adjudicator.
//
// Deliberately gate-free: no #![feature] line anywhere in this file.  A gate appearing
// here would be a finding against the shape, not a repair to the probe.
//
// rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p1_target_keyed.rs

#![no_std]

use core::marker::PhantomData;

pub enum Over {
    Wrap,
    Clamp,
    Refuse,
}
pub enum Dir {
    ToEven,
    TowardZero,
}

/// What a strategy supplies to a quantiser: an out-of-range disposition on each side
/// and an in-range direction.  This is the preset table's content, not the strategy's
/// name, and it is what the law key names.
pub trait Strategy {
    const OVER: Over;
    const UNDER: Over;
    const DIR: Dir;
}

pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

impl Strategy for Hot {
    const OVER: Over = Over::Wrap;
    const UNDER: Over = Over::Wrap;
    const DIR: Dir = Dir::TowardZero;
}
impl Strategy for Warm {
    const OVER: Over = Over::Clamp;
    const UNDER: Over = Over::Clamp;
    const DIR: Dir = Dir::ToEven;
}
impl Strategy for Cold {
    const OVER: Over = Over::Clamp;
    const UNDER: Over = Over::Clamp;
    const DIR: Dir = Dir::TowardZero;
}
impl Strategy for Precise {
    const OVER: Over = Over::Refuse;
    const UNDER: Over = Over::Refuse;
    const DIR: Dir = Dir::ToEven;
}

/// The numeral's four declared members, reduced to what a conversion reads: the grid
/// step and the two endpoints, in units of the step.  The strategy is not a member.
pub trait Numeral {
    const LO: i128;
    const HI: i128;
    const STEP: i128;
}

/// A numeral carried with the strategy the consumer declared for it.
pub struct Num<N, S>(PhantomData<(N, S)>);

/// The conversion.  The source's strategy `SS` appears in the trait's parameter and is
/// therefore constrained, and it appears nowhere in the body.
pub trait Quantise<Src> {
    fn quantise(v: i128) -> Option<i128>;
}

impl<SN, SS, TN, TS> Quantise<Num<SN, SS>> for Num<TN, TS>
where
    SN: Numeral,
    SS: Strategy,
    TN: Numeral,
    TS: Strategy,
{
    fn quantise(v: i128) -> Option<i128> {
        // round onto the target's grid, using the target's direction
        let r = match TS::DIR {
            Dir::TowardZero => v - v.rem_euclid(TN::STEP),
            Dir::ToEven => {
                let q = v.div_euclid(TN::STEP);
                let rem = v.rem_euclid(TN::STEP);
                let up = 2 * rem > TN::STEP || (2 * rem == TN::STEP && q % 2 != 0);
                (q + if up { 1 } else { 0 }) * TN::STEP
            }
        };
        // classify against the target's range, using the target's dispositions
        if r > TN::HI {
            match TS::OVER {
                Over::Wrap => Some(TN::LO + (r - TN::LO).rem_euclid(TN::HI - TN::LO + TN::STEP)),
                Over::Clamp => Some(TN::HI),
                Over::Refuse => None,
            }
        } else if r < TN::LO {
            match TS::UNDER {
                Over::Wrap => Some(TN::LO + (r - TN::LO).rem_euclid(TN::HI - TN::LO + TN::STEP)),
                Over::Clamp => Some(TN::LO),
                Over::Refuse => None,
            }
        } else {
            Some(r)
        }
    }
}

// Three concrete numerals, one per sign domain, so the call sites below exercise the
// mixed-domain conversions the sign-domain question is about.
pub struct NonNeg4;
pub struct Sym4;
pub struct AsymLow4;
impl Numeral for NonNeg4 {
    const LO: i128 = 0;
    const HI: i128 = 15;
    const STEP: i128 = 1;
}
impl Numeral for Sym4 {
    const LO: i128 = -7;
    const HI: i128 = 7;
    const STEP: i128 = 1;
}
impl Numeral for AsymLow4 {
    const LO: i128 = -8;
    const HI: i128 = 7;
    const STEP: i128 = 1;
}

/// Four call sites where the source strategy differs and nothing else does.  If the
/// source strategy were adjudicating anything, these would have to differ.
pub fn source_strategy_is_inert(v: i128) -> [Option<i128>; 4] {
    [
        <Num<Sym4, Warm> as Quantise<Num<NonNeg4, Hot>>>::quantise(v),
        <Num<Sym4, Warm> as Quantise<Num<NonNeg4, Warm>>>::quantise(v),
        <Num<Sym4, Warm> as Quantise<Num<NonNeg4, Cold>>>::quantise(v),
        <Num<Sym4, Warm> as Quantise<Num<NonNeg4, Precise>>>::quantise(v),
    ]
}

/// The same four with the target strategy varying instead.  These are expected to
/// differ, and that is the asymmetry the sentence names.
pub fn target_strategy_governs(v: i128) -> [Option<i128>; 4] {
    [
        <Num<AsymLow4, Hot> as Quantise<Num<NonNeg4, Warm>>>::quantise(v),
        <Num<AsymLow4, Warm> as Quantise<Num<NonNeg4, Warm>>>::quantise(v),
        <Num<AsymLow4, Cold> as Quantise<Num<NonNeg4, Warm>>>::quantise(v),
        <Num<AsymLow4, Precise> as Quantise<Num<NonNeg4, Warm>>>::quantise(v),
    ]
}
