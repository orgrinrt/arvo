//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Negation and subtraction, the addition step at other signs.
//!
//! A signed sum of members lands at the signed sum of their slots plus one copy
//! of the phase fewer than the sum of the signs. Addition is two plus signs and
//! one copy; negation is one minus sign and minus two copies, the position
//! `-a - 2 * phase`; subtraction is a plus and a minus and minus one copy, the
//! position `a - b - phase`. So both steps read the phase's parts at another
//! count of copies and build the position the way `sum_position` does, under
//! the same obligation, and nothing else about them is new.
//!
//! Neither carries an associativity verdict. Negation is unary, and subtraction
//! does not keep the law in the ambient rationals, so there is nothing for an
//! absorption argument to preserve.

use core::marker::PhantomData;

use super::{Addable, graded, positioned};
use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, adapt};
use crate::format::Format;
use crate::slots::{Slot, Slots};
use crate::symmetry::Reach;

/// Negation under one declared signature.
///
/// Arity one. Like `Add` it names its signature and carries nothing else, and the
/// arity is read by `negation_reach`, which asserts it.
pub struct Neg<S: DeclaredSignature>(PhantomData<S>);

impl<S: DeclaredSignature> Operation for Neg<S> {
    type Signature = S;

    const ARITY: Arity = Arity::UNARY;
}

/// Subtraction under one declared signature.
///
/// Arity two, the operands in the order `a - b`. The arity is read by
/// `subtraction_reach`, which asserts it.
pub struct Sub<S: DeclaredSignature>(PhantomData<S>);

impl<S: DeclaredSignature> Operation for Sub<S> {
    type Signature = S;

    const ARITY: Arity = Arity::BINARY;
}

/// The exact position of the negation of the member at slot `a`.
///
/// `-a - 2 * phase`, in slot units. For a member the position is exact, because
/// the addition obligation holds every member inside the index with sixty bits
/// to spare and negation moves it by at most `2^64`. For a slot outside the range
/// the one subtraction saturates at the ends of the index's host integer, so the
/// step still answers.
///
/// Forces the addition obligation, the one `is_addable` reports on.
#[must_use]
#[inline]
pub const fn negation_position<F: Format>(a: Slot) -> Exact {
    let () = Addable::<F>::ADMITTED;
    let parts = Addable::<F>::TWICE_NEGATED_PHASE_PARTS;
    positioned(parts.whole.saturating_sub(a.index()), parts)
}

/// The negation of the member at slot `a`, adapted under the signature.
///
/// The exact step, then `adapt` under the same signature, with the dither passed
/// through. Total over every slot, with no check and no panic, as `add` is.
#[must_use]
#[inline]
pub const fn neg<S: DeclaredSignature>(a: Slot, dither: Dither) -> Slot {
    adapt::<S>(negation_position::<S::Format>(a), dither)
}

/// The exact position of the difference of the members at slots `a` and `b`.
///
/// `a - b - phase`, in slot units. The addition obligation alone does not make
/// this exact: a range spanning up to `2^127 - 1` slots with a phase of `-2^63`
/// puts the top difference past the index. The slot range's own obligation
/// closes it, an admitted width of at most 64 bits keeping the span below `2^64`,
/// so this forces both, and for two members of an admitted format the position
/// is exact. For a slot outside the range the three-term sum saturates once at
/// the ends of the index's host integer.
#[must_use]
#[inline]
pub const fn difference_position<F: Format>(a: Slot, b: Slot) -> Exact {
    let () = Addable::<F>::ADMITTED;
    let () = <F::Slots as Slots>::ADMITTED;
    let parts = Addable::<F>::NEGATED_PHASE_PARTS;
    positioned(
        saturating_difference(a.index(), b.index(), parts.whole),
        parts,
    )
}

/// The difference of the members at slots `a` and `b`, adapted under the
/// signature.
///
/// The exact step, then `adapt` under the same signature, with the dither passed
/// through. Total over every pair of slots, with no check and no panic. `adapt`
/// forces the slot range's obligation already, so `sub` refuses exactly the
/// formats `add` refuses.
#[must_use]
#[inline]
pub const fn sub<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot {
    adapt::<S>(difference_position::<S::Format>(a, b), dither)
}

/// The reach negation supplies.
///
/// From `-MAX + k` to `-MIN + k` in slots, `k` the whole part of minus two copies
/// of the phase, with the grid state that remainder decides. No translation.
#[must_use]
#[inline]
pub const fn negation_reach<S: DeclaredSignature>() -> Reach {
    let () = Addable::<S::Format>::ADMITTED;
    // The cache is settled at minus two copies, which is one operand's signed
    // sum minus one, twice over; the assertion keeps the arity and the cache in
    // step at compile time.
    assert!(
        <Neg<S> as Operation>::ARITY.count() == 1,
        "negation's arity moved away from one"
    );
    let parts = Addable::<S::Format>::TWICE_NEGATED_PHASE_PARTS;
    let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
    let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
    graded(
        Reach::of(
            Slot::at(parts.whole.saturating_sub(max)),
            Slot::at(parts.whole.saturating_sub(min)),
        ),
        parts,
    )
}

/// The reach subtraction supplies.
///
/// From `MIN - MAX + k` to `MAX - MIN + k` in slots, `k` the whole part of minus
/// one copy of the phase, with the grid state that remainder decides. No
/// translation.
#[must_use]
#[inline]
pub const fn subtraction_reach<S: DeclaredSignature>() -> Reach {
    let () = Addable::<S::Format>::ADMITTED;
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    assert!(
        <Sub<S> as Operation>::ARITY.count() == 2,
        "subtraction's arity moved away from two"
    );
    let parts = Addable::<S::Format>::NEGATED_PHASE_PARTS;
    let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
    let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
    graded(
        Reach::of(
            Slot::at(saturating_difference(min, max, parts.whole)),
            Slot::at(saturating_difference(max, min, parts.whole)),
        ),
        parts,
    )
}

/// `a - b + c`, saturated once at the ends of the index's host integer.
///
/// Where `a - b` fits, `c` is added with one saturating add, which is exact
/// when the sum fits and saturates at the end it passes otherwise. Where it
/// does not, the difference has passed the end on `a`'s side: a `c` of the
/// other sign is added to `a` first, which cannot overflow, and `b` is then
/// subtracted once, saturating; a `c` of `a`'s own sign, or zero, leaves the
/// answer past the end it already passed.
#[inline]
const fn saturating_difference(a: i128, b: i128, c: i128) -> i128 {
    match a.checked_sub(b) {
        Some(d) => d.saturating_add(c),
        None => {
            if c != 0 && (a < 0) != (c < 0) {
                (a + c).saturating_sub(b)
            } else if a < 0 {
                i128::MIN
            } else {
                i128::MAX
            }
        },
    }
}
