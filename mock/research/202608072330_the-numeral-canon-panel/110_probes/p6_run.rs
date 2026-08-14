//! Runner for p6_expressibility.rs. The library compiles; this checks that the
//! values it computes are the ones the comments claim, because "it compiled" is
//! a different claim from "it computes what I said".
//!
//! Build: rustc --edition 2021 p6_run.rs -o p6_run   (it includes the lib)

#[path = "p6_expressibility.rs"]
mod p6;

use p6::*;

fn main() {
    let mut failures = 0;

    // Q2: the two spellings of the degenerate primitive unify, and the
    // arithmetic goes through one path.
    let v = canonical_by_construction();
    check(
        "canonical_by_construction saturates at 255",
        v,
        255,
        &mut failures,
    );

    // overflow policy is identity-bearing: same width, same step, same
    // rounding, different answers.
    let (s, w) = overflow_policy_is_identity_bearing();
    check("saturating 200 + 100", s, 255, &mut failures);
    check("wrapping   200 + 100", w, 44, &mut failures);
    if s == w {
        println!("FAIL: overflow policy did not separate the two primitives");
        failures += 1;
    }

    // step is identity-bearing: 1.0 * 1.0 in each primitive's own grid units.
    let (b, d) = distinct_primitives_are_distinct_types();
    check("binary  Q8: 1.0 * 1.0 in grid units", b, 256, &mut failures);
    check(
        "decimal Q3: 1.0 * 1.0 in grid units",
        d,
        1000,
        &mut failures,
    );

    // the reachability degeneracy: identical over {add, mul}, separated by half.
    let (hx, hy) = reachability_degeneracy_is_not_merged();
    check("half(7) under nearest rounding", hx, 4, &mut failures);
    check("half(7) under toward-zero", hy, 3, &mut failures);
    if hx == hy {
        println!("FAIL: half did not separate the two rounding modes at step 1");
        failures += 1;
    } else {
        println!(
            "OK  : the two types were indistinguishable over {{add, mul}} and \
             differ under half, which is why merging them would not have been stable"
        );
    }

    if failures == 0 {
        println!("\nall checks passed");
    } else {
        println!("\n{failures} checks FAILED");
        std::process::exit(1);
    }
}

fn check(what: &str, got: i128, want: i128, failures: &mut u32) {
    if got == want {
        println!("OK  : {what} = {got}");
    } else {
        println!("FAIL: {what} = {got}, expected {want}");
        *failures += 1;
    }
}
