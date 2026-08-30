// p3: does a fold's answer depend on how the reduction is split?
//
// Op's stated intent (32) is that arvo runs at threads = 1, 2, or n, detected
// rather than declared, doing whatever is most efficient in each situation,
// and (34) that this may not cost soundness for any strategy except Hot.
// Splitting a reduction across lanes or cores changes the association order.
// So the intent has a precondition nobody has stated: the fold's operation
// must be associative, or the answer becomes a function of the detected core
// count.
//
// This measures that directly. For each input vector it computes the same
// fold four ways and counts how many vectors give more than one answer:
//
//   seq    strict left fold, one lane        ((((z+x0)+x1)+x2)+x3)
//   tree   balanced pairwise reduction        ((x0+x1)+(x2+x3))
//   lane2  two partial sums, combined         (x0+x2) + (x1+x3)
//   lane4  four partials, combined pairwise
//
// lane2 and lane4 are what a vectorised or multi-core reduction actually does:
// strided partials, then a horizontal combine. tree is what a recursive
// divide-and-conquer scheduler does.
//
// Coverage is exhaustive over the whole input space at each configuration, so
// nothing about which vectors were tried decides the result. The float arm is
// the exception and says so.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p3 p3_reduction_order.rs && ./p3

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    Unsigned,
    Signed,
}

#[inline(always)]
fn clip(x: i128, w: u32, s: Sign, p: Policy) -> i128 {
    let (lo, hi, m) = match s {
        Sign::Unsigned => (0i128, (1i128 << w) - 1, 1i128 << w),
        Sign::Signed => (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1, 1i128 << w),
    };
    match p {
        Policy::Wrap => {
            let mut r = x % m;
            if r > hi {
                r -= m;
            }
            if r < lo {
                r += m;
            }
            r
        }
        Policy::Saturate => {
            if x > hi {
                hi
            } else if x < lo {
                lo
            } else {
                x
            }
        }
    }
}

#[inline(always)]
fn add(a: i128, b: i128, w: u32, s: Sign, p: Policy) -> i128 {
    clip(a + b, w, s, p)
}

fn seq(xs: &[i128], w: u32, s: Sign, p: Policy) -> i128 {
    let mut acc = 0i128;
    for &x in xs {
        acc = add(acc, x, w, s, p);
    }
    acc
}

fn tree(xs: &[i128], w: u32, s: Sign, p: Policy) -> i128 {
    if xs.len() == 1 {
        return clip(xs[0], w, s, p);
    }
    let mid = xs.len() / 2;
    add(
        tree(&xs[..mid], w, s, p),
        tree(&xs[mid..], w, s, p),
        w,
        s,
        p,
    )
}

// L strided partial sums, then a left-fold horizontal combine. This is the
// shape a vector reduction and a per-core partial-sum reduction both take.
fn lanes(xs: &[i128], l: usize, w: u32, s: Sign, p: Policy) -> i128 {
    let mut part = vec![0i128; l];
    for (i, &x) in xs.iter().enumerate() {
        let k = i % l;
        part[k] = add(part[k], x, w, s, p);
    }
    let mut acc = 0i128;
    for &v in &part {
        acc = add(acc, v, w, s, p);
    }
    acc
}

struct Cell {
    w: u32,
    n: usize,
    sign: &'static str,
    policy: &'static str,
    disagreeing: u64,
    total: u64,
    witness: Option<(Vec<i128>, i128, i128)>,
}

fn sweep(w: u32, n: usize, s: Sign, p: Policy, sn: &'static str, pn: &'static str) -> Cell {
    let (lo, hi) = match s {
        Sign::Unsigned => (0i128, (1i128 << w) - 1),
        Sign::Signed => (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1),
    };
    let card = (hi - lo + 1) as u64;

    let mut total = 0u64;
    let mut dis = 0u64;
    let mut witness = None;

    // Exhaustive over every vector of length n from the value set, by
    // odometer over the cardinality.
    let combos = card.pow(n as u32);
    let mut xs = vec![0i128; n];
    for code in 0..combos {
        let mut c = code;
        for i in 0..n {
            xs[i] = lo + (c % card) as i128;
            c /= card;
        }

        let a = seq(&xs, w, s, p);
        let b = tree(&xs, w, s, p);
        let c2 = lanes(&xs, 2, w, s, p);
        let c4 = lanes(&xs, 4, w, s, p);

        total += 1;
        if a != b || a != c2 || a != c4 {
            dis += 1;
            if witness.is_none() {
                let other = if a != b {
                    b
                } else if a != c2 {
                    c2
                } else {
                    c4
                };
                witness = Some((xs.clone(), a, other));
            }
        }
    }

    Cell {
        w,
        n,
        sign: sn,
        policy: pn,
        disagreeing: dis,
        total,
        witness,
    }
}

fn main() {
    println!("w,n,sign,policy,vectors_with_disagreement,total_vectors,pct");

    // Two configurations, both exhaustive:
    //   w=4, n=4  -> 16^4  =     65,536 vectors  (unsigned) / same (signed)
    //   w=3, n=8  ->  8^8  = 16,777,216 vectors
    let configs: [(u32, usize); 2] = [(4, 4), (3, 8)];

    for (w, n) in configs {
        for (s, sn) in [(Sign::Unsigned, "unsigned"), (Sign::Signed, "signed")] {
            for (p, pn) in [(Policy::Wrap, "wrap"), (Policy::Saturate, "saturate")] {
                let c = sweep(w, n, s, p, sn, pn);
                let pct = if c.total == 0 {
                    0.0
                } else {
                    100.0 * c.disagreeing as f64 / c.total as f64
                };
                println!(
                    "{},{},{},{},{},{},{:.4}",
                    c.w, c.n, c.sign, c.policy, c.disagreeing, c.total, pct
                );
                if let Some((xs, a, b)) = c.witness {
                    eprintln!(
                        "witness w={} n={} {} {}: input {:?} -> sequential {} but a split reduction {}",
                        c.w, c.n, c.sign, c.policy, xs, a, b
                    );
                }
            }
        }
    }

    // Float arm. NOT exhaustive: the f32 triple space is 2^96. This is a
    // deterministic pseudo-random sample with a fixed seed, reported as a
    // sample, present because arvo carries FastFloat and StrictFloat and the
    // fixed-point answer above must not be read as covering them.
    let mut state: u64 = 0x243F_6A88_85A3_08D3;
    let mut rnd = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let n = 8usize;
    let trials = 1_000_000u64;
    let mut dis = 0u64;
    for _ in 0..trials {
        let xs: Vec<f32> = (0..n)
            .map(|_| {
                let r = rnd();
                // Spread across several magnitudes so cancellation is reachable.
                let mant = ((r & 0xFFFF) as f32) / 65535.0 - 0.5;
                let ex = ((r >> 16) % 40) as i32 - 20;
                mant * (2.0f32).powi(ex)
            })
            .collect();
        let mut a = 0.0f32;
        for &x in &xs {
            a = a + x;
        }
        fn ftree(xs: &[f32]) -> f32 {
            if xs.len() == 1 {
                return xs[0];
            }
            let m = xs.len() / 2;
            ftree(&xs[..m]) + ftree(&xs[m..])
        }
        let b = ftree(&xs);
        let mut part = [0.0f32; 4];
        for (i, &x) in xs.iter().enumerate() {
            part[i % 4] += x;
        }
        let c = part[0] + part[1] + part[2] + part[3];
        if a != b || a != c {
            dis += 1;
        }
    }
    println!(
        "f32,{},float,ieee_sample,{},{},{:.4}",
        n,
        dis,
        trials,
        100.0 * dis as f64 / trials as f64
    );
    eprintln!("float arm is a 1e6 pseudo-random sample with a fixed seed, not exhaustive");
}
