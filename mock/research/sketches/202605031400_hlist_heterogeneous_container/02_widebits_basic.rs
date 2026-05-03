//! Sketch 02: `WideBits<const BYTES: usize>` basic shape, scalar `BitPrim` impl.
//!
//! Hypothesis: a byte-sequence container at align-1 (`#[repr(C)]` with only a
//! `[u8; BYTES]` field) is sufficient to express `Bits<N>` for `N > 128`. All
//! `BitPrim` ops compose from per-byte primitives; the substrate gets correct
//! semantics with zero SIMD assumption. Strategy-aware alignment (Hot variant
//! with `#[repr(C, align(N))]`) is a separate sketch (03).
//!
//! Outcome target: WORKS.
//! - Layout: `size_of::<WideBits<BYTES>>() == BYTES`, `align_of` == 1.
//! - count_ones, leading_zeros, trailing_zeros, get_bit, set_bit, clear_bit
//!   produce correct results on test patterns.
//!
//! Run: `rustc --edition 2024 02_widebits_basic.rs && ./02_widebits_basic`

#![allow(dead_code)]

use core::mem::{align_of, size_of};

/// Byte-sequence storage for `Bits<N>` where N > 128.
///
/// `BYTES` is `(N + 7) / 8`. Any unused trailing bits (when N is not a
/// multiple of 8) live in the high bits of the last byte and are required
/// to be zero. This invariant is the consumer's responsibility on writes;
/// reads via the bit-indexed accessors below mask trailing bits implicitly.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WideBits<const BYTES: usize> {
    bytes: [u8; BYTES],
}

impl<const BYTES: usize> WideBits<BYTES> {
    pub const fn zero() -> Self {
        Self { bytes: [0u8; BYTES] }
    }

    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(&self) -> &[u8; BYTES] {
        &self.bytes
    }

    pub const fn count_ones(&self) -> u32 {
        let mut sum: u32 = 0;
        let mut i = 0;
        while i < BYTES {
            sum += self.bytes[i].count_ones();
            i += 1;
        }
        sum
    }

    pub const fn count_zeros(&self) -> u32 {
        (BYTES as u32) * 8 - self.count_ones()
    }

    /// Number of leading zero bits. Treats byte 0 as the most-significant
    /// (big-endian-of-bytes), and within each byte the high bit (bit 7) is
    /// most-significant. Picked for visual coherence with hex literals; the
    /// real substrate may pick LE-of-bytes; either choice is internally
    /// consistent if applied uniformly.
    pub const fn leading_zeros(&self) -> u32 {
        let mut count: u32 = 0;
        let mut i = 0;
        while i < BYTES {
            let b = self.bytes[i];
            if b == 0 {
                count += 8;
                i += 1;
                continue;
            }
            count += b.leading_zeros();
            return count;
        }
        count
    }

    pub const fn trailing_zeros(&self) -> u32 {
        let mut count: u32 = 0;
        let mut i = BYTES;
        while i > 0 {
            i -= 1;
            let b = self.bytes[i];
            if b == 0 {
                count += 8;
                continue;
            }
            count += b.trailing_zeros();
            return count;
        }
        count
    }

    /// Bit index 0 is the least-significant bit of byte 0 (LE-within-byte +
    /// LE-across-bytes). Sketch picks one convention; production may differ.
    pub const fn get_bit(&self, idx: usize) -> bool {
        let byte_idx = idx / 8;
        let bit_in_byte = idx % 8;
        (self.bytes[byte_idx] >> bit_in_byte) & 1 == 1
    }

    pub const fn set_bit(&mut self, idx: usize, value: bool) {
        let byte_idx = idx / 8;
        let bit_in_byte = idx % 8;
        let mask = 1u8 << bit_in_byte;
        if value {
            self.bytes[byte_idx] |= mask;
        } else {
            self.bytes[byte_idx] &= !mask;
        }
    }

    pub const fn bitand(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            out[i] = self.bytes[i] & other.bytes[i];
            i += 1;
        }
        Self { bytes: out }
    }

    pub const fn bitor(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            out[i] = self.bytes[i] | other.bytes[i];
            i += 1;
        }
        Self { bytes: out }
    }

    pub const fn bitxor(self, other: Self) -> Self {
        let mut out = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            out[i] = self.bytes[i] ^ other.bytes[i];
            i += 1;
        }
        Self { bytes: out }
    }

    pub const fn bitnot(self) -> Self {
        let mut out = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            out[i] = !self.bytes[i];
            i += 1;
        }
        Self { bytes: out }
    }
}

// Compile-time layout assertions: align-1, exact byte sizing, no padding.
const _: () = {
    assert!(size_of::<WideBits<17>>() == 17, "WideBits<17> must be 17 bytes");
    assert!(align_of::<WideBits<17>>() == 1, "WideBits<17> must be align-1");
    assert!(size_of::<WideBits<25>>() == 25, "WideBits<25> must be 25 bytes (200 logical bits)");
    assert!(align_of::<WideBits<25>>() == 1);
    assert!(size_of::<WideBits<32>>() == 32, "WideBits<32> must be 32 bytes");
    assert!(align_of::<WideBits<32>>() == 1, "WideBits<32> stays align-1 even though native u256 would be 16-aligned");
    assert!(size_of::<WideBits<64>>() == 64);
    assert!(align_of::<WideBits<64>>() == 1);
    assert!(size_of::<WideBits<128>>() == 128);
    assert!(align_of::<WideBits<128>>() == 1);
    assert!(size_of::<WideBits<512>>() == 512, "WideBits<512> = 4096 logical bits");
    assert!(align_of::<WideBits<512>>() == 1);
};

// Compile-time count_ones verification (const-eval).
const _: () = {
    let zero = WideBits::<17>::zero();
    assert!(zero.count_ones() == 0);
    assert!(zero.count_zeros() == 136);

    let all_ones = WideBits::<17>::from_bytes([0xFFu8; 17]);
    assert!(all_ones.count_ones() == 136);
    assert!(all_ones.count_zeros() == 0);

    // Mixed pattern: 0x55 = 0b01010101 = 4 ones per byte; 17 bytes * 4 = 68.
    let alt = WideBits::<17>::from_bytes([0x55u8; 17]);
    assert!(alt.count_ones() == 68);
};

// Compile-time leading/trailing zeros.
const _: () = {
    let zero = WideBits::<4>::zero();
    assert!(zero.leading_zeros() == 32);
    assert!(zero.trailing_zeros() == 32);

    // Big-endian-of-bytes interpretation: byte[0] is most significant.
    // [0x00, 0x00, 0x80, 0x00] → byte 2 has bit 7 set, others zero.
    // leading_zeros = 16 (bytes 0,1) + 0 (byte 2's leading_zeros from MSB) = 16.
    let mid = WideBits::<4>::from_bytes([0x00, 0x00, 0x80, 0x00]);
    assert!(mid.leading_zeros() == 16);
    // trailing_zeros (from byte 3 LSB upward): byte 3 = 0 → +8; byte 2 = 0x80, trailing_zeros = 7.
    assert!(mid.trailing_zeros() == 15);
};

// Compile-time get_bit / set_bit.
const _: () = {
    let mut b = WideBits::<2>::zero();
    b.set_bit(0, true);
    assert!(b.get_bit(0));
    assert!(b.as_bytes()[0] == 1);

    b.set_bit(15, true);
    assert!(b.get_bit(15));
    assert!(b.as_bytes()[1] == 0x80);

    b.set_bit(0, false);
    assert!(!b.get_bit(0));
    assert!(b.as_bytes()[0] == 0);
};

// Compile-time bitwise ops.
const _: () = {
    let a = WideBits::<3>::from_bytes([0xF0, 0x0F, 0xAA]);
    let b = WideBits::<3>::from_bytes([0x0F, 0xF0, 0x55]);
    let and = a.bitand(b);
    assert!(and.as_bytes()[0] == 0x00);
    assert!(and.as_bytes()[1] == 0x00);
    assert!(and.as_bytes()[2] == 0x00);

    let or = a.bitor(b);
    assert!(or.as_bytes()[0] == 0xFF);
    assert!(or.as_bytes()[1] == 0xFF);
    assert!(or.as_bytes()[2] == 0xFF);

    let xor = a.bitxor(b);
    assert!(xor.as_bytes()[0] == 0xFF);
    assert!(xor.as_bytes()[1] == 0xFF);
    assert!(xor.as_bytes()[2] == 0xFF);

    let not = a.bitnot();
    assert!(not.as_bytes()[0] == 0x0F);
    assert!(not.as_bytes()[1] == 0xF0);
    assert!(not.as_bytes()[2] == 0x55);
};

fn main() {
    println!("WideBits<17>: size={} align={}", size_of::<WideBits<17>>(), align_of::<WideBits<17>>());
    println!("WideBits<25>: size={} align={}", size_of::<WideBits<25>>(), align_of::<WideBits<25>>());
    println!("WideBits<32>: size={} align={}", size_of::<WideBits<32>>(), align_of::<WideBits<32>>());
    println!("WideBits<64>: size={} align={}", size_of::<WideBits<64>>(), align_of::<WideBits<64>>());
    println!("WideBits<128>: size={} align={}", size_of::<WideBits<128>>(), align_of::<WideBits<128>>());
    println!("WideBits<512>: size={} align={}", size_of::<WideBits<512>>(), align_of::<WideBits<512>>());

    // Sanity: at runtime, count_ones on 4096-bit all-ones is 4096.
    let all = WideBits::<512>::from_bytes([0xFFu8; 512]);
    println!("WideBits<512> all-ones count_ones: {}", all.count_ones());
    assert_eq!(all.count_ones(), 4096);

    // SHA3-512-sized digest (64 bytes = 512 logical bits).
    let digest_size = WideBits::<64>::zero();
    println!("SHA3-512 digest WideBits<64>: size={}", size_of_val(&digest_size));

    // RSA-4096 modulus (512 bytes).
    println!("RSA-4096 WideBits<512>: size={}", size_of::<WideBits<512>>());
}
