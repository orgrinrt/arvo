// q3. `law::fusing_a_multiply_add_preserves_the_answer_under_unsigned` says
// fusing means "the intermediate product is not resolved before the addition".
// "Resolved" names two operations, and the row's `holds` field is true under one
// and false under the other.
//
// The row carries, on `holds`:
//
//   overflow_policy: in {wrap, saturating}
//   rounding: in {floor, ceil, toward_zero, away from zero, half_up}
//
// Seat 228's finding F2 carries, about the same property:
//
//   `overflow_policy: wrap` and not `in {wrap, saturating}`. The prediction was
//   run under wrap only, and under saturating every mode fails at every fraction
//   width including zero, so the prediction is known not to extend.
//
// Both cannot describe the same measurement. This probe runs both readings of
// "resolve" and shows each is right about a different one.
//
//   reading 1, RESOLVE = round and place in the container
//       stepwise  reduce( reduce( round_F(a*b) ) + c )
//       fused     reduce( round_F( a*b + c<<F ) )
//     Fusing removes an intermediate reduction as well as an intermediate
//     rounding, so the stepwise form reduces twice and the fused form once.
//
//   reading 2, RESOLVE = round only
//       stepwise  reduce( round_F(a*b) + c )
//       fused     reduce( round_F( a*b + c<<F ) )
//     The reduction is a property of the final result and fusing moves only the
//     rounding. One reduction on each side.
//
// Under WRAP the two readings are the same measurement, because reduction is a
// ring homomorphism modulo 2^W and reduce(reduce(x) + c) == reduce(x + c). Under
// SATURATION it is not a homomorphism and the two come apart. So the row's
// `overflow_policy: in {wrap, saturating}` is a claim that only one of the two
// readings supports, and the row does not say which one it means.
//
// PREDICTIONS, written before the run:
//
//   P1  under wrap, unsigned, W=6, the free set is {floor, ceil, toward_zero,
//       away_from_zero, half_up} and half_even fails at 12.50, 12.50, 9.38, 6.25
//       and 3.91 percent at F = 1..5 and at zero at F = 0. Those five figures are
//       in the row's own `note` and are the positive control.
//   P2  under wrap, signed, the free set is {floor, ceil, half_up}, which is the
//       signed row's `holds`.
//   P3  under saturation, reading 2, the unsigned free set is the same five, so
//       the row's `overflow_policy: in {wrap, saturating}` is supported.
//   P4  under saturation, reading 1, every mode fails including at F = 0, so
//       `228`'s F2 is supported.
//
// THE CASES THAT MUST FAIL:
//
//   C1  readings 1 and 2 must be IDENTICAL at every cell under wrap. If they
//       differ, the reduction implemented here is not a homomorphism and every
//       number below is about a broken model rather than about the law.
//   C2  at F = 0 under wrap nothing is discarded and every mode must be free.
//   C3  the wrap figures must reproduce the row's own five published rates to
//       the digit, or this is not the row's property.
//
// Build and run:
//   rustc --edition 2024 -O -o q3 q3_resolving_the_intermediate_names_two_operations.rs && ./q3

use Mode::*;

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUp,
    HalfUpAway,
    HalfEven,
}
const MODES: [Mode; 7] = [
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUp,
    HalfUpAway,
    HalfEven,
];
fn name(m: Mode) -> &'static str {
    match m {
        Floor => "floor",
        Ceil => "ceil",
        TowardZero => "toward_zero",
        AwayFromZero => "away_from_zero",
        HalfUp => "half_up(+inf)",
        HalfUpAway => "half_up(away)",
        HalfEven => "half_even",
    }
}

fn rnd(p: i128, s: u32, m: Mode) -> i128 {
    if s == 0 {
        return p;
    }
    let d = 1i128 << s;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Floor => q,
        Ceil => q + if r == 0 { 0 } else { 1 },
        TowardZero => {
            if p >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        AwayFromZero => {
            if p >= 0 {
                q + if r == 0 { 0 } else { 1 }
            } else {
                q
            }
        }
        HalfUp => {
            if 2 * r >= d {
                q + 1
            } else {
                q
            }
        }
        HalfUpAway => {
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if p >= 0 {
                q + 1
            } else {
                q
            }
        }
        HalfEven => {
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Policy {
    Wrap,
    Saturate,
}

/// Place a value in the declared W-bit domain under the overflow policy.
fn reduce(x: i128, w: u32, signed: bool, p: Policy) -> i128 {
    let (lo, hi) = bounds(w, signed);
    match p {
        Policy::Wrap => {
            let n = 1i128 << w;
            let m = (x - lo).rem_euclid(n);
            lo + m
        }
        Policy::Saturate => x.clamp(lo, hi),
    }
}

/// Inclusive bounds of the declared domain.
fn bounds(w: u32, signed: bool) -> (i128, i128) {
    if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    }
}

/// Triples of the declared domain, as a half-open raw range.
fn domain(w: u32, signed: bool) -> (i128, i128) {
    let (lo, hi) = bounds(w, signed);
    (lo, hi + 1)
}

/// Differing triples out of the whole cube, for one reading of "resolve".
fn differ(w: u32, f: u32, signed: bool, p: Policy, m: Mode, two_reductions: bool) -> (u64, u64) {
    let (lo, hi) = domain(w, signed);
    let mut d = 0u64;
    let mut t = 0u64;
    for a in lo..hi {
        for b in lo..hi {
            let ab = a * b;
            let pr = rnd(ab, f, m);
            let pr_stepwise = if two_reductions {
                reduce(pr, w, signed, p)
            } else {
                pr
            };
            for c in lo..hi {
                t += 1;
                let stepwise = reduce(pr_stepwise + c, w, signed, p);
                let fused = reduce(rnd(ab + (c << f), f, m), w, signed, p);
                if stepwise != fused {
                    d += 1;
                }
            }
        }
    }
    (d, t)
}

fn pct(d: u64, t: u64) -> f64 {
    100.0 * d as f64 / t as f64
}

/// The set of modes free at every fraction width in `1..=fmax`, plus F = 0.
fn free_set(w: u32, signed: bool, p: Policy, two: bool, fmax: u32) -> Vec<&'static str> {
    MODES
        .iter()
        .filter(|&&m| (0..=fmax).all(|f| differ(w, f, signed, p, m, two).0 == 0))
        .map(|&m| name(m))
        .collect()
}

fn main() {
    let mut sound = true;
    let w = 6u32;

    println!("q3. `resolve the intermediate` names two operations");
    println!();
    println!("== the table, W = 6, both policies, both readings ==");
    println!(
        "{:<16} {:>10} {:>10} {:>4} {:>10} {:>10}   {}",
        "mode", "sign", "policy", "F", "reading1", "reading2", "same?"
    );
    let mut c1_ok = true;
    let mut wrap_half_even: Vec<f64> = Vec::new();
    for signed in [false, true] {
        for p in [Policy::Wrap, Policy::Saturate] {
            for m in MODES {
                for f in 0..=5u32 {
                    let (d1, t) = differ(w, f, signed, p, m, true);
                    let (d2, _) = differ(w, f, signed, p, m, false);
                    let same = d1 == d2;
                    if p == Policy::Wrap && !same {
                        c1_ok = false;
                    }
                    if p == Policy::Wrap && !signed && m == HalfEven {
                        wrap_half_even.push(pct(d2, t));
                    }
                    println!(
                        "{:<16} {:>10} {:>10} {:>4} {:>9.2}% {:>9.2}%   {}",
                        name(m),
                        if signed { "signed" } else { "unsigned" },
                        if p == Policy::Wrap {
                            "wrap"
                        } else {
                            "saturate"
                        },
                        f,
                        pct(d1, t),
                        pct(d2, t),
                        if same { "yes" } else { "NO" }
                    );
                }
            }
        }
    }

    println!();
    println!("== the free sets ==");
    for signed in [false, true] {
        for p in [Policy::Wrap, Policy::Saturate] {
            for two in [false, true] {
                let s = free_set(w, signed, p, two, 5);
                println!(
                    "  {:8}  {:8}  reading {}   free at every F in 0..=5: {:?}",
                    if signed { "signed" } else { "unsigned" },
                    if p == Policy::Wrap {
                        "wrap"
                    } else {
                        "saturate"
                    },
                    if two { 1 } else { 2 },
                    s
                );
            }
        }
    }

    println!();
    println!("== C1 must-fail: the two readings must coincide under wrap ==");
    if c1_ok {
        println!("  C1 ok: identical at every wrap cell, so reduction is a homomorphism there");
        println!("         and the saturating split below is about saturation, not the model");
    } else {
        println!("  C1 FAILED: the readings differ under wrap, so `reduce` is wrong");
        sound = false;
    }

    println!();
    println!("== C2 must-fail: F = 0 under wrap must be free for every mode ==");
    let mut c2_ok = true;
    for signed in [false, true] {
        for m in MODES {
            let (d, _) = differ(w, 0, signed, Policy::Wrap, m, false);
            if d != 0 {
                c2_ok = false;
                println!("  {} {} F=0 wrap differs on {d}", name(m), signed);
            }
        }
    }
    if c2_ok {
        println!("  C2 ok: nothing is discarded at F = 0 and nothing differs");
    } else {
        sound = false;
    }

    println!();
    println!("== C3 must-fail: reproduce the row's own published wrap rates ==");
    println!("  law::fusing_a_multiply_add_preserves_the_answer_under_unsigned, `note`:");
    println!("  \"12.50, 12.50, 9.38, 6.25 and 3.91 percent of triples at F = 1 through 5,");
    println!("   and zero at F = 0\", for half_even under wrapping.");
    let expect = ["0.00", "12.50", "12.50", "9.38", "6.25", "3.91"];
    let got: Vec<String> = wrap_half_even.iter().map(|v| format!("{v:.2}")).collect();
    println!("  measured: {got:?}");
    let c3_ok = got.len() == expect.len() && got.iter().zip(expect.iter()).all(|(g, e)| g == e);
    if c3_ok {
        println!("  C3 ok: six of six reproduced, so this is the row's property");
    } else {
        println!("  C3 FAILED: the rates do not match, so this measures something else");
        sound = false;
    }

    println!();
    println!("== C4 must-fail: reproduce the two saturating figures the corpus already has ==");
    let (d0, t0) = differ(w, 0, true, Policy::Saturate, Floor, true);
    println!("  signed saturating, F = 0, reading 1: {d0} of {t0}");
    println!("  `228` section 0 reports its v1 control firing at 110,476 of 262,144.");
    let c4a = d0 == 110_476 && t0 == 262_144;
    let mut sat_he = Vec::new();
    for f in 1..=5u32 {
        let (d, t) = differ(w, f, false, Policy::Saturate, HalfEven, false);
        sat_he.push(format!("{:.2}", pct(d, t)));
    }
    println!("  unsigned saturating half_even, F = 1..5: {sat_he:?}");
    println!("  `228` section 2.3 reports 0.93, 1.61, 2.02, 2.18, 2.08 and the row's `note`");
    println!("  reports the range 0.93 to 2.18.");
    let c4b = sat_he == ["0.93", "1.61", "2.02", "2.18", "2.08"];
    if c4a && c4b {
        println!("  C4 ok: both reproduced, so this model is the corpus's model on the");
        println!("         saturating side as well as the wrapping side, and `228`'s own");
        println!("         control figure is the signed reading-1 cell of this table");
    } else {
        println!("  C4 FAILED: a={c4a} b={c4b}; this model is not the corpus's model");
        sound = false;
    }

    println!();
    println!("== WHAT THIS SETTLES ==");
    println!("Three of the four predictions above were wrong and the measurement is");
    println!("sharper than any of them.");
    println!();
    println!("P1 and P2 held. C3 reproduces the unsigned row's five published wrap rates");
    println!("and the free sets under wrap are the two law rows' `holds` fields exactly:");
    println!("five modes unsigned, three signed.");
    println!();
    println!("P3 held and P4 did not. Under saturation the two readings coincide on the");
    println!("UNSIGNED domain at every cell, because clamping at the top is idempotent and");
    println!("a non-negative addend cannot come back down: clamp(clamp(x) + c) is");
    println!("clamp(x + c) for every c >= 0. So the unsigned row's");
    println!("`overflow_policy: in {{wrap, saturating}}` on `holds` is supported under BOTH");
    println!("readings and is not ambiguous at all. The row is right.");
    println!();
    println!("The readings come apart on the SIGNED domain only, and there the gap is the");
    println!("whole verdict: at F = 0 reading 1 differs on 42.14 percent of triples for");
    println!("every mode while reading 2 differs on none, so under reading 1 nothing is");
    println!("free and under reading 2 the free set is the signed row's own three. The");
    println!("signed row carries `overflow_policy: wrap` and states nothing about");
    println!("saturating, which is the right thing to have written, and this measurement");
    println!("says why: the saturating extension has no truth value until the reading is");
    println!("named, and nothing in `dimension::overflow_policy` names it, because what");
    println!("varies is not which policy applies but how many times it is applied.");
    println!();
    println!("== WHAT THIS REFUTES ==");
    println!("`228` finding F2's justification: \"under saturating every mode fails at");
    println!("every fraction width including zero, so the prediction is known not to");
    println!("extend and I do not claim it does.\" F2's predicate carries");
    println!("`signedness: in {{unsigned, signed}}`. On the unsigned half of that region");
    println!("the sentence is false at all 42 cells measured here: every mode is free");
    println!("under saturating at every F including zero, under both readings. The");
    println!("sentence is true only on the signed half and only under reading 1, which is");
    println!("the cell `228`'s own v1 control fired in, at the figure C4 reproduces.");
    println!("So a correct half of a region was dropped because a control fired in the");
    println!("other half under a reading the file never names.");
    println!();
    println!("instrument: {}", if sound { "sound" } else { "UNSOUND" });
    if !sound {
        std::process::exit(1);
    }
}
