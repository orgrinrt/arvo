//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Addition, the one operation this crate admits.
//!
//! The admission rule asks one thing of an operation, that it be a function of
//! the declared signature, and addition is the operation whose exact step shows
//! most plainly what that means. Two members of a format are `(phase + a) * q`
//! and `(phase + b) * q` at one quantum `q`, so their exact sum is the member at
//! position `a + b + phase`, counted in slots. The quantum and the radix do not
//! appear, so the step reads the phase and the slot range and nothing else, which
//! is why it can be stated here, in the crate that cannot reach a carrier.
//!
//! The step is composed with the declared signature's adaptation, the same map
//! every exact result goes through, and beside it sits the verdict on where the
//! adapted sum is associative. That verdict is the relocation law of `symmetry`
//! read at the positions and translations addition produces, which is how the
//! absorption law the canon states for a reduction onto the representable set
//! becomes a function of the signature rather than a sweep.
//!
//! Three things are refused, and one obligation carries all three: a quantum
//! that moves with the magnitude, a phase remainder the position cannot hold, and
//! a sum the slot coordinate cannot carry. The obligation is forced at the three
//! verbs that compute addition's positions, and `is_addable` asks the same
//! question without forcing it.

use core::marker::PhantomData;

use crate::adapt::{Adaptation, Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, Fraction, adapt, round_slot};
use crate::format::Format;
use crate::quantum::is_constant_family;
use crate::rounding::{Mode, Rounding};
use crate::slots::{Slot, Slots};
use crate::symmetry::{
    Reach,
    completion_is_translation_homomorphic,
    rounding_is_translation_equivariant,
};
use crate::width::Bool;

/// Addition under one declared signature.
///
/// It names the signature it is a function of and its arity, two, and carries
/// nothing else, because the admission rule asks for nothing else. The arity is
/// read: the reach addition supplies is computed from it, so an arity that
/// disagreed with the step would move the reach away from the positions the step
/// produces.
pub struct Add<S: DeclaredSignature>(PhantomData<S>);

impl<S: DeclaredSignature> Operation for Add<S> {
    type Signature = S;

    const ARITY: Arity = Arity::BINARY;
}

/// What a format owes addition, checked rather than asked for.
///
/// Private, because nothing outside this file forces it: the three verbs that
/// compute addition's positions do, and `is_addable` is the verdict beside it.
struct Addable<F>(PhantomData<F>);

impl<F: Format> Addable<F> {
    /// The three refusals. The remainder is asserted first because the other two
    /// read the phase's parts, and a zero denominator has none.
    const ADMITTED: () = {
        assert!(
            remainder_is_held::<F>().get(),
            "addition is refused over a phase whose remainder no position can hold: the phase \
             denominator is zero, or its fractional part reduces to a denominator of 2^63, which \
             the remainder of an exact position does not carry"
        );
        // FIXME: the magnitude-indexed family is refused rather than added, and two
        // things unblock it. In the design, `Exact` and `adapt` carry no magnitude
        // coordinate, so a sum whose operands sit at two magnitudes has no position
        // to be handed as; that is a change to the applied map, not to addition. In
        // the canon, no row says what wrapping means on a union of magnitude ranges,
        // and `where_wrapping_lives` classifies wrapping rather than defining it on
        // such a set.
        assert!(
            is_constant_family::<F::Quantum>().get(),
            "addition is refused over a magnitude-indexed quantum: a slot names a different value \
             at each magnitude, and the applied map carries no magnitude coordinate to land the \
             sum at"
        );
        assert!(
            sum_is_carried::<F>().get(),
            "addition is refused over this format: the exact sum of two members leaves the slot \
             coordinate, so the position it names is not one a slot index can hold"
        );
    };
}

/// `copies` times the phase, as a whole number of slots and a remainder, one
/// domain wider than a slot.
///
/// Answers the whole part, then the remainder's numerator and denominator reduced
/// to lowest terms. The sign moves onto the numerator before the euclidean
/// division, so the remainder is in `[0, 1)` whichever sign the pair was
/// declared with. A zero denominator has no parts and answers zeros, which
/// nothing reads: the obligation and `is_addable` refuse such a phase first.
const fn phase_parts<F: Format>(copies: i128) -> (i128, i128, i128) {
    let num = (F::PHASE.numerator() as i128) * copies;
    let den = F::PHASE.denominator() as i128;
    if den == 0 {
        return (0, 0, 0);
    }
    let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
    let whole = num.div_euclid(den);
    let rem = num.rem_euclid(den);
    let common = greatest_common_divisor(rem, den);
    (whole, rem / common, den / common)
}

/// Euclid's algorithm over a non-negative and a positive operand.
const fn greatest_common_divisor(a: i128, b: i128) -> i128 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Whether the phase's remainder is one an exact position can carry.
///
/// Its reduced denominator has to be a positive value the coordinate holds. It is
/// not when the denominator is zero, and it is not when a denominator of the
/// least value meets an odd numerator, whose remainder reduces to a denominator
/// of `2^63`.
const fn remainder_is_held<F: Format>() -> Bool {
    if !F::PHASE.denotes().get() {
        return Bool::FALSE;
    }
    let (_, _, den) = phase_parts::<F>(1);
    Bool::of(den <= i64::MAX as i128)
}

/// Whether every position the step produces for two members is one a slot
/// index holds.
///
/// Those positions run from `2 * MIN + k` to `2 * MAX + k` plus the remainder,
/// where `k` is the phase's whole part, so the top one rounds as far as one slot
/// above that when the remainder is not zero.
const fn sum_is_carried<F: Format>() -> Bool {
    let (whole, rem, _) = phase_parts::<F>(1);
    let ceiling = if rem == 0 { whole } else { whole + 1 };
    let min = <F::Slots as Slots>::MIN.index() as i128;
    let max = <F::Slots as Slots>::MAX.index() as i128;
    Bool::of(2 * min + whole >= i64::MIN as i128 && 2 * max + ceiling <= i64::MAX as i128)
}

/// A position computed one domain wider, narrowed into the slot coordinate.
///
/// Saturating at the ends of the coordinate, the same act `Reach` performs on a
/// bound that leaves it. For two members the obligation keeps every position
/// inside, so the narrowing is exact there; it moves a value only when an operand
/// is a slot outside the range.
const fn saturated(wide: i128) -> Slot {
    if wide > i64::MAX as i128 {
        Slot::at(i64::MAX)
    } else if wide < i64::MIN as i128 {
        Slot::at(i64::MIN)
    } else {
        Slot::at(wide as i64)
    }
}

/// The exact position of the sum of the members at slots `a` and `b`.
///
/// `a + b + phase`, in slot units: the phase's whole part added to the slot, its
/// fractional part the remainder the rounding region reads. Computed one domain
/// wider than a slot. For two members that position is exact; for a slot outside
/// the range there is no member sum, and the wide sum is saturated into the
/// coordinate so the step still answers.
///
/// Forces the addition obligation, so a format addition refuses stops the build
/// here, at check time when the call is bound in a `const` item.
#[must_use]
pub const fn sum_position<F: Format>(a: Slot, b: Slot) -> Exact {
    let () = Addable::<F>::ADMITTED;
    let (whole, rem, den) = phase_parts::<F>(1);
    let wide = (a.index() as i128) + (b.index() as i128) + whole;
    // The obligation holds the reduced denominator inside the coordinate and the
    // remainder below it, so both narrow exactly.
    Exact::between(saturated(wide), Fraction::of(rem as i64, den as i64))
}

/// The sum of the members at slots `a` and `b`, adapted under the signature.
///
/// The composition the factoring names: the exact step, then `adapt` under the
/// same signature, with the dither passed through for the one mode that reads it.
/// Total over every pair of slots. It carries no check and does not panic; a
/// caller wanting the debug panic asks `panic_on_overflow` about
/// `sum_position(a, b)` with the same dither.
#[must_use]
pub const fn add<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot {
    adapt::<S>(sum_position::<S::Format>(a, b), dither)
}

/// Whether addition admits the format, asked without forcing the obligation.
///
/// The same three conditions, in the same order, so a test can hold a format
/// addition refuses and report on it rather than stop the build.
#[must_use]
pub const fn is_addable<F: Format>() -> Bool {
    if !remainder_is_held::<F>().get() {
        return Bool::FALSE;
    }
    is_constant_family::<F::Quantum>().and(sum_is_carried::<F>())
}

/// The reach addition supplies.
///
/// The positions `ARITY` members sum to: from `ARITY * MIN` to `ARITY * MAX` in
/// slots, offset by `ARITY - 1` copies of the phase, whose whole part moves the
/// bounds and whose remainder decides the grid state. On the grid when that
/// remainder is zero, at a tie when it is one half, and off the grid otherwise.
///
/// It carries no translation. At a fractional phase the second step's
/// translation is a stored operand plus the phase, which is not a whole number of
/// slots and is not something `Reach` can hold, so the verdict below states the
/// translations itself.
#[must_use]
pub const fn addition_reach<S: DeclaredSignature>() -> Reach {
    let () = Addable::<S::Format>::ADMITTED;
    let operands = <Add<S> as Operation>::ARITY.count() as i128;
    let (whole, rem, den) = phase_parts::<S::Format>(operands - 1);
    let min = <<S::Format as Format>::Slots as Slots>::MIN.index() as i128;
    let max = <<S::Format as Format>::Slots as Slots>::MAX.index() as i128;
    let reach = Reach::of(
        saturated(operands * min + whole),
        saturated(operands * max + whole),
    );
    if rem == 0 {
        reach.on_grid()
    } else if 2 * rem == den {
        reach
    } else {
        reach.without_ties()
    }
}

/// The completion half of the relocation law for addition at a whole phase of
/// `k` slots.
///
/// The positions two members sum to, and the translations the second step
/// applies, which are a member plus `k`. All on the grid, because at a whole
/// phase nothing rounds.
const fn completion_at<S: DeclaredSignature>(k: i128) -> Bool {
    let min = <<S::Format as Format>::Slots as Slots>::MIN.index() as i128;
    let max = <<S::Format as Format>::Slots as Slots>::MAX.index() as i128;
    let reach = Reach::of(saturated(2 * min + k), saturated(2 * max + k))
        .translated_by(saturated(min + k), saturated(max + k))
        .on_grid();
    completion_is_translation_homomorphic::<S>(reach)
}

/// Whether the adapted sum is associative under the signature.
///
/// Decided by absorption: reducing a reachable sum before adding a stored
/// operand does not change the adapted result. For addition that is the
/// relocation law read at the positions addition produces and the translations
/// its second step applies, quantified over stored operands rather than over the
/// ambient domain. Three cases.
///
/// At a whole phase every position is on the grid and the verdict is the
/// completion half of the law at the phase's whole part.
///
/// At a fractional phase, where the rounding half of the law holds over the
/// reach, rounding adds one fixed offset of zero or one slot at every position,
/// so addition is the whole-phase addition at the whole part plus that offset,
/// and the verdict is the completion question there. The offset is read by
/// rounding one position of the reach. The stochastic mode's offset is the
/// dither's, so for it the completion question has to hold at both, which is
/// the verdict for every dither at once.
///
/// Where the rounding half does not hold, the offset varies across the reach and
/// the verdict refuses. That is a cost rather than a defect: some of those
/// formats are associative anyway.
#[must_use]
pub const fn addition_is_associative<S: DeclaredSignature>() -> Bool {
    let () = Addable::<S::Format>::ADMITTED;
    let reach = addition_reach::<S>();
    let (whole, rem, den) = phase_parts::<S::Format>(1);
    if !reach.reaches_off_the_grid().get() {
        return completion_at::<S>(whole);
    }
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    if !rounding_is_translation_equivariant(mode, reach).get() {
        return Bool::FALSE;
    }
    if matches!(mode, Mode::Stochastic) {
        return completion_at::<S>(whole).and(completion_at::<S>(whole + 1));
    }
    let low = reach.positions_low();
    let rounded = round_slot(
        mode,
        Exact::between(low, Fraction::of(rem as i64, den as i64)),
        Dither::UNUSED,
    );
    completion_at::<S>(whole + (rounded - low.index() as i128))
}

#[cfg(test)]
mod tests;
