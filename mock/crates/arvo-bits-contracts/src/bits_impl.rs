//! `HasBitWidth` / `BitAccess` / `BitSequence` / `BitLogic` impls on
//! the L0 `arvo_storage::Bits<N, S, Sign>` storage primitive.
//!
//! Per round 202604271346 D-12, the trait declarations live in this
//! crate (`arvo-bits-contracts`); the blanket impls on `Bits` land
//! here too because orphan rules require trait + foreign-type impls
//! to share a crate (`Bits` is in `arvo-storage`).
//!
//! Round 202605021800 generalised these blankets over the Sign axis
//! through the `BitsBitPrim<Sign>` bridge: one impl block now covers
//! both `Bits<N, S, Unsigned>` and `Bits<N, S, Signed>`, where the
//! per-Sign primitive routing happens at the bridge layer.

use arvo_storage::{Bits, Bool, USize};
use arvo_strategy::{BitsContainerFor, Signedness, Strategy};
use arvo_transparent::Transparent;

use crate::{BitAccess, BitLogic, BitSequence, BitsBitPrim, HasBitWidth};

const impl<const N: u16, S: Strategy, Sign: Signedness> HasBitWidth for Bits<N, S, Sign>
where
    S: [const] BitsContainerFor<N, Sign>,
{
    const WIDTH: USize = USize(N as usize);
}

const impl<const N: u16, S: Strategy, Sign: Signedness> BitAccess for Bits<N, S, Sign>
where
    S: [const] BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: [const] BitsBitPrim<Sign>,
{
    fn bit(self, idx: USize) -> Bool {
        <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::get_bit(self.to_raw(), idx)
    }
    fn with_bit_set(self, idx: USize) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::with_bit_set(
                self.to_raw(),
                idx,
            ),
        )
    }
    fn with_bit_cleared(self, idx: USize) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::with_bit_cleared(
                self.to_raw(),
                idx,
            ),
        )
    }
    fn with_bit_toggled(self, idx: USize) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::with_bit_toggled(
                self.to_raw(),
                idx,
            ),
        )
    }
}

const impl<const N: u16, S: Strategy, Sign: Signedness> BitSequence for Bits<N, S, Sign>
where
    S: [const] BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: [const] BitsBitPrim<Sign>,
{
    fn trailing_zeros(self) -> USize {
        <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::trailing_zeros(self.to_raw())
    }
    fn leading_zeros(self) -> USize {
        // The container may be wider than the logical width N; the
        // raw `leading_zeros` includes the container's surplus bits
        // above N, which the contract subtracts back out.
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bare N here is the const-generic bit-width parameter still typed `u16` until Round 2 (#312); the saturating_sub composes through bare usize for one expression to bridge container width - N; tracked: #312
        let lz =
            <USize as Transparent>::raw(<<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<
                Sign,
            >>::leading_zeros(self.to_raw()));
        let container_width = <USize as Transparent>::raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::WIDTH,
        );
        USize(lz.saturating_sub(container_width - N as usize))
    }
    fn count_ones(self) -> USize {
        <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::count_ones(self.to_raw())
    }
    fn count_zeros(self) -> USize {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bare N is the const-generic bit-width parameter still typed `u16` until Round 2 (#312); tracked: #312
        USize(N as usize) - self.count_ones()
    }
    fn is_zero(self) -> Bool {
        // Routes through the const-stable BitsBitPrim::is_zero bridge
        // instead of generic-context PartialEq (which is not stable as
        // const).
        <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::is_zero(self.to_raw())
    }
}

const impl<const N: u16, S: Strategy, Sign: Signedness> BitLogic for Bits<N, S, Sign>
where
    S: [const] BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: [const] BitsBitPrim<Sign>,
{
    fn bitor(self, other: Self) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::bitor(
                self.to_raw(),
                other.to_raw(),
            ),
        )
    }
    fn bitand(self, other: Self) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::bitand(
                self.to_raw(),
                other.to_raw(),
            ),
        )
    }
    fn bitnot(self) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::bitnot(self.to_raw()),
        )
    }
    fn bitxor(self, other: Self) -> Self {
        Self::from_raw(
            <<S as BitsContainerFor<N, Sign>>::T as BitsBitPrim<Sign>>::bitxor(
                self.to_raw(),
                other.to_raw(),
            ),
        )
    }
}
