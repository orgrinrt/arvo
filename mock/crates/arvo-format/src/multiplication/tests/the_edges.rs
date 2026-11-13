//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product at the edges of every width from nine to 64, where the member
//! sweep cannot reach and the carry decision is made.
//!
//! At each width of both families, at the exponents zero, one, minus one and
//! minus 62, the verdict is checked against an independent reading of the four
//! corners, and the step at the corners and the slots beside them against an
//! oracle that forms the slots' product in the host's unsigned 128-bit integer
//! and shifts it, rather than in the step's four limbs. The step is read without
//! forcing the obligation, so the formats it does not admit are swept too, and
//! there a position past the index has to saturate on its sign. A step that
//! keeps the floor and drops the remainder is run beside it as the control: it
//! has to be wrong at every fractional exponent and right at every whole one.

use notko::Maybe;

use super::At;
use crate::apply::{Exact, Fraction};
use crate::format::Format;
use crate::multiplication::{is_multipliable, product_position, unforced_position};
use crate::points::{Integer, UFixed};
use crate::quantum::Quantum;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::PerFormat;

/// Where the oracle puts `a * b * 2^e`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Answer {
    /// Past the end of the index, on the negative side when it says so.
    Past(bool),
    /// A position the index holds.
    At(Exact),
}

/// `a * b * 2^e`, from the magnitudes' product in the host's unsigned 128-bit
/// integer, for slots of at most 64 bits and an exponent from -126 up.
fn oracle(a: i128, b: i128, e: i32) -> Answer {
    let negative = (a < 0) != (b < 0) && a != 0 && b != 0;
    // Each magnitude is under 2^64, so the product is under 2^128.
    let m = a.unsigned_abs() * b.unsigned_abs();
    let (whole, rem, den): (u128, u128, u128) = if e >= 0 {
        if m != 0 && (e >= 128 || m > u128::MAX >> e) {
            return Answer::Past(negative);
        }
        (m << e, 0, 1)
    } else {
        let k = e.unsigned_abs();
        (m >> k, m & ((1u128 << k) - 1), 1u128 << k)
    };
    // The floor below zero takes one more slot and the remainder from the other
    // side.
    let (whole, rem) = if negative && rem != 0 { (whole + 1, den - rem) } else { (whole, rem) };
    let slot = if negative {
        if whole > i128::MIN.unsigned_abs() {
            return Answer::Past(true);
        }
        0i128.wrapping_sub_unsigned(whole)
    } else {
        if whole > i128::MAX as u128 {
            return Answer::Past(false);
        }
        whole as i128
    };
    if rem == 0 {
        return Answer::At(Exact::on_grid(Slot::at(slot)));
    }
    let shift = rem.trailing_zeros();
    Answer::At(Exact::between(
        Slot::at(slot),
        Fraction::of((rem >> shift) as i64, (den >> shift) as i64),
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
fn corners_are_held(min: i128, max: i128, e: i32) -> bool {
    [(min, min), (min, max), (max, min), (max, max)]
        .into_iter()
        .all(|(a, b)| {
            match oracle(a, b, e) {
                Answer::Past(_) => false,
                Answer::At(at) => at.is_on_grid().get() || at.slot().index() < i128::MAX,
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
    formats:        u64,
    held_out:       u64,
    points:         u64,
    floor_wrong:    u64,
    floor_on_whole: u64,
}

impl PerFormat for Edges {
    fn run<F: Format>(&mut self) {
        let min = <F::Slots as Slots>::MIN.index();
        let max = <F::Slots as Slots>::MAX.index();
        let e = <F::Quantum as Quantum>::BASE.power();
        self.formats += 1;
        let held = corners_are_held(min, max, e);
        assert_eq!(
            is_multipliable::<F>().get(),
            held,
            "[{min}, {max}] at 2^{e}"
        );
        self.held_out += u64::from(!held);
        for a in edges(min, max) {
            for b in edges(min, max) {
                let want = reported(oracle(a, b, e));
                let Maybe::Is(got) = unforced_position::<F>(Slot::at(a), Slot::at(b)) else {
                    panic!("2^{e} leaves a remainder every position holds");
                };
                assert_eq!(got, want, "{a} * {b} at 2^{e}");
                self.points += 1;
                if Exact::on_grid(want.slot()) != want {
                    self.floor_wrong += 1;
                    if e >= 0 {
                        self.floor_on_whole += 1;
                    }
                }
            }
        }
    }
}

/// Every width from nine to 64 of both families, at one exponent.
macro_rules! every_width {
    ($walk:ident; $e:literal) => {
        every_width!(@ $walk; $e;
            9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
            53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64);
    };
    (@ $walk:ident; $e:literal; $($w:literal),+) => {
        $(
            $walk.run::<At<Signed<$w>, { $e }>>();
            $walk.run::<At<Unsigned<$w>, { $e }>>();
        )+
    };
}

#[test]
fn the_product_is_exact_and_its_verdict_true_at_the_edges_of_every_width_from_nine_to_64() {
    let mut walk = Edges::default();
    every_width!(walk; 0);
    every_width!(walk; 1);
    every_width!(walk; -1);
    every_width!(walk; -62);
    assert_eq!(walk.formats, 4 * 56 * 2);
    // Seven edge slots signed and four unsigned, squared, at every width.
    assert_eq!(walk.points, 4 * 56 * (49 + 16));
    // Held out: the unsigned 64-bit range at zero, and both 64-bit ranges at
    // one. The unsigned 63-bit range at one is held.
    assert_eq!(walk.held_out, 3);
    // The floor-only control is wrong somewhere and only at a fractional
    // exponent.
    assert!(
        walk.floor_wrong > 0,
        "the floor-only step agreed everywhere"
    );
    assert_eq!(walk.floor_on_whole, 0);
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
        reported(oracle(u64::MAX as i128, u64::MAX as i128, -62))
    );
}
