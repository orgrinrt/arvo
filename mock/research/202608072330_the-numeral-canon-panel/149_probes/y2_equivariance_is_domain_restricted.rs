// y2: two things, both about the rounding axis.
//
// PART ONE. y1 found that under unsigned, five of six rounding modes give a zero
// fusion difference and nearest-half-even does not. But `142` F142-2 partitions the
// six modes three against three by translation equivariance, and toward-zero and
// away-from-zero are on the NON-equivariant side of that partition while showing
// 0.00% under unsigned.
//
// So either the partition does not predict the unsigned half, or the property being
// read is not the one F142-2 states. The second is the case, and the difference is a
// quantifier: `142` tests equivariance over the whole rational line, and what an arm
// actually needs is equivariance **on the domain the cell reaches**.
//
// Under unsigned, every quantity entering the rounding is non-negative, and on
// non-negative arguments toward-zero is floor and away-from-zero is ceiling, both of
// which are equivariant. So the effective partition under unsigned is five against
// one, not three against three, and nearest-half-even is the only survivor because
// its tie rule reads the parity of the result rather than the sign of the argument.
//
// If that holds, the canon does not need two clauses with two argument kinds for the
// two signednesses. It needs one: **fusion is answer-preserving where the rounding
// position is translation equivariant on the domain the cell reaches.**
//
// PART TWO. `142` section 3 contests my replacement B, that spelling the fractional
// shift as an arithmetic shift right rather than an integer division is a spelling
// change with no semantic content on non-negative values. Its F142-4 measures the
// swap changing between 12.50% and 44.53% of multiply answers on signed shapes. I do
// not accept a refutation on someone else's numbers, so part two rebuilds it.
//
// PREDICTIONS, before running:
//   Z1. The domain-restricted equivariance test predicts the fusion table at every
//       one of the 12 (mode, signedness) combinations under wrapping. The unrestricted
//       test predicts only the 6 signed ones and mispredicts toward-zero and
//       away-from-zero under unsigned.
//   Z2. Under unsigned the restricted test finds five equivariant modes and one not;
//       under signed it finds three and three, agreeing with `142`.
//   Z3. Part two reproduces `142` F142-4's signed rows to two decimal places:
//       wrapping 0.00 / 12.50 / 25.00 / 34.38 / 40.62 / 44.53 and saturating
//       0.00 / 2.93 / 9.57 / 20.51 / 34.33 / 44.53, and zero on both unsigned rows.
//   Z4. The mispredictions of the unrestricted test are exactly two modes, not more.
//
// CONTROLS:
//   C1. The restricted and unrestricted tests must DISAGREE somewhere, or part one is
//       measuring one thing twice and the refinement is empty.
//   C2. Part two must be zero on both unsigned rows, since floor and toward-zero are
//       the same function on non-negative arguments, and a nonzero there would mean
//       my two spellings differ for a reason unrelated to sign.
//   C3. Part two must be nonzero somewhere at F = 0, or nonzero nowhere at F = 0. The
//       shift does nothing at F = 0, so a nonzero there is an instrument defect.
//
// Run: rustc -O -o /tmp/y2 y2_equivariance_is_domain_restricted.rs && /tmp/y2

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

fn rnd(p: i128, f: u32, m: Mode) -> i128 {
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Mode::Floor => q,
        Mode::Ceiling => {
            if r == 0 { q } else { q + 1 }
        }
        Mode::TowardZero => {
            if p >= 0 || r == 0 { q } else { q + 1 }
        }
        Mode::AwayFromZero => {
            if p >= 0 {
                if r == 0 { q } else { q + 1 }
            } else {
                q
            }
        }
        Mode::NearestHalfUp => {
            if 2 * r >= d { q + 1 } else { q }
        }
        Mode::NearestHalfEven => {
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
        let r = v.rem_euclid(m);
        if signed && r >= (1i128 << (w - 1)) { r - m } else { r }
    }
}

/// Is `rnd(., f, m)` translation equivariant over the given numerator range and the
/// given set of integer shifts? `restrict` decides which region is swept.
fn equivariant_over(f: u32, m: Mode, num_lo: i128, num_hi: i128, shift_lo: i128, shift_hi: i128) -> bool {
    let d = 1i128 << f;
    for p in num_lo..=num_hi {
        for c in shift_lo..=shift_hi {
            if rnd(p + c * d, f, m) != rnd(p, f, m) + c {
                return false;
            }
        }
    }
    true
}

fn fusion_rate(signed: bool, sat: bool, w: u32, f: u32, m: Mode) -> f64 {
    let (lo, hi) = if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    };
    let mut n = 0u64;
    let mut diff = 0u64;
    for a in lo..=hi {
        for b in lo..=hi {
            let p = a * b;
            for c in lo..=hi {
                n += 1;
                let t = reduce(rnd(p, f, m), signed, sat, w);
                let stepwise = reduce(t + c, signed, sat, w);
                let fused = reduce(rnd(p + (c << f), f, m), signed, sat, w);
                if stepwise != fused {
                    diff += 1;
                }
            }
        }
    }
    100.0 * diff as f64 / n as f64
}

fn main() {
    let w = 6u32;
    println!("y2 part one: is the licensing property domain-restricted?\n");

    // The numerator range a W=6 multiply actually reaches, and the shift range c reaches.
    // signed:   products in [-(2^(w-1))^2 .. 2^(2w-2)], shifts in [-2^(w-1), 2^(w-1)-1]
    // unsigned: products in [0 .. (2^w-1)^2],           shifts in [0, 2^w-1]
    // The sweeps below use a smaller but sign-correct window, which is enough to decide
    // equivariance because a single counterexample settles it.
    println!(
        "{:<20} {:>14} {:>16} {:>12}",
        "mode", "unrestricted", "restricted >= 0", "agree?"
    );
    let mut disagreements = 0;
    for m in MODES {
        // unrestricted: numerators and shifts of both signs
        let un = (1..=5u32).all(|f| equivariant_over(f, m, -256, 256, -8, 8));
        // restricted to the non-negative domain an unsigned cell reaches
        let re = (1..=5u32).all(|f| equivariant_over(f, m, 0, 512, 0, 16));
        if un != re {
            disagreements += 1;
        }
        println!(
            "{:<20} {:>14} {:>16} {:>12}",
            name(m),
            un,
            re,
            if un == re { "same" } else { "DIFFER" }
        );
    }
    println!(
        "\nC1: the two tests disagree on {disagreements} modes (must be > 0, or the refinement is empty)"
    );

    println!("\n=== Z1: does the RESTRICTED test predict the fusion table? ===");
    println!("(wrapping, W = {w}, exhaustive; a mode is predicted zero iff restricted-equivariant)");
    println!(
        "{:<20} {:>10} {:>12} {:>14} {:>10}",
        "mode", "signedness", "restricted", "measured max", "verdict"
    );
    let mut mispred_restricted = 0;
    let mut mispred_unrestricted = 0;
    for signed in [false, true] {
        let (nlo, nhi, slo, shi) = if signed {
            (-256i128, 256i128, -8i128, 8i128)
        } else {
            (0i128, 512i128, 0i128, 16i128)
        };
        for m in MODES {
            let re = (1..=5u32).all(|f| equivariant_over(f, m, nlo, nhi, slo, shi));
            let un = (1..=5u32).all(|f| equivariant_over(f, m, -256, 256, -8, 8));
            let mut maxrate = 0.0f64;
            for f in 0..=5u32 {
                let r = fusion_rate(signed, false, w, f, m);
                if r > maxrate {
                    maxrate = r;
                }
            }
            let zero = maxrate == 0.0;
            if re != zero {
                mispred_restricted += 1;
            }
            if un != zero {
                mispred_unrestricted += 1;
            }
            println!(
                "{:<20} {:>10} {:>12} {:>13.2}% {:>10}",
                name(m),
                if signed { "signed" } else { "unsigned" },
                re,
                maxrate,
                if re == zero { "ok" } else { "MISPREDICT" }
            );
        }
    }
    println!("\n  restricted test mispredictions:   {mispred_restricted} of 12 (Z1 wants 0)");
    println!("  unrestricted test mispredictions: {mispred_unrestricted} of 12 (Z4 wants 2)");

    println!("\n=== part two: reproducing 142 F142-4, the cost of my replacement B ===");
    println!("(rate at which swapping toward-zero for floor changes a MULTIPLY answer)");
    println!(
        "{:<22} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "cell", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
    );
    for signed in [false, true] {
        for sat in [false, true] {
            let (lo, hi) = if signed {
                (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
            } else {
                (0, (1i128 << w) - 1)
            };
            let label = format!(
                "{}, {}",
                if signed { "signed" } else { "unsigned" },
                if sat { "saturating" } else { "wrapping" }
            );
            let mut row = format!("{label:<22}");
            for f in 0..=5u32 {
                let mut n = 0u64;
                let mut diff = 0u64;
                for a in lo..=hi {
                    for b in lo..=hi {
                        n += 1;
                        let p = a * b;
                        let tz = reduce(rnd(p, f, Mode::TowardZero), signed, sat, w);
                        let fl = reduce(rnd(p, f, Mode::Floor), signed, sat, w);
                        if tz != fl {
                            diff += 1;
                        }
                    }
                }
                row.push_str(&format!(" {:>7.2}%", 100.0 * diff as f64 / n as f64));
            }
            println!("{row}");
        }
    }
    println!("\n142 F142-4 reports signed wrapping   0.00 12.50 25.00 34.38 40.62 44.53");
    println!("142 F142-4 reports signed saturating 0.00  2.93  9.57 20.51 34.33 44.53");
    println!("and zero on both unsigned rows.");
}
