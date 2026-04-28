//! `HasBitWidth` / `BitAccess` / `BitSequence` / `BitLogic` impls on
//! the L0 `arvo_storage::Bits<N, S>` storage primitive.
//!
//! Per round 202604271346 D-12, the trait declarations live in this
//! crate (`arvo-bits-contracts`); the blanket impls on `Bits<N, S>`
//! land here too because orphan rules require trait + foreign-type
//! impls to share a crate (`Bits` is in `arvo-storage`).

use arvo_storage::{Bits, Bool, USize};
use arvo_strategy::{Hot, Strategy, UContainerFor};

use crate::{BitAccess, BitLogic, BitPrim, BitSequence, HasBitWidth};

impl<const N: u16, S: Strategy> HasBitWidth for Bits<N, S>
where
    S: UContainerFor<N>,
{
    const WIDTH: USize = USize(N as usize);
}

impl<const N: u16, S: Strategy> BitAccess for Bits<N, S>
where
    S: UContainerFor<N>,
    <S as UContainerFor<N>>::T: BitPrim,
{
    fn bit(self, idx: USize) -> Bool {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim methods take u32 indices; sealed bridge contract; tracked: #256
        Bool(self.to_raw().get_bit(idx.0 as u32))
    }
    fn with_bit_set(self, idx: USize) -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
        Self::from_raw(self.to_raw().with_bit_set(idx.0 as u32))
    }
    fn with_bit_cleared(self, idx: USize) -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
        Self::from_raw(self.to_raw().with_bit_cleared(idx.0 as u32))
    }
    fn with_bit_toggled(self, idx: USize) -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
        Self::from_raw(self.to_raw().with_bit_toggled(idx.0 as u32))
    }
}

impl<const N: u16, S: Strategy> BitSequence for Bits<N, S>
where
    S: UContainerFor<N>,
    <S as UContainerFor<N>>::T: BitPrim,
{
    fn trailing_zeros(self) -> USize {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim returns u32 counts; tracked: #256
        USize(self.to_raw().trailing_zeros() as usize)
    }
    fn leading_zeros(self) -> USize {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim returns u32 counts; tracked: #256
        let lz = self.to_raw().leading_zeros() as usize;
        let container_width = <<S as UContainerFor<N>>::T as BitPrim>::WIDTH as usize;
        USize(lz.saturating_sub(container_width - N as usize))
    }
    fn count_ones(self) -> USize {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim returns u32 counts; tracked: #256
        USize(self.to_raw().count_ones() as usize)
    }
    fn count_zeros(self) -> USize {
        USize(N as usize - self.count_ones().0)
    }
    fn is_zero(self) -> Bool {
        Bool(self.to_raw() == <<S as UContainerFor<N>>::T as BitPrim>::ZERO)
    }
}

impl<const N: u16> BitLogic for Bits<N, Hot>
where
    Hot: UContainerFor<N>,
    <Hot as UContainerFor<N>>::T: BitPrim
        + core::ops::BitOr<Output = <Hot as UContainerFor<N>>::T>
        + core::ops::BitAnd<Output = <Hot as UContainerFor<N>>::T>
        + core::ops::BitXor<Output = <Hot as UContainerFor<N>>::T>
        + core::ops::Not<Output = <Hot as UContainerFor<N>>::T>,
{
    fn bitor(self, other: Self) -> Self {
        Self::from_raw(self.to_raw() | other.to_raw())
    }
    fn bitand(self, other: Self) -> Self {
        Self::from_raw(self.to_raw() & other.to_raw())
    }
    fn bitnot(self) -> Self {
        Self::from_raw(!self.to_raw())
    }
    fn bitxor(self, other: Self) -> Self {
        Self::from_raw(self.to_raw() ^ other.to_raw())
    }
}
