//! Canonical `BitPrim` impl for `MultiContainer<HiT, LoT>`.
//!
//! `MultiContainer<HiT, LoT>` is the multi-value storage carrier
//! defined in `arvo-strategy::multi_container` for logical bit widths
//! beyond a single native primitive (129..=256). Round 202605021800
//! lifts `BitPrim` to the multi-container so any `Bits<N, S, Sign>`
//! resolving to a `MultiContainer` carrier inherits the full bit-
//! level surface.
//!
//! Sealing: orphan rules require `sealed::Bit` to be impl'd in this
//! crate; the impl below seals MultiContainer-of-BitPrim halves.
//!
//! Methods compose half-by-half: count_ones is hi + lo,
//! trailing_zeros / leading_zeros cascade between halves, get_bit and
//! the with_bit_* mutators select half by idx, whole-word ops apply
//! element-wise, is_zero is hi.is_zero & lo.is_zero,
//! clear_lowest_set_bit clears in lo first then falls through to hi.

use arvo_storage::{Bool, USize};
use arvo_strategy::{MultiContainer, MultiContainerHalf};
use arvo_transparent::Transparent;

use crate::{BitPrim, sealed};

impl<HiT: MultiContainerHalf, LoT: MultiContainerHalf> sealed::Bit for MultiContainer<HiT, LoT> where
    HiT: BitPrim, LoT: BitPrim
{
}

// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim impl on MultiContainer composing two BitPrim halves; the bridge surface is fully typed (USize / Bool); body accesses hi/lo fields as the documented multi-container internal boundary; tracked: #311
impl<HiT: MultiContainerHalf + [const] BitPrim, LoT: MultiContainerHalf + [const] BitPrim>
    const BitPrim for MultiContainer<HiT, LoT>
{
    const WIDTH: USize = USize(
        <USize as Transparent>::raw(<HiT as BitPrim>::WIDTH)
            + <USize as Transparent>::raw(<LoT as BitPrim>::WIDTH),
    );
    const ZERO: Self =
        MultiContainer { hi: <HiT as BitPrim>::ZERO, lo: <LoT as BitPrim>::ZERO };
    const ONE: Self = MultiContainer { hi: <HiT as BitPrim>::ZERO, lo: <LoT as BitPrim>::ONE };

    #[inline(always)]
    fn count_ones(self) -> USize {
        let hi_ones =
            <USize as Transparent>::raw(<HiT as BitPrim>::count_ones(self.hi));
        let lo_ones =
            <USize as Transparent>::raw(<LoT as BitPrim>::count_ones(self.lo));
        USize(hi_ones + lo_ones)
    }

    #[inline(always)]
    fn trailing_zeros(self) -> USize {
        if !<LoT as BitPrim>::is_zero(self.lo).0 {
            return <LoT as BitPrim>::trailing_zeros(self.lo);
        }
        let lo_w = <USize as Transparent>::raw(<LoT as BitPrim>::WIDTH);
        let hi_tz = <USize as Transparent>::raw(<HiT as BitPrim>::trailing_zeros(self.hi));
        USize(lo_w + hi_tz)
    }

    #[inline(always)]
    fn leading_zeros(self) -> USize {
        if !<HiT as BitPrim>::is_zero(self.hi).0 {
            return <HiT as BitPrim>::leading_zeros(self.hi);
        }
        let hi_w = <USize as Transparent>::raw(<HiT as BitPrim>::WIDTH);
        let lo_lz = <USize as Transparent>::raw(<LoT as BitPrim>::leading_zeros(self.lo));
        USize(hi_w + lo_lz)
    }

    #[inline(always)]
    fn get_bit(self, idx: USize) -> Bool {
        let i = <USize as Transparent>::raw(idx);
        let lo_w = <USize as Transparent>::raw(<LoT as BitPrim>::WIDTH);
        if i < lo_w {
            <LoT as BitPrim>::get_bit(self.lo, idx)
        } else {
            <HiT as BitPrim>::get_bit(self.hi, USize(i - lo_w))
        }
    }

    #[inline(always)]
    fn with_bit_set(self, idx: USize) -> Self {
        let i = <USize as Transparent>::raw(idx);
        let lo_w = <USize as Transparent>::raw(<LoT as BitPrim>::WIDTH);
        if i < lo_w {
            MultiContainer { hi: self.hi, lo: <LoT as BitPrim>::with_bit_set(self.lo, idx) }
        } else {
            MultiContainer {
                hi: <HiT as BitPrim>::with_bit_set(self.hi, USize(i - lo_w)),
                lo: self.lo,
            }
        }
    }

    #[inline(always)]
    fn with_bit_cleared(self, idx: USize) -> Self {
        let i = <USize as Transparent>::raw(idx);
        let lo_w = <USize as Transparent>::raw(<LoT as BitPrim>::WIDTH);
        if i < lo_w {
            MultiContainer {
                hi: self.hi,
                lo: <LoT as BitPrim>::with_bit_cleared(self.lo, idx),
            }
        } else {
            MultiContainer {
                hi: <HiT as BitPrim>::with_bit_cleared(self.hi, USize(i - lo_w)),
                lo: self.lo,
            }
        }
    }

    #[inline(always)]
    fn with_bit_toggled(self, idx: USize) -> Self {
        let i = <USize as Transparent>::raw(idx);
        let lo_w = <USize as Transparent>::raw(<LoT as BitPrim>::WIDTH);
        if i < lo_w {
            MultiContainer {
                hi: self.hi,
                lo: <LoT as BitPrim>::with_bit_toggled(self.lo, idx),
            }
        } else {
            MultiContainer {
                hi: <HiT as BitPrim>::with_bit_toggled(self.hi, USize(i - lo_w)),
                lo: self.lo,
            }
        }
    }

    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        MultiContainer {
            hi: <HiT as BitPrim>::bitor(self.hi, other.hi),
            lo: <LoT as BitPrim>::bitor(self.lo, other.lo),
        }
    }

    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        MultiContainer {
            hi: <HiT as BitPrim>::bitand(self.hi, other.hi),
            lo: <LoT as BitPrim>::bitand(self.lo, other.lo),
        }
    }

    #[inline(always)]
    fn bitnot(self) -> Self {
        MultiContainer { hi: <HiT as BitPrim>::bitnot(self.hi), lo: <LoT as BitPrim>::bitnot(self.lo) }
    }

    #[inline(always)]
    fn bitxor(self, other: Self) -> Self {
        MultiContainer {
            hi: <HiT as BitPrim>::bitxor(self.hi, other.hi),
            lo: <LoT as BitPrim>::bitxor(self.lo, other.lo),
        }
    }

    #[inline(always)]
    fn clear_lowest_set_bit(self) -> Self {
        if !<LoT as BitPrim>::is_zero(self.lo).0 {
            MultiContainer { hi: self.hi, lo: <LoT as BitPrim>::clear_lowest_set_bit(self.lo) }
        } else {
            MultiContainer { hi: <HiT as BitPrim>::clear_lowest_set_bit(self.hi), lo: self.lo }
        }
    }

    #[inline(always)]
    fn is_zero(self) -> Bool {
        let hi_zero = <HiT as BitPrim>::is_zero(self.hi);
        let lo_zero = <LoT as BitPrim>::is_zero(self.lo);
        Bool(hi_zero.0 && lo_zero.0)
    }
}
