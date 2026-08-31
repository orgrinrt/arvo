// p6: does the equivariance prediction hold at widths other than six?
//
// `p2` establishes that fusing a multiply-add is answer-preserving exactly where
// the rounding rule is translation equivariant on the domain the cell reaches,
// in 84 of 84 cells with no mismatch, at `W = 6`. That is the width both law
// rows are written at, so the finding carries `total_width: 6` and nothing
// wider. This widens it, or fails to.
//
// The prediction is width-free as an argument: the fused and stepwise forms
// agree exactly when `rnd(P + C*2^F) == rnd(P) + C` for every reachable product
// P and every C the container holds, which is the definition of equivariance
// under translation by C grid steps, and nothing in it mentions W. So the
// expected result is zero mismatches at every width, and a mismatch anywhere
// would refute the argument rather than merely bound it.
//
// The case that must fail is carried as C1: a mode reported equivariant must be
// free and one reported not must not be, checked as a positive and a negative at
// each width, so a run that cannot separate them is visible.
//
// Run: rustc --edition 2024 -O p6_the_prediction_across_widths.rs -o /tmp/p6 && /tmp/p6

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
    Floor,
    Ceiling,
    TowardZero,
    AwayFromZero,
    HalfUpPinf,
    HalfUpAway,
    HalfEven,
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

fn rnd(p: i128, f: u32, m: Mode) -> i128 {
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Floor => q,
        Ceiling => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        TowardZero => {
            if p >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        AwayFromZero => {
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
        HalfUpPinf => {
            if 2 * r >= d {
                q + 1
            } else {
                q
            }
        }
        HalfUpAway => {
            if p >= 0 {
                if 2 * r >= d {
                    q + 1
                } else {
                    q
                }
            } else if 2 * r > d {
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

fn bounds(w: u32, signed: bool) -> (i128, i128) {
    if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    }
}

fn wrap(v: i128, signed: bool, w: u32) -> i128 {
    let (_, hi) = bounds(w, signed);
    let m = 1i128 << w;
    let mut r = v.rem_euclid(m);
    if signed && r > hi {
        r -= m;
    }
    r
}

fn free(w: u32, f: u32, m: Mode, signed: bool) -> bool {
    let (lo, hi) = bounds(w, signed);
    let scale = 1i128 << f;
    for a in lo..=hi {
        for b in lo..=hi {
            let p = a * b;
            let t = wrap(rnd(p, f, m), signed, w);
            for c in lo..=hi {
                if wrap(t + c, signed, w) != wrap(rnd(p + c * scale, f, m), signed, w) {
                    return false;
                }
            }
        }
    }
    true
}

fn equivariant(w: u32, f: u32, m: Mode, signed: bool) -> bool {
    if f == 0 {
        return true;
    }
    let (lo, hi) = bounds(w, signed);
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
    println!("p6: the equivariance prediction at widths other than six");
    println!("    overflow policy wrap; exhaustive over every triple at each width");
    println!();
    println!(
        "{:>3} {:>6} {:>8} {:>10} {:>12}",
        "W", "cells", "agree", "mismatch", "verdict"
    );

    let mut total = 0;
    let mut total_mm = 0;
    let mut per_width = Vec::new();
    for w in [3u32, 4, 5, 6, 7] {
        let mut cells = 0;
        let mut mm = 0;
        for &signed in [false, true].iter() {
            for m in MODES {
                for f in 0..=(w - 1) {
                    let eq = equivariant(w, f, m, signed);
                    let fr = free(w, f, m, signed);
                    cells += 1;
                    if eq != fr {
                        mm += 1;
                        println!(
                            "    MISMATCH W={w} signed={signed} F={f} mode={} eq={eq} free={fr}",
                            name(m)
                        );
                    }
                }
            }
        }
        println!(
            "{:>3} {:>6} {:>8} {:>10} {:>12}",
            w,
            cells,
            cells - mm,
            mm,
            if mm == 0 { "agrees" } else { "REFUTED" }
        );
        total += cells;
        total_mm += mm;
        per_width.push((w, cells, mm));
    }
    println!();
    println!("  {total} cells across five widths, {total_mm} mismatch(es)");
    println!();

    // Which modes are free at each width, so the two law rows' regions can be
    // read off at widths they were not written at.
    println!("## the free set at each width, under wrap");
    println!();
    println!(
        "{:>3} {:<9} {}",
        "W", "signed", "free at every F below the width"
    );
    for w in [3u32, 4, 5, 6, 7] {
        for &signed in [false, true].iter() {
            let mut fs = Vec::new();
            for m in MODES {
                if (0..w).all(|f| free(w, f, m, signed)) {
                    fs.push(name(m));
                }
            }
            println!("{:>3} {:<9} {}", w, signed, fs.join(", "));
        }
    }
    println!();

    // ---- controls ----------------------------------------------------------
    println!("## controls");
    println!();
    let mut ok = true;

    // C1: at every width the run must produce both verdicts, or it is not
    // separating anything and the zero above means nothing.
    let mut c1 = true;
    for w in [3u32, 4, 5, 6, 7] {
        let any_free = MODES.iter().any(|&m| free(w, 1, m, true));
        let any_not = MODES.iter().any(|&m| !free(w, 1, m, true));
        println!("     W={w} signed F=1: some mode free {any_free}, some mode not free {any_not}");
        if !(any_free && any_not) {
            c1 = false;
        }
    }
    if c1 {
        println!("  C1 EXPECTED-PASS ok: both verdicts occur at every width");
    } else {
        println!("  C1 BROKEN: a width produced only one verdict");
        ok = false;
    }

    // C2 mutation: break the prediction on purpose by declaring half_even
    // equivariant, and check the comparison notices.
    let mut caught = 0;
    for w in [3u32, 4, 5, 6, 7] {
        for f in 1..w {
            if free(w, f, HalfEven, false) != true {
                caught += 1;
            }
        }
    }
    if caught > 0 {
        println!("  C2 EXPECTED-FAIL ok: asserting half_even free unsigned is refuted in {caught} cell(s)");
    } else {
        println!("  C2 BROKEN");
        ok = false;
    }

    println!();
    println!(
        "controls: {}",
        if ok && total_mm == 0 {
            "clean, and the prediction is unrefuted at every width run"
        } else if ok {
            "clean, and the prediction is REFUTED somewhere above"
        } else {
            "BROKEN"
        }
    );
}
