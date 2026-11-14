//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ending every exact step past the sum shares.
//!
//! The product, the quotient and the conversion each form a numerator in the
//! four-limb integer over a positive denominator in the same four limbs, and place
//! it: the floor is the slot, the residue reduced to lowest terms is the
//! remainder, and a floor the slot index does not hold is past it on the
//! numerator's sign. A numerator or denominator the four limbs do not hold is
//! named apart, as a step the limbs have not formed, because with a denominator
//! this wide it says nothing about where the position is.
//! Each starts from the phase in lowest terms and from the radix raised to the
//! two halves of the quantum's exponent. Those pieces live here, once.

use notko::Maybe;

use crate::apply::{Exact, Remainder};
use crate::format::{Format, greatest_common_divisor, radix};
use crate::quantum::Quantum;
use crate::slots::{Slot, Slots};
use crate::wide::{Divided, Wide};
use crate::width::Bool;

/// The phase `n / d` in lowest terms, `d` positive.
#[derive(Clone, Copy)]
pub(crate) struct Reduced {
    /// `n`, at most `2^63` in magnitude.
    pub(crate) numerator:   i128,
    /// `d`, from one up to `2^63`.
    pub(crate) denominator: i128,
}

/// The phase of `F` in lowest terms, or nothing when it does not denote.
///
/// The fraction's sign moves onto the numerator before it is reduced.
#[inline]
pub(crate) const fn reduced_phase<F: Format>() -> Maybe<Reduced> {
    if !F::PHASE.denotes().get() {
        return Maybe::Isnt;
    }
    Maybe::Is(reduced::<F>())
}

/// The phase of `F` in lowest terms, for a step that has forced `F`'s admission,
/// so the denominator is not zero and nothing is asked of it.
#[inline]
pub(crate) const fn admitted_phase<F: Format>() -> Reduced {
    let () = <F as Format>::ADMITTED;
    reduced::<F>()
}

/// The reduction itself, over a denominator the caller knows is not zero.
#[inline]
const fn reduced<F: Format>() -> Reduced {
    let num = F::PHASE.numerator() as i128;
    let den = F::PHASE.denominator() as i128;
    let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
    let common = greatest_common_divisor(num.abs(), den);
    Reduced {
        numerator:   num / common,
        denominator: den / common,
    }
}

/// The radix of `F` raised to the two halves of its quantum's exponent `e`.
#[derive(Clone, Copy)]
pub(crate) struct Powers {
    /// `U = r^max(e, 0)`, or nothing when it leaves the four limbs.
    pub(crate) up:   Maybe<Wide>,
    /// `V = r^max(-e, 0)`, or nothing when it leaves the four limbs.
    pub(crate) down: Maybe<Wide>,
}

/// `U` and `V` for `F`, read off its constant quantum's exponent.
#[inline]
pub(crate) const fn powers<F: Format>() -> Powers {
    let r = radix::<F>().base() as u64;
    let e = <F::Quantum as Quantum>::BASE.power();
    let (up, down) = if e < 0 { (0, e.unsigned_abs()) } else { (e.unsigned_abs(), 0) };
    Powers {
        up:   Wide::power(r, up),
        down: Wide::power(r, down),
    }
}

/// The product of two values in the four limbs, where either being lost loses
/// the product.
#[inline]
pub(crate) const fn times(a: Maybe<Wide>, b: Maybe<Wide>) -> Maybe<Wide> {
    match (a, b) {
        (Maybe::Is(a), Maybe::Is(b)) => a.times(b),
        _ => Maybe::Isnt,
    }
}

/// Where a step put a position: past an end of the index on one side, not formed
/// in the four limbs, or at a slot with a remainder's numerator over the
/// denominator.
#[derive(Clone, Copy)]
pub(crate) enum Placed {
    /// Past an end of the index: the floor and the residue over the divisor, both
    /// in the four limbs, the side the floor's sign.
    Past(Wide, Wide),
    /// A numerator or denominator the four limbs do not hold, for a position
    /// whose sign is negative when it says so.
    Unformed(bool),
    /// The floor slot and the remainder's numerator.
    At(i128, Wide),
}

/// The floor of `numerator` over a positive `divisor`, and its residue.
#[inline]
pub(crate) const fn over(numerator: Wide, divisor: Wide) -> Placed {
    let Divided {
        quotient,
        remainder,
    } = numerator.floor_divided(divisor);
    match quotient.index() {
        Maybe::Is(slot) => Placed::At(slot, remainder),
        Maybe::Isnt => Placed::Past(quotient, remainder),
    }
}

/// Whether the index holds a placed position, counted one slot past its floor
/// when it is not on the grid, since a mode may round it there.
#[inline]
pub(crate) const fn is_held(at: Placed) -> Bool {
    match at {
        Placed::Past(..) | Placed::Unformed(_) => Bool::FALSE,
        Placed::At(slot, remainder) => Bool::of(remainder.is_zero().get() || slot < i128::MAX),
    }
}

/// Whether a step formed its numerator and denominator in the four limbs, which
/// is asked of a step before whether the index holds what it placed.
#[inline]
pub(crate) const fn is_formed(at: Placed) -> Bool {
    Bool::of(!matches!(at, Placed::Unformed(_)))
}

/// Whether every verdict of an obligation holds, which is what its verdict
/// function answers.
#[inline]
pub(crate) const fn all<const N: usize>(verdicts: [Bool; N]) -> Bool {
    let mut i = 0;
    while i < N {
        if !verdicts[i].get() {
            return Bool::FALSE;
        }
        i += 1;
    }
    Bool::TRUE
}

/// The end of the index on the side of a position whose sign is `negative`.
#[inline]
pub(crate) const fn saturated(negative: bool) -> Exact {
    Exact::on_grid(Slot::at(if negative { i128::MIN } else { i128::MAX }))
}

/// A placed position as an exact one over `divisor`, its remainder in lowest
/// terms.
#[inline]
pub(crate) const fn positioned(divisor: Wide, at: Placed) -> Exact {
    match at {
        Placed::Past(floor, _) => saturated(floor.is_negative().get()),
        Placed::Unformed(negative) => saturated(negative),
        Placed::At(slot, remainder) => {
            if remainder.is_zero().get() {
                return Exact::on_grid(Slot::at(slot));
            }
            // The remainder is below the divisor, so it is already in `[0, 1)`
            // and the position is the floor slot itself.
            Exact::at(Slot::at(slot), Remainder::reduced(remainder, divisor))
        },
    }
}

/// A placed position as wrapping reads it, over `divisor`: a floor past the index
/// reduced modulo the range's count onto the range, its remainder kept, and
/// nothing for a step not formed.
///
/// Wrapping reads the floor's residue and the remainder and nothing else, and
/// rounding one slot up from the reduced floor is rounding one slot up from the
/// true one, which `adapt` wraps from the top to the bottom as it wraps any
/// position past the range. So a position past the index needs no wider slot.
#[inline]
pub(crate) const fn wrapped<S: Slots>(divisor: Wide, at: Placed) -> Maybe<Exact> {
    match at {
        Placed::At(..) => Maybe::Is(positioned(divisor, at)),
        Placed::Unformed(_) => Maybe::Isnt,
        Placed::Past(floor, remainder) => {
            let lo = S::MIN.index();
            let count = S::MAX.index() - lo + 1;
            let Maybe::Is(shifted) = floor.plus(Wide::of(-lo)) else {
                return Maybe::Isnt;
            };
            match shifted.floor_divided(Wide::of(count)).remainder.index() {
                Maybe::Is(residue) => {
                    Maybe::Is(positioned(divisor, Placed::At(lo + residue, remainder)))
                },
                Maybe::Isnt => Maybe::Isnt,
            }
        },
    }
}
