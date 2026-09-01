// k2. What the mode NAMES decide about two committed verdicts.
//
// k1 established which names denote one function on which domain. This asks
// what that costs: it re-measures the two verdicts the nine entries carry, over
// all seven candidate functions rather than over the two or five each original
// instrument swept, and reports where the answer depends on a reading of a name
// rather than on a measurement.
//
// Part A. Translation equivariance, which is the property the fusion rows say
//         the verdict is exactly.
// Part B. The fusion table at W = 6, reproducing `149_probes/y2_out.txt` on the
//         six modes it swept and extending it to the seventh.
// Part C. Chain retraction in fixed point, reproducing
//         `94_probes/c_retraction.out.txt` part 2 on the two modes it swept and
//         extending it to all seven.
//
// PREDICTIONS, written before the first run.
//
//   P1  The equivariant modes are exactly floor, ceil and half_up(+inf).
//       half_up(away) is NOT equivariant, so the two readings of the ratified
//       name `half_up` land on opposite sides of the property the fusion rows
//       are about.
//   P2  On signed wrapping the fusion difference is 0.00% for floor, ceil and
//       half_up(+inf), and nonzero for toward_zero, away_from_zero, half_even
//       AND half_up(away). So `law::..._under_signed_wrapping` holds under one
//       reading of `half_up` and fails under the other.
//   P3  On unsigned wrapping the difference is 0.00% for all six deterministic
//       modes except half_even, both readings of half_up included, so the
//       unsigned row does not depend on the reading.
//   P4  Retraction holds at F = 0 for every one of the seven and fails at every
//       F >= 1 for every one of the seven, so the retraction row's two entries
//       are not about the mode at all.
//
// CONTROLS. Each is a way this probe could be measuring something else.
//
//   R1  My floor column must reproduce `c_retraction`'s `truncate` column digit
//       for digit, because that probe's domain is non-negative and k1 measured
//       floor == toward_zero there. A mismatch means I am not running the same
//       experiment and nothing here transfers.
//   R2  My half_up(away) column must reproduce that probe's `nearest` column,
//       for the same reason: its `nearest` is ties away from zero.
//   R3  My fusion table must reproduce y2's published maxima on the six modes
//       it swept, at both signednesses.
//   R4  At F = 0 the fusion difference must be zero for every mode, or the
//       instrument is measuring something other than rounding.
//   R5  Some mode must be non-equivariant and some must be equivariant, or
//       part A is vacuous.
//
// Build and run:
//   rustc --edition 2024 -O -o k2 k2_the_two_verdicts_the_names_decide.rs && ./k2

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Mode {
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUpPosInf,
    HalfUpAway,
    HalfEven,
}

const MODES: [Mode; 7] = [
    Mode::Floor,
    Mode::Ceil,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::HalfUpPosInf,
    Mode::HalfUpAway,
    Mode::HalfEven,
];

fn name(m: Mode) -> &'static str {
    match m {
        Mode::Floor => "floor",
        Mode::Ceil => "ceil",
        Mode::TowardZero => "toward_zero",
        Mode::AwayFromZero => "away_from_zero",
        Mode::HalfUpPosInf => "half_up(+inf)",
        Mode::HalfUpAway => "half_up(away)",
        Mode::HalfEven => "half_even",
    }
}

/// Identical in body to k1's, on i64 for speed. The domains below stay far
/// from any i64 boundary, and control R1 pins that by reproducing a committed
/// i128 instrument's counts exactly.
fn rnd(p: i64, f: u32, m: Mode) -> i64 {
    if f == 0 {
        return p;
    }
    let d = 1i64 << f;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Mode::Floor => q,
        Mode::Ceil => q + if r == 0 { 0 } else { 1 },
        Mode::TowardZero => {
            if p >= 0 {
                q
            } else {
                q + if r == 0 { 0 } else { 1 }
            }
        }
        Mode::AwayFromZero => {
            if p >= 0 {
                q + if r == 0 { 0 } else { 1 }
            } else {
                q
            }
        }
        Mode::HalfUpPosInf => q + if 2 * r >= d { 1 } else { 0 },
        Mode::HalfUpAway => {
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
        Mode::HalfEven => {
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if q.rem_euclid(2) == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

fn reduce_wrap(v: i64, signed: bool, w: u32) -> i64 {
    let m = 1i64 << w;
    let r = v.rem_euclid(m);
    if signed && r >= (1i64 << (w - 1)) {
        r - m
    } else {
        r
    }
}

// ---------------------------------------------------------------- part A

/// round(p + c*2^f) == round(p) + c, over the window the W = 6 cell reaches.
fn equivariant(f: u32, m: Mode, num_lo: i64, num_hi: i64, c_lo: i64, c_hi: i64) -> bool {
    let d = 1i64 << f;
    for p in num_lo..=num_hi {
        for c in c_lo..=c_hi {
            if rnd(p + c * d, f, m) != rnd(p, f, m) + c {
                return false;
            }
        }
    }
    true
}

// ---------------------------------------------------------------- part B

/// Fraction of (a, b, c) triples where fusing the multiply-add changes the
/// answer. The model is `149_probes/y2`'s: a, b, c are raw grid values, the
/// product sits at scale 2^(2F), and the addend is lifted by 2^F to meet it.
fn fusion_rate(signed: bool, w: u32, f: u32, m: Mode) -> f64 {
    let (lo, hi) = if signed {
        (-(1i64 << (w - 1)), (1i64 << (w - 1)) - 1)
    } else {
        (0, (1i64 << w) - 1)
    };
    let mut n = 0u64;
    let mut diff = 0u64;
    for a in lo..=hi {
        for b in lo..=hi {
            let p = a * b;
            let t = reduce_wrap(rnd(p, f, m), signed, w);
            for c in lo..=hi {
                n += 1;
                let stepwise = reduce_wrap(t + c, signed, w);
                let fused = reduce_wrap(rnd(p + (c << f), f, m), signed, w);
                if stepwise != fused {
                    diff += 1;
                }
            }
        }
    }
    100.0 * diff as f64 / n as f64
}

// ---------------------------------------------------------------- part C

/// `94_probes/c_retraction` part 2, over an arbitrary mode. Non-negative
/// domain, two-multiply chain, eager rescale at every step against one rescale
/// at the end.
fn retraction_differ(w: u32, f: u32, m: Mode) -> (u64, u64) {
    let n: i64 = 1 << w;
    let mut differ = 0u64;
    let mut total = 0u64;
    for a in 0..n {
        for b in 0..n {
            let ab_exact = a * b;
            let ab_q = rnd(ab_exact, f, m);
            for c in 0..n {
                total += 1;
                let eager = rnd(ab_q * c, f, m);
                let deferred = rnd(ab_exact * c, 2 * f, m);
                if eager != deferred {
                    differ += 1;
                }
            }
        }
    }
    (differ, total)
}

fn main() {
    println!("k2. what the mode names decide about two committed verdicts");

    // ------------------------------------------------------------ part A
    println!();
    println!("== part A: translation equivariance, W = 6 window ==");
    let mut equi: Vec<Mode> = Vec::new();
    for m in MODES {
        // the window the W = 6 signed cell reaches: products of two raw values
        // and shifts over the container range.
        let mut all = true;
        for f in 1u32..=5 {
            if !equivariant(f, m, -128, 128, -32, 31) {
                all = false;
                break;
            }
        }
        println!("  {:<16} equivariant: {}", name(m), all);
        if all {
            equi.push(m);
        }
    }
    let p1 = equi.len() == 3
        && equi.contains(&Mode::Floor)
        && equi.contains(&Mode::Ceil)
        && equi.contains(&Mode::HalfUpPosInf);
    println!(
        "P1 the equivariant set is exactly {{floor, ceil, half_up(+inf)}}: {}",
        if p1 { "HOLDS" } else { "REFUTED" }
    );
    let r5 = !equi.is_empty() && equi.len() < MODES.len();
    println!(
        "R5 the property separates the modes rather than accepting or rejecting all: {}",
        if r5 { "pass" } else { "FAIL" }
    );

    // ------------------------------------------------------------ part B
    println!();
    println!("== part B: fusion difference, wrapping, W = 6, exhaustive triples ==");
    println!(
        "{:<16} {:<10} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "mode", "signedness", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
    );
    let mut max_rate = std::collections::BTreeMap::new();
    let mut r4 = true;
    for &signed in &[false, true] {
        for m in MODES {
            print!(
                "{:<16} {:<10}",
                name(m),
                if signed { "signed" } else { "unsigned" }
            );
            let mut mx: f64 = 0.0;
            for f in 0u32..=5 {
                let r = fusion_rate(signed, 6, f, m);
                if f == 0 && r != 0.0 {
                    r4 = false;
                }
                if r > mx {
                    mx = r;
                }
                print!("{:>7.2}%", r);
            }
            println!();
            max_rate.insert((signed, name(m)), mx);
        }
    }
    println!(
        "R4 every mode is free at F = 0, so the sweep measures rounding: {}",
        if r4 { "pass" } else { "FAIL" }
    );

    let z = |s: bool, m: Mode| *max_rate.get(&(s, name(m))).unwrap() == 0.0;
    let p2 = z(true, Mode::Floor)
        && z(true, Mode::Ceil)
        && z(true, Mode::HalfUpPosInf)
        && !z(true, Mode::TowardZero)
        && !z(true, Mode::AwayFromZero)
        && !z(true, Mode::HalfEven)
        && !z(true, Mode::HalfUpAway);
    println!(
        "P2 on signed wrapping the verdict flips with the reading of `half_up`: {}",
        if p2 { "HOLDS" } else { "REFUTED" }
    );
    println!(
        "   half_up(+inf) max {:.2}%, half_up(away) max {:.2}%",
        max_rate[&(true, "half_up(+inf)")],
        max_rate[&(true, "half_up(away)")]
    );

    let p3 = MODES
        .iter()
        .filter(|m| **m != Mode::HalfEven)
        .all(|m| z(false, *m))
        && !z(false, Mode::HalfEven);
    println!(
        "P3 on unsigned wrapping only half_even is not free, either reading: {}",
        if p3 { "HOLDS" } else { "REFUTED" }
    );

    // R3: y2's published maxima on the six modes it swept.
    let y2: [(bool, Mode, f64); 12] = [
        (false, Mode::Floor, 0.00),
        (false, Mode::Ceil, 0.00),
        (false, Mode::TowardZero, 0.00),
        (false, Mode::AwayFromZero, 0.00),
        (false, Mode::HalfUpPosInf, 0.00),
        (false, Mode::HalfEven, 12.50),
        (true, Mode::Floor, 0.00),
        (true, Mode::Ceil, 0.00),
        (true, Mode::TowardZero, 33.40),
        (true, Mode::AwayFromZero, 33.40),
        (true, Mode::HalfUpPosInf, 0.00),
        (true, Mode::HalfEven, 12.50),
    ];
    let mut r3 = true;
    for (s, m, want) in y2 {
        let got = max_rate[&(s, name(m))];
        if (got - want).abs() > 0.005 {
            r3 = false;
            println!(
                "R3 MISMATCH {} {}: y2 says {want:.2}%, I measure {got:.2}%",
                if s { "signed" } else { "unsigned" },
                name(m)
            );
        }
    }
    println!(
        "R3 reproduces y2_out.txt on the six modes it swept, both signednesses: {}",
        if r3 { "pass" } else { "FAIL" }
    );

    // ------------------------------------------------------------ part C
    println!();
    println!("== part C: chain retraction, non-negative domain, two multiplies ==");
    println!(
        "{:<16} {:>3} {:>3} {:>14} {:>9}   {}",
        "mode", "W", "F", "differ", "pct", "verdict"
    );
    let mut p4_holds_at_zero = true;
    let mut p4_fails_above = true;
    let mut floor_col: Vec<u64> = Vec::new();
    let mut away_col: Vec<u64> = Vec::new();
    for m in MODES {
        for w in [4u32, 6, 8] {
            for f in 0..=w {
                let (differ, total) = retraction_differ(w, f, m);
                if f == 0 && differ != 0 {
                    p4_holds_at_zero = false;
                }
                if f >= 1 && differ == 0 {
                    p4_fails_above = false;
                }
                if m == Mode::Floor {
                    floor_col.push(differ);
                }
                if m == Mode::HalfUpAway {
                    away_col.push(differ);
                }
                println!(
                    "{:<16} {:>3} {:>3} {:>14} {:>8.2}%   {}",
                    name(m),
                    w,
                    f,
                    differ,
                    100.0 * differ as f64 / total as f64,
                    if differ == 0 { "RETRACTS" } else { "does not retract" }
                );
            }
        }
    }
    println!(
        "P4 retraction holds at F = 0 for all seven and fails at every F >= 1 for all seven: {}",
        if p4_holds_at_zero && p4_fails_above {
            "HOLDS"
        } else {
            "REFUTED"
        }
    );

    // R1 / R2: c_retraction's two published columns, in its own order.
    let ctrunc: Vec<u64> = vec![
        0, 800, 1128, 910, 543, 0, 61952, 116352, 138880, 128723, 94884, 54201, 0, 4136960,
        8153088, 10872832, 11988992, 11577590, 9872633, 7068031, 3969848,
    ];
    let cnearest: Vec<u64> = vec![
        0, 864, 1248, 880, 550, 0, 62976, 118272, 135200, 107789, 63242, 32909, 0, 4153344,
        8183808, 10813952, 11514240, 10174670, 7402534, 4154563, 2099069,
    ];
    let r1 = floor_col.len() >= 21 && floor_col[..21] == ctrunc[..21];
    let r2 = away_col.len() >= 21 && away_col[..21] == cnearest[..21];
    println!(
        "R1 my floor column reproduces c_retraction's `truncate` column: {}",
        if r1 { "pass" } else { "FAIL" }
    );
    if !r1 {
        println!("   mine  {:?}", &floor_col[..floor_col.len().min(21)]);
        println!("   c_ret {:?}", &ctrunc[..21]);
    }
    println!(
        "R2 my half_up(away) column reproduces its `nearest` column: {}",
        if r2 { "pass" } else { "FAIL" }
    );
    if !r2 {
        println!("   mine  {:?}", &away_col[..away_col.len().min(21)]);
        println!("   c_ret {:?}", &cnearest[..21]);
    }

    println!();
    let all = p1 && p2 && p3 && p4_holds_at_zero && p4_fails_above && r1 && r2 && r3 && r4 && r5;
    println!(
        "k2 verdict: {}",
        if all {
            "every prediction and control holds"
        } else {
            "SOMETHING FAILED, read above"
        }
    );
}
