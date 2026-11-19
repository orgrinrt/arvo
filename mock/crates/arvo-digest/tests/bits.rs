//! A digest narrowed to the width a consumer names, and the carriers each
//! width is held in.

use arvo_bits::{Bits, Bytes};
use arvo_digest::{Carrier, Fnv1a, Output, Xxh3};

/// The low `n` bits of `whole`, by the host's own arithmetic.
fn low(whole: u64, n: u32) -> u64 {
    if n == 64 { whole } else { whole & ((1u64 << n) - 1) }
}

/// The planted wrong narrowing: the high `n` bits of a `width`-bit value,
/// moved down. A check that cannot tell this from `low` checks nothing.
fn high(whole: u64, n: u32, width: u32) -> u64 {
    low(whole >> (width - n), n)
}

/// Every `N` a digest of each width admits, each against the expected
/// narrowing of the same value, and the planted one shown different on the
/// inputs the sweep uses.
macro_rules! narrows_at_every_width {
    ($digest:expr, $width:literal, $($n:literal),+ $(,)?) => {{
        let digest = $digest;
        let whole = digest.raw().raw();
        let mut planted_caught = 0u32;
        $(
            let got = digest.bits::<$n>().raw();
            assert_eq!(got, low(whole, $n), "the low {} bits of a {} bit digest", $n, $width);
            if $n < $width && got != high(whole, $n, $width) {
                planted_caught += 1;
            }
        )+
        planted_caught
    }};
}

#[test]
fn a_32_bit_digest_keeps_its_low_bits_at_every_width() {
    for input in [&b"foobar"[..], b"", b"a", b"the quick brown fox"] {
        let caught = narrows_at_every_width!(
            Fnv1a::<32>::of(Bytes::of(input)),
            32,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15,
            16,
            17,
            18,
            19,
            20,
            21,
            22,
            23,
            24,
            25,
            26,
            27,
            28,
            29,
            30,
            31,
            32,
        );
        assert!(
            caught > 0,
            "the planted high-bits narrowing went unseen for {input:?}"
        );
    }
}

#[test]
fn a_64_bit_digest_keeps_its_low_bits_at_every_width() {
    for input in [&b"foobar"[..], b"", b"a", b"the quick brown fox"] {
        for digest in [Xxh3::<64>::of(Bytes::of(input)), Fnv1a::<64>::of(Bytes::of(input))] {
            let caught = narrows_at_every_width!(
                digest, 64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
                63, 64,
            );
            assert!(
                caught > 0,
                "the planted high-bits narrowing went unseen for {input:?}"
            );
        }
    }
}

/// The carrier each width is held in, pinned as a type: each binding compiles
/// only while `Carrier::Raw` at that width is the container named, so a
/// carrier that changed would refuse this file at build.
#[test]
fn each_width_is_held_in_the_carrier_it_names() {
    let at_32: <Output<32> as Carrier>::Raw = Fnv1a::<32>::of(Bytes::of(b"a")).raw();
    let _: Bits<32> = at_32;
    let at_64: <Output<64> as Carrier>::Raw = Fnv1a::<64>::of(Bytes::of(b"a")).raw();
    let _: Bits<64> = at_64;
    let at_128: <Output<128> as Carrier>::Raw = Xxh3::<128>::of(Bytes::of(b"a")).raw();
    let _: [Bits<64>; 2] = at_128;
}
