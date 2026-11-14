//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at radix three between every pair of exponents at each width from 1 to 32,
//! against the tests' own arithmetic. `common::exponents` says what is swept.

mod common;

exponent_matrix!(narrow, 3, ternary_signed, ternary_unsigned);
