// Probe E. Probe C found that a rounding policy retracts only at F = 0, so a
// per-operation marker cannot deliver "accurate within chains, not only alone"
// for any fractional type. That is a blocker, and this probe attacks it rather
// than reporting it.
//
// The construction under test is the obvious one from exact fixed-point
// arithmetic: DO NOT QUANTISE IN THE INTERIOR. Let the declared width grow
// through the chain and quantise once, where the consumer asks for it. Under
// that construction "accurate within chains" is not a policy at all; it is a
// representation discipline, and the only question left is what it costs and
// where it runs out.
//
// Three things are measured:
//
//   1. does the widening chain in fact land on the exactly-rounded answer,
//      where the per-operation chain does not, and by how much;
//   2. how the width grows, exactly, per chain shape;
//   3. the availability predicate: the chain length at which the grown width
//      leaves the widest native rung, per starting width.
//
// Build and run:
//   rustc --edition 2024 -O -o e_chain_exactness e_chain_exactness.rs
//   ./e_chain_exactness

/// The widest rung a native container reaches.
const WIDEST: u32 = 128;

// --------------------------------------------------------------------------
// Part 1. Accuracy. A chain of k multiplies, evaluated three ways.
// --------------------------------------------------------------------------

fn part1() {
    println!("part 1: accuracy of a k-multiply chain, three evaluation orders");
    println!("        declared type W = 16, F = 8; deterministic stride sweep, NOT exhaustive");
    println!(
        "{:>3} {:>12} {:>16} {:>16} {:>16} {:>16}",
        "k", "chains", "per-op != exact", "per-op max ulp", "widen != exact", "widen max ulp"
    );

    const F: u32 = 8;
    const N: i128 = 1 << 16;

    for k in 1..=5usize {
        let mut per_bad: u64 = 0;
        let mut per_worst: i128 = 0;
        let mut wid_bad: u64 = 0;
        let mut wid_worst: i128 = 0;
        let mut total: u64 = 0;

        // Deterministic stride sweep of k-tuples over the domain, on coprime
        // strides so the low fraction bits are exercised at every magnitude.
        // This is a sweep and not an exhaustive check: the domain of a
        // 5-tuple over 2^16 is 2^80. The claim it supports is existential
        // (per-operation rounding diverges by this much) plus a bound
        // observed over the sweep, not a universal over the whole domain.
        let strides: [i128; 5] = [4409, 2803, 1741, 1123, 733];
        let mut seed: i128 = 1;
        while seed < N {
            let mut vals = [0i128; 5];
            for j in 0..k {
                vals[j] = (seed * strides[j]) % N;
                if vals[j] == 0 {
                    vals[j] = 1;
                }
            }
            total += 1;

            // (a) per-operation: quantise back to F after every multiply
            let mut per = vals[0];
            for j in 1..k {
                per = (per * vals[j]) >> F;
            }

            // (b) widening: never quantise in the interior, quantise once
            let mut wide = vals[0];
            for j in 1..k {
                wide *= vals[j];
            }
            // the product of k values each scaled 2^F is scaled 2^(k*F);
            // returning to the declared scale is one shift of (k-1)*F
            let shift = (k as u32 - 1) * F;
            let widen = wide >> shift;

            // (c) the exactly-rounded reference: same as (b) but rounded
            // rather than truncated, computed independently
            let half: i128 = if shift == 0 { 0 } else { 1 << (shift - 1) };
            let exact = (wide + half) >> shift;

            if per != exact {
                per_bad += 1;
                let d = (per - exact).abs();
                if d > per_worst {
                    per_worst = d;
                }
            }
            if widen != exact {
                wid_bad += 1;
                let d = (widen - exact).abs();
                if d > wid_worst {
                    wid_worst = d;
                }
            }
            seed += 7;
        }
        println!(
            "{:>3} {:>12} {:>16} {:>16} {:>16} {:>16}",
            k, total, per_bad, per_worst, wid_bad, wid_worst
        );
    }
    println!();
    println!("  the widening column's residual is at most 1 ulp and comes from truncation");
    println!("  against a round-to-nearest reference, which is a choice of rounding at the");
    println!("  single final quantisation rather than accumulated interior loss. The per-op");
    println!("  column is interior loss and grows with k.");
}

// --------------------------------------------------------------------------
// Part 2. The cost. Exactly how the declared width grows.
// --------------------------------------------------------------------------

fn mul_width(i: u32, f: u32, k: u32) -> (u32, u32) {
    // k factors, each (i, f): integer parts add, fraction parts add
    (i * k, f * k)
}

fn add_width(i: u32, f: u32, n: u32) -> (u32, u32) {
    // n terms, each (i, f): the integer part gains ceil(log2 n)
    let mut lg = 0u32;
    while (1u32 << lg) < n {
        lg += 1;
    }
    (i + lg, f)
}

fn part2() {
    println!();
    println!("part 2: how the declared width grows under the no-interior-quantisation rule");
    println!(
        "{:>10} {:>6} {:>8} {:>14} {:>14}",
        "shape", "k or n", "start I.F", "grown I.F", "grown total"
    );
    for (i, f) in [(8u32, 8u32), (4, 12), (16, 16), (1, 15)] {
        for k in [2u32, 3, 4, 8] {
            let (gi, gf) = mul_width(i, f, k);
            println!(
                "{:>10} {:>6} {:>8} {:>14} {:>14}",
                "mul chain",
                k,
                format!("{i}.{f}"),
                format!("{gi}.{gf}"),
                gi + gf
            );
        }
        for n in [4u32, 64, 1024, 65536] {
            let (gi, gf) = add_width(i, f, n);
            println!(
                "{:>10} {:>6} {:>8} {:>14} {:>14}",
                "add fold",
                n,
                format!("{i}.{f}"),
                format!("{gi}.{gf}"),
                gi + gf
            );
        }
    }
}

// --------------------------------------------------------------------------
// Part 3. The availability predicate. Where does the construction run out?
// --------------------------------------------------------------------------

fn part3() {
    println!();
    println!("part 3: the availability predicate, i.e. the longest chain that still fits");
    println!("        the widest native rung ({WIDEST} bits)");
    println!(
        "{:>10} {:>10} {:>22} {:>26}",
        "start I.F", "total", "max mul-chain length k", "max add-fold width n"
    );
    for (i, f) in [
        (1u32, 7u32),
        (4, 4),
        (8, 8),
        (4, 12),
        (16, 16),
        (32, 32),
        (1, 15),
        (13, 0),
    ] {
        let w = i + f;
        // multiplication: k factors need k * w bits
        let mut kmax = 1u32;
        while (kmax + 1) * w <= WIDEST {
            kmax += 1;
        }
        // addition: n terms need w + ceil(log2 n) bits, so log2 n <= WIDEST - w
        let headroom = WIDEST.saturating_sub(w);
        let nmax: u128 = if headroom >= 64 {
            u128::MAX
        } else {
            1u128 << headroom
        };
        println!(
            "{:>10} {:>10} {:>22} {:>26}",
            format!("{i}.{f}"),
            w,
            kmax,
            if headroom >= 64 {
                "unbounded in practice".to_string()
            } else {
                format!("{nmax}")
            }
        );
    }
    println!();
    println!("  reading: the add fold is essentially free, since a fold of 2^h terms costs");
    println!("  h bits. The multiply chain is what runs out, and it runs out linearly in the");
    println!("  chain length, which is a compile-time-visible quantity wherever the chain is");
    println!("  written out. That is the predicate the arm is gated on.");
}

fn main() {
    part1();
    part2();
    part3();
}
