//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where a value lands on an IEEE 754 format, and the standard's own rule for
//! where it should, both written from outside the crate.
//!
//! `adapt` takes a position in slots and carries no magnitude, so the magnitude
//! is the caller's: the finest one at which the rounded position lands in the
//! slot range, which `panic_on_overflow` answers. That is the standard's own
//! rounding onto a floating format, the nearest member at the precision the
//! binade allows, with a carry out of a binade answered by the next magnitude.
//! It is a composition a consumer writes, the way the Round alias is, and it is
//! written once over `Landing`, so a declared signature and the ties-away alias
//! land through the same search.
//!
//! The rule is the standard's digit rule in integers, over any radix, reading no
//! arvo code: a value `c * radix^q` keeps at most `p` digits and no exponent
//! below the smallest, the dropped part decides the last kept digit by the
//! direction, and a carry to `radix^p` drops one more. Each format's numbers are
//! written in as the standard's, a `Standard` in the suite that runs it.
//!
//! It is compiled into three test targets and each uses part of it, so the
//! allow below is about the compilation unit rather than about anything here
//! being unused.

#![allow(dead_code)]

use arvo_format::adapt::DeclaredSignature;
use arvo_format::apply::{Dither, Exact, Remainder, adapt, panic_on_overflow};
use arvo_format::format::{Format, step_exponent};
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

/// `radix^e` in the host's unsigned 128-bit integer, or nothing past it.
fn unsigned_power(radix: u32, e: u32) -> Option<u128> {
    if e > 128 {
        return None;
    }
    let mut p: u128 = 1;
    for _ in 0 .. e {
        p = p.checked_mul(radix as u128)?;
    }
    Some(p)
}

// --- the landing -----------------------------------------------------------------------

/// What lands a position on one magnitude's grid: whether it leaves the range,
/// and the slot it reads off as.
pub trait Landing {
    type Format: Format;

    fn overflows(exact: Exact) -> bool;

    fn read_off(exact: Exact) -> i128;
}

/// A declared signature lands by its own adaptation.
impl<S: DeclaredSignature> Landing for S {
    type Format = S::Format;

    fn overflows(exact: Exact) -> bool {
        panic_on_overflow::<S>(exact, Dither::UNUSED).get()
    }

    fn read_off(exact: Exact) -> i128 {
        adapt::<S>(exact, Dither::UNUSED).index()
    }
}

/// The position of `num / den * radix^k` on the grid of step `radix^s`, or
/// nothing where the scaled numerator leaves the index.
///
/// A scaled denominator past the index is a value less than one step from zero,
/// so its floor is 0 or -1. Where the denominator still fits the host's unsigned
/// 128-bit integer the remainder is exact. Past that the value is under half a
/// step, since `2 |num|` is below `2^128`, and the remainder is a stand-in in the
/// same open interval, `(0, 1/2)` or `(1/2, 1)`, which keeps every comparison an
/// undithered adaptation makes of it: against zero and against a half.
fn position(r: u32, num: i128, den: i128, k: i32, s: i32) -> Option<Exact> {
    let shift = k - s;
    if shift >= 0 {
        let n = num.checked_mul(power(r, shift as u32)?)?;
        return Some(Exact::at(
            Slot::at(n.div_euclid(den)),
            Remainder::of(n.rem_euclid(den) as u128, den as u128),
        ));
    }
    let scaled = unsigned_power(r, (-shift) as u32).and_then(|p| (den as u128).checked_mul(p));
    if let Some(d) = scaled.filter(|&d| d <= i128::MAX as u128) {
        let d = d as i128;
        return Some(Exact::at(
            Slot::at(num.div_euclid(d)),
            Remainder::of(num.rem_euclid(d) as u128, d as u128),
        ));
    }
    let (num_part, den_part) = match scaled {
        Some(d) => {
            (
                if num < 0 { d - num.unsigned_abs() } else { num as u128 },
                d,
            )
        },
        None => {
            assert!(
                num != i128::MIN,
                "the inputs keep the numerator above the index's floor"
            );
            let below_half = if num == 0 { 0 } else { 1 };
            let part = if num < 0 { u128::MAX - 1 } else { below_half };
            (part, u128::MAX)
        },
    };
    Some(Exact::at(
        Slot::at(if num < 0 { -1 } else { 0 }),
        Remainder::of(num_part, den_part),
    ))
}

/// `num / den * radix^k`, with `den` positive, landed on the format.
///
/// Returns nothing only where the value cannot be positioned at any magnitude,
/// its scaled numerator past the index at the coarsest one, which the parity
/// inputs never reach and the arms assert. A value below the finest step lands
/// there, however far below it is.
pub fn land<L: Landing>(num: i128, den: i128, k: i32) -> Option<Landed> {
    let r = arvo_format::format::radix::<L::Format>().base();
    let count = <<L::Format as Format>::Quantum as Quantum>::MAGNITUDES.count();
    let mut top = None;
    for m in 0 .. count {
        let s = step_exponent::<L::Format>(Magnitude::at(m)).power();
        let Some(exact) = position(r, num, den, k, s) else {
            continue;
        };
        if !L::overflows(exact) {
            return Some(Landed {
                value:  Value {
                    significand: L::read_off(exact),
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
            significand: L::read_off(exact),
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

/// Whether the landed value is the expected one: the same number where the
/// standard's answer is finite, and no magnitude holding it where it is not.
///
/// Where the standard's answer is finite past the range, the largest member on
/// the side the direction rounds away from, the completion's saturation is what
/// meets it, so a value no magnitude held still agrees when it is that member.
pub fn agrees(r: u32, landed: Option<Landed>, want: Option<Value>) -> bool {
    match (landed, want) {
        (Some(l), Some(w)) => same(r, l.value, w),
        (Some(l), None) => !l.finite,
        (None, _) => false,
    }
}

// --- the standard's rule ---------------------------------------------------------------

/// The five rounding-direction attributes, in the standard's own terms.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    TiesToEven,
    TiesToAway,
    TowardPositive,
    TowardNegative,
    TowardZero,
}

/// A format as the standard's table gives it: the radix, the precision, and the
/// exponent range of the significand's last digit, `emin - (p - 1)` to
/// `emax - (p - 1)`.
#[derive(Clone, Copy, Debug)]
pub struct Standard {
    pub radix: u32,
    pub p:     u32,
    pub qmin:  i32,
    pub qmax:  i32,
}

/// The number of digits of `|c|` in the radix, zero for zero.
fn digits(r: u32, c: i128) -> u32 {
    let mut n = c.unsigned_abs();
    let mut d = 0;
    while n > 0 {
        n /= r as u128;
        d += 1;
    }
    d
}

/// `c` divided by `radix^drop` under a direction, the standard's way: the kept
/// digits and the dropped part against half of what was dropped.
fn round_off(dir: Direction, r: u32, c: i128, drop: u32) -> i128 {
    // The kept digits are read back as a signed significand, and `-2^127` has no
    // positive counterpart, so the rule refuses it rather than wrap.
    assert!(
        c != i128::MIN,
        "the digit rule refuses a significand of -2^127"
    );
    // In the unsigned 128-bit integer, so a unit past the index whose half `|c|`
    // still reaches is read rather than taken for more than twice `|c|`.
    let Some(unit) = unsigned_power(r, drop) else {
        // Everything is dropped and what is dropped is less than half a unit,
        // since `2 |c|` is below `2^128` and the unit is past that.
        return match dir {
            Direction::TowardPositive if c > 0 => 1,
            Direction::TowardNegative if c < 0 => -1,
            _ => 0,
        };
    };
    let magnitude = c.unsigned_abs();
    let kept = (magnitude / unit) as i128 * c.signum();
    let rest = magnitude % unit;
    if rest == 0 {
        return kept;
    }
    let toward_zero = kept;
    let away = kept + c.signum();
    let half = rest.cmp(&(unit - rest));
    match dir {
        Direction::TowardZero => toward_zero,
        Direction::TowardPositive => {
            if c > 0 {
                away
            } else {
                toward_zero
            }
        },
        Direction::TowardNegative => {
            if c < 0 {
                away
            } else {
                toward_zero
            }
        },
        Direction::TiesToAway => {
            if half.is_lt() {
                toward_zero
            } else {
                away
            }
        },
        Direction::TiesToEven => {
            match half {
                core::cmp::Ordering::Less => toward_zero,
                core::cmp::Ordering::Greater => away,
                core::cmp::Ordering::Equal => {
                    if kept % 2 == 0 {
                        toward_zero
                    } else {
                        away
                    }
                },
            }
        },
    }
}

/// The standard's answer for `c * radix^q` under a direction, or nothing where
/// it is an infinity.
pub fn rule(dir: Direction, f: Standard, c: i128, q: i32) -> Option<Value> {
    let too_many = digits(f.radix, c).saturating_sub(f.p) as i32;
    let drop = too_many.max(f.qmin - q).max(0) as u32;
    let mut kept = round_off(dir, f.radix, c, drop);
    let mut at = q + drop as i32;
    if digits(f.radix, kept) > f.p {
        kept /= f.radix as i128;
        at += 1;
    }
    if at > f.qmax {
        let up = (at - f.qmax) as u32;
        match power(f.radix, up).and_then(|s| kept.checked_mul(s)) {
            Some(scaled) if digits(f.radix, scaled) <= f.p => {
                kept = scaled;
                at = f.qmax;
            },
            _ => {
                // Past the largest finite member: an infinity, except toward
                // zero and on a directed mode's own side, where the standard
                // answers the largest finite member.
                let largest = power(f.radix, f.p).unwrap() - 1;
                let finite = |s: i128| {
                    Some(Value {
                        significand: s * largest,
                        exponent:    f.qmax,
                    })
                };
                return match dir {
                    Direction::TowardZero => finite(c.signum()),
                    Direction::TowardPositive if c < 0 => finite(-1),
                    Direction::TowardNegative if c > 0 => finite(1),
                    _ => None,
                };
            },
        }
    }
    Some(Value {
        significand: kept,
        exponent:    at,
    })
}

/// Inputs as `c * radix^q`: structured ties and carries, then a sweep from below
/// the smallest subnormal to past the largest finite member, with a tie drawn
/// beside every value.
pub fn inputs(f: Standard, seed: u64) -> Vec<(i128, i32)> {
    let r = f.radix as i128;
    let half = r / 2;
    let top = power(f.radix, f.p).unwrap();
    let mut v = vec![
        (top * r + half, 0),
        (top * r + r + half, 0),
        (top * r * r - half, 0),
        (-(top * r + half), 0),
        (-(top * r + r + half), 0),
        (half, f.qmin - 1),
        (r + half, f.qmin - 1),
        (-(2 * r + half), f.qmin - 1),
        (top * r - 1, f.qmax),
        (top - 1, f.qmax),
        (top * r - half, f.qmax - 1),
    ];
    let mut d = Draws::seeded(seed);
    for _ in 0 .. 3000 {
        let c = d.within(-999_999_999_999_999_999, 999_999_999_999_999_999) as i128;
        let q = d.within((f.qmin - 20) as i64, (f.qmax + 2) as i64) as i32;
        v.push((c, q));
        // A tie: a full significand, one more digit, and half the radix.
        let s = d.within(power(f.radix, f.p - 1).unwrap() as i64, (top - 1) as i64) as i128;
        let sign = if d.next() & 1 == 0 { 1 } else { -1 };
        v.push((
            sign * (s * r + half),
            d.within((f.qmin - 1) as i64, (f.qmax - 1) as i64) as i32,
        ));
    }
    v.extend(underflow(f, seed));
    v
}

/// The lowest exponent whose unit below the smallest subnormal the unsigned
/// 128-bit integer holds: the first unit past the slot index at radix two, one
/// above it at radix ten. Below it the search hands its stand-in.
pub fn held_below(f: Standard) -> i32 {
    let past = (1 ..)
        .find(|&e| unsigned_power(f.radix, e).is_none())
        .unwrap();
    f.qmin - past as i32 + 1
}

/// Inputs below the smallest subnormal, as far as the slot index's integer and
/// past: the scaled denominator past the index, past the unsigned 128-bit
/// integer, and the tie at the first unit past the index where it is written.
///
/// The standard's answer there is the smallest subnormal or zero by the
/// direction, so a landing that answers nothing, or that misreads the dropped
/// part against a half, misses here.
pub fn underflow(f: Standard, seed: u64) -> Vec<(i128, i32)> {
    let top = power(f.radix, f.p).unwrap();
    // The first exponent whose power of the radix is past the index.
    let past = (1 ..).find(|&e| power(f.radix, e).is_none()).unwrap();
    let below = f.qmin - past as i32;
    let mut v = vec![
        (1, f.qmin - 300),
        (-1, f.qmin - 300),
        (top - 1, f.qmin - 300),
        (-(top - 1), f.qmin - 300),
        (i128::MAX, below),
        (-i128::MAX, below),
        (i128::MAX, below - 1),
        (-i128::MAX, below - 1),
        (1, below),
        (-1, below - 1),
    ];
    // Half the first unit past the index, and its neighbours, where the index
    // holds it: two to the 126th in binary, nothing in decimal.
    if let Some(half) = power(f.radix, past - 1).and_then(|p| p.checked_mul((f.radix / 2) as i128))
    {
        for c in [half - 1, half, half + 1] {
            v.push((c, below));
            v.push((-c, below));
        }
    }
    let mut d = Draws::seeded(seed ^ 0x5EED_0000_0000_0001);
    for _ in 0 .. 500 {
        let c = d.within(-999_999_999_999_999_999, 999_999_999_999_999_999) as i128;
        v.push((
            c,
            d.within(held_below(f) as i64, (f.qmin - 21) as i64) as i32,
        ));
    }
    v
}

/// How many inputs a landing answers differently from the rule.
pub fn misses<L: Landing>(dir: Direction, f: Standard, seed: u64) -> usize {
    misses_over::<L>(dir, f, inputs(f, seed))
}

/// How many of the given inputs a landing answers differently from the rule.
pub fn misses_over<L: Landing>(dir: Direction, f: Standard, inputs: Vec<(i128, i32)>) -> usize {
    inputs
        .into_iter()
        .filter(|&(c, q)| !agrees(f.radix, land::<L>(c, 1, q), rule(dir, f, c, q)))
        .count()
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
