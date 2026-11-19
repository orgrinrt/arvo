//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! FNV-1a, by Fowler, Noll and Vo, at 32 and 64 bits.

use arvo_bits::{Bits, Bytes};
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Quiet, Uint};
use notko::Maybe;

use crate::{Carrier, Digest, Output};

/// FNV-1a, `BITS` wide.
///
/// Each byte is xored into the state and the state multiplied by the
/// width's prime, wrapping. The offset bases and primes are the reference
/// ones, so a value here equals the value any other FNV-1a gives for the
/// same bytes, which is what a consumer with stored FNV-1a values needs.
///
/// Neither step is written here. The product is `Num::mul` under a wrapping
/// overflow policy, with `Quiet` saying the wrap is what it means, and the
/// xor is `Bits::xor`. The state crosses between the two through
/// `Num::index` and `Num::from_index`, the host integer between them
/// inferred from `Bits` rather than named.
pub struct Fnv1a<const BITS: u32>
where
    Output<BITS>: Carrier;

/// The walk over the bytes, written once and instantiated at each width:
/// the width, its state type, its offset basis and its prime are what differ.
macro_rules! fnv1a_at {
    ($bits:literal, $state:ident, $offset:literal, $prime:literal) => {
        /// The state at this width: the product reduced modulo two to the width.
        type $state = Uint<$bits, Adapt<TowardZero, Wrap>, Quiet>;

        impl Fnv1a<$bits> {
            const OFFSET: $state = $state::constant::<$offset>();
            const PRIME: $state = $state::constant::<$prime>();

            #[doc = concat!("The ", stringify!($bits), " bit FNV-1a digest of `bytes`.")]
            #[must_use]
            #[inline]
            pub const fn of(bytes: Bytes<'_>) -> Digest<$bits> {
                let mut state = Self::OFFSET;
                let mut rest = bytes;
                while let Maybe::Is((byte, after)) = rest.split_first() {
                    let mixed = Bits::<$bits>::masked(state.index()).xor(byte.cast());
                    state = $state::from_index(mixed.raw()).mul(Self::PRIME);
                    rest = after;
                }
                Digest::from_raw(Bits::masked(state.index()))
            }
        }
    };
}

fnv1a_at!(32, State32, 0x811C_9DC5, 0x0100_0193);
fnv1a_at!(64, State64, 0xCBF2_9CE4_8422_2325, 0x0000_0100_0000_01B3);
