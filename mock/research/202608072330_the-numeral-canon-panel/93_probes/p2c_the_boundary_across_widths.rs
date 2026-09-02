//! P2c. How far the F = 0 boundary reaches across widths.
//!
//! P2b established the boundary at W = 6 only, so its predicate is `W = 6`
//! and nothing wider. This probe widens the measured region, and states the
//! transfer argument for the half of the claim that has one.
//!
//! The claim splits, and the two halves have different evidential status:
//!
//!   F = 0, laws HOLD. This has a proof and does not depend on the sweep.
//!     At F = 0 the multiply performs no shift, so the operation is exactly the
//!     one induced on the quotient of the naturals by "collapse everything at
//!     or above the bound". That collapse is a semiring congruence for both
//!     reduction mod 2^W and clamping at 2^W - 1, so the quotient is a
//!     commutative semiring and inherits every law, at ANY W. The sweep below
//!     is a check on the argument rather than the source of it.
//!
//!   F > 0, laws FAIL. This has no proof here and rests entirely on exhibited
//!     counterexamples, so it is claimed only at the (W, F) pairs actually
//!     swept and nowhere else.
//!
//! Run: rustc --edition 2024 -O p2c_the_boundary_across_widths.rs -o /tmp/p2c && /tmp/p2c

fn run(w: u32, f: u32, sat: bool) -> (u64, u64, u64) {
    let m: u128 = 1 << w;
    let maxv = m - 1;
    let fit = |x: u128| if sat { x.min(maxv) } else { x % m };
    let add = |a: u128, b: u128| fit(a + b);
    let mul = |a: u128, b: u128| fit((a * b) >> f);

    let mut n_triples = 0u64;
    let mut ma = 0u64;
    let mut di = 0u64;
    for a in 0..m {
        for b in 0..m {
            for c in 0..m {
                n_triples += 1;
                if mul(mul(a, b), c) != mul(a, mul(b, c)) {
                    ma += 1;
                }
                if mul(a, add(b, c)) != add(mul(a, b), mul(a, c)) {
                    di += 1;
                }
            }
        }
    }
    (n_triples, ma, di)
}

fn main() {
    println!("P2c. The F = 0 boundary across widths");
    println!("=====================================");
    println!();
    println!("Exhaustive over every triple. No sampling, no stepping.");
    println!("Rounding is truncation throughout; P2b already showed nearest");
    println!("moves the failure rate and not the boundary.");
    println!();
    println!(
        "  {:>3} {:>3} {:>9} {:>14} {:>16} {:>16}",
        "W", "F", "overflow", "triples", "mul-assoc fails", "distrib fails"
    );
    for w in 3..=8u32 {
        for &sat in &[false, true] {
            for f in 0..=2u32 {
                if f > w {
                    continue;
                }
                let (t, ma, di) = run(w, f, sat);
                println!(
                    "  {:>3} {:>3} {:>9} {:>14} {:>16} {:>16}",
                    w,
                    f,
                    if sat { "saturate" } else { "wrap" },
                    t,
                    ma,
                    di
                );
            }
        }
    }
    println!();
    println!("Every F = 0 row is zero and every F > 0 row is nonzero, across six");
    println!("widths and both overflow policies. The F = 0 rows agree with the");
    println!("congruence argument above, which is what makes them extendable past");
    println!("the swept widths; the F > 0 rows are counterexamples and extend to");
    println!("nothing beyond the pairs listed.");
}
