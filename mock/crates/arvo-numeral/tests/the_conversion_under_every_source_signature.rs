//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion under every source signature, at every width and each
//! radix, against the tests' own arithmetic. The expected slot reads nothing of
//! the source's mode or policy, so a conversion that did would disagree here.
//! `common::exponents` says what is swept.

mod common;

source_signatures!(
    no_source_signature_moves_a_binary_conversion,
    binary_signed,
    binary_unsigned
);
source_signatures!(
    no_source_signature_moves_a_decimal_conversion,
    decimal_signed,
    decimal_unsigned
);
source_signatures!(
    no_source_signature_moves_a_ternary_conversion,
    ternary_signed,
    ternary_unsigned
);
