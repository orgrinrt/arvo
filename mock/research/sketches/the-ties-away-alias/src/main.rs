//! Where a consumer can write the ties-away alias against `arvo-format`.
//!
//! A spike: it checks one question and is not the test. The question is whether
//! `toward_zero(x + sign(x) q/2)` has a spelling in the public surface at every
//! position, and if not, exactly where it stops. Each spelling is swept against
//! an integer statement of ties away from zero, `sign(x) floor(|x| + 1/2)`, read
//! through the same completion the map applies, and each planted wrong spelling
//! has to be reported by the same sweep.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::Integer;
use arvo_format::rounding::{Floor, HalfUp, TowardZero};
use arvo_format::slots::{Slot, Slots};

mod carried;

// --- the oracle ----------------------------------------------------------------

/// How far above `s` ties away from zero puts `s + n/d`, zero or one, from the
/// definition: `floor(x + 1/2)` at or above zero and `-floor(-x + 1/2)` below it,
/// with the integer part taken out so nothing is formed past `i128`.
fn oracle_step(s: i128, n: i64, d: i64) -> i128 {
    let (n, d) = (n as i128, d as i128);
    if s >= 0 {
        (2 * n + d).div_euclid(2 * d)
    } else {
        -(d - 2 * n).div_euclid(2 * d)
    }
}

/// The same rule by brute force on the whole position, for a slot small enough
/// that `2 (s d + n)` fits: sign, magnitude, add a half, floor, sign back.
fn brute_step(s: i128, n: i64, d: i64) -> i128 {
    let (n, d) = (n as i128, d as i128);
    let twice = 2 * (s * d + n);
    let sign = twice.signum();
    let r = sign * ((twice.abs() + d).div_euclid(2 * d));
    r - s
}

/// The completion of `s + step` over `[lo, hi]`, wrapped or saturated.
fn complete(s: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    if wrap {
        let span = hi - lo + 1;
        (s.rem_euclid(span) - lo.rem_euclid(span) + step).rem_euclid(span) + lo
    } else if s > hi || (s == hi && step == 1) {
        hi
    } else if s < lo && !(s == lo - 1 && step == 1) {
        lo
    } else {
        s + step
    }
}

// --- the spellings ---------------------------------------------------------------

type Sig<F, M, O> = Signature<F, Adapt<M, O>>;

/// A spelling of the alias: slot, numerator, denominator in, the adapted slot
/// out, or nothing where the spelling cannot be formed.
type Spelling = fn(i128, i64, i64) -> Option<i128>;

macro_rules! spellings {
    ($modname:ident, $f:ty, $o:ty) => {
        mod $modname {
            use super::*;
            type F = $f;
            type O = $o;

            fn at(s: i128, n: i64, d: i64) -> Exact {
                Exact::between(Slot::at(s), Fraction::of(n, d))
            }

            /// The shift, written as the ruling writes it, from the slot and the
            /// ratio the caller built the position from. An even denominator takes
            /// half of itself, so nothing doubles; an odd one has no tie, and the
            /// alias there is `half_up`.
            pub fn shift(s: i128, n: i64, d: i64) -> Option<i128> {
                if d % 2 == 1 {
                    return Some(adapt::<Sig<F, HalfUp, O>>(at(s, n, d), Dither::UNUSED).index());
                }
                let h = d / 2;
                let shifted = if s >= 0 {
                    match n.checked_add(h) {
                        Some(up) => at(s, up, d),
                        None => at(s.checked_add(1)?, n - h, d),
                    }
                } else {
                    at(s, n - h, d)
                };
                Some(adapt::<Sig<F, TowardZero, O>>(shifted, Dither::UNUSED).index())
            }

            /// The alias read off the position alone: `floor` at a tie below zero,
            /// `half_up` everywhere else.
            pub fn select(s: i128, n: i64, d: i64) -> Option<i128> {
                let x = at(s, n, d);
                Some(if x.is_tie().get() && x.slot().index() < 0 {
                    adapt::<Sig<F, Floor, O>>(x, Dither::UNUSED).index()
                } else {
                    adapt::<Sig<F, HalfUp, O>>(x, Dither::UNUSED).index()
                })
            }

            // --- planted wrong spellings, each of which the sweep must report ---

            /// `half_up` alone.
            pub fn half_up_alone(s: i128, n: i64, d: i64) -> Option<i128> {
                Some(adapt::<Sig<F, HalfUp, O>>(at(s, n, d), Dither::UNUSED).index())
            }

            /// `toward_zero` where `floor` belongs, at a tie below zero.
            pub fn toward_zero_below(s: i128, n: i64, d: i64) -> Option<i128> {
                let x = at(s, n, d);
                Some(if x.is_tie().get() && x.slot().index() < 0 {
                    adapt::<Sig<F, TowardZero, O>>(x, Dither::UNUSED).index()
                } else {
                    adapt::<Sig<F, HalfUp, O>>(x, Dither::UNUSED).index()
                })
            }

            /// Zero read as negative.
            pub fn zero_is_negative(s: i128, n: i64, d: i64) -> Option<i128> {
                let x = at(s, n, d);
                Some(if x.is_tie().get() && x.slot().index() <= 0 {
                    adapt::<Sig<F, Floor, O>>(x, Dither::UNUSED).index()
                } else {
                    adapt::<Sig<F, HalfUp, O>>(x, Dither::UNUSED).index()
                })
            }

            /// The shift always toward positive infinity.
            pub fn shift_up_always(s: i128, n: i64, d: i64) -> Option<i128> {
                if d % 2 == 1 {
                    return Some(adapt::<Sig<F, HalfUp, O>>(at(s, n, d), Dither::UNUSED).index());
                }
                let h = d / 2;
                let shifted = match n.checked_add(h) {
                    Some(up) => at(s, up, d),
                    None => at(s.checked_add(1)?, n - h, d),
                };
                Some(adapt::<Sig<F, TowardZero, O>>(shifted, Dither::UNUSED).index())
            }

            /// The shift composed with `floor` rather than `toward_zero`.
            pub fn shift_then_floor(s: i128, n: i64, d: i64) -> Option<i128> {
                if d % 2 == 1 {
                    return Some(adapt::<Sig<F, HalfUp, O>>(at(s, n, d), Dither::UNUSED).index());
                }
                let h = d / 2;
                let shifted = if s >= 0 {
                    match n.checked_add(h) {
                        Some(up) => at(s, up, d),
                        None => at(s.checked_add(1)?, n - h, d),
                    }
                } else {
                    at(s, n - h, d)
                };
                Some(adapt::<Sig<F, Floor, O>>(shifted, Dither::UNUSED).index())
            }

            /// The shift over a doubled denominator, which is where the earlier
            /// arm stopped.
            pub fn doubled(s: i128, n: i64, d: i64) -> Option<i128> {
                let twice_d = d.checked_mul(2)?;
                let twice_n = n.checked_mul(2)?;
                let num = if s < 0 { twice_n.checked_sub(d)? } else { twice_n.checked_add(d)? };
                let shifted = Exact::between(Slot::at(s), Fraction::of(num, twice_d));
                Some(adapt::<Sig<F, TowardZero, O>>(shifted, Dither::UNUSED).index())
            }

            pub const NAMED: [(&str, Spelling); 8] = [
                ("shift", shift),
                ("select", select),
                ("half_up_alone", half_up_alone),
                ("toward_zero_below", toward_zero_below),
                ("zero_is_negative", zero_is_negative),
                ("shift_up_always", shift_up_always),
                ("shift_then_floor", shift_then_floor),
                ("doubled", doubled),
            ];

            pub fn range() -> (i128, i128) {
                (
                    <<F as Format>::Slots as Slots>::MIN.index(),
                    <<F as Format>::Slots as Slots>::MAX.index(),
                )
            }
        }
    };
}

spellings!(i8_wrap, Integer<8>, Wrap);
spellings!(i8_saturate, Integer<8>, Saturate);
spellings!(i64_wrap, Integer<64>, Wrap);
spellings!(i64_saturate, Integer<64>, Saturate);

// --- the domain ------------------------------------------------------------------

fn slots() -> Vec<i128> {
    let mut v = vec![i128::MIN, i128::MIN + 1, i128::MIN + 2];
    v.extend(-3 ..= 3);
    v.extend([i128::MAX - 2, i128::MAX - 1, i128::MAX]);
    v
}

fn dens() -> Vec<i64> {
    let m = i64::MAX;
    let mut v: Vec<i64> = (1 ..= 8).collect();
    v.extend([m / 2 - 1, m / 2, m / 2 + 1, m / 2 + 2, m - 2, m - 1, m]);
    v
}

fn nums(d: i64) -> Vec<i64> {
    let h = d / 2;
    let mut v: Vec<i64> = [0, 1, 2, h - 1, h, h + 1, d - 2, d - 1]
        .into_iter()
        .filter(|&n| n >= 0 && n < d)
        .collect();
    v.sort();
    v.dedup();
    v
}

// --- the sweep -------------------------------------------------------------------

fn main() {
    carried::run();
    // The two oracles agree wherever the brute one can be formed.
    let mut agreed = 0;
    for s in -3 ..= 3 {
        for d in dens() {
            for n in nums(d) {
                assert_eq!(oracle_step(s, n, d), brute_step(s, n, d), "{s} {n}/{d}");
                agreed += 1;
            }
        }
    }
    println!("oracles agree at {agreed} cells with |slot| <= 3");

    let tables: [(&str, [(&str, Spelling); 8], (i128, i128), bool); 4] = [
        ("Integer<8>, Wrap", i8_wrap::NAMED, i8_wrap::range(), true),
        (
            "Integer<8>, Saturate",
            i8_saturate::NAMED,
            i8_saturate::range(),
            false,
        ),
        (
            "Integer<64>, Wrap",
            i64_wrap::NAMED,
            i64_wrap::range(),
            true,
        ),
        (
            "Integer<64>, Saturate",
            i64_saturate::NAMED,
            i64_saturate::range(),
            false,
        ),
    ];
    for (sig, named, (lo, hi), wrap) in tables {
        println!("\n== {sig}");
        for (name, spell) in named {
            let (mut cells, mut wrong, mut unformed, mut ties) = (0, 0, 0, 0);
            let mut first_wrong = None;
            let mut first_unformed = None;
            for s in slots() {
                for d in dens() {
                    for n in nums(d) {
                        cells += 1;
                        ties += usize::from(2 * (n as i128) == d as i128);
                        let want = complete(s, oracle_step(s, n, d), lo, hi, wrap);
                        match spell(s, n, d) {
                            Some(got) if got == want => {},
                            Some(got) => {
                                wrong += 1;
                                first_wrong.get_or_insert((s, n, d, got, want));
                            },
                            None => {
                                unformed += 1;
                                first_unformed.get_or_insert((s, n, d));
                            },
                        }
                    }
                }
            }
            println!(
                "{name:>18}: {cells} cells, {ties} ties, {wrong} wrong, {unformed} unformed; \
                 first wrong {first_wrong:?}; first unformed {first_unformed:?}"
            );
        }
    }
}
