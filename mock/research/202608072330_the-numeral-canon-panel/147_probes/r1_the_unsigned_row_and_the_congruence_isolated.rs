// r1: two questions for the signature, one instrument.
//
// QUESTION A, the widening check. `146` section 5.5's first predicate says
// fusing a multiply-add is a free lowering at
//
//   signedness = unsigned; overflow in {wrap, saturating};
//   rounding in {floor, ceiling, toward zero, away from zero,
//                nearest-half-up, nearest-half-even}
//
// No instrument in this topic swept six rounding modes on an unsigned domain.
// `139_probes/p2` swept unsigned at toward-zero only, because its shift is
// `p / (1 << sh)` and nothing else. `142_probes/q2` swept six modes but its
// part B pins `wrap(..., w, true)`, so it is signed wrapping only. So the
// candidate's unsigned row is wider than anything measured, and the question is
// whether it is also false.
//
// QUESTION B, the contamination isolation. `146` section 1.1 scopes the shared
// one-sided-clamp congruence to "the unsigned half of the fusion result". That
// is too coarse. Under wrapping the zero comes from reduction being a ring
// homomorphism, which is `141` F3's absorption and has nothing to do with a
// clamp. Under saturating the zero can only come from the clamp being
// one-sided. So the contamination reaches half of the unsigned half, and this
// separates them by making the clamp two-sided on an unsigned domain, which is
// a legal range policy and not a trick: it is a clamp to [lo, hi] with lo > 0.
//
// PREDICTIONS, recorded before the first run:
//   A1 the three equivariant modes give 0.00% on unsigned at both overflow
//      positions, every F.
//   A2 toward-zero and away-from-zero also give 0.00% on unsigned, because on a
//      non-negative domain they coincide with floor and ceiling and inherit
//      equivariance there. So five of six.
//   A3 nearest-half-even gives NONZERO on unsigned, because its tie break reads
//      the parity of the result and adding an integer changes that parity, and
//      that failure needs no negative value. If A3 holds, `146`'s unsigned row
//      is false as written and not merely unmeasured.
//   B1 unsigned wrapping is 0.00% with a two-sided clamp as well, since
//      absorption does not care where the clamp is.
//   B2 unsigned saturating becomes NONZERO once the clamp is two-sided, which
//      is what makes the one-sidedness load-bearing there and nowhere else.
//
// CONTROLS:
//   C1 the signed rows must reproduce `142` F142-3, or this instrument is not
//      the same measurement and nothing it says about unsigned transfers.
//   C2 the two-sided unsigned clamp must actually fire: count how many results
//      hit the low bound. A clamp that never engages proves nothing.
//   C3 at F = 0 every mode must give 0.00% under wrapping, since no rounding
//      occurs there; a nonzero means the instrument is measuring something
//      other than rounding.

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

fn equivariant(m: Mode) -> bool {
    matches!(m, Mode::Floor | Mode::Ceiling | Mode::NearestHalfUp)
}

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

/// Range policies. `SatOneSided` is the ordinary unsigned saturating clamp:
/// the low bound is zero, which is the bottom of the domain, so nothing is ever
/// clipped below. `SatTwoSided` clamps to a positive floor, which is a range
/// policy a consumer can legitimately declare and which makes the clamp
/// two-sided on an unsigned domain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Range {
    Wrap,
    SatOneSided,
    SatTwoSided,
}

fn reduce(v: i128, w: u32, signed: bool, r: Range, lo2: i128) -> (i128, bool) {
    let m = 1i128 << w;
    let (lo, hi) = if signed {
        (-(m >> 1), (m >> 1) - 1)
    } else {
        (0, m - 1)
    };
    match r {
        Range::Wrap => {
            let x = v.rem_euclid(m);
            let x = if signed && x >= (m >> 1) { x - m } else { x };
            (x, false)
        }
        Range::SatOneSided => (v.clamp(lo, hi), false),
        Range::SatTwoSided => {
            let l = if signed { lo } else { lo2 };
            (v.clamp(l, hi), v < l)
        }
    }
}

fn domain(w: u32, signed: bool) -> Vec<i128> {
    let m = 1i128 << w;
    if signed {
        (-(m >> 1)..=(m >> 1) - 1).collect()
    } else {
        (0..m).collect()
    }
}

/// Returns (triples, differences, low-clamp engagements).
fn sweep(w: u32, f: u32, signed: bool, r: Range, m: Mode, lo2: i128) -> (u64, u64, u64) {
    let d = domain(w, signed);
    let (mut n, mut diff, mut engaged) = (0u64, 0u64, 0u64);
    for &a in &d {
        for &b in &d {
            for &c in &d {
                n += 1;
                let (t, e1) = reduce(rnd(a * b, f, m), w, signed, r, lo2);
                let (stepwise, e2) = reduce(t + c, w, signed, r, lo2);
                let (fused, e3) = reduce(rnd(a * b + (c << f), f, m), w, signed, r, lo2);
                if e1 || e2 || e3 {
                    engaged += 1;
                }
                if stepwise != fused {
                    diff += 1;
                }
            }
        }
    }
    (n, diff, engaged)
}

fn main() {
    let mut failures = 0usize;
    let w = 6u32;

    println!("QUESTION A: the unsigned row over all six rounding modes, W=6 exhaustive");
    println!("  146 section 5.5's first predicate claims this cell is free for every mode.");
    println!();
    let mut a3_witnessed = false;
    for r in [Range::Wrap, Range::SatOneSided] {
        for m in MODES {
            let mut row = String::new();
            let mut any_nonzero = false;
            for f in 0..6u32 {
                let (n, diff, _) = sweep(w, f, false, r, m, 0);
                let pct = 100.0 * diff as f64 / n as f64;
                row.push_str(&format!(" F={f}:{pct:>6.2}%"));
                if diff != 0 {
                    any_nonzero = true;
                    if f == 0 && r == Range::Wrap {
                        println!("  C3 FAILED for {}: nonzero at F=0 under wrapping", name(m));
                        failures += 1;
                    }
                }
            }
            let tag = if equivariant(m) { "equivariant" } else { "not equiv " };
            println!(
                "  unsigned {:<4} {:<18} {tag}: {row}",
                if r == Range::Wrap { "Wrap" } else { "Sat" },
                name(m)
            );
            if any_nonzero && m == Mode::NearestHalfEven {
                a3_witnessed = true;
            }
        }
    }
    println!();
    if a3_witnessed {
        println!("  A3 CONFIRMED. The unsigned row is FALSE as written in 146 section 5.5:");
        println!("  at least one rounding position it names changes the answer on unsigned.");
    } else {
        println!("  A3 REFUTED. Every mode is free on unsigned, so 146's row is unmeasured");
        println!("  rather than false, and the repair is a citation rather than a narrowing.");
    }

    println!();
    println!("QUESTION B: which zero rests on the one-sided clamp, and which on absorption");
    println!("  unsigned, floor (equivariant, so rounding contributes nothing here),");
    println!("  against a two-sided clamp with the low bound raised to 8 of 0..63.");
    println!();
    for r in [Range::Wrap, Range::SatOneSided, Range::SatTwoSided] {
        let mut row = String::new();
        let mut eng_total = 0u64;
        for f in 0..6u32 {
            let (n, diff, eng) = sweep(w, f, false, r, Mode::Floor, 8);
            eng_total += eng;
            row.push_str(&format!(" F={f}:{:>6.2}%", 100.0 * diff as f64 / n as f64));
        }
        let label = match r {
            Range::Wrap => "wrap             ",
            Range::SatOneSided => "saturate one-side",
            Range::SatTwoSided => "saturate two-side",
        };
        println!("  unsigned {label}: {row}   low-clamp engaged at {eng_total} comparisons");
        if r == Range::SatTwoSided {
            if eng_total == 0 {
                println!("    C2 FAILED: the two-sided clamp never engaged, so the row proves nothing");
                failures += 1;
            }
        }
    }

    println!();
    println!("CONTROL C1: the signed wrapping rows, which must reproduce 142 F142-3");
    println!();
    for m in MODES {
        let mut row = String::new();
        for f in 0..6u32 {
            let (n, diff, _) = sweep(w, f, true, Range::Wrap, m, 0);
            row.push_str(&format!(" F={f}:{:>6.2}%", 100.0 * diff as f64 / n as f64));
        }
        println!("  signed Wrap {:<18}: {row}", name(m));
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
