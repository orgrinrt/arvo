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
use crate::placing::{Placed, Reduced, admitted_phase, positioned, saturated, times};
use crate::quantum::{Exponent, Magnitude, Quantum};
use crate::slots::Slots;
use crate::wide::{Divided, Wide};

/// A point of the ambient domain with no magnitude in it: `num / den * radix^power`.
///
/// The denominator is non-zero by its type, so a point always names a ratio and
/// building one checks nothing. It has no equality: equality of the parts would
/// call `1/2` and `2/4` different, and nothing here asks whether two points are one.
#[derive(Clone, Copy, Debug)]
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
/// Nothing where the walk meets a magnitude whose position the four limbs neither
/// form nor show to be past the range, since the point may sit there.
#[inline]
pub const fn land<S: DeclaredSignature>(at: Scaled, dither: Dither) -> Maybe<Landing> {
    let () = <S::Format as Format>::ADMITTED;
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let () = Landable::<S>::ADMITTED;
    let Maybe::Is(distance) = distance::<S::Format>(at, admitted_phase::<S::Format>()) else {
        return Maybe::Isnt;
    };
    let wraps = matches!(
        <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY,
        Policy::Wrap
    );
    let last = <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count() - 1;
    let start = first_candidate::<S::Format>(distance);
    let mut i = if start > last { last } else { start };
    loop {
        let magnitude = finest_first::<S::Format>(i);
        let Maybe::Is(exact) = position_at::<S::Format>(distance, magnitude, wraps) else {
            return Maybe::Isnt;
        };
        // The top magnitude takes what no finer one holds, past the range or not.
        if i == last || !panic_on_overflow::<S>(exact, dither).get() {
            return Maybe::Is(Landing {
                magnitude,
                exact,
            });
        }
        i += 1;
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

/// The point's distance from the phase's point, `k / d0` in units of `radix^e`.
///
/// One quantity for every magnitude, so it is formed once and no magnitude's
/// numerator holds two terms that cancel.
#[derive(Clone, Copy)]
struct Distance {
    /// `k`, or the side on which it left the four limbs.
    k:  Term,
    /// `d0 = den q`, positive and under `2^191`.
    d0: Wide,
    /// `e = min(power, BASE)`, or `power` at a zero phase.
    e:  i64,
}

/// A difference formed in the four limbs, or the side it left them on.
#[derive(Clone, Copy)]
enum Term {
    /// The difference.
    Formed(Wide),
    /// At least `2^255` in magnitude, negative where it says so: one term left
    /// the limbs while the other is under `2^255`.
    Past(bool),
}

/// `num q radix^(power - e) - p den radix^(BASE - e)` over `den q`, with
/// `e = min(power, BASE)` and the phase's term left out where it is zero, so a
/// zero phase never forms a power it does not need. Nothing where both terms
/// leave the four limbs, since neither then decides the side.
#[inline]
const fn distance<F: Format>(at: Scaled, phase: Reduced) -> Maybe<Distance> {
    let r = radix::<F>().base() as u64;
    let power = at.power.power() as i64;
    let base = <F::Quantum as Quantum>::BASE.power() as i64;
    let phased = phase.numerator != 0;
    let e = if phased && base < power { base } else { power };
    let Maybe::Is(d0) = Wide::of_unsigned(at.den.get()).times(Wide::of(phase.denominator)) else {
        return Maybe::Isnt;
    };
    // Zero is zero at every shift, so its power is never formed.
    let value = if at.num == 0 {
        Maybe::Is(Wide::of(0))
    } else {
        times(
            Wide::of(at.num).times(Wide::of(phase.denominator)),
            power_of(r, power - e),
        )
    };
    let offset = if phased {
        times(
            Wide::of_unsigned(at.den.get()).times(Wide::of(phase.numerator)),
            power_of(r, base - e),
        )
    } else {
        Maybe::Is(Wide::of(0))
    };
    // A term that left the limbs is at least `2^256`, so against the other under
    // `2^255` it decides the side: the value's is the numerator's, and the offset
    // is taken away, so it is negative where the phase is above zero.
    let k = match (value, offset) {
        (Maybe::Is(v), Maybe::Is(o)) => {
            match v.plus(o.negated()) {
                Maybe::Is(k) => Term::Formed(k),
                // The two had one sign, and their sum left the limbs on it.
                Maybe::Isnt => Term::Past(v.is_negative().get()),
            }
        },
        (Maybe::Isnt, Maybe::Is(o)) if under_half(o) => Term::Past(at.num < 0),
        (Maybe::Is(v), Maybe::Isnt) if under_half(v) => Term::Past(phase.numerator > 0),
        _ => return Maybe::Isnt,
    };
    Maybe::Is(Distance {
        k,
        d0,
        e,
    })
}

/// The first magnitude, counted from the finest, whose position could be in range.
///
/// Every finer one puts the point past the slot range whatever the rounding, so it
/// is not walked. At the magnitude of step `radix^s`, with `m = e - s`, the
/// position is `k radix^m / d0`, and `|k| >= 2^(bits(k) - 1)`, `d0 < 2^bits(d0)`
/// and `radix^m >= 2^(m L)` with `L` the floor of `log2 radix` for `m >= 0` and
/// its ceiling below. So a shift with `m L + bits(k) - 1 - bits(d0)` at or above
/// `max(bits(range), 64) + 2` names a position past twice the range's largest
/// magnitude, the rounding's one slot included. A `k` past the limbs counts as 256
/// bits, fewer than it has. The phase is inside `k`, so this holds on every law.
#[inline]
const fn first_candidate<F: Format>(distance: Distance) -> u32 {
    let slope = <F::Quantum as Quantum>::SLOPE.power() as i64;
    let k_bits = match distance.k {
        Term::Formed(k) => k.bits() as i64,
        Term::Past(_) => 256,
    };
    if k_bits == 0 || slope == 0 {
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
    let finest = step_exponent::<F>(finest_first::<F>(0)).power() as i64;
    // The smallest shift at which `m L(m)` reaches the bound.
    let bound = reach + 2 - (k_bits - 1 - distance.d0.bits() as i64);
    let least = if bound > 0 {
        (bound + floor_log - 1) / floor_log
    } else {
        -((-bound) / ceil_log)
    };
    // The `i`-th magnitude's shift is `e - finest - |slope| i`, so the ones
    // skipped are those with `|slope| i` at most this.
    let room = distance.e - finest - least;
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

/// Whether a numerator of at least `2^255` over `divisor` is past the slot range
/// of `F`: so where the divisor times one more than the range's largest magnitude
/// is under `2^255`.
#[inline]
const fn past_the_range<F: Format>(divisor: Wide) -> bool {
    let lo = <F::Slots as Slots>::MIN.index().unsigned_abs();
    let hi = <F::Slots as Slots>::MAX.index().unsigned_abs();
    let reach = if lo > hi { lo } else { hi };
    matches!(divisor.times(Wide::of_unsigned(reach + 1)), Maybe::Is(v) if under_half(v))
}

/// The point's exact position on the grid of `magnitude`, or nothing where the
/// four limbs neither form it nor show it past the range.
///
/// With `m = e - s` for the step `radix^s`, the position is `k radix^max(m, 0)`
/// over `d0 radix^max(-m, 0)`. Past the index it is the end of the index on its
/// side, which `adapt` pins at the range's end; under wrapping it is the floor
/// reduced modulo the range's count onto the range, the remainder kept, since a
/// wrap reads the residue and the end has lost it.
#[inline]
const fn position_at<F: Format>(
    distance: Distance,
    magnitude: Magnitude,
    wraps: bool,
) -> Maybe<Exact> {
    let r = radix::<F>().base() as u64;
    let m = distance.e - step_exponent::<F>(magnitude).power() as i64;
    let divisor = if m < 0 {
        match times(Maybe::Is(distance.d0), power_of(r, -m)) {
            Maybe::Is(v) => v,
            Maybe::Isnt => return Maybe::Isnt,
        }
    } else {
        distance.d0
    };
    let numerator = match distance.k {
        Term::Formed(k) if m > 0 => times(Maybe::Is(k), power_of(r, m)),
        Term::Formed(k) => Maybe::Is(k),
        Term::Past(negative) => {
            return if !wraps && past_the_range::<F>(divisor) {
                Maybe::Is(saturated(negative))
            } else {
                Maybe::Isnt
            };
        },
    };
    let Maybe::Is(numerator) = numerator else {
        // `k radix^m` left the limbs, so it is at least `2^256` on `k`'s side.
        let negative = matches!(distance.k, Term::Formed(k) if k.is_negative().get());
        return if !wraps && past_the_range::<F>(divisor) {
            Maybe::Is(saturated(negative))
        } else {
            Maybe::Isnt
        };
    };
    let Divided {
        quotient,
        remainder,
    } = numerator.floor_divided(divisor);
    match quotient.index() {
        Maybe::Is(slot) => Maybe::Is(positioned(divisor, Placed::At(slot, remainder))),
        Maybe::Isnt if !wraps => Maybe::Is(saturated(quotient.is_negative().get())),
        Maybe::Isnt => {
            let lo = <F::Slots as Slots>::MIN.index();
            let count = <F::Slots as Slots>::MAX.index() - lo + 1;
            let Maybe::Is(shifted) = quotient.plus(Wide::of(-lo)) else {
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

#[cfg(test)]
mod tests;
