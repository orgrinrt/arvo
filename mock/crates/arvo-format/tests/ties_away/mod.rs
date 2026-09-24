//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ties-away alias, written from the public surface, in the two spellings a
//! consumer has.
//!
//! `ruling::half_up_denotes_ties_toward_positive_infinity` reaches ties away from
//! zero as `toward_zero(x + sign(x) q/2)`, a shift composed with an adaptation,
//! and says it is reached as a result rather than as a name. Both spellings below
//! are that result. Neither is a mode and neither is offered by the crate: they
//! are what a consumer writes, and they live in a module rather than in one of the
//! two test files so that the parity arms and the sweep read the same definition.
//!
//! A module under `tests/` and not a test target of its own, which is what keeps
//! cargo from running this file as a suite with no arms in it. It is compiled
//! into both test targets, and each uses part of it, so the allow below is about
//! the compilation unit rather than about anything here being unused.

#![allow(dead_code)]

pub mod completion;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::Overflow;
use arvo_format::rounding::{Floor, HalfUp, TowardZero};
use arvo_format::slots::Slot;

/// The position `slot + n/d`, as a caller holding the three writes it.
pub fn at(slot: i128, n: i64, d: i64) -> Exact {
    Exact::between(Slot::at(slot), Fraction::of(n, d))
}

/// The alias read off the position: `floor` at a tie below zero, `half_up`
/// everywhere else.
///
/// A tie at or above zero goes up under both rules, and away from zero below it
/// is down, which is `floor`. Everywhere else the position is nearer one
/// neighbour than the other and every nearest rule agrees, so `half_up` answers.
/// It reads `Exact::is_tie` and `Exact::slot`, and the slot's sign is the
/// position's because the remainder is in `[0, 1)`, so this is available to a
/// caller holding a position it did not build.
pub fn select<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> i128 {
    read_off::<F, O>(at(slot, n, d))
}

/// The same spelling over a position the caller already holds.
pub fn read_off<F: Format, O: Overflow>(x: Exact) -> i128 {
    if x.is_tie().get() && x.slot().index() < 0 {
        adapt::<Signature<F, Adapt<Floor, O>>>(x, Dither::UNUSED).index()
    } else {
        adapt::<Signature<F, Adapt<HalfUp, O>>>(x, Dither::UNUSED).index()
    }
}

/// The alias written as the ruling writes it: shift the position half a quantum
/// toward its sign, then round toward zero.
///
/// The half is added over the ratio's own denominator rather than over a doubled
/// one, so nothing is formed that the ratio's integer cannot hold: with an even
/// `d` the half is `d/2`, and where the sum would leave the integer the slot moves
/// by one instead. With an odd `d` no tie is representable, since `2n` is never
/// `d`, so the position is strictly nearer one neighbour and the alias is
/// `half_up`.
///
/// `None` in one region, named in the crate's open items: at the slot
/// `i128::MAX`, with an even `d` and `n + d/2` above the ratio's integer, the
/// shifted position lies past the top of the index and `Exact::between` reaches
/// past the top only through a carry inside the ratio, whose numerator would be
/// larger still. No tie lies there, since `n` is above `d/2`.
pub fn shift<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    if d % 2 == 1 {
        return Some(
            adapt::<Signature<F, Adapt<HalfUp, O>>>(at(slot, n, d), Dither::UNUSED).index(),
        );
    }
    let half = d / 2;
    let shifted = if slot >= 0 {
        match n.checked_add(half) {
            Some(up) => at(slot, up, d),
            None => at(slot.checked_add(1)?, n - half, d),
        }
    } else {
        // `n - half` is in `[-half, half)`, so it never leaves the integer, and
        // the constructor carries the borrow into the slot for it.
        at(slot, n - half, d)
    };
    Some(adapt::<Signature<F, Adapt<TowardZero, O>>>(shifted, Dither::UNUSED).index())
}

/// Whether `shift` can form the shifted position at all, as a predicate over the
/// three coordinates rather than as a list of the cells where it cannot.
pub fn shift_is_formable(slot: i128, n: i64, d: i64) -> bool {
    slot != i128::MAX || d % 2 == 1 || (n as i128) + (d / 2) as i128 <= i64::MAX as i128
}
