//! FNV-1a against the reference values of its authors' test suite.
//!
//! The values for the empty string, `"a"` and `"foobar"` are the ones the
//! FNV reference code prints for those inputs, at 32 bits. The 64-bit FNV-1a
//! and its values join with ticket
//! `arvo_format_admits_the_wrapping_product_over_the_unsigned_64`.

use arvo_bits::Bytes;
use arvo_digest::Fnv1a;

const AT_32: [(&[u8], u64); 3] =
    [(b"", 0x811C_9DC5), (b"a", 0xE40C_292C), (b"foobar", 0xBF9C_F968)];

fn check_32(table: &[(&[u8], u64)]) {
    for &(bytes, want) in table {
        assert_eq!(
            Fnv1a::<32>::of(Bytes::of(bytes)).raw().raw(),
            want,
            "FNV-1a 32 of {bytes:?}"
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

#[test]
fn the_reference_values_at_32_bits() {
    check_32(&AT_32);
}

/// Every byte value and every prefix of a run of them, so a wrap the product
/// got wrong, or a byte the xor widened wrong, shows on some prefix.
#[test]
fn every_prefix_of_every_byte_value_agrees_with_the_hosts_arithmetic() {
    let mut all = [0u8; 512];
    for (i, byte) in all.iter_mut().enumerate() {
        *byte = (i * 7 + 3) as u8;
    }
    for len in 0 ..= all.len() {
        let prefix = &all[.. len];
        assert_eq!(
            Fnv1a::<32>::of(Bytes::of(prefix)).raw().raw(),
            host_fnv1a_32(prefix),
            "length {len}"
        );
    }
}

/// The same values computed by the compiler, so the `const fn` is the
/// function that was checked and not a runtime twin of it.
#[test]
fn the_reference_values_hold_in_const_context() {
    const FOOBAR_32: u64 = Fnv1a::<32>::of(Bytes::of(b"foobar")).raw().raw();
    assert_eq!(FOOBAR_32, AT_32[2].1);
}

/// The planted vector: one bit flipped in a reference value. The check that
/// passes above has to fail here, or it was not checking.
#[test]
#[should_panic(expected = "FNV-1a 32 of")]
fn a_planted_wrong_vector_is_refused_at_32_bits() {
    check_32(&[(b"a", 0xE40C_292D)]);
}
