//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The switched 64 bit digest takes FNV-1a below one length and XXH3 from it on.
//!
//! The switch point is read off the digest rather than restated here, so the
//! test holds whatever the measured crossover is: every length agrees with
//! exactly one of the two, the lengths taking FNV-1a all come before the ones
//! taking XXH3, both arms are reached, and the const path gives the same digest.

use arvo_digest::{Digest, Fnv1a, Xxh3};
use arvo_numeral::Bytes;

const LEN: usize = 512;

const fn input() -> [u8; LEN] {
    let mut bytes = [0u8; LEN];
    let mut at = 0;
    while at < LEN {
        bytes[at] = (at as u8).wrapping_mul(31).wrapping_add(7);
        at += 1;
    }
    bytes
}

const INPUT: [u8; LEN] = input();

fn arm(len: usize) -> bool {
    let bytes = &INPUT[.. len];
    let switched = Digest::<64>::of(Bytes::of(bytes));
    let fnv = Fnv1a::<64>::of(Bytes::of(bytes));
    let xxh3 = Xxh3::<64>::of(Bytes::of(bytes));
    assert!(
        switched == fnv || switched == xxh3,
        "length {len} took neither arm"
    );
    switched == xxh3 && switched != fnv
}

fn switch_point(arms: impl Iterator<Item = (usize, bool)>) -> Option<usize> {
    let mut switch = None;
    for (len, took_xxh3) in arms {
        match (switch, took_xxh3) {
            (None, true) => switch = Some(len),
            (Some(at), false) => panic!("length {len} went back to FNV-1a after XXH3 from {at}"),
            _ => {},
        }
    }
    switch
}

#[test]
fn one_switch_point_and_both_arms_reached() {
    let at = switch_point((1 ..= LEN).map(|len| (len, arm(len))))
        .expect("no length up to 512 took XXH3");
    assert!(at > 1, "no length took FNV-1a");
}

#[test]
fn the_const_path_agrees_with_the_runtime_path() {
    for len in 0 ..= LEN {
        let bytes = &INPUT[.. len];
        assert_eq!(
            Digest::<64>::of(Bytes::of(bytes)),
            Digest::<64>::of_const(Bytes::of(bytes)),
            "length {len}"
        );
    }
}

#[test]
fn the_switched_digest_is_usable_in_a_const() {
    const AT_COMPILE: Digest<64> = Digest::<64>::of_const(Bytes::of(b"foobar"));
    assert_eq!(AT_COMPILE, Digest::<64>::of(Bytes::of(b"foobar")));
}

#[test]
#[should_panic(expected = "went back to FNV-1a")]
fn planted_a_second_switch_is_caught() {
    switch_point([(1, false), (2, true), (3, false)].into_iter());
}
