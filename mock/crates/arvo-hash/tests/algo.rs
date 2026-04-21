//! Trait-bound sanity: a trivial Hasher<N> implementor compiles,
//! and HasherExt's blanket hash() method is callable.

use arvo_bits::Bits;
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
    fn finalize(self) -> Bits<28> {
        Bits::<28>::new(self.0)
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
    assert_eq!(got.bits(), Bits::<28>::new(expected).bits());
}

#[test]
fn hasher_ext_oneshot_blanket() {
    let got = XorHash28::default().hash(b"hello world");
    let mut expected: u64 = 0;
    for &b in b"hello world" {
        expected ^= b as u64;
    }
    assert_eq!(got.bits(), Bits::<28>::new(expected).bits());
}
