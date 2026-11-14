//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion between every ordered pair of the radixes two, ten and
//! three, at every width, against the tests' own arithmetic.
//! `common::exponents` says what a pair is run at.

mod common;

radix_pair!(
    binary_onto_decimal,
    binary_signed,
    binary_unsigned,
    decimal_signed,
    decimal_unsigned
);
radix_pair!(
    decimal_onto_binary,
    decimal_signed,
    decimal_unsigned,
    binary_signed,
    binary_unsigned
);
radix_pair!(
    binary_onto_ternary,
    binary_signed,
    binary_unsigned,
    ternary_signed,
    ternary_unsigned
);
radix_pair!(
    ternary_onto_binary,
    ternary_signed,
    ternary_unsigned,
    binary_signed,
    binary_unsigned
);
radix_pair!(
    decimal_onto_ternary,
    decimal_signed,
    decimal_unsigned,
    ternary_signed,
    ternary_unsigned
);
radix_pair!(
    ternary_onto_decimal,
    ternary_signed,
    ternary_unsigned,
    decimal_signed,
    decimal_unsigned
);
