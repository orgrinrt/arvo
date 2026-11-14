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
//! is a whole multiple of every finer one and the grids nest, and where the slot
//! range holds zero each magnitude's range holds every finer one's, which is what
//! makes the finest grid in range the nearest member of the union. A range that
//! does not hold zero over several magnitudes is refused, so nothing here checks
//! the nesting.
//!
//! The rounding and the range question are `panic_on_overflow`'s, asked once per
//! magnitude, so a landing and the map it hands its position to cannot disagree.
//! A point no magnitude holds is past an end of the union, and lands at the top
//! magnitude, where the overflow policy reads it as on any grid.

use core::cmp::Ordering;
use core::marker::PhantomData;
use core::num::NonZeroU128;

use notko::Maybe;

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::apply::{Dither, Exact, panic_on_overflow};
use crate::format::{Format, is_one_progression, radix, step_exponent};
use crate::overflow::{Overflow, Policy};
use crate::placing::{Reduced, over, positioned, reduced_phase, saturated, times};
use crate::quantum::{Exponent, Magnitude, Quantum};
use crate::slots::Slots;
use crate::wide::Wide;

/// A point of the ambient domain with no magnitude in it: `num / den * radix^power`.
///
/// The denominator is non-zero by its type, so a point always names a ratio and
/// building one checks nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Scaled {
    num:   i128,
    den:   NonZeroU128,
    power: Exponent,
}

impl Scaled {
    /// The point `num / den * radix^power`, in the radix of the format it is landed on.
    #[must_use]
    #[inline]
    pub const fn of(num: i128, den: NonZeroU128, power: Exponent) -> Self {
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
    /// The finest magnitude whose rounded position is in the slot range, or the
    /// top magnitude for a point past an end of the union.
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

/// What `land` asks of a signature before it lands anything on it.
struct Landable<S>(PhantomData<S>);

impl<S: DeclaredSignature> Landable<S> {
    /// One grid, or a slot range holding zero; and wrapping only on one
    /// progression, and over a magnitude-indexed one not yet.
    const ADMITTED: () = {
        let one_grid = <<S::Format as Format>::Quantum as Quantum>::SLOPE.power() == 0
            || <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count() <= 1;
        let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
        let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
        // FIXME(arvo_land_refuses_a_format_whose_slot_range_does_not_hold_ze):
        // between two magnitudes such a range can leave a gap no grid's rounding
        // reaches, where no one grid carries the tie or the dither.
        assert!(
            one_grid || (min <= 0 && max >= 0),
            "landing is not yet admitted on a magnitude-indexed format whose slot range does not \
             hold zero: the union of its magnitude ranges can have a gap between two magnitudes, \
             and no one grid there carries the mode's tie or dither"
        );
        let wraps = matches!(
            <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY,
            Policy::Wrap
        );
        assert!(
            !wraps || is_one_progression::<S::Format>().get(),
            "wrapping is refused on a format whose representable set is not one arithmetic \
             progression: a union of magnitude ranges at more than one quantum has a gap, so no \
             reduction of it is a group; saturate or clamp instead"
        );
        // FIXME(arvo_land_does_not_wrap_on_a_one_progression_set_over_severa): the
        // wrap there is in the flattened progression, not in the top slot range.
        assert!(
            !wraps || one_grid,
            "wrapping is not yet admitted on a magnitude-indexed format whose set is one \
             progression: the wrap is modulo the progression's count of finest steps, not the top \
             magnitude's slot range that the adaptation reads"
        );
    };
}

/// The magnitude a point lands on under `S`, and its position there.
///
/// The finest magnitude at which the point, rounded by `S`'s mode with `dither`,
/// is in the slot range. A point no magnitude holds is past an end of the union,
/// and lands at the top magnitude, where `adapt` reads it under `S`'s policy: a
/// saturation or a clamp pins it to that magnitude's end, which is the union's,
/// and a wrap is admitted only where the whole set is one grid. A constant quantum
/// has one magnitude, so there `land` is the rounding and the range question of
/// `panic_on_overflow`, and the completion of `adapt` past the range.
///
/// Nothing where the phase names no member, or where the top magnitude's position
/// is not formed in the four limbs short of being provably past the slot index.
#[inline]
pub const fn land<S: DeclaredSignature>(at: Scaled, dither: Dither) -> Maybe<Landing> {
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let () = Landable::<S>::ADMITTED;
    let Maybe::Is(phase) = reduced_phase::<S::Format>() else {
        return Maybe::Isnt;
    };
    let count = <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count();
    let mut i = first_candidate::<S::Format>(at, phase);
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
    let top = finest_first::<S::Format>(count - 1);
    match position_at::<S::Format>(at, phase, top) {
        Maybe::Is(exact) => {
            Maybe::Is(Landing {
                magnitude: top,
                exact,
            })
        },
        Maybe::Isnt => Maybe::Isnt,
    }
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
/// Every finer one puts the point past the slot range whatever the rounding, so it
/// is not walked. With `n` and `d` the point's numerator and denominator,
/// `|n| >= 2^(bits(n) - 1)`, `d < 2^bits(d)`, and `radix^t >= 2^(t L)` with `L` the
/// floor of `log2 radix` for `t >= 0` and its ceiling for `t < 0`, so a shift
/// `t = power - step` with `t L + bits(n) - 1 - bits(d)` at or above
/// `max(bits(range), 64) + 2` names a value term past twice the range's largest
/// magnitude. The phase's term is at most `2^63` slots wherever `BASE - step` is
/// not above zero, which is every magnitude of a rising law, so with the rounding's
/// one slot the position is past the range there. A falling law with a phase is
/// walked from its finest magnitude, where that term is largest.
#[inline]
const fn first_candidate<F: Format>(at: Scaled, phase: Reduced) -> u32 {
    let slope = <F::Quantum as Quantum>::SLOPE.power() as i64;
    if at.num == 0 || slope == 0 || (slope < 0 && phase.numerator != 0) {
        return 0;
    }
    let reach = {
        let lo = <F::Slots as Slots>::MIN.index().unsigned_abs();
        let hi = <F::Slots as Slots>::MAX.index().unsigned_abs();
        let top = if lo > hi { lo } else { hi };
        let bits = (u128::BITS - top.leading_zeros()) as i64;
        if bits > 64 { bits } else { 64 }
    };
    let r = radix::<F>().base();
    let floor_log = (u32::BITS - 1 - r.leading_zeros()) as i64;
    let ceil_log = if r.is_power_of_two() { floor_log } else { floor_log + 1 };
    let numerator_bits = (u128::BITS - at.num.unsigned_abs().leading_zeros()) as i64;
    let denominator_bits = (u128::BITS - at.den.get().leading_zeros()) as i64;
    let finest = step_exponent::<F>(finest_first::<F>(0)).power() as i64;
    // The smallest shift at which `t L(t)` reaches the bound.
    let bound = reach + 2 - (numerator_bits - 1 - denominator_bits);
    let least = if bound > 0 {
        (bound + floor_log - 1) / floor_log
    } else {
        -((-bound) / ceil_log)
    };
    // The `i`-th magnitude's shift is `power - finest - |slope| i`, so the ones
    // skipped are those with `|slope| i` at most this.
    let room = at.power.power() as i64 - finest - least;
    if room < 0 {
        return 0;
    }
    let i = room / slope.abs() + 1;
    let count = <F::Quantum as Quantum>::MAGNITUDES.count() as i64;
    if i > count { count as u32 } else { i as u32 }
}

/// `radix^e` in the four limbs, or nothing past them.
#[inline]
const fn power_of(r: u64, e: i64) -> Maybe<Wide> {
    if e > u32::MAX as i64 {
        return Maybe::Isnt;
    }
    Wide::power(r, e as u32)
}

/// Whether a value formed in the four limbs is under `2^255` in magnitude.
#[inline]
const fn under_half(value: Wide) -> bool {
    let Maybe::Is(half) = Wide::power(2, 255) else {
        return false;
    };
    let magnitude = if value.is_negative().get() { value.negated() } else { value };
    matches!(magnitude.ordering(half), Ordering::Less)
}

/// The end of the index on one side, where the divisor is under `2^128` and so a
/// numerator of at least `2^255` puts the position at least `2^127` slots out.
#[inline]
const fn past(divisor: Wide, negative: bool) -> Maybe<Exact> {
    match divisor.unsigned() {
        Maybe::Is(_) => Maybe::Is(saturated(negative)),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// The point's exact position on the grid of `magnitude`, the end of the index on
/// its side where it is past the index, or nothing where it is not formed.
///
/// A member there is `p/q radix^BASE + slot radix^s`, so with `t = power - s` and
/// `b = BASE - s` the position is `num/den radix^t - p/q radix^b`. Over one
/// denominator by `c = max(0, -t, -b)`, with `b` left out at a zero phase, it is
/// `num q radix^(t + c) - p den radix^(b + c)` over `D = den q radix^c`. Where one
/// term or their difference leaves the four limbs while the rest is under `2^255`
/// and `D` is under `2^128`, the position is past the index on the side of what
/// left.
#[inline]
const fn position_at<F: Format>(at: Scaled, phase: Reduced, magnitude: Magnitude) -> Maybe<Exact> {
    let r = radix::<F>().base() as u64;
    let s = step_exponent::<F>(magnitude).power() as i64;
    let t = at.power.power() as i64 - s;
    let b = <F::Quantum as Quantum>::BASE.power() as i64 - s;
    let phased = phase.numerator != 0;
    let mut c = if t < 0 { -t } else { 0 };
    if phased && -b > c {
        c = -b;
    }
    let divisor = match times(
        Wide::of_unsigned(at.den.get()).times(Wide::of(phase.denominator)),
        power_of(r, c),
    ) {
        Maybe::Is(v) if !v.is_zero().get() => v,
        _ => return Maybe::Isnt,
    };
    // Zero is zero at every shift, so its power is never formed.
    let value = if at.num == 0 {
        Maybe::Is(Wide::of(0))
    } else {
        times(
            Wide::of(at.num).times(Wide::of(phase.denominator)),
            power_of(r, t + c),
        )
    };
    let offset = if phased {
        times(
            Wide::of_unsigned(at.den.get()).times(Wide::of(phase.numerator)),
            power_of(r, b + c),
        )
    } else {
        Maybe::Is(Wide::of(0))
    };
    // A term that left the limbs is at least `2^256`, so against the other under
    // `2^255` it decides the sign: the value's is the numerator's, and the offset
    // is taken away, so it is negative where the phase is above zero.
    let numerator = match (value, offset) {
        (Maybe::Is(v), Maybe::Is(o)) => {
            match v.plus(o.negated()) {
                Maybe::Is(n) => n,
                // The two had one sign, and their sum left the limbs on it.
                Maybe::Isnt => return past(divisor, v.is_negative().get()),
            }
        },
        (Maybe::Isnt, Maybe::Is(o)) if under_half(o) => return past(divisor, at.num < 0),
        (Maybe::Is(v), Maybe::Isnt) if under_half(v) => {
            return past(divisor, phase.numerator > 0);
        },
        _ => return Maybe::Isnt,
    };
    Maybe::Is(positioned(divisor, over(numerator, divisor)))
}

#[cfg(test)]
mod tests;
