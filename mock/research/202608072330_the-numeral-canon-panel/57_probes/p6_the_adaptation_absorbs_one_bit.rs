//! Probe 6: chasing the one-bit gap `p5` section 2c opened.
//!
//! `p5` predicted the accumulator width needed for an n-operand fold of a
//! W-bit signed saturating format to be W + ceil(log2 n), which is the width
//! that holds the exact sum. The prediction was SOUND at every n measured, and
//! it was also WRONG BY EXACTLY ONE BIT at every n: measured sufficient widths
//! were 4, 5, 5, 6 against predicted 5, 6, 6, 7.
//!
//! A sound-but-loose bound is a finding to attack, not to report. One bit per
//! accumulator is not nothing: it is the difference between a fold of 16-bit
//! values needing a 32-bit accumulator and needing a 31-bit one, which decides
//! container selection at exactly the boundaries where container selection is
//! interesting.
//!
//! THE MECHANISM I think is responsible. The naive bound asks the accumulator
//! to hold the exact sum. It does not have to. It only has to hold enough to
//! agree with the exact sum AFTER the final adaptation, and the final
//! adaptation clamps into Q anyway. So an accumulator that saturates is
//! harmless whenever the exact value was already outside Q on the same side and
//! the remaining operands cannot pull it back across. The outermost bit of the
//! exact bound is therefore absorbed by the adaptation that was going to happen
//! regardless. This is the same absorption property `p1` and `p2` identify as
//! the associativity criterion, appearing here as a width saving rather than as
//! a law.
//!
//! WHAT THIS PROBE ASKS. Is "exact bits minus one" sufficient in general, or
//! did `p5` measure a coincidence of one format at four fold lengths? Swept
//! over signed formats W = 3..6 and unsigned formats W = 3..5, fold lengths
//! from 2 up to whatever keeps the tuple count under about 3.4e7, exhaustively
//! in every case. Reported per row: the exact-sum width, the measured minimum
//! sufficient accumulator width, and the gap. ANY row where the gap is not one
//! is printed as the boundary of the rule rather than smoothed away.
//!
//! INSTRUMENT VALIDATION. The minimum-sufficient search must find a width at
//! which divergence is nonzero and a width at which it is zero, in the same
//! row, or the row proves nothing; rows failing that are reported. A row where
//! even the format's own width already suffices is reported separately, since
//! there the question does not arise and including it in the gap statistics
//! would dilute them.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p6 p6_the_adaptation_absorbs_one_bit.rs && ./p6

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    Signed,
    Unsigned,
}

fn format_range(s: Sign, w: u32) -> (i64, i64) {
    match s {
        Sign::Signed => (-(1i64 << (w - 1)), (1i64 << (w - 1)) - 1),
        Sign::Unsigned => (0, (1i64 << w) - 1),
    }
}

/// divergence count between "saturate into a width-w accumulator at every step,
/// adapt once at the end" and "sum exactly, adapt once at the end"
fn divergence(s: Sign, w_fmt: u32, n: usize, w_acc: u32) -> u64 {
    let (qlo, qhi) = format_range(s, w_fmt);
    let (alo, ahi) = format_range(s, w_acc);
    let span = (qhi - qlo + 1) as usize;
    let mut diff = 0u64;
    let mut idx = vec![0usize; n];
    loop {
        let mut acc = 0i64;
        let mut exact = 0i64;
        for k in 0..n {
            let v = qlo + idx[k] as i64;
            acc = (acc + v).clamp(alo, ahi);
            exact += v;
        }
        if acc.clamp(qlo, qhi) != exact.clamp(qlo, qhi) {
            diff += 1;
        }
        let mut k = 0;
        loop {
            if k == n {
                return diff;
            }
            idx[k] += 1;
            if idx[k] < span {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
    }
}

/// width needed to hold every exact sum of n operands drawn from the format
fn exact_sum_width(s: Sign, w_fmt: u32, n: usize) -> u32 {
    let (qlo, qhi) = format_range(s, w_fmt);
    let lo = qlo * n as i64;
    let hi = qhi * n as i64;
    let mut w = 1u32;
    loop {
        let (alo, ahi) = format_range(s, w);
        if alo <= lo && hi <= ahi {
            return w;
        }
        w += 1;
        if w > 62 {
            return w;
        }
    }
}

fn main() {
    let mut ok = true;

    println!("=== minimum sufficient accumulator width against the exact-sum width ===");
    println!();
    println!("  'exact' is the width that holds every exact sum. 'measured' is the");
    println!("  smallest accumulator width at which eager saturation into the");
    println!("  accumulator, adapted once, agrees with the exact sum adapted once, over");
    println!("  every tuple. 'gap' is exact minus measured.");
    println!();
    println!(
        "{:>10} {:>4} {:>4} {:>12} {:>8} {:>10} {:>6} {:>26}",
        "sign", "W", "n", "tuples", "exact", "measured", "gap", "note"
    );

    let mut gaps: Vec<i64> = Vec::new();
    let mut anomalies: Vec<String> = Vec::new();
    let mut rows_with_both_verdicts = 0u64;
    let mut rows_trivial = 0u64;

    for (sname, s) in [("signed", Sign::Signed), ("unsigned", Sign::Unsigned)] {
        for w_fmt in 3..=6u32 {
            if s == Sign::Unsigned && w_fmt > 5 {
                continue;
            }
            for n in 2..=8usize {
                let (qlo, qhi) = format_range(s, w_fmt);
                let span = (qhi - qlo + 1) as u128;
                let tuples = span.pow(n as u32);
                if tuples > 34_000_000 {
                    continue;
                }

                let exact = exact_sum_width(s, w_fmt, n);
                let mut measured: Option<u32> = None;
                let mut saw_nonzero = false;
                for w_acc in w_fmt..=(exact + 2) {
                    let d = divergence(s, w_fmt, n, w_acc);
                    if d > 0 {
                        saw_nonzero = true;
                    }
                    if d == 0 && measured.is_none() {
                        measured = Some(w_acc);
                    }
                }
                let measured = measured.unwrap_or(99);
                let gap = exact as i64 - measured as i64;

                let note = if !saw_nonzero {
                    rows_trivial += 1;
                    "format width already suffices"
                } else {
                    rows_with_both_verdicts += 1;
                    gaps.push(gap);
                    if gap != 1 {
                        anomalies.push(format!(
                            "{} W={} n={}: exact={} measured={} gap={}",
                            sname, w_fmt, n, exact, measured, gap
                        ));
                        "GAP IS NOT ONE"
                    } else {
                        ""
                    }
                };

                println!(
                    "{:>10} {:>4} {:>4} {:>12} {:>8} {:>10} {:>6} {:>26}",
                    sname, w_fmt, n, tuples, exact, measured, gap, note
                );
            }
        }
    }

    println!();
    println!("=== what the sweep says ===");
    println!();
    println!(
        "  rows where the question arises (some width diverged, some did not): {}",
        rows_with_both_verdicts
    );
    println!(
        "  rows where the format's own width already sufficed:                 {}",
        rows_trivial
    );
    let all_one = gaps.iter().all(|&g| g == 1);
    println!(
        "  gap is exactly one bit in every row where the question arises:      {}",
        all_one
    );
    if !anomalies.is_empty() {
        println!();
        println!("  rows where it is not, which are the boundary of the rule:");
        for a in &anomalies {
            println!("    {}", a);
        }
    }
    println!();
    println!("  Reading if the gap is uniformly one: the naive 'accumulate the exact");
    println!("  sum' bound is one bit loose, because the adaptation that has to happen");
    println!("  at the end absorbs the outermost bit. The accumulator has to distinguish");
    println!("  which side of the format the result falls on, not to represent how far");
    println!("  outside it fell.");
    println!();
    println!("  This is measured, at the widths and fold lengths in the table and no");
    println!("  further. It is exhaustive within each row and it is not a proof, and the");
    println!("  unsigned rows and the signed rows are the same claim at two sign domains");
    println!("  rather than two independent instances of it.");

    println!();
    println!("=== instrument validation ===");
    println!();
    println!(
        "  rows carrying both a diverging and a non-diverging width: {}",
        rows_with_both_verdicts
    );
    println!("  (a row with only one verdict cannot locate a minimum and is excluded above)");
    // the search must be capable of reporting a gap other than one: check that
    // a deliberately wrong claim is refuted by the same data
    let bogus_gap_zero = gaps.iter().any(|&g| g == 0);
    println!(
        "  the sweep contains a row with gap 0, which would refute the rule: {}",
        bogus_gap_zero
    );
    ok &= rows_with_both_verdicts > 0;

    println!();
    println!("{}", if ok { "P6 WORKS" } else { "P6 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
