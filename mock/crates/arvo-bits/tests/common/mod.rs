//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the bit tier's sweeps share: the mask of a width in the host's own
//! arithmetic, and the values a sweep takes at a width.

#![allow(dead_code, unused_macros)]

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

/// `v` at width `n` with bit `i` set to `to`, built one bit at a time: bit
/// `i` from `to`, every other bit `j` from bit `j` of `v`, so it is not the
/// crate's own mask and or.
pub fn written(v: u64, n: u32, i: u32, to: bool) -> u64 {
    let mut out = 0;
    for j in 0 .. n {
        let set = if j == i { to } else { (v >> j) & 1 == 1 };
        if set {
            out |= 1u64 << j;
        }
    }
    out
}

/// Calls `$cb!(N K NK)` for every width `N` from one to sixty-four and every
/// index or amount `K` below it, with `NK` the amount `(N - K) mod N` a
/// rotation down takes to equal the rotation up by `K`.
///
/// Each index is a const generic, so the calls are expanded rather than looped.
/// The walk adds one index per width: at width `N` the indices from one up are
/// the last width's and `N - 1`, and their `NK` are `N - 1` and the last
/// width's, in the opposite order.
macro_rules! every_index {
    ($cb:ident) => {
        $cb!(1 0 0);
        $crate::common::triangle!(
            $cb; [] [];
            (2 1) (3 2) (4 3) (5 4) (6 5) (7 6) (8 7) (9 8) (10 9) (11 10)
            (12 11) (13 12) (14 13) (15 14) (16 15) (17 16) (18 17) (19 18) (20 19) (21 20)
            (22 21) (23 22) (24 23) (25 24) (26 25) (27 26) (28 27) (29 28) (30 29) (31 30)
            (32 31) (33 32) (34 33) (35 34) (36 35) (37 36) (38 37) (39 38) (40 39) (41 40)
            (42 41) (43 42) (44 43) (45 44) (46 45) (47 46) (48 47) (49 48) (50 49) (51 50)
            (52 51) (53 52) (54 53) (55 54) (56 55) (57 56) (58 57) (59 58) (60 59) (61 60)
            (62 61) (63 62) (64 63)
        );
    };
}

macro_rules! triangle {
    (@emit $cb:ident $n:literal; [$($k:literal)*] [$($nk:literal)*]) => {
        $( $cb!($n $k $nk); )*
    };
    ($cb:ident; [$($k:literal)*] [$($nk:literal)*];) => {};
    ($cb:ident; [$($k:literal)*] [$($nk:literal)*]; ($n:literal $i:literal) $($rest:tt)*) => {
        $cb!($n 0 0);
        $crate::common::triangle!(@emit $cb $n; [$($k)* $i] [$i $($nk)*]);
        $crate::common::triangle!($cb; [$($k)* $i] [$i $($nk)*]; $($rest)*);
    };
}

#[allow(unused_imports)]
pub(crate) use {every_index, triangle};
