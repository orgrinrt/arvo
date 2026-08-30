// p3: locate the last unit of disagreement between my model and 139's.
//
// After p1 and p2 the two models agree on every cell except one, and the
// disagreement is now very small and very specific:
//
//   139 p3 minimum slack, signed wrapping : 0  1  1  1  1  1
//   p2  minimum slack, signed wrapping    : 0  0  0  0  0  0
//   139 p3 minimum slack, signed saturating: 32 32 32 32 32 1
//   p2  minimum slack, signed saturating   : 32 32 32 32 32 1   <- identical
//
// One unit, in one cell family, at F >= 1 only. p2's theorem says the reduction
// relocation cannot cost anything under wrapping, so the unit must come from
// somewhere else, and one unit at F >= 1 on signed values is the signature of a
// ROUNDING relocation rather than a reduction relocation.
//
// HYPOTHESIS. 139's fused arm rounds ONCE over the whole multiply-add, that is
// it computes shift(a*b + c*2^F), while its stepwise arm rounds the product and
// then adds, shift(a*b) + c. Under floor those are equal for every input, because
// floor(x/2^F + c) = floor(x/2^F) + c for integer c. Under truncation TOWARD ZERO
// they differ by at most one unit, and only when the quantity being truncated is
// negative, which is exactly signed-only, F >= 1 only, one unit.
//
// If that reproduces, then what 139 calls fusion is two changes wearing one name:
//
//   (i)  moving the reduction, which p2 proves is free under wrapping; and
//   (ii) moving the rounding, which costs one unit under toward-zero truncation
//        and nothing at all under floor.
//
// And (ii) is a move along the ROUNDING axis, which every file in this panel
// treats as an observable policy axis. So it is a policy change under the design's
// own membership test, and the correct response is to declare the rounding rather
// than to declare a slack.
//
// PREDICTIONS, before running:
//   R5. The one-rounding arm under TOWARD ZERO reproduces 139's signed wrapping
//       rate row 0.00 / 1.64 / 5.54 / 12.34 / 22.22 / 33.40 to two decimals, and
//       its minimum slack row 0 / 1 / 1 / 1 / 1 / 1.
//   R6. The same arm under FLOOR is identical to the stepwise arm everywhere:
//       rate 0.00% and slack 0 in every wrapping cell, both signednesses.
//   R7. The unsigned rows stay at 0.00% under both truncation modes, since
//       toward-zero and floor coincide on non-negative quantities.
//   R8. Every input at which the one-rounding arm differs under toward zero has a
//       negative quantity entering the shift. If any differing input has a
//       non-negative one, the hypothesis is wrong.
//
// CONTROL. R8 is the mechanism control: it is not enough that the numbers match,
// the reason has to match too. A reproduction with the wrong mechanism would be a
// coincidence at two decimal places, and R8 is what distinguishes them.
//
// Run: rustc -O -o /tmp/p3 p3_the_missing_ulp_is_a_rounding_move.rs && /tmp/p3

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    U,
    S,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ovf {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Trunc {
    TowardZero,
    Floor,
}

fn lo(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => 0,
        Sign::S => -(1i128 << (w - 1)),
    }
}
fn hi(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => (1i128 << w) - 1,
        Sign::S => (1i128 << (w - 1)) - 1,
    }
}
fn reduce(v: i128, s: Sign, o: Ovf, w: u32) -> i128 {
    match o {
        Ovf::Sat => v.clamp(lo(s, w), hi(s, w)),
        Ovf::Wrap => {
            let m = 1i128 << w;
            let r = v.rem_euclid(m);
            match s {
                Sign::U => r,
                Sign::S => {
                    if r >= (1i128 << (w - 1)) {
                        r - m
                    } else {
                        r
                    }
                }
            }
        }
    }
}
fn shift(p: i128, f: u32, t: Trunc) -> i128 {
    if f == 0 {
        return p;
    }
    match t {
        Trunc::TowardZero => p / (1i128 << f),
        Trunc::Floor => p >> f,
    }
}

/// The stepwise arm: round the product, reduce it, add, reduce.
fn stepwise(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    reduce(reduce(shift(a * b, f, t), s, o, w) + c, s, o, w)
}
/// The reduction-relocated arm: same rounding, one reduction.
fn fused_same_rounding(
    a: i128,
    b: i128,
    c: i128,
    s: Sign,
    o: Ovf,
    w: u32,
    f: u32,
    t: Trunc,
) -> i128 {
    reduce(shift(a * b, f, t) + c, s, o, w)
}
/// The rounding-relocated arm: one rounding over the whole expression.
fn fused_one_rounding(
    a: i128,
    b: i128,
    c: i128,
    s: Sign,
    o: Ovf,
    w: u32,
    f: u32,
    t: Trunc,
) -> i128 {
    reduce(shift(a * b + (c << f), f, t), s, o, w)
}

struct Cell {
    rate: f64,
    slack: i128,
    /// R8: differing inputs whose shifted quantity is negative.
    diff_negative: u64,
    /// R8: differing inputs whose shifted quantity is non-negative. Must be 0.
    diff_nonnegative: u64,
}

fn sweep(s: Sign, o: Ovf, w: u32, f: u32, t: Trunc, one_rounding: bool) -> Cell {
    let (l, h) = (lo(s, w), hi(s, w));
    let mut d = 0u64;
    let mut n = 0u64;
    let mut worst = 0i128;
    let mut neg = 0u64;
    let mut nonneg = 0u64;
    for a in l..=h {
        for b in l..=h {
            for c in l..=h {
                n += 1;
                let st = stepwise(a, b, c, s, o, w, f, t);
                let fu = if one_rounding {
                    fused_one_rounding(a, b, c, s, o, w, f, t)
                } else {
                    fused_same_rounding(a, b, c, s, o, w, f, t)
                };
                if st != fu {
                    d += 1;
                    if a * b + (c << f) < 0 {
                        neg += 1;
                    } else {
                        nonneg += 1;
                    }
                }
                let g = (st - fu).abs();
                if g > worst {
                    worst = g;
                }
            }
        }
    }
    Cell {
        rate: 100.0 * d as f64 / n as f64,
        slack: worst,
        diff_negative: neg,
        diff_nonnegative: nonneg,
    }
}

fn main() {
    let w = 6u32;
    println!("p3: is the residual unit a rounding relocation?\n");

    for t in [Trunc::TowardZero, Trunc::Floor] {
        let tn = match t {
            Trunc::TowardZero => "toward zero",
            Trunc::Floor => "floor",
        };
        println!("=== one-rounding arm vs stepwise, rounding = {tn} ===");
        println!(
            "{:<22} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
            "rate", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
        );
        for s in [Sign::U, Sign::S] {
            for o in [Ovf::Wrap, Ovf::Sat] {
                let name = format!(
                    "{}, {}",
                    if s == Sign::U { "unsigned" } else { "signed" },
                    if o == Ovf::Wrap {
                        "wrapping"
                    } else {
                        "saturating"
                    }
                );
                let mut r1 = format!("{name:<22}");
                let mut r2 = format!("{:<22}", "  slack");
                let mut r8 = format!("{:<22}", "  R8 nonneg-diffs");
                for f in 0..=5u32 {
                    let c = sweep(s, o, w, f, t, true);
                    r1.push_str(&format!(" {:>7.2}%", c.rate));
                    r2.push_str(&format!(" {:>8}", c.slack));
                    r8.push_str(&format!(" {:>8}", c.diff_nonnegative));
                    let _ = c.diff_negative;
                }
                println!("{r1}\n{r2}\n{r8}");
            }
        }
        println!();
    }

    println!("=== 139 p3's slack row for signed wrapping, for comparison: 0 1 1 1 1 1 ===");
    println!(
        "=== 139 sec 4's rate row for signed wrapping:  0.00 1.64 5.54 12.34 22.22 33.40 ===\n"
    );

    println!("=== R6: the one-rounding arm under FLOOR against the stepwise arm ===");
    for s in [Sign::U, Sign::S] {
        for o in [Ovf::Wrap, Ovf::Sat] {
            let name = format!(
                "{}, {}",
                if s == Sign::U { "unsigned" } else { "signed" },
                if o == Ovf::Wrap {
                    "wrapping"
                } else {
                    "saturating"
                }
            );
            let mut row = format!("{name:<22}");
            for f in 0..=5u32 {
                let c = sweep(s, o, w, f, Trunc::Floor, true);
                row.push_str(&format!(" {:>7.2}%", c.rate));
            }
            println!("{row}");
        }
    }
}
