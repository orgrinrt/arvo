//! P3: at the wide rung, does the carrier's alignment or its limb TYPE decide the
//! codegen, given a word-chunked body? And what does the ragged tail cost?
#![no_std]
#![crate_type = "lib"]

macro_rules! byte_carrier {
    ($name:ident, $align:meta) => {
        #[derive(Clone, Copy)]
        #[repr(C)]
        #[$align]
        pub struct $name<const BYTES: usize> {
            pub bytes: [u8; BYTES],
        }
        impl<const BYTES: usize> $name<BYTES> {
            #[inline]
            pub fn add(self, o: Self) -> Self {
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
                while i < BYTES {
                    let (s, c) = self.bytes[i].carrying_add(o.bytes[i], carry);
                    out[i] = s;
                    carry = c;
                    i += 1;
                }
                $name { bytes: out }
            }
        }
    };
}

byte_carrier!(A1Bytes, cfg_attr(all(), repr(align(1))));
byte_carrier!(A16Bytes, cfg_attr(all(), repr(align(16))));

// --- the word carrier, rounded up: the payload is whole machine words --------
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Words<const W: usize> {
    pub w: [u64; W],
}
impl<const W: usize> Words<W> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let mut out = [0u64; W];
        let mut carry = false;
        let mut i = 0;
        while i < W {
            let (s, c) = self.w[i].carrying_add(o.w[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        Words { w: out }
    }
}

// --- hand-written bars --------------------------------------------------------
#[unsafe(no_mangle)]
pub fn bar_192(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    let (r0, c) = a[0].carrying_add(b[0], false);
    let (r1, c) = a[1].carrying_add(b[1], c);
    let (r2, _) = a[2].carrying_add(b[2], c);
    [r0, r1, r2]
}
#[unsafe(no_mangle)]
pub fn bar_512(a: [u64; 8], b: [u64; 8]) -> [u64; 8] {
    let mut out = [0u64; 8];
    let mut c = false;
    let mut i = 0;
    while i < 8 {
        let (s, cc) = a[i].carrying_add(b[i], c);
        out[i] = s;
        c = cc;
        i += 1;
    }
    out
}

// alignment comparison at 192 bits, 24 bytes, no tail
#[unsafe(no_mangle)]
pub fn a1_192(a: A1Bytes<24>, b: A1Bytes<24>) -> A1Bytes<24> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn a16_192(a: A16Bytes<24>, b: A16Bytes<24>) -> A16Bytes<24> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn words_192(a: Words<3>, b: Words<3>) -> Words<3> {
    a.add(b)
}

// ragged: 200 bits is 25 bytes, one tail byte
#[unsafe(no_mangle)]
pub fn a1_200(a: A1Bytes<25>, b: A1Bytes<25>) -> A1Bytes<25> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn a16_200(a: A16Bytes<25>, b: A16Bytes<25>) -> A16Bytes<25> {
    a.add(b)
}
// the rounded-up word carrier for the same 200 logical bits
#[unsafe(no_mangle)]
pub fn words_200_roundup(a: Words<4>, b: Words<4>) -> Words<4> {
    a.add(b)
}

// 512 bits: does the byte carrier still unroll?
#[unsafe(no_mangle)]
pub fn a1_512(a: A1Bytes<64>, b: A1Bytes<64>) -> A1Bytes<64> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn a16_512(a: A16Bytes<64>, b: A16Bytes<64>) -> A16Bytes<64> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn words_512(a: Words<8>, b: Words<8>) -> Words<8> {
    a.add(b)
}

// footprint, asserted rather than argued
const _: () = assert!(core::mem::size_of::<A1Bytes<17>>() == 17);
const _: () = assert!(core::mem::align_of::<A1Bytes<17>>() == 1);
const _: () = assert!(core::mem::size_of::<A16Bytes<17>>() == 32);
const _: () = assert!(core::mem::size_of::<Words<3>>() == 24);
const _: () = assert!(core::mem::align_of::<Words<3>>() == 8);
