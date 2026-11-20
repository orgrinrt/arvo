//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `BitSequence` and the inherent shifts and rotations, at every amount below
//! every width in `common::WIDTHS`, against the host's shift and a rotation
//! moved one bit at a time.
//!
//! Each amount is a const generic, so the calls below are written out one per
//! amount. The third parameter is `N - K`, or zero at `K` zero, the amount a
//! rotation down has to take to equal the rotation up by `K`.

mod common;

use arvo_bits::{BitSequence, Bits};
use common::{mask, rotated_up, values};

fn up<B: BitSequence, const K: u32>(b: B) -> B {
    b.shift_up::<K>()
}

fn down<B: BitSequence, const K: u32>(b: B) -> B {
    b.shift_down::<K>()
}

fn left<B: BitSequence, const K: u32>(b: B) -> B {
    b.rotate_up::<K>()
}

fn right<B: BitSequence, const K: u32>(b: B) -> B {
    b.rotate_down::<K>()
}

/// Every value at width `N` shifted and rotated by `K`, through the inherent
/// methods and through functions generic over the trait.
fn sequence<const N: u32, const K: u32, const NK: u32>() {
    for v in values(N) {
        let b = Bits::<N>::masked(v);

        let shifted_up = b.shift_up::<K>();
        assert_eq!(
            shifted_up.raw(),
            (v << K) & mask(N),
            "{v:#x} up {K} at {N} bits"
        );
        assert_eq!(up::<_, K>(b), shifted_up);

        let shifted_down = b.shift_down::<K>();
        assert_eq!(shifted_down.raw(), v >> K, "{v:#x} down {K} at {N} bits");
        assert_eq!(down::<_, K>(b), shifted_down);

        let rotated = b.rotate_up::<K>();
        assert_eq!(
            rotated.raw(),
            rotated_up(v, N, K),
            "{v:#x} rotated up {K} at {N} bits"
        );
        assert_eq!(left::<_, K>(b), rotated);
        assert_eq!(rotated.raw().count_ones(), v.count_ones());

        let back = b.rotate_down::<K>();
        assert_eq!(
            back.raw(),
            rotated_up(v, N, NK),
            "{v:#x} rotated down {K} at {N} bits"
        );
        assert_eq!(right::<_, K>(b), back);

        assert_eq!(
            rotated.rotate_down::<K>(),
            b,
            "{v:#x} up then down {K} at {N} bits"
        );
        assert_eq!(
            back.rotate_up::<K>(),
            b,
            "{v:#x} down then up {K} at {N} bits"
        );
        assert_eq!(
            b.rotate_down::<NK>(),
            rotated,
            "{v:#x} up {K} against down {NK} at {N} bits"
        );
    }
}

#[test]
fn every_amount_below_every_width_shifts_and_rotates_as_the_host() {
    sequence::<1, 0, 0>();
    sequence::<2, 0, 0>();
    sequence::<2, 1, 1>();
    sequence::<3, 0, 0>();
    sequence::<3, 1, 2>();
    sequence::<3, 2, 1>();
    sequence::<4, 0, 0>();
    sequence::<4, 1, 3>();
    sequence::<4, 2, 2>();
    sequence::<4, 3, 1>();
    sequence::<5, 0, 0>();
    sequence::<5, 1, 4>();
    sequence::<5, 2, 3>();
    sequence::<5, 3, 2>();
    sequence::<5, 4, 1>();
    sequence::<6, 0, 0>();
    sequence::<6, 1, 5>();
    sequence::<6, 2, 4>();
    sequence::<6, 3, 3>();
    sequence::<6, 4, 2>();
    sequence::<6, 5, 1>();
    sequence::<7, 0, 0>();
    sequence::<7, 1, 6>();
    sequence::<7, 2, 5>();
    sequence::<7, 3, 4>();
    sequence::<7, 4, 3>();
    sequence::<7, 5, 2>();
    sequence::<7, 6, 1>();
    sequence::<8, 0, 0>();
    sequence::<8, 1, 7>();
    sequence::<8, 2, 6>();
    sequence::<8, 3, 5>();
    sequence::<8, 4, 4>();
    sequence::<8, 5, 3>();
    sequence::<8, 6, 2>();
    sequence::<8, 7, 1>();
    sequence::<16, 0, 0>();
    sequence::<16, 1, 15>();
    sequence::<16, 2, 14>();
    sequence::<16, 3, 13>();
    sequence::<16, 4, 12>();
    sequence::<16, 5, 11>();
    sequence::<16, 6, 10>();
    sequence::<16, 7, 9>();
    sequence::<16, 8, 8>();
    sequence::<16, 9, 7>();
    sequence::<16, 10, 6>();
    sequence::<16, 11, 5>();
    sequence::<16, 12, 4>();
    sequence::<16, 13, 3>();
    sequence::<16, 14, 2>();
    sequence::<16, 15, 1>();
    sequence::<28, 0, 0>();
    sequence::<28, 1, 27>();
    sequence::<28, 2, 26>();
    sequence::<28, 3, 25>();
    sequence::<28, 4, 24>();
    sequence::<28, 5, 23>();
    sequence::<28, 6, 22>();
    sequence::<28, 7, 21>();
    sequence::<28, 8, 20>();
    sequence::<28, 9, 19>();
    sequence::<28, 10, 18>();
    sequence::<28, 11, 17>();
    sequence::<28, 12, 16>();
    sequence::<28, 13, 15>();
    sequence::<28, 14, 14>();
    sequence::<28, 15, 13>();
    sequence::<28, 16, 12>();
    sequence::<28, 17, 11>();
    sequence::<28, 18, 10>();
    sequence::<28, 19, 9>();
    sequence::<28, 20, 8>();
    sequence::<28, 21, 7>();
    sequence::<28, 22, 6>();
    sequence::<28, 23, 5>();
    sequence::<28, 24, 4>();
    sequence::<28, 25, 3>();
    sequence::<28, 26, 2>();
    sequence::<28, 27, 1>();
    sequence::<32, 0, 0>();
    sequence::<32, 1, 31>();
    sequence::<32, 2, 30>();
    sequence::<32, 3, 29>();
    sequence::<32, 4, 28>();
    sequence::<32, 5, 27>();
    sequence::<32, 6, 26>();
    sequence::<32, 7, 25>();
    sequence::<32, 8, 24>();
    sequence::<32, 9, 23>();
    sequence::<32, 10, 22>();
    sequence::<32, 11, 21>();
    sequence::<32, 12, 20>();
    sequence::<32, 13, 19>();
    sequence::<32, 14, 18>();
    sequence::<32, 15, 17>();
    sequence::<32, 16, 16>();
    sequence::<32, 17, 15>();
    sequence::<32, 18, 14>();
    sequence::<32, 19, 13>();
    sequence::<32, 20, 12>();
    sequence::<32, 21, 11>();
    sequence::<32, 22, 10>();
    sequence::<32, 23, 9>();
    sequence::<32, 24, 8>();
    sequence::<32, 25, 7>();
    sequence::<32, 26, 6>();
    sequence::<32, 27, 5>();
    sequence::<32, 28, 4>();
    sequence::<32, 29, 3>();
    sequence::<32, 30, 2>();
    sequence::<32, 31, 1>();
    sequence::<63, 0, 0>();
    sequence::<63, 1, 62>();
    sequence::<63, 2, 61>();
    sequence::<63, 3, 60>();
    sequence::<63, 4, 59>();
    sequence::<63, 5, 58>();
    sequence::<63, 6, 57>();
    sequence::<63, 7, 56>();
    sequence::<63, 8, 55>();
    sequence::<63, 9, 54>();
    sequence::<63, 10, 53>();
    sequence::<63, 11, 52>();
    sequence::<63, 12, 51>();
    sequence::<63, 13, 50>();
    sequence::<63, 14, 49>();
    sequence::<63, 15, 48>();
    sequence::<63, 16, 47>();
    sequence::<63, 17, 46>();
    sequence::<63, 18, 45>();
    sequence::<63, 19, 44>();
    sequence::<63, 20, 43>();
    sequence::<63, 21, 42>();
    sequence::<63, 22, 41>();
    sequence::<63, 23, 40>();
    sequence::<63, 24, 39>();
    sequence::<63, 25, 38>();
    sequence::<63, 26, 37>();
    sequence::<63, 27, 36>();
    sequence::<63, 28, 35>();
    sequence::<63, 29, 34>();
    sequence::<63, 30, 33>();
    sequence::<63, 31, 32>();
    sequence::<63, 32, 31>();
    sequence::<63, 33, 30>();
    sequence::<63, 34, 29>();
    sequence::<63, 35, 28>();
    sequence::<63, 36, 27>();
    sequence::<63, 37, 26>();
    sequence::<63, 38, 25>();
    sequence::<63, 39, 24>();
    sequence::<63, 40, 23>();
    sequence::<63, 41, 22>();
    sequence::<63, 42, 21>();
    sequence::<63, 43, 20>();
    sequence::<63, 44, 19>();
    sequence::<63, 45, 18>();
    sequence::<63, 46, 17>();
    sequence::<63, 47, 16>();
    sequence::<63, 48, 15>();
    sequence::<63, 49, 14>();
    sequence::<63, 50, 13>();
    sequence::<63, 51, 12>();
    sequence::<63, 52, 11>();
    sequence::<63, 53, 10>();
    sequence::<63, 54, 9>();
    sequence::<63, 55, 8>();
    sequence::<63, 56, 7>();
    sequence::<63, 57, 6>();
    sequence::<63, 58, 5>();
    sequence::<63, 59, 4>();
    sequence::<63, 60, 3>();
    sequence::<63, 61, 2>();
    sequence::<63, 62, 1>();
    sequence::<64, 0, 0>();
    sequence::<64, 1, 63>();
    sequence::<64, 2, 62>();
    sequence::<64, 3, 61>();
    sequence::<64, 4, 60>();
    sequence::<64, 5, 59>();
    sequence::<64, 6, 58>();
    sequence::<64, 7, 57>();
    sequence::<64, 8, 56>();
    sequence::<64, 9, 55>();
    sequence::<64, 10, 54>();
    sequence::<64, 11, 53>();
    sequence::<64, 12, 52>();
    sequence::<64, 13, 51>();
    sequence::<64, 14, 50>();
    sequence::<64, 15, 49>();
    sequence::<64, 16, 48>();
    sequence::<64, 17, 47>();
    sequence::<64, 18, 46>();
    sequence::<64, 19, 45>();
    sequence::<64, 20, 44>();
    sequence::<64, 21, 43>();
    sequence::<64, 22, 42>();
    sequence::<64, 23, 41>();
    sequence::<64, 24, 40>();
    sequence::<64, 25, 39>();
    sequence::<64, 26, 38>();
    sequence::<64, 27, 37>();
    sequence::<64, 28, 36>();
    sequence::<64, 29, 35>();
    sequence::<64, 30, 34>();
    sequence::<64, 31, 33>();
    sequence::<64, 32, 32>();
    sequence::<64, 33, 31>();
    sequence::<64, 34, 30>();
    sequence::<64, 35, 29>();
    sequence::<64, 36, 28>();
    sequence::<64, 37, 27>();
    sequence::<64, 38, 26>();
    sequence::<64, 39, 25>();
    sequence::<64, 40, 24>();
    sequence::<64, 41, 23>();
    sequence::<64, 42, 22>();
    sequence::<64, 43, 21>();
    sequence::<64, 44, 20>();
    sequence::<64, 45, 19>();
    sequence::<64, 46, 18>();
    sequence::<64, 47, 17>();
    sequence::<64, 48, 16>();
    sequence::<64, 49, 15>();
    sequence::<64, 50, 14>();
    sequence::<64, 51, 13>();
    sequence::<64, 52, 12>();
    sequence::<64, 53, 11>();
    sequence::<64, 54, 10>();
    sequence::<64, 55, 9>();
    sequence::<64, 56, 8>();
    sequence::<64, 57, 7>();
    sequence::<64, 58, 6>();
    sequence::<64, 59, 5>();
    sequence::<64, 60, 4>();
    sequence::<64, 61, 3>();
    sequence::<64, 62, 2>();
    sequence::<64, 63, 1>();
}
