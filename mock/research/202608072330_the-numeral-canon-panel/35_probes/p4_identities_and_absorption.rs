// p4: which fold seeds exist in a numeral, and which top absorbs.
//
// Every fold needs a seed, and the seed is the operation's identity: 0 for a
// sum, 1 for a product, the top for a min, the bottom for a max. A fold whose
// seed is not representable in the numeral it folds is not merely awkward, it
// is wrong, and it is wrong silently.
//
// Two questions, over the whole (W, F) box rather than a sample:
//
//   1. Is the multiplicative identity representable? Its raw encoding is
//      1 << F in a container of W bits, so it fits exactly when F < W, which
//      for an unsigned numeral means at least one integer bit. Where it does
//      not fit, what does a product fold seeded with the nearest thing do?
//
//   2. Is the top absorbing for addition, i.e. does TOP + x == TOP? This is
//      what a min-plus (tropical) computation needs, because its additive
//      identity is infinity and a bounded numeral has to stand infinity on its
//      top. Every shortest-path and DAG-relaxation algorithm is a min-plus or
//      max-plus fold, so this decides whether they can run on the numeral at
//      all.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p4 p4_identities_and_absorption.rs && ./p4

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
}

#[inline(always)]
fn addu(a: u128, b: u128, w: u32, p: Policy) -> u128 {
    let m: u128 = 1u128 << w;
    let s = a + b;
    match p {
        Policy::Wrap => s & (m - 1),
        Policy::Saturate => {
            if s >= m {
                m - 1
            } else {
                s
            }
        }
    }
}

#[inline(always)]
fn mulu(a: u128, b: u128, w: u32, f: u32, p: Policy) -> u128 {
    let m: u128 = 1u128 << w;
    let s = (a * b) >> f;
    match p {
        Policy::Wrap => s & (m - 1),
        Policy::Saturate => {
            if s >= m {
                m - 1
            } else {
                s
            }
        }
    }
}

fn main() {
    println!(
        "w,f,policy,one_representable,x_times_seed_wrong,x_total,top_absorbing_failures,top_total"
    );

    for w in 2..=10u32 {
        for f in 0..=w {
            for (p, pn) in [(Policy::Wrap, "wrap"), (Policy::Saturate, "saturate")] {
                let n: u128 = 1u128 << w;
                let top = n - 1;

                // The raw encoding of the value one.
                let one_raw: u128 = 1u128 << f;
                let one_fits = one_raw < n;

                // The seed a fold reaches for: the true encoding if it fits,
                // otherwise whatever the policy leaves behind when the carry
                // walks out of the container. That is the failure mode the
                // shipped tree documents at arvo/src/ufixed.rs:90-95, measured
                // here rather than quoted.
                let seed = if one_fits {
                    one_raw
                } else {
                    match p {
                        Policy::Wrap => one_raw & (n - 1), // wraps to 0
                        Policy::Saturate => top,           // clamps just below one
                    }
                };

                // How many x have x * seed != x, i.e. how many values a
                // product fold's seed corrupts on the very first step.
                let mut wrong = 0u64;
                for x in 0..n {
                    if mulu(x, seed, w, f, p) != x {
                        wrong += 1;
                    }
                }

                // Is the top absorbing for +? Needed for min-plus.
                let mut absorb_fail = 0u64;
                for x in 0..n {
                    if addu(top, x, w, p) != top {
                        absorb_fail += 1;
                    }
                }

                println!(
                    "{},{},{},{},{},{},{},{}",
                    w, f, pn, one_fits, wrong, n, absorb_fail, n
                );
            }
        }
    }
}
