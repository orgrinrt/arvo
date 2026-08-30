// p3b: p3's numbers reproduced 139's table exactly and p3's MECHANISM control
// failed, so the reproduction was not earned yet.
//
// p3's R8 predicted that every differing input has a negative quantity entering
// the shift, and tested that by the sign of `a*b + (c << F)`. It reported
// thousands of differing inputs with that quantity non-negative, so R8 as written
// is REFUTED and stands refuted in p3_out.txt.
//
// The error is mine and it is in the predicate, not in the hypothesis. Toward-zero
// truncation fails to commute with adding an integer when the quantity being
// truncated BEFORE the addition is negative, which is the product quotient, not
// the sum:
//
//     trunc(-1.5) + 3 = -1 + 3 =  2
//     trunc(-1.5  + 3) = trunc(1.5) =  1
//
// The sum is positive in that witness and the difference still occurs. So the
// correct predicate is `a*b < 0 and the shift is inexact`, and p3's control tested
// the sign of the wrong quantity.
//
// PREDICTIONS, before running:
//   R9.  For every signed WRAPPING cell, every differing input has a*b < 0.
//        Count of differing inputs with a*b >= 0 must be exactly 0.
//   R10. For every signed WRAPPING cell, every differing input has an inexact
//        shift of a*b. Count with an exact shift must be exactly 0.
//   R11. The difference, where it occurs, is exactly one unit in magnitude and
//        always in the same direction. Count of magnitudes other than 1 must be 0.
//   R12. Replacing toward-zero with floor removes every differing input in those
//        cells, which p3 already showed at the level of rates and which is
//        restated here at the level of individual witnesses.
//
// CONTROL. R11 is the one that can fail informatively: if some difference has
// magnitude 2, the mechanism is not a single rounding relocation and the
// explanation is incomplete. A probe that only counts differences cannot tell a
// one-unit rounding move from a coincidence of rates.
//
// Run: rustc -O -o /tmp/p3b p3b_the_mechanism_control_corrected.rs && /tmp/p3b

fn lo_s(w: u32) -> i128 {
    -(1i128 << (w - 1))
}
fn hi_s(w: u32) -> i128 {
    (1i128 << (w - 1)) - 1
}

fn wrap_s(v: i128, w: u32) -> i128 {
    let m = 1i128 << w;
    let r = v.rem_euclid(m);
    if r >= (1i128 << (w - 1)) {
        r - m
    } else {
        r
    }
}

fn tz(p: i128, f: u32) -> i128 {
    if f == 0 {
        p
    } else {
        p / (1i128 << f)
    }
}
fn fl(p: i128, f: u32) -> i128 {
    if f == 0 {
        p
    } else {
        p >> f
    }
}

fn main() {
    let w = 6u32;
    println!("p3b: corrected mechanism control for the one-unit rounding relocation");
    println!("signed, wrapping, W = {w}, exhaustive over all triples\n");

    for (name, sh) in [
        ("toward zero", tz as fn(i128, u32) -> i128),
        ("floor", fl as fn(i128, u32) -> i128),
    ] {
        println!("--- rounding = {name} ---");
        println!(
            "{:>3} {:>9} {:>12} {:>14} {:>14} {:>16}",
            "F", "differing", "with a*b>=0", "with exact shift", "|delta| != 1", "max |delta|"
        );
        for f in 0..=5u32 {
            let (l, h) = (lo_s(w), hi_s(w));
            let mut diff = 0u64;
            let mut nonneg_prod = 0u64;
            let mut exact_shift = 0u64;
            let mut wrong_mag = 0u64;
            let mut maxmag = 0i128;
            for a in l..=h {
                for b in l..=h {
                    let prod = a * b;
                    let q = sh(prod, f);
                    let exact = f == 0 || q * (1i128 << f) == prod;
                    for c in l..=h {
                        let st = wrap_s(wrap_s(q, w) + c, w);
                        let fu = wrap_s(sh(prod + (c << f), f), w);
                        if st != fu {
                            diff += 1;
                            if prod >= 0 {
                                nonneg_prod += 1;
                            }
                            if exact {
                                exact_shift += 1;
                            }
                            // Compare on the pre-wrap values so the magnitude is
                            // the rounding difference and not a wrap artifact.
                            let m = (sh(prod + (c << f), f) - (q + c)).abs();
                            if m != 1 {
                                wrong_mag += 1;
                            }
                            if m > maxmag {
                                maxmag = m;
                            }
                        }
                    }
                }
            }
            println!(
                "{f:>3} {diff:>9} {nonneg_prod:>12} {exact_shift:>16} {wrong_mag:>14} {maxmag:>16}"
            );
        }
        println!();
    }

    println!("Verdicts:");
    println!("  R9  holds iff the 'with a*b>=0' column is 0 in every toward-zero row");
    println!("  R10 holds iff the 'with exact shift' column is 0 in every toward-zero row");
    println!("  R11 holds iff the '|delta| != 1' column is 0 and max |delta| is 1");
    println!("  R12 holds iff the floor block has 0 differing inputs at every F");
}
