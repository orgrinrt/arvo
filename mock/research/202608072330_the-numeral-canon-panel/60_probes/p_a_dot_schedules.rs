// Probe A: the rounding side of a chain schedule.
//
// Hypothesis: for a fixed-point dot product, the schedule (where the narrowing
// adaptations sit) changes the computed function. Three schedules over the same
// exact ops:
//   S-trunc: narrow each product to F by truncation, then sum (adds exact).
//   S-rne:   narrow each product to F by round-nearest-even, then sum.
//   W:       sum all products exactly at 2F, single RNE narrow at the end.
// Claim: W equals the correctly rounded exact composite (grade-a exactness) on
// every input; S-trunc and S-rne do not, and their drift is bounded by the
// number of adaptation points.
//
// Oracle: NOT a recomputation of the same rounding. The checker verifies the
// DEFINING PROPERTY of correct rounding: a claimed result c at F against the
// exact numerator n at 2F satisfies |c*2^F - n| <= 2^(F-1), with the tie going
// to even c. A shared-bug tautology between arm and oracle is structurally
// excluded because the oracle never rounds.
//
// Mutant check: arm W with its final narrow replaced by truncation must be
// FLAGGED by the property checker on at least one input, or the harness is
// declared broken and the probe panics.
//
// Shortcuts taken (this is a spike): unsigned values only, F = 8 fixed, bare
// primitives, std println. None of these bear on the hypothesis.

const F: u32 = 8;
// Six raw values at F=8; 3 and 128 are present so that tie cases (rem == 128)
// occur at the final narrow and the tie-to-even path is exercised.
const RAWS: [u64; 6] = [1, 3, 77, 128, 200, 255];
const K: usize = 3; // dot product length

fn rne_narrow(n: u128, shift: u32) -> u128 {
    let q = n >> shift;
    let rem = n & ((1u128 << shift) - 1);
    let half = 1u128 << (shift - 1);
    if rem > half || (rem == half && (q & 1) == 1) {
        q + 1
    } else {
        q
    }
}

// Defining property of round-nearest-even, checked without rounding anything:
// c is at scale F, n is at scale 2F. Value error |c/2^F - n/2^{2F}| must be
// <= 2^{-F-1}, i.e. |c*2^F - n| <= 2^{F-1}, and equality only with c even.
fn is_correctly_rounded(c: u128, n: u128, f: u32) -> bool {
    let scaled = c << f;
    let diff = if scaled > n { scaled - n } else { n - scaled };
    let half = 1u128 << (f - 1);
    diff < half || (diff == half && (c & 1) == 0)
}

fn main() {
    let mut total = 0u64;
    let mut strunc_wrong = 0u64;
    let mut srne_wrong = 0u64;
    let mut mutant_flagged = 0u64;
    let mut strunc_max_drift = 0i128;
    let mut srne_max_drift = 0i128;

    // exhaustive over RAWS^(2K) = 6^6 = 46656 input tuples
    let m = RAWS.len();
    let mut idx = [0usize; 2 * K];
    loop {
        let a = [RAWS[idx[0]], RAWS[idx[1]], RAWS[idx[2]]];
        let b = [RAWS[idx[3]], RAWS[idx[4]], RAWS[idx[5]]];

        // exact numerator at 2F
        let n_exact: u128 = (0..K).map(|i| (a[i] as u128) * (b[i] as u128)).sum();

        // arm S-trunc
        let s_trunc: u128 = (0..K).map(|i| ((a[i] as u128) * (b[i] as u128)) >> F).sum();
        // arm S-rne
        let s_rne: u128 = (0..K)
            .map(|i| rne_narrow((a[i] as u128) * (b[i] as u128), F))
            .sum();
        // arm W: exact wide sum, single narrow
        let w = rne_narrow(n_exact, F);
        // mutant W': truncating final narrow
        let w_mut = n_exact >> F;

        total += 1;

        // grade-a claim: W is correctly rounded on EVERY input
        if !is_correctly_rounded(w, n_exact, F) {
            panic!("arm W failed correct-rounding property: a={a:?} b={b:?}");
        }
        if !is_correctly_rounded(s_trunc, n_exact, F) {
            strunc_wrong += 1;
            let d = (s_trunc as i128 - w as i128).abs();
            if d > strunc_max_drift {
                strunc_max_drift = d;
            }
        }
        if !is_correctly_rounded(s_rne, n_exact, F) {
            srne_wrong += 1;
            let d = (s_rne as i128 - w as i128).abs();
            if d > srne_max_drift {
                srne_max_drift = d;
            }
        }
        if !is_correctly_rounded(w_mut, n_exact, F) {
            mutant_flagged += 1;
        }

        // odometer
        let mut i = 0;
        loop {
            idx[i] += 1;
            if idx[i] < m {
                break;
            }
            idx[i] = 0;
            i += 1;
            if i == 2 * K {
                break;
            }
        }
        if i == 2 * K {
            break;
        }
    }

    // the instrument must be able to fail
    assert!(
        mutant_flagged > 0,
        "HARNESS BROKEN: truncating mutant was never flagged"
    );

    println!(
        "inputs tested (count = 3-element (a,b) raw-vector pairs over a 6-value set): {total}"
    );
    println!("arm W (wide exact + single RNE narrow): correctly rounded on ALL inputs");
    println!(
        "arm S-trunc: not correctly rounded on {strunc_wrong} inputs, max drift {strunc_max_drift} ulp at F={F}"
    );
    println!(
        "arm S-rne:   not correctly rounded on {srne_wrong} inputs, max drift {srne_max_drift} ulp at F={F}"
    );
    println!("mutant W-trunc flagged on {mutant_flagged} inputs (instrument CAN fail)");
    println!("OUTCOME: WORKS");
}
