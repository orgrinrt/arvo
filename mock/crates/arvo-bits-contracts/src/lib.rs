#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]

//! arvo-bits-contracts. Bit-level trait declarations.
//!
//! `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`,
//! `IBitPrim`, `UBitContainer`, `IBitContainer`. All `pub const
//! trait` per round 202604271346 D-12.
//!
//! `BitPrim` / `IBitPrim` are sealed primitive-bridge traits. Their
//! impls on `u8`/`u16`/`u32`/`u64` and `i8`/`i16`/`i32`/`i64` live
//! in this crate too (orphan rules require trait + foreign-type
//! impls to share a crate). The `UBitContainer` / `IBitContainer`
//! blanket impls over `(Strategy, BITS)` also live here for the
//! same reason.
//!
//! Concrete impls of `HasBitWidth` / `BitAccess` / `BitSequence` /
//! `BitLogic` on `arvo-storage::Bits<N, S>`, `UFixed<I, F, S>`, and
//! `IFixed<I, F, S>` live in `arvo-bits` (arvo-owned types; no
//! orphan issue).

use arvo_storage::{Bool, USize};
use arvo_strategy::{IContainerFor, Strategy, UContainerFor};

mod bits_impl;

mod sealed {
    /// Private supertrait gating `BitPrim`. Impl'd in this crate on
    /// `u8`/`u16`/`u32`/`u64`. Consumers outside this crate cannot
    /// add new primitive widths.
    pub(crate) trait Bit {}
    /// Private supertrait gating `IBitPrim`. Impl'd on
    /// `i8`/`i16`/`i32`/`i64`. Separate from `Bit` so the same width
    /// (e.g. `u8` vs `i8`) has independent sealing.
    pub(crate) trait IBit {}
    /// Sealing trait for the `UBitContainer` bridge. Const-generic so
    /// each `(S, BITS)` pair has its own sealing impl.
    pub(crate) trait UBridge<const BITS: u8> {}
    /// Sealing trait for the `IBitContainer` bridge. Separate module
    /// from `Bit` so the same Strategy can be both a U-bridge and an
    /// I-bridge at the same BITS without blanket-impl collision.
    pub(crate) trait IBridge<const BITS: u8> {}
}

/// Logical bit width at the type level.
///
/// For `UFixed<I, F, S>` this is `I + F`; for `IFixed<I, F, S>` it is
/// `1 + I + F` (the sign bit counts). The width is the logical bit
/// count, not the container size: the container may be wider under
/// `Warm` / `Precise` strategies.
pub const trait HasBitWidth {
    /// Logical bit width.
    const WIDTH: USize;
}

/// Individual bit read / write.
///
/// All mutators take `self` and return `Self` (functional style, no
/// interior mutation). `idx` is LSB-first (bit 0 is least significant).
/// Indices `>= WIDTH` do not panic: `bit` returns `Bool::FALSE` and
/// the three `with_bit_*` mutators return `self` unchanged.
pub const trait BitAccess: HasBitWidth + Copy {
    /// Read bit at position `idx`.
    fn bit(self, idx: USize) -> Bool;
    /// Produce a copy with bit `idx` set to 1.
    fn with_bit_set(self, idx: USize) -> Self;
    /// Produce a copy with bit `idx` cleared to 0.
    fn with_bit_cleared(self, idx: USize) -> Self;
    /// Produce a copy with bit `idx` flipped.
    fn with_bit_toggled(self, idx: USize) -> Self;
}

/// Bulk bit-scanning / popcount.
///
/// Maps to hardware intrinsics on common targets: `count_ones` to
/// `popcnt`, `trailing_zeros` to `cttz` / `tzcnt`, `leading_zeros`
/// to `ctlz` / `lzcnt`. Operates on the raw container bits; the
/// logical-width contract is implicit in the types' construction.
pub const trait BitSequence: HasBitWidth + Copy {
    /// Count trailing (LSB) zero bits.
    fn trailing_zeros(self) -> USize;
    /// Count leading (MSB) zero bits.
    fn leading_zeros(self) -> USize;
    /// Count set bits.
    fn count_ones(self) -> USize;
    /// Count cleared bits.
    fn count_zeros(self) -> USize;
    /// `Bool::TRUE` when every bit is zero.
    fn is_zero(self) -> Bool;
}

/// Whole-word bitwise logic.
///
/// Single-instruction on the backing primitive (x86 `or` / `and` /
/// `xor` / `not`; identical on aarch64). Loop-free. The three
/// bit-level contracts partition: `BitAccess` covers SINGLE-BIT
/// read/write; `BitSequence` covers SCAN / POPCOUNT; `BitLogic`
/// covers WHOLE-WORD logical ops. Together they describe a
/// bit-bearing container.
///
/// Hot-only surface. Other strategies (`Warm`, `Precise`, `Cold`)
/// have wider containers that make whole-word ops meaningless at
/// the logical-width level (NOT would flip surplus container bits).
pub const trait BitLogic: HasBitWidth + Copy {
    /// Whole-word OR.
    fn bitor(self, other: Self) -> Self;
    /// Whole-word AND.
    fn bitand(self, other: Self) -> Self;
    /// Whole-word NOT.
    fn bitnot(self) -> Self;
    /// Whole-word XOR.
    fn bitxor(self, other: Self) -> Self;

    /// Clear the lowest set bit.
    ///
    /// Default impl uses `BitSequence::trailing_zeros` +
    /// `BitAccess::with_bit_cleared`. Concrete types can override with
    /// `self.to_raw() & (self.to_raw() - 1)` for the single-instruction
    /// fast path (x86 `BLSR`). Returns `self` unchanged when the word
    /// is already zero.
    #[inline]
    fn clear_lowest_set_bit(self) -> Self
    where
        Self: [const] BitAccess + [const] BitSequence,
    {
        if <Self as BitSequence>::is_zero(self).0 {
            return self;
        }
        let idx = <Self as BitSequence>::trailing_zeros(self);
        <Self as BitAccess>::with_bit_cleared(self, idx)
    }
}

/// Sealed unsigned primitive bit bridge.
///
/// Implemented for `u8` / `u16` / `u32` / `u64` (in this crate, by
/// orphan rules). Used by the concrete `UFixed` / `Bits` impls of
/// `BitAccess` / `BitSequence` / `BitLogic` in `arvo-bits`.
pub trait BitPrim: sealed::Bit + Copy + 'static {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim is a bare-primitive bridge by definition; tracked: #256
    /// Bit width of this primitive (8, 16, 32, or 64).
    const WIDTH: u8;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim primitive-bridge surface; tracked: #256
    /// Count set bits.
    fn count_ones(self) -> u32;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> u32;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> u32;

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Read bit `idx`. Returns `false` for `idx >= WIDTH`.
    fn get_bit(self, idx: u32) -> bool;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: u32) -> Self;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: u32) -> Self;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
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

/// Sealed signed primitive bit bridge.
///
/// Implemented for `i8` / `i16` / `i32` / `i64`. Bit operations
/// reinterpret the bits through the corresponding unsigned type so
/// signed-shift semantics do not leak in.
pub trait IBitPrim: sealed::IBit + Copy + 'static {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: IBitPrim is a bare-primitive bridge; tracked: #256
    /// Bit width of this primitive (8, 16, 32, or 64).
    const WIDTH: u8;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Count set bits.
    fn count_ones(self) -> u32;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> u32;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> u32;

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Read bit `idx`. Returns `false` for `idx >= WIDTH`.
    fn get_bit(self, idx: u32) -> bool;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: u32) -> Self;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: u32) -> Self;
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: same; tracked: #256
    /// Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_toggled(self, idx: u32) -> Self;
}

// --- BitPrim impls on bare unsigned primitives ----------------------------
//
// Orphan rules require the impls to live in the crate that owns the
// trait. Per-N concrete impls expand to single-instruction sequences
// at codegen.

macro_rules! impl_bit_prim_u {
    ($ty:ty, $width:literal) => {
        impl sealed::Bit for $ty {}

        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: BitPrim impl on the bare primitive that the trait was designed to bridge; tracked: #256
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

// --- IBitPrim impls on bare signed primitives -----------------------------
//
// Reinterpret through the corresponding unsigned type for every bit
// operation. Signed shifts carry sign-extension semantics we don't
// want at the bit level.

macro_rules! impl_bit_prim_i {
    ($ity:ty, $uty:ty, $width:literal) => {
        impl sealed::IBit for $ity {}

        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: IBitPrim impl on the bare primitive that the trait was designed to bridge; tracked: #256
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

// --- Container bridges ----------------------------------------------------
//
// `generic_const_exprs` trips on a cycle when the same anonymous
// const-expr appears in multiple where-clause predicates on the same
// impl block (e.g. `S: UContainerFor<{K}>` alongside
// `<S as UContainerFor<{K}>>::T: BitPrim`). The bridge traits below
// collapse both requirements into a single predicate per impl block.

/// Sealed bridge: `(S, BITS)` where `S: UContainerFor<BITS>` **and**
/// the container type is `BitPrim`. Collapses the two predicates into
/// one to sidestep the const-expr cycle.
pub trait UBitContainer<const BITS: u8>: sealed::UBridge<BITS> + UContainerFor<BITS> {
    /// The container primitive for this `(S, BITS)` pair.
    type Prim: BitPrim;
    /// Coerce the strategy-selected container into the bridge's
    /// primitive type. Identity at runtime: same underlying integer
    /// type; the coercion exists only to route `UContainerFor::T`
    /// values into `BitPrim` methods inside generic contexts.
    fn to_prim(t: <Self as UContainerFor<BITS>>::T) -> Self::Prim;
    /// Reverse of `to_prim`.
    fn from_prim(p: Self::Prim) -> <Self as UContainerFor<BITS>>::T;
}

/// Signed counterpart of `UBitContainer`.
pub trait IBitContainer<const BITS: u8>: sealed::IBridge<BITS> + IContainerFor<BITS> {
    /// The container primitive for this `(S, BITS)` pair.
    type Prim: IBitPrim;
    /// See `UBitContainer::to_prim`.
    fn to_prim(t: <Self as IContainerFor<BITS>>::T) -> Self::Prim;
    /// Reverse of `to_prim`.
    fn from_prim(p: Self::Prim) -> <Self as IContainerFor<BITS>>::T;
}

// Blanket impl: every strategy that picks a `BitPrim` container at a
// given BITS gets the bridge for free. The `sealed::UBridge` bound on
// the trait keeps downstream code from implementing the bridge, while
// the blanket below covers the intended `(S, BITS)` pairs.
//
// The blanket uses identity coercion because `<S as UContainerFor<BITS>>::T`
// is exactly the primitive type when the `: BitPrim` bound holds. The
// associated `Prim` type can be set to the container type.

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

// Signed bridge: blanket over Strategy + IContainerFor + IBitPrim.
// Uses its own sealing trait (`sealed::IBridge`) so one Strategy can
// carry both a U-bridge and an I-bridge at the same BITS without the
// two blankets colliding on a shared sealing impl.

impl<S, const BITS: u8> sealed::IBridge<BITS> for S
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

// --- Narrow ----------------------------------------------------------------
//
// Moved here from the previously-separate `arvo-narrow-contracts` crate
// during round 202604280034. The merge is justified because every consumer
// of `Narrow` already depends on `BitLogic` (declared above) for the
// default-body composition; the single-trait crate split was ceremony.

/// Narrowed-result expression alias.
///
/// Cosmetic alias for the result of `narrow_to::<N>` so consumer
/// signatures read `fn foo() -> Narrowed<13, u16>` instead of bare
/// `u16`. The const-generic `N` is documentation only at the type
/// level (the value has been narrowed to `N` bits but the carrier
/// is still `T`); the precise shape (whether it gains an associated
/// type, becomes a const-bounded newtype, or stays a transparent
/// alias) is open per the doc CL's DOC-iteration carve-out. The
/// transparent form below is the cleanest landing for round
/// 202604271346; future rounds may tighten if a concrete need
/// surfaces.
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic N type carrier deferred to follow-up round; Width newtype targeted in next iteration; tracked: #256
pub type Narrowed<const N: u8, T> = T;

/// Narrow `Self` to the lowest `N` bits as type `T`.
///
/// `Self` is a wider raw value (a bare primitive or a `Bits<M, S>`
/// with `M > N`). `T` is the target container type. The default
/// body composes `Mask<W>::mask_for_width(N)` from
/// `arvo-mask-contracts` with `BitLogic::and` from this crate, then
/// the `as` operator (sound under the mask precondition) to cast to
/// the target primitive type.
///
/// Per topic Q-C, the trait declaration lives here (post-round-
/// 202604280034 merge) and the concrete impls live in `arvo-bitmask`
/// (mask-side) and `arvo-narrow` (cross-primitive). Const trait so
/// consumers can call `wide.narrow_to::<13>()` in const fn bodies
/// under generic `Narrow<T>` bounds.
pub const trait Narrow<T> {
    /// Truncate to the lowest `N` bits and return as `T`.
    fn narrow_to<const N: u8>(self) -> T
    where
        Self: Sized;

    /// Cast to `T` without masking the high bits.
    ///
    /// Sound only when the caller knows the high bits above `N` are
    /// already zero (chained narrow, just-shifted-down bitfield
    /// extraction, value carried through a `Bits<M, ...>` projection
    /// of equal-or-narrower width). Skips the mask op for the hot
    /// path. Calling with non-zero high bits produces silent garbage
    /// in the unmasked path.
    fn narrow_to_unmasked<const N: u8>(self) -> T
    where
        Self: Sized;
}
