//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the bit tier's sweeps share: the mask of a width in the host's own
//! arithmetic, and the values a sweep takes at a width.

#![allow(dead_code)]

/// The low `n` bits set, computed in 128 bits so the full width needs no case of
/// its own here, unlike in the crate.
pub const fn mask(n: u32) -> u64 {
    ((1u128 << n) - 1) as u64
}

/// The values a sweep takes at width `n`: every value up to eight bits, and
/// above that the ends of the width, every single bit, the two alternating
/// patterns and seeded values, each within the width.
pub fn values(n: u32) -> Vec<u64> {
    let m = mask(n);
    if n <= 8 {
        return (0 ..= m).collect();
    }
    let mut out = vec![
        0,
        1,
        2,
        3,
        m,
        m - 1,
        m >> 1,
        0x5555_5555_5555_5555 & m,
        0xAAAA_AAAA_AAAA_AAAA & m,
    ];
    out.extend((0 .. n).map(|j| 1u64 << j));
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15 ^ n as u64;
    for _ in 0 .. 64 {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        out.push((z ^ (z >> 31)) & m);
    }
    out
}

/// A small set of values for a law over three operands at width `n`: every
/// value up to four bits, and above that the first sixteen of `values`.
pub fn few(n: u32) -> Vec<u64> {
    let all = values(n);
    if n <= 4 { all } else { all.into_iter().take(16).collect() }
}

/// Bit `j` of `v` moved to place `(j + k) mod n`, one bit at a time: a
/// rotation up by `k` within `n` bits, written without a shift of the whole
/// value so it is not the crate's own formula.
pub fn rotated_up(v: u64, n: u32, k: u32) -> u64 {
    let mut out = 0;
    for j in 0 .. n {
        if (v >> j) & 1 == 1 {
            out |= 1u64 << ((j + k) % n);
        }
    }
    out
}

/// The widths the sweeps over an index or an amount take every index or
/// amount at.
pub const WIDTHS: [u32; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 16, 28, 32, 63, 64];
