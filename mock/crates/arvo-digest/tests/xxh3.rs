//! XXH3 against xxHash's own sanity vectors.
//!
//! The vectors are `XSUM_XXH3_testdata` and `XSUM_XXH128_testdata` in
//! `tests/sanity_test_vectors.h` of xxHash v0.8.3, at seed zero, over the
//! buffer `tests/sanity_test.c` fills: a 64 bit generator starting at
//! `PRIME32`, each byte its top eight bits, then multiplied by `PRIME64`.
//! One length is taken from each of the algorithm's length classes and from
//! each side of every class boundary: 0, 1 to 3, 4 to 8, 9 to 16, 17 to 128,
//! 129 to 240, and the long path from 241 on, across a stripe and a block.

use arvo_bits::{Bits, Bytes};
use arvo_digest::Xxh3;

/// A 128 bit digest's two halves joined back into the host's integer, low
/// half first, the way the vectors state it.
const fn joined([low, high]: [Bits<64>; 2]) -> u128 {
    ((high.raw() as u128) << 64) | low.raw() as u128
}

const PRIME32: u64 = 2_654_435_761;
const PRIME64: u64 = 11_400_714_785_074_694_797;
const LEN: usize = 4096 + 64 + 1;

const fn sanity_buffer() -> [u8; LEN] {
    let mut buffer = [0u8; LEN];
    let mut generator = PRIME32;
    let mut at = 0;
    while at < LEN {
        buffer[at] = (generator >> 56) as u8;
        generator = generator.wrapping_mul(PRIME64);
        at += 1;
    }
    buffer
}

const BUFFER: [u8; LEN] = sanity_buffer();

const AT_64: [(usize, u64); 16] = [
    (0, 0x2D06_8005_38D3_94C2),
    (1, 0xC44B_DFF4_074E_ECDB),
    (3, 0x5424_7382_A8D6_B94D),
    (4, 0xE5DC_74BC_5184_8A51),
    (8, 0x24CC_C9AC_AA9F_65E4),
    (9, 0x14D5_001C_15DD_3F2B),
    (16, 0x981B_17D3_6C74_98C9),
    (17, 0x796F_5ACD_3A60_F862),
    (128, 0xFCFF_2412_6754_D861),
    (129, 0x98F1_B0A6_79A2_CA29),
    (240, 0x81C3_C2B6_7F56_8CCF),
    (241, 0xC5A6_39EC_D203_0E5E),
    (1024, 0xDD85_C9B5_C110_9C5C),
    (2048, 0xDD59_E2C3_A5F0_38E0),
    (2240, 0x6E73_A905_39CF_2948),
    (4096, 0xE912_0642_9D1F_48F9),
];

/// Low 64 bits first, as the sanity table writes them.
const AT_128: [(usize, u64, u64); 14] = [
    (0, 0x6001_C324_468D_497F, 0x99AA_06D3_0147_98D8),
    (1, 0xC44B_DFF4_074E_ECDB, 0xA6CD_5E93_9200_0F6A),
    (3, 0x5424_7382_A8D6_B94D, 0x20EF_C49F_F024_22EA),
    (4, 0x2E7D_8D68_76A3_9FE9, 0x970D_585A_C632_BF8E),
    (8, 0x64C6_9CAB_4BB2_1DC5, 0x47A7_F080_D82B_B456),
    (9, 0xED7C_CBC5_01EB_7501, 0x564E_F607_8950_D457),
    (16, 0x5629_8025_8A99_8629, 0xC68C_368E_CF8A_9C05),
    (17, 0xABBC_12D1_1973_D7DB, 0x955F_A786_43ED_3669),
    (128, 0xEBB1_5E34_A7FB_5AB1, 0x3999_2220_E045_260A),
    (129, 0x86C9_E3BC_8F0A_3B5C, 0x0381_5FC9_1F1B_30B6),
    (240, 0x5C9A_AE94_C8EB_E5A0, 0xAA42_02DA_A276_9DC8),
    (241, 0xC5A6_39EC_D203_0E5E, 0x99A8_0ECF_0ECF_C647),
    (1024, 0xDD85_C9B5_C110_9C5C, 0x0D30_D240_71C6_4C57),
    (2240, 0x6E73_A905_39CF_2948, 0xCCB1_34FB_FA7C_E49D),
];

fn check_64(table: &[(usize, u64)]) {
    for &(len, want) in table {
        let bytes = &BUFFER[.. len];
        assert_eq!(
            Xxh3::<64>::of(Bytes::of(bytes)).raw().raw(),
            want,
            "XXH3 64 runtime, length {len}"
        );
        assert_eq!(
            Xxh3::<64>::of_const(Bytes::of(bytes)).raw().raw(),
            want,
            "XXH3 64 const, length {len}"
        );
    }
}

fn check_128(table: &[(usize, u64, u64)]) {
    for &(len, low, high) in table {
        let bytes = &BUFFER[.. len];
        let want = (u128::from(high) << 64) | u128::from(low);
        assert_eq!(
            joined(Xxh3::<128>::of(Bytes::of(bytes)).raw()),
            want,
            "XXH3 128 runtime, length {len}"
        );
        assert_eq!(
            joined(Xxh3::<128>::of_const(Bytes::of(bytes)).raw()),
            want,
            "XXH3 128 const, length {len}"
        );
    }
}

#[test]
fn the_sanity_vectors_at_64_bits() {
    check_64(&AT_64);
}

#[test]
fn the_sanity_vectors_at_128_bits() {
    check_128(&AT_128);
}

/// The buffer itself, checked at its first bytes against the generator
/// written out by hand, so a wrong buffer cannot pass by agreeing with a
/// wrong expectation.
#[test]
fn the_sanity_buffer_is_the_reference_one() {
    assert_eq!(BUFFER[0], (PRIME32 >> 56) as u8);
    assert_eq!(BUFFER[1], (PRIME32.wrapping_mul(PRIME64) >> 56) as u8);
}

/// Evaluated by the compiler, at a length on the long path, so `of_const`
/// is shown to be usable where it says it is.
#[test]
fn the_const_path_runs_in_const_context() {
    const LONG: u64 = Xxh3::<64>::of_const(Bytes::of(BUFFER.split_at(1024).0))
        .raw()
        .raw();
    const SHORT: u128 = joined(Xxh3::<128>::of_const(Bytes::of(BUFFER.split_at(17).0)).raw());
    assert_eq!(LONG, 0xDD85_C9B5_C110_9C5C);
    assert_eq!(
        SHORT,
        (0x955F_A786_43ED_3669_u128 << 64) | 0xABBC_12D1_1973_D7DB
    );
}

/// The two paths over every length up to past the first block, not only at
/// the vectors' lengths, since a mismatch inside a class would not show at
/// its edges.
#[test]
fn the_runtime_and_const_paths_agree_at_every_length() {
    for len in 0 ..= 1100 {
        let bytes = &BUFFER[.. len];
        assert_eq!(
            Xxh3::<64>::of(Bytes::of(bytes)),
            Xxh3::<64>::of_const(Bytes::of(bytes)),
            "64, length {len}"
        );
        assert_eq!(
            Xxh3::<128>::of(Bytes::of(bytes)),
            Xxh3::<128>::of_const(Bytes::of(bytes)),
            "128, length {len}"
        );
    }
}

#[test]
#[should_panic(expected = "XXH3 64 runtime, length 241")]
fn a_planted_wrong_vector_is_refused() {
    check_64(&[(241, 0xC5A6_39EC_D203_0E5F)]);
}

#[test]
#[should_panic(expected = "XXH3 128 runtime, length 9")]
fn a_planted_wrong_vector_is_refused_at_128_bits() {
    check_128(&[(9, 0xED7C_CBC5_01EB_7501, 0x564E_F607_8950_D456)]);
}
