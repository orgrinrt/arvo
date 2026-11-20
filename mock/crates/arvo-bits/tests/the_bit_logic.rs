//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `BitLogic` and the inherent `and`, `or`, `xor`, `not` and `is_empty`, at
//! every width from one to sixty-four, against the host's operation on the
//! carrier: every pair of values up to eight bits, and above that every pair of
//! the values `common::values` takes.
//!
//! The algebra, `not` an involution, commuting, associating, De Morgan, the
//! intersection test, is the host's and follows from these equalities over the
//! same values, so none of it is asserted apart.

mod common;

use arvo_bits::{BitLogic, Bits};
use common::{mask, values};

fn generic<B: BitLogic>(a: B, b: B) -> (B, B, B, B, bool) {
    (a.and(b), a.or(b), a.xor(b), a.not(), a.is_empty().get())
}

/// The laws over one width.
fn logic<const N: u32>() {
    let vs = values(N);
    for &x in &vs {
        let a = Bits::<N>::masked(x);
        assert_eq!(a.not().raw(), !x & mask(N), "not {x:#x} at {N} bits");
        assert_eq!(a.is_empty().get(), x == 0, "{x:#x} empty at {N} bits");
        for &y in &vs {
            let b = Bits::<N>::masked(y);
            let (and, or, xor) = (a.and(b), a.or(b), a.xor(b));
            assert_eq!(and.raw(), x & y, "{x:#x} and {y:#x} at {N} bits");
            assert_eq!(or.raw(), x | y, "{x:#x} or {y:#x} at {N} bits");
            assert_eq!(xor.raw(), x ^ y, "{x:#x} xor {y:#x} at {N} bits");
            assert_eq!(
                generic(a, b),
                (and, or, xor, a.not(), x == 0),
                "generic at {N} bits"
            );
        }
    }
}

macro_rules! widths {
    ($($n:literal)+) => { $( logic::<$n>(); )+ };
}

#[test]
fn the_logic_is_the_hosts_within_every_width() {
    widths!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
    widths!(33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60);
    widths!(61 62 63 64);
}
