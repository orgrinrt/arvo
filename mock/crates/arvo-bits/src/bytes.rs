//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A borrowed run of the host's bytes, walked one `Bits<8>` at a time.

use notko::Maybe;

use crate::Bits;

/// A borrowed run of the host's bytes.
///
/// The door the host's byte image enters the stack by. It sits here because
/// this crate introduces the container a byte is read into, so a consumer that
/// reads bytes, a digest among them, names no host integer of its own.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bytes<'a>(&'a [u8]);

impl<'a> Bytes<'a> {
    /// The run over a host byte slice.
    #[must_use]
    pub const fn of(host: &'a [u8]) -> Self {
        Self(host)
    }

    /// The host byte slice back, for the one place a host contract needs it.
    #[must_use]
    pub const fn host(self) -> &'a [u8] {
        self.0
    }

    /// The first byte and the rest of the run, or `Isnt` when the run is
    /// empty.
    ///
    /// The one way to walk a run: it needs no index type and no length, and it
    /// is a `const fn`, so a walk over it can be one too.
    #[must_use]
    pub const fn split_first(self) -> Maybe<(Bits<8>, Bytes<'a>)> {
        match self.0 {
            [first, rest @ ..] => Maybe::Is((Bits::<8>::masked(*first as u64), Bytes(rest))),
            [] => Maybe::Isnt,
        }
    }
}

#[cfg(test)]
mod tests;
