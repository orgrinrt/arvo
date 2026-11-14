//! A digest narrowed to the width a consumer names.

use arvo_bits::Bytes;
use arvo_digest::{Fnv1a, Xxh3};

#[test]
fn the_low_bits_are_kept() {
    let digest = Xxh3::<64>::of(Bytes::of(b"foobar"));
    let whole = digest.raw().raw();
    assert_eq!(digest.bits::<28>().raw(), whole & ((1 << 28) - 1));
    assert_eq!(digest.bits::<64>().raw(), whole);
}

#[test]
fn a_32_bit_digest_narrows_and_fills_its_own_width() {
    let digest = Fnv1a::<32>::of(Bytes::of(b"foobar"));
    assert_eq!(digest.bits::<12>().raw(), 0xBF9C_F968 & 0xFFF);
    assert_eq!(digest.bits::<32>().raw(), 0xBF9C_F968);
}

/// The planted case: a narrowing that kept the high bits instead would give
/// this value, and it must not be what comes back.
#[test]
fn the_high_bits_are_not_what_is_kept() {
    let digest = Xxh3::<64>::of(Bytes::of(b"foobar"));
    assert_ne!(digest.bits::<16>().raw(), digest.raw().raw() >> 48);
}
