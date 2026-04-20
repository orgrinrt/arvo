//! Sealed primitive bit-op bridges.
//!
//! `BitPrim` and `IBitPrim` are internal helper traits that expose a
//! uniform bit-manipulation surface across Rust's fixed-width integer
//! primitives. The concrete `impl BitWidth / BitAccess / BitSequence
//! for UFixed<I, F, S>` (and the `IFixed` mirror) delegates through
//! these bridges: `UFixed::to_raw()` returns the container primitive,
//! the `BitPrim` method runs, and `UFixed::from_raw(...)` re-wraps.
//!
//! Sealed via a private supertrait — consumers cannot add new
//! primitives. Out-of-range `idx` is non-panicking: `get_bit` returns
//! `false` and the three `with_bit_*` mutators leave the value
//! unchanged.

mod sealed {
    /// Private supertrait. Implemented only inside this module.
    pub trait Bit {}
    pub trait IBit {}
    /// Sealing trait for the `UBitContainer` bridge. Implemented only
    /// on `(Strategy, BITS)` pairs whose unsigned container is a
    /// `BitPrim`. Const-generic so each `(S, BITS)` pair has its own
    /// sealing impl.
    pub trait UBridge<const BITS: u8> {}
}

mod sealed_i {
    /// Sealing trait for the `IBitContainer` bridge. Separate module
    /// from `sealed` so the same Strategy can be both a U-bridge and
    /// an I-bridge at the same BITS without blanket-impl collision.
    pub trait IBridge<const BITS: u8> {}
}

/// Unsigned primitive bit bridge.
///
/// Implemented for `u8` / `u16` / `u32` / `u64`. Used by the concrete
/// `UFixed` impls in `ufixed_impl.rs`.
pub trait BitPrim: sealed::Bit + Copy + 'static {
    /// Bit width of this primitive (8, 16, 32, or 64).
    const WIDTH: u8;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    /// Count set bits.
    fn count_ones(self) -> u32;
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> u32;
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> u32;

    /// Read bit `idx`. Returns `false` for `idx >= WIDTH`.
    fn get_bit(self, idx: u32) -> bool;
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: u32) -> Self;
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: u32) -> Self;
    /// Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_toggled(self, idx: u32) -> Self;

    /// Whole-word OR.
    fn bitor(self, other: Self) -> Self;
    /// Whole-word AND.
    fn bitand(self, other: Self) -> Self;
    /// Whole-word NOT.
    fn bitnot(self) -> Self;
    /// Whole-word XOR.
    fn bitxor(self, other: Self) -> Self;
    /// Clear the lowest set bit. `self & (self.wrapping_sub(1))`.
    fn clear_lowest_set_bit(self) -> Self;
}

/// Signed primitive bit bridge.
///
/// Implemented for `i8` / `i16` / `i32` / `i64`. Bit operations
/// reinterpret the bits through the corresponding unsigned type so
/// signed-shift semantics do not leak in.
pub trait IBitPrim: sealed::IBit + Copy + 'static {
    /// Bit width of this primitive (8, 16, 32, or 64).
    const WIDTH: u8;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    /// Count set bits.
    fn count_ones(self) -> u32;
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> u32;
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> u32;

    /// Read bit `idx`. Returns `false` for `idx >= WIDTH`.
    fn get_bit(self, idx: u32) -> bool;
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: u32) -> Self;
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: u32) -> Self;
    /// Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_toggled(self, idx: u32) -> Self;
}

// --- Unsigned impls --------------------------------------------------------

macro_rules! impl_bit_prim_u {
    ($ty:ty, $width:literal) => {
        impl sealed::Bit for $ty {}

        impl BitPrim for $ty {
            const WIDTH: u8 = $width;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn count_ones(self) -> u32 {
                <$ty>::count_ones(self)
            }

            #[inline(always)]
            fn trailing_zeros(self) -> u32 {
                <$ty>::trailing_zeros(self)
            }

            #[inline(always)]
            fn leading_zeros(self) -> u32 {
                <$ty>::leading_zeros(self)
            }

            #[inline(always)]
            fn get_bit(self, idx: u32) -> bool {
                if idx >= $width {
                    return false;
                }
                (self >> idx) & 1 == 1
            }

            #[inline(always)]
            fn with_bit_set(self, idx: u32) -> Self {
                if idx >= $width {
                    return self;
                }
                self | (1 as $ty) << idx
            }

            #[inline(always)]
            fn with_bit_cleared(self, idx: u32) -> Self {
                if idx >= $width {
                    return self;
                }
                self & !((1 as $ty) << idx)
            }

            #[inline(always)]
            fn with_bit_toggled(self, idx: u32) -> Self {
                if idx >= $width {
                    return self;
                }
                self ^ (1 as $ty) << idx
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
        }
    };
}

impl_bit_prim_u!(u8, 8);
impl_bit_prim_u!(u16, 16);
impl_bit_prim_u!(u32, 32);
impl_bit_prim_u!(u64, 64);

// --- Signed impls ----------------------------------------------------------
//
// Reinterpret through the corresponding unsigned type for every bit
// operation. Signed shifts carry sign-extension semantics we don't
// want at the bit level.

macro_rules! impl_bit_prim_i {
    ($ity:ty, $uty:ty, $width:literal) => {
        impl sealed::IBit for $ity {}

        impl IBitPrim for $ity {
            const WIDTH: u8 = $width;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn count_ones(self) -> u32 {
                <$ity>::count_ones(self)
            }

            #[inline(always)]
            fn trailing_zeros(self) -> u32 {
                <$ity>::trailing_zeros(self)
            }

            #[inline(always)]
            fn leading_zeros(self) -> u32 {
                <$ity>::leading_zeros(self)
            }

            #[inline(always)]
            fn get_bit(self, idx: u32) -> bool {
                if idx >= $width {
                    return false;
                }
                ((self as $uty) >> idx) & 1 == 1
            }

            #[inline(always)]
            fn with_bit_set(self, idx: u32) -> Self {
                if idx >= $width {
                    return self;
                }
                ((self as $uty) | (1 as $uty) << idx) as $ity
            }

            #[inline(always)]
            fn with_bit_cleared(self, idx: u32) -> Self {
                if idx >= $width {
                    return self;
                }
                ((self as $uty) & !((1 as $uty) << idx)) as $ity
            }

            #[inline(always)]
            fn with_bit_toggled(self, idx: u32) -> Self {
                if idx >= $width {
                    return self;
                }
                ((self as $uty) ^ (1 as $uty) << idx) as $ity
            }
        }
    };
}

impl_bit_prim_i!(i8, u8, 8);
impl_bit_prim_i!(i16, u16, 16);
impl_bit_prim_i!(i32, u32, 32);
impl_bit_prim_i!(i64, u64, 64);

// --- Bridge traits ---------------------------------------------------------
//
// `generic_const_exprs` trips on a cycle when the same anonymous
// const-expr appears in multiple where-clause predicates on the same
// impl block (e.g. `S: UContainerFor<{K}>` alongside
// `<S as UContainerFor<{K}>>::T: BitPrim`). The bridge traits below
// collapse both requirements into a single predicate per impl block.
// Same sealed-trait-table pattern L0's `UContainerFor` uses, one
// indirection up.

use arvo::strategy::{IContainerFor, Strategy, UContainerFor};

/// Sealed bridge: `(S, BITS)` where `S: UContainerFor<BITS>` **and**
/// the container type is `BitPrim`. Collapses the two predicates into
/// one to sidestep the const-expr cycle.
pub trait UBitContainer<const BITS: u8>: sealed::UBridge<BITS> + UContainerFor<BITS> {
    /// The container primitive for this `(S, BITS)` pair.
    type Prim: BitPrim;
    /// Coerce the strategy-selected container into the bridge's
    /// primitive type. Identity at runtime — same underlying integer
    /// type; the coercion exists only to route `UContainerFor::T`
    /// values into `BitPrim` methods inside generic contexts.
    fn to_prim(t: <Self as UContainerFor<BITS>>::T) -> Self::Prim;
    /// Reverse of `to_prim`.
    fn from_prim(p: Self::Prim) -> <Self as UContainerFor<BITS>>::T;
}

/// Signed counterpart of `UBitContainer`.
pub trait IBitContainer<const BITS: u8>: sealed_i::IBridge<BITS> + IContainerFor<BITS> {
    /// The container primitive for this `(S, BITS)` pair.
    type Prim: IBitPrim;
    /// See `UBitContainer::to_prim`.
    fn to_prim(t: <Self as IContainerFor<BITS>>::T) -> Self::Prim;
    /// Reverse of `to_prim`.
    fn from_prim(p: Self::Prim) -> <Self as IContainerFor<BITS>>::T;
}

// Blanket impl: every strategy that picks a `BitPrim` container at a
// given BITS gets the bridge for free. The `sealed::Bridge` bound on
// the trait keeps downstream code from implementing the bridge, while
// the blanket below keeps the intended `(S, BITS)` pairs covered.
//
// The blanket uses identity coercion because `<S as UContainerFor<BITS>>::T`
// is exactly the primitive type when the `: BitPrim` bound holds —
// so the associated `Prim` type can be set to the container type.

impl<S, const BITS: u8> sealed::UBridge<BITS> for S
where
    S: Strategy,
    S: UContainerFor<BITS>,
    <S as UContainerFor<BITS>>::T: BitPrim,
{
}

impl<S, const BITS: u8> UBitContainer<BITS> for S
where
    S: Strategy,
    S: UContainerFor<BITS>,
    <S as UContainerFor<BITS>>::T: BitPrim,
{
    type Prim = <S as UContainerFor<BITS>>::T;

    #[inline(always)]
    fn to_prim(t: <Self as UContainerFor<BITS>>::T) -> Self::Prim {
        t
    }

    #[inline(always)]
    fn from_prim(p: Self::Prim) -> <Self as UContainerFor<BITS>>::T {
        p
    }
}

// Signed bridge: second blanket over Strategy + IContainerFor + IBitPrim.
// Uses its own sealing module (`sealed_i`) so one Strategy can carry
// both a U-bridge and an I-bridge at the same BITS without the two
// blankets colliding on a shared sealing impl.

impl<S, const BITS: u8> sealed_i::IBridge<BITS> for S
where
    S: Strategy,
    S: IContainerFor<BITS>,
    <S as IContainerFor<BITS>>::T: IBitPrim,
{
}

impl<S, const BITS: u8> IBitContainer<BITS> for S
where
    S: Strategy,
    S: IContainerFor<BITS>,
    <S as IContainerFor<BITS>>::T: IBitPrim,
{
    type Prim = <S as IContainerFor<BITS>>::T;

    #[inline(always)]
    fn to_prim(t: <Self as IContainerFor<BITS>>::T) -> Self::Prim {
        t
    }

    #[inline(always)]
    fn from_prim(p: Self::Prim) -> <Self as IContainerFor<BITS>>::T {
        p
    }
}
