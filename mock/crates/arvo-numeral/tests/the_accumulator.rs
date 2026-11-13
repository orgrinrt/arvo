//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The accumulator against the exact sum, computed here in wide integers.
//!
//! Every array of three terms at widths 1 to 4 and of two at width 5, under
//! both policies and both slot families, and seeded slices of up to 300 terms
//! at widths 8, 13 and 32: `total` is the exact sum, `finish` is the terms'
//! adaptation of it and `fits` is whether it is a member. `take` is checked on
//! slices shorter than, equal to and longer than the capacity. A few sums are
//! named with their slots written out, so that the suite does not rest on the
//! oracle alone.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, Integer, UFixed};
use arvo_format::rounding::{HalfEven, TowardZero};
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Accumulator, Num, Quiet, Sole};
use common::oracle::completed;
use common::{SplitMix, range, value};

type Sig<P, O> = Signature<P, Adapt<TowardZero, O>>;

/// The three readings of one accumulator against the exact sum.
fn reads<S: Sole, T: Sole, const CAP: usize>(acc: Accumulator<S, T, CAP, Quiet>, sum: i128) {
    let (min, max) = range::<S>();
    assert_eq!(acc.total().slot().index(), sum, "`total` is not the sum");
    assert_eq!(
        acc.finish().slot().index(),
        completed::<S>(sum),
        "`finish` is not the adaptation of {sum}"
    );
    assert_eq!(
        acc.fits().get(),
        sum >= min && sum <= max,
        "`fits` at {sum}"
    );
}

/// Every array of three members of `S`.
fn threes<S: Sole, T: Sole>(checked: &mut u64) {
    let (min, max) = range::<S>();
    for a in min ..= max {
        for b in min ..= max {
            for c in min ..= max {
                let terms = [value::<S>(a), value::<S>(b), value::<S>(c)];
                reads(Accumulator::<S, T, 3, Quiet>::of(&terms), a + b + c);
                *checked += 1;
            }
        }
    }
}

/// Every array of two members of `S`.
fn twos<S: Sole, T: Sole>(checked: &mut u64) {
    let (min, max) = range::<S>();
    for a in min ..= max {
        for b in min ..= max {
            let terms = [value::<S>(a), value::<S>(b)];
            reads(Accumulator::<S, T, 2, Quiet>::of(&terms), a + b);
            *checked += 1;
        }
    }
}

/// One width's terms in both families under both policies, each into an
/// accumulator two bits wider.
macro_rules! at_width {
    ($body:ident, $w:literal, $t:literal, $checked:expr) => {{
        $body::<Sig<Integer<$w>, Wrap>, Sig<Integer<$t>, Wrap>>($checked);
        $body::<Sig<Integer<$w>, Saturate>, Sig<Integer<$t>, Saturate>>($checked);
        $body::<Sig<UFixed<$w, 0>, Wrap>, Sig<UFixed<$t, 0>, Saturate>>($checked);
        $body::<Sig<UFixed<$w, 0>, Saturate>, Sig<UFixed<$t, 0>, Wrap>>($checked);
    }};
}

#[test]
fn every_small_array_sums_exactly() {
    let mut n = 0u64;
    at_width!(threes, 1, 3, &mut n);
    at_width!(threes, 2, 4, &mut n);
    at_width!(threes, 3, 5, &mut n);
    at_width!(threes, 4, 6, &mut n);
    assert_eq!(n, 4 * (8 + 64 + 512 + 4096));
    let mut n = 0u64;
    at_width!(twos, 5, 7, &mut n);
    assert_eq!(n, 4 * 1024);
}

/// Seeded slices of `S`, of seeded lengths up to the capacity of 300.
fn slices<S: Sole, T: Sole>(checked: &mut u64) {
    let (min, max) = range::<S>();
    let mut rng = SplitMix::seeded();
    for round in 0 .. 64u64 {
        // The longest and the empty slice are always among them.
        let len = match round {
            0 => 300,
            1 => 0,
            _ => (rng.next() % 301) as usize,
        };
        let mut sum = 0i128;
        let mut terms = Vec::with_capacity(len);
        for i in 0 .. len {
            // The first slice sits at one end of the range throughout.
            let a = if round == 0 {
                if i % 2 == 0 || min == 0 { max } else { max - 1 }
            } else {
                rng.member(min, max)
            };
            sum += a;
            terms.push(value::<S>(a));
        }
        let (acc, rest) = Accumulator::<S, T, 300, Quiet>::take(&terms);
        assert!(rest.is_empty(), "a slice within the capacity left a rest");
        reads(acc, sum);
        *checked += 1;
    }
}

#[test]
fn seeded_slices_sum_exactly_at_wide_widths() {
    let mut n = 0u64;
    at_width!(slices, 8, 17, &mut n);
    at_width!(slices, 13, 22, &mut n);
    at_width!(slices, 32, 41, &mut n);
    assert_eq!(n, 3 * 4 * 64);
}

/// The lowest member three hundred times: the other end of the bound.
#[test]
fn the_lowest_member_at_capacity_is_held() {
    type S = Sig<Integer<32>, Saturate>;
    type T = Sig<Integer<41>, Saturate>;
    let (min, _) = range::<S>();
    let terms = [value::<S>(min); 300];
    reads(Accumulator::<S, T, 300, Quiet>::of(&terms), 300 * min);
}

#[test]
fn take_sums_a_block_and_hands_back_the_rest() {
    type S = Sig<Integer<6>, Saturate>;
    type T = Sig<Integer<9>, Wrap>;
    let terms: Vec<Num<S, Quiet>> = (0 .. 20).map(|i| value::<S>(i - 4)).collect();
    let exact = |n: usize| -> i128 { (0 .. n as i128).map(|i| i - 4).sum() };
    for len in [0, 1, 7, 8, 9, 16, 19, 20] {
        let (acc, rest) = Accumulator::<S, T, 8, Quiet>::take(&terms[.. len]);
        let taken = len.min(8);
        reads(acc, exact(taken));
        assert_eq!(rest.len(), len - taken, "the rest of {len} terms");
        assert_eq!(rest, &terms[taken .. len], "the rest is not the tail");
    }
    // Block by block, the whole slice is read and nothing twice.
    let mut rest = &terms[..];
    let mut blocks = 0;
    let mut whole = 0i128;
    while !rest.is_empty() {
        let (acc, after) = Accumulator::<S, T, 8, Quiet>::take(rest);
        whole += acc.total().slot().index();
        rest = after;
        blocks += 1;
    }
    assert_eq!(blocks, 3);
    assert_eq!(whole, exact(20));
}

#[test]
fn plus_is_the_sum_of_both_and_minus_their_difference() {
    type S = Sig<Integer<3>, Saturate>;
    type T = Sig<Integer<6>, Saturate>;
    let (min, max) = range::<S>();
    let mut n = 0u64;
    for a in min ..= max {
        for b in min ..= max {
            for c in min ..= max {
                let two = Accumulator::<S, T, 2, Quiet>::of(&[value::<S>(a), value::<S>(b)]);
                let one = Accumulator::<S, T, 1, Quiet>::of(&[value::<S>(c)]);
                reads(Accumulator::<S, T, 3, Quiet>::plus(two, one), a + b + c);
                reads(Accumulator::<S, T, 3, Quiet>::plus(one, two), a + b + c);
                let difference = Accumulator::<S, T, 3, Quiet>::minus(two, one);
                assert_eq!(difference.slot().index(), a + b - c);
                assert_eq!(
                    difference.convert::<S>().slot().index(),
                    completed::<S>(a + b - c)
                );
                n += 1;
            }
        }
    }
    assert_eq!(n, 512);
    let none = Accumulator::<S, T, 3, Quiet>::empty();
    reads(none, 0);
    reads(Accumulator::<S, T, 6, Quiet>::plus(none, none), 0);
}

/// Over unsigned terms a difference goes below zero, so it is held on a signed
/// grid, and comes back onto the terms' by their own policy.
#[test]
fn a_difference_of_unsigned_sums_reaches_below_zero() {
    type S = Sig<UFixed<4, 0>, Saturate>;
    type T = Sig<Integer<8>, Saturate>;
    let (_, max) = range::<S>();
    let mut n = 0u64;
    for a in 0 ..= max {
        for b in 0 ..= max {
            let lhs = Accumulator::<S, T, 2, Quiet>::of(&[value::<S>(a), value::<S>(1)]);
            let rhs = Accumulator::<S, T, 2, Quiet>::of(&[value::<S>(b), value::<S>(max)]);
            let difference = Accumulator::<S, T, 2, Quiet>::minus(lhs, rhs);
            assert_eq!(difference.slot().index(), a + 1 - b - max);
            assert_eq!(
                difference.convert::<S>().slot().index(),
                (a + 1 - b - max).clamp(0, max)
            );
            n += 1;
        }
    }
    assert_eq!(n, 256);
    let low = Accumulator::<S, T, 2, Quiet>::minus(
        Accumulator::<S, T, 2, Quiet>::empty(),
        Accumulator::<S, T, 2, Quiet>::of(&[value::<S>(max), value::<S>(max)]),
    );
    assert_eq!(low.slot().index(), -30);
}

/// Seven, seven and minus eight at four signed saturating bits. The exact sum
/// is six. The signature's own `add` gives six or minus one by the order, and
/// the accumulator gives six in every one.
#[test]
fn the_order_of_terms_does_not_change_the_finished_sum() {
    type S = Sig<Integer<4>, Saturate>;
    type T = Sig<Integer<6>, Saturate>;
    let (p, q, m) = (value::<S>(7), value::<S>(7), value::<S>(-8));
    assert_eq!(p.add(q).add(m).slot().index(), -1);
    assert_eq!(p.add(m).add(q).slot().index(), 6);
    for terms in [[p, q, m], [p, m, q], [m, p, q]] {
        let acc = Accumulator::<S, T, 3, Quiet>::of(&terms);
        assert_eq!(acc.total().slot().index(), 6);
        assert_eq!(acc.finish().slot().index(), 6);
        assert!(acc.fits().get());
    }
    // Every order of every three members, against the first order.
    let (min, max) = range::<S>();
    for a in min ..= max {
        for b in min ..= max {
            for c in min ..= max {
                let (a, b, c) = (value::<S>(a), value::<S>(b), value::<S>(c));
                let first = Accumulator::<S, T, 3, Quiet>::of(&[a, b, c]).finish();
                for terms in [[a, c, b], [b, a, c], [b, c, a], [c, a, b], [c, b, a]] {
                    assert_eq!(Accumulator::<S, T, 3, Quiet>::of(&terms).finish(), first);
                }
            }
        }
    }
}

/// The probe's arm through the real type: fifteens at four unsigned bits into
/// eight. Sixteen of them are 240. Thirty-two are 480, which eight bits do not
/// hold, and a fold written by hand over them reads 224.
#[test]
fn the_capacity_bounds_what_a_slice_puts_in() {
    type S = Sig<UFixed<4, 0>, Saturate>;
    type T = Sig<UFixed<8, 0>, Wrap>;
    let fifteens = [value::<S>(15); 32];
    let sixteen: &[Num<S, Quiet>; 16] = fifteens[.. 16].try_into().unwrap();
    assert_eq!(
        Accumulator::<S, T, 16, Quiet>::of(sixteen)
            .total()
            .slot()
            .index(),
        240
    );
    let (acc, rest) = Accumulator::<S, T, 16, Quiet>::take(&fifteens);
    assert_eq!(acc.total().slot().index(), 240);
    assert_eq!(rest.len(), 16);
    assert_eq!(acc.finish().slot().index(), 15);
    assert!(!acc.fits().get());
}

/// A sum on a fixed-point grid: the accumulator shares the terms' eighths.
#[test]
fn a_fixed_point_sum_is_exact_on_the_terms_grid() {
    type S = Signature<Fixed<6, -3>, Adapt<HalfEven, Saturate>>;
    type T = Signature<Fixed<10, -3>, Adapt<HalfEven, Wrap>>;
    let mut n = 0u64;
    threes::<S, Signature<Fixed<8, -3>, Adapt<HalfEven, Wrap>>>(&mut n);
    assert_eq!(n, 64 * 64 * 64);
    let terms = [value::<S>(31); 16];
    let acc = Accumulator::<S, T, 16, Quiet>::of(&terms);
    assert_eq!(acc.total().slot().index(), 496);
    assert_eq!(acc.finish().slot().index(), 31);
}

#[test]
fn an_accumulated_sum_is_a_constant() {
    type S = Sig<Integer<8>, Saturate>;
    type T = Sig<Integer<16>, Saturate>;
    const TERMS: [Num<S, Quiet>; 3] =
        [Num::constant::<100>(), Num::constant::<100>(), Num::constant::<-90>()];
    const SUM: Num<S, Quiet> = Accumulator::<S, T, 4, Quiet>::of(&TERMS).finish();
    const BLOCK: Num<T, Quiet> = Accumulator::<S, T, 2, Quiet>::take(&TERMS).0.total();
    assert_eq!(SUM.slot().index(), 110);
    assert_eq!(BLOCK.slot().index(), 200);
}
