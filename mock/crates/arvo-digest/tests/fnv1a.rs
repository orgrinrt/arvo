//! FNV-1a against the reference values of its authors' test suite.
//!
//! The values for the empty string, `"a"` and `"foobar"` are the ones the
//! FNV reference code prints for those inputs, at 32 and at 64 bits.

use arvo_bits::Bytes;
use arvo_digest::Fnv1a;

const AT_32: [(&[u8], u64); 3] =
    [(b"", 0x811C_9DC5), (b"a", 0xE40C_292C), (b"foobar", 0xBF9C_F968)];

const AT_64: [(&[u8], u64); 3] = [
    (b"", 0xCBF2_9CE4_8422_2325),
    (b"a", 0xAF63_DC4C_8601_EC8C),
    (b"foobar", 0x8594_4171_F739_67E8),
];

fn check_32(table: &[(&[u8], u64)]) {
    for &(bytes, want) in table {
        assert_eq!(
            Fnv1a::<32>::of(Bytes::of(bytes)).raw().raw(),
            want,
            "FNV-1a 32 of {bytes:?}"
        );
    }
}

fn check_64(table: &[(&[u8], u64)]) {
    for &(bytes, want) in table {
        assert_eq!(
            Fnv1a::<64>::of(Bytes::of(bytes)).raw().raw(),
            want,
            "FNV-1a 64 of {bytes:?}"
        );
    }
}

/// FNV-1a 32 written in the host's own arithmetic, the reference the walk
/// over `Num::mul` and `Bits::xor` is checked against.
fn host_fnv1a_32(bytes: &[u8]) -> u64 {
    let mut state: u32 = 0x811C_9DC5;
    for &byte in bytes {
        state = (state ^ u32::from(byte)).wrapping_mul(0x0100_0193);
    }
    u64::from(state)
}

/// FNV-1a 64 written in the host's own arithmetic.
fn host_fnv1a_64(bytes: &[u8]) -> u64 {
    let mut state: u64 = 0xCBF2_9CE4_8422_2325;
    for &byte in bytes {
        state = (state ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01B3);
    }
    state
}

/// A run of 512 bytes taking every byte value, twice over.
fn the_run() -> [u8; 512] {
    let mut all = [0u8; 512];
    for (i, byte) in all.iter_mut().enumerate() {
        *byte = (i * 7 + 3) as u8;
    }
    all
}

#[test]
fn the_reference_values_at_32_bits() {
    check_32(&AT_32);
}

#[test]
fn the_reference_values_at_64_bits() {
    check_64(&AT_64);
}

/// Every byte value and every prefix of a run of them, so a wrap the product
/// got wrong, or a byte the xor widened wrong, shows on some prefix.
#[test]
fn every_prefix_of_every_byte_value_agrees_with_the_hosts_arithmetic_at_32_bits() {
    let all = the_run();
    for len in 0 ..= all.len() {
        let prefix = &all[.. len];
        assert_eq!(
            Fnv1a::<32>::of(Bytes::of(prefix)).raw().raw(),
            host_fnv1a_32(prefix),
            "length {len}"
        );
    }
}

#[test]
fn every_prefix_of_every_byte_value_agrees_with_the_hosts_arithmetic_at_64_bits() {
    let all = the_run();
    for len in 0 ..= all.len() {
        let prefix = &all[.. len];
        assert_eq!(
            Fnv1a::<64>::of(Bytes::of(prefix)).raw().raw(),
            host_fnv1a_64(prefix),
            "length {len}"
        );
    }
}

/// The same values computed by the compiler, so the `const fn` is the
/// function that was checked and not a runtime twin of it.
#[test]
fn the_reference_values_hold_in_const_context() {
    const FOOBAR_32: u64 = Fnv1a::<32>::of(Bytes::of(b"foobar")).raw().raw();
    const FOOBAR_64: u64 = Fnv1a::<64>::of(Bytes::of(b"foobar")).raw().raw();
    assert_eq!(FOOBAR_32, AT_32[2].1);
    assert_eq!(FOOBAR_64, AT_64[2].1);
}

/// The planted vectors: one bit flipped in a reference value. The check that
/// passes above has to fail here, or it was not checking.
#[test]
#[should_panic(expected = "FNV-1a 32 of")]
fn a_planted_wrong_vector_is_refused_at_32_bits() {
    check_32(&[(b"a", 0xE40C_292D)]);
}

#[test]
#[should_panic(expected = "FNV-1a 64 of")]
fn a_planted_wrong_vector_is_refused_at_64_bits() {
    check_64(&[(b"a", 0xAF63_DC4C_8601_EC8D)]);
}
