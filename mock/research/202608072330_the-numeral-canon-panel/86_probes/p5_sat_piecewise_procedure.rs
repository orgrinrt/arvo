// PROBE p5. A decision procedure for the MONOTONE UNIVARIATE SATURATING
// fragment, attacking p4's own sentence that "for this family there is no
// procedure row available at all". That sentence is true of the difference
// criterion and turns out to be false of the fragment.
//
// THE OBSERVATION. Saturating arithmetic is not modular. It is exact integer
// arithmetic composed with clamps. For terms built from sat_add, sat_mul,
// nonnegative constants and one variable x, every subterm is a nondecreasing
// function of x, so every clamp node fires on a final segment [b, MAX], and
// its breakpoint b is found by binary search in ~W evaluations. Between
// consecutive breakpoints the set of clamped nodes is constant, so each side
// of a law is an honest integer polynomial of degree <= D (the syntactic
// degree bound) on that piece. Two integer polynomials of degree <= D that
// agree at D+1 distinct points are identical over Q and hence agree on the
// whole piece. Therefore:
//
//   verdict at width W = for each piece: sweep it if it has <= D+1 points,
//   else compare D+1 consecutive points inside it.
//
// Sound and complete for the fragment, at ANY width including 64, in
// O(C*W + (C+1)*(D+1)) evaluations, C = clamp-node count, D = degree bound.
// The two trusted inputs mirror the ring procedure's: fragment membership
// (monotone ops, nonnegative constants, one variable) and the degree bound.
//
// This probe validates the procedure against exhaustive sweeps over a random
// battery of monotone term pairs at widths 1..=12, then runs it at width 64
// on the p3 family: E_63 (false, threshold 63), E_64 (true at 64, false at
// 65), and the C-member (false at 64), with evaluation counts printed.
//
// Toolchain: pinned nightly-2026-05-28. Runtime probe; std/Vec are spike
// scaffolding per the panel's probe discipline, not design shape.

fn umax(w: u32) -> u128 {
    if w >= 64 {
        u64::MAX as u128
    } else {
        (1u128 << w) - 1
    }
}

#[derive(Clone)]
enum T {
    X,
    C(u64),
    Add(Box<T>, Box<T>),
    Mul(Box<T>, Box<T>),
}

impl T {
    /// Exact saturating evaluation at width w; also reports, per clamp node
    /// visited, whether it clamped (pre-clamp value exceeded MAX).
    fn eval(&self, x: u128, w: u32, evals: &mut u64) -> u128 {
        let m = umax(w);
        match self {
            T::X => x,
            T::C(c) => {
                // clamp-embedded constant: min(c, MAX). The convention-
                // dependence of wrap-embedding is p3's subject, not this one.
                let c = *c as u128;
                if c > m {
                    m
                } else {
                    c
                }
            }
            T::Add(a, b) => {
                let (va, vb) = (a.eval(x, w, evals), b.eval(x, w, evals));
                *evals += 1;
                let s = va + vb;
                if s > m {
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let (va, vb) = (a.eval(x, w, evals), b.eval(x, w, evals));
                *evals += 1;
                let s = va * vb; // both <= m <= 2^64-1, product < 2^128
                if s > m {
                    m
                } else {
                    s
                }
            }
        }
    }

    /// Does any node of this term clamp at input x, width w?
    /// Returns the set of clamp events as a bitmask over node indices
    /// (preorder), so the piece structure can be keyed on it.
    fn clamp_mask(&self, x: u128, w: u32, idx: &mut u32, mask: &mut u128) -> u128 {
        let m = umax(w);
        match self {
            T::X => x,
            T::C(c) => {
                let c = *c as u128;
                if c > m {
                    m
                } else {
                    c
                }
            }
            T::Add(a, b) => {
                let va = a.clamp_mask(x, w, idx, mask);
                let vb = b.clamp_mask(x, w, idx, mask);
                let my = *idx;
                *idx += 1;
                let s = va + vb;
                if s > m {
                    *mask |= 1u128 << my;
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let va = a.clamp_mask(x, w, idx, mask);
                let vb = b.clamp_mask(x, w, idx, mask);
                let my = *idx;
                *idx += 1;
                let s = va * vb;
                if s > m {
                    *mask |= 1u128 << my;
                    m
                } else {
                    s
                }
            }
        }
    }

    fn n_ops(&self) -> u32 {
        match self {
            T::X | T::C(_) => 0,
            T::Add(a, b) | T::Mul(a, b) => 1 + a.n_ops() + b.n_ops(),
        }
    }

    /// Syntactic degree bound, ignoring clamps (clamping only lowers degree).
    fn degree(&self) -> u64 {
        match self {
            T::X => 1,
            T::C(_) => 0,
            T::Add(a, b) => a.degree().max(b.degree()),
            T::Mul(a, b) => a.degree() + b.degree(),
        }
    }
}

fn mask_at(t: &T, x: u128, w: u32) -> u128 {
    let mut idx = 0;
    let mut mask = 0;
    t.clamp_mask(x, w, &mut idx, &mut mask);
    mask
}

/// First x in [lo, hi] where the pair's combined clamp mask differs from its
/// value at lo, by binary search per clamp node. Monotonicity: each node's
/// clamp indicator is upward-closed in x for the monotone fragment.
fn breakpoints(a: &T, b: &T, w: u32, evals: &mut u64) -> Vec<u128> {
    let m = umax(w);
    let mut bps: Vec<u128> = Vec::new();
    // per clamp node of each term, binary-search its first-clamp position
    for (term, other_ops) in [(a, 0u32), (b, a.n_ops())] {
        let n = term.n_ops();
        for node in 0..n {
            let bit = 1u128 << node;
            let clamps_at = |x: u128, evals: &mut u64| -> bool {
                *evals += term.n_ops() as u64; // count the evaluation work
                mask_at(term, x, w) & bit != 0
            };
            if !clamps_at(m, evals) {
                continue; // never clamps at this width
            }
            if clamps_at(0, evals) {
                continue; // clamps everywhere; no interior breakpoint
            }
            let (mut lo, mut hi) = (0u128, m); // lo: not clamped, hi: clamped
            while hi - lo > 1 {
                let mid = lo + (hi - lo) / 2;
                if clamps_at(mid, evals) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            bps.push(hi);
        }
        let _ = other_ops;
    }
    bps.sort_unstable();
    bps.dedup();
    bps
}

/// The piecewise verdict: sound and complete for the monotone fragment.
/// Returns (verdict, witness if false, evaluations spent).
fn piecewise_verdict(a: &T, b: &T, w: u32) -> (bool, Option<u128>, u64) {
    let m = umax(w);
    let mut evals = 0u64;
    let d = (a.degree().max(b.degree())) as u128;
    let bps = breakpoints(a, b, w, &mut evals);
    // piece boundaries: 0, each breakpoint, m+1
    let mut cuts: Vec<u128> = vec![0];
    cuts.extend(bps.iter().copied());
    cuts.push(m + 1);
    cuts.dedup();
    for win in cuts.windows(2) {
        let (lo, hi) = (win[0], win[1]); // piece [lo, hi)
        let len = hi - lo;
        let probe_n = if len <= d + 1 { len } else { d + 1 };
        for i in 0..probe_n {
            let x = lo + i;
            let va = a.eval(x, w, &mut evals);
            let vb = b.eval(x, w, &mut evals);
            if va != vb {
                return (false, Some(x), evals);
            }
        }
    }
    (true, None, evals)
}

/// Exhaustive sweep for validation at small widths.
fn sweep_verdict(a: &T, b: &T, w: u32) -> bool {
    let m = umax(w);
    let mut evals = 0u64;
    let mut x = 0u128;
    while x <= m {
        if a.eval(x, w, &mut evals) != b.eval(x, w, &mut evals) {
            return false;
        }
        x += 1;
    }
    true
}

struct Xorshift(u64);
impl Xorshift {
    fn next(&mut self) -> u64 {
        let mut v = self.0;
        v ^= v << 13;
        v ^= v >> 7;
        v ^= v << 17;
        self.0 = v;
        v
    }
}

fn random_term(rng: &mut Xorshift, depth: u32) -> T {
    if depth == 0 || rng.next() % 4 == 0 {
        if rng.next() % 2 == 0 {
            T::X
        } else {
            T::C((rng.next() % 8) << (rng.next() % 4))
        }
    } else {
        let a = Box::new(random_term(rng, depth - 1));
        let b = Box::new(random_term(rng, depth - 1));
        if rng.next() % 2 == 0 {
            T::Add(a, b)
        } else {
            T::Mul(a, b)
        }
    }
}

/// x^d as a left-fold sat_mul chain.
fn pow_term(d: u32) -> T {
    let mut t = T::X;
    for _ in 1..d {
        t = T::Mul(Box::new(t), Box::new(T::X));
    }
    t
}

fn main() {
    println!("p5: the piecewise decision procedure for monotone univariate saturating laws\n");

    // ---------------- validation battery ----------------
    let mut rng = Xorshift(0x5555_8686_5555);
    let mut pairs: Vec<(T, T)> = Vec::new();
    while pairs.len() < 300 {
        let a = random_term(&mut rng, 3);
        let b = random_term(&mut rng, 3);
        // keep degree sane so D+1 sampling stays cheap
        if a.degree() <= 16 && b.degree() <= 16 && (a.n_ops() + b.n_ops()) > 0 {
            pairs.push((a, b));
        }
    }
    // structured members: E_d for d = 2..=10, and near-miss pairs
    for d in 2..=10u32 {
        pairs.push((pow_term(d), pow_term(d + 1)));
    }

    let wmax = 12u32;
    let (mut checked, mut mismatch) = (0u64, 0u64);
    let mut true_cnt = 0u64;
    let mut false_cnt = 0u64;
    let mut witness_bad = 0u64;
    for (a, b) in &pairs {
        for w in 1..=wmax {
            let s = sweep_verdict(a, b, w);
            let (p, wit, _) = piecewise_verdict(a, b, w);
            checked += 1;
            if s != p {
                mismatch += 1;
            }
            if p {
                true_cnt += 1;
            } else {
                false_cnt += 1;
            }
            // a returned witness must actually witness
            if let Some(x) = wit {
                let mut e = 0u64;
                if a.eval(x, w, &mut e) == b.eval(x, w, &mut e) {
                    witness_bad += 1;
                }
            }
        }
    }
    println!(
        "battery: {} monotone term pairs x widths 1..={}",
        pairs.len(),
        wmax
    );
    println!(
        "  (pair, width) verdicts checked against sweeps: {}",
        checked
    );
    println!(
        "  true verdicts: {}, false verdicts: {}",
        true_cnt, false_cnt
    );
    println!(
        "  procedure vs sweep mismatches:                 {}",
        mismatch
    );
    println!(
        "  returned witnesses that fail to witness:       {}",
        witness_bad
    );
    assert!(mismatch == 0 && witness_bad == 0);
    assert!(true_cnt > 200 && false_cnt > 200, "battery too one-sided");

    // ---------------- the p3 family at the shipped width ----------------
    println!("\nwidth-64 verdicts by the piecewise procedure (no sweep, no band, no transfer):");
    let e63 = (pow_term(63), pow_term(64));
    let (v, wit, ev) = piecewise_verdict(&e63.0, &e63.1, 64);
    println!(
        "  E_63 (x^63 == x^64):  {}  witness {:?}  in {} evaluations",
        v, wit, ev
    );
    assert!(!v && wit == Some(2));
    let e64 = (pow_term(64), pow_term(65));
    let (v, wit, ev) = piecewise_verdict(&e64.0, &e64.1, 64);
    println!(
        "  E_64 (x^64 == x^65):  {}  witness {:?}  in {} evaluations",
        v, wit, ev
    );
    assert!(v);
    // E_64 at width 65 is outside u64 carriers; the family's own argument
    // (p3) places its falsity there and nothing here claims it.
    let c: u64 = (1u64 << 63) - 1;
    let cm = (
        T::Mul(Box::new(T::X), Box::new(T::C(c))),
        T::Mul(
            Box::new(T::X),
            Box::new(T::Add(Box::new(T::C(c)), Box::new(T::C(1)))),
        ),
    );
    let (v, wit, ev) = piecewise_verdict(&cm.0, &cm.1, 64);
    println!(
        "  C-member at 64:       {}  witness {:?}  in {} evaluations",
        v, wit, ev
    );
    assert!(!v && wit == Some(1));
    let (v, _, ev) = piecewise_verdict(&cm.0, &cm.1, 40);
    println!(
        "  C-member at 40:       {}  (both constants embed to MAX)  in {} evaluations",
        v, ev
    );
    assert!(v);

    println!("\nall checks passed");
}
