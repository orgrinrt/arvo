//! `HasBitWidth` / `BitAccess` / `BitSequence` impls on `IFixed<I, F, S>`.
//!
//! Mirror of `ufixed_impl.rs` keyed on `IBitContainer` (which bundles
//! `IContainerFor<BITS>` + `IBitPrim` on the container).

use arvo_bits_contracts::{BitAccess, BitSequence, HasBitWidth, IBitContainer, IBitPrim};
use arvo_storage::{Bool, FBits, IBits, USize};
use arvo_strategy::Strategy;

use crate::ifixed::IFixed;
use crate::strategy::ifixed_bits;

impl<const I: IBits, const F: FBits, S: Strategy> HasBitWidth for IFixed<I, F, S>
where
    S: IBitContainer<{ ifixed_bits(I, F) }>,
{
    // IFixed's logical width is `1 + I + F` (sign bit counts).
    const WIDTH: USize = USize(1 + I.0 as usize + F.0 as usize);
}

impl<const I: IBits, const F: FBits, S: Strategy> BitAccess for IFixed<I, F, S>
where
    S: IBitContainer<{ ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn bit(self, idx: USize) -> Bool {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        Bool(<<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::get_bit(
            prim, idx.0 as u32,
        ))
    }

    #[inline(always)]
    fn with_bit_set(self, idx: USize) -> Self {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out = <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::with_bit_set(
            prim, idx.0 as u32,
        );
        Self::from_raw(<S as IBitContainer<{ ifixed_bits(I, F) }>>::from_prim(out))
    }

    #[inline(always)]
    fn with_bit_cleared(self, idx: USize) -> Self {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out =
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::with_bit_cleared(
                prim,
                idx.0 as u32,
            );
        Self::from_raw(<S as IBitContainer<{ ifixed_bits(I, F) }>>::from_prim(out))
    }

    #[inline(always)]
    fn with_bit_toggled(self, idx: USize) -> Self {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        let out =
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::with_bit_toggled(
                prim,
                idx.0 as u32,
            );
        Self::from_raw(<S as IBitContainer<{ ifixed_bits(I, F) }>>::from_prim(out))
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> BitSequence for IFixed<I, F, S>
where
    S: IBitContainer<{ ifixed_bits(I, F) }>,
{
    #[inline(always)]
    fn trailing_zeros(self) -> USize {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        USize(
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::trailing_zeros(prim)
                as usize,
        )
    }

    #[inline(always)]
    fn leading_zeros(self) -> USize {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        let container_lz =
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::leading_zeros(prim)
                as usize;
        let logical = 1 + I.0 as usize + F.0 as usize;
        let container =
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::WIDTH as usize;
        let surplus = container - logical;
        USize(container_lz.saturating_sub(surplus))
    }

    #[inline(always)]
    fn count_ones(self) -> USize {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        USize(
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::count_ones(prim)
                as usize,
        )
    }

    #[inline(always)]
    fn count_zeros(self) -> USize {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        let ones =
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::count_ones(prim)
                as usize;
        let logical = 1 + I.0 as usize + F.0 as usize;
        USize(logical - ones)
    }

    #[inline(always)]
    fn is_zero(self) -> Bool {
        let prim = <S as IBitContainer<{ ifixed_bits(I, F) }>>::to_prim(self.to_raw());
        Bool(
            <<S as IBitContainer<{ ifixed_bits(I, F) }>>::Prim as IBitPrim>::count_ones(prim) == 0,
        )
    }
}
