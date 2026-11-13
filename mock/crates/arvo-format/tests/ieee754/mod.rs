//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where a value lands on an IEEE 754 format, written from the public surface.
//!
//! `adapt` takes a position in slots and carries no magnitude, so the magnitude
//! is the caller's: the finest one at which the rounded position lands in the
//! slot range, which `panic_on_overflow` answers. That is the standard's own
//! rounding onto a floating format, the nearest member at the precision the
//! binade allows, with a carry out of a binade answered by the next magnitude.
//! It is a composition a consumer writes, the way the Round alias is, and it
//! lives in a module so both parity files read one definition.
//!
//! `allow(dead_code)` because each file that declares this module uses part of
//! it, which is a fact about the compilation unit.

#![allow(dead_code)]

use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Exact, Remainder, adapt, panic_on_overflow};
use arvo_format::format::{Format, radix, step_exponent};
use arvo_format::quantum::{Magnitude, Quantum};
use arvo_format::slots::Slot;

/// A value as a significand and an exponent of the format's radix.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub significand: i128,
    pub exponent:    i32,
}

/// What the composition returns: the value a member names, and whether any
/// magnitude held it. `finite` false is the overflow region, where the value is
/// the completion's answer at the top magnitude.
#[derive(Clone, Copy, Debug)]
pub struct Landed {
    pub value:  Value,
    pub finite: bool,
}

/// `radix^e`, or nothing past the slot index.
pub fn power(radix: u32, e: u32) -> Option<i128> {
    // Every radix this reads is at least two, so past 127 the power has left
    // the index, and saying so here keeps a search over thousands of
    // magnitudes from multiplying its way there each time.
    if e > 127 {
        return None;
    }
    let mut p: i128 = 1;
    for _ in 0 .. e {
        p = p.checked_mul(radix as i128)?;
    }
    Some(p)
}

/// The position of `num / den * radix^k` on the grid of step `radix^s`, or
/// nothing where the scaled numerator or denominator leaves the index.
fn position(r: u32, num: i128, den: i128, k: i32, s: i32) -> Option<Exact> {
    let shift = k - s;
    let (n, d) = if shift >= 0 {
        (num.checked_mul(power(r, shift as u32)?)?, den)
    } else {
        (num, den.checked_mul(power(r, (-shift) as u32)?)?)
    };
    Some(Exact::at(
        Slot::at(n.div_euclid(d)),
        Remainder::of(n.rem_euclid(d) as u128, d as u128),
    ))
}

/// `num / den * radix^k`, with `den` positive, landed on the signature's format.
///
/// Returns nothing only where the value cannot be positioned at any magnitude in
/// the slot index's integer, which the parity inputs never reach and the arms
/// assert.
pub fn land<S: DeclaredSignature>(num: i128, den: i128, k: i32) -> Option<Landed> {
    let r = radix::<S::Format>().base();
    let count = <<S::Format as Format>::Quantum as Quantum>::MAGNITUDES.count();
    let mut top = None;
    for m in 0 .. count {
        let s = step_exponent::<S::Format>(Magnitude::at(m)).power();
        let Some(exact) = position(r, num, den, k, s) else {
            continue;
        };
        if !panic_on_overflow::<S>(exact, Dither::UNUSED).get() {
            let slot = adapt::<S>(exact, Dither::UNUSED);
            return Some(Landed {
                value:  Value {
                    significand: slot.index(),
                    exponent:    s,
                },
                finite: true,
            });
        }
        top = Some((exact, s));
    }
    let (exact, s) = top?;
    Some(Landed {
        value:  Value {
            significand: adapt::<S>(exact, Dither::UNUSED).index(),
            exponent:    s,
        },
        finite: false,
    })
}

/// Whether two values in one radix are the same number.
pub fn same(r: u32, a: Value, b: Value) -> bool {
    if a.significand == 0 || b.significand == 0 {
        return a.significand == b.significand;
    }
    let (hi, lo) = if a.exponent >= b.exponent { (a, b) } else { (b, a) };
    match power(r, (hi.exponent - lo.exponent) as u32).and_then(|p| hi.significand.checked_mul(p)) {
        Some(scaled) => scaled == lo.significand,
        None => false,
    }
}

/// A deterministic stream of 64-bit draws, so a sweep is the same every run.
pub struct Draws(u64);

impl Draws {
    pub fn seeded(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next(&mut self) -> u64 {
        // SplitMix64.
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A draw in `[lo, hi]`.
    pub fn within(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % ((hi - lo) as u64 + 1)) as i64
    }
}
