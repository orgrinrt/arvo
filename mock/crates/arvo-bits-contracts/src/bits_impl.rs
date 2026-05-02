//! `HasBitWidth` / `BitAccess` / `BitSequence` / `BitLogic` impls on
//! the L0 `arvo_storage::Bits<N, S>` storage primitive.
//!
//! Per round 202604271346 D-12, the trait declarations live in this
//! crate (`arvo-bits-contracts`); the blanket impls on `Bits<N, S>`
//! land here too because orphan rules require trait + foreign-type
//! impls to share a crate (`Bits` is in `arvo-storage`).

use arvo_storage::{Bits, Bool, USize};
use arvo_strategy::{Hot, Strategy, UContainerFor};
use arvo_transparent::Transparent;

use crate::{BitAccess, BitLogic, BitPrim, BitSequence, HasBitWidth};

impl<const N: u16, S: Strategy> const HasBitWidth for Bits<N, S>
where
    S: [const] UContainerFor<N>,
{
    const WIDTH: USize = USize(N as usize);
}

impl<const N: u16, S: Strategy> const BitAccess for Bits<N, S>
where
    S: [const] UContainerFor<N>,
    <S as UContainerFor<N>>::T: [const] BitPrim,
{
    fn bit(self, idx: USize) -> Bool {
        self.to_raw().get_bit(idx)
    }
    fn with_bit_set(self, idx: USize) -> Self {
        Self::from_raw(self.to_raw().with_bit_set(idx))
    }
    fn with_bit_cleared(self, idx: USize) -> Self {
        Self::from_raw(self.to_raw().with_bit_cleared(idx))
    }
    fn with_bit_toggled(self, idx: USize) -> Self {
        Self::from_raw(self.to_raw().with_bit_toggled(idx))
    }
}

impl<const N: u16, S: Strategy> const BitSequence for Bits<N, S>
where
    S: [const] UContainerFor<N>,
    <S as UContainerFor<N>>::T: [const] BitPrim,
{
    fn trailing_zeros(self) -> USize {
        self.to_raw().trailing_zeros()
    }
    fn leading_zeros(self) -> USize {
        // The container may be wider than the logical width N; the
        // raw `leading_zeros` includes the container's surplus bits
        // above N, which the contract subtracts back out.
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bare N here is the const-generic bit-width parameter still typed `u16` until Round 2 (#312); the saturating_sub composes through bare usize for one expression to bridge container width - N
        let lz = <USize as Transparent>::raw(self.to_raw().leading_zeros());
        let container_width =
            <USize as Transparent>::raw(<<S as UContainerFor<N>>::T as BitPrim>::WIDTH);
        USize(lz.saturating_sub(container_width - N as usize))
    }
    fn count_ones(self) -> USize {
        self.to_raw().count_ones()
    }
    fn count_zeros(self) -> USize {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bare N is the const-generic bit-width parameter still typed `u16` until Round 2 (#312)
        USize(N as usize) - self.count_ones()
    }
    fn is_zero(self) -> Bool {
        // Routes through the const-stable BitPrim::is_zero bridge instead
        // of generic-context PartialEq (which is not stable as const).
        <<S as UContainerFor<N>>::T as BitPrim>::is_zero(self.to_raw())
    }
}

impl<const N: u16> const BitLogic for Bits<N, Hot>
where
    Hot: [const] UContainerFor<N>,
    <Hot as UContainerFor<N>>::T: [const] BitPrim
        + [const] core::ops::BitOr<Output = <Hot as UContainerFor<N>>::T>
        + [const] core::ops::BitAnd<Output = <Hot as UContainerFor<N>>::T>
        + [const] core::ops::BitXor<Output = <Hot as UContainerFor<N>>::T>
        + [const] core::ops::Not<Output = <Hot as UContainerFor<N>>::T>,
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
