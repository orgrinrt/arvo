//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The tier through `const fn`s generic over `[const]` bounds, evaluated in
//! constants, and `BitWidth::WIDTH` at every width against the full mask.

#![feature(const_trait_impl)]

use arvo_bits::{BitAccess, BitLogic, BitSequence, BitWidth, Bits};
use arvo_format::{Bool, Width};

/// Whether two sets meet: the intersection test the set obligation names.
const fn meet<B: [const] BitLogic>(a: B, b: B) -> Bool {
    a.and(b).is_empty().not()
}

/// The top bit of a container, read after a rotation up by one.
const fn top_after_rotation<B: [const] BitAccess + [const] BitSequence>(b: B) -> Bool {
    b.rotate_up::<1>().bit::<0>()
}

const fn width_of<B: BitWidth>() -> Width {
    B::WIDTH
}

const MEETS: Bool = meet(Bits::<8>::masked(0b1100), Bits::<8>::masked(0b0110));
const APART: Bool = meet(Bits::<8>::masked(0b1100), Bits::<8>::masked(0b0011));
const TOP_SET: Bool = top_after_rotation(Bits::<28>::masked(1 << 27));
const TOP_CLEAR: Bool = top_after_rotation(Bits::<28>::masked(1 << 26));
const ALIGNED: Bits<32> = Bits::<32>::masked(13 + 3).and(Bits::<32>::masked(3).not());

/// What `every` gives: each of the eleven operations once, in the order the tier
/// declares them.
type Every<B> = (Bool, B, B, B, B, B, Bool, B, B, B, B);

/// Every operation of the tier, through `[const]` bounds alone.
const fn every<B: [const] BitAccess + [const] BitLogic + [const] BitSequence>(
    a: B,
    b: B,
) -> Every<B> {
    (
        a.bit::<1>(),
        a.with_bit::<0>(Bool::TRUE),
        a.and(b),
        a.or(b),
        a.xor(b),
        a.not(),
        a.is_empty(),
        a.shift_up::<3>(),
        a.shift_down::<3>(),
        a.rotate_up::<3>(),
        a.rotate_down::<3>(),
    )
}

const EVERY: Every<Bits<8>> = every(
    Bits::<8>::masked(0b1011_0010),
    Bits::<8>::masked(0b0110_0110),
);

#[test]
fn the_tier_evaluates_in_a_constant() {
    assert!(MEETS.get());
    assert!(!APART.get());
    assert!(TOP_SET.get());
    assert!(!TOP_CLEAR.get());
    assert_eq!(ALIGNED.raw(), 16);
}

#[test]
fn every_operation_of_the_tier_evaluates_in_a_constant() {
    let (bit, with_bit, and, or, xor, not, is_empty, up, down, rotated_up, rotated_down) = EVERY;
    assert!(bit.get());
    assert_eq!(with_bit.raw(), 0b1011_0011);
    assert_eq!(and.raw(), 0b0010_0010);
    assert_eq!(or.raw(), 0b1111_0110);
    assert_eq!(xor.raw(), 0b1101_0100);
    assert_eq!(not.raw(), 0b0100_1101);
    assert!(!is_empty.get());
    assert_eq!(up.raw(), 0b1001_0000);
    assert_eq!(down.raw(), 0b0001_0110);
    assert_eq!(rotated_up.raw(), 0b1001_0101);
    assert_eq!(rotated_down.raw(), 0b0101_0110);
}

/// Whether the width `Bits<N>` gives back through the tier counts the bits its
/// full mask sets, which the mask computes from `N` without reading `WIDTH`.
const fn the_full_mask_has_the_width<const N: u32>() -> bool {
    Bits::<N>::masked(u64::MAX).raw().count_ones() == width_of::<Bits<N>>().count()
}

macro_rules! widths {
    ($($n:literal)+) => {
        $( assert!(const { the_full_mask_has_the_width::<$n>() }, "the full mask at {} bits", $n); )+
    };
}

#[test]
fn the_width_counts_the_bits_of_the_full_mask() {
    widths!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
    widths!(33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60);
    widths!(61 62 63 64);
}
