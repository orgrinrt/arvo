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
            // lint:allow(no-bare-numeric) reason: XOR test hasher operating on u64 state; tracked: #256
            self.0 ^= b as u64;
        }
    }
    fn finalize(self) -> Bits<28, Hot> {
        // Bits<28, Hot> uses u32 container.
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast per D-7; tracked: #256
        Bits::from_raw((self.0 & 0x0FFF_FFFF_u64) as u32)
    }
}

#[test]
fn hasher_streaming_path() {
    let mut h = XorHash28::default();
    h.update(b"hello ");
    h.update(b"world");
    let got = h.finalize();
    // lint:allow(no-bare-numeric) reason: test scaffolding mirrors XorHash28 internals; tracked: #256
    let mut expected: u64 = 0;
    for &b in b"hello world" {
        // lint:allow(no-bare-numeric) reason: XOR test mirror; tracked: #256
        expected ^= b as u64;
    }
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mirror finalize; tracked: #256
    let expected_bits: Bits<28, Hot> = Bits::from_raw((expected & 0x0FFF_FFFF_u64) as u32);
    assert_eq!(got, expected_bits);
}

#[test]
fn hasher_ext_oneshot_blanket() {
    let got = XorHash28::default().hash(b"hello world");
    // lint:allow(no-bare-numeric) reason: test scaffolding; tracked: #256
    let mut expected: u64 = 0;
    for &b in b"hello world" {
        // lint:allow(no-bare-numeric) reason: XOR test mirror; tracked: #256
        expected ^= b as u64;
    }
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mirror finalize; tracked: #256
    let expected_bits: Bits<28, Hot> = Bits::from_raw((expected & 0x0FFF_FFFF_u64) as u32);
    assert_eq!(got, expected_bits);
}

#[test]
fn fnv1a_64_known_vector() {
    // FNV-1a-64 of "" = 0xcbf29ce484222325 (offset basis)
    // lint:allow(no-bare-numeric) reason: known vector; tracked: #256
    assert_eq!(arvo_hash::fnv1a_64(b""), 0xcbf2_9ce4_8422_2325);
    // FNV-1a-64 of "a" = 0xaf63dc4c8601ec8c
    // lint:allow(no-bare-numeric) reason: known vector; tracked: #256
    assert_eq!(arvo_hash::fnv1a_64(b"a"), 0xaf63_dc4c_8601_ec8c);
}
