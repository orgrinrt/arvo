//! Trait-bound sanity: a trivial Hasher<N> implementor compiles,
//! and HasherExt's blanket hash() method is callable.

use arvo::{Bits, Hot};
use arvo_hash::{Hasher, HasherExt};

/// Minimal test hasher: XORs each incoming byte into a running
/// 28-bit state. Not a real hash algorithm; exists to prove the
/// trait surface is implementable and the ext blanket applies.
#[derive(Default)]
struct XorHash28(u64);

impl Hasher<28> for XorHash28 {
    fn update(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= b as u64;
        }
    }
    fn finalize(self) -> Bits<28, Hot> {
        Bits::<28, Hot>::from_raw_u64(self.0)
    }
}

#[test]
fn hasher_streaming_path() {
    let mut h = XorHash28::default();
    h.update(b"hello ");
    h.update(b"world");
    let got = h.finalize();
    let mut expected: u64 = 0;
    for &b in b"hello world" {
        expected ^= b as u64;
    }
    assert_eq!(got.to_raw_u64(), Bits::<28, Hot>::from_raw_u64(expected).to_raw_u64());
}

#[test]
fn hasher_ext_oneshot_blanket() {
    let got = XorHash28::default().hash(b"hello world");
    let mut expected: u64 = 0;
    for &b in b"hello world" {
        expected ^= b as u64;
    }
    assert_eq!(got.to_raw_u64(), Bits::<28, Hot>::from_raw_u64(expected).to_raw_u64());
}

#[test]
fn fnv1a_64_known_vector() {
    // FNV-1a-64 of "" = 0xcbf29ce484222325 (offset basis)
    assert_eq!(arvo_hash::fnv1a_64(b""), 0xcbf2_9ce4_8422_2325);
    // FNV-1a-64 of "a" = 0xaf63dc4c8601ec8c
    assert_eq!(arvo_hash::fnv1a_64(b"a"), 0xaf63_dc4c_8601_ec8c);
}
