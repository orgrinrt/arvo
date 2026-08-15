// p3c: two mechanism guesses refuted by my own controls (R8 in p3, R9 in p3b).
// Derive the mechanism instead of guessing at it, then check the derivation.
//
// The two arms are, writing x = (a*b) / 2^F as an exact rational and c an integer:
//
//     stepwise:  round(x) + c
//     one-round: round(x + c)
//
// They agree for every x and c exactly when `round` is TRANSLATION EQUIVARIANT
// over the integers, that is round(x + c) = round(x) + c.
//
//   - Floor is translation equivariant: floor(x + c) = floor(x) + c for integer c.
//     So under floor the two arms are the same function. That is R12, already
//     confirmed at 0 differing inputs in every cell.
//   - Truncation toward zero is NOT, and its failure is fully characterised:
//     trunc(y) = floor(y) when y >= 0 and floor(y) + 1 when y < 0 and y is not an
//     integer. So round(x) + c and round(x + c) differ exactly when x and x + c
//     fall on OPPOSITE SIDES OF ZERO and the relevant quantity is not an integer.
//
// That is why both earlier controls failed. R8 tested the sign of the sum alone
// and R9 tested the sign of the product alone; the condition is about the two
// disagreeing, so each one-sided test finds roughly half the witnesses on the
// wrong side and reports a large non-zero count.
//
// PREDICTIONS, before running:
//   R13. Differing inputs are EXACTLY the inputs where the shift is inexact and
//        x and x+c lie on opposite sides of zero. Both error columns, predicted-
//        but-agreed and agreed-but-predicted, must be 0.
//   R14. The one that is confirmed by R13 is a fact about the ROUNDING MODE and
//        nothing else, so it must hold identically for unsigned values whenever
//        a negative quantity can arise. Unsigned cannot produce one here, which
//        is why the unsigned rows are 0.00% and why that zero says nothing about
//        fusion.
//
// CONTROL. R13 is stated as a biconditional and checked in both directions, which
// is what distinguishes a mechanism from a correlation. A predicate that merely
// covers the differing inputs would be satisfied by "true".
//
// Run: rustc -O -o /tmp/p3c p3c_translation_equivariance_is_the_mechanism.rs && /tmp/p3c

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
    if f == 0 { p } else { p / (1i128 << f) }
}

fn main() {
    let w = 6u32;
    println!("p3c: translation equivariance as the mechanism");
    println!("signed, wrapping, toward-zero rounding, W = {w}, exhaustive\n");
    println!(
        "{:>3} {:>10} {:>12} {:>22} {:>22}",
        "F", "differing", "predicted", "predicted but agreed", "differed but unpredicted"
    );

    for f in 0..=5u32 {
        let (l, h) = (lo_s(w), hi_s(w));
        let scale = 1i128 << f;
        let mut differing = 0u64;
        let mut predicted = 0u64;
        let mut pred_but_agreed = 0u64;
        let mut diff_but_unpred = 0u64;
        for a in l..=h {
            for b in l..=h {
                let prod = a * b;
                for c in l..=h {
                    let sum = prod + (c << f);
                    // exact rational comparison against zero, without division
                    let x_neg = prod < 0;
                    let xc_neg = sum < 0;
                    // "not an integer" on the side that is negative
                    let prod_inexact = f > 0 && prod.rem_euclid(scale) != 0;
                    // opposite sides of zero, with the negative side non-integral
                    let cond = prod_inexact && (x_neg != xc_neg);

                    let st = wrap_s(wrap_s(tz(prod, f), w) + c, w);
                    let fu = wrap_s(tz(sum, f), w);
                    let d = st != fu;

                    if d {
                        differing += 1;
                    }
                    if cond {
                        predicted += 1;
                    }
                    if cond && !d {
                        pred_but_agreed += 1;
                    }
                    if d && !cond {
                        diff_but_unpred += 1;
                    }
                }
            }
        }
        println!("{f:>3} {differing:>10} {predicted:>12} {pred_but_agreed:>22} {diff_but_unpred:>24}");
    }

    println!("\nR13 holds iff both error columns are 0 at every F.");
    println!("If they are, the disagreement between 139's table and mine is entirely");
    println!("the failure of toward-zero truncation to commute with integer translation,");
    println!("and floor, which does commute, removes it at no cost.");
}
