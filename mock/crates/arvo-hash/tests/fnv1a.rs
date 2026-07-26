//! Smoke tests for `Fnv1a<N>` streaming hasher and `ConstHash::hash_const`.
//!
//! Post-round-4 (#314): per-N inherent `hash_const` is gone; tests reach
//! through the `ConstHash<N, Hot, Unsigned>` trait. The streaming and
//! one-shot paths produce identical digests.

#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::strategy::Unsigned;
use arvo::{Bits, Hot};
use arvo_hash::{fnv1a_64, ConstHash, Fnv1a, Hasher};

#[test]
fn streaming_matches_oneshot() {
    let mut h: Fnv1a<28> = Fnv1a::new();
    h.update(b"hello");
    let streamed: Bits<28, Hot> = h.finalize();

    let oneshot: Bits<28, Hot> = <Fnv1a<28> as ConstHash<28, Hot, Unsigned>>::hash_const(b"hello");

    assert_eq!(streamed, oneshot);
}

#[test]
fn hash_const_matches_streaming() {
    const HELLO: Bits<28, Hot> = <Fnv1a<28> as ConstHash<28, Hot, Unsigned>>::hash_const(b"hello");
    let mut h: Fnv1a<28> = Fnv1a::new();
    h.update(b"hello");
    let runtime: Bits<28, Hot> = h.finalize();
    assert_eq!(HELLO, runtime);
}

#[test]
fn hash_const_projects_fnv1a_64() {
    let raw = fnv1a_64(b"hello");
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mirror Fnv1a's per-N mask + container cast at the test site; tracked: #256
    let masked: Bits<28, Hot> = Bits::from_raw((raw & 0x0FFF_FFFF_u64) as u32);
    let via_trait = <Fnv1a<28> as ConstHash<28, Hot, Unsigned>>::hash_const(b"hello");
    assert_eq!(masked, via_trait);
}

#[test]
fn empty_input_yields_offset_basis_truncated() {
    let raw = fnv1a_64(b"");
    // lint:allow(no-bare-numeric) reason: FNV offset basis check; tracked: #256
    assert_eq!(raw, 0xcbf2_9ce4_8422_2325);

    let mut h: Fnv1a<32> = Fnv1a::new();
    h.update(b"");
    let got: Bits<32, Hot> = h.finalize();
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast; tracked: #256
    let expected: Bits<32, Hot> =
        Bits::from_raw((0xcbf2_9ce4_8422_2325_u64 & 0xFFFF_FFFF_u64) as u32);
    assert_eq!(got, expected);
}

#[test]
fn chunked_update_matches_full_update() {
    let mut chunked: Fnv1a<24> = Fnv1a::new();
    chunked.update(b"foo");
    chunked.update(b"bar");
    let chunked_out = chunked.finalize();

    let mut full: Fnv1a<24> = Fnv1a::new();
    full.update(b"foobar");
    let full_out = full.finalize();
    assert_eq!(chunked_out, full_out);
}

#[test]
fn different_widths_share_high_bits_after_mask() {
    let raw = fnv1a_64(b"width-test");
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast; tracked: #256
    let b32: Bits<32, Hot> = Bits::from_raw((raw & 0xFFFF_FFFF_u64) as u32);
    let b32_via_trait = <Fnv1a<32> as ConstHash<32, Hot, Unsigned>>::hash_const(b"width-test");
    assert_eq!(b32, b32_via_trait);

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast; tracked: #256
    let b16: Bits<16, Hot> = Bits::from_raw((raw & 0xFFFF_u64) as u16);
    let b16_via_trait = <Fnv1a<16> as ConstHash<16, Hot, Unsigned>>::hash_const(b"width-test");
    assert_eq!(b16, b16_via_trait);
}
