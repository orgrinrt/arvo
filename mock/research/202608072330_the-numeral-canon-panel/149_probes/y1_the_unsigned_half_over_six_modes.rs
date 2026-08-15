// y1: is the unsigned half of the fusion arm free for EVERY rounding position?
//
// `146` section 5.5, first block, and `145` A1, first predicate block, both claim:
//
//   holds for: ... signedness = unsigned; overflow in {wrap, saturating};
//              rounding in {floor, ceiling, toward zero, away from zero,
//                           nearest-half-up, nearest-half-even}; ...
//
// with the argument kind given as "closure for the unsigned half (a one-sided clamp
// of a monotone operation is a congruence, so reducing early and late land in the
// same place)".
//
// THE OBJECTION. That closure argument is about the REDUCTION. The arm being fused
// relocates two things, not one. `142`'s own derivation writes the pair as
// `R(rnd(x) + c)` against `R(rnd(x + c))`, and its q2 part B implements the fused
// arm as `R(rnd(ab + c*2^F))`. So the rounding moves as well, and the congruence
// argument says nothing about a rounding relocation.
//
// `142` q2 part B swept that pair over six modes at `W = 6` SIGNED WRAPPING only:
// its loop calls `wrap(..., w, true)`, and the signed flag is fixed true. Nobody has
// swept the unsigned half over six modes. The unsigned rows in `139`, `141` and my
// own p1 were all at truncate toward zero.
//
// And there is a mode that should break it. Nearest-half-even is not translation
// equivariant, and its failure has nothing to do with sign: `rne(1/2) = 0` while
// `rne(1/2 + 1) = 2`, and both arguments are non-negative. So on unsigned values,
// where every other mode in the set collapses onto floor or ceiling and both of
// those are equivariant, half-even should still differ.
//
// PREDICTIONS, before running:
//   Y1. Under unsigned, nearest-half-even shows a NONZERO fusion difference at every
//       F >= 1, under both wrap and saturating. If so, both predicate blocks claim a
//       region in which the claim is false, and the widening is by one mode.
//   Y2. The other five modes are 0.00% at every F under unsigned, because on
//       non-negative values toward-zero collapses onto floor, away-from-zero onto
//       ceiling, and floor, ceiling and half-up are all equivariant.
//   Y3. Every mode is 0.00% at F = 0 under unsigned, because no rounding happens.
//   Y4. The half-even rate under unsigned is close to but not identical to the
//       signed wrapping rate `142` reports (12.50 / 12.50 / 9.38 / 6.25 / 3.91),
//       because the tie density differs between the two domains.
//
// CONTROLS:
//   C1 (reach): count triples whose rounding is inexact and whose quotient is
//      exactly on a tie. A cell reporting 0.00% with zero ties present is vacuous
//      for half-even and is printed as VACUOUS rather than as agreement.
//   C2 (mutation): a fused arm that rounds with a different mode than the stepwise
//      arm must be caught in every cell. If it is not, the comparator cannot see a
//      rounding difference and Y2's zeros are worthless.
//   C3 (cross-check): reproduce `142`'s signed wrapping row for half-even on my own
//      implementation of the six modes. If my modes disagree with `142`'s there, the
//      unsigned result is about my rounding rather than about the claim.
//
// Run: rustc -O -o /tmp/y1 y1_the_unsigned_half_over_six_modes.rs && /tmp/y1

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

/// Round p / 2^F to an integer under the given mode. Exact integer arithmetic,
/// written from the definitions rather than transcribed from any other probe.
fn rnd(p: i128, f: u32, m: Mode) -> i128 {
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d); // floor
    let r = p.rem_euclid(d); // 0 <= r < d
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
            if p >= 0 {
                q
            } else if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::AwayFromZero => {
            if p >= 0 {
                if r == 0 {
                    q
                } else {
                    q + 1
                }
            } else {
                q
            }
        }
        Mode::NearestHalfUp => {
            // floor(x + 1/2)
            if 2 * r >= d {
                q + 1
            } else {
                q
            }
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
        if signed && r >= (1i128 << (w - 1)) {
            r - m
        } else {
            r
        }
    }
}

struct Cell {
    rate: f64,
    ties: u64,
    inexact: u64,
    mutant_caught: u64,
}

fn sweep(signed: bool, sat: bool, w: u32, f: u32, m: Mode) -> Cell {
    let (lo, hi) = if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    };
    let d = 1i128 << f;
    let mut n = 0u64;
    let mut diff = 0u64;
    let mut ties = 0u64;
    let mut inexact = 0u64;
    let mut caught = 0u64;
    // the mutant fused arm rounds with floor regardless of m
    for a in lo..=hi {
        for b in lo..=hi {
            let p = a * b;
            let pr = if f == 0 { 0 } else { p.rem_euclid(d) };
            let is_inexact = f > 0 && pr != 0;
            let is_tie = f > 0 && 2 * pr == d;
            for c in lo..=hi {
                n += 1;
                if is_inexact {
                    inexact += 1;
                }
                if is_tie {
                    ties += 1;
                }
                let t = reduce(rnd(p, f, m), signed, sat, w);
                let stepwise = reduce(t + c, signed, sat, w);
                let fused = reduce(rnd(p + (c << f), f, m), signed, sat, w);
                if stepwise != fused {
                    diff += 1;
                }
                let mutant = reduce(rnd(p + (c << f), f, Mode::Floor), signed, sat, w);
                if stepwise != mutant {
                    caught += 1;
                }
            }
        }
    }
    Cell {
        rate: 100.0 * diff as f64 / n as f64,
        ties,
        inexact,
        mutant_caught: caught,
    }
}

fn main() {
    let w = 6u32;
    println!("y1: the unsigned half of the fusion arm, over six rounding modes");
    println!("W = {w}, exhaustive over all triples per cell");
    println!("stepwise = R(R(rnd(ab)) + c), fused = R(rnd(ab + c*2^F))\n");

    for (signed, sat, label) in [
        (false, false, "unsigned, wrapping"),
        (false, true, "unsigned, saturating"),
    ] {
        println!("=== {label} ===");
        println!(
            "{:<20} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
            "mode", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
        );
        for m in MODES {
            let mut row = format!("{:<20}", name(m));
            let mut notes: Vec<String> = Vec::new();
            for f in 0..=5u32 {
                let cell = sweep(signed, sat, w, f, m);
                row.push_str(&format!(" {:>7.2}%", cell.rate));
                if f > 0 && cell.rate == 0.0 && m == Mode::NearestHalfEven && cell.ties == 0 {
                    notes.push(format!("F={f} C1 VACUOUS: no ties present"));
                }
                if f > 0 && cell.inexact == 0 {
                    notes.push(format!("F={f} C1 VACUOUS: rounding never inexact"));
                }
                if f > 0 && m != Mode::Floor && cell.mutant_caught == 0 {
                    notes.push(format!("F={f} C2 TOOTHLESS: mutant never caught"));
                }
                if f == 0 && cell.rate != 0.0 {
                    notes.push(format!("F=0 nonzero, which should be impossible"));
                }
            }
            println!("{row}");
            for n in notes {
                println!("      ! {n}");
            }
        }
        println!();
    }

    println!("=== C3 cross-check: signed wrapping, my own mode implementations ===");
    println!("(142 q2 part B reports half-even at 0.00 / 12.50 / 12.50 / 9.38 / 6.25 / 3.91)");
    println!(
        "{:<20} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "mode", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
    );
    for m in MODES {
        let mut row = format!("{:<20}", name(m));
        for f in 0..=5u32 {
            let cell = sweep(true, false, w, f, m);
            row.push_str(&format!(" {:>7.2}%", cell.rate));
        }
        println!("{row}");
    }

    println!("\n=== tie reach under unsigned, which is what half-even needs ===");
    for f in 1..=5u32 {
        let cell = sweep(false, false, w, f, Mode::NearestHalfEven);
        println!(
            "F={f}: ties present at {:>7} triples, rounding inexact at {:>7}, mutant caught at {:>7}",
            cell.ties, cell.inexact, cell.mutant_caught
        );
    }
}
