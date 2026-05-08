//! Sketch 05: single-impl projection via `feature(generic_const_exprs)`.
//!
//! Hypothesis: a single `impl` block can compute its associated type from a
//! const fn over `N: u16`, replacing the per-N container table with one
//! projection. The trait-solver should accept this without cycling or ICE.
//!
//! Two scoped questions:
//!
//! Q1. Does `where [(); bytes_for(N)]: ` plus `type T = WideBits<{bytes_for(N)}>;`
//!     compile and resolve correctly across a wide range of N values?
//!
//! Q2. Can the same shape express the strategy axis — i.e., one impl per
//!     strategy (Hot / Warm / Cold / Precise) where the projection differs
//!     per strategy without per-N enumeration?
//!
//! Outcome target: WORKS for both Q1 and Q2.
//!
//! Run: `rustc --edition 2024 05_single_impl_projection.rs && ./05_single_impl_projection`

#![feature(generic_const_exprs)]
#![allow(incomplete_features, dead_code, unused_imports)]

use core::marker::PhantomData;
use core::mem::{align_of, size_of};

// ---------------------------------------------------------------------------
// Substrate primitives (minimised).
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

// Strategy markers.
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

// Sign axis.
pub struct Unsigned;
pub struct Signed;

// ---------------------------------------------------------------------------
// Const fn helpers — these drive the projection.
// ---------------------------------------------------------------------------

/// Bytes needed to hold N logical bits (round up to nearest byte).
pub const fn bytes_for(n: u16) -> usize {
    (n as usize).div_ceil(8)
}

// ---------------------------------------------------------------------------
// Q1: single-impl projection with const-fn associated type.
// ---------------------------------------------------------------------------

pub trait BitsContainer<const N: u16, S> {
    type T: Copy;
}

// One impl per strategy. Per-N projection is a const fn applied at impl level.
// rustc resolves the where clause + associated type at monomorphization.

impl<const N: u16> BitsContainer<N, Warm> for ()
where
    [(); bytes_for(N)]: ,
{
    type T = WideBits<{ bytes_for(N) }>;
}

impl<const N: u16> BitsContainer<N, Hot> for ()
where
    [(); bytes_for(N)]: ,
{
    // Hot uses the SIMD-aligned tier. For this sketch, fix at align(16);
    // the production design selects per cfg(target_feature).
    type T = AlignedWideBits16<{ bytes_for(N) }>;
}

impl<const N: u16> BitsContainer<N, Cold> for ()
where
    [(); bytes_for(N)]: ,
{
    // Cold mirrors Warm at this layer. Bitpacked column-store happens above.
    type T = WideBits<{ bytes_for(N) }>;
}

impl<const N: u16> BitsContainer<N, Precise> for ()
where
    [(); bytes_for(N)]: ,
{
    type T = WideBits<{ bytes_for(N) }>;
}

// ---------------------------------------------------------------------------
// Q2: project per-N into a concrete type via the trait.
// ---------------------------------------------------------------------------

// Type alias for ergonomic access (still resolves via the impl above).
pub type Container<const N: u16, S> = <() as BitsContainer<N, S>>::T;

// Compile-time verification that the projection picks the expected concrete type.
// rustc proves these by walking the trait + associated type + const-fn.
const _: () = {
    // Warm: WideBits<bytes_for(N)>.
    assert!(size_of::<Container<7, Warm>>() == 1);    // 7 bits → 1 byte
    assert!(size_of::<Container<8, Warm>>() == 1);    // 8 bits → 1 byte
    assert!(size_of::<Container<13, Warm>>() == 2);   // 13 bits → 2 bytes
    assert!(size_of::<Container<128, Warm>>() == 16);
    assert!(size_of::<Container<200, Warm>>() == 25);
    assert!(size_of::<Container<256, Warm>>() == 32);
    assert!(size_of::<Container<4096, Warm>>() == 512);

    // Warm always align-1.
    assert!(align_of::<Container<7, Warm>>() == 1);
    assert!(align_of::<Container<200, Warm>>() == 1);
    assert!(align_of::<Container<4096, Warm>>() == 1);

    // Hot: AlignedWideBits16<bytes_for(N)>. Size = round_up(BYTES, 16).
    assert!(size_of::<Container<128, Hot>>() == 16);
    assert!(size_of::<Container<200, Hot>>() == 32);   // 25 bytes → padded to 32
    assert!(size_of::<Container<256, Hot>>() == 32);
    assert!(size_of::<Container<4096, Hot>>() == 512);
    assert!(align_of::<Container<128, Hot>>() == 16);
    assert!(align_of::<Container<200, Hot>>() == 16);
    assert!(align_of::<Container<4096, Hot>>() == 16);

    // Cold: same shape as Warm at this layer.
    assert!(size_of::<Container<200, Cold>>() == 25);
    assert!(align_of::<Container<200, Cold>>() == 1);

    // Precise: same as Warm.
    assert!(size_of::<Container<256, Precise>>() == 32);
    assert!(align_of::<Container<256, Precise>>() == 1);
};

// ---------------------------------------------------------------------------
// Confirm that the projection works at runtime too (not just const-eval).
// ---------------------------------------------------------------------------

fn print_projection<const N: u16, S: 'static>() -> (usize, usize)
where
    (): BitsContainer<N, S>,
    Container<N, S>: Sized,
{
    (size_of::<Container<N, S>>(), align_of::<Container<N, S>>())
}

fn main() {
    let cases: &[(&str, (usize, usize))] = &[
        ("Container<7, Warm>",     print_projection::<7, Warm>()),
        ("Container<128, Warm>",   print_projection::<128, Warm>()),
        ("Container<200, Warm>",   print_projection::<200, Warm>()),
        ("Container<256, Warm>",   print_projection::<256, Warm>()),
        ("Container<4096, Warm>",  print_projection::<4096, Warm>()),
        ("Container<200, Hot>",    print_projection::<200, Hot>()),
        ("Container<200, Cold>",   print_projection::<200, Cold>()),
        ("Container<200, Precise>",print_projection::<200, Precise>()),
        ("Container<4096, Hot>",   print_projection::<4096, Hot>()),
    ];

    for (name, (sz, al)) in cases {
        println!("{name}: size={sz} align={al}");
    }

    println!("\nProjection compiles + resolves across {} N values × 4 strategies", cases.len());
    println!("via single-impl-per-strategy + bytes_for() const fn.");
    println!("No per-N table; replaces the existing arvo container projection table.");
}
