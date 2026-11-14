//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Arithmetic the tests carry themselves, for the operations the value hands
//! to the format.
//!
//! The quotient, the conversion and the product at a quantum exponent other
//! than zero are the format's function called on slots, so that function
//! cannot be what they are checked against. What is here computes the same
//! results from the members' values: a ratio of two integers rounded by a mode,
//! and a completion onto a range. It reads what a signature declares, its mode,
//! policy, range, radix and exponent, and calls nothing of the format to
//! compute.
//!
//! A ratio is formed in the host's 128-bit integer where its terms fit there,
//! and in the wide integer of the format's own suites where they do not, so no
//! shape is left out for want of room. `the_oracle.rs` holds the two against
//! each other where both answer.

use arvo_format::overflow::{Overflow, Policy};
use arvo_format::rounding::{Mode, Rounding};
use arvo_format::{Adaptation, DeclaredSignature, Format, Quantum, radix};
use arvo_numeral::Sole;
use notko::Maybe;

use super::big::Big;
use super::range;

/// The rounding mode the signature declares.
fn mode<S: DeclaredSignature>() -> Mode {
    <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE
}

/// Whether the signature completes by saturation. The other policy a value
/// admits is wrapping.
fn saturates<S: DeclaredSignature>() -> bool {
    matches!(
        <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY,
        Policy::Saturate
    )
}

/// A power of a radix, the quantum of a format or its inverse.
#[derive(Clone, Copy)]
pub struct Scale {
    pub radix: u32,
    pub power: i32,
}

impl Scale {
    /// The quantum of the format: a slot is its radix to this power.
    pub fn of<F: Format>() -> Self {
        Self {
            radix: radix::<F>().base(),
            power: <F::Quantum as Quantum>::BASE.power(),
        }
    }

    /// One over this scale.
    pub fn inverse(self) -> Self {
        Self {
            radix: self.radix,
            power: -self.power,
        }
    }
}

/// `num / den` rounded to an integer by the mode.
pub fn rounded(num: i128, den: i128, mode: Mode) -> i128 {
    assert!(den != 0, "a ratio over zero");
    let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
    let floor = num.div_euclid(den);
    let rest = num.rem_euclid(den);
    if rest == 0 {
        return floor;
    }
    let up = match mode {
        Mode::Floor => false,
        Mode::Ceil => true,
        Mode::TowardZero => num < 0,
        // A tie goes toward positive infinity.
        Mode::HalfUp => 2 * rest >= den,
        Mode::HalfEven => 2 * rest > den || (2 * rest == den && floor.rem_euclid(2) == 1),
        Mode::Stochastic => unreachable!("a value admits no stochastic mode"),
    };
    if up { floor + 1 } else { floor }
}

/// An integer completed onto the signature's range by its policy.
pub fn completed<S: Sole>(x: i128) -> i128 {
    let (min, max) = range::<S>();
    if saturates::<S>() {
        x.clamp(min, max)
    } else {
        min + (x - min).rem_euclid(max - min + 1)
    }
}

/// What an operation gives: the slot of its result, and whether the rounded
/// result was a member before any completion.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Landed {
    pub slot: i128,
    pub fits: bool,
}

/// The magnitude a ratio's terms stay within for `rounded` and `completed` to
/// stay inside the host's integer: `rounded` doubles a remainder below the
/// denominator, and `completed` takes a range's end from the rounded value.
const ROOM: u128 = 1 << 126;

/// The product of the factors and of each scale's side of the ratio, in the
/// host's integer, or nothing where a term leaves the room it has.
fn terms_in_room(factors: &[i128], divisor: i128, scales: &[Scale]) -> Option<(i128, i128)> {
    let mut num = 1i128;
    for &f in factors {
        num = num.checked_mul(f)?;
    }
    let mut den = divisor;
    for s in scales {
        let by = i128::from(s.radix).checked_pow(s.power.unsigned_abs())?;
        if s.power >= 0 {
            num = num.checked_mul(by)?;
        } else {
            den = den.checked_mul(by)?;
        }
    }
    (num.unsigned_abs() <= ROOM && den.unsigned_abs() < ROOM).then_some((num, den))
}

/// `num / den` landed on `S` in the host's integer.
fn landed_in_room<S: Sole>(num: i128, den: i128) -> Landed {
    let (min, max) = range::<S>();
    let at = rounded(num, den, mode::<S>());
    Landed {
        slot: completed::<S>(at),
        fits: at >= min && at <= max,
    }
}

/// The same ratio landed on `S` in the wide integer, the denominator as wide as
/// the numerator. The format forms the product's and the quotient's step four
/// 64-bit limbs wide, so a denominator below `2^256` is one it can admit.
fn landed_wide<S: Sole>(factors: &[i128], divisor: i128, scales: &[Scale]) -> Landed {
    let mut num = Big::of(if divisor < 0 { -1 } else { 1 });
    for &f in factors {
        num = num.times(Big::of(f));
    }
    let mut den = Big::of_unsigned(divisor.unsigned_abs());
    for s in scales {
        let by = Big::power(i128::from(s.radix), s.power.unsigned_abs());
        if s.power >= 0 {
            num = num.times(by);
        } else {
            den = den.times(by);
        }
    }
    let (floor, rest) = num.floor_over_big(den);
    // The remainder against its complement in the denominator: below zero
    // where the remainder is the smaller, zero at the tie.
    let against = rest.plus(den.plus(rest.negated()).negated());
    let zero = Big::of(0);
    let up = rest != zero
        && match mode::<S>() {
            Mode::Floor => false,
            Mode::Ceil => true,
            Mode::TowardZero => floor.is_negative(),
            Mode::HalfUp => !against.is_negative(),
            Mode::HalfEven => {
                (!against.is_negative() && against != zero)
                    || (against == zero && floor.floor_over(2).1 == 1)
            },
            Mode::Stochastic => unreachable!("a value admits no stochastic mode"),
        };
    let at = if up { floor.plus(Big::of(1)) } else { floor };
    let (min, max) = range::<S>();
    let fits = matches!(at.index(), Maybe::Is(i) if i >= min && i <= max);
    let slot = if saturates::<S>() {
        match at.index() {
            Maybe::Is(i) => i.clamp(min, max),
            Maybe::Isnt if at.is_negative() => min,
            Maybe::Isnt => max,
        }
    } else {
        let span = (max - min) as u128 + 1;
        min + at.plus(Big::of(-min)).floor_over(span).1 as i128
    };
    Landed {
        slot,
        fits,
    }
}

/// The product of the factors times the scales, over the divisor, landed on
/// `S`: rounded by its mode, then completed by its policy.
fn landed<S: Sole>(factors: &[i128], divisor: i128, scales: &[Scale]) -> Landed {
    assert!(divisor != 0, "a ratio over zero");
    match terms_in_room(factors, divisor, scales) {
        Some((num, den)) => landed_in_room::<S>(num, den),
        None => landed_wide::<S>(factors, divisor, scales),
    }
}

/// Both ways of landing one ratio, where the host's integer has room for it.
pub fn landed_both_ways<S: Sole>(
    factors: &[i128],
    divisor: i128,
    scales: &[Scale],
) -> Option<(Landed, Landed)> {
    let (num, den) = terms_in_room(factors, divisor, scales)?;
    Some((
        landed_in_room::<S>(num, den),
        landed_wide::<S>(factors, divisor, scales),
    ))
}

/// The quotient of the members at slots `a` and `b`. The two quanta cancel, so
/// the quotient is `a / b` as a number, and its slot is that over the quantum.
pub fn quotient<S: Sole>(a: i128, b: i128) -> Landed {
    landed::<S>(&[a], b, &[Scale::of::<S::Format>().inverse()])
}

/// The product of the members at slots `a` and `b`: the slots' product times
/// the quantum squared, and its slot is that over the quantum once.
pub fn product<S: Sole>(a: i128, b: i128) -> Landed {
    landed::<S>(&[a, b], 1, &[Scale::of::<S::Format>()])
}

/// The member at slot `a` of `S`, on the grid of `T`: the slot times the
/// source's quantum over the target's, whatever the two radixes are.
pub fn converted<S: Sole, T: Sole>(a: i128) -> Landed {
    landed::<T>(&[a], 1, &[
        Scale::of::<S::Format>(),
        Scale::of::<T::Format>().inverse(),
    ])
}
