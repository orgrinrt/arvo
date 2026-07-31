//! PROBE 4: the one grade in this design that is a NUMBER rather than a
//! boolean, and the thing it decides.
//!
//! Probe 3 found that the shipped fixed-point multiply breaks reassociation
//! and distributivity even for an UNBOUNDED exact numeral, i.e. with no range
//! recovery firing at all, purely from the unconditional `>> FRAC` that
//! returns a product to scale (`arith_macros.rs:33-34, 95-101`). That failure
//! is not a range phenomenon and cannot be keyed on the range-recovery rule.
//!
//! What it IS keyed on is a count: how many times a value is quantised on the
//! way from the inputs to the answer. This probe measures that directly.
//!
//! Three evaluations of the same expression `a * b * c` in Q2.2:
//!
//!   k = 1   one quantisation. Keep the exact 2^6-scaled product, round once.
//!           This is `Growth::Exact` done properly.
//!   k = 2   two quantisations. Round after each multiply.
//!           This is `Growth::Narrowed` to the operand width.
//!   k = 2'  two quantisations, but the intermediate uses round-to-odd, the
//!           field's classical cure for double rounding. The draft already
//!           notes round-to-odd is expressible under its vocabulary (5.1) and
//!           that the unwritten multiplication work "will meet double
//!           rounding immediately".
//!
//! If the grade were boolean ("does this quantise: yes/no") none of these
//! three could be told apart, because all three quantise. The distinction is
//! the count, and the count is what selects the intermediate's rounding rule.
//!
//! Build:  rustc -O 04_rounding_count_is_the_grade.rs && ./04_rounding_count_is_the_grade

const FRAC: u32 = 2;
const LO: i64 = -8;
const HI: i64 = 7;

/// Round to nearest, ties to even. The draft's stated in-range resolution for
/// Warm / Cold / Precise (`11_current_shape_draft.md:327`).
fn near_even(x: i64, s: u32) -> i64 {
    if s == 0 {
        return x;
    }
    let half = 1i64 << (s - 1);
    let mask = (1i64 << s) - 1;
    let q = x >> s; // floor
    let rem = x & mask; // non-negative remainder relative to the floor
    if rem > half {
        q + 1
    } else if rem < half {
        q
    } else if q & 1 == 1 {
        q + 1
    } else {
        q
    }
}

/// Round to odd: exact results pass through, inexact ones land on an odd
/// value. The point of it is that a subsequent rounding of an odd value can
/// never be a tie, so the second rounding cannot go the wrong way.
fn to_odd(x: i64, s: u32) -> i64 {
    if s == 0 {
        return x;
    }
    let mask = (1i64 << s) - 1;
    let q = x >> s;
    if (x & mask) != 0 {
        q | 1
    } else {
        q
    }
}

/// Truncate, which is what the shipped multiply actually does for every
/// strategy including the three the preset table says round to nearest-even.
fn trunc(x: i64, s: u32) -> i64 {
    x >> s
}

type Round = fn(i64, u32) -> i64;

fn one_rounding(a: i64, b: i64, c: i64, r: Round) -> i64 {
    // a*b*c is 2^(3*FRAC)-scaled; one quantisation brings it to 2^FRAC.
    r(a * b * c, 2 * FRAC)
}

fn two_roundings(a: i64, b: i64, c: i64, inner: Round, outer: Round) -> i64 {
    let ab = inner(a * b, FRAC);
    outer(ab * c, FRAC)
}

fn main() {
    let names: [(&str, Round); 2] = [
        ("nearest-even", near_even as Round),
        ("truncate (as shipped)", trunc as Round),
    ];

    for (label, r) in names {
        let mut disagree_k2 = 0usize;
        let mut disagree_k2_odd = 0usize;
        let mut worst_k2 = 0i64;
        let mut worst_case: Option<(i64, i64, i64, i64, i64)> = None;
        let mut total = 0usize;

        for a in LO..=HI {
            for b in LO..=HI {
                for c in LO..=HI {
                    total += 1;
                    let k1 = one_rounding(a, b, c, r);
                    let k2 = two_roundings(a, b, c, r, r);
                    let k2o = two_roundings(a, b, c, to_odd, r);
                    if k1 != k2 {
                        disagree_k2 += 1;
                        let d = (k1 - k2).abs();
                        if d > worst_k2 {
                            worst_k2 = d;
                            worst_case = Some((a, b, c, k1, k2));
                        }
                    }
                    if k1 != k2o {
                        disagree_k2_odd += 1;
                    }
                }
            }
        }

        println!("--- intermediate and final rounding rule: {label} ---");
        println!("  cases                                    {total}");
        println!("  k=1 vs k=2   disagree                    {disagree_k2}");
        println!("  k=1 vs k=2'  disagree (round-to-odd mid) {disagree_k2_odd}");
        println!("  worst k=2 error, in raw units            {worst_k2}");
        if let Some((a, b, c, k1, k2)) = worst_case {
            println!(
                "  worst case  a={a} b={b} c={c}  ->  k=1 gives {k1}, k=2 gives {k2}   (raw, Q2.2)"
            );
        }
        println!();
    }

    // The single case worth reading aloud: an exact product that only one of
    // the two evaluations can see.
    let (a, b, c) = (1i64, 1i64, 6i64); // 0.25 * 0.25 * 1.5
    println!("worked case a=0.25 b=0.25 c=1.50 (raws {a},{b},{c}), nearest-even:");
    println!(
        "  k=1 -> raw {}  ({:.4})",
        one_rounding(a, b, c, near_even),
        one_rounding(a, b, c, near_even) as f64 / 4.0
    );
    println!(
        "  k=2 -> raw {}  ({:.4})",
        two_roundings(a, b, c, near_even, near_even),
        two_roundings(a, b, c, near_even, near_even) as f64 / 4.0
    );
    println!(
        "  k=2 with round-to-odd intermediate -> raw {}",
        two_roundings(a, b, c, to_odd, near_even)
    );
    println!("  exact value 0.09375");
}
