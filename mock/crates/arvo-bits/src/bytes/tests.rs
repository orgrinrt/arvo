//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The walk law, and the `xor` and `halves` laws `DESIGN.md.tmpl` names.

use notko::Maybe;

use crate::{Bits, Bytes};

/// Walks `run` to its end, checking each byte against `expected` in order,
/// and returns how many it saw.
fn walk_against(run: Bytes<'_>, expected: &[u8]) -> usize {
    let mut rest = run;
    let mut seen = 0;
    while let Maybe::Is((byte, after)) = rest.split_first() {
        assert_eq!(
            byte.raw(),
            u64::from(expected[seen]),
            "byte {seen} of the walk"
        );
        seen += 1;
        rest = after;
    }
    seen
}

#[test]
fn the_empty_run_ends_at_once() {
    assert!(matches!(Bytes::of(&[]).split_first(), Maybe::Isnt));
    assert_eq!(walk_against(Bytes::of(&[]), &[]), 0);
}

/// Every byte value, in order, so a walk that dropped, reordered or masked a
/// byte wrong shows here.
#[test]
fn the_walk_yields_every_byte_value_in_order() {
    let mut all = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        all[i] = i as u8;
        i += 1;
    }
    assert_eq!(walk_against(Bytes::of(&all), &all), 256);
    assert_eq!(Bytes::of(&all).host(), &all[..]);
}

/// The planted case: a walk checked against a different slice fails.
#[test]
#[should_panic(expected = "byte 1 of the walk")]
fn a_walk_against_the_wrong_bytes_fails() {
    walk_against(Bytes::of(b"ab"), b"ac");
}

/// `xor` agrees with the host's xor, is its own inverse and has zero as
/// identity, at both mask arms and an interior width, over a spread of values.
#[test]
fn xor_agrees_with_the_hosts_and_inverts_itself() {
    let values = [
        0,
        1,
        0x5555_5555_5555_5555,
        0xAAAA_AAAA_AAAA_AAAA,
        0x8000_0000_0000_0001,
        u64::MAX,
    ];
    fn at<const N: u32>(values: [u64; 6]) {
        for a in values {
            for b in values {
                let (x, y) = (Bits::<N>::masked(a), Bits::<N>::masked(b));
                assert_eq!(x.xor(y).raw(), x.raw() ^ y.raw());
                assert_eq!(x.xor(y).xor(y), x);
                assert_eq!(x.xor(Bits::<N>::masked(0)), x);
            }
        }
    }
    at::<1>(values);
    at::<8>(values);
    at::<32>(values);
    at::<63>(values);
    at::<64>(values);
}

/// `halves` loses nothing, with only the low half set, only the high half,
/// both, and neither.
#[test]
fn halves_lose_nothing() {
    for raw in [0u128, 0xDEAD_BEEF, 0xDEAD_BEEF << 64, u128::MAX, (1 << 127) | 1] {
        let [low, high] = Bits::<64>::halves(raw);
        assert_eq!(low.raw(), raw as u64);
        assert_eq!(high.raw(), (raw >> 64) as u64);
        assert_eq!((u128::from(high.raw()) << 64) | u128::from(low.raw()), raw);
    }
}
