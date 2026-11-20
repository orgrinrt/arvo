//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `BitAccess` and the inherent `bit` and `with_bit`, at every index of every
//! width from one to sixty-four, against the host's own bit of the carrier and
//! a value written one bit at a time.
//!
//! Each index is a const generic, so `common::every_index` expands one call per
//! width and index rather than looping.

mod common;

use arvo_bits::{BitAccess, Bits};
use arvo_format::Bool;
use common::{every_index, values, written};

fn read<B: BitAccess, const I: u32>(b: B) -> Bool {
    b.bit::<I>()
}

fn write<B: BitAccess, const I: u32>(b: B, to: Bool) -> B {
    b.with_bit::<I>(to)
}

/// Bit `I` of every value at width `N`, read and written both ways, through
/// the inherent method and through a function generic over the trait.
fn access<const N: u32, const I: u32>() {
    for v in values(N) {
        let b = Bits::<N>::masked(v);
        let set = (v >> I) & 1 == 1;
        assert_eq!(b.bit::<I>().get(), set, "bit {I} of {v:#x} at {N} bits");
        assert_eq!(
            read::<_, I>(b).get(),
            set,
            "bit {I} of {v:#x} at {N} bits, generic"
        );
        for to in [false, true] {
            let w = b.with_bit::<I>(Bool::of(to));
            assert_eq!(
                w.raw(),
                written(v, N, I, to),
                "bit {I} of {v:#x} written {to} at {N} bits"
            );
            assert_eq!(w.bit::<I>().get(), to);
            assert_eq!(write::<_, I>(b, Bool::of(to)), w);
        }
        assert_eq!(
            b.with_bit::<I>(b.bit::<I>()),
            b,
            "bit {I} of {v:#x} rewritten at {N} bits"
        );
    }
}

macro_rules! call {
    ($n:literal $i:literal $ni:literal) => {
        access::<$n, $i>();
    };
}

#[test]
fn every_bit_of_every_value_reads_and_writes_as_the_hosts() {
    every_index!(call);
}
