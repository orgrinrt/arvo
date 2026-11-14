//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at the ends of every source width from nine to 64.
//!
//! The position is affine and increasing in the slot, so the range's ends are
//! where the step is decided to be formed. At each source width of both families onto
//! 64-bit targets at several exponents, at phases on both sides and between
//! radix ten and radix two, the verdict is checked to admit every pair, which an
//! oracle in `Big` forming both ends confirms, and the step at and beside the
//! ends and around zero against that oracle, past the slot index too. The step is read without forcing the obligation,
//! so the pairs it holds out are swept too.

use notko::Maybe;

use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Remainder};
use crate::conversion::{converted_position, is_convertible, unforced_position};
use crate::format::{Format, radix};
use crate::quantum::{Constant, Quantum};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::big::Big;
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;
use crate::wide::Wide;

/// The oracle's reading of one format: the phase in lowest terms, the radix,
/// and the moves up and down.
struct Reading {
    n:    i128,
    d:    i128,
    r:    i128,
    up:   u32,
    down: u32,
}

fn reading<F: Format>() -> Reading {
    let phase = Ratio::phase_of::<F>();
    let e = <F::Quantum as Quantum>::BASE.power();
    Reading {
        n:    phase.num,
        d:    phase.den,
        r:    i128::from(radix::<F>().base()),
        up:   if e > 0 { e.unsigned_abs() } else { 0 },
        down: if e < 0 { e.unsigned_abs() } else { 0 },
    }
}

/// `(n_F + a d_F) U_F V_T d_T - n_T d_F V_F U_T` over `d_F V_F U_T d_T`, floored
/// by `Big`'s own division into the exact position, its floor whole.
fn oracle(f: &Reading, t: &Reading, a: i128) -> Exact {
    let fall = Big::of(f.d)
        .times(Big::power(f.r, f.down))
        .times(Big::power(t.r, t.up));
    let den = fall.times(Big::of(t.d));
    let numerator = Big::of(a)
        .times(Big::of(f.d))
        .plus(Big::of(f.n))
        .times(Big::power(f.r, f.up))
        .times(Big::power(t.r, t.down))
        .times(Big::of(t.d))
        .plus(Big::of(-t.n).times(fall));
    let (quotient, rem) = numerator.floor_over_big(den);
    if rem == Big::of(0) {
        return Exact::floored(quotient.wide(), Remainder::ZERO);
    }
    let g = rem.common_divisor(den);
    let (num, _) = rem.floor_over_big(g);
    let (den, _) = den.floor_over_big(g);
    Exact::floored(quotient.wide(), Remainder::reduced(num.wide(), den.wide()))
}

/// What the sweep found.
#[derive(Default)]
struct Edges {
    pairs:    u64,
    points:   u64,
    off_grid: u64,
}

impl Edges {
    #[inline(never)]
    fn run<F: Format, T: Format>(&mut self) {
        let min = <F::Slots as Slots>::MIN.index();
        let max = <F::Slots as Slots>::MAX.index();
        let (f, t) = (reading::<F>(), reading::<T>());
        self.pairs += 1;
        assert!(is_convertible::<F, T>().get(), "[{min}, {max}]");
        let all = [min, min + 1, -1, 0, 1, max - 1, max];
        for i in 0 .. all.len() {
            let a = all[i];
            if !(min ..= max).contains(&a) || all[.. i].contains(&a) {
                continue;
            }
            let answer = oracle(&f, &t, a);
            let Maybe::Is(got) = unforced_position::<F, T>(Slot::at(a)) else {
                panic!("the step's constants are formed where the oracle's are");
            };
            assert_eq!(got, answer, "{a} over [{min}, {max}]");
            self.points += 1;
            self.off_grid += u64::from(!got.is_on_grid().get());
        }
    }
}

/// Every source width from nine to 64 of both families onto one target.
macro_rules! every_width {
    ($walk:ident; $a:ty, $e:literal, $n:literal, $d:literal => $t:ty) => {
        every_width!(@ $walk; $a, $e, $n, $d, $t;
            9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
            53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64);
    };
    (@ $walk:ident; $a:ty, $e:literal, $n:literal, $d:literal, $t:ty; $($w:literal),+) => {
        $(
            $walk.run::<Grid<$a, Constant<{ $e }>, Signed<$w>, { $n }, { $d }>, $t>();
            $walk.run::<Grid<$a, Constant<{ $e }>, Unsigned<$w>, { $n }, { $d }>, $t>();
        )+
    };
}

/// A binary 64-bit signed target at the quantum `2^E` and zero phase.
type Target<const E: i32> = Grid<BinaryRationals, Constant<E>, Signed<64>, 0, 1>;

#[test]
fn the_conversion_is_exact_and_its_verdict_true_at_the_ends_of_every_width_from_nine_to_64() {
    let mut walk = Edges::default();
    every_width!(walk; BinaryRationals, 0, 0, 1 => Target<-60>);
    every_width!(walk; BinaryRationals, 0, 0, 1 => Target<-64>);
    every_width!(walk; BinaryRationals, -40, 0, 1 => Target<20>);
    every_width!(walk; BinaryRationals, 0, 1, 3 => Grid<BinaryRationals, Constant<-2>, Signed<64>, -1, 7>);
    every_width!(walk; DecimalRationals, -18, 0, 1 => Target<-60>);
    every_width!(walk; BinaryRationals, -30, 0, 1 => Grid<DecimalRationals, Constant<-9>, Signed<64>, 2, 3>);
    // `D = 2^240`, a divisor of four limbs.
    every_width!(walk; BinaryRationals, -200, 0, 1 => Target<40>);
    assert_eq!(walk.pairs, 7 * 56 * 2);
    // The unsigned 64-bit range onto `2^-64` is among them, its top landing at
    // `2^128 - 2^64`, past the slot index, and admitted.
    assert!(walk.points > walk.pairs * 5);
    assert!(walk.off_grid > 0, "no remainder was exercised");
}

#[test]
fn the_ends_land_where_the_oracle_puts_them_at_the_deepest_target() {
    type Source = Grid<BinaryRationals, Constant<0>, Signed<64>, 0, 1>;
    let bottom = converted_position::<Source, Target<-64>>(Slot::at(i128::from(i64::MIN)));
    assert_eq!(bottom, Exact::on_grid(Slot::at(i128::MIN)));
    let top = converted_position::<Source, Target<-64>>(Slot::at(i128::from(i64::MAX)));
    assert_eq!(top, Exact::on_grid(Slot::at((i128::from(i64::MAX)) << 64)));
}

#[test]
fn the_denominator_stops_at_the_four_limbs() {
    // From `2^-128` onto `2^127` the denominator is `2^255`, and onto `2^128` it
    // is `2^256`, which the four limbs do not form.
    type Source = Grid<BinaryRationals, Constant<-128>, Unsigned<8>, 0, 1>;
    type Last = Grid<BinaryRationals, Constant<127>, Unsigned<8>, 0, 1>;
    type Past = Grid<BinaryRationals, Constant<128>, Unsigned<8>, 0, 1>;
    assert!(is_convertible::<Source, Last>().get());
    assert!(!is_convertible::<Source, Past>().get());
    assert!(matches!(
        unforced_position::<Source, Past>(Slot::at(1)),
        Maybe::Isnt
    ));
    let (f, t) = (reading::<Source>(), reading::<Last>());
    for a in [0, 1, 2, 3, 128, 255] {
        assert_eq!(
            converted_position::<Source, Last>(Slot::at(a)),
            oracle(&f, &t, a),
            "{a}"
        );
    }
    // `255 / 2^255` is already in lowest terms, and `128 / 2^255` is `1 / 2^248`.
    assert!(
        !converted_position::<Source, Last>(Slot::at(255))
            .is_on_grid()
            .get()
    );
}

#[test]
fn a_numerator_past_the_four_limbs_is_a_step_not_formed_rather_than_past_the_index() {
    // From the unsigned 64-bit range at `2^e` onto the integers, `D` is one and
    // the top member's numerator is `(2^64 - 1) 2^e`: formed at `e = 192`, where
    // the position leaves the index and is admitted all the same, and not at
    // `193`, where the step is what fails.
    fn first_failing(verdicts: [crate::width::Bool; 3]) -> Maybe<usize> {
        match verdicts.iter().position(|v| !v.get()) {
            Some(i) => Maybe::Is(i),
            None => Maybe::Isnt,
        }
    }
    type Integers = Grid<BinaryRationals, Constant<0>, Signed<64>, 0, 1>;
    type Formed = Grid<BinaryRationals, Constant<192>, Unsigned<64>, 0, 1>;
    type Unformed = Grid<BinaryRationals, Constant<193>, Unsigned<64>, 0, 1>;
    assert!(first_failing(crate::conversion::conditions::<Formed, Integers>()) == Maybe::Isnt);
    assert!(first_failing(crate::conversion::conditions::<Unformed, Integers>()) == Maybe::Is(2));
    // The formed one lands its top exactly, past the index. The unformed one
    // saturates at the end of the four limbs, the step answering for a member the
    // obligation holds out.
    let top = Slot::at(i128::from(u64::MAX));
    let (f, t) = (reading::<Formed>(), reading::<Integers>());
    assert!(matches!(
        unforced_position::<Formed, Integers>(top),
        Maybe::Is(x) if x == oracle(&f, &t, top.index())
    ));
    assert!(matches!(
        unforced_position::<Unformed, Integers>(top),
        Maybe::Is(x) if x == Exact::floored(Wide::end(false), Remainder::ZERO)
    ));
}
