//! `BitPrim` and `IBitPrim` impls on bare unsigned and signed primitives.
//!
//! Extracted from lib.rs in round 202605031748 (#313) to keep the trait
//! declaration file under the 500-line lint limit. Orphan rules still
//! place these impls in the same crate as the traits they implement.

use arvo_storage::{Bool, USize};
use arvo_transparent::Transparent;

use crate::{BitPrim, IBitPrim, sealed};

// --- BitPrim impls on bare unsigned primitives ----------------------------
//
// Orphan rules require the impls to live in the crate that owns the
// trait. Per-N concrete impls expand to single-instruction sequences
// at codegen.

macro_rules! impl_bit_prim_u {
    ($ty:ty, $width:literal) => {
        impl sealed::Bit for $ty {}

        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim impl on the bare primitive that the trait was designed to bridge; the bridge surface itself is fully typed (USize / Bool); body wraps each raw result at the boundary; tracked: #311
        impl const BitPrim for $ty {
            const WIDTH: USize = USize($width);
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn count_ones(self) -> USize {
                USize(<$ty>::count_ones(self) as usize)
            }

            #[inline(always)]
            fn trailing_zeros(self) -> USize {
                USize(<$ty>::trailing_zeros(self) as usize)
            }

            #[inline(always)]
            fn leading_zeros(self) -> USize {
                USize(<$ty>::leading_zeros(self) as usize)
            }

            #[inline(always)]
            fn get_bit(self, idx: USize) -> Bool {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return Bool(false);
                }
                Bool((self >> i) & 1 == 1)
            }

            #[inline(always)]
            fn with_bit_set(self, idx: USize) -> Self {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return self;
                }
                self | (1 as $ty) << i
            }

            #[inline(always)]
            fn with_bit_cleared(self, idx: USize) -> Self {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return self;
                }
                self & !((1 as $ty) << i)
            }

            #[inline(always)]
            fn with_bit_toggled(self, idx: USize) -> Self {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return self;
                }
                self ^ (1 as $ty) << i
            }

            #[inline(always)]
            fn bitor(self, other: Self) -> Self {
                self | other
            }

            #[inline(always)]
            fn bitand(self, other: Self) -> Self {
                self & other
            }

            #[inline(always)]
            fn bitnot(self) -> Self {
                !self
            }

            #[inline(always)]
            fn bitxor(self, other: Self) -> Self {
                self ^ other
            }

            #[inline(always)]
            fn clear_lowest_set_bit(self) -> Self {
                self & self.wrapping_sub(1)
            }

            #[inline(always)]
            fn is_zero(self) -> Bool {
                Bool(self == 0)
            }

            #[inline(always)]
            fn mask_low(n: USize) -> Self {
                let i = <USize as Transparent>::raw(n);
                if i == 0 {
                    0
                } else if i >= $width {
                    <$ty>::MAX
                } else {
                    ((1 as $ty) << i) - 1
                }
            }
        }
    };
}

impl_bit_prim_u!(u8, 8);
impl_bit_prim_u!(u16, 16);
impl_bit_prim_u!(u32, 32);
impl_bit_prim_u!(u64, 64);
// Round 202604281000 Pass D: u128 BitPrim impl required by Precise
// 33..=64 promotion to u128 container. WIDTH at 128 saturates at the
// u8-typed const but matches the primitive's bit count.
impl_bit_prim_u!(u128, 128);

// --- IBitPrim impls on bare signed primitives -----------------------------
//
// Reinterpret through the corresponding unsigned type for every bit
// operation. Signed shifts carry sign-extension semantics we don't
// want at the bit level.

macro_rules! impl_bit_prim_i {
    ($ity:ty, $uty:ty, $width:literal) => {
        impl sealed::IBit for $ity {}

        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: IBitPrim impl on the bare primitive that the trait was designed to bridge; the bridge surface itself is fully typed (USize / Bool); body wraps each raw result at the boundary; tracked: #311
        impl const IBitPrim for $ity {
            const WIDTH: USize = USize($width);
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn count_ones(self) -> USize {
                USize(<$ity>::count_ones(self) as usize)
            }

            #[inline(always)]
            fn trailing_zeros(self) -> USize {
                USize(<$ity>::trailing_zeros(self) as usize)
            }

            #[inline(always)]
            fn leading_zeros(self) -> USize {
                USize(<$ity>::leading_zeros(self) as usize)
            }

            #[inline(always)]
            fn get_bit(self, idx: USize) -> Bool {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return Bool(false);
                }
                Bool(((self as $uty) >> i) & 1 == 1)
            }

            #[inline(always)]
            fn with_bit_set(self, idx: USize) -> Self {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return self;
                }
                ((self as $uty) | (1 as $uty) << i) as $ity
            }

            #[inline(always)]
            fn with_bit_cleared(self, idx: USize) -> Self {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return self;
                }
                ((self as $uty) & !((1 as $uty) << i)) as $ity
            }

            #[inline(always)]
            fn with_bit_toggled(self, idx: USize) -> Self {
                let i = <USize as Transparent>::raw(idx);
                if i >= $width {
                    return self;
                }
                ((self as $uty) ^ (1 as $uty) << i) as $ity
            }

            #[inline(always)]
            fn bitor(self, other: Self) -> Self {
                ((self as $uty) | (other as $uty)) as $ity
            }

            #[inline(always)]
            fn bitand(self, other: Self) -> Self {
                ((self as $uty) & (other as $uty)) as $ity
            }

            #[inline(always)]
            fn bitnot(self) -> Self {
                (!(self as $uty)) as $ity
            }

            #[inline(always)]
            fn bitxor(self, other: Self) -> Self {
                ((self as $uty) ^ (other as $uty)) as $ity
            }

            #[inline(always)]
            fn clear_lowest_set_bit(self) -> Self {
                let u = self as $uty;
                (u & u.wrapping_sub(1)) as $ity
            }

            #[inline(always)]
            fn is_zero(self) -> Bool {
                Bool(self == 0)
            }

            #[inline(always)]
            fn mask_low(n: USize) -> Self {
                let i = <USize as Transparent>::raw(n);
                if i == 0 {
                    0
                } else if i >= $width {
                    <$uty>::MAX as $ity
                } else {
                    (((1 as $uty) << i) - 1) as $ity
                }
            }
        }
    };
}

impl_bit_prim_i!(i8, u8, 8);
impl_bit_prim_i!(i16, u16, 16);
impl_bit_prim_i!(i32, u32, 32);
impl_bit_prim_i!(i64, u64, 64);
// Round 202604281000 Pass D: i128 IBitPrim impl required by Precise
// IFixed BITS=33..=64 promotion to i128 container.
impl_bit_prim_i!(i128, u128, 128);
