//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product through the value type at every shape a value is held for,
//! against the tests' own arithmetic.
//!
//! `common::shapes` says what a shape is and which are taken. At exponent zero
//! the product is the carrier's and the bands hold it at the integer points;
//! here it is taken at every radix and at the exponents where it is the
//! format's function as well. The exponents, per width `w`, are the highest
//! one up to one that the format's product admits, the highest up to zero,
//! minus one, minus half of `w` rounded up, and minus `w`, where the range is a
//! pure fraction, or the deepest the radix has a denominator for. A test in the
//! radix two file holds the two ends against the format's own verdict.
//!
//! The pairs of a signature are every pair up to six bits, and above that the
//! edge lattice against the ends and the neighbours of zero with seeded pairs.
//!
//! This file is radix two. The sweep is one test binary a radix, since the
//! three in one do not build in the memory a container has.

mod common;

include!("every_shape/product.rs");

bands!(
    binary_at,
    binary_products_at_widths_1_to_16,
    binary_products_at_widths_17_to_32,
    binary_products_at_widths_33_to_48,
    binary_products_at_widths_49_to_64
);

/// The family's ends are the obligation's own: one past either is refused.
#[test]
fn the_end_exponents_are_the_last_the_product_admits() {
    use arvo_format::is_multipliable;
    // The unsigned 64-bit product leaves the slot index at exponent zero.
    assert_eq!(whole(64, false, 2), -1);
    assert!(is_multipliable::<binary_unsigned!(64, -1)>().get());
    assert!(!is_multipliable::<binary_unsigned!(64, 0)>().get());
    // The signed one holds at zero and not at one.
    assert_eq!(top(64, true, 2), 0);
    assert!(is_multipliable::<binary_signed!(64, 0)>().get());
    assert!(!is_multipliable::<binary_signed!(64, 1)>().get());
    assert_eq!(top(63, true, 2), 1);
    assert!(is_multipliable::<binary_signed!(63, 1)>().get());
    // A decimal denominator ends at ten to the thirty-eighth.
    assert_eq!(deep(64, true, 10), -38);
    assert!(is_multipliable::<decimal_signed!(64, -38)>().get());
    assert!(!is_multipliable::<decimal_signed!(64, -39)>().get());
    assert_eq!(deep(64, false, 3), -64);
    assert!(is_multipliable::<ternary_unsigned!(64, -64)>().get());
}
