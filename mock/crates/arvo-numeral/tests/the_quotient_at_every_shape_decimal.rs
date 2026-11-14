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
//! This file is radix ten. The sweep is one test binary a radix, since the
//! three in one do not build in the memory a container has.

mod common;

include!("every_shape/quotient.rs");

bands!(
    decimal_at,
    decimal_quotients_at_widths_1_to_16,
    decimal_quotients_at_widths_17_to_32,
    decimal_quotients_at_widths_33_to_48,
    decimal_quotients_at_widths_49_to_64
);

quotient_edges!(
    the_decimal_family_ends_where_the_quotient_does,
    10,
    decimal_signed,
    decimal_unsigned
);
