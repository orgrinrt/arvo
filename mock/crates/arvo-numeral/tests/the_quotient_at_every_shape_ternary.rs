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
//! This file is radix three. The sweep is one test binary a radix, since the
//! three in one do not build in the memory a container has.

mod common;

include!("every_shape/quotient.rs");

bands!(
    ternary_at,
    ternary_quotients_at_widths_1_to_16,
    ternary_quotients_at_widths_17_to_32,
    ternary_quotients_at_widths_33_to_48,
    ternary_quotients_at_widths_49_to_64
);
