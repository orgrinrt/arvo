//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at the ends of every source width from nine to 64.
//!
//! The position is affine and increasing in the slot, so the range's ends are
//! where the carry decision is made. At each source width of both families onto
//! 64-bit targets at several exponents, at phases on both sides and between
//! radix ten and radix two, the verdict is checked against an independent
//! reading of the two ends, and the step at and beside the ends and around zero
//! against an oracle in `Big`. The step is read without forcing the obligation,
//! so the pairs it holds out are swept too.

use notko::Maybe;

use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Remainder};
use crate::conversion::{converted_position, is_convertible, unforced_position};
use crate::format::{Format, greatest_common_divisor_unsigned, radix};
use crate::quantum::{Constant, Quantum};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::big::Big;
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;

/// Where the oracle puts a converted member.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Answer {
    /// Past the end of the index, on the negative side when it says so.
    Past(bool),
    /// A position the index holds.
    At(Exact),
}

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
/// by `Big`'s own division: an exact position, or past the index on its sign.
/// Nothing where the denominator passes `2^128 - 1`.
fn oracle(f: &Reading, t: &Reading, a: i128) -> Maybe<Answer> {
    let fall = Big::of(f.d)
        .times(Big::power(f.r, f.down))
        .times(Big::power(t.r, t.up));
    let Maybe::Is(den) = fall.times(Big::of(t.d)).unsigned() else {
        return Maybe::Isnt;
    };
    let numerator = Big::of(a)
        .times(Big::of(f.d))
        .plus(Big::of(f.n))
        .times(Big::power(f.r, f.up))
        .times(Big::power(t.r, t.down))
        .times(Big::of(t.d))
        .plus(Big::of(-t.n).times(fall));
    let (quotient, rem) = numerator.floor_over(den);
    let Maybe::Is(slot) = quotient.index() else {
        return Maybe::Is(Answer::Past(quotient.is_negative()));
    };
    if rem == 0 {
        return Maybe::Is(Answer::At(Exact::on_grid(Slot::at(slot))));
    }
    let g = greatest_common_divisor_unsigned(rem, den);
    Maybe::Is(Answer::At(Exact::at(
        Slot::at(slot),
        Remainder::of(rem / g, den / g),
    )))
}

/// The answer as the step reports it: past an end is the end, on the grid.
fn reported(answer: Answer) -> Exact {
    match answer {
        Answer::Past(negative) => {
            Exact::on_grid(Slot::at(if negative { i128::MIN } else { i128::MAX }))
        },
        Answer::At(at) => at,
    }
}

/// Whether the oracle holds the position, one slot past its floor included when
/// it is not on the grid.
fn held(answer: Answer) -> bool {
    match answer {
        Answer::Past(_) => false,
        Answer::At(x) => x.is_on_grid().get() || x.slot().index() < i128::MAX,
    }
}

/// What the sweep found.
#[derive(Default)]
struct Edges {
    pairs:    u64,
    held_out: u64,
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
        let Maybe::Is(low) = oracle(&f, &t, min) else {
            panic!("the sweep stays where the denominator fits");
        };
        let Maybe::Is(high) = oracle(&f, &t, max) else {
            panic!("the sweep stays where the denominator fits");
        };
        let admitted = held(low) && held(high);
        assert_eq!(is_convertible::<F, T>().get(), admitted, "[{min}, {max}]");
        self.held_out += u64::from(!admitted);
        let all = [min, min + 1, -1, 0, 1, max - 1, max];
        for i in 0 .. all.len() {
            let a = all[i];
            if !(min ..= max).contains(&a) || all[.. i].contains(&a) {
                continue;
            }
            let Maybe::Is(answer) = oracle(&f, &t, a) else {
                panic!("the sweep stays where the denominator fits");
            };
            let Maybe::Is(got) = unforced_position::<F, T>(Slot::at(a)) else {
                panic!("the step's constants are formed where the oracle's are");
            };
            assert_eq!(got, reported(answer), "{a} over [{min}, {max}]");
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
    assert_eq!(walk.pairs, 6 * 56 * 2);
    // Held out: the unsigned 64-bit range onto `2^-64`, whose top lands at
    // `2^128 - 2^64`; the signed range's ends land at `-2^127` and under `2^127`.
    assert_eq!(walk.held_out, 1);
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
