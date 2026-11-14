//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at radix three between every pair of widths, from exponent
//! minus one onto exponent zero, against the tests' own arithmetic.
//! `common::widths` says what is swept. A source slot is one part in three of a
//! target slot, so the mode decides most members, and it is `HalfEven`, the one
//! that reads both the remainder and the slot below.

mod common;

width_matrix!(ternary_signed, ternary_unsigned, -1, 0, HalfEven);
