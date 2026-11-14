//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion under every source signature, at every width and each
//! radix, against the tests' own arithmetic, from exponent zero onto minus one.
//! Every member past the target's range on either end is completed there by the
//! target's policy, and the expected slot reads nothing of the source's, so a
//! conversion that completed by the source's policy, or by saturation wherever
//! the source saturates, would disagree here. The test asserts some member left
//! the range. `common::exponents` says what is swept.

mod common;

source_signatures!(
    no_source_policy_moves_a_binary_conversion,
    0 => -1,
    leaves true,
    binary_signed,
    binary_unsigned
);
source_signatures!(
    no_source_policy_moves_a_decimal_conversion,
    0 => -1,
    leaves true,
    decimal_signed,
    decimal_unsigned
);
source_signatures!(
    no_source_policy_moves_a_ternary_conversion,
    0 => -1,
    leaves true,
    ternary_signed,
    ternary_unsigned
);
