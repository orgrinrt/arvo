//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at radix three at the positive edge of either side, at every
//! width and pair of ranges, against the tests' own arithmetic, and each edge
//! held against the format's verdict. `common::exponents` says what is swept
//! and `common::edges` where each edge is.

mod common;

positive_edges!(
    3,
    ternary_signed,
    ternary_unsigned,
    ternary_positive_edges_at_widths_1_to_16,
    ternary_positive_edges_at_widths_17_to_32,
    ternary_positive_edges_at_widths_33_to_48,
    ternary_positive_edges_at_widths_49_to_64
);
