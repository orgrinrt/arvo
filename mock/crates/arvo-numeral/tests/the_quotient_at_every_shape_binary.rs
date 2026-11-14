//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient through the value type at every shape a value is held for,
//! against the tests' own arithmetic.
//!
//! `common::shapes` says what a shape is and which are taken. The exponents
//! here, per width `w`, are the highest the format's quotient admits, zero,
//! minus one, minus half of `w` rounded up, and the deepest it admits. A test in
//! each radix file holds the two ends against the format's own verdict at every
//! width and both ranges, admitted at each and refused one past it.
//!
//! The dividends of a signature are every member up to eight bits and the edge
//! lattice with seeded members above that. Its divisors are constants of its
//! format: the ends of the range, one, two and three with their negations, a
//! large odd slot, and the slot of the number one.
//!
//! This file is radix two. The sweep is one test binary a radix, since the
//! three in one do not build in the memory a container has.

mod common;

include!("every_shape/quotient.rs");

bands!(
    binary_at,
    binary_quotients_at_widths_1_to_16,
    binary_quotients_at_widths_17_to_32,
    binary_quotients_at_widths_33_to_48,
    binary_quotients_at_widths_49_to_64
);

use arvo_format::points::Integer;

/// The integer point is a format of its own beside the fixed point at
/// exponent zero, so it is taken at every width as well.
macro_rules! integer_at {
    ($w:literal, $shapes:expr) => {
        signed!(Integer<$w>, $w, 2, 0, $shapes)
    };
}

#[test]
fn integer_quotients_at_every_width() {
    let mut shapes = Shapes::default();
    widths_1_to_16!(integer_at!(&mut shapes));
    widths_17_to_32!(integer_at!(&mut shapes));
    widths_33_to_48!(integer_at!(&mut shapes));
    widths_49_to_64!(integer_at!(&mut shapes));
    shapes.agree("`div`", 64 * 10);
}

quotient_edges!(
    the_binary_family_ends_where_the_quotient_does,
    2,
    binary_signed,
    binary_unsigned
);

/// A few edges written out, so the edge arithmetic is held to numbers as well
/// as to the format.
#[test]
fn the_binary_edges_are_where_arithmetic_puts_them() {
    assert_eq!(deep(64, true, 2), -192);
    assert_eq!(deep(64, false, 2), -192);
    assert_eq!(deep(33, true, 10), -67);
    assert_eq!(deep(17, false, 3), -150);
    assert_eq!(top(64, true, 2), 192);
    assert_eq!(top(64, false, 2), 192);
    assert_eq!(top(1, true, 2), 255);
    assert_eq!(top(64, true, 10), 58);
}
