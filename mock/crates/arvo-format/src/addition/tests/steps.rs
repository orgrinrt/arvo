//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The three signed steps, as one shape the sweeps run over.
//!
//! Addition, negation and subtraction are one step at three sign patterns, so a
//! law every one of them owes is written once, over `Step`, and run at each. An
//! impl names the signs and the crate's own functions for the step, and nothing
//! of the arithmetic: the oracles compute from the signs alone.

use crate::adapt::DeclaredSignature;
use crate::addition::{
    add,
    addition_reach,
    difference_position,
    neg,
    negation_position,
    negation_reach,
    sub,
    subtraction_reach,
    sum_position,
};
use crate::apply::{Dither, Exact};
use crate::format::Format;
use crate::slots::{Slot, Slots};
use crate::symmetry::Reach;
use crate::width::Bool;

/// One signed step, read through the crate's functions for it.
///
/// A unary step takes its operand as `a` and ignores `b`.
pub(super) trait Step {
    /// One flag per operand, whether that operand enters negated.
    const NEGATED: &'static [Bool];

    /// The exact position.
    fn position<F: Format>(a: Slot, b: Slot) -> Exact;

    /// The adapted answer.
    fn adapted<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot;

    /// The reach the step supplies.
    fn reach<S: DeclaredSignature>() -> Reach;
}

/// Addition, the signs `(+1, +1)`.
#[derive(Default)]
pub(super) struct Sum;

impl Step for Sum {
    const NEGATED: &'static [Bool] = &[Bool::FALSE, Bool::FALSE];

    fn position<F: Format>(a: Slot, b: Slot) -> Exact {
        sum_position::<F>(a, b)
    }

    fn adapted<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot {
        add::<S>(a, b, dither)
    }

    fn reach<S: DeclaredSignature>() -> Reach {
        addition_reach::<S>()
    }
}

/// Negation, the sign `(-1)`.
#[derive(Default)]
pub(super) struct Negation;

impl Step for Negation {
    const NEGATED: &'static [Bool] = &[Bool::TRUE];

    fn position<F: Format>(a: Slot, _: Slot) -> Exact {
        negation_position::<F>(a)
    }

    fn adapted<S: DeclaredSignature>(a: Slot, _: Slot, dither: Dither) -> Slot {
        neg::<S>(a, dither)
    }

    fn reach<S: DeclaredSignature>() -> Reach {
        negation_reach::<S>()
    }
}

/// Subtraction, the signs `(+1, -1)`.
#[derive(Default)]
pub(super) struct Difference;

impl Step for Difference {
    const NEGATED: &'static [Bool] = &[Bool::FALSE, Bool::TRUE];

    fn position<F: Format>(a: Slot, b: Slot) -> Exact {
        difference_position::<F>(a, b)
    }

    fn adapted<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot {
        sub::<S>(a, b, dither)
    }

    fn reach<S: DeclaredSignature>() -> Reach {
        subtraction_reach::<S>()
    }
}

/// The sign an operand enters with: minus one when negated, one otherwise.
pub(super) fn sign(negated: Bool) -> i128 {
    if negated.get() { -1 } else { 1 }
}

/// How many copies of the phase the step's position carries: the sum of the
/// signs, less one.
pub(super) fn copies<T: Step>() -> i128 {
    T::NEGATED.iter().map(|&n| sign(n)).sum::<i128>() - 1
}

/// The signed sum of the operands' slots, or of any per-operand quantity.
pub(super) fn signed<T: Step, V: Copy>(operands: (V, V), of: impl Fn(V) -> i128) -> i128 {
    let (a, b) = operands;
    T::NEGATED
        .iter()
        .zip([a, b])
        .map(|(&n, v)| sign(n) * of(v))
        .sum()
}

/// Every operand tuple over the members of `F`: each pair for a binary step,
/// each member once for a unary one, with `b` repeating `a`.
pub(super) fn operands<T: Step, F: Format>() -> impl Iterator<Item = (Slot, Slot)> {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    let unary = T::NEGATED.len() == 1;
    (min ..= max).flat_map(move |a| {
        let bs = if unary { a ..= a } else { min ..= max };
        bs.map(move |b| (Slot::at(a), Slot::at(b)))
    })
}

/// How many operand tuples `operands` yields over `F`.
pub(super) fn operand_count<T: Step, F: Format>() -> u64 {
    let members = (<F::Slots as Slots>::MAX.index() - <F::Slots as Slots>::MIN.index() + 1) as u64;
    members.pow(T::NEGATED.len() as u32)
}
