//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product where the exponent moves the slot.
//!
//! At a quantum exponent other than zero the members' product is not the
//! slots' product: it is scaled by the radix and, below zero, rounded by the
//! declared mode. Every pair of a signed and an unsigned six-bit format at
//! eighths, and of an unsigned one at eights, in every mode and both policies,
//! against the tests' own arithmetic in `common::oracle`, since the value's
//! product here is the format's function. One pair is named in each mode: a
//! tie below zero.
//!
//! The control is the slots' product completed onto the range, which is what a
//! product that ignored the exponent would give. It has to disagree with the
//! value somewhere at each of these formats, or the suite would not see a
//! value that left the exponent out.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, UFixed};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Identity, Multiplicative, Num, Quiet, Sole};
use common::{Tally, exhaustive_products, lattice_and_seeded_products, oracle, range, value};

#[test]
fn the_signed_product_at_eighths_is_the_oracles_on_every_pair() {
    let mut tally = Tally::default();
    every_mode!(Fixed<6, -3>, Wrap, exhaustive_products, &mut tally);
    every_mode!(Fixed<6, -3>, Saturate, exhaustive_products, &mut tally);
    // 64 members, every pair, five modes, two policies.
    assert_eq!(tally.products, 64 * 64 * 10);
    tally.check_products("Fixed<6, -3>");
}

#[test]
fn the_unsigned_product_at_eighths_is_the_oracles_on_every_pair() {
    let mut tally = Tally::default();
    every_mode!(UFixed<6, -3>, Wrap, exhaustive_products, &mut tally);
    every_mode!(UFixed<6, -3>, Saturate, exhaustive_products, &mut tally);
    assert_eq!(tally.products, 64 * 64 * 10);
    tally.check_products("UFixed<6, -3>");
}

#[test]
fn the_unsigned_product_at_eights_is_the_oracles_on_every_pair() {
    let mut tally = Tally::default();
    every_mode!(UFixed<6, 3>, Wrap, exhaustive_products, &mut tally);
    every_mode!(UFixed<6, 3>, Saturate, exhaustive_products, &mut tally);
    assert_eq!(tally.products, 64 * 64 * 10);
    tally.check_products("UFixed<6, 3>");
}

/// The same at 16 and 32 bits, where a range has too many pairs to take each:
/// the edge lattice, every pair, and seeded pairs. A product of two 64-bit
/// slots leaves the tests' wide integer no room to scale, so 64 bits is not
/// here.
#[test]
fn the_product_at_wide_fixed_point_formats_is_the_oracles_on_the_lattice_and_seeded_pairs() {
    let mut tally = Tally::default();
    every_mode!(Fixed<16, -8>, Wrap, lattice_and_seeded_products, &mut tally);
    every_mode!(Fixed<16, -8>, Saturate, lattice_and_seeded_products, &mut tally);
    every_mode!(UFixed<16, -8>, Wrap, lattice_and_seeded_products, &mut tally);
    every_mode!(UFixed<16, -8>, Saturate, lattice_and_seeded_products, &mut tally);
    every_mode!(Fixed<32, -16>, Wrap, lattice_and_seeded_products, &mut tally);
    every_mode!(Fixed<32, -16>, Saturate, lattice_and_seeded_products, &mut tally);
    every_mode!(UFixed<32, -16>, Wrap, lattice_and_seeded_products, &mut tally);
    every_mode!(UFixed<32, -16>, Saturate, lattice_and_seeded_products, &mut tally);
    every_mode!(UFixed<32, 5>, Wrap, lattice_and_seeded_products, &mut tally);
    every_mode!(UFixed<32, 5>, Saturate, lattice_and_seeded_products, &mut tally);
    // Five points, both policies, five modes, and the seeded pairs alone.
    assert!(tally.products > 5 * 10 * common::SEEDED_PAIRS as u64);
    tally.check_products("fixed point at 16 and 32 bits");
}

/// The slot of the product of the members at `a` and `b`.
fn product<S: Sole>(a: i128, b: i128) -> i128 {
    value::<S>(a).mul(value::<S>(b)).slot().index()
}

type Sat<P, R> = Signature<P, Adapt<R, Saturate>>;

/// Minus three eighths times four eighths is minus twelve sixty-fourths, one
/// and a half slots below zero: a tie, and each mode says where it goes.
#[test]
fn a_tie_below_zero_goes_where_each_mode_sends_it() {
    type P = Fixed<6, -3>;
    assert_eq!(product::<Sat<P, TowardZero>>(-3, 4), -1);
    assert_eq!(product::<Sat<P, Ceil>>(-3, 4), -1);
    assert_eq!(product::<Sat<P, HalfUp>>(-3, 4), -1);
    assert_eq!(product::<Sat<P, Floor>>(-3, 4), -2);
    assert_eq!(product::<Sat<P, HalfEven>>(-3, 4), -2);
    // And its neighbour tie, two and a half slots below zero, where half-even
    // goes the other way.
    assert_eq!(product::<Sat<P, HalfEven>>(-5, 4), -2);
    assert_eq!(product::<Sat<P, HalfUp>>(-5, 4), -2);
    assert_eq!(product::<Sat<P, Floor>>(-5, 4), -3);
}

/// Pairs on which the slots' product, completed onto the range with no
/// scaling, differs from the value's product.
fn ignoring_the_exponent_is_wrong_on<S: Sole>() -> u64 {
    let (min, max) = range::<S>();
    let mut wrong = 0;
    for a in min ..= max {
        for b in min ..= max {
            if oracle::completed::<S>(a * b) != product::<S>(a, b) {
                wrong += 1;
            }
        }
    }
    wrong
}

#[test]
fn a_product_that_ignored_the_exponent_would_be_seen() {
    type A<P> = Signature<P, Adapt<Floor, Saturate>>;
    assert!(ignoring_the_exponent_is_wrong_on::<A<Fixed<6, -3>>>() > 0);
    assert!(ignoring_the_exponent_is_wrong_on::<A<UFixed<6, -3>>>() > 0);
    assert!(ignoring_the_exponent_is_wrong_on::<A<UFixed<6, 3>>>() > 0);
}

/// One at eighths is slot eight, and multiplying by it moves no member in any
/// mode, since every such product is on the grid.
fn one_at_slot_eight_moves_nothing<S: Sole>(_: &mut Tally) {
    let one = <Num<S, Quiet> as Identity<Multiplicative>>::IDENTITY;
    assert_eq!(one.slot().index(), 8);
    let (min, max) = range::<S>();
    for a in min ..= max {
        let v = value::<S>(a);
        assert_eq!(v.mul(one), v, "multiplying {a} by one moved it");
        assert_eq!(one.mul(v), v, "multiplying one by {a} moved it");
    }
}

#[test]
fn the_multiplicative_identity_at_eighths_is_slot_eight_and_moves_no_member() {
    let mut tally = Tally::default();
    every_mode!(Fixed<6, -3>, Wrap, one_at_slot_eight_moves_nothing, &mut tally);
    every_mode!(Fixed<6, -3>, Saturate, one_at_slot_eight_moves_nothing, &mut tally);
    every_mode!(UFixed<6, -3>, Wrap, one_at_slot_eight_moves_nothing, &mut tally);
    every_mode!(UFixed<6, -3>, Saturate, one_at_slot_eight_moves_nothing, &mut tally);
}
