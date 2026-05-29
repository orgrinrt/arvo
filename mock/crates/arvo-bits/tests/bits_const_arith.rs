//! Const-context smoke test for `Bits<N, S, Sign>` const-trait surface.
//!
//! Closes audit Finding 27 (round 202605041051, task #324). Round 305
//! lifted `BitAccess` / `BitSequence` / `BitLogic` blanket impls on
//! `Bits` to `impl const`. Round 306 added `Identity` / `Bounded`
//! blankets. This file exercises that surface in const blocks across
//! the container-dispatch buckets (8, 16, 32, 64, 128, 256) for
//! Sign = Unsigned, plus a representative Sign = Signed entry.

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo_bits::{BitAccess, BitLogic, BitSequence};
use arvo_storage::{Bits, USize};
use arvo_strategy::{Bounded, Hot, Identity, Signed, Unsigned, Warm};

const _BITS8_ZERO: Bits<8, Hot, Unsigned> = <Bits<8, Hot, Unsigned> as Identity>::ZERO;
const _BITS8_ONE: Bits<8, Hot, Unsigned> = <Bits<8, Hot, Unsigned> as Identity>::ONE;
const _BITS8_MIN: Bits<8, Hot, Unsigned> = <Bits<8, Hot, Unsigned> as Bounded>::MIN;
const _BITS8_MAX: Bits<8, Hot, Unsigned> = <Bits<8, Hot, Unsigned> as Bounded>::MAX;

const _BITS16_ZERO: Bits<16, Hot, Unsigned> = <Bits<16, Hot, Unsigned> as Identity>::ZERO;
const _BITS32_ZERO: Bits<32, Hot, Unsigned> = <Bits<32, Hot, Unsigned> as Identity>::ZERO;
const _BITS64_ZERO: Bits<64, Hot, Unsigned> = <Bits<64, Hot, Unsigned> as Identity>::ZERO;
const _BITS128_ZERO: Bits<128, Hot, Unsigned> = <Bits<128, Hot, Unsigned> as Identity>::ZERO;
const _BITS256_ZERO: Bits<256, Warm, Unsigned> = <Bits<256, Warm, Unsigned> as Identity>::ZERO;

const _BITS_SIGNED_ZERO: Bits<32, Hot, Signed> = <Bits<32, Hot, Signed> as Identity>::ZERO;
const _BITS_SIGNED_ONE: Bits<32, Hot, Signed> = <Bits<32, Hot, Signed> as Identity>::ONE;

const _BITS_BIT_ACCESS_PROBE: () = {
    type B = Bits<32, Hot, Unsigned>;
    let zero = <B as Identity>::ZERO;
    let one = <B as BitAccess>::with_bit_set(zero, USize(3));
    assert!(<B as BitAccess>::bit(one, USize(3)).0);
    assert!(!<B as BitAccess>::bit(one, USize(0)).0);
    let cleared = <B as BitAccess>::with_bit_cleared(one, USize(3));
    assert!(!<B as BitAccess>::bit(cleared, USize(3)).0);
};

const _BITS_BIT_SEQUENCE_PROBE: () = {
    type B = Bits<64, Hot, Unsigned>;
    let zero = <B as Identity>::ZERO;
    assert!(<B as BitSequence>::is_zero(zero).0);
    let one = <B as BitAccess>::with_bit_set(zero, USize(0));
    let two_bits =
        <B as BitAccess>::with_bit_set(<B as BitAccess>::with_bit_set(zero, USize(0)), USize(1));
    assert!(<B as BitSequence>::count_ones(one).0 == 1);
    assert!(<B as BitSequence>::count_ones(two_bits).0 == 2);
    assert!(<B as BitSequence>::trailing_zeros(one).0 == 0);
};

const _BITS_BIT_LOGIC_PROBE: () = {
    type B = Bits<16, Hot, Unsigned>;
    let zero = <B as Identity>::ZERO;
    let bit0 = <B as BitAccess>::with_bit_set(zero, USize(0));
    let bit1 = <B as BitAccess>::with_bit_set(zero, USize(1));
    let either = <B as BitLogic>::bitor(bit0, bit1);
    assert!(<B as BitSequence>::count_ones(either).0 == 2);
    let both = <B as BitLogic>::bitand(bit0, bit1);
    assert!(<B as BitSequence>::is_zero(both).0);
    let xor = <B as BitLogic>::bitxor(bit0, bit1);
    assert!(<B as BitSequence>::count_ones(xor).0 == 2);
    let inv = <B as BitLogic>::bitnot(zero);
    assert!(<B as BitSequence>::count_ones(inv).0 == 16);
};

const _BITS256_LEADING_ZEROS_PROBE: () = {
    type B = Bits<256, Warm, Unsigned>;
    let zero = <B as Identity>::ZERO;
    let _ = <B as BitSequence>::leading_zeros(zero);
};

#[test]
fn bits_const_runtime_parity() {
    type B32 = Bits<32, Hot, Unsigned>;
    let zero = <B32 as Identity>::ZERO;
    let max = <B32 as Bounded>::MAX;
    assert_eq!(<B32 as BitSequence>::is_zero(zero).0, true);
    assert_eq!(<B32 as BitSequence>::count_ones(max).0, 32);

    type B64 = Bits<64, Hot, Unsigned>;
    let one = <B64 as Identity>::ONE;
    assert_eq!(<B64 as BitAccess>::bit(one, USize(0)).0, true);
}
