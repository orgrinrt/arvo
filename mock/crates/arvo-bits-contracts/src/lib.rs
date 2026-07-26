#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
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

use arvo_storage::{Bits, Bool, USize};
use arvo_strategy::{BitsContainerFor, Signed, Signedness, Strategy, Unsigned};

mod bits_impl;
mod cross_domain;
mod narrow_from;
mod prim_impls;
mod widebits_impl;
mod widen;

pub use narrow_from::NarrowFromU64;
pub use widen::{Widen, Widened};

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
    pub(crate) trait UBridge<const BITS: u16> {}
    /// Sealing trait for the `IBitContainer` bridge. Separate module
    /// from `Bit` so the same Strategy can be both a U-bridge and an
    /// I-bridge at the same BITS without blanket-impl collision.
    pub(crate) trait IBridge<const BITS: u16> {}
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
/// Implemented for `u8` / `u16` / `u32` / `u64` / `u128` (in this
/// crate, by orphan rules). Used by the concrete `UFixed` / `Bits`
/// impls of `BitAccess` / `BitSequence` / `BitLogic` in `arvo-bits`.
///
/// The trait surface is fully typed: bit counts and indices are
/// `USize`, predicates return `Bool`. The macro impls on each bare
/// primitive route through `<$ty>::BITS` / `<$ty>::count_ones` etc.
/// and wrap results in the typed surface at the boundary, so the
/// only `bool` / `u32` exposure is one wrap inside each impl body.
pub const trait BitPrim: sealed::Bit + Copy + 'static {
    /// Bit width of this primitive (8, 16, 32, 64, or 128).
    const WIDTH: USize;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    /// Count set bits.
    fn count_ones(self) -> USize;
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> USize;
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> USize;

    /// Read bit `idx`. Returns `Bool::FALSE` for `idx >= WIDTH`.
    fn get_bit(self, idx: USize) -> Bool;
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: USize) -> Self;
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: USize) -> Self;
    /// Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_toggled(self, idx: USize) -> Self;

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

    /// `Bool::TRUE` when every bit of the primitive is zero.
    ///
    /// Bridges around `core::cmp::PartialEq` not yet being const-stable
    /// on bare primitives: per-primitive impls evaluate `self == 0`
    /// directly on the concrete type (const-stable for bare integers)
    /// and wrap the result. Consumers reach the predicate through the
    /// trait projection in const-generic bodies.
    fn is_zero(self) -> Bool;

    /// Mask with the lowest `n` bits set, all higher bits cleared.
    ///
    /// Round 3 (#313) substrate helper. Saturates at `WIDTH`: if `n
    /// >= WIDTH`, returns the all-ones value. If `n == 0`, returns
    /// zero. Otherwise returns `(ONE << n) - ONE`.
    ///
    /// Consumed by mask / hash / bitfield call sites that need a
    /// low-N-bits mask. Replaces the prior free fn
    /// `arvo-strategy::width::mask_low_bits` and the `1u64 << N` /
    /// `u64::MAX` raw-shift patterns scattered across the substrate.
    fn mask_low(n: USize) -> Self;
}

/// Sealed signed primitive bit bridge.
///
/// Implemented for `i8` / `i16` / `i32` / `i64` / `i128`. Bit
/// operations reinterpret the bits through the corresponding unsigned
/// type so signed-shift semantics do not leak in.
///
/// Mirrors `BitPrim` exactly: same `USize` / `Bool` surface, same
/// six whole-word and clear-lowest-bit ops, same `is_zero`. Round
/// 202605021800 added the parity methods (six previously absent).
pub const trait IBitPrim: sealed::IBit + Copy + 'static {
    /// Bit width of this primitive (8, 16, 32, 64, or 128).
    const WIDTH: USize;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    /// Count set bits.
    fn count_ones(self) -> USize;
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> USize;
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> USize;

    /// Read bit `idx`. Returns `Bool::FALSE` for `idx >= WIDTH`.
    fn get_bit(self, idx: USize) -> Bool;
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: USize) -> Self;
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: USize) -> Self;
    /// Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_toggled(self, idx: USize) -> Self;

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

    /// `Bool::TRUE` when every bit of the primitive is zero.
    fn is_zero(self) -> Bool;

    /// Mask with the lowest `n` bits set, all higher bits cleared.
    /// See `BitPrim::mask_low` for semantics; the signed version
    /// reinterprets through the corresponding unsigned type so the
    /// shift carries no sign-extension surprise.
    fn mask_low(n: USize) -> Self;
}

// --- Sign-axis primitive bridge -------------------------------------------
//
// `BitsBitPrim<Sign>` collapses the `BitPrim` (Unsigned) / `IBitPrim`
// (Signed) dichotomy into a single Sign-keyed trait. Consumers like
// `bits_impl.rs` can then carry one bound `<S as BitsContainerFor<N,
// Sign>>::T: BitsBitPrim<Sign>` instead of branching on Sign at the
// blanket-impl level. The blanket impls below route per-method to
// the underlying BitPrim or IBitPrim trait projection.
//
// Round 202605021800 introduced this bridge to let `Bits<N, S,
// Signed>` resolve all bit-level methods (previously the blanket was
// gated on Sign=Unsigned via the `BitPrim` bound on the container,
// which IBitPrim primitives do not satisfy).

/// Sign-keyed primitive bridge.
///
/// Implemented for every primitive that satisfies `BitPrim`
/// (`Sign = Unsigned`) or `IBitPrim` (`Sign = Signed`). Methods
/// mirror the primitive bridge but route through the Sign-appropriate
/// underlying trait.
pub const trait BitsBitPrim<Sign: Signedness>: Copy + 'static {
    /// Bit width of this primitive.
    const WIDTH: USize;
    /// Zero value.
    const ZERO: Self;
    /// One value.
    const ONE: Self;

    /// Count set bits.
    fn count_ones(self) -> USize;
    /// Count trailing zero bits (LSB-first).
    fn trailing_zeros(self) -> USize;
    /// Count leading zero bits (MSB-first).
    fn leading_zeros(self) -> USize;
    /// `Bool::TRUE` when every bit is zero.
    fn is_zero(self) -> Bool;

    /// Read bit `idx`. Returns `Bool::FALSE` for `idx >= WIDTH`.
    fn get_bit(self, idx: USize) -> Bool;
    /// Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_set(self, idx: USize) -> Self;
    /// Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_cleared(self, idx: USize) -> Self;
    /// Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`.
    fn with_bit_toggled(self, idx: USize) -> Self;

    /// Whole-word OR.
    fn bitor(self, other: Self) -> Self;
    /// Whole-word AND.
    fn bitand(self, other: Self) -> Self;
    /// Whole-word NOT.
    fn bitnot(self) -> Self;
    /// Whole-word XOR.
    fn bitxor(self, other: Self) -> Self;
    /// Clear the lowest set bit.
    fn clear_lowest_set_bit(self) -> Self;
}

const impl<T: [const] BitPrim> BitsBitPrim<Unsigned> for T {
    const WIDTH: USize = <T as BitPrim>::WIDTH;
    const ZERO: Self = <T as BitPrim>::ZERO;
    const ONE: Self = <T as BitPrim>::ONE;

    #[inline(always)]
    fn count_ones(self) -> USize {
        <T as BitPrim>::count_ones(self)
    }
    #[inline(always)]
    fn trailing_zeros(self) -> USize {
        <T as BitPrim>::trailing_zeros(self)
    }
    #[inline(always)]
    fn leading_zeros(self) -> USize {
        <T as BitPrim>::leading_zeros(self)
    }
    #[inline(always)]
    fn is_zero(self) -> Bool {
        <T as BitPrim>::is_zero(self)
    }
    #[inline(always)]
    fn get_bit(self, idx: USize) -> Bool {
        <T as BitPrim>::get_bit(self, idx)
    }
    #[inline(always)]
    fn with_bit_set(self, idx: USize) -> Self {
        <T as BitPrim>::with_bit_set(self, idx)
    }
    #[inline(always)]
    fn with_bit_cleared(self, idx: USize) -> Self {
        <T as BitPrim>::with_bit_cleared(self, idx)
    }
    #[inline(always)]
    fn with_bit_toggled(self, idx: USize) -> Self {
        <T as BitPrim>::with_bit_toggled(self, idx)
    }
    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        <T as BitPrim>::bitor(self, other)
    }
    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        <T as BitPrim>::bitand(self, other)
    }
    #[inline(always)]
    fn bitnot(self) -> Self {
        <T as BitPrim>::bitnot(self)
    }
    #[inline(always)]
    fn bitxor(self, other: Self) -> Self {
        <T as BitPrim>::bitxor(self, other)
    }
    #[inline(always)]
    fn clear_lowest_set_bit(self) -> Self {
        <T as BitPrim>::clear_lowest_set_bit(self)
    }
}

const impl<T: [const] IBitPrim> BitsBitPrim<Signed> for T {
    const WIDTH: USize = <T as IBitPrim>::WIDTH;
    const ZERO: Self = <T as IBitPrim>::ZERO;
    const ONE: Self = <T as IBitPrim>::ONE;

    #[inline(always)]
    fn count_ones(self) -> USize {
        <T as IBitPrim>::count_ones(self)
    }
    #[inline(always)]
    fn trailing_zeros(self) -> USize {
        <T as IBitPrim>::trailing_zeros(self)
    }
    #[inline(always)]
    fn leading_zeros(self) -> USize {
        <T as IBitPrim>::leading_zeros(self)
    }
    #[inline(always)]
    fn is_zero(self) -> Bool {
        <T as IBitPrim>::is_zero(self)
    }
    #[inline(always)]
    fn get_bit(self, idx: USize) -> Bool {
        <T as IBitPrim>::get_bit(self, idx)
    }
    #[inline(always)]
    fn with_bit_set(self, idx: USize) -> Self {
        <T as IBitPrim>::with_bit_set(self, idx)
    }
    #[inline(always)]
    fn with_bit_cleared(self, idx: USize) -> Self {
        <T as IBitPrim>::with_bit_cleared(self, idx)
    }
    #[inline(always)]
    fn with_bit_toggled(self, idx: USize) -> Self {
        <T as IBitPrim>::with_bit_toggled(self, idx)
    }
    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        <T as IBitPrim>::bitor(self, other)
    }
    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        <T as IBitPrim>::bitand(self, other)
    }
    #[inline(always)]
    fn bitnot(self) -> Self {
        <T as IBitPrim>::bitnot(self)
    }
    #[inline(always)]
    fn bitxor(self, other: Self) -> Self {
        <T as IBitPrim>::bitxor(self, other)
    }
    #[inline(always)]
    fn clear_lowest_set_bit(self) -> Self {
        <T as IBitPrim>::clear_lowest_set_bit(self)
    }
}

// --- Container bridges ----------------------------------------------------
//
// `generic_const_exprs` trips on a cycle when the same anonymous
// const-expr appears in multiple where-clause predicates on the same
// impl block (e.g. `S: UContainerFor<{K}>` alongside
// `<S as UContainerFor<{K}>>::T: BitPrim`). The bridge traits below
// collapse both requirements into a single predicate per impl block.

/// Sealed bridge: `(S, BITS)` where `S: BitsContainerFor<BITS, Unsigned>` **and**
/// the container type is `BitPrim`. Collapses the two predicates into
/// one to sidestep the const-expr cycle.
pub const trait UBitContainer<const BITS: u16>:
    sealed::UBridge<BITS> + [const] BitsContainerFor<BITS, Unsigned>
{
    /// The container primitive for this `(S, BITS)` pair.
    type Prim: [const] BitPrim;
    /// Coerce the strategy-selected container into the bridge's
    /// primitive type. Identity at runtime: same underlying integer
    /// type; the coercion exists only to route `UContainerFor::T`
    /// values into `BitPrim` methods inside generic contexts.
    fn to_prim(t: <Self as BitsContainerFor<BITS, Unsigned>>::T) -> Self::Prim;
    /// Reverse of `to_prim`.
    fn from_prim(p: Self::Prim) -> <Self as BitsContainerFor<BITS, Unsigned>>::T;
}

/// Signed counterpart of `UBitContainer`.
pub const trait IBitContainer<const BITS: u16>:
    sealed::IBridge<BITS> + [const] BitsContainerFor<BITS, Signed>
{
    /// The container primitive for this `(S, BITS)` pair.
    type Prim: [const] IBitPrim;
    /// See `UBitContainer::to_prim`.
    fn to_prim(t: <Self as BitsContainerFor<BITS, Signed>>::T) -> Self::Prim;
    /// Reverse of `to_prim`.
    fn from_prim(p: Self::Prim) -> <Self as BitsContainerFor<BITS, Signed>>::T;
}

// Blanket impl: every strategy that picks a `BitPrim` container at a
// given BITS gets the bridge for free. The `sealed::UBridge` bound on
// the trait keeps downstream code from implementing the bridge, while
// the blanket below covers the intended `(S, BITS)` pairs.
//
// The blanket uses identity coercion because `<S as BitsContainerFor<BITS, Unsigned>>::T`
// is exactly the primitive type when the `: BitPrim` bound holds. The
// associated `Prim` type can be set to the container type.

impl<S, const BITS: u16> sealed::UBridge<BITS> for S
where
    S: Strategy,
    S: BitsContainerFor<BITS, Unsigned>,
    <S as BitsContainerFor<BITS, Unsigned>>::T: BitPrim,
{
}

const impl<S, const BITS: u16> UBitContainer<BITS> for S
where
    S: Strategy,
    S: [const] BitsContainerFor<BITS, Unsigned>,
    <S as BitsContainerFor<BITS, Unsigned>>::T: [const] BitPrim,
{
    type Prim = <S as BitsContainerFor<BITS, Unsigned>>::T;

    #[inline(always)]
    fn to_prim(t: <Self as BitsContainerFor<BITS, Unsigned>>::T) -> Self::Prim {
        t
    }

    #[inline(always)]
    fn from_prim(p: Self::Prim) -> <Self as BitsContainerFor<BITS, Unsigned>>::T {
        p
    }
}

// Signed bridge: blanket over Strategy + IContainerFor + IBitPrim.
// Uses its own sealing trait (`sealed::IBridge`) so one Strategy can
// carry both a U-bridge and an I-bridge at the same BITS without the
// two blankets colliding on a shared sealing impl.

impl<S, const BITS: u16> sealed::IBridge<BITS> for S
where
    S: Strategy,
    S: BitsContainerFor<BITS, Signed>,
    <S as BitsContainerFor<BITS, Signed>>::T: IBitPrim,
{
}

const impl<S, const BITS: u16> IBitContainer<BITS> for S
where
    S: Strategy,
    S: [const] BitsContainerFor<BITS, Signed>,
    <S as BitsContainerFor<BITS, Signed>>::T: [const] IBitPrim,
{
    type Prim = <S as BitsContainerFor<BITS, Signed>>::T;

    #[inline(always)]
    fn to_prim(t: <Self as BitsContainerFor<BITS, Signed>>::T) -> Self::Prim {
        t
    }

    #[inline(always)]
    fn from_prim(p: Self::Prim) -> <Self as BitsContainerFor<BITS, Signed>>::T {
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
pub type Narrowed<const N: u16, T> = T;

/// Narrow `Self` to the lowest `N` bits as type `T`.
///
/// `Self` is a wider raw value (a bare primitive or a `Bits<M, S>`
/// with `M > N`). `T` is the target container type. The default
/// body composes `<W as BitPrim>::mask_low(N)` (or `IBitPrim` for
/// signed) with `BitLogic::and` from this crate, then the `as`
/// operator (sound under the mask precondition) to cast to the
/// target primitive type.
///
/// Per topic Q-C, the trait declaration lives here (post-round-
/// 202604280034 merge) and the concrete impls live in `arvo-bitmask`
/// (mask-side) and `arvo-refit` (cross-primitive). Const trait so
/// consumers can call `wide.narrow_to::<13>()` in const fn bodies
/// under generic `Narrow<T>` bounds.
pub const trait Narrow<T> {
    /// Truncate to the lowest `N` bits and return as `T`.
    fn narrow_to<const N: u16>(self) -> T
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
    fn narrow_to_unmasked<const N: u16>(self) -> T
    where
        Self: Sized;
}

// --- Cross-primitive Narrow impls (round 202604281000 Pass B.4) ---------
//
// Coverage: u16/u32/u64/u128 unsigned + i16/i32/i64/i128 signed sources,
// narrowing to every smaller-width primitive of the same sign family.
// Cross-sign narrowing (e.g. u16 -> i8) is out of scope; consumers cast
// sign first via separate impls outside this crate.
//
// These live in arvo-bits-contracts rather than arvo-refit because
// orphan rules require either the trait or one of the type arguments
// to be local to the implementing crate. Since both Narrow and the
// primitive types are foreign to arvo-refit, the impls anchor here
// with Narrow.

macro_rules! impl_narrow_u {
    ($src:ty => $($dst:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-primitive narrow impl on bare primitives that the trait was designed to bridge; tracked: #259
            impl const Narrow<$dst> for $src {
                #[inline(always)]
                fn narrow_to<const N: u16>(self) -> $dst {
                    if N == 0 {
                        return 0 as $dst;
                    }
                    if (N as u32) >= <$src>::BITS {
                        return self as $dst;
                    }
                    let mask: $src = ((1 as $src) << (N as u32)).wrapping_sub(1);
                    (self & mask) as $dst
                }

                #[inline(always)]
                fn narrow_to_unmasked<const N: u16>(self) -> $dst {
                    let _ = N;
                    self as $dst
                }
            }
        )+
    };
}

impl_narrow_u!(u8 => u8);
impl_narrow_u!(u16 => u8, u16);
impl_narrow_u!(u32 => u8, u16, u32);
impl_narrow_u!(u64 => u8, u16, u32, u64);
impl_narrow_u!(u128 => u8, u16, u32, u64, u128);

macro_rules! impl_narrow_i {
    ($src:ty, $unsigned:ty => $($dst:ty),+) => {
        $(
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cross-primitive signed narrow; sign-preserving mask through unsigned reinterpretation; tracked: #259
            impl const Narrow<$dst> for $src {
                #[inline(always)]
                fn narrow_to<const N: u16>(self) -> $dst {
                    if N == 0 {
                        return 0 as $dst;
                    }
                    if (N as u32) >= <$src>::BITS {
                        return self as $dst;
                    }
                    let raw = self as $unsigned;
                    let mask: $unsigned = ((1 as $unsigned) << (N as u32)).wrapping_sub(1);
                    let masked = raw & mask;
                    masked as $dst
                }

                #[inline(always)]
                fn narrow_to_unmasked<const N: u16>(self) -> $dst {
                    let _ = N;
                    self as $dst
                }
            }
        )+
    };
}

impl_narrow_i!(i8, u8 => i8);
impl_narrow_i!(i16, u16 => i8, i16);
impl_narrow_i!(i32, u32 => i8, i16, i32);
impl_narrow_i!(i64, u64 => i8, i16, i32, i64);
impl_narrow_i!(i128, u128 => i8, i16, i32, i64, i128);

// --- Typed Bits<M, S, Sign> -> Bits<N, S, Sign> for M > N ---------------
//
// Forwards through the underlying primitive `Narrow<T_N>` impl on the
// source's container type. Where M > N is not enforced at the type
// level (Rust lacks negative const-bounds in stable form); consumers
// supplying M <= N still get a valid result, but the masking is a
// no-op for cells where the source width already fits.

const impl<const M: u16, const N: u16, S: Strategy, Sign: Signedness> Narrow<Bits<N, S, Sign>>
    for Bits<M, S, Sign>
where
    S: BitsContainerFor<M, Sign>,
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<M, Sign>>::T: [const] Narrow<<S as BitsContainerFor<N, Sign>>::T>,
{
    #[inline(always)]
    fn narrow_to<const W: u16>(self) -> Bits<N, S, Sign> {
        let raw = self.to_raw();
        let narrowed = raw.narrow_to::<W>();
        Bits::from_raw(narrowed)
    }

    #[inline(always)]
    fn narrow_to_unmasked<const W: u16>(self) -> Bits<N, S, Sign> {
        let raw = self.to_raw();
        let narrowed = raw.narrow_to_unmasked::<W>();
        Bits::from_raw(narrowed)
    }
}
