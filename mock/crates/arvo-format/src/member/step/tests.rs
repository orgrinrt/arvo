//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The member step held to the slot step's position, and the bound at its edge.
//!
//! The step is a second formation of a position the slot steps already form, so
//! the law it owes is equality with them: the same floor and a remainder of the
//! same ratio, for every pair of members at six bits and at the 64-bit corners
//! either side of each edge. Each sweep first shows its format takes the host's
//! integer, so the equality is a claim about the arm it names.

use core::marker::PhantomData;

use super::{Crossing, Sides};
use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::ambient::{Ambient, BinaryRationals, DecimalRationals, Radix, UnsignedBinaryRationals};
use crate::apply::{Dither, Exact};
use crate::conversion::converted_position;
use crate::division::{Divisor, quotient_position};
use crate::format::{Format, Phase};
use crate::member::Member;
use crate::multiplication::product_position;
use crate::overflow::{Clamp, Saturate, Wrap};
use crate::points::{Fixed, UFixed};
use crate::quantum::Constant;
use crate::rounding::HalfEven;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::width::Bool;

pub struct UnsignedDecimalRationals;

impl Ambient for UnsignedDecimalRationals {
    const RADIX: Radix = Radix::DECIMAL;
    const SIGNED: Bool = Bool::FALSE;
}

pub struct TernaryRationals;

impl Ambient for TernaryRationals {
    const RADIX: Radix = Radix::of(3);
    const SIGNED: Bool = Bool::TRUE;
}

pub struct UnsignedTernaryRationals;

impl Ambient for UnsignedTernaryRationals {
    const RADIX: Radix = Radix::of(3);
    const SIGNED: Bool = Bool::FALSE;
}

/// A format over an ambient and a range at a constant exponent and a zero phase.
pub struct Scaled<A, Sl, const EXP: i32>(PhantomData<(A, Sl)>);

impl<A: Ambient, Sl: Slots, const EXP: i32> Format for Scaled<A, Sl, EXP> {
    type Ambient = A;
    type Quantum = Constant<EXP>;
    type Slots = Sl;

    const PHASE: Phase = Phase::ZERO;
}

/// The member at `at`, which the caller keeps in the range.
fn member<F: Format>(at: i128) -> Member<F> {
    Member {
        slot:   Slot::at(at),
        format: PhantomData,
    }
}

/// Every slot of `F`'s range.
fn slots<F: Format>() -> impl Iterator<Item = i128> + Clone {
    <F::Slots as Slots>::MIN.index() ..= <F::Slots as Slots>::MAX.index()
}

/// The step's product, quotient and conversion against the slot steps, over every
/// pair the iterator gives, onto the three targets for the conversion.
fn against<F: Format, D: Format, U: Format>(
    what: &'static str,
    at: &'static str,
    of: impl Iterator<Item = i128> + Clone,
) {
    for a in of.clone() {
        let m = member::<F>(a);
        for b in of.clone() {
            let pair = (a, b);
            assert!(
                m.product(member::<F>(b))
                    .same(product_position::<F>(Slot::at(a), Slot::at(b))),
                "{what} at {at}: product of {pair:?}"
            );
            if b != 0 {
                let by = Divisor::<F>::assumed(Slot::at(b));
                assert!(
                    m.quotient(by).same(quotient_position::<F>(Slot::at(a), by)),
                    "{what} at {at}: quotient of {pair:?}"
                );
            }
        }
        assert!(
            m.converted::<D>()
                .same(converted_position::<F, D>(Slot::at(a))),
            "{what} at {at}: {a} converted down"
        );
        assert!(
            m.converted::<U>()
                .same(converted_position::<F, U>(Slot::at(a))),
            "{what} at {at}: {a} converted up"
        );
        assert!(
            m.converted::<F>()
                .same(converted_position::<F, F>(Slot::at(a))),
            "{what} at {at}: {a} converted onto itself"
        );
    }
}

/// The sweep over every pair of `F`, after showing `F` takes the host's integer
/// for all three.
fn hosted<F: Format, D: Format, U: Format>(what: &'static str, at: &'static str) {
    assert!(
        Sides::<F>::PRODUCT.get(),
        "{what} at {at}: the product takes the four limbs"
    );
    assert!(
        Sides::<F>::QUOTIENT.get(),
        "{what} at {at}: the quotient takes the four limbs"
    );
    assert!(
        Crossing::<F, D>::HOLDS.get(),
        "{what} at {at}: the conversion down takes the four limbs"
    );
    assert!(
        Crossing::<F, U>::HOLDS.get(),
        "{what} at {at}: the conversion up takes the four limbs"
    );
    against::<F, D, U>(what, at, slots::<F>());
}

/// One ambient and range at six bits, at exponents -5, -1, 0 and 2, converted
/// down three and up four.
fn at_six_bits<A: Ambient, Sl: Slots>(what: &'static str) {
    hosted::<Scaled<A, Sl, -5>, Scaled<A, Sl, -8>, Scaled<A, Sl, -1>>(what, "-5");
    hosted::<Scaled<A, Sl, -1>, Scaled<A, Sl, -4>, Scaled<A, Sl, 3>>(what, "-1");
    hosted::<Scaled<A, Sl, 0>, Scaled<A, Sl, -3>, Scaled<A, Sl, 4>>(what, "0");
    hosted::<Scaled<A, Sl, 2>, Scaled<A, Sl, -1>, Scaled<A, Sl, 6>>(what, "2");
}

#[test]
fn every_pair_at_six_bits_is_the_slot_steps_position() {
    at_six_bits::<BinaryRationals, Signed<6>>("BinaryRationals");
    at_six_bits::<UnsignedBinaryRationals, Unsigned<6>>("UnsignedBinaryRationals");
    at_six_bits::<DecimalRationals, Signed<6>>("DecimalRationals");
    at_six_bits::<UnsignedDecimalRationals, Unsigned<6>>("UnsignedDecimalRationals");
    at_six_bits::<TernaryRationals, Signed<6>>("TernaryRationals");
    at_six_bits::<UnsignedTernaryRationals, Unsigned<6>>("UnsignedTernaryRationals");
}

/// The ends of `F`'s range, their neighbours, and the three slots around zero
/// that the range holds.
fn corners<F: Format>() -> impl Iterator<Item = i128> + Clone {
    let lo = <F::Slots as Slots>::MIN.index();
    let hi = <F::Slots as Slots>::MAX.index();
    [lo, lo + 1, -1, 0, 1, hi - 1, hi]
        .into_iter()
        .filter(move |at| lo <= *at && *at <= hi)
}

/// The corners of `F` against the slot steps, converted onto the grid one
/// exponent down and one up.
fn ring_at_the_corners<F: Format, D: Format, U: Format>(what: &'static str) {
    against::<F, D, U>(what, "the corners", corners::<F>());
}

#[test]
fn the_64_bit_corners_are_the_slot_steps_position_either_side_of_the_edge() {
    // The product's edge at radix two is the exponent -127, past which `2^128`
    // is no 128-bit integer; the quotient's is -63 in both ranges.
    ring_at_the_corners::<Fixed<64, -1>, Fixed<64, -2>, Fixed<64, 0>>("signed at -1");
    ring_at_the_corners::<UFixed<64, -1>, UFixed<64, -2>, UFixed<64, 0>>("unsigned at -1");
    ring_at_the_corners::<Fixed<64, 0>, Fixed<64, -1>, Fixed<64, 1>>("signed at 0");
    ring_at_the_corners::<UFixed<64, 0>, UFixed<64, -1>, UFixed<64, 1>>("unsigned at 0");
    ring_at_the_corners::<Fixed<64, -63>, Fixed<64, -64>, Fixed<64, -62>>(
        "signed at -63, the quotient's edge",
    );
    ring_at_the_corners::<UFixed<64, -63>, UFixed<64, -64>, UFixed<64, -62>>(
        "unsigned at -63, the quotient's edge",
    );
    ring_at_the_corners::<Fixed<64, -64>, Fixed<64, -65>, Fixed<64, -63>>(
        "signed at -64, the quotient past its edge",
    );
    ring_at_the_corners::<UFixed<64, -64>, UFixed<64, -65>, UFixed<64, -63>>(
        "unsigned at -64, the quotient past its edge",
    );
    ring_at_the_corners::<Fixed<64, -127>, Fixed<64, -128>, Fixed<64, -126>>(
        "signed at -127, the product's edge",
    );
    ring_at_the_corners::<Fixed<64, -128>, Fixed<64, -129>, Fixed<64, -127>>(
        "signed at -128, the product past its edge",
    );
}

#[test]
fn the_64_bit_corners_at_radix_ten_and_three_are_the_slot_steps_position() {
    // The 128-bit division at the radixes the six-bit sweep only reaches with
    // terms that fit 64 bits: either side of the quotient's edge in each range,
    // and at the product's. Past the product's edge both sides are the slot
    // step, and the remainders' cross product `same` reads passes four limbs.
    ring_at_the_corners::<Ten<Signed<64>, -1>, Ten<Signed<64>, -2>, Ten<Signed<64>, 0>>(
        "decimal signed at -1",
    );
    ring_at_the_corners::<UTen<Unsigned<64>, -1>, UTen<Unsigned<64>, -2>, UTen<Unsigned<64>, 0>>(
        "decimal unsigned at -1",
    );
    ring_at_the_corners::<Ten<Signed<64>, -19>, Ten<Signed<64>, -20>, Ten<Signed<64>, -18>>(
        "decimal signed at -19, the quotient's edge",
    );
    ring_at_the_corners::<Ten<Signed<64>, -20>, Ten<Signed<64>, -21>, Ten<Signed<64>, -19>>(
        "decimal signed at -20, the quotient past its edge",
    );
    ring_at_the_corners::<UTen<Unsigned<64>, -18>, UTen<Unsigned<64>, -19>, UTen<Unsigned<64>, -17>>(
        "decimal unsigned at -18, the quotient's edge",
    );
    ring_at_the_corners::<Ten<Signed<64>, -38>, Ten<Signed<64>, -39>, Ten<Signed<64>, -37>>(
        "decimal signed at -38, the product's edge",
    );
    ring_at_the_corners::<Three<Signed<64>, -1>, Three<Signed<64>, -2>, Three<Signed<64>, 0>>(
        "ternary signed at -1",
    );
    ring_at_the_corners::<
        UThree<Unsigned<64>, -1>,
        UThree<Unsigned<64>, -2>,
        UThree<Unsigned<64>, 0>,
    >("ternary unsigned at -1");
    ring_at_the_corners::<Three<Signed<64>, -40>, Three<Signed<64>, -41>, Three<Signed<64>, -39>>(
        "ternary signed at -40, the quotient's edge",
    );
    ring_at_the_corners::<
        UThree<Unsigned<64>, -40>,
        UThree<Unsigned<64>, -41>,
        UThree<Unsigned<64>, -39>,
    >("ternary unsigned at -40, the quotient past its edge");
    ring_at_the_corners::<Three<Signed<64>, -80>, Three<Signed<64>, -81>, Three<Signed<64>, -79>>(
        "ternary signed at -80, the product's edge",
    );
}

/// The edges in arithmetic of the tests' own, apart from the step's.
mod edge {
    /// The widest magnitude of a range of `w` bits.
    pub const fn widest(w: u32, signed: bool) -> u128 {
        if signed { 1 << (w - 1) } else { (1 << w) - 1 }
    }

    /// The largest `k` at which `m radix^k` is still below `2^127`, for an `m`
    /// other than zero.
    pub const fn below_2_127(m: u128, radix: u128) -> i32 {
        let mut at = m;
        let mut k = 0;
        loop {
            match at.checked_mul(radix) {
                Some(next) if next < 1 << 127 => {
                    at = next;
                    k += 1;
                },
                _ => return k,
            }
        }
    }

    /// The largest `k` at which `radix^k` is still a 128-bit unsigned integer.
    pub const fn power_of(radix: u128) -> i32 {
        held(1, radix)
    }

    /// The largest `k` at or above zero at which the square of `m` times
    /// `radix^k` is below `2^127`, or -1 where the square alone is not.
    pub const fn square_below_2_127(m: u128, radix: u128) -> i32 {
        match m.checked_mul(m) {
            Some(square) if square < 1 << 127 => below_2_127(square, radix),
            _ => -1,
        }
    }

    /// `k`, or zero where `k` is below it.
    pub const fn at_least_zero(k: i32) -> i32 {
        if k < 0 { 0 } else { k }
    }

    /// The largest `k` at which `m radix^k` is still a 128-bit unsigned integer.
    pub const fn held(m: u128, radix: u128) -> i32 {
        let mut at = m;
        let mut k = 0;
        loop {
            match at.checked_mul(radix) {
                Some(next) => {
                    at = next;
                    k += 1;
                },
                None => return k,
            }
        }
    }
}

/// Where the product leaves the host's integer: where `r^-e` leaves 128 bits.
macro_rules! product_edge {
    ($radix:literal) => {
        -edge::power_of($radix)
    };
}

/// Where the quotient and the conversion leave the host's integer: where the
/// widest member times `r^-e` reaches `2^127`.
macro_rules! host_edge {
    ($w:literal, $signed:literal, $radix:literal) => {
        -edge::below_2_127(edge::widest($w, $signed), $radix)
    };
}

/// The product, the quotient and the conversion from exponent zero, each
/// taking the host's integer at the last exponent the tests' arithmetic puts
/// inside it and not one past it, over one ambient and range.
///
/// The product's denominator is `r^-e` and the square of a member up to 64
/// bits over it stays below `2^127`, so its edge is where the power leaves 128
/// bits. The quotient's numerator and the conversion's scaled range are the
/// widest member times `r^-e`, which has to stay below `2^127`.
macro_rules! at_the_edge {
    ($ambient:ty, $slots:ident, $w:literal, $signed:literal, $radix:literal) => {{
        let p = product_edge!($radix);
        let q = host_edge!($w, $signed, $radix);
        type At<const E: i32> = Scaled<$ambient, $slots<$w>, E>;
        let at = ($w, $signed, $radix);
        assert!(
            Sides::<At<{ product_edge!($radix) }>>::PRODUCT.get(),
            "product refused at {p}, {at:?}"
        );
        assert!(
            !Sides::<At<{ product_edge!($radix) - 1 }>>::PRODUCT.get(),
            "product past {p}, {at:?}"
        );
        assert!(
            Sides::<At<{ host_edge!($w, $signed, $radix) }>>::QUOTIENT.get(),
            "quotient refused at {q}, {at:?}"
        );
        assert!(
            !Sides::<At<{ host_edge!($w, $signed, $radix) - 1 }>>::QUOTIENT.get(),
            "quotient past {q}, {at:?}"
        );
        assert!(
            Crossing::<At<0>, At<{ host_edge!($w, $signed, $radix) }>>::HOLDS.get(),
            "conversion refused at {q}, {at:?}"
        );
        assert!(
            !Crossing::<At<0>, At<{ host_edge!($w, $signed, $radix) - 1 }>>::HOLDS.get(),
            "conversion past {q}, {at:?}"
        );
        above_zero!($ambient, $slots, $w, $signed, $radix);
    }};
}

/// The same three bounds from exponent zero upward, where `u = r^e` and `v` is one.
///
/// The product's numerator is the square of the widest member times `r^e`,
/// which has to stay below `2^127`; a range whose square is not already there
/// at zero takes the four limbs at every exponent above it. The quotient's
/// denominator is the widest member times `r^e`, which has to stay a 128-bit
/// integer, while its ratio, at most the widest member, stays below `2^127`.
/// The conversion from `e` onto zero scales by `r^e`, which the widest member
/// times it has to keep below `2^127`, and the conversion from zero onto `e`
/// divides by `r^e`, which has to stay a 128-bit integer.
macro_rules! above_zero {
    ($ambient:ty, $slots:ident, $w:literal, $signed:literal, $radix:literal) => {{
        type At<const E: i32> = Scaled<$ambient, $slots<$w>, E>;
        let at = ($w, $signed, $radix);
        let p = edge::square_below_2_127(edge::widest($w, $signed), $radix);
        assert_eq!(
            Sides::<At<{
                edge::at_least_zero(edge::square_below_2_127(edge::widest($w, $signed), $radix))
            }>>::PRODUCT.get(),
            p >= 0,
            "product above zero at {p}, {at:?}"
        );
        assert!(
            !Sides::<At<{ edge::square_below_2_127(edge::widest($w, $signed), $radix) + 1 }>>::PRODUCT
                .get(),
            "product past {p} above zero, {at:?}"
        );
        let q = edge::held(edge::widest($w, $signed), $radix);
        assert!(
            Sides::<At<{ edge::held(edge::widest($w, $signed), $radix) }>>::QUOTIENT.get(),
            "quotient refused at {q}, {at:?}"
        );
        assert!(
            !Sides::<At<{ edge::held(edge::widest($w, $signed), $radix) + 1 }>>::QUOTIENT.get(),
            "quotient past {q}, {at:?}"
        );
        let c = edge::below_2_127(edge::widest($w, $signed), $radix);
        assert!(
            Crossing::<At<{ edge::below_2_127(edge::widest($w, $signed), $radix) }>, At<0>>::HOLDS
                .get(),
            "conversion onto zero refused from {c}, {at:?}"
        );
        assert!(
            !Crossing::<At<{ edge::below_2_127(edge::widest($w, $signed), $radix) + 1 }>, At<0>>::HOLDS
                .get(),
            "conversion onto zero past {c}, {at:?}"
        );
        let d = edge::power_of($radix);
        assert!(
            Crossing::<At<0>, At<{ edge::power_of($radix) }>>::HOLDS.get(),
            "conversion onto {d} refused, {at:?}"
        );
        assert!(
            !Crossing::<At<0>, At<{ edge::power_of($radix) + 1 }>>::HOLDS.get(),
            "conversion onto {d} past, {at:?}"
        );
    }};
}

/// Both ranges of each width at the three radixes.
macro_rules! at_width {
    ($($w:literal),*) => {$(
        at_the_edge!(BinaryRationals, Signed, $w, true, 2);
        at_the_edge!(UnsignedBinaryRationals, Unsigned, $w, false, 2);
        at_the_edge!(DecimalRationals, Signed, $w, true, 10);
        at_the_edge!(UnsignedDecimalRationals, Unsigned, $w, false, 10);
        at_the_edge!(TernaryRationals, Signed, $w, true, 3);
        at_the_edge!(UnsignedTernaryRationals, Unsigned, $w, false, 3);
    )*};
}

#[test]
fn the_bound_holds_at_its_edge_and_not_past_it_at_every_radix_and_width() {
    at_width!(1, 8, 16, 32, 63, 64);
}

#[test]
fn the_host_arm_stops_where_the_bound_does_at_radix_two() {
    // The square of a 64-bit member over `2^127` stays below `2^127`, so the
    // product holds down to where the power itself passes 128 bits.
    assert!(Sides::<Fixed<64, -1>>::PRODUCT.get());
    assert!(Sides::<Fixed<64, -127>>::PRODUCT.get());
    assert!(!Sides::<Fixed<64, -128>>::PRODUCT.get());
    assert!(Sides::<UFixed<64, -1>>::PRODUCT.get());
    assert!(Sides::<UFixed<64, -127>>::PRODUCT.get());
    assert!(!Sides::<UFixed<64, -128>>::PRODUCT.get());
    // The unsigned square reaches `2^128 - 2^65 + 1`, past `2^127` at exponent
    // zero and above it.
    assert!(!Sides::<UFixed<64, 0>>::PRODUCT.get());
    assert!(!Sides::<UFixed<64, 1>>::PRODUCT.get());
    assert!(!Sides::<Fixed<64, 2>>::PRODUCT.get());
    assert!(Sides::<Fixed<8, 2>>::PRODUCT.get());
    // `|a| v / u` is `(2^64 - 1) 2^k` unsigned and `2^63 2^k` signed, below
    // `2^127` up to `k = 63` for both.
    assert!(Sides::<UFixed<64, -63>>::QUOTIENT.get());
    assert!(!Sides::<UFixed<64, -64>>::QUOTIENT.get());
    assert!(Sides::<Fixed<64, -63>>::QUOTIENT.get());
    assert!(!Sides::<Fixed<64, -64>>::QUOTIENT.get());
    assert!(Crossing::<UFixed<64, 0>, UFixed<64, -63>>::HOLDS.get());
    assert!(!Crossing::<UFixed<64, 0>, UFixed<64, -64>>::HOLDS.get());
    assert!(Crossing::<Fixed<64, 0>, Fixed<64, -63>>::HOLDS.get());
    assert!(!Crossing::<Fixed<64, 0>, Fixed<64, -64>>::HOLDS.get());
    assert!(Crossing::<UFixed<8, -100>, UFixed<8, 0>>::HOLDS.get());
}

type Ten<Sl, const E: i32> = Scaled<DecimalRationals, Sl, E>;
type UTen<Sl, const E: i32> = Scaled<UnsignedDecimalRationals, Sl, E>;
type Three<Sl, const E: i32> = Scaled<TernaryRationals, Sl, E>;
type UThree<Sl, const E: i32> = Scaled<UnsignedTernaryRationals, Sl, E>;

#[test]
fn the_host_arm_stops_where_the_bound_does_at_radix_ten_and_three() {
    // The product's denominator is `r^k`, held up to `10^38` and `3^80`.
    assert!(Sides::<Ten<Signed<64>, -38>>::PRODUCT.get());
    assert!(!Sides::<Ten<Signed<64>, -39>>::PRODUCT.get());
    assert!(Sides::<UTen<Unsigned<64>, -38>>::PRODUCT.get());
    assert!(!Sides::<UTen<Unsigned<64>, -39>>::PRODUCT.get());
    assert!(Sides::<Three<Signed<64>, -80>>::PRODUCT.get());
    assert!(!Sides::<Three<Signed<64>, -81>>::PRODUCT.get());
    assert!(!Sides::<UTen<Unsigned<64>, 0>>::PRODUCT.get());
    // The quotient's numerator is `|a| r^k`: below `2^127` while `r^k` is below
    // `2^64` signed and `2^63` unsigned.
    assert!(Sides::<Ten<Signed<64>, -19>>::QUOTIENT.get());
    assert!(!Sides::<Ten<Signed<64>, -20>>::QUOTIENT.get());
    assert!(Sides::<UTen<Unsigned<64>, -18>>::QUOTIENT.get());
    assert!(!Sides::<UTen<Unsigned<64>, -19>>::QUOTIENT.get());
    assert!(Sides::<Three<Signed<64>, -40>>::QUOTIENT.get());
    assert!(!Sides::<Three<Signed<64>, -41>>::QUOTIENT.get());
    assert!(Sides::<UThree<Unsigned<64>, -39>>::QUOTIENT.get());
    assert!(!Sides::<UThree<Unsigned<64>, -40>>::QUOTIENT.get());
    // The conversion from exponent zero scales by `r^k` the same way.
    assert!(Crossing::<Ten<Signed<64>, 0>, Ten<Signed<64>, -19>>::HOLDS.get());
    assert!(!Crossing::<Ten<Signed<64>, 0>, Ten<Signed<64>, -20>>::HOLDS.get());
    assert!(Crossing::<UThree<Unsigned<64>, 0>, UThree<Unsigned<64>, -39>>::HOLDS.get());
    assert!(!Crossing::<UThree<Unsigned<64>, 0>, UThree<Unsigned<64>, -40>>::HOLDS.get());
}

#[test]
fn the_terms_take_the_64_bit_division_where_they_fit_it_and_not_past() {
    assert!(Sides::<UFixed<16, -8>>::QUOTIENT_NARROW.get());
    assert!(Sides::<UFixed<16, -8>>::PRODUCT_NARROW.get());
    assert!(Sides::<UFixed<32, 0>>::PRODUCT_NARROW.get());
    assert!(!Sides::<UFixed<33, 0>>::PRODUCT_NARROW.get());
    assert!(!Sides::<UFixed<64, -1>>::QUOTIENT_NARROW.get());
    assert!(Crossing::<UFixed<16, -8>, UFixed<16, -4>>::NARROW.get());
    assert!(!Crossing::<UFixed<64, 0>, UFixed<64, -1>>::NARROW.get());
}

/// `landed` against `adapted` at a position on the grid, under `S`, over the
/// slots around both ends, around zero and far past either.
///
/// `adapt` hands a rounded position whose floor the index holds to the same
/// `complete_index` that `landed` calls, so this does not test the completion,
/// which the applied map's oracle suites hold. It holds the hand-off: that
/// rounding a position on the grid moves it nowhere, and that the guard on the
/// floor plus the step sends the slots far past either end the way `landed`
/// sends them.
fn lands_as_adapted<F: Format, S: DeclaredSignature<Format = F>>(
    what: &'static str,
    policy: &'static str,
) {
    let lo = <F::Slots as Slots>::MIN.index();
    let hi = <F::Slots as Slots>::MAX.index();
    let near = |s: i128| s - 3 ..= s + 3;
    let past = [i128::MIN, i128::MIN + 1, -(1 << 100), 1 << 100, i128::MAX - 1, i128::MAX];
    let every = near(lo).chain(near(hi)).chain(near(0)).chain(past);
    for s in every {
        let landed = Member::<F>::landed::<S>(Slot::at(s)).slot();
        let adapted = Member::<F>::adapted::<S>(Exact::on_grid(Slot::at(s)), Dither::UNUSED).slot();
        assert_eq!(landed, adapted, "{what} {policy}: slot {s}");
    }
}

/// One format under the three shipped policies.
fn under_each_policy<F: Format>(what: &'static str) {
    lands_as_adapted::<F, Signature<F, Adapt<HalfEven, Wrap>>>(what, "wrapping");
    lands_as_adapted::<F, Signature<F, Adapt<HalfEven, Saturate>>>(what, "saturating");
    lands_as_adapted::<F, Signature<F, Adapt<HalfEven, Clamp>>>(what, "clamping");
}

#[test]
fn a_slot_landed_takes_the_completion_adapt_hands_a_position_on_the_grid() {
    under_each_policy::<Fixed<1, 0>>("signed at 1 bit");
    under_each_policy::<UFixed<1, 0>>("unsigned at 1 bit");
    under_each_policy::<Fixed<6, 0>>("signed at 6 bits");
    under_each_policy::<UFixed<6, 0>>("unsigned at 6 bits");
    under_each_policy::<Fixed<8, -3>>("signed at 8 bits");
    under_each_policy::<UFixed<8, -3>>("unsigned at 8 bits");
    under_each_policy::<Fixed<64, 0>>("signed at 64 bits");
    under_each_policy::<UFixed<64, 0>>("unsigned at 64 bits");
}
