//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product of two members at a binary quantum.
//!
//! Two members at a constant quantum `q` are `(phase + a) * q` and
//! `(phase + b) * q`, so their product is the member at position
//! `(phase + a)(phase + b) * q - phase`. The sum's quantum dropped out; the
//! product's stays in. At radix two and zero phase the position is
//! `a * b * 2^e`, `e` the quantum's exponent: the slots' product moved `e`
//! binary places, up onto the grid or down with a remainder over `2^-e`. That is
//! the step this module states, composed with the declared signature's
//! adaptation the way addition's is.
//!
//! One obligation carries five refusals: a nonzero phase, a radix other than
//! two, a magnitude-indexed quantum, an exponent below `-62`, and a product the
//! slot index cannot carry. It is forced by the exact step, and `is_multipliable`
//! asks the same questions without forcing it.

use core::marker::PhantomData;

use notko::Maybe;

use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::ambient::Radix;
use crate::apply::{Dither, Exact, Fraction, adapt};
use crate::format::{Format, radix};
use crate::quantum::{Exponent, Quantum, is_constant_family};
use crate::slots::{Slot, Slots};
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

/// The lowest quantum exponent whose remainder an exact position holds.
///
/// The remainder's denominator is `2^-e`, and the fraction carries a positive
/// signed 64-bit denominator, whose largest power of two is `2^62`.
const LOWEST_EXPONENT: Exponent = Exponent::of(-62);

/// The widest move up a product of the index's integer can take and stay
/// nonzero: `1 << 126` is the largest power of two the index holds.
const WIDEST_MOVE: Exponent = Exponent::of(126);

/// What a format owes the product, checked rather than asked for.
struct Multipliable<F>(PhantomData<F>);

impl<F: Format> Multipliable<F> {
    /// The five refusals, in the order `is_multipliable` asks them.
    const ADMITTED: () = {
        assert!(
            F::PHASE.is_zero().get(),
            "multiplication is refused over a phased format: the product of two members sits at \
             (phase + a)(phase + b) * q - phase, which reads the quantum and the phase together, \
             and the step here computes the zero-phase case only"
        );
        assert!(
            is_binary::<F>().get(),
            "multiplication is refused at a radix other than two: moving the product down by the \
             quantum's exponent would be a division by a power of that radix, and its remainder's \
             denominator would not be a power of two"
        );
        assert!(
            is_constant_family::<F::Quantum>().get(),
            "multiplication is refused over a magnitude-indexed quantum: a slot names a different \
             value at each magnitude, and the applied map carries no magnitude coordinate to land \
             the product at"
        );
        assert!(
            exponent_is_held::<F>().get(),
            "multiplication is refused at a quantum exponent below -62: the remainder's denominator \
             is 2^-e, and an exact position carries a positive signed 64-bit denominator, whose \
             largest power of two is 2^62"
        );
        assert!(
            product_is_carried::<F>().get(),
            "multiplication is refused over this format: the exact product of two members leaves \
             the slot coordinate, so the position it names is not one a slot index can hold"
        );
    };
    /// The quantum's exponent, settled once per format.
    const EXPONENT: Exponent = <F::Quantum as Quantum>::BASE;
}

/// Whether the format's ambient counts in twos.
const fn is_binary<F: Format>() -> Bool {
    let r: Radix = radix::<F>();
    Bool::of(r.base() == Radix::BINARY.base())
}

/// Whether the remainder the down move leaves has a denominator a position holds.
const fn exponent_is_held<F: Format>() -> Bool {
    Bool::of(Multipliable::<F>::EXPONENT.power() >= LOWEST_EXPONENT.power())
}

/// `p` moved up `e` places, or nothing when that leaves the index.
const fn moved_up(p: i128, e: i32) -> Maybe<i128> {
    if p == 0 {
        return Maybe::Is(0);
    }
    if e > WIDEST_MOVE.power() {
        return Maybe::Isnt;
    }
    match p.checked_mul(1i128 << e) {
        Some(at) => Maybe::Is(at),
        None => Maybe::Isnt,
    }
}

/// The slot product of the corner `(a, b)` moved up by a nonnegative exponent,
/// or nothing when either step leaves the index.
const fn corner(a: i128, b: i128, e: i32) -> Maybe<i128> {
    match a.checked_mul(b) {
        Some(p) => {
            if e > 0 {
                moved_up(p, e)
            } else {
                Maybe::Is(p)
            }
        },
        None => Maybe::Isnt,
    }
}

/// Whether the corner `(a, b)` moved by `e` is one the index holds.
const fn corner_is_held(a: i128, b: i128, e: i32) -> Bool {
    Bool::of(matches!(corner(a, b, e), Maybe::Is(_)))
}

/// Whether every product of two members, moved by the exponent, is one a slot
/// index holds.
///
/// The extremes of `a * b` over a box of slots sit at its corners, so the four
/// corners decide it. A down move only shrinks the slot, so only an up move is
/// checked after the product.
const fn product_is_carried<F: Format>() -> Bool {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    let e = Multipliable::<F>::EXPONENT.power();
    Bool::of(
        corner_is_held(min, min, e).get()
            && corner_is_held(min, max, e).get()
            && corner_is_held(max, min, e).get()
            && corner_is_held(max, max, e).get(),
    )
}

/// The end of the index on the side of a product whose sign is `negative`.
const fn saturated(negative: bool) -> Exact {
    Exact::on_grid(Slot::at(if negative { i128::MIN } else { i128::MAX }))
}

/// The exact position of the product of the members at slots `a` and `b`.
///
/// `a * b * 2^e`, in slot units. Up, on the grid, for `e` at or above zero;
/// down for `e` below it, the slot the arithmetic shift, which is the floor, and
/// the remainder the low `-e` bits over `2^-e` in lowest terms. For two members
/// the position is exact. For a slot outside the range, a product or an up move
/// that leaves the index saturates at the end on the product's sign, so the step
/// still answers.
///
/// Forces the multiplication obligation, so a format the product refuses stops
/// the build here, at check time when the call is bound in a `const` item.
#[must_use]
pub const fn product_position<F: Format>(a: Slot, b: Slot) -> Exact {
    let () = Multipliable::<F>::ADMITTED;
    let e = Multipliable::<F>::EXPONENT.power();
    let (a, b) = (a.index(), b.index());
    let Some(p) = a.checked_mul(b) else {
        return saturated((a < 0) != (b < 0));
    };
    if e >= 0 {
        return match moved_up(p, e) {
            Maybe::Is(at) => Exact::on_grid(Slot::at(at)),
            Maybe::Isnt => saturated(p < 0),
        };
    }
    let k = -e;
    let whole = p >> k;
    let rem = p & ((1i128 << k) - 1);
    if rem == 0 {
        return Exact::on_grid(Slot::at(whole));
    }
    // Lowest terms: the remainder's trailing zeros come off both halves.
    let shift = rem.trailing_zeros() as i32;
    Exact::between(
        Slot::at(whole),
        Fraction::of((rem >> shift) as i64, (1i64) << (k - shift)),
    )
}

/// The product of the members at slots `a` and `b`, adapted under the signature.
///
/// The exact step, then `adapt` under the same signature, with the dither passed
/// through for the one mode that reads it. Total over every pair of slots, with
/// no check and no panic.
#[must_use]
pub const fn mul<S: DeclaredSignature>(a: Slot, b: Slot, dither: Dither) -> Slot {
    adapt::<S>(product_position::<S::Format>(a, b), dither)
}

/// Whether multiplication admits the format, asked without forcing the
/// obligation.
///
/// The same five conditions in the same order, so a test can hold a format the
/// product refuses and report on it rather than stop the build.
#[must_use]
pub const fn is_multipliable<F: Format>() -> Bool {
    if !F::PHASE.is_zero().get() || !is_binary::<F>().get() {
        return Bool::FALSE;
    }
    if !is_constant_family::<F::Quantum>().get() || !exponent_is_held::<F>().get() {
        return Bool::FALSE;
    }
    product_is_carried::<F>()
}

#[cfg(test)]
mod tests;
