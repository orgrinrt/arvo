// q2: `141` found that toward-zero truncation is not translation equivariant and
// that floor is, and used that to explain `139`'s signed wrapping row. I derived
// the same mechanism before reading `141`'s probe code, having read its prose
// claim, so this is a re-derivation and a re-measurement rather than a blind
// instance, and I say so rather than claiming a rung I did not earn.
//
// What I want past confirming it is the SET. `141` tested two modes. The
// rounding axis has more positions than two, and if translation equivariance is
// the property that licenses relocating a rounding across an integer addition,
// then it partitions the axis and the partition is the arm's predicate.
//
// My derivation, from scratch:
//
//   Let x = a*b / 2^F as an exact rational, and c an integer.
//   Under wrapping, reduction is a ring homomorphism mod 2^W, so
//     stepwise = R(rnd(x) + c) and fused = R(rnd(x + c)).
//   These agree for all inputs exactly when rnd(x + c) = rnd(x) + c,
//   which is translation equivariance on integer shifts.
//
// PREDICTIONS, recorded before the first run. I expect the partition NOT to be
// "symmetric modes fail, asymmetric modes pass", which is the obvious guess:
//   E1 floor is equivariant.
//   E2 ceiling is equivariant.
//   E3 toward-zero is NOT.
//   E4 away-from-zero is NOT.
//   E5 nearest-half-up IS equivariant, because it is floor(x + 1/2) and floor
//      is, which puts a nearest mode on the same side as floor.
//   E6 nearest-half-EVEN is NOT, because its tie break reads the parity of the
//      result and adding c changes that parity. rne(1/2) = 0 but
//      rne(1/2 + 1) = 2, not 1. This is the interesting one: the IEEE default
//      mode does not have the property.
//   E7 under wrapping, the fusion difference is zero for exactly the equivariant
//      modes and nonzero for exactly the others.
//
// CONTROLS:
//   C1 at least one mode must be non-equivariant and at least one must be
//      equivariant. A checker reporting one answer for every mode is measuring
//      nothing, and this is the case that must fail.
//   C2 the sweep must contain non-integral x, or equivariance holds trivially.
//      Counted and reported per shape; a zero there voids the row.
//   C3 for the fusion half, the difference must be zero at F = 0 for every mode,
//      because no rounding occurs there at all. A mode showing a difference at
//      F = 0 means the instrument is measuring something other than rounding.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Floor,
    Ceiling,
    TowardZero,
    AwayFromZero,
    NearestHalfUp,
    NearestHalfEven,
}

const MODES: [Mode; 6] = [
    Mode::Floor,
    Mode::Ceiling,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::NearestHalfUp,
    Mode::NearestHalfEven,
];

fn name(m: Mode) -> &'static str {
    match m {
        Mode::Floor => "floor",
        Mode::Ceiling => "ceiling",
        Mode::TowardZero => "toward-zero",
        Mode::AwayFromZero => "away-from-zero",
        Mode::NearestHalfUp => "nearest-half-up",
        Mode::NearestHalfEven => "nearest-half-even",
    }
}

/// Divide `p` by `2^sh` under the named mode. Exact rational arithmetic on
/// integers: `q` is the floor quotient and `r` the non-negative remainder.
fn rnd(p: i128, sh: u32, m: Mode) -> i128 {
    if sh == 0 {
        return p;
    }
    let d = 1i128 << sh;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    let half = d >> 1;
    match m {
        Mode::Floor => q,
        Mode::Ceiling => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::TowardZero => {
            if p >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::AwayFromZero => {
            if r == 0 {
                q
            } else if p > 0 {
                q + 1
            } else {
                q
            }
        }
        Mode::NearestHalfUp => {
            if r >= half {
                q + 1
            } else {
                q
            }
        }
        Mode::NearestHalfEven => {
            if r > half {
                q + 1
            } else if r < half {
                q
            } else if q.rem_euclid(2) == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

fn wrap(v: i128, w: u32, signed: bool) -> i128 {
    let m = 1i128 << w;
    let r = v.rem_euclid(m);
    if signed && r >= (m >> 1) {
        r - m
    } else {
        r
    }
}

fn saturate(v: i128, w: u32, signed: bool) -> i128 {
    let m = 1i128 << w;
    let (lo, hi) = if signed {
        (-(m >> 1), (m >> 1) - 1)
    } else {
        (0, m - 1)
    };
    v.clamp(lo, hi)
}

fn main() {
    let mut failures = 0usize;

    // ---------------------------------------------------------------- part A
    println!("PART A: which rounding modes are translation equivariant?");
    println!("  rnd((p + c*2^F) / 2^F) against rnd(p / 2^F) + c, over exact integers");
    println!();
    let mut equivariant = Vec::new();
    for m in MODES {
        let (mut viol, mut n, mut inexact) = (0u64, 0u64, 0u64);
        let mut witness: Option<(i128, i128, u32)> = None;
        for f in 1..=5u32 {
            let d = 1i128 << f;
            for p in -256i128..=256 {
                if p.rem_euclid(d) != 0 {
                    inexact += 1;
                }
                for c in -32i128..=32 {
                    n += 1;
                    let lhs = rnd(p + c * d, f, m);
                    let rhs = rnd(p, f, m) + c;
                    if lhs != rhs {
                        viol += 1;
                        if witness.is_none() {
                            witness = Some((p, c, f));
                        }
                    }
                }
            }
        }
        let verdict = if viol == 0 { "EQUIVARIANT" } else { "not" };
        if viol == 0 {
            equivariant.push(m);
        }
        print!(
            "  {:<18} {:<12} violations {:>7} of {n}   inexact p sampled {inexact}",
            name(m),
            verdict,
            viol
        );
        match witness {
            Some((p, c, f)) => println!("   witness p={p} c={c} F={f}"),
            None => println!(),
        }
        if inexact == 0 {
            println!("    C2 FAILED: no inexact quotient in the sweep, the row is vacuous");
            failures += 1;
        }
    }
    println!();
    if equivariant.is_empty() || equivariant.len() == MODES.len() {
        println!("  C1 FAILED: the check returned one answer for every mode, so it is");
        println!("     not measuring the property it names.");
        failures += 1;
    } else {
        println!(
            "  C1 PASS: {} of {} modes are equivariant, so the partition is real.",
            equivariant.len(),
            MODES.len()
        );
    }

    // ---------------------------------------------------------------- part B
    println!();
    println!("PART B: does the partition predict the fusion difference under wrapping?");
    println!("  stepwise = R(R(rnd(ab)) + c) against fused = R(rnd(ab + c*2^F)), W=6 signed");
    println!();
    let w = 6u32;
    let lo = -(1i128 << (w - 1));
    let hi = (1i128 << (w - 1)) - 1;
    let dom: Vec<i128> = (lo..=hi).collect();
    for m in MODES {
        let mut row = String::new();
        let mut predicted_zero = equivariant.contains(&m);
        let mut all_zero_above_f0 = true;
        for f in 0..6u32 {
            let (mut n, mut diff) = (0u64, 0u64);
            for &a in &dom {
                for &b in &dom {
                    for &c in &dom {
                        n += 1;
                        let t = wrap(rnd(a * b, f, m), w, true);
                        let stepwise = wrap(t + c, w, true);
                        let fused = wrap(rnd(a * b + (c << f), f, m), w, true);
                        if stepwise != fused {
                            diff += 1;
                        }
                    }
                }
            }
            let pct = 100.0 * diff as f64 / n as f64;
            row.push_str(&format!(" F={f}:{pct:>6.2}%"));
            if f == 0 && diff != 0 {
                println!("  C3 FAILED for {}: nonzero at F=0", name(m));
                failures += 1;
            }
            if f > 0 && diff != 0 {
                all_zero_above_f0 = false;
            }
        }
        let agree = predicted_zero == all_zero_above_f0;
        println!(
            "  {:<18} {}   equivariant={:<5} all-zero={:<5} {}",
            name(m),
            row,
            predicted_zero,
            all_zero_above_f0,
            if agree { "E7 ok" } else { "E7 MISMATCH" }
        );
        if !agree {
            failures += 1;
        }
        predicted_zero = false;
        let _ = predicted_zero;
    }

    // ---------------------------------------------------------------- part C
    println!();
    println!("PART C: what does replacing toward-zero with floor cost in answers?");
    println!("  `141` replacement B calls this a spelling change with no semantic content");
    println!("  on non-negative values. The cell it is invoked for is signed.");
    println!();
    for signed in [false, true] {
        let lo = if signed { -(1i128 << (w - 1)) } else { 0 };
        let hi = if signed {
            (1i128 << (w - 1)) - 1
        } else {
            (1i128 << w) - 1
        };
        let dom: Vec<i128> = (lo..=hi).collect();
        for sat in [false, true] {
            let mut row = String::new();
            for f in 0..6u32 {
                let (mut n, mut diff) = (0u64, 0u64);
                for &a in &dom {
                    for &b in &dom {
                        n += 1;
                        let red = |v: i128| {
                            if sat {
                                saturate(v, w, signed)
                            } else {
                                wrap(v, w, signed)
                            }
                        };
                        let tz = red(rnd(a * b, f, Mode::TowardZero));
                        let fl = red(rnd(a * b, f, Mode::Floor));
                        if tz != fl {
                            diff += 1;
                        }
                    }
                }
                row.push_str(&format!(" F={f}:{:>6.2}%", 100.0 * diff as f64 / n as f64));
            }
            println!(
                "  signed={:<5} {}: multiply answers changed by the swap {}",
                signed,
                if sat { "Sat " } else { "Wrap" },
                row
            );
        }
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
