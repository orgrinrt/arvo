//! Sanity: ContentHash aliases Bits<28> and flows through the
//! underlying API.

use arvo_bits::Bits;
use arvo_hash::ContentHash;

#[test]
fn content_hash_is_bits_28() {
    let h: ContentHash = Bits::<28>::new(0xDEAD_BEEF);
    assert_eq!(h.bits(), 0x0EAD_BEEF);
}

#[test]
fn content_hash_roundtrip() {
    let a = ContentHash::new(0x0123_4567);
    let b = ContentHash::new(0x0123_4567);
    assert_eq!(a, b);
}
