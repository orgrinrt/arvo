//! Opaque N-bit container. Non-arithmetic; compared by identity.
//!
//! Stores an N-bit value in a `u64`; only the low N bits are
//! addressable. `N` is compile-time asserted to lie in 1..=64.
//! Ordering is not provided; these are identity values, not
//! numerics.
//!
//! Primary consumer: `arvo-hash` `ContentHash` alias (`Bits<28>`).

use arvo::{Bool, USize};

use crate::traits::{BitAccess, BitWidth};

/// N-bit opaque bit-pattern. Construct via `Bits::new`; the
/// constructor masks to the low N bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Bits<const N: u8>(u64);

impl<const N: u8> Bits<N> {
    const _BOUNDS: () = {
        assert!(N > 0, "Bits<0> is invalid; use Bool for 1-bit values");
        assert!(N <= 64, "Bits<N> requires N <= 64");
    };

    const MASK: u64 = if N == 64 { u64::MAX } else { (1u64 << N) - 1 };

    /// Construct a `Bits<N>` from raw bits, masking to the low N bits.
    pub const fn new(raw: u64) -> Self {
        let _ = Self::_BOUNDS;
        Self(raw & Self::MASK)
    }

    /// The underlying bit pattern, in the low N bits of the `u64`.
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl<const N: u8> BitWidth for Bits<N> {
    const WIDTH: USize = USize(N as usize);
}

impl<const N: u8> BitAccess for Bits<N> {
    fn bit(self, idx: USize) -> Bool {
        if idx.0 >= N as usize {
            return Bool::FALSE;
        }
        Bool(((self.0 >> idx.0) & 1) != 0)
    }

    fn with_bit_set(self, idx: USize) -> Self {
        if idx.0 >= N as usize {
            return self;
        }
        Self::new(self.0 | (1u64 << idx.0))
    }

    fn with_bit_cleared(self, idx: USize) -> Self {
        if idx.0 >= N as usize {
            return self;
        }
        Self::new(self.0 & !(1u64 << idx.0))
    }

    fn with_bit_toggled(self, idx: USize) -> Self {
        if idx.0 >= N as usize {
            return self;
        }
        Self::new(self.0 ^ (1u64 << idx.0))
    }
}
