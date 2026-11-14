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
//! This file is radix ten. The sweep is one test binary a radix, since the
//! three in one do not build in the memory a container has.

mod common;

include!("every_shape/product.rs");

bands!(
    decimal_at,
    decimal_products_at_widths_1_to_16,
    decimal_products_at_widths_17_to_32,
    decimal_products_at_widths_33_to_48,
    decimal_products_at_widths_49_to_64
);
