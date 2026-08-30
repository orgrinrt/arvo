// PROBE p8 (file 89). The MECHANISM behind 86's F6, which 86 reported as a
// measured fact without one: signed saturating equation truth sets are gapped
// and interior-run, so neither band direction transfers.
//
// THE MECHANISM. Every criterion in this file's family (p1 Theorem B, p4
// Theorem C, and 86's piecewise procedure) rests on one structural fact:
//
//     saturating evaluation = clamp(exact polynomial value)
//
// that is, clamping early and clamping late coincide, so a term is a single
// polynomial seen through one clamp. That is TRUE for the monotone nonnegative
// fragment (p1 Theorem A, measured over 23.9M evaluations) and it is FALSE for
// signed terms with both clamps reachable. The minimal counterexample is one
// line of arithmetic: at width 4, MAX = 7, MIN = -8,
//
//     sat_add(sat_add(7, 7), -7)   clamp-early = 7 + (-7) = 0
//     clamp(7 + 7 - 7)             clamp-late  = 7
//
// The ceiling clamp DISCARDS magnitude that a later negative operand would
// have restored. Once that happens the term is not a clamped polynomial, the
// falling-factorial interpolation argument has nothing to interpolate, and the
// truth set in width can do anything, which is exactly what 86 measured.
//
// This probe does three things. It measures how often the min-form property
// fails across the signed term space and where. It checks that every one of
// 86's non-monotone truth-set members is outside the min-form class, so the
// pathology and the fragment boundary coincide rather than merely coexist. And
// it reports what fraction of the signed term space the criterion reaches, per
// declared window, which is the honest answer to "how far does this go".
//
// Runtime spike; std/Vec/Box are scaffolding, not design shape.

fn smin(w: u32) -> i128 {
    -(1i128 << (w - 1))
}
fn smax(w: u32) -> i128 {
    (1i128 << (w - 1)) - 1
}

#[derive(Clone, PartialEq)]
enum S {
    V(usize),
    K(i64),
    Add(Box<S>, Box<S>),
    Mul(Box<S>, Box<S>),
}

impl S {
    /// saturating: clamp at every node
    fn sat(&self, xs: &[i128], w: u32) -> i128 {
        let (lo, hi) = (smin(w), smax(w));
        let cl = |v: i128| {
            if v < lo {
                lo
            } else if v > hi {
                hi
            } else {
                v
            }
        };
        match self {
            S::V(i) => xs[*i],
            S::K(c) => cl(*c as i128),
            S::Add(a, b) => cl(a.sat(xs, w) + b.sat(xs, w)),
            S::Mul(a, b) => cl(a.sat(xs, w) * b.sat(xs, w)),
        }
    }
    /// exact over Z (leaves clamped only at the root), with a huge guard
    fn exact(&self, xs: &[i128], w: u32) -> i128 {
        const H: i128 = i128::MAX / 4;
        let (lo, hi) = (smin(w), smax(w));
        match self {
            S::V(i) => xs[*i],
            S::K(c) => {
                let c = *c as i128;
                if c < lo {
                    lo
                } else if c > hi {
                    hi
                } else {
                    c
                }
            }
            S::Add(a, b) => {
                let v = a.exact(xs, w) + b.exact(xs, w);
                if v > H {
                    H
                } else if v < -H {
                    -H
                } else {
                    v
                }
            }
            S::Mul(a, b) => {
                let (u, v) = (a.exact(xs, w), b.exact(xs, w));
                if u == 0 || v == 0 {
                    0
                } else if u.saturating_mul(v).abs() > H {
                    if (u < 0) != (v < 0) {
                        -H
                    } else {
                        H
                    }
                } else {
                    u * v
                }
            }
        }
    }
    fn depth(&self) -> u32 {
        match self {
            S::V(_) | S::K(_) => 0,
            S::Add(a, b) | S::Mul(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
    fn show(&self) -> String {
        match self {
            S::V(i) => (if *i == 0 { "x" } else { "y" }).into(),
            S::K(c) => format!("{}", c),
            S::Add(a, b) => format!("({}+{})", a.show(), b.show()),
            S::Mul(a, b) => format!("({}*{})", a.show(), b.show()),
        }
    }
}

fn ad(a: S, b: S) -> S {
    S::Add(Box::new(a), Box::new(b))
}
fn mu(a: S, b: S) -> S {
    S::Mul(Box::new(a), Box::new(b))
}

/// does clamp-early equal clamp-late over the whole domain at width w?
fn min_form_holds(t: &S, w: u32, lo: i128, hi: i128, k: usize) -> bool {
    let (cl_lo, cl_hi) = (smin(w), smax(w));
    let cl = |v: i128| {
        if v < cl_lo {
            cl_lo
        } else if v > cl_hi {
            cl_hi
        } else {
            v
        }
    };
    let mut pts = vec![vec![]];
    for _ in 0..k {
        let mut nxt = Vec::new();
        for pre in &pts {
            let mut v = lo;
            while v <= hi {
                let mut p = pre.clone();
                p.push(v);
                nxt.push(p);
                v += 1;
            }
        }
        pts = nxt;
    }
    for p in pts {
        if t.sat(&p, w) != cl(t.exact(&p, w)) {
            return false;
        }
    }
    true
}

fn truth_set(a: &S, b: &S, k: usize, wmax: u32) -> Vec<bool> {
    (1..=wmax)
        .map(|w| {
            let (lo, hi) = (smin(w), smax(w));
            let mut pts = vec![vec![]];
            for _ in 0..k {
                let mut nxt = Vec::new();
                for pre in &pts {
                    let mut v = lo;
                    while v <= hi {
                        let mut p = pre.clone();
                        p.push(v);
                        nxt.push(p);
                        v += 1;
                    }
                }
                pts = nxt;
            }
            pts.iter().all(|p| a.sat(p, w) == b.sat(p, w))
        })
        .collect()
}

/// initial segment, full or empty
fn is_monotone_shape(t: &[bool]) -> bool {
    let mut seen_false = false;
    for &b in t {
        if seen_false && b {
            return false;
        }
        if !b {
            seen_false = true;
        }
    }
    true
}

fn main() {
    println!(
        "p8: why signed saturating truth sets are gapped, and how far the criterion reaches\n"
    );

    // ---- 1. the minimal min-form failure, printed as arithmetic ----
    let w = 4u32;
    let t = ad(ad(S::K(7), S::K(7)), S::K(-7));
    let e = t.exact(&[0], w);
    let s = t.sat(&[0], w);
    println!(
        "minimal min-form failure at width {} (MAX = {}, MIN = {}):",
        w,
        smax(w),
        smin(w)
    );
    println!("  term {}", t.show());
    println!("  clamp at every node (saturating):        {}", s);
    println!("  exact over Z then clamp once at the root: {}", e);
    println!(
        "  the ceiling clamp discarded magnitude a later negative operand would have restored\n"
    );
    assert!(
        s != e,
        "the minimal counterexample stopped counterexampling"
    );

    // ---- 2. the term space: how often does min-form hold ----
    let atoms: Vec<S> = vec![S::V(0), S::V(1)];
    let mut d1: Vec<S> = atoms.clone();
    for a in &atoms {
        for b in &atoms {
            d1.push(ad(a.clone(), b.clone()));
            d1.push(mu(a.clone(), b.clone()));
        }
    }
    let mut d2: Vec<S> = d1.clone();
    for a in &d1 {
        for b in &d1 {
            d2.push(ad(a.clone(), b.clone()));
            d2.push(mu(a.clone(), b.clone()));
        }
    }
    // dedupe by value table at width 4 over the full signed square
    let mut seen: Vec<Vec<i128>> = Vec::new();
    let mut terms: Vec<S> = Vec::new();
    for t in &d2 {
        let (lo, hi) = (smin(4), smax(4));
        let mut tab = Vec::new();
        let mut x = lo;
        while x <= hi {
            let mut y = lo;
            while y <= hi {
                tab.push(t.sat(&[x, y], 4));
                y += 1;
            }
            x += 1;
        }
        if !seen.contains(&tab) {
            seen.push(tab);
            terms.push(t.clone());
        }
    }
    println!(
        "term space: constant-free, two variables, depth <= 2, deduped by value table: {} terms",
        terms.len()
    );
    println!(
        "{:>28} {:>12} {:>12} {:>12}",
        "domain", "width", "min-form holds", "fails"
    );
    for (label, mkwin) in [
        ("full signed [MIN, MAX]", 0usize),
        ("non-negative [0, MAX]", 1),
        ("non-positive [MIN, 0]", 2),
    ] {
        for w in [3u32, 4, 5] {
            let (lo, hi) = match mkwin {
                1 => (0, smax(w)),
                2 => (smin(w), 0),
                _ => (smin(w), smax(w)),
            };
            let (mut ok, mut bad) = (0u64, 0u64);
            for t in &terms {
                if min_form_holds(t, w, lo, hi, 2) {
                    ok += 1
                } else {
                    bad += 1
                }
            }
            println!("{:>28} {:>12} {:>12} {:>12}", label, w, ok, bad);
        }
    }

    // ---- 3. do the non-monotone truth sets live outside the min-form class? ----
    println!("\n86's F6 shape catalogue, re-derived, with the min-form class marked:");
    let wmax = 8u32;
    let (mut npairs, mut nnonmono, mut nonmono_with_minform) = (0u64, 0u64, 0u64);
    let mut examples: Vec<(String, String, String)> = Vec::new();
    for i in 0..terms.len() {
        for j in (i + 1)..terms.len() {
            npairs += 1;
            let ts = truth_set(&terms[i], &terms[j], 2, wmax);
            if !is_monotone_shape(&ts) {
                nnonmono += 1;
                // is either side in the min-form class on the full signed domain
                // at every width the shape was measured at?
                let both_minform = (1..=wmax).all(|w| {
                    let (lo, hi) = (smin(w), smax(w));
                    min_form_holds(&terms[i], w, lo, hi, 2)
                        && min_form_holds(&terms[j], w, lo, hi, 2)
                });
                if both_minform {
                    nonmono_with_minform += 1;
                }
                if examples.len() < 4 {
                    let shape: String = ts.iter().map(|&b| if b { 'T' } else { 'f' }).collect();
                    examples.push((terms[i].show(), terms[j].show(), shape));
                }
            }
        }
    }
    println!(
        "  pairs classified:                                   {}",
        npairs
    );
    println!(
        "  pairs with a NON-monotone truth set over widths 1..={}:  {}",
        wmax, nnonmono
    );
    println!(
        "  of those, pairs where BOTH sides are min-form at every width: {}",
        nonmono_with_minform
    );
    for (a, b, s) in &examples {
        println!("    {}  ==  {}   truth set {}", a, b, s);
    }
    assert!(
        nnonmono > 0,
        "the catalogue found no pathology, so it is not reproducing 86's F6"
    );
    assert!(
        nonmono_with_minform == 0,
        "a non-monotone truth set inside the min-form class would refute the stated mechanism"
    );

    println!("\n  Zero. Every non-monotone truth set in this space has at least one side");
    println!("  outside the min-form class at some width, so the pathology and the");
    println!("  fragment boundary coincide rather than merely coexist.");
    println!("\nall checks passed");
}
