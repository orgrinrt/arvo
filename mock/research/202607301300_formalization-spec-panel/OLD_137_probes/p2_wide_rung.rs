//! P2: the wide rung's real price. Above 128 bits there is no machine register,
//! so the bar is not a native add: it is the best hand-written multi-limb add at
//! the same width. Five bodies at 256 bits, then the ragged case at 136 bits.
#![no_std]
#![crate_type = "lib"]

// --- the bar: what a bignum author writes by hand at 256 bits ----------------
#[unsafe(no_mangle)]
pub fn bar_256_handwritten(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let (r0, c) = a[0].carrying_add(b[0], false);
    let (r1, c) = a[1].carrying_add(b[1], c);
    let (r2, c) = a[2].carrying_add(b[2], c);
    let (r3, _) = a[3].carrying_add(b[3], c);
    [r0, r1, r2, r3]
}

// --- A: the shipped wide shape, byte limbs -----------------------------------
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WideU8<const BYTES: usize> {
    pub bytes: [u8; BYTES],
}

impl<const BYTES: usize> WideU8<BYTES> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let mut out = [0u8; BYTES];
        let mut carry = false;
        let mut i = 0;
        while i < BYTES {
            let (s, c) = self.bytes[i].carrying_add(o.bytes[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        WideU8 { bytes: out }
    }
}

// --- B: same byte payload, u64 limbs read out and written back ---------------
// BYTES is a multiple of 8 here; the ragged case is section D.
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct WideW<const WORDS: usize> {
    pub words: [u64; WORDS],
}

impl<const WORDS: usize> WideW<WORDS> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let mut out = [0u64; WORDS];
        let mut carry = false;
        let mut i = 0;
        while i < WORDS {
            let (s, c) = self.words[i].carrying_add(o.words[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        WideW { words: out }
    }
}

// --- C: byte-addressed payload, u64 limbs via chunk reads --------------------
// The shipped WideBits keeps [u8; BYTES]. This body keeps that layout and reads
// eight bytes at a time, so the limb width is a property of the BODY, not of the
// carrier. BYTES must be a multiple of 8 on this path.
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct WideBytesAligned<const BYTES: usize> {
    pub bytes: [u8; BYTES],
}

impl<const BYTES: usize> WideBytesAligned<BYTES> {
    #[inline]
    pub fn add_word_chunks(self, o: Self) -> Self {
        let mut out = [0u8; BYTES];
        let mut carry = false;
        let mut i = 0;
        while i + 8 <= BYTES {
            let x = u64::from_le_bytes([
                self.bytes[i],
                self.bytes[i + 1],
                self.bytes[i + 2],
                self.bytes[i + 3],
                self.bytes[i + 4],
                self.bytes[i + 5],
                self.bytes[i + 6],
                self.bytes[i + 7],
            ]);
            let y = u64::from_le_bytes([
                o.bytes[i],
                o.bytes[i + 1],
                o.bytes[i + 2],
                o.bytes[i + 3],
                o.bytes[i + 4],
                o.bytes[i + 5],
                o.bytes[i + 6],
                o.bytes[i + 7],
            ]);
            let (s, c) = x.carrying_add(y, carry);
            carry = c;
            let sb = s.to_le_bytes();
            let mut k = 0;
            while k < 8 {
                out[i + k] = sb[k];
                k += 1;
            }
            i += 8;
        }
        // tail bytes, for a ragged BYTES
        while i < BYTES {
            let (s, c) = self.bytes[i].carrying_add(o.bytes[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        WideBytesAligned { bytes: out }
    }
}

// --- D: u128 limbs, since aarch64 lowers u128 add to adds/adcs ----------------
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct WideQ<const Q: usize> {
    pub q: [u128; Q],
}

impl<const Q: usize> WideQ<Q> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let mut out = [0u128; Q];
        let mut carry = false;
        let mut i = 0;
        while i < Q {
            let (s, c) = self.q[i].carrying_add(o.q[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        WideQ { q: out }
    }
}

// --- sites: 256 bits ----------------------------------------------------------
#[unsafe(no_mangle)]
pub fn w_u8_256(a: WideU8<32>, b: WideU8<32>) -> WideU8<32> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn w_w64_256(a: WideW<4>, b: WideW<4>) -> WideW<4> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn w_chunk_256(a: WideBytesAligned<32>, b: WideBytesAligned<32>) -> WideBytesAligned<32> {
    a.add_word_chunks(b)
}
#[unsafe(no_mangle)]
pub fn w_q128_256(a: WideQ<2>, b: WideQ<2>) -> WideQ<2> {
    a.add(b)
}

// --- sites: 136 bits, the ragged case (17 bytes) ------------------------------
#[unsafe(no_mangle)]
pub fn w_u8_136(a: WideU8<17>, b: WideU8<17>) -> WideU8<17> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn w_chunk_136(a: WideBytesAligned<17>, b: WideBytesAligned<17>) -> WideBytesAligned<17> {
    a.add_word_chunks(b)
}

// --- sites: 1024 bits, to see how the ratio scales ----------------------------
#[unsafe(no_mangle)]
pub fn w_u8_1024(a: WideU8<128>, b: WideU8<128>) -> WideU8<128> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn w_w64_1024(a: WideW<16>, b: WideW<16>) -> WideW<16> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn w_chunk_1024(a: WideBytesAligned<128>, b: WideBytesAligned<128>) -> WideBytesAligned<128> {
    a.add_word_chunks(b)
}
