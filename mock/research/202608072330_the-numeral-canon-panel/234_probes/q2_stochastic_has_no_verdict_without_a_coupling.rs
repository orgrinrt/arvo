// q2. `228`'s finding F4 writes, on the `fails` side of
// `law::rounding_retraction_is_the_identity`:
//
//   rounding: rounding any: swept, five ratified deterministic modes and
//   away-from-zero at all 108 cells, stochastic by construction since the eager
//   step is then not a function of the triple
//
// The clause is offered as a `construction` warrant, which
// `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` defines as
// "an axis that cannot enter the argument at all, with the clause saying what
// makes it unable to". This probe asks whether the clause is true.
//
// It is not, and it fails in the direction that matters. "The eager step is not
// a function of the triple" is correct and does not license a `fails` verdict:
// a randomised rule turns the comparison into a question about a JOINT
// distribution over the draws, and the answer depends on how the draws at the
// three quantisation points are coupled. Two couplings, both defensible, give
// two different counts, so the row cannot name `stochastic` in `fails` without
// naming a coupling, and no instrument in this panel names one.
//
// The chain has three quantisation points:
//
//   eager    = rnd_F( rnd_F(a*b) ; u1 ) * c , then rnd_F( . ; u2 )
//   deferred = rnd_2F( a*b*c ; u3 )
//
// Proportional stochastic rounding, in the hardware realisation: draw u
// uniformly from [0, 2^s) and compute (p + u) >> s. That rounds up exactly when
// the discarded residue r satisfies r + u >= 2^s, so up with probability
// r / 2^s, which is Parker's definition.
//
//   coupling A, independent   u1, u2, u3 range freely and independently
//   coupling B, shared bits   u3 is built from the same bits the eager side
//                             used: u3 = u1 * 2^F + u2, so the deferred draw is
//                             the concatenation of the two eager draws
//
// Both are what an implementation might actually do. B is what a single shared
// entropy source feeding a 2F-bit field looks like; A is two independent calls.
//
// THE CASES THAT MUST FAIL, stated before the run:
//
//   C1  the difference between the two couplings must be about the draw and not
//       about the harness. Replacing the proportional rule with a DEGENERATE
//       draw distribution that always rounds up off-grid makes the mode
//       deterministic (it is `ceil`), and the two couplings must then report the
//       IDENTICAL rate. If they differ, the harness distinguishes couplings for
//       a reason that has nothing to do with randomness and nothing below counts.
//   C2  at F = 0 nothing is discarded, so no draw can act, and both couplings
//       must report exactly zero under every rule. A nonzero there means the
//       draw is being applied where there is no residue.
//   C3  the proportional rule must reproduce a deterministic mode when its draw
//       space is collapsed: forcing u = 2^s - 1 must give `ceil`'s count and
//       forcing u = 0 must give `floor`'s count, both already measured in q1.
//       Without this the stochastic arm is not known to be the same comparison.
//
// Build and run:
//   rustc --edition 2024 -O -o q2 q2_stochastic_has_no_verdict_without_a_coupling.rs && ./q2

/// Which draw rule the run is using at a quantisation point.
#[derive(Copy, Clone, PartialEq)]
enum Draw {
    /// Proportional: u ranges over the whole draw space.
    Proportional,
    /// The draw space collapsed to its top value, so every off-grid value goes
    /// up. Deterministically `ceil`.
    AlwaysUp,
    /// The draw space collapsed to zero, so nothing ever goes up.
    /// Deterministically `floor`.
    AlwaysDown,
}

/// `(p + u) >> s`, the hardware realisation of proportional stochastic
/// rounding, with the draw supplied.
fn rnd_with(p: i128, s: u32, u: i128) -> i128 {
    if s == 0 {
        return p;
    }
    (p + u).div_euclid(1i128 << s)
}

/// The draw space at a quantisation point of `s` bits under a given rule.
fn draws(s: u32, d: Draw) -> Vec<i128> {
    if s == 0 {
        return vec![0];
    }
    let n = 1i128 << s;
    match d {
        Draw::Proportional => (0..n).collect(),
        Draw::AlwaysUp => vec![n - 1],
        Draw::AlwaysDown => vec![0],
    }
}

fn domain(w: u32, signed: bool) -> (i128, i128) {
    if signed {
        (-(1i128 << (w - 1)), 1i128 << (w - 1))
    } else {
        (0, 1i128 << w)
    }
}

/// Coupling A. `u1`, `u2` and `u3` are independent.
///
/// Returns (differing outcomes, total outcomes) over triples crossed with the
/// whole joint draw space.
fn independent(w: u32, f: u32, signed: bool, d: Draw) -> (u64, u64) {
    let (lo, hi) = domain(w, signed);
    let us = draws(f, d);
    let u3s = draws(2 * f, d);
    let mut differ = 0u64;
    let mut total = 0u64;
    for a in lo..hi {
        for b in lo..hi {
            let ab = a * b;
            for c in lo..hi {
                for &u1 in &us {
                    let ab_q = rnd_with(ab, f, u1);
                    for &u2 in &us {
                        let eager = rnd_with(ab_q * c, f, u2);
                        for &u3 in &u3s {
                            total += 1;
                            if eager != rnd_with(ab * c, 2 * f, u3) {
                                differ += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    (differ, total)
}

/// Coupling B. The deferred draw is the concatenation of the two eager draws,
/// `u3 = u1 * 2^F + u2`, which is what one shared entropy source feeding a
/// 2F-bit field produces.
fn shared_bits(w: u32, f: u32, signed: bool, d: Draw) -> (u64, u64) {
    let (lo, hi) = domain(w, signed);
    let us = draws(f, d);
    let mut differ = 0u64;
    let mut total = 0u64;
    for a in lo..hi {
        for b in lo..hi {
            let ab = a * b;
            for c in lo..hi {
                for &u1 in &us {
                    let ab_q = rnd_with(ab, f, u1);
                    for &u2 in &us {
                        let eager = rnd_with(ab_q * c, f, u2);
                        let u3 = if f == 0 { 0 } else { u1 * (1i128 << f) + u2 };
                        total += 1;
                        if eager != rnd_with(ab * c, 2 * f, u3) {
                            differ += 1;
                        }
                    }
                }
            }
        }
    }
    (differ, total)
}

fn pct(d: u64, t: u64) -> f64 {
    100.0 * d as f64 / t as f64
}

fn main() {
    let mut sound = true;

    println!("q2. a stochastic mode has no retraction verdict until a coupling is named");
    println!();
    println!("== the two couplings, proportional draw ==");
    println!(
        "{:>3} {:>3} {:>9} {:>16} {:>9} {:>16} {:>9}   {}",
        "W", "F", "sign", "A independent", "rate", "B shared bits", "rate", "same?"
    );
    let mut ever_differed = false;
    for &(w, signed) in &[(4u32, false), (4, true), (6, false)] {
        for f in 0..=3u32 {
            let (da, ta) = independent(w, f, signed, Draw::Proportional);
            let (db, tb) = shared_bits(w, f, signed, Draw::Proportional);
            let same = (pct(da, ta) - pct(db, tb)).abs() < 1e-12;
            if !same {
                ever_differed = true;
            }
            println!(
                "{:>3} {:>3} {:>9} {:>16} {:>8.4}% {:>16} {:>8.4}%   {}",
                w,
                f,
                if signed { "signed" } else { "unsigned" },
                da,
                pct(da, ta),
                db,
                pct(db, tb),
                if same { "yes" } else { "NO" }
            );
        }
    }
    println!();
    if !ever_differed {
        println!("  the two couplings never differed, which would leave 228's clause standing");
        sound = false;
    } else {
        println!("  The two couplings disagree. `does this retract` is therefore not a");
        println!("  property of the triple and the mode; it is a property of the triple,");
        println!("  the mode and the coupling. A `fails` region naming `stochastic` and no");
        println!("  coupling states no region, which is the defect 228 set out to repair");
        println!("  and reintroduced on the same field.");
    }

    println!();
    println!("== C1 must-fail: collapse the draw and the couplings must agree ==");
    let mut c1_ok = true;
    for &(w, signed) in &[(4u32, false), (4, true)] {
        for f in 1..=3u32 {
            for d in [Draw::AlwaysUp, Draw::AlwaysDown] {
                let (da, ta) = independent(w, f, signed, d);
                let (db, tb) = shared_bits(w, f, signed, d);
                let agree = da * (tb as u64) == db * (ta as u64);
                if !agree {
                    c1_ok = false;
                }
                println!(
                    "  W={w} F={f} {:8} {:11} A {:>8.4}%  B {:>8.4}%  {}",
                    if signed { "signed" } else { "unsigned" },
                    if d == Draw::AlwaysUp {
                        "always-up"
                    } else {
                        "always-down"
                    },
                    pct(da, ta),
                    pct(db, tb),
                    if agree { "agree" } else { "DISAGREE" }
                );
            }
        }
    }
    if c1_ok {
        println!("  C1 ok: with the randomness removed the couplings are indistinguishable,");
        println!("         so the disagreement above is about the draw and not the harness");
    } else {
        println!("  C1 FAILED: the couplings differ even on a deterministic rule, so the");
        println!("             harness is what distinguishes them and q2 measures nothing");
        sound = false;
    }

    println!();
    println!("== C2 must-fail: F = 0 must be zero under every rule and both couplings ==");
    let mut c2_ok = true;
    for &(w, signed) in &[(4u32, false), (4, true), (6, false)] {
        for d in [Draw::Proportional, Draw::AlwaysUp, Draw::AlwaysDown] {
            let (da, _) = independent(w, 0, signed, d);
            let (db, _) = shared_bits(w, 0, signed, d);
            if da != 0 || db != 0 {
                c2_ok = false;
                println!("  W={w} F=0 nonzero: A={da} B={db}");
            }
        }
    }
    if c2_ok {
        println!("  C2 ok: every F = 0 cell is zero, so no draw acts where no residue exists.");
        println!("         The `holds` side's construction argument survives for stochastic;");
        println!("         it is the `fails` side that does not.");
    } else {
        sound = false;
    }

    println!();
    println!("== C3 must-fail: the collapsed draws must reproduce q1's floor and ceil ==");
    println!("  q1, unsigned W=4: floor 800 1128 910 543 and ceil 800 1128 925 495 at F=1..4");
    let mut c3 = Vec::new();
    for f in 1..=3u32 {
        let (down, t) = independent(4, f, false, Draw::AlwaysDown);
        let (up, _) = independent(4, f, false, Draw::AlwaysUp);
        println!("    F={f}  always-down {down} of {t}   always-up {up} of {t}");
        c3.push((down, up));
    }
    let expect_floor = [800u64, 1128, 910];
    let expect_ceil = [800u64, 1128, 925];
    let c3_ok = c3
        .iter()
        .zip(expect_floor.iter().zip(expect_ceil.iter()))
        .all(|((d, u), (ef, ec))| d == ef && u == ec);
    if c3_ok {
        println!("  C3 ok: the collapsed stochastic rule reproduces floor and ceil exactly,");
        println!("         so this is the same comparison q1 and `94_probes/c_retraction` run");
    } else {
        println!("  C3 FAILED: the collapsed rule does not reproduce the deterministic counts,");
        println!("             so the stochastic arm is measuring a different comparison");
        sound = false;
    }

    println!();
    println!("== WHAT THIS REFUTES ==");
    println!("`228` F4, the `fails` side clause \"stochastic by construction since the");
    println!("eager step is then not a function of the triple\". The premise is true and");
    println!("the conclusion does not follow: the comparison becomes a question about a");
    println!("joint distribution, two defensible couplings give two answers, and neither");
    println!("`228` nor any instrument in this panel names one. The clause therefore does");
    println!("not discharge the `construction` warrant that");
    println!("`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` defines,");
    println!("which obliges an instrument that varied the axis and found NO movement.");
    println!("This instrument varied it and found movement.");
    println!();
    println!("instrument: {}", if sound { "sound" } else { "UNSOUND" });
    if !sound {
        std::process::exit(1);
    }
}
