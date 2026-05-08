//! Sketch 03: `AlignedWideBitsN<const BYTES>` for the Hot strategy.
//!
//! Hypothesis: `#[repr(C, align(N))]` where N is a literal (16/32/64) gives
//! us SIMD-friendly storage. Const-generic alignment isn't expressible in
//! Rust, so we ship discrete tiers and pick one per cfg-gated target feature.
//!
//! Outcome target: WORKS.
//! - `Aligned16<BYTES>::size_of` == round_up_to_multiple(BYTES, 16).
//! - `Aligned16<BYTES>::align_of` == 16.
//! - Same shape for `Aligned32<BYTES>` / `Aligned64<BYTES>`.
//! - The `[u8; BYTES]` field is preserved; trailing pad is invisible to ops.
//!
//! Run: `rustc --edition 2024 03_aligned_widebits.rs && ./03_aligned_widebits`

#![allow(dead_code)]

use core::mem::{align_of, size_of};

/// SSE2 / NEON baseline alignment (16 bytes).
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Aligned16<const BYTES: usize> {
    bytes: [u8; BYTES],
}

/// AVX-2 / SVE2 alignment (32 bytes).
#[repr(C, align(32))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Aligned32<const BYTES: usize> {
    bytes: [u8; BYTES],
}

/// AVX-512 / cache-line alignment (64 bytes).
#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Aligned64<const BYTES: usize> {
    bytes: [u8; BYTES],
}

impl<const BYTES: usize> Aligned16<BYTES> {
    pub const fn zero() -> Self { Self { bytes: [0u8; BYTES] } }
    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self { Self { bytes } }
    pub const fn as_bytes(&self) -> &[u8; BYTES] { &self.bytes }
}

impl<const BYTES: usize> Aligned32<BYTES> {
    pub const fn zero() -> Self { Self { bytes: [0u8; BYTES] } }
    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self { Self { bytes } }
    pub const fn as_bytes(&self) -> &[u8; BYTES] { &self.bytes }
}

impl<const BYTES: usize> Aligned64<BYTES> {
    pub const fn zero() -> Self { Self { bytes: [0u8; BYTES] } }
    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self { Self { bytes } }
    pub const fn as_bytes(&self) -> &[u8; BYTES] { &self.bytes }
}

// Layout assertions: alignment is fixed; size rounds up to next multiple of alignment.

const fn round_up(n: usize, m: usize) -> usize {
    n.div_ceil(m) * m
}

const _: () = {
    // Aligned16<17> → 32 bytes (next multiple of 16).
    assert!(align_of::<Aligned16<17>>() == 16);
    assert!(size_of::<Aligned16<17>>() == round_up(17, 16));
    assert!(size_of::<Aligned16<17>>() == 32);

    // Aligned16<25> → 32 bytes.
    assert!(align_of::<Aligned16<25>>() == 16);
    assert!(size_of::<Aligned16<25>>() == 32);

    // Aligned16<32> → 32 bytes (exact).
    assert!(align_of::<Aligned16<32>>() == 16);
    assert!(size_of::<Aligned16<32>>() == 32);

    // Aligned16<512> → 512 bytes (exact, 512 = 32 * 16).
    assert!(align_of::<Aligned16<512>>() == 16);
    assert!(size_of::<Aligned16<512>>() == 512);
};

const _: () = {
    // Aligned32<17> → 32 bytes (next multiple of 32).
    assert!(align_of::<Aligned32<17>>() == 32);
    assert!(size_of::<Aligned32<17>>() == 32);

    // Aligned32<33> → 64 bytes.
    assert!(align_of::<Aligned32<33>>() == 32);
    assert!(size_of::<Aligned32<33>>() == 64);

    // Aligned32<64> → 64 bytes (exact).
    assert!(align_of::<Aligned32<64>>() == 32);
    assert!(size_of::<Aligned32<64>>() == 64);

    // Aligned32<512> → 512 bytes.
    assert!(align_of::<Aligned32<512>>() == 32);
    assert!(size_of::<Aligned32<512>>() == 512);
};

const _: () = {
    // Aligned64<17> → 64 bytes.
    assert!(align_of::<Aligned64<17>>() == 64);
    assert!(size_of::<Aligned64<17>>() == 64);

    // Aligned64<65> → 128 bytes.
    assert!(align_of::<Aligned64<65>>() == 64);
    assert!(size_of::<Aligned64<65>>() == 128);

    // Aligned64<512> → 512 bytes.
    assert!(align_of::<Aligned64<512>>() == 64);
    assert!(size_of::<Aligned64<512>>() == 512);
};

// Field access still works: trailing pad is invisible to bytes ops.
const _: () = {
    let a = Aligned16::<17>::from_bytes([0x55u8; 17]);
    assert!(a.as_bytes()[0] == 0x55);
    assert!(a.as_bytes()[16] == 0x55);
    // The struct occupies 32 bytes but the array is only 17.
    assert!(size_of::<Aligned16<17>>() == 32);
    assert!(size_of_val(a.as_bytes()) == 17);
};

fn main() {
    println!("Aligned16<17>: size={} align={} (logical 136 bits, 8 trailing pad bytes)",
             size_of::<Aligned16<17>>(), align_of::<Aligned16<17>>());
    println!("Aligned16<25>: size={} align={} (logical 200 bits, 7 trailing pad bytes)",
             size_of::<Aligned16<25>>(), align_of::<Aligned16<25>>());
    println!("Aligned16<32>: size={} align={} (logical 256 bits, 0 pad)",
             size_of::<Aligned16<32>>(), align_of::<Aligned16<32>>());
    println!("Aligned16<512>: size={} align={} (logical 4096 bits, 0 pad)",
             size_of::<Aligned16<512>>(), align_of::<Aligned16<512>>());

    println!("Aligned32<17>: size={} align={}", size_of::<Aligned32<17>>(), align_of::<Aligned32<17>>());
    println!("Aligned32<33>: size={} align={}", size_of::<Aligned32<33>>(), align_of::<Aligned32<33>>());

    println!("Aligned64<17>: size={} align={}", size_of::<Aligned64<17>>(), align_of::<Aligned64<17>>());

    // Demonstrate the strategy distinction:
    // - Warm: WideBits<25> = 25 bytes, align 1. Storage cost = logical width rounded to byte.
    // - Hot (SSE2/NEON):  Aligned16<25> = 32 bytes, align 16. SIMD-ready, 7 bytes trailing pad.
    // - Hot (AVX-2):      Aligned32<25> = 32 bytes, align 32. SIMD-ready, 7 bytes trailing pad.
    // - Hot (AVX-512):    Aligned64<25> = 64 bytes, align 64. SIMD-ready, 39 bytes trailing pad.
    // For a 4096-bit value (BYTES=512), trailing pad is zero across all alignment tiers.

    let a16 = Aligned16::<25>::from_bytes([0xFFu8; 25]);
    let count: u32 = a16.as_bytes().iter().map(|b| b.count_ones()).sum();
    println!("Aligned16<25> all-ones count_ones via byte iter: {}", count);
    assert_eq!(count, 200);
}
