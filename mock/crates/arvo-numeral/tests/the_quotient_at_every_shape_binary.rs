//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient through the value type at every shape a value is held for,
//! against the tests' own arithmetic.
//!
//! `common::shapes` says what a shape is and which are taken. The exponents
//! here, per width `w`, are zero, one, minus one, minus half of `w` rounded up,
//! and the deepest one the format's quotient admits at that width and radix,
//! which a test in the radix two file holds against the format's own verdict.
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

/// The family's last exponent is the last one the format admits: one deeper is
/// refused, so the sweep ends at the obligation's own edge.
#[test]
fn the_deepest_exponent_is_the_last_the_quotient_admits() {
    use arvo_format::is_divisible;
    assert!(is_divisible::<binary_signed!(64, -63)>().get());
    assert!(!is_divisible::<binary_signed!(64, -64)>().get());
    assert!(is_divisible::<binary_unsigned!(64, -63)>().get());
    assert!(!is_divisible::<binary_unsigned!(64, -64)>().get());
    assert_eq!(deep(64, true, 2), -63);
    assert_eq!(deep(64, false, 2), -63);
    assert!(is_divisible::<decimal_signed!(33, -28)>().get());
    assert!(!is_divisible::<decimal_signed!(33, -29)>().get());
    assert_eq!(deep(33, true, 10), -28);
    assert!(is_divisible::<ternary_unsigned!(17, -69)>().get());
    assert!(!is_divisible::<ternary_unsigned!(17, -70)>().get());
    assert_eq!(deep(17, false, 3), -69);
}
