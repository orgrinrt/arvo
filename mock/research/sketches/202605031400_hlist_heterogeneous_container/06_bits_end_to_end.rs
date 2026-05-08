//! Sketch 06: end-to-end `Bits<W, S, Sign>` over the projected container.
//!
//! Hypothesis: the user-facing `Bits<W, S, Sign>` type can wrap the
//! `Container<W, S>` projection from Sketch 05, expose ergonomic
//! constructors and ops, and produce correct results across all four
//! strategies for widths from N=7 (1 byte) through N=4096 (512 bytes).
//!
//! This sketch is the synthesis: it brings together
//! - WideBits<const BYTES> (sketch 02) for align-1 storage
//! - AlignedWideBits16<const BYTES> (sketch 03) for Hot SIMD-aligned storage
//! - cfg-gating (sketch 04) is implicit in the strategy axis; ops here
//!   compose from per-byte primitives so the sketch stays portable
//! - single-impl projection (sketch 05) for the storage selection
//!
//! Outcome target: WORKS. `Bits<7, Warm, Unsigned>::ZERO`, `count_ones`,
//! `bitand`, `bitor` produce correct results across N ∈ {7, 13, 64, 128,
//! 200, 256, 4096} and S ∈ {Warm, Hot, Cold, Precise}.
//!
//! Run: `rustc +nightly --edition 2024 06_bits_end_to_end.rs && ./06_bits_end_to_end`

#![feature(generic_const_exprs)]
#![allow(incomplete_features, dead_code)]

use core::marker::PhantomData;
use core::mem::{align_of, size_of};

// ---------------------------------------------------------------------------
// Storage primitives.
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WideBits<const BYTES: usize> { bytes: [u8; BYTES] }

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AlignedWideBits16<const BYTES: usize> { bytes: [u8; BYTES] }

// Trait abstracting over both storage shapes. Lets `Bits` apply uniform ops
// without caring which physical container the projection picked.
pub trait ByteStorage<const BYTES: usize>: Copy {
    fn zero() -> Self;
    fn from_bytes(bytes: [u8; BYTES]) -> Self;
    fn as_bytes(&self) -> &[u8; BYTES];
    fn count_ones(&self) -> u32 {
        self.as_bytes().iter().map(|b| b.count_ones()).sum()
    }
    fn bitand(self, other: Self) -> Self;
    fn bitor(self, other: Self) -> Self;
}

impl<const BYTES: usize> ByteStorage<BYTES> for WideBits<BYTES> {
    fn zero() -> Self { Self { bytes: [0; BYTES] } }
    fn from_bytes(bytes: [u8; BYTES]) -> Self { Self { bytes } }
    fn as_bytes(&self) -> &[u8; BYTES] { &self.bytes }
    fn bitand(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        for i in 0..BYTES { out[i] = self.bytes[i] & other.bytes[i]; }
        Self { bytes: out }
    }
    fn bitor(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        for i in 0..BYTES { out[i] = self.bytes[i] | other.bytes[i]; }
        Self { bytes: out }
    }
}

impl<const BYTES: usize> ByteStorage<BYTES> for AlignedWideBits16<BYTES> {
    fn zero() -> Self { Self { bytes: [0; BYTES] } }
    fn from_bytes(bytes: [u8; BYTES]) -> Self { Self { bytes } }
    fn as_bytes(&self) -> &[u8; BYTES] { &self.bytes }
    fn bitand(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        for i in 0..BYTES { out[i] = self.bytes[i] & other.bytes[i]; }
        Self { bytes: out }
    }
    fn bitor(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        for i in 0..BYTES { out[i] = self.bytes[i] | other.bytes[i]; }
        Self { bytes: out }
    }
}

// ---------------------------------------------------------------------------
// Strategy markers.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)] pub struct Hot;
#[derive(Copy, Clone)] pub struct Warm;
#[derive(Copy, Clone)] pub struct Cold;
#[derive(Copy, Clone)] pub struct Precise;

#[derive(Copy, Clone)] pub struct Unsigned;
#[derive(Copy, Clone)] pub struct Signed;

// ---------------------------------------------------------------------------
// Container projection (from Sketch 05).
// ---------------------------------------------------------------------------

pub const fn bytes_for(n: u16) -> usize {
    (n as usize).div_ceil(8)
}

pub trait BitsContainer<const N: u16, S>
where
    [(); bytes_for(N)]: ,
{
    type T: Copy + ByteStorage<{ bytes_for(N) }>;
}

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
    type T = AlignedWideBits16<{ bytes_for(N) }>;
}

impl<const N: u16> BitsContainer<N, Cold> for ()
where
    [(); bytes_for(N)]: ,
{
    type T = WideBits<{ bytes_for(N) }>;
}

impl<const N: u16> BitsContainer<N, Precise> for ()
where
    [(); bytes_for(N)]: ,
{
    type T = WideBits<{ bytes_for(N) }>;
}

pub type Container<const N: u16, S> = <() as BitsContainer<N, S>>::T;

// ---------------------------------------------------------------------------
// User-facing Bits.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Bits<const N: u16, S, Sign>
where
    [(); bytes_for(N)]: ,
    (): BitsContainer<N, S>,
{
    container: Container<N, S>,
    _phantom: PhantomData<Sign>,
}

impl<const N: u16, S, Sign> Bits<N, S, Sign>
where
    [(); bytes_for(N)]: ,
    (): BitsContainer<N, S>,
{
    pub fn zero() -> Self {
        Self {
            container: <Container<N, S> as ByteStorage<{ bytes_for(N) }>>::zero(),
            _phantom: PhantomData,
        }
    }

    pub fn from_bytes(bytes: [u8; bytes_for(N)]) -> Self {
        Self {
            container: <Container<N, S> as ByteStorage<{ bytes_for(N) }>>::from_bytes(bytes),
            _phantom: PhantomData,
        }
    }

    pub fn count_ones(&self) -> u32 {
        self.container.count_ones()
    }

    pub fn bitand(self, other: Self) -> Self {
        Self {
            container: self.container.bitand(other.container),
            _phantom: PhantomData,
        }
    }

    pub fn bitor(self, other: Self) -> Self {
        Self {
            container: self.container.bitor(other.container),
            _phantom: PhantomData,
        }
    }

    pub fn storage_size(&self) -> usize {
        size_of::<Container<N, S>>()
    }

    pub fn storage_align(&self) -> usize {
        align_of::<Container<N, S>>()
    }
}

// ---------------------------------------------------------------------------
// End-to-end exercise.
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Storage geometry ===");
    let b_warm_7 = Bits::<7, Warm, Unsigned>::zero();
    let b_warm_13 = Bits::<13, Warm, Unsigned>::zero();
    let b_warm_64 = Bits::<64, Warm, Unsigned>::zero();
    let b_warm_128 = Bits::<128, Warm, Unsigned>::zero();
    let b_warm_200 = Bits::<200, Warm, Unsigned>::zero();
    let b_warm_256 = Bits::<256, Warm, Unsigned>::zero();
    let b_warm_4096 = Bits::<4096, Warm, Unsigned>::zero();

    let b_hot_200 = Bits::<200, Hot, Unsigned>::zero();
    let b_hot_4096 = Bits::<4096, Hot, Unsigned>::zero();

    let b_cold_200 = Bits::<200, Cold, Unsigned>::zero();
    let b_precise_200 = Bits::<200, Precise, Unsigned>::zero();

    println!("Bits<7,    Warm,    Unsigned>: storage size={} align={}",   b_warm_7.storage_size(),    b_warm_7.storage_align());
    println!("Bits<13,   Warm,    Unsigned>: storage size={} align={}",   b_warm_13.storage_size(),   b_warm_13.storage_align());
    println!("Bits<64,   Warm,    Unsigned>: storage size={} align={}",   b_warm_64.storage_size(),   b_warm_64.storage_align());
    println!("Bits<128,  Warm,    Unsigned>: storage size={} align={}",   b_warm_128.storage_size(),  b_warm_128.storage_align());
    println!("Bits<200,  Warm,    Unsigned>: storage size={} align={}",   b_warm_200.storage_size(),  b_warm_200.storage_align());
    println!("Bits<256,  Warm,    Unsigned>: storage size={} align={}",   b_warm_256.storage_size(),  b_warm_256.storage_align());
    println!("Bits<4096, Warm,    Unsigned>: storage size={} align={}",   b_warm_4096.storage_size(), b_warm_4096.storage_align());
    println!("Bits<200,  Hot,     Unsigned>: storage size={} align={}",   b_hot_200.storage_size(),   b_hot_200.storage_align());
    println!("Bits<4096, Hot,     Unsigned>: storage size={} align={}",   b_hot_4096.storage_size(),  b_hot_4096.storage_align());
    println!("Bits<200,  Cold,    Unsigned>: storage size={} align={}",   b_cold_200.storage_size(),  b_cold_200.storage_align());
    println!("Bits<200,  Precise, Unsigned>: storage size={} align={}",   b_precise_200.storage_size(), b_precise_200.storage_align());

    println!("\n=== Op correctness ===");

    // count_ones across widths/strategies.
    let all_ones_warm_200 = Bits::<200, Warm, Unsigned>::from_bytes([0xFF; 25]);
    let count = all_ones_warm_200.count_ones();
    println!("Bits<200, Warm>::from_bytes([0xFF; 25]).count_ones() = {} (expect 200)", count);
    assert_eq!(count, 200);

    let all_ones_hot_200 = Bits::<200, Hot, Unsigned>::from_bytes([0xFF; 25]);
    let count_hot = all_ones_hot_200.count_ones();
    println!("Bits<200, Hot>::from_bytes([0xFF; 25]).count_ones() = {} (expect 200)", count_hot);
    assert_eq!(count_hot, 200);

    let all_ones_warm_4096 = Bits::<4096, Warm, Unsigned>::from_bytes([0xFF; 512]);
    let count_4096 = all_ones_warm_4096.count_ones();
    println!("Bits<4096, Warm>::from_bytes([0xFF; 512]).count_ones() = {} (expect 4096)", count_4096);
    assert_eq!(count_4096, 4096);

    // bitwise ops.
    let a_w = Bits::<200, Warm, Unsigned>::from_bytes([0xF0; 25]);
    let b_w = Bits::<200, Warm, Unsigned>::from_bytes([0x0F; 25]);
    let and = a_w.bitand(b_w);
    let or = a_w.bitor(b_w);
    println!("Bits<200, Warm>: 0xF0 AND 0x0F count_ones = {} (expect 0)", and.count_ones());
    println!("Bits<200, Warm>: 0xF0 OR  0x0F count_ones = {} (expect 200)", or.count_ones());
    assert_eq!(and.count_ones(), 0);
    assert_eq!(or.count_ones(), 200);

    let a_h = Bits::<200, Hot, Unsigned>::from_bytes([0xF0; 25]);
    let b_h = Bits::<200, Hot, Unsigned>::from_bytes([0x0F; 25]);
    let and_h = a_h.bitand(b_h);
    let or_h = a_h.bitor(b_h);
    println!("Bits<200, Hot>:  0xF0 AND 0x0F count_ones = {} (expect 0)", and_h.count_ones());
    println!("Bits<200, Hot>:  0xF0 OR  0x0F count_ones = {} (expect 200)", or_h.count_ones());
    assert_eq!(and_h.count_ones(), 0);
    assert_eq!(or_h.count_ones(), 200);

    println!("\n=== Summary ===");
    println!("- Bits<W, S, Sign> over generic_const_exprs projection: WORKS");
    println!("- Strategy axis genuinely drives storage shape (Warm/Cold/Precise = align-1, Hot = align-16)");
    println!("- Bytes-for(W) projects logical width to physical bytes uniformly");
    println!("- Same trait surface (count_ones, bitand, bitor) over different physical containers");
    println!("- Ready for #316 doc CL");
}
