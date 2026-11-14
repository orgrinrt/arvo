//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product at the edges of every width from nine to 64, where the member
//! sweep cannot reach and the carry decision is made.
//!
//! At each width of both families, at radix two and zero phase over seven
//! exponents, at a phase, at radix ten and at both, the verdict is checked
//! against an independent reading of the four corners, and the step at the
//! corners and the slots beside them against an oracle in `Big`, which forms the
//! numerator and the denominator on a pair of the host's unsigned 128-bit
//! integers and divides one bit at a time. There the numerator passes `2^128`
//! and the denominator is not a power of two, or passes `2^64` at `2^-127` and
//! `10^-38` and `2^128` from `2^-128` down to `2^-255`, so the step's division by
//! one, two, three and four limbs is what is compared. The step is read without
//! forcing the obligation, so the formats it does not admit are swept too, and
//! there a position past the index has to saturate on its sign. A step that keeps
//! the floor and drops the remainder is run beside it as the control: it has to
//! be wrong somewhere and never where the denominator is one.

use notko::Maybe;

use super::At;
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Fraction, Remainder};
use crate::format::{Format, radix};
use crate::multiplication::{is_multipliable, product_position, unforced_position};
use crate::points::{Integer, UFixed};
use crate::quantum::{Constant, Quantum};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::big::Big;
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;

/// Where the oracle puts a product.
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

/// The denominator `d^2 r^down`, formed in the oracle's own wide integer, which
/// holds it wherever the step forms it.
fn denominator(at: &Reading) -> Big {
    Big::of(at.d)
        .times(Big::of(at.d))
        .times(Big::power(at.r, at.down))
}

/// `(phase + a)(phase + b) * q - phase` for the slots `a` and `b`, as the
/// numerator `(n + a d)(n + b d) r^up - n d r^down` in `Big` over the
/// denominator, floored one bit at a time.
fn oracle(at: &Reading, a: i128, b: i128) -> Answer {
    let factor = |s: i128| Big::of(s).times(Big::of(at.d)).plus(Big::of(at.n));
    let term = Big::of(-(at.n * at.d)).times(Big::power(at.r, at.down));
    let numerator = factor(a)
        .times(factor(b))
        .times(Big::power(at.r, at.up))
        .plus(term);
    let den = denominator(at);
    let (quotient, rem) = numerator.floor_over_big(den);
    let Maybe::Is(slot) = quotient.index() else {
        return Answer::Past(quotient.is_negative());
    };
    if rem == Big::of(0) {
        return Answer::At(Exact::on_grid(Slot::at(slot)));
    }
    // Lowest terms by the oracle's own Euclid, so the step's reduction of a pair
    // already in lowest terms leaves it as it is.
    let g = rem.common_divisor(den);
    let (num, _) = rem.floor_over_big(g);
    let (den, _) = den.floor_over_big(g);
    Answer::At(Exact::at(
        Slot::at(slot),
        Remainder::reduced(num.wide(), den.wide()),
    ))
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

/// Whether the oracle holds every corner, one slot past its floor included
/// when it is not on the grid.
fn corners_are_held(at: &Reading, min: i128, max: i128) -> bool {
    [(min, min), (min, max), (max, min), (max, max)]
        .into_iter()
        .all(|(a, b)| {
            match oracle(at, a, b) {
                Answer::Past(_) => false,
                Answer::At(x) => x.is_on_grid().get() || x.slot().index() < i128::MAX,
            }
        })
}

/// The slots at and beside the ends of a range, and around zero, each once.
fn edges(min: i128, max: i128) -> impl Iterator<Item = i128> {
    let all = [min, min + 1, -1, 0, 1, max - 1, max];
    (0 .. all.len())
        .filter(move |&i| (min ..= max).contains(&all[i]) && !all[.. i].contains(&all[i]))
        .map(move |i| all[i])
}

/// What the sweep found.
#[derive(Default)]
struct Edges {
    formats:      u64,
    held_out:     u64,
    points:       u64,
    past:         u64,
    floor_wrong:  u64,
    floor_on_one: u64,
}

impl PerFormat for Edges {
    #[inline(never)]
    fn run<F: Format>(&mut self) {
        let min = <F::Slots as Slots>::MIN.index();
        let max = <F::Slots as Slots>::MAX.index();
        let at = reading::<F>();
        let den = denominator(&at);
        self.formats += 1;
        let held = corners_are_held(&at, min, max);
        assert_eq!(
            is_multipliable::<F>().get(),
            held,
            "[{min}, {max}] at phase {}/{}, radix {}, up {}, down {}",
            at.n,
            at.d,
            at.r,
            at.up,
            at.down
        );
        self.held_out += u64::from(!held);
        for a in edges(min, max) {
            for b in edges(min, max) {
                let answer = oracle(&at, a, b);
                let want = reported(answer);
                let Maybe::Is(got) = unforced_position::<F>(Slot::at(a), Slot::at(b)) else {
                    panic!("the sweep stays where every remainder is held");
                };
                assert_eq!(got, want, "{a} * {b} over [{min}, {max}], D = {den:?}");
                self.points += 1;
                self.past += u64::from(matches!(answer, Answer::Past(_)));
                if Exact::on_grid(want.slot()) != want {
                    self.floor_wrong += 1;
                    if den == Big::of(1) {
                        self.floor_on_one += 1;
                    }
                }
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

/// Seven edge slots signed and four unsigned, squared, at every width.
fn points_per_grid() -> u64 {
    56 * (49 + 16)
}

#[test]
fn the_product_is_exact_and_its_verdict_true_at_the_edges_of_every_width_from_nine_to_64() {
    let mut walk = Edges::default();
    every_width!(walk; BinaryRationals, 0, 0, 1);
    every_width!(walk; BinaryRationals, 1, 0, 1);
    every_width!(walk; BinaryRationals, -1, 0, 1);
    every_width!(walk; BinaryRationals, -127, 0, 1);
    // Past `2^128`: a divisor of three limbs, of four, and the last exponent the
    // four limbs form `D` at.
    every_width!(walk; BinaryRationals, -128, 0, 1);
    every_width!(walk; BinaryRationals, -200, 0, 1);
    every_width!(walk; BinaryRationals, -255, 0, 1);
    assert_eq!(walk.formats, 7 * 56 * 2);
    assert_eq!(walk.points, 7 * points_per_grid());
    // Held out: the unsigned 64-bit range at zero, and both 64-bit ranges at
    // one. The unsigned 63-bit range at one is held.
    assert_eq!(walk.held_out, 3);
    assert!(
        walk.floor_wrong > 0,
        "the floor-only step agreed everywhere"
    );
    assert_eq!(walk.floor_on_one, 0);
}

#[test]
fn the_product_is_exact_at_a_phase_and_at_radix_ten_at_every_width_from_nine_to_64() {
    let mut walk = Edges::default();
    every_width!(walk; BinaryRationals, 0, 1, 3);
    every_width!(walk; BinaryRationals, -5, -2, 3);
    every_width!(walk; DecimalRationals, -18, 0, 1);
    every_width!(walk; DecimalRationals, -38, 0, 1);
    every_width!(walk; DecimalRationals, 1, 0, 1);
    every_width!(walk; DecimalRationals, -3, 2, 7);
    // The last exponent at radix ten whose `D` the four limbs form.
    every_width!(walk; DecimalRationals, -77, 0, 1);
    assert_eq!(walk.formats, 7 * 56 * 2);
    assert_eq!(walk.points, 7 * points_per_grid());
    // Some of each: radix ten at `10^1` and a third at the 64-bit unsigned
    // range leave the index, and the rest is admitted.
    assert!(walk.held_out > 0 && walk.held_out < walk.formats);
    assert!(walk.floor_wrong > 0);
    assert_eq!(walk.floor_on_one, 0);
}

#[test]
fn a_numerator_of_four_limbs_is_placed_past_the_index_on_its_sign() {
    // At `10^20` the 64-bit corners reach about `2^194`, four limbs, which only
    // a format the obligation holds out reaches.
    let mut walk = Edges::default();
    every_width!(walk; DecimalRationals, 20, 0, 1);
    assert_eq!(walk.points, points_per_grid());
    assert!(walk.past > 0);
    assert!(walk.held_out > 0 && walk.held_out < walk.formats);
}

#[test]
fn the_named_edges_are_what_the_obligation_admits() {
    let (min, max) = (Slot::at(i64::MIN as i128), Slot::at(i64::MAX as i128));
    assert_eq!(
        product_position::<Integer<64>>(min, min),
        Exact::on_grid(Slot::at(1 << 126))
    );
    assert_eq!(
        product_position::<Integer<64>>(min, max),
        Exact::on_grid(Slot::at(-(1 << 126) + (1 << 63)))
    );
    let top63 = Slot::at((1 << 63) - 1);
    assert_eq!(
        product_position::<UFixed<63, 1>>(top63, top63),
        Exact::on_grid(Slot::at(((1i128 << 63) - 1) * ((1 << 63) - 1) * 2))
    );
    // `(2^64 - 1)^2 / 2` is `2^127 - 2^64` and a half.
    let top64 = Slot::at(u64::MAX as i128);
    assert_eq!(
        product_position::<UFixed<64, -1>>(top64, top64),
        Exact::between(Slot::at(i128::MAX - (1 << 64) + 1), Fraction::HALF)
    );
    assert_eq!(
        product_position::<UFixed<64, -62>>(top64, top64),
        reported(oracle(
            &reading::<UFixed<64, -62>>(),
            u64::MAX as i128,
            u64::MAX as i128
        ))
    );
    // At `2^-127` the denominator is two limbs, at `2^-128` three, at `2^-200`
    // and `2^-255` four, the last the limbs form.
    assert_eq!(
        product_position::<UFixed<64, -127>>(top64, top64),
        reported(oracle(
            &reading::<UFixed<64, -127>>(),
            u64::MAX as i128,
            u64::MAX as i128
        ))
    );
    assert_eq!(
        product_position::<UFixed<64, -128>>(top64, top64),
        reported(oracle(
            &reading::<UFixed<64, -128>>(),
            u64::MAX as i128,
            u64::MAX as i128
        ))
    );
    assert_eq!(
        product_position::<Integer<64>>(min, max),
        reported(oracle(&reading::<Integer<64>>(), min.index(), max.index()))
    );
    type Two200 = Grid<BinaryRationals, Constant<-200>, Signed<64>, 0, 1>;
    type Last = Grid<BinaryRationals, Constant<-255>, Signed<64>, 0, 1>;
    type Past = Grid<BinaryRationals, Constant<-256>, Signed<64>, 0, 1>;
    type DecimalLast = Grid<DecimalRationals, Constant<-77>, Signed<64>, 0, 1>;
    type DecimalPast = Grid<DecimalRationals, Constant<-78>, Signed<64>, 0, 1>;
    assert!(is_multipliable::<Two200>().get());
    assert!(is_multipliable::<Last>().get());
    assert!(is_multipliable::<DecimalLast>().get());
    // One step past, `D` leaves the limbs and the step is not formed at all.
    assert!(!is_multipliable::<Past>().get());
    assert!(!is_multipliable::<DecimalPast>().get());
    assert!(matches!(unforced_position::<Past>(min, max), Maybe::Isnt));
    assert!(matches!(
        unforced_position::<DecimalPast>(min, max),
        Maybe::Isnt
    ));
    // `MIN * MAX` at `2^-255` lies just below zero: the slot `-1` and a
    // remainder over four limbs.
    assert_eq!(
        product_position::<Last>(min, max),
        reported(oracle(&reading::<Last>(), min.index(), max.index()))
    );
    assert_eq!(
        product_position::<Two200>(min, min),
        reported(oracle(&reading::<Two200>(), min.index(), min.index()))
    );
    // The signed 64-bit range at a phase of a third, and at radix ten at
    // `10^-18` and at `10^-38`, all admitted.
    type Third = Grid<BinaryRationals, Constant<0>, Signed<64>, 1, 3>;
    type Atto = Grid<DecimalRationals, Constant<-18>, Signed<64>, 0, 1>;
    type Deepest = Grid<DecimalRationals, Constant<-38>, Signed<64>, 0, 1>;
    assert!(is_multipliable::<Third>().get());
    assert!(is_multipliable::<Atto>().get());
    assert!(is_multipliable::<Deepest>().get());
    for (a, b) in [(min, min), (min, max), (max, max)] {
        assert_eq!(
            product_position::<Deepest>(a, b),
            reported(oracle(&reading::<Deepest>(), a.index(), b.index()))
        );
        assert_eq!(
            product_position::<Third>(a, b),
            reported(oracle(&reading::<Third>(), a.index(), b.index()))
        );
        assert_eq!(
            product_position::<Atto>(a, b),
            reported(oracle(&reading::<Atto>(), a.index(), b.index()))
        );
    }
    let _ = At::<Signed<8>, 0>::PHASE;
}
