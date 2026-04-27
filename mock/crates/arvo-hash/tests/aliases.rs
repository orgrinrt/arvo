//! Sanity: ContentHash aliases Bits<28, Hot> and flows through the
//! underlying API.

use arvo::{Bits, Hot};
use arvo_hash::ContentHash;

#[test]
fn content_hash_is_bits_28() {
    let h: ContentHash = Bits::<28, Hot>::from_raw_u64(0xDEAD_BEEF);
    assert_eq!(h.to_raw_u64(), 0x0EAD_BEEF);
}

#[test]
fn content_hash_roundtrip() {
    let a = ContentHash::from_raw_u64(0x0123_4567);
    let b = ContentHash::from_raw_u64(0x0123_4567);
    assert_eq!(a, b);
}
