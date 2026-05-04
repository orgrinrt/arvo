//! Const-context smoke test for `BitPrim` and `IBitPrim` const traits.
//!
//! Closes audit Finding 30 (round 202605041051, task #324). Both
//! traits are `pub const trait` after the round 202605021800 sweep.
//! This file exercises the const-callable surface in const blocks
//! across u8/u16/u32/u64/u128 (BitPrim) and i8/i16/i32/i64/i128
//! (IBitPrim), forcing const-eval at compile time.
//!
//! Apply-time finding: the src CL listed `set_bit`/`clear_bit`/
//! `has_bit` as method names; the actual `BitPrim`/`IBitPrim`
//! surface uses `with_bit_set`/`with_bit_cleared`/`get_bit`. The
//! test exercises the actual surface; the deviation is recorded in
//! the src CL apply-time-finding section.

#![feature(const_trait_impl)]

use arvo_bits_contracts::{BitPrim, IBitPrim};
use arvo_storage::{Bool, USize};

const _BITPRIM_U8_WIDTH: USize = <u8 as BitPrim>::WIDTH;
const _BITPRIM_U16_WIDTH: USize = <u16 as BitPrim>::WIDTH;
const _BITPRIM_U32_WIDTH: USize = <u32 as BitPrim>::WIDTH;
const _BITPRIM_U64_WIDTH: USize = <u64 as BitPrim>::WIDTH;
const _BITPRIM_U128_WIDTH: USize = <u128 as BitPrim>::WIDTH;

const _BITPRIM_U8_ZERO: u8 = <u8 as BitPrim>::ZERO;
const _BITPRIM_U64_ONE: u64 = <u64 as BitPrim>::ONE;

const _BITPRIM_U16_COUNT_ONES: USize = <u16 as BitPrim>::count_ones(0xAAAA);
const _BITPRIM_U32_TRAILING: USize = <u32 as BitPrim>::trailing_zeros(0b1000);
const _BITPRIM_U64_LEADING: USize = <u64 as BitPrim>::leading_zeros(0x1);

const _BITPRIM_U8_GET_BIT: Bool = <u8 as BitPrim>::get_bit(0b0010_0000, USize(5));
const _BITPRIM_U16_WITH_SET: u16 = <u16 as BitPrim>::with_bit_set(0, USize(3));
const _BITPRIM_U32_WITH_CLEAR: u32 = <u32 as BitPrim>::with_bit_cleared(0xFFFF_FFFF, USize(0));

const _BITPRIM_U8_IS_ZERO: Bool = <u8 as BitPrim>::is_zero(0);
const _BITPRIM_U64_NONZERO: Bool = <u64 as BitPrim>::is_zero(42);

const _BITPRIM_U32_BITOR: u32 = <u32 as BitPrim>::bitor(0xF0F0, 0x0F0F);
const _BITPRIM_U16_BITAND: u16 = <u16 as BitPrim>::bitand(0xFF00, 0x0FF0);
const _BITPRIM_U8_BITXOR: u8 = <u8 as BitPrim>::bitxor(0xFF, 0xAA);
const _BITPRIM_U64_BITNOT: u64 = <u64 as BitPrim>::bitnot(0);

const _BITPRIM_U64_MASK_LOW: u64 = <u64 as BitPrim>::mask_low(USize(8));
const _BITPRIM_U32_MASK_LOW: u32 = <u32 as BitPrim>::mask_low(USize(16));

const _IBITPRIM_I8_WIDTH: USize = <i8 as IBitPrim>::WIDTH;
const _IBITPRIM_I16_WIDTH: USize = <i16 as IBitPrim>::WIDTH;
const _IBITPRIM_I32_WIDTH: USize = <i32 as IBitPrim>::WIDTH;
const _IBITPRIM_I64_WIDTH: USize = <i64 as IBitPrim>::WIDTH;
const _IBITPRIM_I128_WIDTH: USize = <i128 as IBitPrim>::WIDTH;

const _IBITPRIM_I8_ZERO: i8 = <i8 as IBitPrim>::ZERO;
const _IBITPRIM_I64_ONE: i64 = <i64 as IBitPrim>::ONE;

const _IBITPRIM_I16_COUNT_ONES: USize = <i16 as IBitPrim>::count_ones(-1);
const _IBITPRIM_I32_TRAILING: USize = <i32 as IBitPrim>::trailing_zeros(8);
const _IBITPRIM_I64_LEADING: USize = <i64 as IBitPrim>::leading_zeros(1);

const _IBITPRIM_I8_GET_BIT: Bool = <i8 as IBitPrim>::get_bit(0b0010_0000_i8, USize(5));
const _IBITPRIM_I16_WITH_SET: i16 = <i16 as IBitPrim>::with_bit_set(0, USize(3));
const _IBITPRIM_I32_WITH_CLEAR: i32 = <i32 as IBitPrim>::with_bit_cleared(-1, USize(0));

const _IBITPRIM_I8_IS_ZERO: Bool = <i8 as IBitPrim>::is_zero(0);

const _IBITPRIM_I32_MASK_LOW: i32 = <i32 as IBitPrim>::mask_low(USize(16));

const _CONST_BITPRIM_VALUE_PROBE: () = {
    assert!(<u8 as BitPrim>::WIDTH.0 == 8);
    assert!(<u64 as BitPrim>::WIDTH.0 == 64);
    assert!(<u128 as BitPrim>::WIDTH.0 == 128);
    assert!(<u16 as BitPrim>::count_ones(0xAAAA).0 == 8);
    assert!(<u8 as BitPrim>::is_zero(0).0);
    assert!(!<u8 as BitPrim>::is_zero(1).0);
    assert!(<u32 as BitPrim>::bitor(0xF0F0, 0x0F0F) == 0xFFFF);
    assert!(<u64 as BitPrim>::mask_low(USize(8)) == 0xFF);
};

const _CONST_IBITPRIM_VALUE_PROBE: () = {
    assert!(<i8 as IBitPrim>::WIDTH.0 == 8);
    assert!(<i64 as IBitPrim>::WIDTH.0 == 64);
    assert!(<i128 as IBitPrim>::WIDTH.0 == 128);
    assert!(<i8 as IBitPrim>::is_zero(0).0);
    assert!(<i32 as IBitPrim>::mask_low(USize(16)) == 0xFFFF);
};

#[test]
fn bitprim_const_runtime_parity() {
    assert_eq!(<u8 as BitPrim>::WIDTH, USize(8));
    assert_eq!(<u32 as BitPrim>::count_ones(0xFFFF_FFFF), USize(32));
    assert_eq!(<u64 as BitPrim>::mask_low(USize(8)), 0xFF);
    assert_eq!(<i32 as IBitPrim>::WIDTH, USize(32));
}
