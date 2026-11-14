//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion under every source signature, at every width and each
//! radix, against the tests' own arithmetic, from exponent minus one onto zero.
//! The expected slot reads nothing of the source's mode, and on this step the
//! mode decides and no member leaves the target's range, so a conversion that
//! rounded by the source's mode would disagree here. The step that consults the
//! policy is `the_conversion_under_every_source_signature_past_the_range`.
//! `common::exponents` says what is swept.

mod common;

source_signatures!(
    no_source_mode_moves_a_binary_conversion,
    -1 => 0,
    leaves false,
    binary_signed,
    binary_unsigned
);
source_signatures!(
    no_source_mode_moves_a_decimal_conversion,
    -1 => 0,
    leaves false,
    decimal_signed,
    decimal_unsigned
);
source_signatures!(
    no_source_mode_moves_a_ternary_conversion,
    -1 => 0,
    leaves false,
    ternary_signed,
    ternary_unsigned
);
