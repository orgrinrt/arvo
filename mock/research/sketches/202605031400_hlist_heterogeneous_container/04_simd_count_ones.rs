//! Sketch 04: cfg-gated SIMD count_ones over WideBits.
//!
//! Hypothesis: cfg(target_arch) + cfg(target_feature) gating compiles cleanly
//! and the intrinsic-based path produces the same numeric result as the
//! scalar baseline.
//!
//! This sketch validates the *surface* — i.e., that the cfg-arms and
//! `core::arch::*` imports compose without breaking. It does not benchmark
//! anything; that's #320.
//!
//! Three paths defined:
//!
//! 1. `count_ones_scalar` — per-byte `u8::count_ones()`. Always available;
//!    matches Sketch 02 exactly.
//!
//! 2. `count_ones_chunked_u64` — read the byte array as `u64` chunks, sum
//!    `u64::count_ones()`. Modern compilers auto-vectorize this; on x86 with
//!    `popcnt` feature it lowers to native popcnt instructions.
//!
//! 3. `count_ones_sse2_load` — explicit SIMD: cfg-gated to x86_64, loads
//!    16 bytes per iteration via `_mm_loadu_si128`, converts to u128 for
//!    the popcount. Shows the intrinsic surface compiles cleanly. NEON
//!    variant for aarch64 mirrors via `vld1q_u8` + `vaddvq_u8` after
//!    `vcntq_u8`.
//!
//! Outcome target: WORKS. All three paths return identical results across
//! several test patterns and widths.
//!
//! Run: `rustc --edition 2024 -C target-cpu=native 04_simd_count_ones.rs && ./04_simd_count_ones`

#![allow(dead_code)]
#![allow(unused_imports)]

use core::mem::{align_of, size_of};

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WideBits<const BYTES: usize> {
    bytes: [u8; BYTES],
}

impl<const BYTES: usize> WideBits<BYTES> {
    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self { Self { bytes } }
    pub const fn as_bytes(&self) -> &[u8; BYTES] { &self.bytes }

    /// Path 1: per-byte scalar baseline.
    pub fn count_ones_scalar(&self) -> u32 {
        self.bytes.iter().map(|b| b.count_ones()).sum()
    }

    /// Path 2: chunked u64 reads. Compiler auto-vectorizes on most targets.
    pub fn count_ones_chunked_u64(&self) -> u32 {
        let mut sum: u32 = 0;
        let chunks = self.bytes.chunks_exact(8);
        let remainder = chunks.remainder();
        for chunk in chunks {
            let arr: [u8; 8] = chunk.try_into().unwrap();
            sum += u64::from_ne_bytes(arr).count_ones();
        }
        for &b in remainder {
            sum += b.count_ones();
        }
        sum
    }

    /// Path 3a: explicit SSE2 16-byte load on x86_64. Falls through to the
    /// scalar baseline on non-x86_64 targets.
    #[cfg(target_arch = "x86_64")]
    pub fn count_ones_sse2_load(&self) -> u32 {
        use core::arch::x86_64::{_mm_loadu_si128, __m128i};
        let mut sum: u32 = 0;
        let mut i = 0;
        // SAFETY: _mm_loadu_si128 supports unaligned loads; we bounds-check
        // via the chunk loop and only load 16 bytes at a time within range.
        unsafe {
            while i + 16 <= BYTES {
                let ptr = self.bytes.as_ptr().add(i) as *const __m128i;
                let v = _mm_loadu_si128(ptr);
                // Round-trip back to u128 for the popcount; demonstrates the
                // load works. A real impl would popcount lanes via
                // `_mm_popcnt_u64` (SSE4.2) or VPOPCNTDQ (AVX-512).
                let bytes: [u8; 16] = core::mem::transmute(v);
                sum += u128::from_ne_bytes(bytes).count_ones();
                i += 16;
            }
        }
        // Tail: bytes that didn't fit in a full 16-byte chunk.
        while i < BYTES {
            sum += self.bytes[i].count_ones();
            i += 1;
        }
        sum
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn count_ones_sse2_load(&self) -> u32 {
        self.count_ones_scalar()
    }

    /// Path 3b: explicit NEON 16-byte load on aarch64.
    #[cfg(target_arch = "aarch64")]
    pub fn count_ones_neon_load(&self) -> u32 {
        use core::arch::aarch64::{vld1q_u8, vcntq_u8, vaddvq_u8};
        let mut sum: u32 = 0;
        let mut i = 0;
        // SAFETY: vld1q_u8 takes a u8*; we bounds-check via the chunk loop.
        unsafe {
            while i + 16 <= BYTES {
                let ptr = self.bytes.as_ptr().add(i);
                let v = vld1q_u8(ptr);
                let counts = vcntq_u8(v);  // per-byte popcount
                sum += vaddvq_u8(counts) as u32;  // horizontal sum
                i += 16;
            }
        }
        while i < BYTES {
            sum += self.bytes[i].count_ones();
            i += 1;
        }
        sum
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn count_ones_neon_load(&self) -> u32 {
        self.count_ones_scalar()
    }
}

fn main() {
    // Test patterns across widths.
    let cases: &[(&str, fn() -> u32, fn() -> u32, fn() -> u32, fn() -> u32, u32)] = &[
        // (name, scalar, chunked, sse2, neon, expected)
        ("zero-32",
         || WideBits::<32>::from_bytes([0x00; 32]).count_ones_scalar(),
         || WideBits::<32>::from_bytes([0x00; 32]).count_ones_chunked_u64(),
         || WideBits::<32>::from_bytes([0x00; 32]).count_ones_sse2_load(),
         || WideBits::<32>::from_bytes([0x00; 32]).count_ones_neon_load(),
         0),
        ("ones-32",
         || WideBits::<32>::from_bytes([0xFF; 32]).count_ones_scalar(),
         || WideBits::<32>::from_bytes([0xFF; 32]).count_ones_chunked_u64(),
         || WideBits::<32>::from_bytes([0xFF; 32]).count_ones_sse2_load(),
         || WideBits::<32>::from_bytes([0xFF; 32]).count_ones_neon_load(),
         256),
        ("alt55-25",
         || WideBits::<25>::from_bytes([0x55; 25]).count_ones_scalar(),
         || WideBits::<25>::from_bytes([0x55; 25]).count_ones_chunked_u64(),
         || WideBits::<25>::from_bytes([0x55; 25]).count_ones_sse2_load(),
         || WideBits::<25>::from_bytes([0x55; 25]).count_ones_neon_load(),
         100),  // 25 bytes * 4 ones-per-byte
        ("ones-512",
         || WideBits::<512>::from_bytes([0xFF; 512]).count_ones_scalar(),
         || WideBits::<512>::from_bytes([0xFF; 512]).count_ones_chunked_u64(),
         || WideBits::<512>::from_bytes([0xFF; 512]).count_ones_sse2_load(),
         || WideBits::<512>::from_bytes([0xFF; 512]).count_ones_neon_load(),
         4096),
        ("alt33-17",
         || WideBits::<17>::from_bytes([0x33; 17]).count_ones_scalar(),
         || WideBits::<17>::from_bytes([0x33; 17]).count_ones_chunked_u64(),
         || WideBits::<17>::from_bytes([0x33; 17]).count_ones_sse2_load(),
         || WideBits::<17>::from_bytes([0x33; 17]).count_ones_neon_load(),
         68),  // 0x33 = 4 ones; 17 bytes * 4 = 68
    ];

    for (name, scalar, chunked, sse2, neon, expected) in cases {
        let s = scalar();
        let c = chunked();
        let x = sse2();
        let n = neon();
        println!("{name}: scalar={s} chunked={c} sse2={x} neon={n} expected={expected}");
        assert_eq!(s, *expected, "scalar mismatch for {name}");
        assert_eq!(c, *expected, "chunked mismatch for {name}");
        assert_eq!(x, *expected, "sse2 mismatch for {name}");
        assert_eq!(n, *expected, "neon mismatch for {name}");
    }
    println!("\nAll three paths agree across {} test cases.", cases.len());
    println!("Active arch: {}",
        if cfg!(target_arch = "x86_64") { "x86_64 (sse2 path live, neon falls through)" }
        else if cfg!(target_arch = "aarch64") { "aarch64 (neon path live, sse2 falls through)" }
        else { "other (both fall through to scalar)" }
    );
}
