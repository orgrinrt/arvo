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
//! are not yet admitted rather than refused, a magnitude-indexed quantum, a step
//! whose denominator or corner numerators pass the four limbs, and a corner
//! position the slot index does not hold. It is forced by the exact step, and `is_divisible`
//! asks the same without forcing it.

use core::marker::PhantomData;

use notko::Maybe;

use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, adapt};
use crate::format::{Format, cancelling_slot};
use crate::placing::{
    Placed,
    Powers,
    Reduced,
    all,
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
/// time, or by `of`, which is admitted only where no slot of the format names the
/// zero. Neither reads the slot's value to decide anything, so the property is
/// carried by the type rather than checked at the call, and the step never meets
/// a zero factor.
pub struct Divisor<F: Format> {
    slot:   Slot,
    format: PhantomData<F>,
}

impl<F: Format> Clone for Divisor<F> {
    #[inline]
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

/// What a divisor read at run time owes: a format in which no slot names the
/// zero.
///
/// `phase + b` is zero only at `b = -n / d`, an integer only when the reduced
/// denominator `d` is one, so past one no slot is the zero, member or not. A slot
/// is not typed as a member of its format, so at `d = 1` the slot `-n` could be
/// handed in whether or not the range holds it.
struct NoSlotNamesTheZero<F>(PhantomData<F>);

impl<F: Format> NoSlotNamesTheZero<F> {
    // FIXME(arvo_a_runtime_divisor_over_a_whole_phase_range_without_the): a range
    // that leaves the zero out at a whole phase has no zero member, and only the
    // slot's lack of a membership type keeps it from dividing by a runtime slot.
    const ADMITTED: () = assert!(
        no_slot_names_the_zero::<F>().get(),
        "Divisor::of is refused where a slot names the zero, which is wherever the phase's \
         denominator in lowest terms is one: a slot read at run time could be that slot, and \
         telling it apart would be a check on the value. Name the divisor with Divisor::constant, \
         or declare a phase whose denominator is past one"
    );
}

/// Whether no slot of `F` names the zero: its phase denotes, and its reduced
/// denominator is past one.
#[inline]
const fn no_slot_names_the_zero<F: Format>() -> Bool {
    Bool::of(matches!(reduced_phase::<F>(), Maybe::Is(phase) if phase.denominator > 1))
}

impl<F: Format> Divisor<F> {
    /// The divisor at the constant slot `B`, refused at build time when `B` is
    /// outside the range or is the zero member.
    #[must_use]
    #[inline]
    pub const fn constant<const B: i128>() -> Self {
        let () = ConstantDivisor::<F, B>::ADMITTED;
        Self {
            slot:   Slot::at(B),
            format: PhantomData,
        }
    }

    /// The divisor at `slot`, admitted only where no slot of `F` names the zero.
    ///
    /// A slot outside the range is taken as every operation here takes one, and
    /// the step answers for it as for any position past the index. None of them
    /// is the zero, so the step never meets one.
    #[must_use]
    #[inline]
    pub const fn of(slot: Slot) -> Self {
        let () = NoSlotNamesTheZero::<F>::ADMITTED;
        Self {
            slot,
            format: PhantomData,
        }
    }

    /// The slot the divisor sits at.
    #[must_use]
    #[inline]
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
    /// The four conditions, each asserted from `conditions`, which `is_divisible`
    /// folds. The phase is first because the others read its parts.
    const ADMITTED: () = {
        let [denotes, constant, fits, carried] = conditions::<F>();
        assert!(
            denotes.get(),
            "division is refused over a phase whose denominator is zero: such a phase names no \
             member, so there is no quotient to position"
        );
        // FIXME(arvo_quotient_and_conversion_over_a_magnitude_indexed_quantu): the
        // quotient is a function of the signature here too. `Exact` and `adapt`
        // carry no magnitude coordinate to land it at.
        assert!(
            constant.get(),
            "division is not yet admitted over a magnitude-indexed quantum: a slot names a \
             different value at each magnitude, and the applied map carries no magnitude \
             coordinate to land the quotient at yet"
        );
        // FIXME(arvo_s_exact_position_carries_no_remainder_denominator_past): the
        // position is a function of the signature here too, and only its
        // numerator and denominator are past the four limbs they are formed in.
        assert!(
            fits.get(),
            "division is not yet admitted where the remainder's denominator d |n + b d| r^max(e, 0), \
             with n / d the reduced phase, b a member divisor, r the radix and e the quantum's \
             exponent, or the numerator at a corner of the range, passes 2^256 - 1: an exact step is \
             formed in four 64-bit limbs"
        );
        // FIXME(arvo_quotient_and_conversion_positions_past_the_slot_index): the
        // corner positions are functions of the signature, and only the slot
        // index's own integer is too narrow to hold them.
        assert!(
            carried.get(),
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
#[inline]
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
#[inline]
const fn factor(step: Step, a: i128) -> Maybe<Wide> {
    match Wide::of(a).times(Wide::of(step.phase.denominator)) {
        Maybe::Is(ad) => ad.plus(Wide::of(step.phase.numerator)),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// Whether the slot `b` is a divisor the step can place by: its factor is
/// formed and is not zero.
#[inline]
const fn is_nonzero(step: Step, b: i128) -> bool {
    matches!(factor(step, b), Maybe::Is(f) if !f.is_zero().get())
}

/// `d |n + b d| u`, the denominator over the divisor's factor `fb`, or nothing
/// when it leaves the four limbs. Zero where the factor is.
#[inline]
const fn denominator(step: Step, fb: Wide) -> Maybe<Wide> {
    let magnitude = if fb.is_negative().get() { fb.negated() } else { fb };
    times(
        times(
            Maybe::Is(Wide::of(step.phase.denominator)),
            Maybe::Is(magnitude),
        ),
        step.powers.up,
    )
}

/// `d (n + a d) v - n (n + b d) u` over the two factors, or nothing when it or
/// either of its terms leaves the four limbs.
#[inline]
const fn numerator(step: Step, fa: Wide, fb: Wide) -> Maybe<Wide> {
    let first = if fa.is_zero().get() {
        Maybe::Is(fa)
    } else {
        times(
            times(Maybe::Is(Wide::of(step.phase.denominator)), Maybe::Is(fa)),
            step.powers.down,
        )
    };
    // `|n (n + b d) u|` is at most `2^63` times the denominator.
    let second = times(
        times(Maybe::Is(Wide::of(step.phase.numerator)), Maybe::Is(fb)),
        step.powers.up,
    );
    match (first, second) {
        (Maybe::Is(x), Maybe::Is(y)) => x.plus(y.negated()),
        _ => Maybe::Isnt,
    }
}

/// The quotient of the slots `a` and `b` placed by the step.
///
/// The numerator `d (n + a d) v - n (n + b d) u` in four limbs, negated with the
/// denominator when `n + b d` is below zero, then its floor over the
/// denominator. A denominator or a numerator that leaves the limbs is a step not
/// formed, and a floor that leaves the index is past it, each on the sign of the
/// quotient of the two factors; for a member divisor the obligation has ruled all
/// three out.
///
/// `b` is never a slot whose factor is zero: the shipped callers pass a
/// `Divisor`'s slot or a member `extreme_divisors` has already left the pole out
/// of, and the test-only `unforced_position` is called with slots its sweeps
/// keep off the pole, so the denominator below is positive and `over` divides by
/// it.
#[inline]
const fn placed(step: Step, a: i128, b: i128) -> (Placed, Wide) {
    let one = Wide::of_unsigned(1);
    // Both factors are under `2^191`, so neither is lost; the arm is the
    // answer the step gives if one were.
    let (Maybe::Is(fa), Maybe::Is(fb)) = (factor(step, a), factor(step, b)) else {
        return (Placed::Unformed(false), one);
    };
    let flip = fb.is_negative().get();
    let negative = fa.is_negative().get() != flip;
    let Maybe::Is(divisor) = denominator(step, fb) else {
        return (Placed::Unformed(negative), one);
    };
    match numerator(step, fa, fb) {
        Maybe::Is(x) => (over(if flip { x.negated() } else { x }, divisor), divisor),
        Maybe::Isnt => (Placed::Unformed(negative), one),
    }
}

/// Whether the step is formed at every pair of members: the denominator at
/// every member divisor, and the numerator at every pair.
///
/// `|n + b d|` is affine in `b`, so the range's two ends decide the
/// denominator, and each term of the numerator is affine in one slot, so the
/// numerator's magnitude over the box of slots is greatest at a corner, the
/// pole's corners included, since the numerator divides by nothing. Nothing
/// where the phase does not denote.
#[inline]
const fn quotient_is_formed<F: Format>() -> Bool {
    let Maybe::Is(step) = Divisible::<F>::STEP else {
        return Bool::FALSE;
    };
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    let ends = [min, max];
    let mut i = 0;
    while i < 2 {
        let Maybe::Is(fb) = factor(step, ends[i]) else {
            return Bool::FALSE;
        };
        if matches!(denominator(step, fb), Maybe::Isnt) {
            return Bool::FALSE;
        }
        let mut j = 0;
        while j < 2 {
            let Maybe::Is(fa) = factor(step, ends[j]) else {
                return Bool::FALSE;
            };
            if matches!(numerator(step, fa, fb), Maybe::Isnt) {
                return Bool::FALSE;
            }
            j += 1;
        }
        i += 1;
    }
    Bool::TRUE
}

/// The member divisors the quotient is extreme at: the range's two ends and the
/// members nearest the pole on each side, each skipped when it is not in range
/// or is the zero member.
#[inline]
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
#[inline]
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
            && (!is_held(placed(step, min, b).0).get() || !is_held(placed(step, max, b).0).get())
        {
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
/// position past the index, or a numerator or denominator past the four limbs,
/// saturates at the end on the quotient's sign.
///
/// Forces the division obligation, so a format the quotient does not admit stops
/// the build here, at check time when the call is bound in a `const` item.
#[must_use]
#[inline]
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
#[inline]
pub const fn div<S: DeclaredSignature>(a: Slot, b: Divisor<S::Format>, dither: Dither) -> Slot {
    adapt::<S>(quotient_position::<S::Format>(a, b), dither)
}

/// The obligation's four verdicts, in order: the phase denotes, the quantum is
/// constant, the step is formed, the quotient is carried.
///
/// The one place they are stated, read by the obligation and by `is_divisible`.
/// The last two read the step and answer false where the phase does not denote.
#[inline]
const fn conditions<F: Format>() -> [Bool; 4] {
    [
        F::PHASE.denotes(),
        is_constant_family::<F::Quantum>(),
        quotient_is_formed::<F>(),
        quotient_is_carried::<F>(),
    ]
}

/// Whether division admits the format, asked without forcing the obligation.
///
/// The obligation's own verdicts folded, so a test can hold a format the
/// quotient does not admit and report on it rather than stop the build.
#[must_use]
#[inline]
pub const fn is_divisible<F: Format>() -> Bool {
    all(conditions::<F>())
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
