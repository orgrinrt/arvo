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
//! `n + b d` is below zero. The divisor is a `Divisor`, whose three constructors
//! keep the zero out before the program runs, so the step is never asked to
//! divide by it.
//!
//! One obligation carries three conditions: a phase that denotes, and two that
//! are not yet admitted rather than refused, a magnitude-indexed quantum and a
//! step whose denominator or corner numerators pass the four limbs. A position
//! past the slot index is no condition: the step holds its floor in the four
//! limbs and `adapt` wraps, saturates or clamps it by that floor. The obligation
//! is forced by the exact step, and `is_divisible` asks the same without forcing
//! it.

use core::cmp::Ordering;
use core::marker::PhantomData;

use notko::Maybe;

use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, adapt};
use crate::format::{Format, cancelling_slot};
use crate::member::Member;
use crate::placing::{
    Placed,
    Powers,
    Reduced,
    all,
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
/// time, by `of`, which is admitted only where no slot of the format names the
/// zero, or by `member`, which is admitted only where no member of its range
/// names the zero. None reads the slot's value to decide anything, so the property is
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
/// handed in whether or not the range holds it, and `member` is the door for such
/// a range.
struct NoSlotNamesTheZero<F>(PhantomData<F>);

impl<F: Format> NoSlotNamesTheZero<F> {
    // FIXME(may_arvo_divide_by_a_signed_divisor_read_at_run_time_of_eith): a divisor
    // read at run time that may be either sign, over a whole-phase range holding
    // the zero, has no door here or through `member`, since a zero-free range lies
    // on one side of its zero slot. Whether signed division is the known use the
    // runtime membership door waits on is the open question.
    const ADMITTED: () = assert!(
        no_slot_names_the_zero::<F>().get(),
        "Divisor::of is refused where a slot names the zero, which is wherever the phase's \
         denominator in lowest terms is one: a slot read at run time could be that slot, and \
         telling it apart would be a check on the value. Name the divisor with Divisor::constant, \
         take it with Divisor::member from a range on this grid that leaves the zero out, or \
         declare a phase whose denominator is past one"
    );
}

/// Whether no slot of `F` names the zero: its phase denotes, and its reduced
/// denominator is past one.
#[inline]
const fn no_slot_names_the_zero<F: Format>() -> Bool {
    Bool::of(matches!(reduced_phase::<F>(), Maybe::Is(phase) if phase.denominator > 1))
}

/// What a divisor taken from a member of `G` owes: `G` sits on `F`'s grid inside
/// `F`'s range, and no member of `G` names the zero.
///
/// The grid is one constant quantum and one reduced phase, so slot `b` of `G`
/// denotes the value `(phase + b) q` slot `b` of `F` does, whichever the radixes.
/// Inside the range, the divisor is a member of `F` too, so the quotient is exact.
struct MemberDivisor<F, G>(PhantomData<(F, G)>);

impl<F: Format, G: Format> MemberDivisor<F, G> {
    const ADMITTED: () = {
        let [quantum, phase, inside, zero_free] = member_conditions::<F, G>();
        assert!(
            quantum.get(),
            "Divisor::member is refused from a format whose quantum is not constant or is another \
             value: slot b names (phase + b) q, so another q is another value than the same slot of \
             the divisor format. The radixes may differ where r^e is one value on both sides, as at \
             exponent zero. A pair whose powers leave the four limbs is not formed, and is refused \
             as well"
        );
        assert!(
            phase.get(),
            "Divisor::member is refused from a format of another reduced phase, or one whose phase \
             does not denote: its slot names a value off the divisor format's grid"
        );
        assert!(
            inside.get(),
            "Divisor::member is refused from a range wider than the divisor format's: a divisor past \
             that range takes the quotient past the exact step, which saturates there"
        );
        assert!(
            zero_free.get(),
            "Divisor::member is refused from a range that holds its zero slot: a member read at run \
             time could be that slot. Declare a range that leaves the slot -n out, n / 1 being the \
             reduced phase, or name the divisor with Divisor::constant"
        );
    };
}

/// The member door's four verdicts, in order: one quantum, the reduced phase,
/// `G`'s range inside `F`'s, and no member of `G` naming the zero.
///
/// The one place they are stated, read by the obligation and by the door's
/// suite, which reads each without forcing the const that refuses it.
#[inline]
const fn member_conditions<F: Format, G: Format>() -> [Bool; 4] {
    [
        one_quantum::<F, G>(),
        same_phase::<F, G>(),
        range_inside::<F, G>(),
        no_member_names_the_zero::<G>(),
    ]
}

/// Whether both quanta are constant and one value.
///
/// `r^e` is `U / V` with `U = r^max(e, 0)` and `V = r^max(-e, 0)`, so the two are
/// one value where `U_F V_G = U_G V_F`, whichever the radixes. A side whose powers
/// or product leave the four limbs answers false, as a pair not formed.
#[inline]
const fn one_quantum<F: Format, G: Format>() -> Bool {
    if !(is_constant_family::<F::Quantum>().get() && is_constant_family::<G::Quantum>().get()) {
        return Bool::FALSE;
    }
    let (f, g) = (powers::<F>(), powers::<G>());
    Bool::of(matches!(
        (times(f.up, g.down), times(g.up, f.down)),
        (Maybe::Is(left), Maybe::Is(right)) if matches!(left.ordering(right), Ordering::Equal)
    ))
}

/// Whether both phases denote and are one ratio in lowest terms.
#[inline]
const fn same_phase<F: Format, G: Format>() -> Bool {
    Bool::of(matches!(
        (reduced_phase::<F>(), reduced_phase::<G>()),
        (Maybe::Is(f), Maybe::Is(g))
            if f.numerator == g.numerator && f.denominator == g.denominator
    ))
}

/// Whether `G`'s slot range lies inside `F`'s.
#[inline]
const fn range_inside<F: Format, G: Format>() -> Bool {
    let (f_min, f_max) = (
        <F::Slots as Slots>::MIN.index(),
        <F::Slots as Slots>::MAX.index(),
    );
    let (g_min, g_max) = (
        <G::Slots as Slots>::MIN.index(),
        <G::Slots as Slots>::MAX.index(),
    );
    Bool::of(g_min >= f_min && g_max <= f_max)
}

/// Whether no member of `G` names the zero: its phase denotes, and either its
/// reduced denominator is past one or, at a whole phase `n / 1`, the slot `-n` is
/// outside its range.
#[inline]
const fn no_member_names_the_zero<G: Format>() -> Bool {
    Bool::of(match reduced_phase::<G>() {
        Maybe::Is(phase) => {
            phase.denominator > 1 || !slot_in_range::<G::Slots>(Slot::at(-phase.numerator)).get()
        },
        Maybe::Isnt => false,
    })
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
    /// A slot outside the range is taken as every operation here takes one. The
    /// step positions it, and a divisor past the range can take the quotient past
    /// the exact step, which saturates there. None of the slots is the zero, so the
    /// step never meets one.
    #[must_use]
    #[inline]
    pub const fn of(slot: Slot) -> Self {
        let () = NoSlotNamesTheZero::<F>::ADMITTED;
        Self {
            slot,
            format: PhantomData,
        }
    }

    /// The divisor at a member of `G`.
    ///
    /// Admitted only where `G` sits on `F`'s grid inside `F`'s range and no member
    /// of `G` names the zero.
    ///
    /// With `G` as `F` it is a whole-phase format whose range leaves the zero out,
    /// dividing by its own slots read at run time. Nothing is read of the member's
    /// value: the obligation is over the two formats alone.
    #[must_use]
    #[inline]
    pub const fn member<G: Format>(member: Member<G>) -> Self {
        let () = MemberDivisor::<F, G>::ADMITTED;
        Self {
            slot:   member.slot(),
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
    /// The three conditions, each asserted from `conditions`, which `is_divisible`
    /// folds. The phase is first because the others read its parts.
    const ADMITTED: () = {
        let [denotes, constant, fits] = conditions::<F>();
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
/// formed, on the sign of the quotient of the two factors; for a member divisor
/// the obligation has ruled both out, and the floor is held whole in the limbs.
///
/// `b` is never a slot whose factor is zero: the shipped callers pass a
/// `Divisor`'s slot, and the test-only `unforced_position` is called with slots its sweeps
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

/// The exact position of the quotient of the member at slot `a` by `b`.
///
/// `(phase + a) / ((phase + b) q) - phase`, in slot units: on the grid, or at the
/// floor slot with the remainder over `d |n + b d| r^max(e, 0)` in lowest terms.
/// For two members the position is exact, past the slot index too. A numerator or
/// denominator past the four limbs, which only a slot outside the range reaches,
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

/// The obligation's three verdicts, in order: the phase denotes, the quantum is
/// constant, the step is formed.
///
/// The one place they are stated, read by the obligation and by `is_divisible`.
/// The last reads the step and answers false where the phase does not denote.
#[inline]
const fn conditions<F: Format>() -> [Bool; 3] {
    [
        F::PHASE.denotes(),
        is_constant_family::<F::Quantum>(),
        quotient_is_formed::<F>(),
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
