//! Sanity: ContentHash aliases Bits<64, Hot> and flows through the
//! underlying API. Bits<64, Hot> uses u64 as its container.

use arvo::{Bits, Hot};
use arvo_hash::ContentHash;

#[test]
fn content_hash_is_bits_64() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal at 64-bit ContentHash width; tracked: #256
    let h: ContentHash = Bits::<64, Hot>::from_raw(0xDEAD_BEEF_CAFE_F00D_u64);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.to_raw(), 0xDEAD_BEEF_CAFE_F00D_u64);
}

#[test]
fn content_hash_roundtrip() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let a = ContentHash::from_raw(0x0123_4567_89AB_CDEF_u64);
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b = ContentHash::from_raw(0x0123_4567_89AB_CDEF_u64);
    assert_eq!(a, b);
}
