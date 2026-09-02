//! Probe p2b: at w = 4, F = 3, trunc, p2's table shows signed saturation and
//! signed wrap failing multiplicative associativity at the IDENTICAL count,
//! 380. A count equality can be coincidence; a set equality is a mechanism
//! statement. This probe partitions the triples.
//!
//! HYPOTHESIS. At F = w - 1 the rescale divides by 2^(w-1), so almost every
//! pairwise product lands inside the representable range before the
//! reduction map ever fires; the failures are then pure coarsening, shared
//! verbatim by both policies, and the failure SETS should be equal, not
//! merely the counts. If sets are equal at F = 3 and diverge at F = 1
//! (where p2's counts differ: sat 704, wrap 1460), the reading is that the
//! reduction policy's contribution to the failure set shrinks to zero as F
//! approaches the width.
//!
//! INSTRUMENT VALIDATION. The F = 1 rows must show a nonempty symmetric
//! difference (the sets are NOT equal there, per p2's differing counts), so
//! an empty difference at F = 3 is a measurement rather than a comparison
//! that cannot distinguish anything.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p2b p2b_failure_sets_at_deep_fraction.rs && ./p2b

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Sat,
    Wrap,
}

fn reduce(p: Policy, lo: i64, hi: i64, x: i64) -> i64 {
    let m = hi - lo + 1;
    match p {
        Policy::Sat => x.clamp(lo, hi),
        Policy::Wrap => (x - lo).rem_euclid(m) + lo,
    }
}

fn mul(p: Policy, lo: i64, hi: i64, f: u32, a: i64, b: i64) -> i64 {
    reduce(p, lo, hi, (a * b) / (1i64 << f))
}

fn assoc_fails(p: Policy, lo: i64, hi: i64, f: u32, a: i64, b: i64, c: i64) -> bool {
    mul(p, lo, hi, f, mul(p, lo, hi, f, a, b), c) != mul(p, lo, hi, f, a, mul(p, lo, hi, f, b, c))
}

fn partition(lo: i64, hi: i64, f: u32) -> (u64, u64, u64) {
    let (mut both, mut sat_only, mut wrap_only) = (0u64, 0u64, 0u64);
    for a in lo..=hi {
        for b in lo..=hi {
            for c in lo..=hi {
                let s = assoc_fails(Policy::Sat, lo, hi, f, a, b, c);
                let w = assoc_fails(Policy::Wrap, lo, hi, f, a, b, c);
                match (s, w) {
                    (true, true) => both += 1,
                    (true, false) => sat_only += 1,
                    (false, true) => wrap_only += 1,
                    _ => {}
                }
            }
        }
    }
    (both, sat_only, wrap_only)
}

fn main() {
    let mut ok = true;
    let (lo, hi) = (-8i64, 7i64);

    println!("  triples failing *assoc, partitioned by which policy fails them (w=4, trunc):");
    println!();
    println!(
        "  {:>2} {:>8} {:>10} {:>11}",
        "F", "both", "sat only", "wrap only"
    );
    let mut f1_differs = false;
    let mut f3_sets_equal = false;
    for f in 1..=3u32 {
        let (both, so, wo) = partition(lo, hi, f);
        println!("  {:>2} {:>8} {:>10} {:>11}", f, both, so, wo);
        if f == 1 && (so > 0 || wo > 0) {
            f1_differs = true;
        }
        if f == 3 && so == 0 && wo == 0 && both > 0 {
            f3_sets_equal = true;
        }
    }
    println!();
    println!(
        "  F=1 sets differ (instrument can distinguish the policies): {}",
        f1_differs
    );
    println!(
        "  F=3 failure sets are IDENTICAL (pure shared coarsening): {}",
        f3_sets_equal
    );
    ok &= f1_differs && f3_sets_equal;

    println!();
    println!("{}", if ok { "P2B WORKS" } else { "P2B FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
