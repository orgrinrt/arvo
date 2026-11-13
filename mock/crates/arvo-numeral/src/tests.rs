//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The rungs against the oracle, and the map against the placement.
//!
//! Every width from 1 to 8 is computed in each of the four rungs, and every
//! width from 9 to 12 in the three rungs wide enough to carry it, both slot
//! families, both policies, every pair of members, and compared with
//! `arvo_format::add`, and the same for the difference, the negation and the
//! product at exponent zero against `sub`, `neg` and `mul`. That checks the
//! narrowest carrier against every wider one, the 64-bit carrier among them. A
//! rung's mutant feature makes these tests fail, and so does an operation's;
//! nothing here expects either.

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::slots::{Signed, Unsigned};
use arvo_format::{
    Adapt,
    DeclaredSignature,
    Dither,
    Policy,
    Signature,
    Slot,
    Slots,
    add,
    mul,
    neg,
    sub,
};

use crate::rung;

fn oracle<S: DeclaredSignature>(a: i128, b: i128) -> i128 {
    add::<S>(Slot::at(a), Slot::at(b), Dither::UNUSED).index()
}

/// Disagreements with the oracle per rung, in the order 8, 16, 32, 64.
#[derive(Default)]
struct Wrong([u64; 4]);

impl Wrong {
    fn record(&mut self, rung: usize, got: i128, want: i128) {
        if got != want {
            self.0[rung] += 1;
        }
    }

    /// Every rung agrees with the oracle on every pair it computed.
    fn check(&self, what: &'static str) {
        self.check_as("the sum", what);
    }

    /// The same, naming the operation.
    fn check_as(&self, operation: &'static str, what: &'static str) {
        for (i, &wrong) in self.0.iter().enumerate() {
            let bits = 8 << i;
            assert_eq!(
                wrong, 0,
                "{what}: the {bits}-bit rung disagrees with the oracle on {operation}"
            );
        }
    }
}

/// The three ring operations' disagreements, each per rung.
#[derive(Default)]
struct Ring {
    sub: Wrong,
    neg: Wrong,
    mul: Wrong,
}

impl Ring {
    fn check(&self, what: &'static str) {
        self.sub.check_as("the difference", what);
        self.neg.check_as("the negation", what);
        self.mul.check_as("the product", what);
    }
}

/// Every pair at width `W` in every rung that can carry it.
fn signed_width<const W: u32, O>(policy: Policy, wrong: &mut Wrong)
where
    Signed<W>: Slots,
    O: arvo_format::Overflow,
{
    type Sig<const W: u32, O> = Signature<Integer<W>, Adapt<TowardZero, O>>;
    let width = W;
    let (min, max) = (
        <Signed<W> as Slots>::MIN.index(),
        <Signed<W> as Slots>::MAX.index(),
    );
    for a in min ..= max {
        for b in min ..= max {
            let want = oracle::<Sig<W, O>>(a, b);
            if W <= 8 {
                let got = rung::add_s8(rung::from_s8(a), rung::from_s8(b), width, policy);
                wrong.record(0, rung::index_s8(got), want);
            }
            let got = rung::add_s16(rung::from_s16(a), rung::from_s16(b), width, policy);
            wrong.record(1, rung::index_s16(got), want);
            let got = rung::add_s32(rung::from_s32(a), rung::from_s32(b), width, policy);
            wrong.record(2, rung::index_s32(got), want);
            let got = rung::add_s64(rung::from_s64(a), rung::from_s64(b), width, policy);
            wrong.record(3, rung::index_s64(got), want);
        }
    }
}

/// The difference, the negation and the product of every pair at width `W` in
/// every rung that can carry it. The product is asked at exponent zero, the one
/// place `Num::mul` calls the rung.
fn signed_ring<const W: u32, O>(policy: Policy, wrong: &mut Ring)
where
    Signed<W>: Slots,
    O: arvo_format::Overflow,
{
    type Sig<const W: u32, O> = Signature<Integer<W>, Adapt<TowardZero, O>>;
    let width = W;
    let (min, max) = (
        <Signed<W> as Slots>::MIN.index(),
        <Signed<W> as Slots>::MAX.index(),
    );
    for a in min ..= max {
        let want = neg::<Sig<W, O>>(Slot::at(a), Dither::UNUSED).index();
        if W <= 8 {
            wrong.neg.record(
                0,
                rung::index_s8(rung::neg_s8(rung::from_s8(a), width, policy)),
                want,
            );
        }
        wrong.neg.record(
            1,
            rung::index_s16(rung::neg_s16(rung::from_s16(a), width, policy)),
            want,
        );
        wrong.neg.record(
            2,
            rung::index_s32(rung::neg_s32(rung::from_s32(a), width, policy)),
            want,
        );
        wrong.neg.record(
            3,
            rung::index_s64(rung::neg_s64(rung::from_s64(a), width, policy)),
            want,
        );
        for b in min ..= max {
            let want = sub::<Sig<W, O>>(Slot::at(a), Slot::at(b), Dither::UNUSED).index();
            if W <= 8 {
                let got = rung::sub_s8(rung::from_s8(a), rung::from_s8(b), width, policy);
                wrong.sub.record(0, rung::index_s8(got), want);
            }
            let got = rung::sub_s16(rung::from_s16(a), rung::from_s16(b), width, policy);
            wrong.sub.record(1, rung::index_s16(got), want);
            let got = rung::sub_s32(rung::from_s32(a), rung::from_s32(b), width, policy);
            wrong.sub.record(2, rung::index_s32(got), want);
            let got = rung::sub_s64(rung::from_s64(a), rung::from_s64(b), width, policy);
            wrong.sub.record(3, rung::index_s64(got), want);

            let want = mul::<Sig<W, O>>(Slot::at(a), Slot::at(b), Dither::UNUSED).index();
            if W <= 8 {
                let got = rung::mul_s8(rung::from_s8(a), rung::from_s8(b), width, policy);
                wrong.mul.record(0, rung::index_s8(got), want);
            }
            let got = rung::mul_s16(rung::from_s16(a), rung::from_s16(b), width, policy);
            wrong.mul.record(1, rung::index_s16(got), want);
            let got = rung::mul_s32(rung::from_s32(a), rung::from_s32(b), width, policy);
            wrong.mul.record(2, rung::index_s32(got), want);
            let got = rung::mul_s64(rung::from_s64(a), rung::from_s64(b), width, policy);
            wrong.mul.record(3, rung::index_s64(got), want);
        }
    }
}

/// The same over the unsigned family.
fn unsigned_ring<const W: u32, O>(policy: Policy, wrong: &mut Ring)
where
    Unsigned<W>: Slots,
    O: arvo_format::Overflow,
{
    type Sig<const W: u32, O> = Signature<UFixed<W, 0>, Adapt<TowardZero, O>>;
    let width = W;
    let max = <Unsigned<W> as Slots>::MAX.index();
    for a in 0 ..= max {
        let want = neg::<Sig<W, O>>(Slot::at(a), Dither::UNUSED).index();
        if W <= 8 {
            wrong.neg.record(
                0,
                rung::index_u8(rung::neg_u8(rung::from_u8(a), width, policy)),
                want,
            );
        }
        wrong.neg.record(
            1,
            rung::index_u16(rung::neg_u16(rung::from_u16(a), width, policy)),
            want,
        );
        wrong.neg.record(
            2,
            rung::index_u32(rung::neg_u32(rung::from_u32(a), width, policy)),
            want,
        );
        wrong.neg.record(
            3,
            rung::index_u64(rung::neg_u64(rung::from_u64(a), width, policy)),
            want,
        );
        for b in 0 ..= max {
            let want = sub::<Sig<W, O>>(Slot::at(a), Slot::at(b), Dither::UNUSED).index();
            if W <= 8 {
                let got = rung::sub_u8(rung::from_u8(a), rung::from_u8(b), width, policy);
                wrong.sub.record(0, rung::index_u8(got), want);
            }
            let got = rung::sub_u16(rung::from_u16(a), rung::from_u16(b), width, policy);
            wrong.sub.record(1, rung::index_u16(got), want);
            let got = rung::sub_u32(rung::from_u32(a), rung::from_u32(b), width, policy);
            wrong.sub.record(2, rung::index_u32(got), want);
            let got = rung::sub_u64(rung::from_u64(a), rung::from_u64(b), width, policy);
            wrong.sub.record(3, rung::index_u64(got), want);

            let want = mul::<Sig<W, O>>(Slot::at(a), Slot::at(b), Dither::UNUSED).index();
            if W <= 8 {
                let got = rung::mul_u8(rung::from_u8(a), rung::from_u8(b), width, policy);
                wrong.mul.record(0, rung::index_u8(got), want);
            }
            let got = rung::mul_u16(rung::from_u16(a), rung::from_u16(b), width, policy);
            wrong.mul.record(1, rung::index_u16(got), want);
            let got = rung::mul_u32(rung::from_u32(a), rung::from_u32(b), width, policy);
            wrong.mul.record(2, rung::index_u32(got), want);
            let got = rung::mul_u64(rung::from_u64(a), rung::from_u64(b), width, policy);
            wrong.mul.record(3, rung::index_u64(got), want);
        }
    }
}

/// Every pair at width `W` in every rung that can carry it.
fn unsigned_width<const W: u32, O>(policy: Policy, wrong: &mut Wrong)
where
    Unsigned<W>: Slots,
    O: arvo_format::Overflow,
{
    type Sig<const W: u32, O> = Signature<UFixed<W, 0>, Adapt<TowardZero, O>>;
    let width = W;
    let max = <Unsigned<W> as Slots>::MAX.index();
    for a in 0 ..= max {
        for b in 0 ..= max {
            let want = oracle::<Sig<W, O>>(a, b);
            if W <= 8 {
                let got = rung::add_u8(rung::from_u8(a), rung::from_u8(b), width, policy);
                wrong.record(0, rung::index_u8(got), want);
            }
            let got = rung::add_u16(rung::from_u16(a), rung::from_u16(b), width, policy);
            wrong.record(1, rung::index_u16(got), want);
            let got = rung::add_u32(rung::from_u32(a), rung::from_u32(b), width, policy);
            wrong.record(2, rung::index_u32(got), want);
            let got = rung::add_u64(rung::from_u64(a), rung::from_u64(b), width, policy);
            wrong.record(3, rung::index_u64(got), want);
        }
    }
}

macro_rules! every_narrow_width {
    ($f:ident, $o:ty, $policy:expr, $wrong:expr) => {
        $f::<1, $o>($policy, $wrong);
        $f::<2, $o>($policy, $wrong);
        $f::<3, $o>($policy, $wrong);
        $f::<4, $o>($policy, $wrong);
        $f::<5, $o>($policy, $wrong);
        $f::<6, $o>($policy, $wrong);
        $f::<7, $o>($policy, $wrong);
        $f::<8, $o>($policy, $wrong);
    };
}

macro_rules! every_wide_width {
    ($f:ident, $o:ty, $policy:expr, $wrong:expr) => {
        $f::<9, $o>($policy, $wrong);
        $f::<10, $o>($policy, $wrong);
        $f::<11, $o>($policy, $wrong);
        $f::<12, $o>($policy, $wrong);
    };
}

#[test]
fn every_rung_adds_every_signed_pair_at_widths_one_to_eight_as_the_oracle_does() {
    let mut wrong = Wrong::default();
    every_narrow_width!(signed_width, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("signed, wrapping");
    let mut wrong = Wrong::default();
    every_narrow_width!(signed_width, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("signed, saturating");
}

#[test]
fn every_rung_adds_every_unsigned_pair_at_widths_one_to_eight_as_the_oracle_does() {
    let mut wrong = Wrong::default();
    every_narrow_width!(unsigned_width, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("unsigned, wrapping");
    let mut wrong = Wrong::default();
    every_narrow_width!(unsigned_width, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("unsigned, saturating");
}

#[test]
fn every_wide_rung_adds_every_signed_pair_at_widths_nine_to_twelve_as_the_oracle_does() {
    let mut wrong = Wrong::default();
    every_wide_width!(signed_width, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("signed, wrapping");
    let mut wrong = Wrong::default();
    every_wide_width!(signed_width, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("signed, saturating");
}

#[test]
fn every_wide_rung_adds_every_unsigned_pair_at_widths_nine_to_twelve_as_the_oracle_does() {
    let mut wrong = Wrong::default();
    every_wide_width!(unsigned_width, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("unsigned, wrapping");
    let mut wrong = Wrong::default();
    every_wide_width!(unsigned_width, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("unsigned, saturating");
}

#[test]
fn every_rung_subtracts_negates_and_multiplies_signed_at_widths_one_to_eight_as_the_oracle_does() {
    let mut wrong = Ring::default();
    every_narrow_width!(signed_ring, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("signed, wrapping");
    let mut wrong = Ring::default();
    every_narrow_width!(signed_ring, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("signed, saturating");
}

#[test]
fn every_rung_subtracts_negates_and_multiplies_unsigned_at_widths_one_to_eight_as_the_oracle_does()
{
    let mut wrong = Ring::default();
    every_narrow_width!(unsigned_ring, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("unsigned, wrapping");
    let mut wrong = Ring::default();
    every_narrow_width!(unsigned_ring, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("unsigned, saturating");
}

#[test]
fn every_wide_rung_subtracts_negates_and_multiplies_signed_at_widths_nine_to_twelve_as_the_oracle_does()
 {
    let mut wrong = Ring::default();
    every_wide_width!(signed_ring, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("signed, wrapping");
    let mut wrong = Ring::default();
    every_wide_width!(signed_ring, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("signed, saturating");
}

#[test]
fn every_wide_rung_subtracts_negates_and_multiplies_unsigned_at_widths_nine_to_twelve_as_the_oracle_does()
 {
    let mut wrong = Ring::default();
    every_wide_width!(unsigned_ring, Wrap, Policy::Wrap, &mut wrong);
    wrong.check("unsigned, wrapping");
    let mut wrong = Ring::default();
    every_wide_width!(unsigned_ring, Saturate, Policy::Saturate, &mut wrong);
    wrong.check("unsigned, saturating");
}

#[test]
fn every_rung_orders_and_round_trips_the_signed_eight_bit_members() {
    for a in -128 ..= 127 {
        assert_eq!(rung::index_s8(rung::from_s8(a)), a);
        assert_eq!(rung::index_s16(rung::from_s16(a)), a);
        assert_eq!(rung::index_s32(rung::from_s32(a)), a);
        assert_eq!(rung::index_s64(rung::from_s64(a)), a);
        for b in -128 ..= 127 {
            let want = a.cmp(&b);
            assert_eq!(rung::cmp_s8(rung::from_s8(a), rung::from_s8(b)), want);
            assert_eq!(rung::cmp_s16(rung::from_s16(a), rung::from_s16(b)), want);
            assert_eq!(rung::cmp_s32(rung::from_s32(a), rung::from_s32(b)), want);
            assert_eq!(rung::cmp_s64(rung::from_s64(a), rung::from_s64(b)), want);
        }
    }
}

#[test]
fn every_rung_orders_and_round_trips_the_unsigned_eight_bit_members() {
    for a in 0 ..= 255 {
        assert_eq!(rung::index_u8(rung::from_u8(a)), a);
        assert_eq!(rung::index_u16(rung::from_u16(a)), a);
        assert_eq!(rung::index_u32(rung::from_u32(a)), a);
        assert_eq!(rung::index_u64(rung::from_u64(a)), a);
        for b in 0 ..= 255 {
            let want = a.cmp(&b);
            assert_eq!(rung::cmp_u8(rung::from_u8(a), rung::from_u8(b)), want);
            assert_eq!(rung::cmp_u16(rung::from_u16(a), rung::from_u16(b)), want);
            assert_eq!(rung::cmp_u32(rung::from_u32(a), rung::from_u32(b)), want);
            assert_eq!(rung::cmp_u64(rung::from_u64(a), rung::from_u64(b)), want);
        }
    }
}

#[test]
fn the_bit_copy_keeps_every_bit_between_integers_of_one_size() {
    use crate::machine::copy;
    for x in [i64::MIN, -1, 0, 1, i64::MAX] {
        assert_eq!(copy::<u64, i64>(copy::<i64, u64>(x)), x);
    }
    assert_eq!(copy::<i8, u8>(-1), 255);
    assert_eq!(copy::<u16, i16>(0x8000), i16::MIN);
}

#[test]
#[should_panic(expected = "a bit copy between integers of two sizes")]
fn the_bit_copy_refuses_two_sizes() {
    let _ = crate::machine::copy::<u8, u16>(1);
}
