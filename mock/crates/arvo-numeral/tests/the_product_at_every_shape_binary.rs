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
//! the format's product admits, the highest up to zero, minus one, minus half
//! of `w` rounded up, and the deepest the format's product admits. A test in
//! each radix file holds the two ends against the format's own verdict at
//! every width and both ranges, admitted at each and refused one past it.
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

product_edges!(
    the_binary_family_ends_where_the_product_does,
    2,
    binary_signed,
    binary_unsigned
);

/// A few edges written out, so the edge arithmetic is held to numbers as well
/// as to the format: the unsigned 64-bit product leaves the slot index at
/// exponent zero, the signed one holds at zero, a one-bit range holds to two
/// to the 126th, and the remainder's denominator ends at two to the 127th.
#[test]
fn the_binary_edges_are_where_arithmetic_puts_them() {
    assert_eq!(top(64, false, 2), -1);
    assert_eq!(top(64, true, 2), 0);
    assert_eq!(top(63, true, 2), 2);
    assert_eq!(top(8, true, 2), 112);
    assert_eq!(top(1, true, 2), 126);
    assert_eq!(top(1, false, 2), 126);
    assert_eq!(deep(64, true, 2), -127);
}
