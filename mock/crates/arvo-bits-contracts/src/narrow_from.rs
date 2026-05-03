//! `NarrowFromU64<N, S, Sign>` — bounded-generic narrowing of a u64
//! to the dispatched container.
//!
//! Used by `ConstHash` (arvo-hash) to produce typed `Bits<N, S, Sign>`
//! from a u64 algorithm state, and available to any consumer that
//! needs to project a u64 into a strategy-aware container.
//!
//! Per the bridge-home rule, this trait lives in `arvo-bits-contracts`:
//! the lowest layer reachable from arvo-hash where both the
//! `BitsContainerFor` projection and the `WideBits<BYTES, A>`
//! wide-bucket type are visible.
//!
//! Round 202605031548 (#314) introduces this trait. Sketch 01 in
//! `mock/research/sketches/202605031548_round_4_const_hash_and_narrowing/`
//! validates that the per-`(S, Sign)`-keyed blanket impls compose
//! without E0119 conflicts.

use arvo_strategy::{
    A1, A16, Align, Cold, Hot, Precise, Signed, Signedness, Strategy, Unsigned, Warm, WideBits,
    mask_low_bits,
};

/// Narrow a u64 to `Self` under the `(N, S, Sign)` projection.
///
/// `Self` is the dispatched container for `(N, S, Sign)` per
/// `BitsContainerFor`. The trait does not state that bound; consumer
/// code (`ConstHash<N, S, Sign>`) carries the
/// `<S as BitsContainerFor<N, Sign>>::T: [const] NarrowFromU64<N, S, Sign>`
/// supertrait constraint to ensure invocation is type-correct.
///
/// Implementations land in this crate per the orphan rule: the trait
/// is local; impls cover bare primitives (u8..u128, i8..i128) plus
/// `WideBits<BYTES, A>`.
pub const trait NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>: Sized {
    /// Mask the low `N` bits of `raw` and project into `Self`.
    fn narrow_u64(raw: u64) -> Self;
}

// ---------------------------------------------------------------------------
// Native primitive blanket impls.
//
// Per (S, Sign) cell:
//   Hot, Cold, Unsigned: u8 / u16 / u32 / u64 / u128.
//   Hot, Cold, Signed:   i8 / i16 / i32 / i64 / i128.
//   Warm, Precise, Unsigned: u16 / u32 / u64 / u128 (no u8; 2x-logical).
//   Warm, Precise, Signed:   i16 / i32 / i64 / i128.
//
// 36 native impls total. Sketch 01 confirmed the (S, Sign) partitioning
// makes them non-overlapping.
//
// File-local macro keeps the impl bodies single-line so the trait
// shape stays inspection-friendly. The macro is `pub(crate)` discipline
// only (private to this module); consumer-side dedup is not exposed.
// ---------------------------------------------------------------------------

// lint:allow(no-bare-numeric) reason: bare-primitive bridge impls; the trait
// is the substrate's narrowing-from-u64 contract; primitive widths here are
// the dispatched containers per BitsContainerFor; tracked: #314
macro_rules! impl_narrow_for_prims {
    (
        $strategy:ty, $sign:ty,
        $( $prim:ty ),+ $(,)?
    ) => {
        $(
            impl<const N: u16> const NarrowFromU64<N, $strategy, $sign> for $prim {
                #[inline(always)]
                fn narrow_u64(raw: u64) -> Self {
                    (raw & mask_low_bits(N)) as $prim
                }
            }
        )+
    };
}

impl_narrow_for_prims!(Hot,     Unsigned, u8, u16, u32, u64, u128);
impl_narrow_for_prims!(Hot,     Signed,   i8, i16, i32, i64, i128);
impl_narrow_for_prims!(Cold,    Unsigned, u8, u16, u32, u64, u128);
impl_narrow_for_prims!(Cold,    Signed,   i8, i16, i32, i64, i128);
impl_narrow_for_prims!(Warm,    Unsigned, u16, u32, u64, u128);
impl_narrow_for_prims!(Warm,    Signed,   i16, i32, i64, i128);
impl_narrow_for_prims!(Precise, Unsigned, u16, u32, u64, u128);
impl_narrow_for_prims!(Precise, Signed,   i16, i32, i64, i128);

// ---------------------------------------------------------------------------
// Wide-bucket impl.
//
// Generic over (S, Sign, BYTES, A). Places the masked u64 in the low 8
// bytes of the byte-sequence storage. The byte-ordering convention
// matches `u128::trailing_zeros` / `u128::leading_zeros` semantics:
// bit 0 of N is the LSB of byte 0.
// ---------------------------------------------------------------------------

// lint:allow(no-bare-numeric) reason: WideBits inner storage is bare-byte by
// design; narrow_u64 places the masked u64 little-endian into the low 8
// bytes of the byte sequence; tracked: #314
impl<const N: u16, S: Strategy, Sign: Signedness, const BYTES: usize, A: Align>
    const NarrowFromU64<N, S, Sign> for WideBits<BYTES, A>
{
    #[inline]
    fn narrow_u64(raw: u64) -> Self {
        let masked = raw & mask_low_bits(N);
        let masked_bytes = masked.to_le_bytes();
        let mut bytes = [0u8; BYTES];
        let copy_len = if BYTES < 8 { BYTES } else { 8 };
        let mut i = 0;
        while i < copy_len {
            bytes[i] = masked_bytes[i];
            i += 1;
        }
        Self::from_bytes(bytes)
    }
}
