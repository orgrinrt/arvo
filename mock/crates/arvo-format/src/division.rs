//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient of two members at a constant quantum, by a divisor that cannot
//! be zero.
//!
//! Two members at a constant quantum `q` are `(phase + a) q` and
//! `(phase + b) q`, so their quotient is the member at position
//! `(phase + a) / ((phase + b) q) - phase`, a function of the declared signature
//! wherever `phase + b` is not zero. With the phase `n / d` in lowest terms,
//! `u = r^max(e, 0)` and `v = r^max(-e, 0)`, that position is the numerator
//! `d (n + a d) v - n (n + b d) u` over `d (n + b d) u`, both negated when
//! `n + b d` is below zero. The divisor is a `Divisor`, whose two constructors
//! keep the zero out before the program runs, so the step is never asked to
//! divide by it.
//!
//! One obligation carries four conditions: a phase that denotes, and three that
//! are not yet admitted rather than refused, a magnitude-indexed quantum, a
//! denominator past `2^128 - 1` at a member divisor, and a corner position the
//! slot index does not hold. It is forced by the exact step, and `is_divisible`
//! asks the same without forcing it.

use core::marker::PhantomData;

use notko::Maybe;

use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, adapt};
use crate::format::{Format, cancelling_slot, has_additive_identity};
use crate::placing::{
    Placed,
    Powers,
    Reduced,
    is_held,
    over,
    positioned,
    powers,
    reduced_phase,
    saturated,
    times,
};
use crate::quantum::{Magnitude, is_constant_family};
use crate::slots::{Slot, Slots, slot_in_range};
use crate::wide::Wide;
use crate::width::Bool;

/// Division under one declared signature.
///
/// It names the signature it is a function of and its arity, two, and carries
/// nothing else.
pub struct Div<S: DeclaredSignature>(PhantomData<S>);

impl<S: DeclaredSignature> Operation for Div<S> {
    type Signature = S;

    const ARITY: Arity = Arity::BINARY;
}

/// A slot of `F` that is not the format's zero member.
///
/// Built by `constant`, which refuses a zero or out-of-range constant at build
/// time, or by `of`, which is admitted only over a format that has no zero
/// member. Neither reads the slot's value to decide anything, so the property is
/// carried by the type rather than checked at the call.
pub struct Divisor<F: Format> {
    slot:   Slot,
    format: PhantomData<F>,
}

impl<F: Format> Clone for Divisor<F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<F: Format> Copy for Divisor<F> {}

/// What a constant divisor owes, checked rather than asked for.
struct ConstantDivisor<F, const B: i128>(PhantomData<F>);

impl<F: Format, const B: i128> ConstantDivisor<F, B> {
    const ADMITTED: () = {
        assert!(
            slot_in_range::<F::Slots>(Slot::at(B)).get(),
            "a constant divisor is refused outside the format's slot range: such a slot is not a \
             member, so it is no divisor of this format"
        );
        assert!(
            !matches!(cancelling_slot::<F>(Magnitude::SMALLEST), Maybe::Is(s) if s.index() == B),
            "a constant divisor is refused at the format's zero member: phase + B is zero there, and \
             a quotient by zero is not a function of the signature"
        );
    };
}

/// What a divisor read at run time owes: a format with no zero member.
struct NoZeroMember<F>(PhantomData<F>);

impl<F: Format> NoZeroMember<F> {
    const ADMITTED: () = assert!(
        !has_additive_identity::<F>().get(),
        "Divisor::of is refused over a format that has a zero member: a slot read at run time could \
         be that member, and telling it apart would be a check on the value. Name the divisor with \
         Divisor::constant, or declare a slot range that leaves the zero out"
    );
}

impl<F: Format> Divisor<F> {
    /// The divisor at the constant slot `B`, refused at build time when `B` is
    /// outside the range or is the zero member.
    #[must_use]
    pub const fn constant<const B: i128>() -> Self {
        let () = ConstantDivisor::<F, B>::ADMITTED;
        Self {
            slot:   Slot::at(B),
            format: PhantomData,
        }
    }

    /// The divisor at `slot`, admitted only over a format with no zero member.
    ///
    /// A slot outside the range is taken as every operation here takes one, and
    /// the step answers for it; the one such slot that names the zero, which
    /// exists only where the phase's denominator is one, saturates toward the
    /// positive end.
    #[must_use]
    pub const fn of(slot: Slot) -> Self {
        let () = NoZeroMember::<F>::ADMITTED;
        Self {
            slot,
            format: PhantomData,
        }
    }

    /// The slot the divisor sits at.
    #[must_use]
    pub const fn slot(self) -> Slot {
        self.slot
    }

    /// A divisor at `slot` with no obligation, for the sweeps, which divide by
    /// every non-zero member of a format whose range holds the zero.
    #[cfg(test)]
    pub(crate) const fn assumed(slot: Slot) -> Self {
        Self {
            slot,
            format: PhantomData,
        }
    }
}

/// What a format owes the quotient, checked rather than asked for.
struct Divisible<F>(PhantomData<F>);

impl<F: Format> Divisible<F> {
    /// The four conditions, in the order `is_divisible` asks them. The phase is
    /// asserted first because the others read its parts.
    const ADMITTED: () = {
        assert!(
            F::PHASE.denotes().get(),
            "division is refused over a phase whose denominator is zero: such a phase names no \
             member, so there is no quotient to position"
        );
        // FIXME(arvo_quotient_and_conversion_over_a_magnitude_indexed_quantu): the
        // quotient is a function of the signature here too. `Exact` and `adapt`
        // carry no magnitude coordinate to land it at.
        assert!(
            is_constant_family::<F::Quantum>().get(),
            "division is not yet admitted over a magnitude-indexed quantum: a slot names a \
             different value at each magnitude, and the applied map carries no magnitude \
             coordinate to land the quotient at yet"
        );
        // FIXME(arvo_s_exact_position_carries_no_remainder_denominator_past): the
        // position is a function of the signature here too, and only its
        // remainder's denominator is past the unsigned 128 bits it is carried over.
        assert!(
            denominator_fits::<F>().get(),
            "division is not yet admitted where the remainder's denominator d |n + b d| r^max(e, 0), \
             with n / d the reduced phase, b a member divisor, r the radix and e the quantum's \
             exponent, passes 2^128 - 1 at an end of the range: an exact position carries its \
             remainder over an unsigned 128-bit denominator"
        );
        // FIXME(arvo_quotient_and_conversion_positions_past_the_slot_index): the
        // corner positions are functions of the signature, and only the slot
        // index's own integer is too narrow to hold them.
        assert!(
            quotient_is_carried::<F>().get(),
            "division is not yet admitted over this format: the exact quotient of two members \
             leaves the slot coordinate, so the position it names is not one a slot index can \
             hold yet"
        );
    };
    /// The step's constants, or nothing where the phase does not denote.
    const STEP: Maybe<Step> = step::<F>();
}

/// What the step reads of the phase, the radix and the exponent.
#[derive(Clone, Copy)]
struct Step {
    /// `n` and `d`, the phase in lowest terms.
    phase:  Reduced,
    /// `u` and `v`, the radix to the two halves of the exponent.
    powers: Powers,
}

/// The step's constants for `F`, or nothing where the phase does not denote.
const fn step<F: Format>() -> Maybe<Step> {
    match reduced_phase::<F>() {
        Maybe::Is(phase) => {
            Maybe::Is(Step {
                phase,
                powers: powers::<F>(),
            })
        },
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// `n + a d`, the factor a slot brings, under `2^191` in magnitude for any slot.
const fn factor(step: Step, a: i128) -> Maybe<Wide> {
    match Wide::of(a).times(Wide::of(step.phase.denominator)) {
        Maybe::Is(ad) => ad.plus(Wide::of(step.phase.numerator)),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// Whether the slot `b` is a divisor the step can place by: its factor is
/// formed and is not zero.
const fn is_nonzero(step: Step, b: i128) -> bool {
    matches!(factor(step, b), Maybe::Is(f) if !f.is_zero().get())
}

/// `d |n + b d| u`, the denominator over the divisor's factor `fb`, or nothing
/// when it passes `2^128 - 1`. Zero where the factor is.
const fn denominator(step: Step, fb: Wide) -> Maybe<u128> {
    let magnitude = if fb.is_negative().get() { fb.negated() } else { fb };
    match times(
        times(
            Maybe::Is(Wide::of(step.phase.denominator)),
            Maybe::Is(magnitude),
        ),
        step.powers.up,
    ) {
        Maybe::Is(v) => v.unsigned(),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// The quotient of the slots `a` and `b` placed by the step.
///
/// The numerator `d (n + a d) v - n (n + b d) u` in four limbs, negated with the
/// denominator when `n + b d` is below zero, then its floor over the
/// denominator. A zero divisor factor saturates toward the positive end. A
/// denominator past `2^128 - 1`, a numerator that leaves the limbs, or a floor
/// that leaves the index, is past it on the sign of the quotient of the two
/// factors; for a member divisor the obligation has ruled all three out.
const fn placed(step: Step, a: i128, b: i128) -> (Placed, u128) {
    // Both factors are under `2^191`, so neither is lost; the arm is the
    // answer the step gives if one were.
    let (Maybe::Is(fa), Maybe::Is(fb)) = (factor(step, a), factor(step, b)) else {
        return (Placed::Past(false), 1);
    };
    if fb.is_zero().get() {
        return (Placed::Past(false), 1);
    }
    let flip = fb.is_negative().get();
    let negative = fa.is_negative().get() != flip;
    let Maybe::Is(divisor) = denominator(step, fb) else {
        return (Placed::Past(negative), 1);
    };
    let first = if fa.is_zero().get() {
        Maybe::Is(fa)
    } else {
        times(
            times(Maybe::Is(Wide::of(step.phase.denominator)), Maybe::Is(fa)),
            step.powers.down,
        )
    };
    // `|n (n + b d) u|` is at most `2^63` times the denominator, so it is formed.
    let second = times(
        times(Maybe::Is(Wide::of(step.phase.numerator)), Maybe::Is(fb)),
        step.powers.up,
    );
    let numerator = match (first, second) {
        (Maybe::Is(x), Maybe::Is(y)) => x.plus(y.negated()),
        _ => Maybe::Isnt,
    };
    match numerator {
        Maybe::Is(x) => (over(if flip { x.negated() } else { x }, divisor), divisor),
        Maybe::Isnt => (Placed::Past(negative), 1),
    }
}

/// Whether the denominator fits at every member divisor.
///
/// `|n + b d|` is affine in `b`, so the larger of the range's two ends decides
/// it. Nothing where the phase does not denote.
const fn denominator_fits<F: Format>() -> Bool {
    let Maybe::Is(step) = Divisible::<F>::STEP else {
        return Bool::FALSE;
    };
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    Bool::of(fits_at(step, min) && fits_at(step, max))
}

/// Whether the denominator at the divisor slot `b` fits.
const fn fits_at(step: Step, b: i128) -> bool {
    match factor(step, b) {
        Maybe::Is(fb) => matches!(denominator(step, fb), Maybe::Is(_)),
        Maybe::Isnt => false,
    }
}

/// The member divisors the quotient is extreme at: the range's two ends and the
/// members nearest the pole on each side, each skipped when it is not in range
/// or is the zero member.
const fn extreme_divisors(step: Step, min: i128, max: i128) -> [Maybe<i128>; 4] {
    let (n, d) = (step.phase.numerator, step.phase.denominator);
    // The smallest `b` with `n + b d` above zero, and the largest with it below.
    let above = (-n).div_euclid(d) + 1;
    let below = -(n.div_euclid(d)) - 1;
    let candidates = [min, max, above, below];
    let mut out = [Maybe::Isnt; 4];
    let mut i = 0;
    while i < 4 {
        let b = candidates[i];
        if b >= min && b <= max && is_nonzero(step, b) {
            out[i] = Maybe::Is(b);
        }
        i += 1;
    }
    out
}

/// Whether every quotient of two members with a non-zero divisor is a position
/// a slot index holds.
///
/// For a fixed divisor the position is affine in the dividend, and for a fixed
/// dividend it is monotone in the divisor on each side of the pole, so the eight
/// corners `extreme_divisors` names against the dividend's two ends decide it.
const fn quotient_is_carried<F: Format>() -> Bool {
    let Maybe::Is(step) = Divisible::<F>::STEP else {
        return Bool::FALSE;
    };
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    let divisors = extreme_divisors(step, min, max);
    let mut i = 0;
    while i < 4 {
        if let Maybe::Is(b) = divisors[i]
            && (!is_held(placed(step, min, b).0).get() || !is_held(placed(step, max, b).0).get()) {
                return Bool::FALSE;
            }
        i += 1;
    }
    Bool::TRUE
}

/// The exact position of the quotient of the member at slot `a` by `b`.
///
/// `(phase + a) / ((phase + b) q) - phase`, in slot units: on the grid, or at the
/// floor slot with the remainder over `d |n + b d| r^max(e, 0)` in lowest terms.
/// For two members the position is exact. For a slot outside the range, a
/// position past the index, a numerator past the four limbs or a denominator
/// past `2^128 - 1` saturates at the end on the quotient's sign, and the one
/// non-member that names the zero saturates toward the positive end.
///
/// Forces the division obligation, so a format the quotient does not admit stops
/// the build here, at check time when the call is bound in a `const` item.
#[must_use]
pub const fn quotient_position<F: Format>(a: Slot, b: Divisor<F>) -> Exact {
    let () = Divisible::<F>::ADMITTED;
    match Divisible::<F>::STEP {
        Maybe::Is(step) => {
            let (at, divisor) = placed(step, a.index(), b.slot.index());
            positioned(divisor, at)
        },
        // The obligation above has refused this format already.
        Maybe::Isnt => saturated(false),
    }
}

/// The quotient of the member at slot `a` by `b`, adapted under the signature.
///
/// The exact step, then `adapt` under the same signature, with the dither passed
/// through for the one mode that reads it. Total over every slot and every
/// divisor, with no check and no panic.
#[must_use]
pub const fn div<S: DeclaredSignature>(a: Slot, b: Divisor<S::Format>, dither: Dither) -> Slot {
    adapt::<S>(quotient_position::<S::Format>(a, b), dither)
}

/// Whether division admits the format, asked without forcing the obligation.
///
/// The same four conditions in the same order, so a test can hold a format the
/// quotient does not admit and report on it rather than stop the build.
#[must_use]
pub const fn is_divisible<F: Format>() -> Bool {
    if !F::PHASE.denotes().get() || !is_constant_family::<F::Quantum>().get() {
        return Bool::FALSE;
    }
    Bool::of(denominator_fits::<F>().get() && quotient_is_carried::<F>().get())
}

/// The step at slots `a` and `b` without forcing the obligation, for the edge
/// sweep, which reads the step at formats the obligation does not admit.
#[cfg(test)]
const fn unforced_position<F: Format>(a: Slot, b: Slot) -> Maybe<Exact> {
    match Divisible::<F>::STEP {
        Maybe::Is(step) => {
            let (at, divisor) = placed(step, a.index(), b.index());
            Maybe::Is(positioned(divisor, at))
        },
        Maybe::Isnt => Maybe::Isnt,
    }
}

#[cfg(test)]
mod tests;
