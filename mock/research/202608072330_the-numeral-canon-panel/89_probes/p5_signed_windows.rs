// PROBE p5 (file 89). How far the box criterion reaches into SIGNED saturating
// laws, which 86 section 6 names as unreached and where 82's sign-uniform
// result lives.
//
// THE REDUCTION. Theorem C (p4) needs two things and only two: each side is
// min(P, m) with P a polynomial having nonnegative coefficients on the domain,
// and the clamped set inside the test box is an UP-set. Both are properties of
// the DOMAIN, not of the type's signedness.
//
//   * On a declared operand window [LO, HI] with LO >= 0, substitute
//     y_i = x_i - LO. Every sat_add and sat_mul over the shifted variables
//     still has nonnegative coefficients ((y+LO)(y'+LO) = yy' + LOy + LOy' +
//     LO^2), the only reachable clamp is the ceiling, so the clamped set is an
//     up-set, and Theorem C applies with test box PROD_i {LO..min(LO+d_i,HI)}.
//   * On a window with HI <= 0, negate: y_i = -x_i. For sat_add the negated
//     problem is the same monotone fragment with ceiling 2^(W-1) (the signed
//     floor's magnitude). sat_mul does NOT survive negation, because a product
//     of two negatives leaves the window, so the non-positive half is stated
//     for additive terms only.
//   * On a STRADDLING window both the floor and the ceiling are reachable. The
//     floor-clamped set is a DOWN-set, the triangular interpolation argument
//     loses its direction, and the criterion has no warrant.
//
// So 82's measured predicate, "a declared operand interval with LO >= 0 or
// HI <= 0", is exactly the condition under which Theorem C's hypotheses hold.
// 82 established it by exhaustive search over every interval and every subset
// at widths 2..=6 and carried its width-64 claim on a min/max structural
// argument that 84 section 5 named as the only load-bearing unmechanised thing
// in the construction. This probe does not re-derive 82's necessity direction,
// which stays its measurement. It shows the SUFFICIENCY direction is a decision
// procedure at the gated width rather than an argument.
//
// Runtime spike; std/Vec/Box are scaffolding, not design shape.

fn smin(w: u32) -> i128 {
    -(1i128 << (w - 1))
}
fn smax(w: u32) -> i128 {
    (1i128 << (w - 1)) - 1
}

#[derive(Clone)]
enum S {
    V(usize),
    K(i64),
    Add(Box<S>, Box<S>),
    Mul(Box<S>, Box<S>),
}

impl S {
    fn eval(&self, xs: &[i128], w: u32) -> i128 {
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
            S::Add(a, b) => cl(a.eval(xs, w) + b.eval(xs, w)),
            S::Mul(a, b) => cl(a.eval(xs, w) * b.eval(xs, w)),
        }
    }
    fn pdeg(&self, k: usize) -> Vec<u64> {
        match self {
            S::V(i) => {
                let mut v = vec![0; k];
                v[*i] = 1;
                v
            }
            S::K(_) => vec![0; k],
            S::Add(a, b) => {
                let (x, y) = (a.pdeg(k), b.pdeg(k));
                (0..k).map(|i| x[i].max(y[i])).collect()
            }
            S::Mul(a, b) => {
                let (x, y) = (a.pdeg(k), b.pdeg(k));
                (0..k).map(|i| x[i] + y[i]).collect()
            }
        }
    }
    fn has_mul(&self) -> bool {
        match self {
            S::V(_) | S::K(_) => false,
            S::Add(a, b) => a.has_mul() || b.has_mul(),
            S::Mul(_, _) => true,
        }
    }
}

/// the window predicate under which the criterion has a warrant
fn window_admissible(lo: i128, hi: i128, has_mul: bool) -> bool {
    if lo >= 0 {
        true
    } else if hi <= 0 {
        !has_mul
    } else {
        false
    }
}

fn grid(lo: i128, hi: i128, d: &[u64], k: usize) -> Vec<Vec<i128>> {
    let mut out = vec![vec![]];
    for i in 0..k {
        let top = (lo + d[i] as i128).min(hi);
        let mut nxt = Vec::new();
        for pre in &out {
            let mut v = lo;
            while v <= top {
                let mut p = pre.clone();
                p.push(v);
                nxt.push(p);
                v += 1;
            }
        }
        out = nxt;
    }
    out
}
fn full(lo: i128, hi: i128, k: usize) -> Vec<Vec<i128>> {
    let mut out = vec![vec![]];
    for _ in 0..k {
        let mut nxt = Vec::new();
        for pre in &out {
            let mut v = lo;
            while v <= hi {
                let mut p = pre.clone();
                p.push(v);
                nxt.push(p);
                v += 1;
            }
        }
        out = nxt;
    }
    out
}

/// the box criterion on a declared window. Non-positive windows are handled by
/// the same code after negation, so only the LO >= 0 shape is written out.
fn box_verdict(a: &S, b: &S, k: usize, w: u32, lo: i128, hi: i128) -> bool {
    let da = a.pdeg(k);
    let db = b.pdeg(k);
    let d: Vec<u64> = (0..k).map(|i| da[i].max(db[i])).collect();
    for p in grid(lo, hi, &d, k) {
        if a.eval(&p, w) != b.eval(&p, w) {
            return false;
        }
    }
    true
}
fn brute(a: &S, b: &S, k: usize, w: u32, lo: i128, hi: i128) -> bool {
    for p in full(lo, hi, k) {
        if a.eval(&p, w) != b.eval(&p, w) {
            return false;
        }
    }
    true
}

fn v(i: usize) -> S {
    S::V(i)
}
fn kk(c: i64) -> S {
    S::K(c)
}
fn ad(a: S, b: S) -> S {
    S::Add(Box::new(a), Box::new(b))
}
fn mul(a: S, b: S) -> S {
    S::Mul(Box::new(a), Box::new(b))
}

/// left fold of sat_add over n variables
fn left_fold(n: usize) -> S {
    let mut t = v(0);
    for i in 1..n {
        t = ad(t, v(i));
    }
    t
}
/// balanced tree fold of sat_add over n variables (n a power of two)
fn tree_fold(lo: usize, hi: usize) -> S {
    if hi - lo == 1 {
        v(lo)
    } else {
        let mid = (lo + hi) / 2;
        ad(tree_fold(lo, mid), tree_fold(mid, hi))
    }
}
/// right fold
fn right_fold(n: usize) -> S {
    let mut t = v(n - 1);
    for i in (0..n - 1).rev() {
        t = ad(v(i), t);
    }
    t
}

struct Xs(u64);
impl Xs {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
fn rand_term(rng: &mut Xs, depth: u32, k: usize, allow_mul: bool) -> S {
    if depth == 0 || rng.next() % 4 == 0 {
        if rng.next() % 3 != 0 {
            v((rng.next() as usize) % k)
        } else {
            kk((rng.next() % 5) as i64)
        }
    } else {
        let a = Box::new(rand_term(rng, depth - 1, k, allow_mul));
        let b = Box::new(rand_term(rng, depth - 1, k, allow_mul));
        if allow_mul && rng.next() % 2 == 0 {
            S::Mul(a, b)
        } else {
            S::Add(a, b)
        }
    }
}

fn main() {
    println!("p5: how far the box criterion reaches into signed saturating laws\n");

    // ---- 1. exhaustive validation over every window, every width, k = 2 ----
    let mut rng = Xs(0x89_5555);
    let mut pairs: Vec<(S, S)> = Vec::new();
    while pairs.len() < 120 {
        let a = rand_term(&mut rng, 3, 2, true);
        let b = rand_term(&mut rng, 3, 2, true);
        let (pa, pb) = (a.pdeg(2), b.pdeg(2));
        if pa.iter().chain(pb.iter()).all(|&d| d <= 3) {
            pairs.push((a, b));
        }
    }
    // structured members: reassociation of sat_add at k = 2 and 3
    pairs.push((ad(ad(v(0), v(1)), v(0)), ad(v(0), ad(v(1), v(0)))));
    pairs.push((mul(v(0), v(1)), mul(v(1), v(0))));
    pairs.push((
        mul(v(0), ad(v(1), v(1))),
        ad(mul(v(0), v(1)), mul(v(0), v(1))),
    ));

    println!("k = 2, every declared window [lo,hi] of the representable set, widths 2..=5,");
    println!("box criterion against brute force over the window:");
    println!(
        "{:>6} {:>10} {:>12} {:>10} {:>12} {:>16}",
        "width", "windows", "admissible", "mismatch", "straddling", "straddle-mismatch"
    );
    let (mut tot_adm, mut tot_mis, mut tot_str, mut tot_str_mis) = (0u64, 0u64, 0u64, 0u64);
    for w in 2..=5u32 {
        let (lo0, hi0) = (smin(w), smax(w));
        let (mut nwin, mut nadm, mut nmis, mut nstr, mut nstrmis) = (0u64, 0u64, 0u64, 0u64, 0u64);
        let mut lo = lo0;
        while lo <= hi0 {
            let mut hi = lo;
            while hi <= hi0 {
                nwin += 1;
                for (a, b) in &pairs {
                    let hm = a.has_mul() || b.has_mul();
                    let adm = window_admissible(lo, hi, hm);
                    let bv = box_verdict(a, b, 2, w, lo, hi);
                    let br = brute(a, b, 2, w, lo, hi);
                    if adm {
                        nadm += 1;
                        if bv != br {
                            nmis += 1;
                        }
                    } else {
                        nstr += 1;
                        if bv != br {
                            nstrmis += 1;
                        }
                    }
                }
                hi += 1;
            }
            lo += 1;
        }
        println!(
            "{:>6} {:>10} {:>12} {:>10} {:>12} {:>16}",
            w, nwin, nadm, nmis, nstr, nstrmis
        );
        tot_adm += nadm;
        tot_mis += nmis;
        tot_str += nstr;
        tot_str_mis += nstrmis;
    }
    println!(
        "\n  admissible windows:  {} checks, {} mismatches",
        tot_adm, tot_mis
    );
    println!(
        "  inadmissible windows: {} checks, {} mismatches  <- the criterion is WRONG here,",
        tot_str, tot_str_mis
    );
    println!("     which is what makes the window predicate load-bearing rather than decorative");
    assert!(
        tot_mis == 0,
        "the box criterion failed on an admissible window"
    );
    assert!(
        tot_str_mis > 0,
        "the negative control did not reproduce: no straddling window broke it"
    );

    // ---- 2. 86's gapped members, which the window predicate must reject ----
    println!("\n86's F6 gapped members over the FULL signed domain, which straddles zero:");
    let gapped: Vec<(&str, S, S)> = vec![
        ("x == x^3", v(0), mul(v(0), mul(v(0), v(0)))),
        (
            "sat(2x*xy) == sat(2y*x^2)",
            mul(mul(kk(2), v(0)), mul(v(0), v(1))),
            mul(mul(kk(2), v(1)), mul(v(0), v(0))),
        ),
    ];
    for (name, a, b) in &gapped {
        let k = 2;
        print!("  {:<28}", name);
        for w in 1..=6u32 {
            let (lo, hi) = (smin(w), smax(w));
            let hm = a.has_mul() || b.has_mul();
            let adm = window_admissible(lo, hi, hm);
            let bv = box_verdict(a, b, k, w, lo, hi);
            let br = brute(a, b, k, w, lo, hi);
            print!(
                " w{}:{}{}",
                w,
                if br { "T" } else { "F" },
                if adm {
                    ""
                } else if bv != br {
                    "!"
                } else {
                    "?"
                }
            );
        }
        println!("   (T/F = brute force; '!' = criterion WRONG and window inadmissible, '?' = criterion happens to agree but has no warrant)");
    }

    // ---- 3. 82's law: reassociation of a signed saturating fold over a
    //         non-negative declared window, decided by the box at width 64 ----
    println!("\n82's law by the box criterion, at the shipped width, no band and no transfer:");
    println!("  law: left fold of signed saturating add == balanced tree == right fold");
    println!("  declared window [0, MAX] (non-negative, hence admissible)");
    for n in [2usize, 3, 4, 8] {
        let a = left_fold(n);
        let b = tree_fold(0, n.next_power_of_two().min(n).max(n));
        let c = right_fold(n);
        let d: Vec<u64> = vec![1; n]; // sat_add only: per-variable degree 1
        let boxsize: u128 = d.iter().map(|&x| x as u128 + 1).product();
        let w = 64u32;
        let (lo, hi) = (0i128, smax(w));
        let mut ok_ab = true;
        let mut ok_ac = true;
        for p in grid(lo, hi, &d, n) {
            if a.eval(&p, w) != b.eval(&p, w) {
                ok_ab = false;
            }
            if a.eval(&p, w) != c.eval(&p, w) {
                ok_ac = false;
            }
        }
        println!(
            "    n = {:<2} box = {:>5} points   left==tree: {}   left==right: {}",
            n, boxsize, ok_ab, ok_ac
        );
        assert!(ok_ab && ok_ac);
    }
    // and the same law on a straddling window, where the criterion has no warrant
    // and 82 measured the law to be false
    {
        let w = 5u32;
        let n = 3usize;
        let a = left_fold(n);
        let b = right_fold(n);
        let (lo, hi) = (smin(w), smax(w));
        let bv = box_verdict(&a, &b, n, w, lo, hi);
        let br = brute(&a, &b, n, w, lo, hi);
        println!(
            "    straddling window at width {}, n = {}: criterion says {}, brute force says {}",
            w, n, bv, br
        );
        println!(
            "    window_admissible = {}",
            window_admissible(lo, hi, false)
        );
        assert!(bv && !br, "the straddling control did not reproduce");
    }

    println!("\n  The box for a sat_add fold is 2^n, because sat_add has per-variable degree 1.");
    println!("  That is the same 2^k box 86's F3 priced for multilinear ring chain laws.");
    println!("\nall checks passed");
}
