//! `HasBitWidth` / `BitAccess` / `BitSequence` impls on `UFixed<I, F, S>`.
//!
//! The bound `S: UBitContainer<{ ufixed_bits(I, F) }>` pulls in
//! `BitsContainerFor<BITS, Unsigned>` and the `BitPrim` requirement on the
//! container in a single predicate — avoids the const-expr cycle
//! that two separate predicates would trigger.

use arvo_bits_contracts::{BitAccess, BitLogic, BitPrim, BitSequence, HasBitWidth, UBitContainer};
use arvo_storage::{Bool, FBits, IBits, USize};
use arvo_strategy::{Hot, Strategy};
use arvo_transparent::Transparent;

use crate::strategy::ufixed_bits;
use crate::ufixed::UFixed;

impl<const I: IBits, const F: FBits, S: Strategy> HasBitWidth for UFixed<I, F, S>
where
    S: UBitContainer<{ ufixed_bits(I, F) }>,
{
    const WIDTH: USize = USize(I.raw() as usize + F.raw() as usize);
}

impl<const I: IBits, const F: FBits, S: Strategy> BitAccess for UFixed<I, F, S>
where
    S: UBitContainer<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn bit(self, idx: USize) -> Bool {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::get_bit(prim, idx)
    }

    #[inline(always)]
    fn with_bit_set(self, idx: USize) -> Self {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out =
            <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::with_bit_set(prim, idx);
        Self::from_raw(<S as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(out))
    }

    #[inline(always)]
    fn with_bit_cleared(self, idx: USize) -> Self {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out = <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::with_bit_cleared(
            prim, idx,
        );
        Self::from_raw(<S as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(out))
    }

    #[inline(always)]
    fn with_bit_toggled(self, idx: USize) -> Self {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out = <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::with_bit_toggled(
            prim, idx,
        );
        Self::from_raw(<S as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(out))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> BitSequence for UFixed<I, F, S>
where
    S: UBitContainer<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn trailing_zeros(self) -> USize {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::trailing_zeros(prim)
    }

    #[inline(always)]
    fn leading_zeros(self) -> USize {
        // Container may be wider than the logical width under Warm /
        // Precise. Report logical leading zeros by subtracting the
        // container's surplus bits.
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let container_lz =
            <USize as Transparent>::raw(
                <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::leading_zeros(prim),
            );
        let logical = I.raw() as usize + F.raw() as usize;
        let container = <USize as Transparent>::raw(
            <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::WIDTH,
        );
        let surplus = container - logical;
        USize(container_lz.saturating_sub(surplus))
    }

    #[inline(always)]
    fn count_ones(self) -> USize {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::count_ones(prim)
    }

    #[inline(always)]
    fn count_zeros(self) -> USize {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let ones = <USize as Transparent>::raw(
            <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::count_ones(prim),
        );
        let logical = I.raw() as usize + F.raw() as usize;
        USize(logical - ones)
    }

    #[inline(always)]
    fn is_zero(self) -> Bool {
        let prim = <S as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        <<S as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::is_zero(prim)
    }
}

impl<const I: IBits, const F: FBits> BitLogic for UFixed<I, F, Hot>
where
    Hot: UBitContainer<{ ufixed_bits(I, F) }>,
{
    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        let a = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let b = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(other.to_raw());
        let out = <<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::bitor(a, b);
        Self::from_raw(<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(
            out,
        ))
    }

    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        let a = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let b = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(other.to_raw());
        let out = <<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::bitand(a, b);
        Self::from_raw(<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(
            out,
        ))
    }

    #[inline(always)]
    fn bitnot(self) -> Self {
        let a = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out = <<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::bitnot(a);
        Self::from_raw(<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(
            out,
        ))
    }

    #[inline(always)]
    fn bitxor(self, other: Self) -> Self {
        let a = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let b = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(other.to_raw());
        let out = <<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::bitxor(a, b);
        Self::from_raw(<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(
            out,
        ))
    }

    #[inline(always)]
    fn clear_lowest_set_bit(self) -> Self
    where
        Self: BitAccess + BitSequence,
    {
        let a = <Hot as UBitContainer<{ ufixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out =
            <<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::Prim as BitPrim>::clear_lowest_set_bit(
                a,
            );
        Self::from_raw(<Hot as UBitContainer<{ ufixed_bits(I, F) }>>::from_prim(
            out,
        ))
    }
}
