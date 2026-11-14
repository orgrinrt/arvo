//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The switched digest: FNV-1a below the crossover, XXH3 from it on.

use arvo_numeral::{Bool, Bytes};

use crate::{Digest, Fnv1a, Xxh3};

/// Whether `bytes` is shorter than the crossover, the shortest input length digested with XXH3.
///
/// Part of what the digest is: moving it changes the digest of every input
/// between the old value and the new one. Taken from the `digest-crossover`
/// bench, `mock/benches/digest-crossover_n*_findings.md`, and argued in
/// `mock/research/202611140014_digest-crossover/NOTES.md`.
const fn below_crossover(bytes: Bytes<'_>) -> Bool {
    bytes.shorter_than::<16>()
}

impl Digest<64> {
    /// The stack's 64 bit digest of `bytes`, on the runtime path.
    ///
    /// FNV-1a when `bytes` is shorter than the crossover, XXH3 otherwise.
    #[must_use]
    #[inline]
    pub fn of(bytes: Bytes<'_>) -> Self {
        if below_crossover(bytes).get() {
            Fnv1a::<64>::of(bytes)
        } else {
            Xxh3::<64>::of(bytes)
        }
    }

    /// The stack's 64 bit digest of `bytes`, in const context.
    ///
    /// The same rule as [`Digest::<64>::of`], so the same value.
    #[must_use]
    pub const fn of_const(bytes: Bytes<'_>) -> Self {
        if below_crossover(bytes).get() {
            Fnv1a::<64>::of(bytes)
        } else {
            Xxh3::<64>::of_const(bytes)
        }
    }
}
