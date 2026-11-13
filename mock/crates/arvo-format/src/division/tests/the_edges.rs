//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient at the divisor's extremes, at every width from nine to 64.
//!
//! The quotient is largest in magnitude next to the pole at `b = -n / d` and
//! smallest at the far ends of the range, so the divisors swept are the range's
//! two ends, the two members nearest the pole on each side and the members
//! beside those, against dividends at and beside the range's ends and around
//! zero. At each width of both families, at radix two over four exponents, at a
//! phase and at radix ten, the verdict is checked against an independent reading
//! of the eight corners, and the step at every point against an oracle in `Big`,
//! which forms the numerator and the per-divisor denominator on pairs of the
//! host's unsigned 128-bit integers and divides by an algorithm other than the
//! step's. The step is read without forcing the obligation, so the formats it
//! holds out are swept too, and there a position past the index saturates on its
//! sign.

use notko::Maybe;

use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Remainder};
use crate::division::{is_divisible, unforced_position};
use crate::format::{Format, greatest_common_divisor_unsigned, radix};
use crate::quantum::{Constant, Quantum};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::big::Big;
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;

/// Where the oracle puts a quotient.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Answer {
    /// Past the end of the index, on the negative side when it says so.
    Past(bool),
    /// A position the index holds.
    At(Exact),
}

/// The oracle's reading of `F`: the phase in lowest terms, the radix, and the
/// moves up and down.
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

/// `n + s d`, a slot's factor.
fn factor(at: &Reading, s: i128) -> Big {
    Big::of(s).times(Big::of(at.d)).plus(Big::of(at.n))
}

/// `(phase + a) / ((phase + b) q) - phase` as the numerator
/// `d (n + a d) r^down - n (n + b d) r^up` over `d (n + b d) r^up`, the sign
/// moved onto the numerator, floored by `Big`'s own division.
fn oracle(at: &Reading, a: i128, b: i128) -> Answer {
    let (fa, fb) = (factor(at, a), factor(at, b));
    assert!(
        fb != Big::of(0),
        "the oracle divides by members that are not zero"
    );
    let numerator = Big::of(at.d)
        .times(fa)
        .times(Big::power(at.r, at.down))
        .plus(Big::of(-at.n).times(fb).times(Big::power(at.r, at.up)));
    let flip = if fb.is_negative() { Big::of(-1) } else { Big::of(1) };
    let Maybe::Is(den) = Big::of(at.d)
        .times(fb)
        .times(flip)
        .times(Big::power(at.r, at.up))
        .unsigned()
    else {
        return Answer::Past(fa.is_negative() != fb.is_negative());
    };
    let (quotient, rem) = numerator.times(flip).floor_over(den);
    let Maybe::Is(slot) = quotient.index() else {
        return Answer::Past(quotient.is_negative());
    };
    if rem == 0 {
        return Answer::At(Exact::on_grid(Slot::at(slot)));
    }
    let g = greatest_common_divisor_unsigned(rem, den);
    Answer::At(Exact::at(Slot::at(slot), Remainder::of(rem / g, den / g)))
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

/// The members nearest the pole: the smallest with `n + b d` above zero and the
/// largest with it below, found by stepping from the rounded pole rather than by
/// the step's floor division.
fn pole_neighbours(at: &Reading) -> (i128, i128) {
    let guess = -at.n / at.d;
    let mut above = guess - 2;
    while at.n + above * at.d <= 0 {
        above += 1;
    }
    let mut below = guess + 2;
    while at.n + below * at.d >= 0 {
        below -= 1;
    }
    (above, below)
}

/// The slots of `all` that are in `[min, max]` and pass `keep`, each once.
fn distinct(
    all: [i128; 8],
    min: i128,
    max: i128,
    keep: impl Fn(i128) -> bool,
) -> impl Iterator<Item = i128> {
    (0 .. all.len())
        .filter(move |&i| {
            (min ..= max).contains(&all[i]) && keep(all[i]) && !all[.. i].contains(&all[i])
        })
        .map(move |i| all[i])
}

/// The divisors swept: the range's ends, the pole's neighbours and the members
/// beside each, every one in range and not zero.
fn divisors(at: &Reading, min: i128, max: i128) -> impl Iterator<Item = i128> {
    let (above, below) = pole_neighbours(at);
    let (n, d) = (at.n, at.d);
    distinct(
        [min, min + 1, below - 1, below, above, above + 1, max - 1, max],
        min,
        max,
        move |b| n + b * d != 0,
    )
}

/// The dividends swept: at and beside the range's ends, and around zero.
fn dividends(min: i128, max: i128) -> impl Iterator<Item = i128> {
    distinct(
        [min, min + 1, -1, 0, 1, max - 1, max, max],
        min,
        max,
        |_| true,
    )
}

/// Whether the oracle holds every one of the eight corners and finds every
/// member divisor's denominator under `2^128`.
fn oracle_admits(at: &Reading, min: i128, max: i128) -> bool {
    let (above, below) = pole_neighbours(at);
    let fits = |b: i128| {
        let fb = factor(at, b);
        let flip = if fb.is_negative() { Big::of(-1) } else { Big::of(1) };
        matches!(
            Big::of(at.d)
                .times(fb)
                .times(flip)
                .times(Big::power(at.r, at.up))
                .unsigned(),
            Maybe::Is(_)
        )
    };
    if !fits(min) || !fits(max) {
        return false;
    }
    [min, max, above, below]
        .into_iter()
        .filter(|&b| (min ..= max).contains(&b) && at.n + b * at.d != 0)
        .all(|b| {
            [min, max].into_iter().all(|a| {
                match oracle(at, a, b) {
                    Answer::Past(_) => false,
                    Answer::At(x) => x.is_on_grid().get() || x.slot().index() < i128::MAX,
                }
            })
        })
}

/// What the sweep found.
#[derive(Default)]
struct Edges {
    formats:  u64,
    held_out: u64,
    points:   u64,
    past:     u64,
    off_grid: u64,
}

impl PerFormat for Edges {
    #[inline(never)]
    fn run<F: Format>(&mut self) {
        let min = <F::Slots as Slots>::MIN.index();
        let max = <F::Slots as Slots>::MAX.index();
        let at = reading::<F>();
        self.formats += 1;
        let admitted = oracle_admits(&at, min, max);
        assert_eq!(
            is_divisible::<F>().get(),
            admitted,
            "[{min}, {max}] at phase {}/{}, radix {}, up {}, down {}",
            at.n,
            at.d,
            at.r,
            at.up,
            at.down
        );
        self.held_out += u64::from(!admitted);
        for b in divisors(&at, min, max) {
            for a in dividends(min, max) {
                let answer = oracle(&at, a, b);
                let want = reported(answer);
                let Maybe::Is(got) = unforced_position::<F>(Slot::at(a), Slot::at(b)) else {
                    panic!("the sweep stays where the phase denotes");
                };
                assert_eq!(got, want, "{a} / {b} over [{min}, {max}]");
                self.points += 1;
                self.past += u64::from(matches!(answer, Answer::Past(_)));
                self.off_grid += u64::from(!want.is_on_grid().get());
            }
        }
    }
}

/// Every width from nine to 64 of both families, in one grid.
macro_rules! every_width {
    ($walk:ident; $a:ty, $e:literal, $n:literal, $d:literal) => {
        every_width!(@ $walk; $a, $e, $n, $d;
            9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
            53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64);
    };
    (@ $walk:ident; $a:ty, $e:literal, $n:literal, $d:literal; $($w:literal),+) => {
        $(
            $walk.run::<Grid<$a, Constant<{ $e }>, Signed<$w>, { $n }, { $d }>>();
            $walk.run::<Grid<$a, Constant<{ $e }>, Unsigned<$w>, { $n }, { $d }>>();
        )+
    };
}

#[test]
fn the_quotient_is_exact_at_the_divisor_s_extremes_at_every_width_from_nine_to_64() {
    let mut walk = Edges::default();
    every_width!(walk; BinaryRationals, 0, 0, 1);
    every_width!(walk; BinaryRationals, 7, 0, 1);
    every_width!(walk; BinaryRationals, -3, 0, 1);
    every_width!(walk; BinaryRationals, -64, 0, 1);
    every_width!(walk; BinaryRationals, 0, 1, 3);
    every_width!(walk; BinaryRationals, -1, -5, 7);
    every_width!(walk; DecimalRationals, -2, 0, 1);
    every_width!(walk; DecimalRationals, 3, 2, 3);
    assert_eq!(walk.formats, 8 * 56 * 2);
    // Held out at `2^-64`: the signed 64-bit range at `MIN / -1`, and the
    // unsigned 64-bit range at `MAX / 1`; the unsigned 63-bit range is held.
    assert_eq!(walk.held_out, 2);
    assert!(walk.points > walk.formats * 20);
    assert!(walk.past > 0, "no point reached past the index");
    assert!(
        walk.off_grid > walk.points / 4,
        "too few remainders to exercise"
    );
}

#[test]
fn the_quotient_by_the_divisor_next_to_the_pole_is_the_largest_position() {
    // At zero phase the pole's neighbours are `-1` and `1`, and the quotient by
    // them is the dividend scaled by `2^-e`, the largest in magnitude over the
    // divisors; at a phase of `-5/7` they are `0` and `1`.
    type Fine = Grid<BinaryRationals, Constant<-63>, Signed<64>, 0, 1>;
    type Offset = Grid<BinaryRationals, Constant<-1>, Signed<64>, -5, 7>;
    let min = i128::from(i64::MIN);
    let at = reading::<Fine>();
    assert_eq!(pole_neighbours(&at), (1, -1));
    let Maybe::Is(next) = unforced_position::<Fine>(Slot::at(min), Slot::at(-1)) else {
        panic!("the phase denotes");
    };
    assert_eq!(next, Exact::on_grid(Slot::at(1 << 126)));
    for b in divisors(&at, min, i128::from(i64::MAX)) {
        let Maybe::Is(x) = unforced_position::<Fine>(Slot::at(min), Slot::at(b)) else {
            panic!("the phase denotes");
        };
        assert!(x.slot().index().abs() <= 1 << 126, "{b}");
    }
    assert_eq!(pole_neighbours(&reading::<Offset>()), (1, 0));
}
