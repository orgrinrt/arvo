//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Landing a point of the ambient domain on a format's magnitudes.
//!
//! `adapt` lands a position on one slot index, and a magnitude-indexed format is
//! a union of grids, one per magnitude. Which grid a value belongs on is the
//! finest one at which its rounded position is in the slot range. The law is
//! affine in the magnitude over a radix of two or more, so each coarser quantum
//! is a whole multiple of every finer one and the grids nest, which is what makes
//! the finest grid in range the nearest member of the union. That holds for
//! every law the quantum concept admits, so nothing here checks it.
//!
//! The rounding and the range question are `panic_on_overflow`'s, asked once per
//! magnitude, so a landing and the map it hands its position to cannot disagree.

use notko::Maybe;

use crate::adapt::DeclaredSignature;
use crate::apply::{Dither, Exact, panic_on_overflow};
use crate::format::{Format, radix, step_exponent};
use crate::placing::{Placed, Reduced, over, positioned, reduced_phase, times};
use crate::quantum::{Exponent, Magnitude, Quantum};
use crate::slots::Slots;
use crate::wide::Wide;

/// A point of the ambient domain with no magnitude in it: `num / den * radix^power`.
///
/// The denominator is unsigned and taken as `Remainder::of` takes its own: a
/// zero one names no ratio and reads as zero, so the point is zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Scaled {
    num:   i128,
    den:   u128,
    power: Exponent,
}

impl Scaled {
    /// The point `num / den * radix^power`, in the radix of the format it is landed on.
    #[must_use]
    #[inline]
    pub const fn of(num: i128, den: u128, power: Exponent) -> Self {
        if den == 0 {
            return Self {
                num: 0,
                den: 1,
                power,
            };
        }
        Self {
            num,
            den,
            power,
        }
    }
}

/// The magnitude a point lands on, and its exact position on that magnitude's grid.
///
/// The slot is `adapt` of the position under the same signature and dither, so a
/// landing carries no slot of its own to disagree with the map.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Landing {
    magnitude: Magnitude,
    exact:     Exact,
}

impl Landing {
    /// The finest magnitude whose rounded position is in the slot range.
    #[must_use]
    #[inline]
    pub const fn magnitude(self) -> Magnitude {
        self.magnitude
    }

    /// The exact position on that magnitude's grid, for `adapt`.
    #[must_use]
    #[inline]
    pub const fn exact(self) -> Exact {
        self.exact
    }
}

/// The magnitude a point lands on under `S`, and its position there.
///
/// The finest magnitude at which the point, rounded by `S`'s mode with `dither`,
/// is in the slot range. A constant quantum has one magnitude, so there `land` is
/// the rounding and the range question of `panic_on_overflow`.
///
/// Nothing where no magnitude holds the rounded position: past the top of the
/// union, and where every position the point names is past the slot index or
/// carries a remainder over a denominator past `2^128 - 1`.
#[inline]
pub const fn land<S: DeclaredSignature>(at: Scaled, dither: Dither) -> Maybe<Landing> {
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let Maybe::Is(phase) = reduced_phase::<S::Format>() else {
        return Maybe::Isnt;
    };
    let count = <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count();
    let mut i = first_candidate::<S::Format>(at);
    while i < count {
        let magnitude = finest_first::<S::Format>(i);
        if let Maybe::Is(exact) = position_at::<S::Format>(at, phase, magnitude)
            && !panic_on_overflow::<S>(exact, dither).get()
        {
            return Maybe::Is(Landing {
                magnitude,
                exact,
            });
        }
        i += 1;
    }
    // FIXME(arvo_landing_a_value_on_a_magnitude_indexed_format_is_a_call): what
    // completion means past the top of a union of magnitude ranges is a canon
    // question, so a point no magnitude holds does not land anywhere yet.
    Maybe::Isnt
}

/// The `i`-th magnitude from the finest, which is magnitude zero for a law that
/// rises with magnitude and the largest for one that falls.
#[inline]
const fn finest_first<F: Format>(i: u32) -> Magnitude {
    let q = <F::Quantum as Quantum>::MAGNITUDES;
    if <F::Quantum as Quantum>::SLOPE.power() < 0 {
        Magnitude::at(q.largest().index() - i)
    } else {
        Magnitude::at(i)
    }
}

/// The first magnitude, counted from the finest, whose position could be in range.
///
/// Every finer one puts the point past the slot index whatever the rounding, so
/// it is not walked. With `n` and `d` the point's numerator and denominator,
/// `|n| >= 2^(bits(n) - 1)`, `d < 2^bits(d)`, and `radix^t >= 2^(t floor(log2 radix))`,
/// so a shift `t = power - step` with `t floor(log2 radix) + bits(n) - 1 - bits(d)`
/// at or above `max(bits(range), 64) + 2` names a position past twice the range's
/// largest magnitude, the phase's at most `2^63` slots and the rounding's one
/// slot included.
#[inline]
const fn first_candidate<F: Format>(at: Scaled) -> u32 {
    let slope = <F::Quantum as Quantum>::SLOPE.power() as i64;
    if at.num == 0 || slope == 0 {
        return 0;
    }
    let reach = {
        let lo = <F::Slots as Slots>::MIN.index().unsigned_abs();
        let hi = <F::Slots as Slots>::MAX.index().unsigned_abs();
        let top = if lo > hi { lo } else { hi };
        let bits = (u128::BITS - top.leading_zeros()) as i64;
        if bits > 64 { bits } else { 64 }
    };
    let log = (u32::BITS - 1 - radix::<F>().base().leading_zeros()) as i64;
    let numerator_bits = (u128::BITS - at.num.unsigned_abs().leading_zeros()) as i64;
    let denominator_bits = (u128::BITS - at.den.leading_zeros()) as i64;
    let finest = step_exponent::<F>(finest_first::<F>(0)).power() as i64;
    // The smallest `i` with `log (power - finest - |slope| i) < bound`.
    let bound = reach + 2 - numerator_bits + 1 + denominator_bits;
    let over = log * (at.power.power() as i64 - finest) - bound;
    if over < 0 {
        return 0;
    }
    let i = over / (log * slope.abs()) + 1;
    let count = <F::Quantum as Quantum>::MAGNITUDES.count() as i64;
    if i > count { count as u32 } else { i as u32 }
}

/// The point's exact position on the grid of `magnitude`, or nothing where that
/// position is past the slot index or its remainder's denominator is past
/// `2^128 - 1`.
///
/// With the phase `p / q` in lowest terms and the shift `t = power - step`, split
/// as `U = radix^max(t, 0)` and `V = radix^max(-t, 0)`, the position is
/// `n q U - p d V` over `D = d q V`. `D` is formed first, so where it fits, `p d V`
/// is under `2^191` and only the first term can leave the four limbs, on the sign
/// of `n`.
#[inline]
const fn position_at<F: Format>(at: Scaled, phase: Reduced, magnitude: Magnitude) -> Maybe<Exact> {
    let r = radix::<F>().base() as u64;
    let shift = at.power.power() as i64 - step_exponent::<F>(magnitude).power() as i64;
    let (up, down) = if shift < 0 { (0, shift.unsigned_abs()) } else { (shift.unsigned_abs(), 0) };
    // A shift past `u32` names a power past the four limbs in either direction.
    if up > u32::MAX as u64 || down > u32::MAX as u64 {
        return Maybe::Isnt;
    }
    let Maybe::Is(fall) = Wide::power(r, down as u32) else {
        return Maybe::Isnt;
    };
    let Maybe::Is(den_fall) = Wide::of_unsigned(at.den).times(fall) else {
        return Maybe::Isnt;
    };
    let divisor = match den_fall.times(Wide::of(phase.denominator)) {
        Maybe::Is(v) => {
            match v.unsigned() {
                Maybe::Is(u) => u,
                Maybe::Isnt => return Maybe::Isnt,
            }
        },
        Maybe::Isnt => return Maybe::Isnt,
    };
    let Maybe::Is(offset) = den_fall.times(Wide::of(phase.numerator)) else {
        return Maybe::Isnt;
    };
    // Zero is zero at every shift, so its power is never formed.
    let scaled = if at.num == 0 {
        Maybe::Is(Wide::of(0))
    } else {
        times(
            Wide::of(at.num).times(Wide::of(phase.denominator)),
            Wide::power(r, up as u32),
        )
    };
    let numerator = match scaled {
        Maybe::Is(s) => s.plus(offset.negated()),
        Maybe::Isnt => Maybe::Isnt,
    };
    let Maybe::Is(numerator) = numerator else {
        return Maybe::Isnt;
    };
    match over(numerator, divisor) {
        Placed::Past(_) => Maybe::Isnt,
        placed => Maybe::Is(positioned(divisor, placed)),
    }
}

#[cfg(test)]
mod tests;
