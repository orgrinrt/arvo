//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product of two members at a constant quantum.
//!
//! Two members at a constant quantum `q` are `(phase + a) * q` and
//! `(phase + b) * q`, so their product is the member at position
//! `(phase + a)(phase + b) * q - phase`, a function of the declared signature at
//! every phase and radix. With the phase `n / d` in lowest terms, the radix `r`
//! and the exponent `e`, that position is a numerator over
//! `D = d^2 r^max(-e, 0)`: `(n + a d)(n + b d) r^max(e, 0)` less the phase's
//! term `n d r^max(-e, 0)`. The numerator is formed four 64-bit limbs wide and
//! divided once by `D`, the floor the position's and the residue the remainder. That
//! is the step this module states, composed with the declared signature's
//! adaptation the way addition's is.
//!
//! One obligation carries three conditions: a phase that denotes, and two that
//! are not yet admitted rather than refused, a magnitude-indexed quantum and a
//! step whose `D` or corner numerators pass the four limbs. A formed step has a
//! position wherever its floor is, so nothing about the slot index is asked. It
//! is forced by the exact step and by the composition, and `is_multipliable`
//! asks the same without forcing it.

use core::marker::PhantomData;

use notko::Maybe;

use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, adapt};
use crate::format::Format;
use crate::placing::{
    Placed,
    Powers,
    Reduced,
    is_formed,
    over,
    positioned,
    powers,
    reduced_phase,
    saturated,
    times,
};
use crate::quantum::is_constant_family;
use crate::slots::{Slot, Slots};
use crate::wide::Wide;
use crate::width::Bool;

/// Multiplication under one declared signature.
///
/// It names the signature it is a function of and its arity, two, and carries
/// nothing else.
pub struct Mul<S: DeclaredSignature>(PhantomData<S>);

impl<S: DeclaredSignature> Operation for Mul<S> {
    type Signature = S;

    const ARITY: Arity = Arity::BINARY;
}

/// What a format owes the product, checked rather than asked for.
struct Multipliable<F>(PhantomData<F>);

impl<F: Format> Multipliable<F> {
    /// The three conditions, in the order `is_multipliable` asks them. The phase
    /// is asserted first because the others read its parts, and a zero
    /// denominator has none.
    const ADMITTED: () = {
        assert!(
            F::PHASE.denotes().get(),
            "multiplication is refused over a phase whose denominator is zero: such a phase \
             names no member, so there is no product to position"
        );
        // FIXME(arvo_product_over_a_magnitude_indexed_quantum_is_a_placehold): the
        // product is a function of the signature here too. `Exact` and `adapt`
        // carry no magnitude coordinate to land it at, and no canon row says what
        // wrapping means on a union of magnitude ranges.
        assert!(
            is_constant_family::<F::Quantum>().get(),
            "multiplication is not yet admitted over a magnitude-indexed quantum: a slot names a \
             different value at each magnitude, and the applied map carries no magnitude \
             coordinate to land the product at yet"
        );
        // FIXME(arvo_s_exact_position_carries_no_remainder_denominator_past): the
        // position is a function of the signature here too, and only the four
        // limbs its numerator and denominator are formed in are too narrow.
        assert!(
            product_is_formed::<F>().get(),
            "multiplication is not yet admitted where the remainder's denominator d^2 r^max(-e, 0), \
             with d the phase's reduced denominator, r the radix and e the quantum's exponent, \
             or the numerator at a corner of the range, passes 2^256 - 1: an exact step is formed \
             in four 64-bit limbs"
        );
    };
    /// The step's constants, or nothing where `D` or the phase's term leaves the
    /// four limbs, settled once per format.
    const STEP: Maybe<Step> = step::<F>();
}

/// What the step reads of the phase, the radix and the exponent.
#[derive(Clone, Copy)]
struct Step {
    /// `n`, the phase's numerator in lowest terms.
    numerator:   i128,
    /// `d`, the phase's denominator in lowest terms, positive.
    denominator: i128,
    /// `r^max(e, 0)`, or nothing when it leaves the four limbs, in which case any
    /// product but zero does too.
    scale:       Maybe<Wide>,
    /// `n d r^max(-e, 0)`, the phase's term, at most `2^63` times `D`.
    offset:      Wide,
    /// `D = d^2 r^max(-e, 0)`, positive and in the four limbs.
    divisor:     Wide,
}

/// The step's constants for `F`, or nothing where `D` or the phase's term
/// leaves the four limbs.
///
/// A zero phase denominator, and a zero radix at a negative exponent, give no
/// positive `D`, and the radix's own obligation refuses the second.
#[inline]
const fn step<F: Format>() -> Maybe<Step> {
    let Maybe::Is(Reduced {
        numerator: n,
        denominator: d,
    }) = reduced_phase::<F>()
    else {
        return Maybe::Isnt;
    };
    let Powers {
        up,
        down,
    } = powers::<F>();
    // `d r^k`, then `D` as `d` times it. Either leaving the four limbs is a `D`
    // not yet admitted.
    let lowered = match times(Maybe::Is(Wide::of(d)), down) {
        Maybe::Is(v) => v,
        Maybe::Isnt => return Maybe::Isnt,
    };
    let divisor = match Wide::of(d).times(lowered) {
        Maybe::Is(v) if !v.is_zero().get() => v,
        _ => return Maybe::Isnt,
    };
    // `d r^k` is at most `D` and `|n|` at most `2^63`, and their product leaving
    // the limbs is a step not formed.
    let offset = match Wide::of(n).times(lowered) {
        Maybe::Is(v) => v,
        Maybe::Isnt => return Maybe::Isnt,
    };
    Maybe::Is(Step {
        numerator: n,
        denominator: d,
        scale: up,
        offset,
        divisor,
    })
}

/// `n + a d`, one factor of the numerator.
#[inline]
const fn factor(step: Step, a: i128) -> Maybe<Wide> {
    match Wide::of(a).times(Wide::of(step.denominator)) {
        Maybe::Is(ad) => ad.plus(Wide::of(step.numerator)),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// The product of the slots `a` and `b` placed by the step.
///
/// The numerator `(n + a d)(n + b d) r^max(e, 0) - n d r^max(-e, 0)` in four
/// limbs, then its floor over `D`. A numerator that leaves the limbs is a step
/// not formed, and a floor that leaves the index is past it, each on the
/// position's sign; for two members the obligation has ruled both out.
#[inline]
const fn placed(step: Step, a: i128, b: i128) -> Placed {
    // Each factor is under `2^191` in magnitude for any slot, so it is formed;
    // the arms are the answer the step gives if one were not.
    let fa = match factor(step, a) {
        Maybe::Is(f) => f,
        Maybe::Isnt => return Placed::Unformed(a < 0),
    };
    let fb = match factor(step, b) {
        Maybe::Is(f) => f,
        Maybe::Isnt => return Placed::Unformed(b < 0),
    };
    let negative = fa.is_negative().get() != fb.is_negative().get();
    let product = match fa.times(fb) {
        Maybe::Is(p) => p,
        Maybe::Isnt => return Placed::Unformed(negative),
    };
    let scaled = if product.is_zero().get() {
        product
    } else {
        match times(Maybe::Is(product), step.scale) {
            Maybe::Is(p) => p,
            Maybe::Isnt => return Placed::Unformed(negative),
        }
    };
    match scaled.plus(step.offset.negated()) {
        Maybe::Is(numerator) => over(numerator, step.divisor),
        Maybe::Isnt => Placed::Unformed(negative),
    }
}

/// Whether the step is formed at every pair of members: `D` and the phase's
/// term in the four limbs, and the numerator at each of the four corners.
///
/// Every intermediate of the numerator, and the numerator itself, is bilinear in
/// the two slots, so its magnitude over a box of slots is greatest at a corner,
/// and the corners forming means every pair does.
#[inline]
const fn product_is_formed<F: Format>() -> Bool {
    let Maybe::Is(step) = Multipliable::<F>::STEP else {
        return Bool::FALSE;
    };
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    Bool::of(
        is_formed(placed(step, min, min)).get()
            && is_formed(placed(step, min, max)).get()
            && is_formed(placed(step, max, min)).get()
            && is_formed(placed(step, max, max)).get(),
    )
}

/// The exact position of the product of the members at slots `a` and `b`.
///
/// `(phase + a)(phase + b) * q - phase`, in slot units: on the grid, or at the
/// floor slot with the remainder over `d^2 r^max(-e, 0)` in lowest terms. For two
/// members the position is exact, wherever it is. For a slot outside the range, a
/// numerator past the four limbs saturates at the end of the limbs on the
/// position's sign, so the step still answers.
///
/// Forces the multiplication obligation, so a format the product does not admit
/// stops the build here, at check time when the call is bound in a `const` item.
#[must_use]
#[inline]
pub const fn product_position<F: Format>(a: Slot, b: Slot) -> Exact {
    let () = Multipliable::<F>::ADMITTED;
    match Multipliable::<F>::STEP {
        Maybe::Is(step) => positioned(step.divisor, placed(step, a.index(), b.index())),
        // The obligation above has refused this format already.
        Maybe::Isnt => saturated(false),
    }
}

/// The product of the members at slots `a` and `b`, adapted under the signature.
///
/// The exact step, then `adapt` under the same signature, with the dither passed
/// through for the one mode that reads it. Total over every pair of slots, with
/// no check and no panic.
///
/// The position's floor is held in four limbs, so a product past the slot index
/// is still one position, and `adapt` wraps, saturates or clamps it by its own
/// floor, as on the unsigned 64-bit range at exponents from 0 to 128.
#[must_use]
#[inline]
pub const fn mul<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot {
    let () = Multipliable::<S::Format>::ADMITTED;
    let Maybe::Is(step) = Multipliable::<S::Format>::STEP else {
        // The obligation above has refused this format already.
        return adapt::<S>(saturated(false), dither);
    };
    adapt::<S>(
        positioned(step.divisor, placed(step, a.index(), b.index())),
        dither,
    )
}

/// Whether multiplication admits the format, asked without forcing the
/// obligation.
///
/// The same three conditions in the same order, so a test can hold a format the
/// product does not admit and report on it rather than stop the build.
#[must_use]
#[inline]
pub const fn is_multipliable<F: Format>() -> Bool {
    if !F::PHASE.denotes().get() || !is_constant_family::<F::Quantum>().get() {
        return Bool::FALSE;
    }
    product_is_formed::<F>()
}

/// The step at slots `a` and `b` without forcing the obligation, for the edge
/// sweep, which reads the step at formats the obligation does not admit.
#[cfg(test)]
const fn unforced_position<F: Format>(a: Slot, b: Slot) -> Maybe<Exact> {
    match Multipliable::<F>::STEP {
        Maybe::Is(step) => Maybe::Is(positioned(step.divisor, placed(step, a.index(), b.index()))),
        Maybe::Isnt => Maybe::Isnt,
    }
}

#[cfg(test)]
mod tests;
