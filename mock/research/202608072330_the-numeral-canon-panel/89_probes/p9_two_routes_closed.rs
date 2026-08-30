// PROBE p9 (file 89). Two routes attacked and closed, each with the
// counterexample that closes it, so neither is left as a caution.
//
// ROUTE A: does the box criterion survive a fraction width, F > 0?
// Every finding in this unit carries F = 0. That is normally a statement that
// nothing was measured above it. Here it is a boundary with a two-point
// witness. A fixed-point saturating multiply is clamp(floor(a*b / 2^F)), and a
// right shift by a constant is nondecreasing, so the MONOTONICITY half of the
// box criterion's hypothesis survives F > 0 intact. What does not survive is
// the POLYNOMIAL half: with a floor in the term the value is not a polynomial
// of bounded degree, and the falling-factorial interpolation that turns
// agreement on the box into agreement everywhere has nothing to interpolate.
// The counterexample is two shifts, degree 1 each, agreeing on the whole
// degree box {0, 1} and differing at 2.
//
// ROUTE B: can the general signed case be decided by a saturation radius?
// For a term of degree D the value saturates once |x| exceeds about
// MAX^(1/D), so a sweep of [-R, R] plus a tail argument would decide the law
// with 2R+1 evaluations, which is inside the const budget as soon as D >= 4.
// The route needs the tails to be CONSTANT. They are not: after the high-degree
// nodes clamp, a lower-degree remainder survives and keeps moving. The
// counterexample is one term, and it kills the cheap form of the route while
// leaving the expensive form (exact coefficient tracking plus root isolation)
// unbuilt and open.
//
// Runtime spike; std/Vec/Box are scaffolding, not design shape.

fn umax(w: u32) -> i128 {
    (1i128 << w) - 1
}
fn smin(w: u32) -> i128 {
    -(1i128 << (w - 1))
}
fn smax(w: u32) -> i128 {
    (1i128 << (w - 1)) - 1
}

// ---------------------------------------------------------------- route A

/// unsigned saturating term with a right shift by a constant, which is what a
/// fraction width introduces
#[derive(Clone)]
enum F {
    X,
    K(i128),
    Add(Box<F>, Box<F>),
    Mul(Box<F>, Box<F>),
    Shr(Box<F>, u32),
}

impl F {
    fn sat(&self, x: i128, w: u32) -> i128 {
        let m = umax(w);
        let cl = |v: i128| {
            if v > m {
                m
            } else if v < 0 {
                0
            } else {
                v
            }
        };
        match self {
            F::X => x,
            F::K(c) => cl(*c),
            F::Add(a, b) => cl(a.sat(x, w) + b.sat(x, w)),
            F::Mul(a, b) => cl(a.sat(x, w) * b.sat(x, w)),
            F::Shr(a, s) => a.sat(x, w) >> s,
        }
    }
    /// the same syntactic degree bound the criterion uses. A shift does not
    /// raise the degree, so the extractor reports 1 for both sides below.
    fn degree(&self) -> u64 {
        match self {
            F::X => 1,
            F::K(_) => 0,
            F::Add(a, b) => a.degree().max(b.degree()),
            F::Mul(a, b) => a.degree() + b.degree(),
            F::Shr(a, _) => a.degree(),
        }
    }
    fn show(&self) -> String {
        match self {
            F::X => "x".into(),
            F::K(c) => format!("{}", c),
            F::Add(a, b) => format!("({}+{})", a.show(), b.show()),
            F::Mul(a, b) => format!("({}*{})", a.show(), b.show()),
            F::Shr(a, s) => format!("({}>>{})", a.show(), s),
        }
    }
}

fn box_verdict_f(a: &F, b: &F, w: u32) -> bool {
    let m = umax(w);
    let d = a.degree().max(b.degree()) as i128;
    let hi = if d < m { d } else { m };
    let mut x = 0i128;
    while x <= hi {
        if a.sat(x, w) != b.sat(x, w) {
            return false;
        }
        x += 1;
    }
    true
}
fn sweep_f(a: &F, b: &F, w: u32) -> Option<i128> {
    let m = umax(w);
    let mut x = 0i128;
    while x <= m {
        if a.sat(x, w) != b.sat(x, w) {
            return Some(x);
        }
        x += 1;
    }
    None
}

// ---------------------------------------------------------------- route B

#[derive(Clone)]
enum S {
    V,
    K(i128),
    Add(Box<S>, Box<S>),
    Mul(Box<S>, Box<S>),
}
impl S {
    fn sat(&self, x: i128, w: u32) -> i128 {
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
            S::V => x,
            S::K(c) => cl(*c),
            S::Add(a, b) => cl(a.sat(x, w) + b.sat(x, w)),
            S::Mul(a, b) => cl(a.sat(x, w) * b.sat(x, w)),
        }
    }
    fn degree(&self) -> u64 {
        match self {
            S::V => 1,
            S::K(_) => 0,
            S::Add(a, b) => a.degree().max(b.degree()),
            S::Mul(a, b) => a.degree() + b.degree(),
        }
    }
    fn show(&self) -> String {
        match self {
            S::V => "x".into(),
            S::K(c) => format!("{}", c),
            S::Add(a, b) => format!("({}+{})", a.show(), b.show()),
            S::Mul(a, b) => format!("({}*{})", a.show(), b.show()),
        }
    }
}

fn main() {
    println!("p9: two routes attacked and closed\n");

    // ---------- ROUTE A ----------
    println!("ROUTE A. The box criterion at F > 0, where a shift enters the term.");
    let a = F::Mul(Box::new(F::Shr(Box::new(F::X), 2)), Box::new(F::K(4)));
    let b = F::Mul(Box::new(F::Shr(Box::new(F::X), 1)), Box::new(F::K(2)));
    println!("  A = {}   (degree bound {})", a.show(), a.degree());
    println!("  B = {}   (degree bound {})", b.show(), b.degree());
    for w in 3..=8u32 {
        let bv = box_verdict_f(&a, &b, w);
        let sw = sweep_f(&a, &b, w);
        println!(
            "    width {:>2}: box criterion says {:<5}  sweep says {:<5} (first witness {:?})",
            w,
            bv,
            sw.is_none(),
            sw
        );
        assert!(
            bv && sw.is_some(),
            "route A's counterexample stopped working at width {}",
            w
        );
    }
    println!("  The criterion says TRUE at every width and the law is FALSE at every");
    println!("  width, first witness x = 2. Both sides have degree bound 1, so the box");
    println!("  is {{0, 1}}, and a shift makes the value non-polynomial, so agreement on");
    println!("  the box carries nothing. F = 0 is a hard boundary of the criterion, not");
    println!("  an untested caution.\n");

    // and the same shape counted over a small term space, so the failure is a
    // class rather than one hand-built pair
    let base: Vec<F> = vec![F::X, F::K(1), F::K(2), F::K(3)];
    let mut sh: Vec<F> = Vec::new();
    for t in &base {
        for s in 1..=3u32 {
            sh.push(F::Shr(Box::new(t.clone()), s));
        }
    }
    let mut terms: Vec<F> = base.clone();
    terms.extend(sh);
    let mut more: Vec<F> = terms.clone();
    for t in &terms {
        for u in &terms {
            more.push(F::Add(Box::new(t.clone()), Box::new(u.clone())));
            more.push(F::Mul(Box::new(t.clone()), Box::new(u.clone())));
        }
    }
    let (mut checked, mut lies) = (0u64, 0u64);
    for w in 4..=6u32 {
        for i in 0..more.len() {
            for j in (i + 1)..more.len() {
                checked += 1;
                if box_verdict_f(&more[i], &more[j], w) && sweep_f(&more[i], &more[j], w).is_some()
                {
                    lies += 1;
                }
            }
        }
    }
    println!(
        "  over a shift-carrying term space, widths 4..=6: {} pairs checked,",
        checked
    );
    println!(
        "  {} of them are TRUE on the box and FALSE in the domain.",
        lies
    );
    assert!(lies > 0);

    // ---------- ROUTE B ----------
    println!("\nROUTE B. A saturation radius for the general signed case.");
    let t = S::Add(
        Box::new(S::Mul(Box::new(S::V), Box::new(S::V))),
        Box::new(S::V),
    );
    println!("  T = {}   (degree {})", t.show(), t.degree());
    println!("  the route needs T to be CONSTANT outside a radius. On the negative tail");
    println!("  the square saturates to MAX and the surviving linear term keeps moving:");
    for w in [8u32, 10] {
        let lo = smin(w);
        let vals: Vec<i128> = (0..6).map(|i| t.sat(lo + i, w)).collect();
        println!("    width {:>2}: T at MIN..MIN+5 = {:?}", w, vals);
        let constant = vals.windows(2).all(|p| p[0] == p[1]);
        assert!(!constant, "route B's counterexample stopped working");
    }
    println!("  Not constant, so a sweep of [-R, R] plus a constant-tail argument does");
    println!("  not decide the law. What survives after the top-degree nodes clamp is a");
    println!("  lower-degree remainder, which is the piecewise structure again. The");
    println!("  expensive form of the route, exact coefficient tracking with root");
    println!("  isolation to locate the clamp boundaries, is not built here and stays open.");

    println!("\nall checks passed");
}
