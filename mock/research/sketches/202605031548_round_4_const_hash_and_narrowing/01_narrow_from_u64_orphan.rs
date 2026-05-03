//! Sketch 01: NarrowFromU64<N, S, Sign> blanket-impl coherence and orphan check.
//!
//! Hypothesis: `NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>`
//! can be blanket-implemented per native primitive (u8, u16, u32, u64, u128 +
//! signed siblings) keyed on the matching `(S, Sign)` pair, without rustc
//! flagging E0119 (overlapping impl) or coherence violations.
//!
//! Two scoped questions:
//!
//! Q1. Does the `(S, Sign)` constraint on the blanket impls disambiguate
//!     the per-primitive narrowing without overlap?
//!
//! Q2. Can `WideBits<const BYTES: usize, A: Align>` impl `NarrowFromU64`
//!     under a wide-bucket bound keyed on `A` without conflicting with the
//!     native-primitive impls?
//!
//! Outcome target: WORKS for Q1 + Q2.
//!
//! Run: `rustc --edition 2024 01_narrow_from_u64_orphan.rs && ./01_narrow_from_u64_orphan`

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features, dead_code, unused_imports)]

use core::marker::{ConstParamTy, PhantomData};

// ---------------------------------------------------------------------------
// Strategy markers (mirror substrate).
// ---------------------------------------------------------------------------

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Hot;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Warm;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Cold;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Precise;

mod sealed { pub trait Sealed {} }

pub trait Strategy: sealed::Sealed + Copy + Clone + Default + 'static {}
impl sealed::Sealed for Hot {} impl Strategy for Hot {}
impl sealed::Sealed for Warm {} impl Strategy for Warm {}
impl sealed::Sealed for Cold {} impl Strategy for Cold {}
impl sealed::Sealed for Precise {} impl Strategy for Precise {}

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Unsigned;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Signed;

pub trait Signedness: sealed::Sealed + Copy + Clone + Default + 'static {}
impl sealed::Sealed for Unsigned {} impl Signedness for Unsigned {}
impl sealed::Sealed for Signed {} impl Signedness for Signed {}

// Align markers for WideBits.
pub trait Align: sealed::Sealed + Copy + Clone + Default + 'static {}
#[repr(C, align(1))] #[derive(Copy, Clone, Default)] pub struct A1;
#[repr(C, align(16))] #[derive(Copy, Clone, Default)] pub struct A16;
impl sealed::Sealed for A1 {} impl Align for A1 {}
impl sealed::Sealed for A16 {} impl Align for A16 {}

// WideBits storage.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WideBits<const BYTES: usize, A: Align = A1> {
    bytes: [u8; BYTES],
    _align: [A; 0],
}

// ---------------------------------------------------------------------------
// NarrowFromU64 trait.
// ---------------------------------------------------------------------------

/// Narrow a u64 to Self under the (N, S, Sign) projection.
///
/// Self is the dispatched container for `(N, S, Sign)`. The trait does
/// not state that bound; it relies on consumer-side `BitsContainerFor`
/// constraints to ensure this trait is only invoked with a matching Self.
pub const trait NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>: Sized {
    fn narrow_u64(raw: u64) -> Self;
}

// ---------------------------------------------------------------------------
// Blanket impls per native primitive, keyed on (S, Sign).
//
// Hot + Cold + Unsigned: u8/u16/u32/u64/u128.
// Hot + Cold + Signed: i8/i16/i32/i64/i128.
// Warm + Precise + Unsigned: u16/u32/u64/u128 (no u8 because Warm 2x-logical).
// Warm + Precise + Signed: i16/i32/i64/i128.
//
// The (S, Sign) constraint partitions the impls so no two impls overlap
// on the same primitive Self type. Each primitive carries impls only
// for the (S, Sign) combinations where that primitive is the dispatched
// container under BitsContainerFor.
// ---------------------------------------------------------------------------

const fn mask_low(n: u16) -> u64 {
    if n >= 64 { u64::MAX } else { (1u64 << n) - 1 }
}

// Hot + Unsigned: native u8/u16/u32/u64/u128 covering N=1..=128.
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u8 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u8 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u16 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u16 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u32 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u32 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u64 {
    fn narrow_u64(raw: u64) -> Self { raw & mask_low(N) }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u128 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u128 }
}

// Cold + Unsigned: same primitive ladder as Hot; (S, Sign) tag distinguishes.
// This is where the orphan question matters: u8 already impls
// NarrowFromU64<N, Hot, Unsigned>; can it also impl NarrowFromU64<N, Cold, Unsigned>?
// Different (S, Sign) tags so the impls are non-overlapping.
impl<const N: u16> const NarrowFromU64<N, Cold, Unsigned> for u8 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u8 }
}
impl<const N: u16> const NarrowFromU64<N, Cold, Unsigned> for u16 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u16 }
}
impl<const N: u16> const NarrowFromU64<N, Cold, Unsigned> for u32 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u32 }
}
impl<const N: u16> const NarrowFromU64<N, Cold, Unsigned> for u64 {
    fn narrow_u64(raw: u64) -> Self { raw & mask_low(N) }
}
impl<const N: u16> const NarrowFromU64<N, Cold, Unsigned> for u128 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u128 }
}

// Warm + Unsigned: 2x-logical ladder. u16/u32/u64/u128, no u8.
impl<const N: u16> const NarrowFromU64<N, Warm, Unsigned> for u16 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u16 }
}
impl<const N: u16> const NarrowFromU64<N, Warm, Unsigned> for u32 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u32 }
}
impl<const N: u16> const NarrowFromU64<N, Warm, Unsigned> for u64 {
    fn narrow_u64(raw: u64) -> Self { raw & mask_low(N) }
}
impl<const N: u16> const NarrowFromU64<N, Warm, Unsigned> for u128 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u128 }
}

// Precise + Unsigned: same primitive ladder as Warm.
impl<const N: u16> const NarrowFromU64<N, Precise, Unsigned> for u16 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u16 }
}
impl<const N: u16> const NarrowFromU64<N, Precise, Unsigned> for u32 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u32 }
}
impl<const N: u16> const NarrowFromU64<N, Precise, Unsigned> for u64 {
    fn narrow_u64(raw: u64) -> Self { raw & mask_low(N) }
}
impl<const N: u16> const NarrowFromU64<N, Precise, Unsigned> for u128 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u128 }
}

// Signed siblings. Hash output is conceptually unsigned, but the substrate
// is symmetric across Sign so the trait covers both axes. Narrowing for
// Signed reinterprets the masked u64 as the matching signed primitive.
impl<const N: u16> const NarrowFromU64<N, Hot, Signed> for i8 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as i8 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Signed> for i16 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as i16 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Signed> for i32 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as i32 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Signed> for i64 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as i64 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Signed> for i128 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as i128 }
}

// (Cold/Warm/Precise + Signed elided in the sketch; same shape, no surprises
// expected — the orphan question is settled by the (S, Sign) tag pair.)

// ---------------------------------------------------------------------------
// WideBits impls. Wide bucket dispatch.
//
// Per the bridge-home rule, WideBits narrowing is one impl per (S, Sign, A)
// triple keyed by alignment marker. Hot uses A=A16 (SSE2/NEON baseline);
// others use A=A1.
// ---------------------------------------------------------------------------

impl<const N: u16, const BYTES: usize, A: Align> const NarrowFromU64<N, Hot, Unsigned>
    for WideBits<BYTES, A>
{
    fn narrow_u64(raw: u64) -> Self {
        // Place the masked u64 in the low 8 bytes of the byte-sequence.
        let mut bytes = [0u8; BYTES];
        let masked = raw & mask_low(N);
        let masked_bytes = masked.to_le_bytes();
        let copy_len = if BYTES < 8 { BYTES } else { 8 };
        let mut i = 0;
        while i < copy_len {
            bytes[i] = masked_bytes[i];
            i += 1;
        }
        WideBits { bytes, _align: [] }
    }
}

// ---------------------------------------------------------------------------
// Validation: try to instantiate at multiple widths and (S, Sign) tags.
// If rustc accepts the file, the orphan + coherence questions are answered.
// ---------------------------------------------------------------------------

const fn check_hot_unsigned<const N: u16>(raw: u64) -> u64 {
    // For Hot+Unsigned at N <= 8, the dispatched container is u8.
    // Use trait-method dispatch to verify the impl is reachable.
    let v = <u8 as NarrowFromU64<N, Hot, Unsigned>>::narrow_u64(raw);
    v as u64
}

const fn check_warm_unsigned<const N: u16>(raw: u64) -> u64 {
    // Warm+Unsigned at N <= 8 dispatches to u16 (2x logical).
    let v = <u16 as NarrowFromU64<N, Warm, Unsigned>>::narrow_u64(raw);
    v as u64
}

const fn check_cold_unsigned<const N: u16>(raw: u64) -> u64 {
    // Cold+Unsigned at N <= 8 dispatches to u8 (same as Hot).
    let v = <u8 as NarrowFromU64<N, Cold, Unsigned>>::narrow_u64(raw);
    v as u64
}

fn main() {
    let raw = 0xdead_beef_cafe_babeu64;

    // Hot+Unsigned, various N.
    let h7 = check_hot_unsigned::<7>(raw);
    let h13 = <u16 as NarrowFromU64<13, Hot, Unsigned>>::narrow_u64(raw);
    let h32 = <u32 as NarrowFromU64<32, Hot, Unsigned>>::narrow_u64(raw);
    let h64 = <u64 as NarrowFromU64<64, Hot, Unsigned>>::narrow_u64(raw);
    let h100 = <u128 as NarrowFromU64<100, Hot, Unsigned>>::narrow_u64(raw);
    println!("Hot Unsigned: 7={h7:#x} 13={h13:#x} 32={h32:#x} 64={h64:#x} 100={h100:#x}");

    // Warm+Unsigned, various N (N<=64 covered, no native bucket at 65..=128).
    let w7 = check_warm_unsigned::<7>(raw);
    let w13 = <u32 as NarrowFromU64<13, Warm, Unsigned>>::narrow_u64(raw);
    let w32 = <u64 as NarrowFromU64<32, Warm, Unsigned>>::narrow_u64(raw);
    let w64 = <u128 as NarrowFromU64<64, Warm, Unsigned>>::narrow_u64(raw);
    println!("Warm Unsigned: 7={w7:#x} 13={w13:#x} 32={w32:#x} 64={w64:#x}");

    // Cold+Unsigned, ladder same as Hot but distinct (S, Sign) tag.
    let c7 = check_cold_unsigned::<7>(raw);
    let c64 = <u64 as NarrowFromU64<64, Cold, Unsigned>>::narrow_u64(raw);
    println!("Cold Unsigned: 7={c7:#x} 64={c64:#x}");

    // Signed.
    let s7 = <i8 as NarrowFromU64<7, Hot, Signed>>::narrow_u64(raw);
    let s32 = <i32 as NarrowFromU64<32, Hot, Signed>>::narrow_u64(raw);
    println!("Hot Signed: 7={s7:#x} 32={s32:#x}");

    // WideBits wide bucket.
    let wide17: WideBits<17, A16> = <WideBits<17, A16> as NarrowFromU64<130, Hot, Unsigned>>::narrow_u64(raw);
    println!("WideBits<17,A16> N=130 byte0={:#x}", wide17.bytes[0]);

    println!("ORPHAN CHECK: rustc accepted all impls without E0119 conflict.");
}
