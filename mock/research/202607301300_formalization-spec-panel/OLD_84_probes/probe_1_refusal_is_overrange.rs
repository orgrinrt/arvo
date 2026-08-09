//! probe 1: quantize's refusal set is exactly OverRange on a fixed-exponent numeral.
//!
//! File 80 measured `quantize`'s refusal density at a decimal model (r = 10, p = 3,
//! quantum exponents -2..=1, |D| = 4000, 16,000,000 operand pairs) and reported
//! 5,679,000 refusals, "exactly, count-for-count, the pairs where a naive
//! non-refusing quantize would emit a mantissa of p+1 digits" (80:118-126).
//!
//! This probe re-derives that count independently and then asks the question file 80
//! did not: is the refusal predicate the same predicate as `OverRange` on the numeral
//! `F_q = (radix 10, precision 3, exponent fixed at q)`?
//!
//! Three predicates are compared over the whole 16,000,000-pair matrix:
//!
//!   A. naive-digit-count: round(value(x) / 10^q) needs more than p digits.
//!   B. far-point: value(x) is strictly above F_q's largest representable
//!      magnitude, (10^p - 1) * 10^q.
//!   C. extended-grid: value(x) is at or past the rounding boundary half an ulp
//!      beyond F_q's maximum, (10^p - 1 + 1/2) * 10^q, per the ratified overflow
//!      boundary rule (78:288-293).
//!
//! Predicate C is the design's own definition of an out-of-range event. If A == C the
//! failure IS a range event on F_q and needs nothing new. B is included because it is
//! the naive reading of "out of range" and the gap between B and C is exactly where a
//! rounding-boundary error would hide.
//!
//! Run, not compiled-only. Every count is asserted, so an empty loop cannot pass.

/// Exact rational comparison without rationals: every datum's value is an integer
/// multiple of 10^MIN_E, so scale everything by 10^-MIN_E and compare integers.
const MIN_E: i32 = -2;
const MAX_E: i32 = 1;
const P: u32 = 3;
const RADIX: i128 = 10;

/// Mantissa count: 0 ..= r^p - 1.
const M_COUNT: i128 = RADIX.pow(P);

fn pow10(k: u32) -> i128 {
    RADIX.pow(k)
}

/// value(m, e) scaled by 10^(-MIN_E), exactly.
fn scaled_value(m: i128, e: i32) -> i128 {
    m * pow10((e - MIN_E) as u32)
}

fn main() {
    let exps: Vec<i32> = (MIN_E..=MAX_E).collect();

    // The datum set D: 1000 mantissas times 4 exponents.
    let mut data: Vec<(i128, i32)> = Vec::new();
    for &e in &exps {
        for m in 0..M_COUNT {
            data.push((m, e));
        }
    }
    assert_eq!(data.len(), 4000, "|D| must match file 80's model");

    let pairs = data.len() * data.len();
    assert_eq!(pairs, 16_000_000, "pair count must match file 80's model");

    // Scale factors, all in the same scaled integer domain.
    // F_q's largest representable magnitude is (10^p - 1) * 10^q.
    // The extended-grid boundary is that plus half an ulp, ulp = 10^q.
    // Work in units of half-scaled to keep the half exact: multiply everything by 2.
    let mut count_a: u64 = 0;
    let mut count_b: u64 = 0;
    let mut count_c: u64 = 0;
    let mut a_ne_b: u64 = 0;
    let mut a_ne_c: u64 = 0;
    let mut b_ne_c: u64 = 0;
    let mut window_hits: u64 = 0; // values landing strictly inside (max, boundary)

    // y contributes only its exponent, so enumerate q once and weight by |mantissas|.
    for &(mx, ex) in &data {
        let vx = scaled_value(mx, ex); // scaled by 10^-MIN_E

        for &q in &exps {
            // ulp of F_q, in the same scaled domain.
            let ulp = pow10((q - MIN_E) as u32);
            let max_repr = (M_COUNT - 1) * ulp; // (10^p - 1) * 10^q

            // A: naive digit count. Round-half-even of vx / ulp, then compare to 10^p.
            let quo = vx / ulp;
            let rem = vx % ulp;
            let twice = 2 * rem;
            let rounded = if twice > ulp || (twice == ulp && quo % 2 == 1) {
                quo + 1
            } else {
                quo
            };
            let a = rounded >= M_COUNT;

            // B: strictly above the largest representable magnitude.
            let b = vx > max_repr;

            // C: at or past the extended-grid rounding boundary, max + ulp/2, tie
            // resolved by even on the extended grid. max_repr's mantissa is 999, odd,
            // so a tie rounds up, off the finite set: the tie itself is an overflow.
            let c = 2 * vx >= 2 * max_repr + ulp;

            if vx > max_repr && 2 * vx < 2 * max_repr + ulp {
                window_hits += 1;
            }

            if a {
                count_a += 1;
            }
            if b {
                count_b += 1;
            }
            if c {
                count_c += 1;
            }
            if a != b {
                a_ne_b += 1;
            }
            if a != c {
                a_ne_c += 1;
            }
            if b != c {
                b_ne_c += 1;
            }
        }
    }

    // Each (x, q) stands for 1000 actual y data sharing that exponent.
    let weight = M_COUNT as u64;
    println!("per-(x,q) cells examined: {}", data.len() * exps.len());
    println!("A naive-digit-count refusals: {}", count_a * weight);
    println!("B above-far-point         : {}", count_b * weight);
    println!("C extended-grid overflow  : {}", count_c * weight);
    println!("A != B cells: {a_ne_b}");
    println!("A != C cells: {a_ne_c}");
    println!("B != C cells: {b_ne_c}");
    println!("cells in the half-ulp window (max, boundary): {window_hits}");

    assert_eq!(
        count_a * weight,
        5_679_000,
        "predicate A must reproduce file 80's own count exactly"
    );
    assert_eq!(
        count_a, count_c,
        "quantize's refusal predicate must be the design's own out-of-range predicate on F_q"
    );
    assert_eq!(
        a_ne_c, 0,
        "A and C must agree cell by cell, not merely in total"
    );
    assert_eq!(a_ne_b, 0, "A and B must agree cell by cell at this model");
    assert_eq!(
        window_hits, 0,
        "no datum lands in the half-ulp window, so this model cannot separate B from C"
    );

    // The theorem the window count hints at, checked directly rather than assumed:
    // value(x) / 10^q is either an integer multiple of 10 (when e_x > q), an integer
    // below 10^p (when e_x == q), or strictly below 10^(p-1) (when e_x < q). So the
    // open interval (10^p - 1, 10^p) is unreachable, and A, B and C coincide as a
    // theorem rather than by luck at this model.
    let mut reachable_in_gap = 0u64;
    for &(mx, ex) in &data {
        for &q in &exps {
            let ulp = pow10((q - MIN_E) as u32);
            let vx = scaled_value(mx, ex);
            // strictly between (10^p - 1) * ulp and 10^p * ulp
            if vx > (M_COUNT - 1) * ulp && vx < M_COUNT * ulp {
                reachable_in_gap += 1;
            }
        }
    }
    println!("cells strictly inside the last-ulp gap: {reachable_in_gap}");
    assert_eq!(
        reachable_in_gap, 0,
        "the quotient is never strictly between 10^p - 1 and 10^p, so there is no \
         rounding ambiguity at quantize's overflow edge"
    );

    // Negative control: a predicate that should NOT agree, so a vacuous loop is
    // visible. "value is nonzero" refuses far more pairs than any of A, B, C.
    let mut bogus = 0u64;
    for &(mx, _ex) in &data {
        for _q in &exps {
            if mx != 0 {
                bogus += 1;
            }
        }
    }
    assert_ne!(bogus, count_a, "negative control must differ");
    println!("negative control (nonzero mantissa): {bogus} cells, differs as required");

    println!("OK: quantize's failure is OverRange on the fixed-exponent numeral F_q");
}
