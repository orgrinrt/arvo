//! Smoke tests for `Fnv1a<N>` streaming hasher and `hash_const`.
//!
//! Post-pass-5/6: per-size helpers on `Bits` are gone. Tests now mask
//! the FNV state explicitly and construct via `Bits::from_raw` with
//! the dispatched container type. Bits<28> + Bits<32> use u32; Bits<24>
//! uses u32; Bits<16> uses u16.

use arvo::{Bits, Hot};
use arvo_hash::{Fnv1a, Hasher, HasherExt, fnv1a_64};

#[test]
fn streaming_matches_oneshot() {
    let mut h: Fnv1a<28> = Fnv1a::new();
    h.update(b"hello");
    let streamed: Bits<28, Hot> = h.finalize();

    let oneshot: Bits<28, Hot> = Fnv1a::<28>::new().hash(b"hello");

    assert_eq!(streamed, oneshot);
}

#[test]
fn hash_const_matches_streaming() {
    const HELLO: Bits<28, Hot> = Fnv1a::<28>::hash_const(b"hello");
    let runtime: Bits<28, Hot> = Fnv1a::<28>::new().hash(b"hello");
    assert_eq!(HELLO, runtime);
}

#[test]
fn hash_const_projects_fnv1a_64() {
    let raw = fnv1a_64(b"hello");
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mirror Fnv1a's per-N mask + container cast at the test site; tracked: #256
    let masked: Bits<28, Hot> = Bits::from_raw((raw & 0x0FFF_FFFF_u64) as u32);
    let via_struct = Fnv1a::<28>::hash_const(b"hello");
    assert_eq!(masked, via_struct);
}

#[test]
fn empty_input_yields_offset_basis_truncated() {
    let raw = fnv1a_64(b"");
    // lint:allow(no-bare-numeric) reason: FNV offset basis check; tracked: #256
    assert_eq!(raw, 0xcbf2_9ce4_8422_2325);

    let h: Bits<32, Hot> = Fnv1a::<32>::new().hash(b"");
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast; tracked: #256
    let expected: Bits<32, Hot> = Bits::from_raw((0xcbf2_9ce4_8422_2325_u64 & 0xFFFF_FFFF_u64) as u32);
    assert_eq!(h, expected);
}

#[test]
fn chunked_update_matches_full_update() {
    let mut chunked: Fnv1a<24> = Fnv1a::new();
    chunked.update(b"foo");
    chunked.update(b"bar");
    let chunked_out = chunked.finalize();

    let full: Bits<24, Hot> = Fnv1a::<24>::new().hash(b"foobar");
    assert_eq!(chunked_out, full);
}

#[test]
fn different_widths_share_high_bits_after_mask() {
    let raw = fnv1a_64(b"width-test");
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast; tracked: #256
    let b32: Bits<32, Hot> = Bits::from_raw((raw & 0xFFFF_FFFF_u64) as u32);
    let b32_via_struct = Fnv1a::<32>::hash_const(b"width-test");
    assert_eq!(b32, b32_via_struct);

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: mask + container cast; tracked: #256
    let b16: Bits<16, Hot> = Bits::from_raw((raw & 0xFFFF_u64) as u16);
    let b16_via_struct = Fnv1a::<16>::hash_const(b"width-test");
    assert_eq!(b16, b16_via_struct);
}
