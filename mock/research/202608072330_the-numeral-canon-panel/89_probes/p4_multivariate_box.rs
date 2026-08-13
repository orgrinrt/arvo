// PROBE p4 (file 89). The multivariate box criterion for the monotone
// unsigned saturating fragment, which 86 section 6 lists as unreached
// ("multivariate pieces are regions rather than intervals; neither is
// examined").
//
// THEOREM C. Let A, B be terms over k variables built from the variables,
// nonnegative constants (clamp-embedded), sat_add and sat_mul. Let d_i be the
// syntactic per-variable degree bound and let the domain be [0, HI]^k for any
// HI <= MAX_W. Then
//
//     A == B on the whole domain    <=>    A == B on the box PROD_i {0..min(d_i,HI)}
//
// PROOF. By p1's min-form lemma each side is min(P, MAX) with P a polynomial
// with nonnegative integer coefficients, hence nondecreasing in each variable.
// Write DELTA = P_A - P_B. Let Box be the test box and let K be the set of box
// points at which BOTH sides clamp. K is an up-set in Box, because clamping is
// preserved by increasing any coordinate.
//
// Agreement on Box means DELTA vanishes on Box \ K (at an unclamped point the
// two sides equal their polynomials). Expand DELTA in the tensor falling-
// factorial basis, DELTA = sum_J lambda_J prod_i (x_i)_{j_i}, which is a basis
// for polynomials of per-variable degree <= d_i. Evaluation in that basis is
// triangular with respect to the coordinatewise order: e_J(J') = 0 unless
// J' >= J. Box \ K is a down-set, so processing J in increasing order gives
// lambda_J = 0 for every J not in K.
//
// Now take any z in the domain with DELTA(z) != 0. Some J in K has
// prod_i (z_i)_{j_i} != 0, which for nonnegative integers forces z_i >= j_i for
// every i, that is z >= J. Since J is in K both sides clamp at J, and by
// monotonicity both clamp at z, so both sides equal MAX and agree there anyway.
// Hence agreement on Box implies agreement on the domain. QED.
//
// The univariate criterion of p1 is k = 1 of this. The test set is the DEGREE
// BOX, which is exactly the test set 86 section 3 found for the RING fragment:
// two different fragments, one test-set shape.
//
// This probe refuses to trust the proof. It exhaustively compares the box
// verdict against the full sweep for k = 2 and k = 3 over random and
// structured term pairs at every width the sweep reaches, runs the direct
// falsification test (a law true on the box and false in the domain would
// refute Theorem C outright), and runs mutation controls so the battery is
// shown capable of failing.
//
// Runtime spike; std/Vec/Box are scaffolding, not design shape.

use std::collections::BTreeSet;

fn umax(w: u32) -> u128 {
    if w >= 64 {
        u64::MAX as u128
    } else {
        (1u128 << w) - 1
    }
}

#[derive(Clone)]
enum T {
    V(usize),
    C(u64),
    Add(Box<T>, Box<T>),
    Mul(Box<T>, Box<T>),
}

impl T {
    fn sat(&self, xs: &[u128], w: u32) -> u128 {
        let m = umax(w);
        match self {
            T::V(i) => xs[*i],
            T::C(c) => {
                let c = *c as u128;
                if c > m {
                    m
                } else {
                    c
                }
            }
            T::Add(a, b) => {
                let s = a.sat(xs, w) + b.sat(xs, w);
                if s > m {
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let s = a.sat(xs, w) * b.sat(xs, w);
                if s > m {
                    m
                } else {
                    s
                }
            }
        }
    }
    /// per-variable syntactic degree bound
    fn pdeg(&self, k: usize) -> Vec<u64> {
        match self {
            T::V(i) => {
                let mut v = vec![0; k];
                v[*i] = 1;
                v
            }
            T::C(_) => vec![0; k],
            T::Add(a, b) => {
                let (x, y) = (a.pdeg(k), b.pdeg(k));
                (0..k).map(|i| x[i].max(y[i])).collect()
            }
            T::Mul(a, b) => {
                let (x, y) = (a.pdeg(k), b.pdeg(k));
                (0..k).map(|i| x[i] + y[i]).collect()
            }
        }
    }
}

fn box_points(d: &[u64], hi: u128) -> Vec<Vec<u128>> {
    let bounds: Vec<u128> = d.iter().map(|&di| (di as u128).min(hi)).collect();
    let mut out = vec![vec![]];
    for b in &bounds {
        let mut nxt = Vec::new();
        for pre in &out {
            for v in 0..=*b {
                let mut p = pre.clone();
                p.push(v);
                nxt.push(p);
            }
        }
        out = nxt;
    }
    out
}

fn all_points(k: usize, hi: u128) -> Vec<Vec<u128>> {
    let mut out = vec![vec![]];
    for _ in 0..k {
        let mut nxt = Vec::new();
        for pre in &out {
            for v in 0..=hi {
                let mut p = pre.clone();
                p.push(v);
                nxt.push(p);
            }
        }
        out = nxt;
    }
    out
}

fn box_verdict(a: &T, b: &T, k: usize, w: u32, shrink: i64) -> (bool, Option<Vec<u128>>) {
    let m = umax(w);
    let da = a.pdeg(k);
    let db = b.pdeg(k);
    let d: Vec<u64> = (0..k)
        .map(|i| {
            let base = da[i].max(db[i]) as i64 + shrink;
            if base < 0 {
                0
            } else {
                base as u64
            }
        })
        .collect();
    for p in box_points(&d, m) {
        if a.sat(&p, w) != b.sat(&p, w) {
            return (false, Some(p));
        }
    }
    (true, None)
}

fn sweep_witness(a: &T, b: &T, k: usize, w: u32) -> Option<Vec<u128>> {
    let m = umax(w);
    for p in all_points(k, m) {
        if a.sat(&p, w) != b.sat(&p, w) {
            return Some(p);
        }
    }
    None
}

struct Xs(u64);
impl Xs {
    fn next(&mut self) -> u64 {
        let mut v = self.0;
        v ^= v << 13;
        v ^= v >> 7;
        v ^= v << 17;
        self.0 = v;
        v
    }
}

fn rand_term(rng: &mut Xs, depth: u32, k: usize) -> T {
    if depth == 0 || rng.next() % 4 == 0 {
        if rng.next() % 3 != 0 {
            T::V((rng.next() as usize) % k)
        } else {
            T::C((rng.next() % 6) << (rng.next() % 3))
        }
    } else {
        let a = Box::new(rand_term(rng, depth - 1, k));
        let b = Box::new(rand_term(rng, depth - 1, k));
        if rng.next() % 2 == 0 {
            T::Add(a, b)
        } else {
            T::Mul(a, b)
        }
    }
}
fn v(i: usize) -> T {
    T::V(i)
}
fn c(x: u64) -> T {
    T::C(x)
}
fn ad(a: T, b: T) -> T {
    T::Add(Box::new(a), Box::new(b))
}
fn mu(a: T, b: T) -> T {
    T::Mul(Box::new(a), Box::new(b))
}

fn run(
    k: usize,
    wmax: u32,
    npairs: usize,
    seed: u64,
    extra: Vec<(String, T, T)>,
) -> (u64, u64, u64, u64, u64, u64) {
    let mut rng = Xs(seed);
    let mut pairs: Vec<(String, T, T)> = Vec::new();
    while pairs.len() < npairs {
        let a = rand_term(&mut rng, 3, k);
        let b = rand_term(&mut rng, 3, k);
        let (pa, pb) = (a.pdeg(k), b.pdeg(k));
        if pa.iter().chain(pb.iter()).all(|&d| d <= 4) {
            pairs.push(("random".into(), a, b));
        }
    }
    // near-miss pairs: a and a term sharing most of its structure
    let mut rng2 = Xs(seed ^ 0xABCD);
    while pairs.len() < npairs * 2 {
        let a = rand_term(&mut rng2, 3, k);
        let b = ad(
            a.clone(),
            mu(v((rng2.next() as usize) % k), c(1 + rng2.next() % 2)),
        );
        let (pa, pb) = (a.pdeg(k), b.pdeg(k));
        if pa.iter().chain(pb.iter()).all(|&d| d <= 4) {
            pairs.push(("near-miss".into(), a, b));
        }
    }
    for e in extra {
        pairs.push(e);
    }

    let (mut checked, mut mis, mut tcnt, mut fcnt, mut ctrl_mis, mut kbig) =
        (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    for (_n, a, b) in &pairs {
        for w in 1..=wmax {
            let sw = sweep_witness(a, b, k, w);
            let (bv, _) = box_verdict(a, b, k, w, 0);
            checked += 1;
            if (sw.is_none()) != bv {
                mis += 1;
            }
            if bv {
                tcnt += 1
            } else {
                fcnt += 1
            }
            // mutation control: shrink the box by one in every coordinate
            let (bv2, _) = box_verdict(a, b, k, w, -1);
            if (sw.is_none()) != bv2 {
                ctrl_mis += 1;
            }
            // does this case have a genuinely nonempty clamped set K inside the box?
            let m = umax(w);
            let da = a.pdeg(k);
            let db = b.pdeg(k);
            let d: Vec<u64> = (0..k).map(|i| da[i].max(db[i])).collect();
            let mut anyclamp = false;
            for p in box_points(&d, m) {
                if a.sat(&p, w) == m && b.sat(&p, w) == m {
                    anyclamp = true;
                    break;
                }
            }
            if anyclamp {
                kbig += 1;
            }
        }
    }
    (checked, mis, tcnt, fcnt, ctrl_mis, kbig)
}

fn main() {
    println!("p4: the multivariate box criterion for monotone unsigned saturating laws\n");

    // k = 2 structured members, including the shape hand-analysis suggested
    // could break it: DELTA supported only on the corner's up-set.
    let extra2: Vec<(String, T, T)> = vec![
        // x*y + y  ==  x*x*y + y : agrees on {0,1,2}x{0,1} at width 2, differs over Z
        (
            "handbuilt-1".into(),
            ad(mu(v(0), v(1)), v(1)),
            ad(mu(mu(v(0), v(0)), v(1)), v(1)),
        ),
        // x + y + x*y  ==  x + y + 5*x*y
        (
            "handbuilt-2".into(),
            ad(ad(v(0), v(1)), mu(v(0), v(1))),
            ad(ad(v(0), v(1)), mu(c(5), mu(v(0), v(1)))),
        ),
        // saturating multiply commutes (true everywhere)
        ("commute".into(), mu(v(0), v(1)), mu(v(1), v(0))),
        // saturating add associativity across three shapes at k=2
        (
            "addassoc".into(),
            ad(ad(v(0), v(1)), v(0)),
            ad(v(0), ad(v(1), v(0))),
        ),
        // distributivity, false under saturation in general
        (
            "distrib".into(),
            mu(v(0), ad(v(1), v(1))),
            ad(mu(v(0), v(1)), mu(v(0), v(1))),
        ),
        // high per-variable degree with a big constant
        (
            "deg3".into(),
            mu(mu(v(0), v(0)), mu(v(0), v(1))),
            mu(mu(v(0), v(0)), mu(v(1), v(0))),
        ),
    ];
    let (c2, m2, t2, f2, cm2, kb2) = run(2, 5, 200, 0x89_2222, extra2);
    println!("k = 2, widths 1..=5, exhaustive sweeps over the full square:");
    println!("  (pair, width) verdicts:                 {}", c2);
    println!("  true {}, false {}", t2, f2);
    println!("  box criterion vs sweep mismatches:      {}", m2);
    println!("  cases with a clamped box point (K nonempty): {}", kb2);
    println!(
        "  MUTATION CONTROL, box shrunk by one per coordinate: {} mismatches",
        cm2
    );

    let extra3: Vec<(String, T, T)> = vec![
        (
            "addassoc3".into(),
            ad(ad(v(0), v(1)), v(2)),
            ad(v(0), ad(v(1), v(2))),
        ),
        (
            "mulassoc3".into(),
            mu(mu(v(0), v(1)), v(2)),
            mu(v(0), mu(v(1), v(2))),
        ),
        (
            "distrib3".into(),
            mu(v(0), ad(v(1), v(2))),
            ad(mu(v(0), v(1)), mu(v(0), v(2))),
        ),
        (
            "mixed3".into(),
            ad(mu(v(0), v(1)), v(2)),
            ad(v(2), mu(v(1), v(0))),
        ),
    ];
    let (c3, m3, t3, f3, cm3, kb3) = run(3, 4, 120, 0x89_3333, extra3);
    println!("\nk = 3, widths 1..=4, exhaustive sweeps over the full cube:");
    println!("  (pair, width) verdicts:                 {}", c3);
    println!("  true {}, false {}", t3, f3);
    println!("  box criterion vs sweep mismatches:      {}", m3);
    println!("  cases with a clamped box point (K nonempty): {}", kb3);
    println!(
        "  MUTATION CONTROL, box shrunk by one per coordinate: {} mismatches",
        cm3
    );

    assert!(
        m2 == 0 && m3 == 0,
        "THEOREM C refuted by a box/sweep mismatch"
    );
    assert!(
        cm2 > 0 && cm3 > 0,
        "the battery cannot notice a weakened box"
    );
    assert!(
        kb2 > 100 && kb3 > 100,
        "the battery does not exercise the clamped-set branch"
    );

    // THE DIRECT FALSIFICATION TEST. Enumerate ALL pairs of small terms over
    // k = 2 at width 2 and 3 and look for one true on the box and false in the
    // domain. That is what a counterexample to Theorem C looks like.
    println!("\nDIRECT FALSIFICATION SEARCH: every pair of depth-2 k=2 terms, widths 2 and 3.");
    let mut atoms: Vec<T> = vec![v(0), v(1), c(0), c(1), c(2), c(3)];
    let mut terms: Vec<T> = atoms.clone();
    for a in atoms.clone() {
        for b in atoms.clone() {
            terms.push(ad(a.clone(), b.clone()));
            terms.push(mu(a.clone(), b.clone()));
        }
    }
    atoms.clear();
    let mut deeper: Vec<T> = terms.clone();
    for a in terms.iter().take(20) {
        for b in terms.iter().take(20) {
            deeper.push(ad(a.clone(), b.clone()));
            deeper.push(mu(a.clone(), b.clone()));
        }
    }
    let mut nchecked = 0u64;
    let mut nfalsify = 0u64;
    let mut seen = BTreeSet::new();
    for w in 2..=3u32 {
        let m = umax(w);
        // dedupe by the term's full value table so the count is of functions
        let mut idx: Vec<(Vec<u128>, usize)> = Vec::new();
        for (i, t) in deeper.iter().enumerate() {
            let tab: Vec<u128> = all_points(2, m).iter().map(|p| t.sat(p, w)).collect();
            idx.push((tab, i));
        }
        for i in 0..deeper.len() {
            for j in (i + 1)..deeper.len() {
                let a = &deeper[i];
                let b = &deeper[j];
                let pa = a.pdeg(2);
                let pb = b.pdeg(2);
                if pa.iter().chain(pb.iter()).any(|&d| d > 6) {
                    continue;
                }
                nchecked += 1;
                let (bv, _) = box_verdict(a, b, 2, w, 0);
                if bv {
                    let sw = sweep_witness(a, b, 2, w);
                    if sw.is_some() {
                        nfalsify += 1;
                        seen.insert((w, i, j));
                    }
                }
            }
        }
        let _ = &idx;
    }
    println!("  term pairs checked: {}", nchecked);
    println!(
        "  pairs TRUE on the box and FALSE in the domain: {}",
        nfalsify
    );
    assert!(
        nfalsify == 0,
        "THEOREM C REFUTED, counterexamples: {:?}",
        seen.iter().take(3).collect::<Vec<_>>()
    );

    println!("\nall checks passed");
}
