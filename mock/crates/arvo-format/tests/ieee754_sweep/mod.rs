//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The sweep over a format's whole range, from below the smallest subnormal to
//! past the largest finite member, which the binary and decimal suites count a
//! landing's misses over.
//!
//! A module under `tests/` beside `ieee754`, declared by the two suites that run
//! the sweep, so the suite that sweeps only below the smallest subnormal does
//! not compile it.

use super::ieee754::{Direction, Draws, Landing, Standard, misses_over, power, underflow};

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

/// How many inputs a landing answers differently from the rule.
pub fn misses<L: Landing>(dir: Direction, f: Standard, seed: u64) -> usize {
    misses_over::<L>(dir, f, inputs(f, seed))
}
