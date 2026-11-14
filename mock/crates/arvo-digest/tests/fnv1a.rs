//! FNV-1a against the reference values of its authors' test suite.
//!
//! The values for the empty string, `"a"` and `"foobar"` are the ones the
//! FNV reference code prints for those inputs, at 32 and at 64 bits.

use arvo_digest::Fnv1a;
use arvo_numeral::Bytes;

const AT_32: [(&[u8], u32); 3] =
    [(b"", 0x811C_9DC5), (b"a", 0xE40C_292C), (b"foobar", 0xBF9C_F968)];

const AT_64: [(&[u8], u64); 3] = [
    (b"", 0xCBF2_9CE4_8422_2325),
    (b"a", 0xAF63_DC4C_8601_EC8C),
    (b"foobar", 0x8594_4171_F739_67E8),
];

fn check_32(table: &[(&[u8], u32)]) {
    for &(bytes, want) in table {
        assert_eq!(
            Fnv1a::<32>::of(Bytes::of(bytes)).raw().host(),
            want,
            "FNV-1a 32 of {bytes:?}"
        );
    }
}

fn check_64(table: &[(&[u8], u64)]) {
    for &(bytes, want) in table {
        assert_eq!(
            Fnv1a::<64>::of(Bytes::of(bytes)).raw().host(),
            want,
            "FNV-1a 64 of {bytes:?}"
        );
    }
}

#[test]
fn the_reference_values_at_32_bits() {
    check_32(&AT_32);
}

#[test]
fn the_reference_values_at_64_bits() {
    check_64(&AT_64);
}

/// The same values computed by the compiler, so the `const fn` is the
/// function that was checked and not a runtime twin of it.
#[test]
fn the_reference_values_hold_in_const_context() {
    const FOOBAR_32: u32 = Fnv1a::<32>::of(Bytes::of(b"foobar")).raw().host();
    const FOOBAR_64: u64 = Fnv1a::<64>::of(Bytes::of(b"foobar")).raw().host();
    assert_eq!(FOOBAR_32, AT_32[2].1);
    assert_eq!(FOOBAR_64, AT_64[2].1);
}

/// The planted vector: one bit flipped in a reference value. The check that
/// passes above has to fail here, or it was not checking.
#[test]
#[should_panic(expected = "FNV-1a 64 of")]
fn a_planted_wrong_vector_is_refused() {
    check_64(&[(b"foobar", 0x8594_4171_F739_67E9)]);
}

#[test]
#[should_panic(expected = "FNV-1a 32 of")]
fn a_planted_wrong_vector_is_refused_at_32_bits() {
    check_32(&[(b"a", 0xE40C_292D)]);
}
