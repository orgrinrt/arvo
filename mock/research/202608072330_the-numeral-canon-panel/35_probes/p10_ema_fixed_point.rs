// p10: the distributivity failure from p2, landed on a named downstream
// consumer as a checkable prediction.
//
// While closing the retraction question (3.9) I read hilavitkutin's adapt-EMA
// provider, read-only, and found a documented invariant that my p2 measurement
// bears on directly. Its module documentation states the update as
//
//     dst[i] = dst[i] * NORM_7_OVER_8 + src[i] * NORM_1_OVER_8
//
// with the two blend factors carried as sixteen-bit fractional constants whose
// raw patterns are 0xE000 and 0x2000, and states that they "sum to exactly
// 1.0 (0x10000 in the same repr), preserving the EMA invariant".
//
// The two constants do sum to one. The claim that follows from it does not,
// and the reason is exactly p2's result: with a fractional part the
// multiplication truncates, so the numeral is not distributive, and
//
//     x*a + x*b  =  x*(a+b)  =  x*1  =  x
//
// is a chain of three steps of which the FIRST is the one that fails. Two
// constants summing to one is a fact about the constants; it does not make the
// computation fix its input.
//
// So the prediction: feeding a constant stream x, the EMA does not settle on
// x. It settles below x, and the shortfall is a function of x mod 8.
//
// WHAT THIS IS AND IS NOT. This is a statement about the formula as documented,
// under truncating fixed-point multiplication. It is not a claim about shipped
// behaviour: that provider's body is marked a stub in its own source, so there
// is nothing shipped to be wrong. It is not a claim about another repository's
// design either, since that source is agent output like everything else. It is
// a prediction handed over, with the arithmetic that produces it, for whoever
// implements the body.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p10 p10_ema_fixed_point.rs && ./p10

// Sixteen fractional bits, the documented BlendFactor repr.
const ONE: u128 = 0x1_0000;
const A: u128 = 0xE000; // 7/8
const B: u128 = 0x2000; // 1/8

// The multiply as a fixed-point multiply: exact product, then rescale by the
// fractional bit count. Truncating, which is what a shift gives.
#[inline(always)]
fn fmul(x: u128, k: u128) -> u128 {
    (x * k) >> 16
}

fn main() {
    // First, the constants really do sum to one, so the premise is not in
    // dispute and the failure is downstream of it.
    assert_eq!(A + B, ONE);

    println!("check,detail,failures,total,pct");

    // 1. Does one update fix its input? dst == src == x should give x.
    let n = 4096u128;
    let mut fp_fail = 0u64;
    let mut worst_shortfall = 0i128;
    for x in 0..n {
        let out = fmul(x, A) + fmul(x, B);
        if out != x {
            fp_fail += 1;
            let d = x as i128 - out as i128;
            if d > worst_shortfall {
                worst_shortfall = d;
            }
        }
    }
    println!(
        "one_update_fixes_input,x in 0..4096,{},{},{:.4}",
        fp_fail,
        n,
        100.0 * fp_fail as f64 / n as f64
    );
    eprintln!("worst single-update shortfall: {}", worst_shortfall);

    // The predicted structure: it holds exactly when x is a multiple of 8.
    let mut structure_violations = 0u64;
    for x in 0..n {
        let holds = fmul(x, A) + fmul(x, B) == x;
        if holds != (x % 8 == 0) {
            structure_violations += 1;
        }
    }
    println!(
        "holds_iff_x_multiple_of_8,x in 0..4096,{},{},{:.4}",
        structure_violations,
        n,
        100.0 * structure_violations as f64 / n as f64
    );

    // 2. The steady state under a constant stream. Iterate to a fixed point and
    // report how far below the input it settles. This is what a consumer of the
    // metric actually observes.
    let mut settled_low = 0u64;
    let mut worst_steady = 0i128;
    let mut worst_x = 0u128;
    for x in 0..n {
        let mut d = x;
        for _ in 0..512 {
            let next = fmul(d, A) + fmul(x, B);
            if next == d {
                break;
            }
            d = next;
        }
        if d != x {
            settled_low += 1;
            let err = x as i128 - d as i128;
            if err > worst_steady {
                worst_steady = err;
                worst_x = x;
            }
        }
    }
    println!(
        "steady_state_equals_input,x in 0..4096,{},{},{:.4}",
        settled_low,
        n,
        100.0 * settled_low as f64 / n as f64
    );
    eprintln!(
        "worst steady-state shortfall: {} at x = {} (relative {:.4}%)",
        worst_steady,
        worst_x,
        100.0 * worst_steady as f64 / worst_x.max(1) as f64
    );

    // 3. The control that says this is the fractional part and not the
    // constants. Round-to-nearest instead of truncation, same constants.
    #[inline(always)]
    fn fmul_rn(x: u128, k: u128) -> u128 {
        ((x * k) + (1 << 15)) >> 16
    }
    let mut rn_fail = 0u64;
    for x in 0..n {
        if fmul_rn(x, A) + fmul_rn(x, B) != x {
            rn_fail += 1;
        }
    }
    println!(
        "one_update_fixes_input_round_nearest,x in 0..4096,{},{},{:.4}",
        rn_fail,
        n,
        100.0 * rn_fail as f64 / n as f64
    );

    // 4. And the control that says it is not specific to 7/8 and 1/8. Every
    // pair of sixteen-bit fractional constants summing to exactly one.
    let mut pairs_with_failures = 0u64;
    let mut pairs = 0u64;
    let mut k = 0u128;
    while k <= ONE {
        let a = k;
        let b = ONE - k;
        pairs += 1;
        let mut bad = false;
        let mut x = 0u128;
        while x < 512 {
            if fmul(x, a) + fmul(x, b) != x {
                bad = true;
                break;
            }
            x += 1;
        }
        if bad {
            pairs_with_failures += 1;
        }
        k += 1;
    }
    println!(
        "any_pair_summing_to_one_fails,all 65537 pairs over x in 0..512,{},{},{:.4}",
        pairs_with_failures,
        pairs,
        100.0 * pairs_with_failures as f64 / pairs as f64
    );
}
