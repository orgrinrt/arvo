//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! FNV-1a, by Fowler, Noll and Vo, at 32 bits.

use arvo_bits::{Bits, Bytes};
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Quiet, Uint};
use notko::Maybe;

use crate::Digest;

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
pub struct Fnv1a<const BITS: u16>;

/// The 32 bit state: the product reduced modulo two to the thirty-second.
type State32 = Uint<32, Adapt<TowardZero, Wrap>, Quiet>;

impl Fnv1a<32> {
    const OFFSET: State32 = State32::constant::<0x811C_9DC5>();
    const PRIME: State32 = State32::constant::<0x0100_0193>();

    /// The 32 bit FNV-1a digest of `bytes`.
    #[must_use]
    pub const fn of(bytes: Bytes<'_>) -> Digest<32> {
        let mut state = Self::OFFSET;
        let mut rest = bytes;
        while let Maybe::Is((byte, after)) = rest.split_first() {
            let mixed = Bits::<32>::masked(state.index()).xor(byte.cast());
            state = State32::from_index(mixed.raw()).mul(Self::PRIME);
            rest = after;
        }
        Digest::from_raw(Bits::masked(state.index()))
    }
}

// FIXME(arvo_format_admits_the_wrapping_product_over_the_unsigned_64): `Fnv1a<64>`,
// the same walk over `Uint<64, Adapt<TowardZero, Wrap>, Quiet>` at the offset basis
// 0xCBF2_9CE4_8422_2325 and prime 0x0000_0100_0000_01B3, joins once arvo-format
// admits the wrapping product over the unsigned 64-bit range; until then that
// `mul` is refused at build.
