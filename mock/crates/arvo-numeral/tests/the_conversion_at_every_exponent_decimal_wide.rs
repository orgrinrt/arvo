//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at radix ten between every pair of exponents at each width from 33 to 64,
//! against the tests' own arithmetic. `common::exponents` says what is swept.

mod common;

exponent_matrix!(wide, 10, decimal_signed, decimal_unsigned);
