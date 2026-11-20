//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `BitSequence` and the inherent shifts and rotations, at every amount below
//! every width from one to sixty-four, against the host's shift and a rotation
//! moved one bit at a time.
//!
//! Each amount is a const generic, so `common::every_index` expands one call
//! per width and amount rather than looping. The third parameter is `N - K`,
//! or zero at `K` zero, the amount a rotation down has to take to equal the
//! rotation up by `K`.

mod common;

use arvo_bits::{BitSequence, Bits};
use common::{every_index, mask, rotated_up, values};

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

macro_rules! call {
    ($n:literal $k:literal $nk:literal) => {
        sequence::<$n, $k, $nk>();
    };
}

#[test]
fn every_amount_below_every_width_shifts_and_rotates_as_the_host() {
    every_index!(call);
}
