//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at radix three between every pair of widths, both sides at
//! exponent zero, against the tests' own arithmetic. `common::widths` says
//! what is swept. No member falls between two slots here, so the mode named
//! decides nothing and the policy decides every member the target does not
//! hold.

mod common;

width_matrix!(ternary_signed, ternary_unsigned, 0, 0, TowardZero);
