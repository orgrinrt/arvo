//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ties-away alias against the integer rule, at every denominator.
//!
//! The rule is `sign(x) floor(|x| + 1/2)`, written here in integers that read no
//! arvo code, and both spellings of the alias in `ties_away` are swept against it
//! over slots at both ends of the slot index, denominators at both parities up to
//! the ratio's own maximum, and both overflow policies. Six wrong spellings are
//! planted and each has to be reported, which is what says the sweep can fail.
//!
//! What this replaces: an arm that pinned the alias as unwritable past
//! `i64::MAX / 2`. That was a limit of a helper which built the shift over a
//! doubled denominator, and the helper is here as one of the planted spellings.

mod ties_away;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::{Overflow, Saturate, Wrap};
use arvo_format::points::Integer;
use arvo_format::rounding::{Floor, HalfUp, TowardZero};
use arvo_format::slots::{Slot, Slots};
use ties_away::completion::{self, complete};
use ties_away::{at, select, shift, shift_is_formable};

// --- the rule, in integers -----------------------------------------------------

/// How far above `slot` ties away from zero puts `slot + n/d`, zero or one.
///
/// From the definition and nothing else: `floor(x + 1/2)` at or above zero,
/// `-floor(-x + 1/2)` below it, with the slot taken out of both so nothing wider
/// than the ratio's own arithmetic is formed and the ends of the index are
/// ordinary cells.
fn oracle_step(slot: i128, n: i64, d: i64) -> i128 {
    let (n, d) = (n as i128, d as i128);
    if slot >= 0 {
        (2 * n + d).div_euclid(2 * d)
    } else {
        -(d - 2 * n).div_euclid(2 * d)
    }
}

/// The same rule read off the whole position, for a slot small enough that
/// `2 (slot d + n)` fits: sign, magnitude, add a half, floor, sign back.
///
/// The control on the oracle above, which is a rearrangement of this and could be
/// rearranged wrongly.
fn brute_step(slot: i128, n: i64, d: i64) -> i128 {
    let (n, d) = (n as i128, d as i128);
    let twice = 2 * (slot * d + n);
    twice.signum() * ((twice.abs() + d).div_euclid(2 * d)) - slot
}

/// The rule over a ratio that carries out of `[0, 1)`, so the position may lie
/// past either end of the index, completed the same way.
fn oracle_carried(slot: i128, num: i64, d: i64, lo: i128, hi: i128, wrap: bool) -> i128 {
    let whole = num.div_euclid(d) as i128;
    let rem = num.rem_euclid(d) as i128;
    let wide = d as i128;
    let at = slot.checked_add(whole);
    let negative = match at {
        Some(v) => v < 0,
        None => whole < 0,
    };
    let step = if negative {
        -(wide - 2 * rem).div_euclid(2 * wide)
    } else {
        (2 * rem + wide).div_euclid(2 * wide)
    };
    if wrap {
        let span = hi - lo + 1;
        (slot.rem_euclid(span) + whole.rem_euclid(span) - lo.rem_euclid(span) + step)
            .rem_euclid(span)
            + lo
    } else {
        match at {
            Some(v) => complete(v, step, lo, hi, false),
            None if whole > 0 => hi,
            None => lo,
        }
    }
}

// --- the domain ----------------------------------------------------------------

/// Both ends of the slot index, and the slots around zero where the sign changes.
fn slots() -> Vec<i128> {
    let mut v = vec![i128::MIN, i128::MIN + 1, i128::MIN + 2];
    v.extend(-3i128 ..= 3);
    v.extend([i128::MAX - 2, i128::MAX - 1, i128::MAX]);
    v
}

/// Small denominators of both parities, and both parities at each end of the
/// ratio's own integer, including the three the retired claim named.
fn dens() -> Vec<i64> {
    let m = i64::MAX;
    let mut v: Vec<i64> = (1 ..= 8).collect();
    v.extend([m / 2 - 1, m / 2, m / 2 + 1, m / 2 + 2, m - 2, m - 1, m]);
    v
}

/// Numerators at both ends of a denominator and at its midpoint, which is the
/// tie where one exists.
fn nums(d: i64) -> Vec<i64> {
    let h = d / 2;
    let mut v: Vec<i64> = [0, 1, 2, h - 1, h, h + 1, d - 2, d - 1]
        .into_iter()
        .filter(|&n| n >= 0 && n < d)
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}

/// Numerators outside `[0, d)`, which the constructor carries into the slot and
/// past the end of the index where the slot sits at one.
fn carried_nums(d: i64) -> Vec<i64> {
    let h = d / 2;
    [
        Some(-1),
        Some(-h),
        d.checked_neg().map(|x| x + h),
        d.checked_neg(),
        d.checked_neg().and_then(|x| x.checked_sub(h)),
        Some(d),
        d.checked_add(h),
        d.checked_mul(2).and_then(|x| x.checked_add(h)),
        Some(i64::MAX),
        Some(i64::MIN + 1),
    ]
    .into_iter()
    .flatten()
    .filter(|&n| n < 0 || n >= d)
    .collect()
}

// --- the planted spellings -----------------------------------------------------

/// A spelling of the alias: the three coordinates in, the adapted slot out, or
/// nothing where that spelling cannot form the position it needs.
type Spelling = fn(i128, i64, i64) -> Option<i128>;

/// `half_up` alone, which is the alias everywhere except at a tie below zero.
fn half_up_alone<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    Some(adapt::<Signature<F, Adapt<HalfUp, O>>>(at(slot, n, d), Dither::UNUSED).index())
}

/// `toward_zero` where `floor` belongs, which is the tie below zero sent the
/// wrong way.
fn toward_zero_below<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    let x = at(slot, n, d);
    Some(if x.is_tie().get() && x.slot().index() < 0 {
        adapt::<Signature<F, Adapt<TowardZero, O>>>(x, Dither::UNUSED).index()
    } else {
        adapt::<Signature<F, Adapt<HalfUp, O>>>(x, Dither::UNUSED).index()
    })
}

/// A zero slot read as negative, which is the one cell the comparison's boundary
/// decides.
fn zero_is_negative<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    let x = at(slot, n, d);
    Some(if x.is_tie().get() && x.slot().index() <= 0 {
        adapt::<Signature<F, Adapt<Floor, O>>>(x, Dither::UNUSED).index()
    } else {
        adapt::<Signature<F, Adapt<HalfUp, O>>>(x, Dither::UNUSED).index()
    })
}

/// The shift taken toward positive infinity whatever the sign, which is the
/// ruling's own reading of `half_up` wearing the alias's name.
fn shift_up_always<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    if d % 2 == 1 {
        return Some(
            adapt::<Signature<F, Adapt<HalfUp, O>>>(at(slot, n, d), Dither::UNUSED).index(),
        );
    }
    let half = d / 2;
    let shifted = match n.checked_add(half) {
        Some(up) => at(slot, up, d),
        None => at(slot.checked_add(1)?, n - half, d),
    };
    Some(adapt::<Signature<F, Adapt<TowardZero, O>>>(shifted, Dither::UNUSED).index())
}

/// The shift composed with `floor` rather than with `toward_zero`.
fn shift_then_floor<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    if d % 2 == 1 {
        return Some(
            adapt::<Signature<F, Adapt<HalfUp, O>>>(at(slot, n, d), Dither::UNUSED).index(),
        );
    }
    let half = d / 2;
    let shifted = if slot >= 0 {
        match n.checked_add(half) {
            Some(up) => at(slot, up, d),
            None => at(slot.checked_add(1)?, n - half, d),
        }
    } else {
        at(slot, n - half, d)
    };
    Some(adapt::<Signature<F, Adapt<Floor, O>>>(shifted, Dither::UNUSED).index())
}

/// The shift over a doubled denominator, which is the spelling the retired claim
/// measured and mistook for the alias.
fn doubled<F: Format, O: Overflow>(slot: i128, n: i64, d: i64) -> Option<i128> {
    let twice_d = d.checked_mul(2)?;
    let twice_n = n.checked_mul(2)?;
    let num = if slot < 0 { twice_n.checked_sub(d)? } else { twice_n.checked_add(d)? };
    let shifted = Exact::between(Slot::at(slot), Fraction::of(num, twice_d));
    Some(adapt::<Signature<F, Adapt<TowardZero, O>>>(shifted, Dither::UNUSED).index())
}

// --- one declared signature, with its arms -------------------------------------

/// What the sweep runs over: a signature, the range its format declares, and the
/// spellings to try against the rule.
struct Arms {
    name:    &'static str,
    lo:      i128,
    hi:      i128,
    wrap:    bool,
    alias:   [(&'static str, Spelling); 2],
    planted: [(&'static str, Spelling); 6],
}

fn arms<F: Format, O: Overflow>(name: &'static str, wrap: bool) -> Arms {
    Arms {
        name,
        lo: <<F as Format>::Slots as Slots>::MIN.index(),
        hi: <<F as Format>::Slots as Slots>::MAX.index(),
        wrap,
        alias: [
            ("shift", shift::<F, O>),
            ("read off the position", |s, n, d| {
                Some(select::<F, O>(s, n, d))
            }),
        ],
        planted: [
            ("half_up alone", half_up_alone::<F, O>),
            ("toward_zero below zero", toward_zero_below::<F, O>),
            ("zero read as negative", zero_is_negative::<F, O>),
            ("the shift always upward", shift_up_always::<F, O>),
            ("the shift then floor", shift_then_floor::<F, O>),
            ("the doubled shift", doubled::<F, O>),
        ],
    }
}

/// The four signatures the sweep runs: two widths, both policies.
fn table() -> [Arms; 4] {
    [
        arms::<Integer<8>, Wrap>("Integer<8> under Wrap", true),
        arms::<Integer<8>, Saturate>("Integer<8> under Saturate", false),
        arms::<Integer<64>, Wrap>("Integer<64> under Wrap", true),
        arms::<Integer<64>, Saturate>("Integer<64> under Saturate", false),
    ]
}

/// The four ranges the sweep runs over, as the completion arms ask for them.
fn ranges() -> Vec<(i128, i128, bool)> {
    table().iter().map(|a| (a.lo, a.hi, a.wrap)).collect()
}

/// Cells, cells answered wrongly, and cells the spelling could not form.
fn tally(a: &Arms, spell: Spelling) -> (usize, usize, usize) {
    let (mut cells, mut wrong, mut unformed) = (0, 0, 0);
    for slot in slots() {
        for d in dens() {
            for n in nums(d) {
                cells += 1;
                let want = complete(slot, oracle_step(slot, n, d), a.lo, a.hi, a.wrap);
                match spell(slot, n, d) {
                    Some(got) if got == want => {},
                    Some(_) => wrong += 1,
                    None => unformed += 1,
                }
            }
        }
    }
    (cells, wrong, unformed)
}

// --- the arms ------------------------------------------------------------------

#[test]
fn the_two_statements_of_the_rule_agree_wherever_the_position_fits() {
    let mut checked = 0;
    for slot in -3i128 ..= 3 {
        for d in dens() {
            for n in nums(d) {
                assert_eq!(
                    oracle_step(slot, n, d),
                    brute_step(slot, n, d),
                    "the rule and its rearrangement disagree at {slot} + {n}/{d}"
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 600, "the control ran over {checked} cells");
}

#[test]
fn the_two_statements_of_the_completion_agree_over_every_range_the_sweep_runs() {
    let cells = completion::agrees(completion::complete, &ranges())
        .expect("the completion and its rearrangement disagree");
    assert!(cells > 200, "the agreement ran over {cells} cells");
}

#[test]
fn the_control_every_planted_completion_is_reported() {
    // Without this the arm above is two spellings of one mistake agreeing with
    // each other, which is what a single-stated completion was.
    for &(name, spell) in completion::PLANTED {
        assert!(
            completion::agrees(spell, &ranges()).is_err(),
            "the planted completion `{name}` was not reported"
        );
    }
}

#[test]
fn both_spellings_of_the_alias_are_ties_away_from_zero() {
    for a in table() {
        for (name, spell) in a.alias {
            let (cells, wrong, unformed) = tally(&a, spell);
            assert_eq!(wrong, 0, "{name} disagrees with the rule under {}", a.name);
            assert!(
                cells > 1000,
                "{name} ran over {cells} cells under {}",
                a.name
            );
            // The shift has one region it cannot form and the reading has none,
            // which the next arm states as a predicate.
            assert!(
                unformed == 0 || name == "shift",
                "{name} could not be formed at {unformed} cells under {}",
                a.name
            );
        }
    }
}

#[test]
fn the_shift_is_unformable_exactly_where_the_design_says_and_no_tie_is_there() {
    let a = &table()[0];
    let mut unformable = 0;
    for slot in slots() {
        for d in dens() {
            for n in nums(d) {
                let formable = shift_is_formable(slot, n, d);
                assert_eq!(
                    shift::<Integer<8>, Wrap>(slot, n, d).is_some(),
                    formable,
                    "the region and the spelling disagree at {slot} + {n}/{d}"
                );
                if !formable {
                    unformable += 1;
                    // Nothing about the alias turns on the region, because the
                    // numerator is above the midpoint there and the alias is
                    // `half_up`, which is formable.
                    assert!(!at(slot, n, d).is_tie().get(), "a tie at {slot} + {n}/{d}");
                    let want = complete(slot, oracle_step(slot, n, d), a.lo, a.hi, a.wrap);
                    assert_eq!(select::<Integer<8>, Wrap>(slot, n, d), want);
                }
            }
        }
    }
    assert!(unformable > 0, "the region was never reached");
}

#[test]
fn the_control_every_planted_spelling_is_reported() {
    for a in table() {
        for (name, spell) in a.planted {
            let (_, wrong, unformed) = tally(&a, spell);
            assert!(
                wrong + unformed > 0,
                "{name} was not reported under {}",
                a.name
            );
        }
    }
}

#[test]
fn the_doubled_shift_stops_at_or_below_half_the_ratios_integer() {
    // The claim this replaces put the limit past `i64::MAX / 2`. The doubling
    // fails at that denominator too, because the numerator it forms is up to
    // three halves of it, so the retired claim was wrong about where its own
    // helper stopped as well as about the alias.
    assert_eq!(doubled::<Integer<8>, Wrap>(0, 1, i64::MAX / 2), Some(0));
    let big = i64::MAX / 2;
    assert_eq!(doubled::<Integer<8>, Wrap>(0, big - 1, big), None);
    assert_eq!(doubled::<Integer<8>, Wrap>(0, 1, i64::MAX / 2 + 1), None);
    // And the alias is formable at both, by both spellings.
    assert!(shift::<Integer<8>, Wrap>(0, big - 1, big).is_some());
    assert!(shift::<Integer<8>, Wrap>(0, 1, i64::MAX / 2 + 1).is_some());
}

#[test]
fn the_alias_is_ties_away_from_zero_over_ratios_that_carry_past_the_index() {
    for a in table() {
        let (mut cells, mut ties) = (0, 0);
        for slot in slots() {
            for d in dens() {
                for num in carried_nums(d) {
                    cells += 1;
                    ties += usize::from(2 * (num.rem_euclid(d) as i128) == d as i128);
                    let want = oracle_carried(slot, num, d, a.lo, a.hi, a.wrap);
                    // The reading off the position, at this signature. The shift
                    // is stated over a remainder in `[0, 1)` and is not swept
                    // here.
                    let (name, spell) = a.alias[1];
                    assert_eq!(
                        spell(slot, num, d),
                        Some(want),
                        "{name} at {slot} + {num}/{d} under {}",
                        a.name
                    );
                }
            }
        }
        assert!(cells > 1000 && ties > 100, "{cells} cells, {ties} ties");
    }
}

#[test]
fn the_alias_differs_from_half_up_exactly_at_a_tie_below_zero() {
    let a = &table()[0];
    let mut differing = 0;
    for slot in slots() {
        for d in dens() {
            for n in nums(d) {
                let x = at(slot, n, d);
                let alias = select::<Integer<8>, Wrap>(slot, n, d);
                let up = half_up_alone::<Integer<8>, Wrap>(slot, n, d).unwrap();
                let below = x.is_tie().get() && x.slot().index() < 0;
                assert_eq!(alias != up, below, "at {slot} + {n}/{d} under {}", a.name);
                differing += usize::from(below);
            }
        }
    }
    assert!(differing > 0, "no tie below zero was reached");
}

#[test]
fn the_control_the_sweep_reaches_every_kind_of_cell() {
    let (mut ties, mut below, mut above, mut past_top, mut past_bottom, mut odd) =
        (0, 0, 0, 0, 0, 0);
    for slot in slots() {
        for d in dens() {
            for n in nums(d) {
                let x = at(slot, n, d);
                let tie = x.is_tie().get();
                ties += usize::from(tie);
                below += usize::from(tie && slot < 0);
                above += usize::from(tie && slot >= 0);
                odd += usize::from(d % 2 == 1);
                past_top += usize::from(slot == i128::MAX && n > 0);
                past_bottom += usize::from(slot == i128::MIN && n > 0);
            }
        }
    }
    assert!(ties > 50, "{ties} ties");
    assert!(
        below > 10 && above > 10,
        "{below} below zero, {above} at or above"
    );
    assert!(
        odd > 50,
        "{odd} cells at an odd denominator, where no tie exists"
    );
    assert!(past_top > 0 && past_bottom > 0, "the ends of the index");
}
