// p2: the fusion arm re-measured over seven modes, where the corpus swept six.
//
// The two rows this bears on say fusing a multiply-add is answer-preserving on
// a stated set of rounding positions. The set is written in six names, one of
// which, `away from zero`, is outside the ratified vocabulary, and one of
// which, `nearest-half-up`, is inside it under a name that does not say which
// of two operations it is.
//
// So this runs both readings of `half_up` beside the rest:
//
//   half_up(+inf)  floor(x + 1/2), which is what every instrument in the corpus
//                  implements, `149_probes/y1` included, in those words.
//   half_up(away)  ties away from zero, which is what Java's RoundingMode.HALF_UP
//                  and Python decimal's ROUND_HALF_UP mean by the same name.
//
// The arm. Stepwise resolves the product to the declared format and then adds:
// `reduce(reduce(rnd(A*B)) + C)`. Fused carries the product at its own scale
// into the addition and resolves once: `reduce(rnd(A*B + C*2^F))`. Exhaustive
// over every triple the declared container holds.
//
// FAITHFULNESS. The instrument is checked against numbers this file did not
// choose, published in `law::fusing_a_multiply_add_preserves_the_answer_under_
// signed_wrapping`: toward-zero at 1.64, 5.54, 12.34, 22.22, 33.40 percent and
// half-even at 12.50, 12.50, 9.38, 6.25, 3.91, at W = 6, F = 1 to 5. If these
// digits do not come back, this implementation is measuring something else and
// every other line here is void.
//
// Run: rustc --edition 2024 -O p2_fusion_over_both_half_up_readings.rs -o /tmp/p2 && /tmp/p2

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Floor,
    Ceiling,
    TowardZero,
    AwayFromZero,
    HalfUpPinf,
    HalfUpAway,
    HalfEven,
}
use Mode::*;

const MODES: [Mode; 7] = [
    Floor, Ceiling, TowardZero, AwayFromZero, HalfUpPinf, HalfUpAway, HalfEven,
];

fn name(m: Mode) -> &'static str {
    match m {
        Floor => "floor",
        Ceiling => "ceil",
        TowardZero => "toward_zero",
        AwayFromZero => "away_from_zero",
        HalfUpPinf => "half_up(+inf)",
        HalfUpAway => "half_up(away)",
        HalfEven => "half_even",
    }
}

/// Is this one of the six names op ratified?
fn ratified(m: Mode) -> bool {
    !matches!(m, AwayFromZero)
}

/// Round p / 2^f to an integer. Exact integer arithmetic, written from the
/// definitions.
fn rnd(p: i128, f: u32, m: Mode) -> i128 {
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Floor => q,
        Ceiling => if r == 0 { q } else { q + 1 },
        TowardZero => if p >= 0 || r == 0 { q } else { q + 1 },
        AwayFromZero => {
            if p >= 0 {
                if r == 0 { q } else { q + 1 }
            } else {
                q
            }
        }
        HalfUpPinf => if 2 * r >= d { q + 1 } else { q },
        HalfUpAway => {
            if p >= 0 {
                if 2 * r >= d { q + 1 } else { q }
            } else {
                // ties away from zero on the negative side: round the magnitude
                if 2 * r > d { q + 1 } else { q }
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

fn reduce(v: i128, signed: bool, sat: bool, w: u32) -> i128 {
    let (lo, hi) = if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    };
    if sat {
        v.clamp(lo, hi)
    } else {
        let m = 1i128 << w;
        let mut r = v.rem_euclid(m);
        if signed && r > hi {
            r -= m;
        }
        r
    }
}

/// Percentage of triples where stepwise and fused disagree.
/// `fused_mode` is normally the same as `m`; the mutation control passes a
/// different one.
fn measure(w: u32, f: u32, m: Mode, fused_mode: Mode, signed: bool, sat: bool) -> (u64, u64) {
    let (lo, hi) = if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    };
    let scale = 1i128 << f;
    let mut differ: u64 = 0;
    let mut total: u64 = 0;
    for a in lo..=hi {
        for b in lo..=hi {
            let p = a * b; // scale 2f
            let step_t = reduce(rnd(p, f, m), signed, sat, w);
            for c in lo..=hi {
                total += 1;
                let stepwise = reduce(step_t + c, signed, sat, w);
                let fused = reduce(rnd(p + c * scale, f, fused_mode), signed, sat, w);
                if stepwise != fused {
                    differ += 1;
                }
            }
        }
    }
    (differ, total)
}

/// Translation equivariance of `rnd(., f, m)` on the domain the cell reaches.
/// The domain is the set of products A*B, so under unsigned it is the
/// non-negative scaled integers and under signed it is both signs. Translations
/// are by C grid steps for C in the container range, which is what the fused
/// form applies.
fn equivariant_on_domain(w: u32, f: u32, m: Mode, signed: bool) -> bool {
    if f == 0 {
        return true;
    }
    let (lo, hi) = if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    };
    let scale = 1i128 << f;
    for a in lo..=hi {
        for b in lo..=hi {
            let p = a * b;
            let base = rnd(p, f, m);
            for c in lo..=hi {
                if rnd(p + c * scale, f, m) != base + c {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let w: u32 = 6;
    println!("p2: fusing a multiply-add, seven modes, W = {w}");
    println!("    exhaustive over every triple the declared container holds");
    println!("    stepwise reduce(reduce(rnd(A*B)) + C) against fused reduce(rnd(A*B + C*2^F))");
    println!();

    // ---- faithfulness against numbers this file did not choose -------------
    println!("## 0. faithfulness. Published signed-wrapping rates, re-derived here.");
    println!();
    let published_tz = [1.64f64, 5.54, 12.34, 22.22, 33.40];
    let published_he = [12.50f64, 12.50, 9.38, 6.25, 3.91];
    println!("{:<14} {:>3} {:>12} {:>12}  {}", "mode", "F", "published", "measured", "verdict");
    let mut faithful = true;
    for f in 1..=5u32 {
        let (d, t) = measure(w, f, TowardZero, TowardZero, true, false);
        let pct = 100.0 * d as f64 / t as f64;
        let want = published_tz[(f - 1) as usize];
        let ok = (pct * 100.0).round() == (want * 100.0).round();
        if !ok { faithful = false; }
        println!("{:<14} {:>3} {:>11.2}% {:>11.2}%  {}", "toward_zero", f, want, pct,
            if ok { "matches" } else { "DIFFERS" });
    }
    for f in 1..=5u32 {
        let (d, t) = measure(w, f, HalfEven, HalfEven, true, false);
        let pct = 100.0 * d as f64 / t as f64;
        let want = published_he[(f - 1) as usize];
        let ok = (pct * 100.0).round() == (want * 100.0).round();
        if !ok { faithful = false; }
        println!("{:<14} {:>3} {:>11.2}% {:>11.2}%  {}", "half_even", f, want, pct,
            if ok { "matches" } else { "DIFFERS" });
    }
    println!();
    println!("  faithfulness: {}", if faithful {
        "ten of ten digits reproduced, so this implements the same arm"
    } else {
        "MISMATCH. This implements a different arm and the rest of the file is void."
    });
    println!();

    // ---- the table ---------------------------------------------------------
    for &(signed, sat, label) in [
        (false, false, "unsigned, wrap"),
        (false, true, "unsigned, saturating"),
        (true, false, "signed, wrap"),
        (true, true, "signed, saturating"),
    ].iter() {
        println!("## {label}");
        println!();
        print!("{:<16} {:<10}", "mode", "ratified");
        for f in 0..=5u32 { print!("{:>9}", format!("F={f}")); }
        println!("   free at");
        for m in MODES {
            print!("{:<16} {:<10}", name(m), if ratified(m) { "yes" } else { "no" });
            let mut free = Vec::new();
            for f in 0..=5u32 {
                let (d, t) = measure(w, f, m, m, signed, sat);
                let pct = 100.0 * d as f64 / t as f64;
                if d == 0 { free.push(f.to_string()); }
                if d == 0 { print!("{:>9}", "-"); } else { print!("{:>8.2}%", pct); }
            }
            println!("   F in {{{}}}", free.join(","));
        }
        println!();
    }

    // ---- the equivariance prediction, cell by cell -------------------------
    println!("## the prediction. Free exactly where rnd is equivariant on the domain reached.");
    println!();
    println!("{:<16} {:<9} {:>3} {:<14} {:<10} {}", "mode", "signed", "F", "equivariant", "free", "agree");
    let mut cells = 0;
    let mut mismatches = 0;
    for &signed in [false, true].iter() {
        for m in MODES {
            for f in 0..=5u32 {
                let eq = equivariant_on_domain(w, f, m, signed);
                let (d, _) = measure(w, f, m, m, signed, false);
                let free = d == 0;
                cells += 1;
                if eq != free {
                    mismatches += 1;
                    println!("{:<16} {:<9} {:>3} {:<14} {:<10} MISMATCH",
                        name(m), signed, f, eq, free);
                }
            }
        }
    }
    println!("  {cells} cells under wrap, {mismatches} mismatch(es) between the prediction and the measurement");
    println!();

    // ---- controls ----------------------------------------------------------
    println!("## controls");
    println!();
    let mut ok = true;

    // C1: at F = 0 the grid is the whole value set, so every mode is the
    // identity and the rounding axis cannot enter. The invariant is therefore
    // that all seven modes give the IDENTICAL count, not that the count is
    // zero. The first version of this probe asserted zero, and the control
    // fired: under signed saturating all seven are nonzero and equal, because
    // the stepwise form clamps twice and the fused form clamps once, which is
    // a difference in the overflow policy and not in the rounding.
    let mut c1_bad = 0;
    for &signed in [false, true].iter() {
        for &sat in [false, true].iter() {
            let counts: Vec<u64> = MODES.iter().map(|&m| measure(w, 0, m, m, signed, sat).0).collect();
            let first = counts[0];
            if counts.iter().any(|&c| c != first) { c1_bad += 1; }
            let tag = if sat { "saturating" } else { "wrap" };
            let sg = if signed { "signed" } else { "unsigned" };
            println!("     F=0 {sg:<9} {tag:<11} all seven modes: {counts:?}");
        }
    }
    if c1_bad == 0 {
        println!("  C1 EXPECTED-PASS ok: at F = 0 the rounding axis moves nothing, in all four cells");
    } else {
        println!("  C1 BROKEN: {c1_bad} cell(s) where modes differ at F = 0");
        ok = false;
    }

    // C1b mutation for C1: a rnd that is NOT the identity at F = 0 must break
    // the mode-independence C1 just asserted, or C1 is asserting nothing.
    fn rnd_broken_at_zero(p: i128, f: u32, m: Mode) -> i128 {
        if f == 0 { return p + 1; }
        rnd(p, f, m)
    }
    {
        let (lo, hi) = (0i128, (1i128 << w) - 1);
        let mut differ = 0u64;
        for a in lo..=hi { for b in lo..=hi { let pr = a * b;
            let t = reduce(rnd_broken_at_zero(pr, 0, Floor), false, false, w);
            for c in lo..=hi {
                let step = reduce(t + c, false, false, w);
                let fus = reduce(rnd(pr + c, 0, Floor), false, false, w);
                if step != fus { differ += 1; }
            } } }
        if differ > 0 {
            println!("  C1b EXPECTED-FAIL ok: a rnd that is not the identity at F = 0 differs on {differ} triples");
        } else {
            println!("  C1b BROKEN: the F = 0 arm cannot see a non-identity rnd");
            ok = false;
        }
    }

    // C2 mutation: fuse with a different mode than the stepwise used. A mode
    // reported free must stop being free, or the two sides are not compared.
    let (d, t) = measure(w, 3, Floor, Ceiling, true, false);
    if d > 0 {
        println!("  C2 EXPECTED-FAIL ok: floor stepwise against ceil fused differs on {d}/{t}");
    } else {
        println!("  C2 BROKEN: mutated arm still reported free");
        ok = false;
    }

    // C3: the two readings of half_up must differ somewhere signed, or the
    // half_up finding is about nothing.
    let mut c3 = false;
    for f in 1..=5u32 {
        let (a, _) = measure(w, f, HalfUpPinf, HalfUpPinf, true, false);
        let (b, _) = measure(w, f, HalfUpAway, HalfUpAway, true, false);
        if a != b { c3 = true; }
    }
    if c3 {
        println!("  C3 EXPECTED-FAIL ok: the two readings of half_up give different signed rates");
    } else {
        println!("  C3 BROKEN: the two readings of half_up are indistinguishable here");
        ok = false;
    }

    // C4: under unsigned the two readings must agree, since the collapse is the
    // whole claim. If they differ here, the collapse claim is wrong.
    let mut c4 = true;
    for f in 0..=5u32 {
        let (a, _) = measure(w, f, HalfUpPinf, HalfUpPinf, false, false);
        let (b, _) = measure(w, f, HalfUpAway, HalfUpAway, false, false);
        if a != b { c4 = false; }
    }
    if c4 {
        println!("  C4 EXPECTED-PASS ok: the two readings agree at every F under unsigned");
    } else {
        println!("  C4 BROKEN: the readings differ under unsigned, so the collapse claim is wrong");
        ok = false;
    }

    // C5: away_from_zero and toward_zero are negation conjugates, and under
    // signed wrap they are measured here at identical rates. Checked at the
    // count rather than at the percentage, because two different counts can
    // print the same two decimals.
    {
        let mut same = true;
        for f in 1..=5u32 {
            let (a, _) = measure(w, f, TowardZero, TowardZero, true, false);
            let (b, _) = measure(w, f, AwayFromZero, AwayFromZero, true, false);
            print!("     F={f}: toward_zero {a}, away_from_zero {b}{}", if a == b { "  equal\n" } else { "  DIFFER\n" });
            if a != b { same = false; }
        }
        println!("  C5 {}: the two conjugate modes {} under signed wrap",
            if same { "note" } else { "note" },
            if same { "carry identical counts" } else { "carry different counts" });
    }

    println!();
    println!("controls: {}", if ok && faithful { "clean, and the arm reproduces the published digits" } else { "BROKEN" });
}
