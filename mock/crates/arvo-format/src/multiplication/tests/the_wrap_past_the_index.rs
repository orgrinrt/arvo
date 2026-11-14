//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The wrapping product on the unsigned 64-bit range, whose top corner the slot
//! index does not carry, against the host's wrapping product on 64 unsigned bits.
//!
//! At exponent `k` a member at slot `a` is `a 2^k`, so the product of two is
//! the member at slot `a b 2^k`, which wrapping reads modulo `2^64`: the host's
//! `a.wrapping_mul(b).wrapping_mul(2^k)`, which reads no arvo code.

use super::*;
use crate::apply::Dither;
use crate::multiplication::mul;
use crate::points::UFixed;
use crate::rounding::TowardZero;

type Wrapped<const K: i32> = Signature<UFixed<64, K>, Adapt<TowardZero, Wrap>>;

/// Pairs within 40 of either end of the range, and a few across it.
fn pairs(mut visit: impl FnMut(u64, u64)) {
    let near = |i: u64| if i < 40 { i } else { u64::MAX - (i - 40) };
    for i in 0 .. 80 {
        for j in 0 .. 80 {
            visit(near(i), near(j));
        }
    }
    for (a, b) in [(1 << 32, 1 << 32), (u64::MAX / 3, 7), (0x9E37_79B9_7F4A_7C15, 0x100_0000_01B3)]
    {
        visit(a, b);
    }
}

/// How many pairs `mul` under `Wrapped<K>` puts anywhere but the host's slot.
fn strays<const K: i32>() -> usize
where
    UFixed<64, K>: Format,
{
    let scale = 1_u64 << K;
    let mut strays = 0;
    pairs(|a, b| {
        let got = mul::<Wrapped<K>>(Slot::at(a as i128), Slot::at(b as i128), Dither::UNUSED);
        let want = a.wrapping_mul(b).wrapping_mul(scale);
        strays += usize::from(got != Slot::at(want as i128));
    });
    strays
}

#[test]
fn the_top_corner_wraps_as_the_host_s_product_does() {
    // `(2^64 - 1)^2 = 2^128 - 2^65 + 1`, past the slot index, is 1 modulo `2^64`,
    // and `(2^64 - 1) 2` is `2^64 - 2`.
    let top = Slot::at(u64::MAX as i128);
    assert_eq!(mul::<Wrapped<0>>(top, top, Dither::UNUSED), Slot::at(1));
    assert_eq!(
        mul::<Wrapped<0>>(top, Slot::at(2), Dither::UNUSED),
        Slot::at((u64::MAX - 1) as i128)
    );
}

#[test]
fn every_pair_near_the_ends_wraps_as_the_host_s_product_does() {
    assert_eq!(strays::<0>(), 0);
    assert_eq!(strays::<3>(), 0);
    assert_eq!(strays::<63>(), 0);
}

#[test]
fn the_unsigned_64_bit_product_is_admitted_under_every_policy() {
    use crate::multiplication::is_multipliable;
    use crate::overflow::{Clamp, Saturate};
    assert!(is_multipliable::<UFixed<64, 0>>().get());
    // The top corner `(2^64 - 1)^2` is past `2^127` and pins to the range's top.
    let top = Slot::at(u64::MAX as i128);
    type Sat = Signature<UFixed<64, 0>, Adapt<TowardZero, Saturate>>;
    type Cl = Signature<UFixed<64, 0>, Adapt<TowardZero, Clamp>>;
    assert_eq!(mul::<Sat>(top, top, Dither::UNUSED), top);
    assert_eq!(mul::<Cl>(top, top, Dither::UNUSED), top);
    assert_eq!(mul::<Sat>(top, Slot::at(1), Dither::UNUSED), top);
    // A condition every product owes still refuses: a step leaving the four limbs.
    assert!(!is_multipliable::<UFixed<64, -300>>().get());
    // The unsigned 64-bit range at exponent zero: admitted under wrapping only.
    assert!(is_multipliable_under::<Wrapped<0>>().get());
    assert!(
        !is_multipliable_under::<Signature<UFixed<64, 0>, Adapt<TowardZero, Saturate>>>().get()
    );
    assert!(!is_multipliable_under::<Signature<UFixed<64, 0>, Adapt<TowardZero, Clamp>>>().get());
    // One step down the exponent the corner is carried, so every policy is admitted.
    assert!(
        is_multipliable_under::<Signature<UFixed<64, -1>, Adapt<TowardZero, Saturate>>>().get()
    );
    assert!(is_multipliable_under::<Signature<UFixed<64, -1>, Adapt<TowardZero, Clamp>>>().get());
    // A condition every product owes refuses under wrapping too: an exponent
    // whose `D = 2^300` leaves the four limbs.
    assert!(!is_multipliable_under::<Signature<UFixed<64, -300>, Adapt<TowardZero, Wrap>>>().get());
    assert!(!crate::multiplication::is_multipliable::<UFixed<64, -300>>().get());
}

#[test]
fn the_admission_ends_where_the_step_stops_forming() {
    use crate::multiplication::{is_multipliable, is_multipliable_under};
    use crate::overflow::Saturate;
    // Under wrapping the top corner `(2^64 - 1)^2 2^k` is formed up to `k = 128`
    // and passes `2^256` at `k = 129`.
    assert!(is_multipliable_under::<Wrapped<128>>().get());
    assert!(!is_multipliable_under::<Wrapped<129>>().get());
    // Below zero `D = 2^-k` is formed down to `k = -255` and passes `2^256 - 1`
    // at `k = -256`, under every policy.
    assert!(is_multipliable::<UFixed<64, -255>>().get());
    assert!(!is_multipliable::<UFixed<64, -256>>().get());
    assert!(is_multipliable_under::<Wrapped<-255>>().get());
    assert!(!is_multipliable_under::<Wrapped<-256>>().get());
    assert!(
        is_multipliable_under::<Signature<UFixed<64, -255>, Adapt<TowardZero, Saturate>>>().get()
    );
    assert!(
        !is_multipliable_under::<Signature<UFixed<64, -256>, Adapt<TowardZero, Saturate>>>().get()
    );
    // The narrower range is held at every exponent from `-255` to 1.
    assert!(is_multipliable::<UFixed<63, 1>>().get());
    assert!(!is_multipliable::<UFixed<63, 2>>().get());
    assert!(is_multipliable::<UFixed<63, -255>>().get());
}
