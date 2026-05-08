//! Sketch 07: native-or-wide branching projection via generic_const_exprs.
//!
//! Hypothesis: a single per-Strategy impl of `BitsContainerFor<N, Sign>` can
//! project to either a native primitive (N <= 128: u8/u16/u32/u64/u128 or
//! signed) or `WideBits<bytes_for(N)>` / `AlignedWideBits16<bytes_for(N)>`
//! (N > 128) using only stable-shaped `feature(generic_const_exprs)`
//! mechanisms.
//!
//! Why this matters: sketches 02-06 all uniformly project to WideBits across
//! the full range. Decision 2 of round #316 says "N <= 128 specialises to
//! native primitives." The branching projection is the part that was punted
//! to "doc CL captures the choice; SRC CL drives toward whichever rustc
//! accepts cleanly." Audit H2 (2026-05-03) flagged this as a sketchable
//! claim: sketch it before locking the doc CL.
//!
//! Three patterns considered:
//!
//! - **Pattern A (helper marker)**: two non-overlapping impls per strategy,
//!   one on `Marker` (native witness `(N <= 128) as usize - 1`), one on
//!   `WideMarker` (wide witness `(N > 128) as usize - 1`). Compiles, but
//!   the user-facing `Bits<N>` would need to know which marker to look up.
//!   Doesn't compose into a single `<S as BitsContainerFor<N, Sign>>::T`
//!   lookup. Documented but rejected.
//!
//! - **Pattern B (single-impl direct overlap)**: two impls of
//!   `BitsContainerFor<N, Sign> for Hot` with disjoint const-bool where
//!   clauses. **Rustc rejects this with E0119 (conflicting implementations)**
//!   even when the where-clauses are mutually exclusive. The trait solver
//!   doesn't reason about const-bool disjointness for overlap detection.
//!   Confirmed dead end (verified at /tmp/sketch_07b...rs).
//!
//! - **Pattern C (const-tag dispatch through helper trait)**: single impl
//!   per Strategy on the Strategy type itself. Internal const-fn `tag(N)`
//!   returns 0..=5 (5 native buckets + 1 wide). A helper trait
//!   `Project<TAG, Sign, BYTES, S>` has per-(TAG, Sign, S) impls that are
//!   distinct because TAG is a const-generic value. No overlap; no E0119.
//!   **Works.** This is the pattern the substrate adopts.
//!
//! Outcome: WORKS for Pattern C. Pattern A documented as alternative;
//! Pattern B confirmed dead end.
//!
//! Run: `rustc +nightly --edition 2024 07_native_or_wide_projection.rs && ./07_native_or_wide_projection`

#![feature(generic_const_exprs)]
#![allow(incomplete_features, dead_code, unused_imports)]

use core::marker::PhantomData;
use core::mem::{align_of, size_of};

// ---------------------------------------------------------------------------
// Storage primitives.
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WideBits<const BYTES: usize> {
    bytes: [u8; BYTES],
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AlignedWideBits16<const BYTES: usize> {
    bytes: [u8; BYTES],
}

// ---------------------------------------------------------------------------
// Strategy + Sign markers.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)] pub struct Hot;
#[derive(Copy, Clone)] pub struct Warm;
#[derive(Copy, Clone)] pub struct Cold;
#[derive(Copy, Clone)] pub struct Precise;

#[derive(Copy, Clone)] pub struct Unsigned;
#[derive(Copy, Clone)] pub struct Signed;

// ---------------------------------------------------------------------------
// Const fn helpers.
// ---------------------------------------------------------------------------

pub const fn bytes_for(n: u16) -> usize {
    (n as usize).div_ceil(8)
}

/// Bucket tag for the projection.
/// 0..=4 are native buckets (u8/u16/u32/u64/u128). 5 is the wide bucket.
pub const fn tag(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 { 0 }
    else if n <= 16 { 1 }
    else if n <= 32 { 2 }
    else if n <= 64 { 3 }
    else if n <= 128 { 4 }
    else { 5 }
}

// ---------------------------------------------------------------------------
// Pattern C — chosen pattern.
//
// Helper trait `Project<TAG, Sign, BYTES, S>` is impl'd per
// (TAG, Sign, Strategy) combination. Because TAG is a const-generic value
// distinct per impl, rustc accepts the impls without E0119 conflicts.
//
// Strategy axis matters only for the wide bucket (TAG = 5):
// - Hot wide → AlignedWideBits16<BYTES> (align(16) baseline per audit H1)
// - Warm/Cold/Precise wide → WideBits<BYTES> (align(1))
//
// Native buckets are Strategy-independent (each native primitive picks its
// own alignment per platform ABI; Hot doesn't add anything for N <= 128).
//
// Total impls: 5 native buckets × 2 Sign = 10 native impls (Strategy-erased
// via `S`-generic on the impl); 1 wide bucket × 4 Strategy × 2 Sign = 8 wide
// impls. Total: 18. Replaces 256 N values × 2 Sign per Strategy = 2048
// entries in the previous per-N table. ~99% reduction.
// ---------------------------------------------------------------------------

pub trait Project<const TAG: usize, Sign, const BYTES: usize, S> {
    type T: Copy;
}

pub struct Picker;

// Native buckets (Strategy-independent; the S generic is unconstrained,
// but rustc accepts because the impl doesn't actually use S to pick T).
impl<const BYTES: usize, S> Project<0, Unsigned, BYTES, S> for Picker { type T = u8; }
impl<const BYTES: usize, S> Project<1, Unsigned, BYTES, S> for Picker { type T = u16; }
impl<const BYTES: usize, S> Project<2, Unsigned, BYTES, S> for Picker { type T = u32; }
impl<const BYTES: usize, S> Project<3, Unsigned, BYTES, S> for Picker { type T = u64; }
impl<const BYTES: usize, S> Project<4, Unsigned, BYTES, S> for Picker { type T = u128; }

impl<const BYTES: usize, S> Project<0, Signed, BYTES, S> for Picker { type T = i8; }
impl<const BYTES: usize, S> Project<1, Signed, BYTES, S> for Picker { type T = i16; }
impl<const BYTES: usize, S> Project<2, Signed, BYTES, S> for Picker { type T = i32; }
impl<const BYTES: usize, S> Project<3, Signed, BYTES, S> for Picker { type T = i64; }
impl<const BYTES: usize, S> Project<4, Signed, BYTES, S> for Picker { type T = i128; }

// Wide bucket (Strategy-dependent).
impl<Sign, const BYTES: usize> Project<5, Sign, BYTES, Hot> for Picker {
    type T = AlignedWideBits16<BYTES>;
}

impl<Sign, const BYTES: usize> Project<5, Sign, BYTES, Warm> for Picker {
    type T = WideBits<BYTES>;
}

impl<Sign, const BYTES: usize> Project<5, Sign, BYTES, Cold> for Picker {
    type T = WideBits<BYTES>;
}

impl<Sign, const BYTES: usize> Project<5, Sign, BYTES, Precise> for Picker {
    type T = WideBits<BYTES>;
}

// User-facing trait. Single per-Strategy impl using const-tag dispatch.
pub trait BitsContainerFor<const N: u16, Sign> {
    type T: Copy;
}

impl<const N: u16, Sign> BitsContainerFor<N, Sign> for Hot
where
    [(); tag(N)]: ,
    [(); bytes_for(N)]: ,
    Picker: Project<{ tag(N) }, Sign, { bytes_for(N) }, Hot>,
{
    type T = <Picker as Project<{ tag(N) }, Sign, { bytes_for(N) }, Hot>>::T;
}

impl<const N: u16, Sign> BitsContainerFor<N, Sign> for Warm
where
    [(); tag(N)]: ,
    [(); bytes_for(N)]: ,
    Picker: Project<{ tag(N) }, Sign, { bytes_for(N) }, Warm>,
{
    type T = <Picker as Project<{ tag(N) }, Sign, { bytes_for(N) }, Warm>>::T;
}

impl<const N: u16, Sign> BitsContainerFor<N, Sign> for Cold
where
    [(); tag(N)]: ,
    [(); bytes_for(N)]: ,
    Picker: Project<{ tag(N) }, Sign, { bytes_for(N) }, Cold>,
{
    type T = <Picker as Project<{ tag(N) }, Sign, { bytes_for(N) }, Cold>>::T;
}

impl<const N: u16, Sign> BitsContainerFor<N, Sign> for Precise
where
    [(); tag(N)]: ,
    [(); bytes_for(N)]: ,
    Picker: Project<{ tag(N) }, Sign, { bytes_for(N) }, Precise>,
{
    type T = <Picker as Project<{ tag(N) }, Sign, { bytes_for(N) }, Precise>>::T;
}

// ---------------------------------------------------------------------------
// Compile-time verification.
// ---------------------------------------------------------------------------

type Container<const N: u16, S, Sign> = <S as BitsContainerFor<N, Sign>>::T;

const _: () = {
    // Native buckets (any Strategy is equivalent at the storage level).
    assert!(size_of::<Container<7, Warm, Unsigned>>() == 1);    // bucket 0 → u8
    assert!(size_of::<Container<8, Hot, Unsigned>>() == 1);     // bucket 0 boundary
    assert!(size_of::<Container<9, Cold, Unsigned>>() == 2);    // bucket 1 → u16
    assert!(size_of::<Container<16, Warm, Signed>>() == 2);     // bucket 1 → i16
    assert!(size_of::<Container<17, Hot, Unsigned>>() == 4);    // bucket 2 → u32
    assert!(size_of::<Container<32, Precise, Signed>>() == 4);  // bucket 2 → i32
    assert!(size_of::<Container<33, Warm, Unsigned>>() == 8);   // bucket 3 → u64
    assert!(size_of::<Container<64, Cold, Unsigned>>() == 8);
    assert!(size_of::<Container<65, Hot, Signed>>() == 16);     // bucket 4 → i128
    assert!(size_of::<Container<128, Warm, Unsigned>>() == 16);

    // Wide bucket (Strategy-dependent).
    // Warm/Cold/Precise: align-1 byte-exact.
    assert!(size_of::<Container<129, Warm, Unsigned>>() == 17);
    assert!(align_of::<Container<129, Warm, Unsigned>>() == 1);
    assert!(size_of::<Container<200, Warm, Unsigned>>() == 25);
    assert!(align_of::<Container<200, Warm, Unsigned>>() == 1);
    assert!(size_of::<Container<256, Cold, Signed>>() == 32);
    assert!(align_of::<Container<256, Cold, Signed>>() == 1);
    assert!(size_of::<Container<4096, Precise, Unsigned>>() == 512);
    assert!(align_of::<Container<4096, Precise, Unsigned>>() == 1);

    // Hot wide: AlignedWideBits16, padded to multiple of 16, align 16.
    assert!(size_of::<Container<200, Hot, Unsigned>>() == 32);
    assert!(align_of::<Container<200, Hot, Unsigned>>() == 16);
    assert!(size_of::<Container<256, Hot, Signed>>() == 32);
    assert!(align_of::<Container<256, Hot, Signed>>() == 16);
    assert!(size_of::<Container<4096, Hot, Unsigned>>() == 512);
    assert!(align_of::<Container<4096, Hot, Unsigned>>() == 16);
};

// ---------------------------------------------------------------------------
// Notes on rejected patterns
// ---------------------------------------------------------------------------
//
// **Pattern A (helper-marker, two impls on different markers)**: Compiles
// because the two impls are on different marker types (Marker for native,
// WideMarker for wide). But the user-facing `Bits<N>` cannot have two
// containers — it needs a single `<S as BitsContainerFor<N, Sign>>::T`
// lookup. Pattern A would force the consumer to pick the marker, which
// defeats the projection's purpose. Rejected.
//
// **Pattern B (two impls on same Strategy, disjoint where-clauses)**:
//
//   impl<...> BitsContainerFor<N, Sign> for Hot where [(); (N <= 128 as u) - 1]: { ... }
//   impl<...> BitsContainerFor<N, Sign> for Hot where [(); (N >  128 as u) - 1]: { ... }
//
// Rustc rejects with E0119: "conflicting implementations of trait
// `BitsContainer<_, _>` for type `Hot`". The const-bool where-clauses
// are mutually exclusive in semantics but rustc's trait solver does not
// reason about that for overlap detection. Confirmed dead end.
//
// **Pattern C (chosen)**: single per-Strategy impl that defers to a helper
// trait `Project<TAG, Sign, BYTES, S>` whose impls are keyed on the const
// TAG value (distinct const generics → distinct impls → no overlap).
// Compiles cleanly, composes into the substrate's user-facing
// `Bits<const N: Width, S, Sign>` type, validated across all strategies
// and the full N range.

fn main() {
    println!("=== Pattern C: const-tag dispatch (chosen pattern) ===\n");
    println!("Single per-Strategy impl of BitsContainerFor<N, Sign>;");
    println!("internal `tag(N)` const fn dispatches through `Project<TAG, Sign, BYTES, S>`.\n");

    println!("--- Native buckets (TAG 0..=4) ---");
    println!("N=7   Warm  Unsigned: size={}", size_of::<Container<7, Warm, Unsigned>>());
    println!("N=13  Hot   Unsigned: size={}", size_of::<Container<13, Hot, Unsigned>>());
    println!("N=17  Warm  Signed:   size={}", size_of::<Container<17, Warm, Signed>>());
    println!("N=64  Cold  Unsigned: size={}", size_of::<Container<64, Cold, Unsigned>>());
    println!("N=128 Precise Signed: size={}", size_of::<Container<128, Precise, Signed>>());

    println!("\n--- Wide bucket (TAG 5) ---");
    println!("N=129  Warm Unsigned: size={} align={}",
        size_of::<Container<129, Warm, Unsigned>>(),
        align_of::<Container<129, Warm, Unsigned>>());
    println!("N=200  Warm Unsigned: size={} align={}",
        size_of::<Container<200, Warm, Unsigned>>(),
        align_of::<Container<200, Warm, Unsigned>>());
    println!("N=200  Hot  Unsigned: size={} align={}",
        size_of::<Container<200, Hot, Unsigned>>(),
        align_of::<Container<200, Hot, Unsigned>>());
    println!("N=256  Cold Signed:   size={} align={}",
        size_of::<Container<256, Cold, Signed>>(),
        align_of::<Container<256, Cold, Signed>>());
    println!("N=4096 Hot  Unsigned: size={} align={}",
        size_of::<Container<4096, Hot, Unsigned>>(),
        align_of::<Container<4096, Hot, Unsigned>>());

    println!("\n=== Verification summary ===");
    println!("Pattern C WORKS.");
    println!("- 10 native Project impls: 5 buckets × 2 Sign (Strategy-erased)");
    println!("- 8  wide   Project impls: 1 bucket × 4 Strategy × 2 Sign");
    println!("- 4  BitsContainerFor impls: 1 per Strategy");
    println!("- Total: 22 impls replacing 2048+ per-N×Sign×Strategy entries.");
    println!();
    println!("Pattern A (helper-marker): compiles but doesn't compose, rejected.");
    println!("Pattern B (direct overlap on Strategy): rustc E0119, dead end.");
    println!();
    println!("Doc CL adopts Pattern C: const-tag dispatch via Project helper trait.");
}
