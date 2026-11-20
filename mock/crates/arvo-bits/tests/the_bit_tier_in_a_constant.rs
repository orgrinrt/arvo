//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The tier through `const fn`s generic over `[const]` bounds, evaluated in
//! constants, and `BitWidth::WIDTH` at every width.

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

#[test]
fn the_tier_evaluates_in_a_constant() {
    assert!(MEETS.get());
    assert!(!APART.get());
    assert!(TOP_SET.get());
    assert!(!TOP_CLEAR.get());
    assert_eq!(ALIGNED.raw(), 16);
}

macro_rules! widths {
    ($($n:literal)+) => { $( assert_eq!(width_of::<Bits<$n>>(), Width::bits($n)); )+ };
}

#[test]
fn the_width_is_the_declared_one() {
    widths!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
    widths!(33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60);
    widths!(61 62 63 64);
}
