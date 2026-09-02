// Probe D. Whether a per-operation contract composes into a chain contract.
//
// Take the strongest per-operation accuracy contract there is: every operation
// returns the correctly rounded result, round-to-nearest, ties to even. If
// per-operation contracts composed, a chain of correctly rounded operations
// would give the correctly rounded chain result. This probe asks whether it does.
//
// The instance: a fixed-point product. Two Q(.F) fractions multiply to an exact
// Q(.2F) value. A design that stores the intermediate at M fraction bits, F <= M
// <= 2F, rounds twice: once to M and once to F. A design that keeps the exact
// product rounds once. Compare, exhaustively over every operand pair.
//
// THE CASE THAT MUST FAIL.
//   NC6  At M = 2F the intermediate is exact, so the first rounding is the
//        identity and the two arms MUST agree. A disagreement there means the
//        instrument is wrong rather than the arithmetic.
//   NC7  There must exist an M in (F, 2F) where they disagree. If they agreed at
//        every M, per-operation correct rounding would compose and this probe
//        would have found nothing.
//
// Exhaustive: every (a, b) in [0, 2^F)^2. No timing.

/// Round `v`, an integer at scale 2^-from, to scale 2^-to. Nearest, ties to even.
fn rte(v: u64, from: u32, to: u32) -> u64 {
    if to >= from {
        return v << (to - from);
    }
    let s = from - to;
    let half = 1u64 << (s - 1);
    let lo = v & ((1u64 << s) - 1);
    let hi = v >> s;
    if lo > half {
        hi + 1
    } else if lo < half {
        hi
    } else if hi & 1 == 1 {
        hi + 1
    } else {
        hi
    }
}

fn sweep(f: u32) -> Vec<(u32, u64, u64)> {
    let n = 1u64 << f;
    let mut out = Vec::new();
    for m in f..=(2 * f) {
        let mut bad = 0u64;
        let mut total = 0u64;
        for a in 0..n {
            for b in 0..n {
                let p = a * b; // exact, at scale 2^-2f
                total += 1;
                let direct = rte(p, 2 * f, f);
                let via = rte(rte(p, 2 * f, m), m, f);
                if direct != via {
                    bad += 1;
                }
            }
        }
        out.push((m, bad, total));
    }
    out
}

fn main() {
    for &f in &[6u32, 8, 10] {
        println!("== F = {f} fraction bits, exact product has {} ==", 2 * f);
        println!(
            "{:>6}  {:>14}  {:>14}  {:>12}",
            "M", "pairs", "disagreements", "ppm"
        );
        let rows = sweep(f);
        for (m, bad, total) in &rows {
            let ppm = (*bad as f64) * 1.0e6 / (*total as f64);
            println!("{m:>6}  {total:>14}  {bad:>14}  {ppm:>12.2}");
        }
        let at_2f = rows.iter().find(|(m, _, _)| *m == 2 * f).unwrap();
        let any_bad = rows.iter().any(|(m, b, _)| *m > f && *m < 2 * f && *b > 0);
        println!(
            "  NC6 (M = 2F must agree): {}",
            if at_2f.1 == 0 { "ok" } else { "FAIL" }
        );
        println!(
            "  NC7 (some M in (F, 2F) must disagree): {}",
            if any_bad { "ok" } else { "FAIL, nothing shown" }
        );
        // The smallest M at which the double rounding becomes harmless.
        let first_clean = rows
            .iter()
            .filter(|(_, b, _)| *b == 0)
            .map(|(m, _, _)| *m)
            .min()
            .unwrap();
        println!(
            "  smallest M with zero disagreements: {first_clean}  (2F = {})",
            2 * f
        );
        println!();
    }
}
