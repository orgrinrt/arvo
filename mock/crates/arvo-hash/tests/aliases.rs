//! Sanity: ContentHash aliases Bits<28, Hot> and flows through the
//! underlying API. Bits<28, Hot> uses u32 as its container.

use arvo::{Bits, Hot};
use arvo_hash::ContentHash;

#[test]
fn content_hash_is_bits_28() {
    // 0xDEAD_BEEF masked to 28 bits = 0x0EAD_BEEF.
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal masked to 28 bits per D-7 contract; tracked: #256
    let h: ContentHash = Bits::<28, Hot>::from_raw(0xDEAD_BEEF_u32 & 0x0FFF_FFFF_u32);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.to_raw(), 0x0EAD_BEEF_u32);
}

#[test]
fn content_hash_roundtrip() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let a = ContentHash::from_raw(0x0123_4567_u32);
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b = ContentHash::from_raw(0x0123_4567_u32);
    assert_eq!(a, b);
}
