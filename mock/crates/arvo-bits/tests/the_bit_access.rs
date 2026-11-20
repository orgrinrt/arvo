//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `BitAccess` and the inherent `bit` and `with_bit`, at every index of every
//! width in `common::WIDTHS`, against the host's own bit of the carrier.
//!
//! The calls below take each index as a const generic, so they are written out
//! one per index rather than looped.

mod common;

use arvo_bits::{BitAccess, Bits};
use arvo_format::Bool;
use common::values;

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
                (v & !(1u64 << I)) | ((to as u64) << I),
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

#[test]
fn every_bit_of_every_value_reads_and_writes_as_the_hosts() {
    access::<1, 0>();
    access::<2, 0>();
    access::<2, 1>();
    access::<3, 0>();
    access::<3, 1>();
    access::<3, 2>();
    access::<4, 0>();
    access::<4, 1>();
    access::<4, 2>();
    access::<4, 3>();
    access::<5, 0>();
    access::<5, 1>();
    access::<5, 2>();
    access::<5, 3>();
    access::<5, 4>();
    access::<6, 0>();
    access::<6, 1>();
    access::<6, 2>();
    access::<6, 3>();
    access::<6, 4>();
    access::<6, 5>();
    access::<7, 0>();
    access::<7, 1>();
    access::<7, 2>();
    access::<7, 3>();
    access::<7, 4>();
    access::<7, 5>();
    access::<7, 6>();
    access::<8, 0>();
    access::<8, 1>();
    access::<8, 2>();
    access::<8, 3>();
    access::<8, 4>();
    access::<8, 5>();
    access::<8, 6>();
    access::<8, 7>();
    access::<16, 0>();
    access::<16, 1>();
    access::<16, 2>();
    access::<16, 3>();
    access::<16, 4>();
    access::<16, 5>();
    access::<16, 6>();
    access::<16, 7>();
    access::<16, 8>();
    access::<16, 9>();
    access::<16, 10>();
    access::<16, 11>();
    access::<16, 12>();
    access::<16, 13>();
    access::<16, 14>();
    access::<16, 15>();
    access::<28, 0>();
    access::<28, 1>();
    access::<28, 2>();
    access::<28, 3>();
    access::<28, 4>();
    access::<28, 5>();
    access::<28, 6>();
    access::<28, 7>();
    access::<28, 8>();
    access::<28, 9>();
    access::<28, 10>();
    access::<28, 11>();
    access::<28, 12>();
    access::<28, 13>();
    access::<28, 14>();
    access::<28, 15>();
    access::<28, 16>();
    access::<28, 17>();
    access::<28, 18>();
    access::<28, 19>();
    access::<28, 20>();
    access::<28, 21>();
    access::<28, 22>();
    access::<28, 23>();
    access::<28, 24>();
    access::<28, 25>();
    access::<28, 26>();
    access::<28, 27>();
    access::<32, 0>();
    access::<32, 1>();
    access::<32, 2>();
    access::<32, 3>();
    access::<32, 4>();
    access::<32, 5>();
    access::<32, 6>();
    access::<32, 7>();
    access::<32, 8>();
    access::<32, 9>();
    access::<32, 10>();
    access::<32, 11>();
    access::<32, 12>();
    access::<32, 13>();
    access::<32, 14>();
    access::<32, 15>();
    access::<32, 16>();
    access::<32, 17>();
    access::<32, 18>();
    access::<32, 19>();
    access::<32, 20>();
    access::<32, 21>();
    access::<32, 22>();
    access::<32, 23>();
    access::<32, 24>();
    access::<32, 25>();
    access::<32, 26>();
    access::<32, 27>();
    access::<32, 28>();
    access::<32, 29>();
    access::<32, 30>();
    access::<32, 31>();
    access::<63, 0>();
    access::<63, 1>();
    access::<63, 2>();
    access::<63, 3>();
    access::<63, 4>();
    access::<63, 5>();
    access::<63, 6>();
    access::<63, 7>();
    access::<63, 8>();
    access::<63, 9>();
    access::<63, 10>();
    access::<63, 11>();
    access::<63, 12>();
    access::<63, 13>();
    access::<63, 14>();
    access::<63, 15>();
    access::<63, 16>();
    access::<63, 17>();
    access::<63, 18>();
    access::<63, 19>();
    access::<63, 20>();
    access::<63, 21>();
    access::<63, 22>();
    access::<63, 23>();
    access::<63, 24>();
    access::<63, 25>();
    access::<63, 26>();
    access::<63, 27>();
    access::<63, 28>();
    access::<63, 29>();
    access::<63, 30>();
    access::<63, 31>();
    access::<63, 32>();
    access::<63, 33>();
    access::<63, 34>();
    access::<63, 35>();
    access::<63, 36>();
    access::<63, 37>();
    access::<63, 38>();
    access::<63, 39>();
    access::<63, 40>();
    access::<63, 41>();
    access::<63, 42>();
    access::<63, 43>();
    access::<63, 44>();
    access::<63, 45>();
    access::<63, 46>();
    access::<63, 47>();
    access::<63, 48>();
    access::<63, 49>();
    access::<63, 50>();
    access::<63, 51>();
    access::<63, 52>();
    access::<63, 53>();
    access::<63, 54>();
    access::<63, 55>();
    access::<63, 56>();
    access::<63, 57>();
    access::<63, 58>();
    access::<63, 59>();
    access::<63, 60>();
    access::<63, 61>();
    access::<63, 62>();
    access::<64, 0>();
    access::<64, 1>();
    access::<64, 2>();
    access::<64, 3>();
    access::<64, 4>();
    access::<64, 5>();
    access::<64, 6>();
    access::<64, 7>();
    access::<64, 8>();
    access::<64, 9>();
    access::<64, 10>();
    access::<64, 11>();
    access::<64, 12>();
    access::<64, 13>();
    access::<64, 14>();
    access::<64, 15>();
    access::<64, 16>();
    access::<64, 17>();
    access::<64, 18>();
    access::<64, 19>();
    access::<64, 20>();
    access::<64, 21>();
    access::<64, 22>();
    access::<64, 23>();
    access::<64, 24>();
    access::<64, 25>();
    access::<64, 26>();
    access::<64, 27>();
    access::<64, 28>();
    access::<64, 29>();
    access::<64, 30>();
    access::<64, 31>();
    access::<64, 32>();
    access::<64, 33>();
    access::<64, 34>();
    access::<64, 35>();
    access::<64, 36>();
    access::<64, 37>();
    access::<64, 38>();
    access::<64, 39>();
    access::<64, 40>();
    access::<64, 41>();
    access::<64, 42>();
    access::<64, 43>();
    access::<64, 44>();
    access::<64, 45>();
    access::<64, 46>();
    access::<64, 47>();
    access::<64, 48>();
    access::<64, 49>();
    access::<64, 50>();
    access::<64, 51>();
    access::<64, 52>();
    access::<64, 53>();
    access::<64, 54>();
    access::<64, 55>();
    access::<64, 56>();
    access::<64, 57>();
    access::<64, 58>();
    access::<64, 59>();
    access::<64, 60>();
    access::<64, 61>();
    access::<64, 62>();
    access::<64, 63>();
}
