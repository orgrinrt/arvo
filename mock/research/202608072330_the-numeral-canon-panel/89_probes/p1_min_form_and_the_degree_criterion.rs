// PROBE p1 (file 89). Two theorems that collapse 86's piecewise procedure.
//
// THEOREM A (min-form). Over the monotone unsigned saturating fragment
// (terms over one variable x, nonnegative constants clamp-embedded, sat_add,
// sat_mul), saturating evaluation at width W equals
//
//     eval_sat(t, x, W)  =  min( P_t(x), MAX_W )
//
// where P_t is the exact integer polynomial of t with its constants embedded.
// Clamping early and clamping late coincide, including the c = 0 case where
// both give 0. Every P_t has nonnegative integer coefficients, hence is
// nondecreasing on x >= 0.
//
// THEOREM B (the degree criterion). Let D = max(deg P_A, deg P_B), computed
// syntactically (exact for this fragment: no cancellation). Then
//
//     forall x in [0, MAX_W]: A(x) == B(x)     <=>     A(x) == B(x) for x in 0..=D
//
// Proof. (<=) trivial. (=>) Suppose agreement on 0..=D.
//   Case 1: P_A(D) <= MAX and P_B(D) <= MAX. By monotonicity nothing clamps at
//     any x <= D, so P_A = P_B at D+1 points; both have degree <= D, so
//     P_A == P_B identically, so min(P_A,MAX) == min(P_B,MAX) everywhere.
//   Case 2: P_A(D) > MAX (symmetrically P_B). Then A(D) = MAX, so B(D) = MAX,
//     so P_B(D) >= MAX. For x > D both P_A(x) >= P_A(D) > MAX and
//     P_B(x) >= P_B(D) >= MAX, so both sides are MAX. For x <= D the points are
//     checked directly. QED.
//
// CONSEQUENCE. 86's breakpoint search, its binary searches, its piece
// bookkeeping and its monotonicity-of-clamp-indicator induction are all
// unnecessary. The verdict is D+1 evaluations of each side, from 0, and the
// test set is the degree grid, exactly as 86 section 3 found for the RING
// fragment. Two fragments, one test-set shape.
//
// This probe refuses to trust either theorem and measures both, then runs
// mutation controls so the battery is shown capable of failing, then reports
// which branch of Theorem B's proof each case exercises.
//
// Runtime spike; std/Vec/Box are scaffolding, not design shape.

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
    /// saturating evaluation, clamping at every node
    fn sat(&self, x: u128, w: u32) -> u128 {
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
                let s = a.sat(x, w) + b.sat(x, w);
                if s > m {
                    m
                } else {
                    s
                }
            }
            T::Mul(a, b) => {
                let s = a.sat(x, w) * b.sat(x, w);
                if s > m {
                    m
                } else {
                    s
                }
            }
        }
    }
    /// exact integer evaluation with the constants embedded at width w,
    /// saturating at u128::MAX. Sound for comparison against MAX_W because the
    /// fragment is nonnegative and monotone: once above u128::MAX/4 the true
    /// value can only grow, and MAX_W <= 2^64-1.
    fn exact(&self, x: u128, w: u32) -> u128 {
        const HUGE: u128 = u128::MAX;
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
            T::Add(a, b) => a.exact(x, w).saturating_add(b.exact(x, w)),
            T::Mul(a, b) => {
                let (u, v) = (a.exact(x, w), b.exact(x, w));
                if u == 0 || v == 0 {
                    0
                } else if u > HUGE / v {
                    HUGE
                } else {
                    u * v
                }
            }
        }
    }
    fn degree(&self) -> u64 {
        match self {
            T::X => 1,
            T::C(_) => 0,
            T::Add(a, b) => a.degree().max(b.degree()),
            T::Mul(a, b) => a.degree() + b.degree(),
        }
    }
    fn n_ops(&self) -> u32 {
        match self {
            T::X | T::C(_) => 0,
            T::Add(a, b) | T::Mul(a, b) => 1 + a.n_ops() + b.n_ops(),
        }
    }
}

/// THEOREM B as a procedure. Returns (verdict, witness).
fn degree_criterion(a: &T, b: &T, w: u32) -> (bool, Option<u128>) {
    let m = umax(w);
    let d = a.degree().max(b.degree()) as u128;
    let hi = if d < m { d } else { m };
    let mut x = 0u128;
    while x <= hi {
        if a.sat(x, w) != b.sat(x, w) {
            return (false, Some(x));
        }
        x += 1;
    }
    (true, None)
}

/// mutants, each deleting one thing the criterion relies on
#[derive(Clone, Copy, PartialEq)]
enum Mut {
    Ctl,
    DSamples,
    StartAtOne,
    Spread,
    HalfDegree,
}

fn mutant(a: &T, b: &T, w: u32, mu: Mut) -> bool {
    let m = umax(w);
    let d = a.degree().max(b.degree()) as u128;
    let pts: Vec<u128> = match mu {
        Mut::Ctl => (0..=d.min(m)).collect(),
        Mut::DSamples => (0..d.min(m)).collect(),
        Mut::StartAtOne => (1..=(d + 1).min(m)).collect(),
        Mut::HalfDegree => (0..=(d / 2).min(m)).collect(),
        Mut::Spread => {
            let n = d.min(m) + 1;
            (0..n)
                .map(|i| if n <= 1 { 0 } else { i * m / (n - 1) })
                .collect()
        }
    };
    for x in pts {
        if a.sat(x, w) != b.sat(x, w) {
            return false;
        }
    }
    true
}

fn sweep_first_witness(a: &T, b: &T, w: u32) -> Option<u128> {
    let m = umax(w);
    let mut x = 0u128;
    while x <= m {
        if a.sat(x, w) != b.sat(x, w) {
            return Some(x);
        }
        x += 1;
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

fn rand_term(rng: &mut Xs, depth: u32, big_consts: bool) -> T {
    if depth == 0 || rng.next() % 4 == 0 {
        if rng.next() % 2 == 0 {
            T::X
        } else if big_consts && rng.next() % 3 == 0 {
            T::C((rng.next() % 4096) << (rng.next() % 12))
        } else {
            T::C((rng.next() % 8) << (rng.next() % 4))
        }
    } else {
        let a = Box::new(rand_term(rng, depth - 1, big_consts));
        let b = Box::new(rand_term(rng, depth - 1, big_consts));
        if rng.next() % 2 == 0 {
            T::Add(a, b)
        } else {
            T::Mul(a, b)
        }
    }
}
fn pow(d: u32) -> T {
    let mut t = T::X;
    for _ in 1..d {
        t = T::Mul(Box::new(t), Box::new(T::X));
    }
    t
}
fn c(v: u64) -> T {
    T::C(v)
}
fn add(a: T, b: T) -> T {
    T::Add(Box::new(a), Box::new(b))
}
fn mul(a: T, b: T) -> T {
    T::Mul(Box::new(a), Box::new(b))
}

fn main() {
    println!(
        "p1: the min-form lemma and the degree criterion for monotone unsigned saturating laws\n"
    );

    // ---------------- battery ----------------
    let mut rng = Xs(0x89_1337_5AFE);
    let mut pairs: Vec<(String, T, T)> = Vec::new();
    // deeper and higher-degree than 86's: depth 4, degree cap 28
    while pairs.len() < 500 {
        let a = rand_term(&mut rng, 4, true);
        let b = rand_term(&mut rng, 4, true);
        if a.degree() <= 28 && b.degree() <= 28 && a.n_ops() + b.n_ops() > 0 {
            pairs.push(("random".into(), a, b));
        }
    }
    // near-miss pairs: same term with one leaf perturbed, so agreement is
    // decided by clamping rather than by the terms being obviously different
    let mut rng2 = Xs(0xBEEF_0089);
    while pairs.len() < 700 {
        let a = rand_term(&mut rng2, 4, true);
        let b = mul(a.clone(), c(1 + (rng2.next() % 3)));
        if a.degree() <= 28 && b.degree() <= 28 {
            pairs.push(("near-miss".into(), a, b));
        }
    }
    // the E_d family, the family 86 built
    for d in 2..=14u32 {
        pairs.push((format!("E_{}", d), pow(d), pow(d + 1)));
    }
    // constant-embedding family: two constants that collide below a width and
    // separate above it
    for k in 3..=13u32 {
        let cc = (1u64 << k) - 1;
        pairs.push((
            format!("Cpair_{}", k),
            mul(T::X, c(cc)),
            mul(T::X, add(c(cc), c(1))),
        ));
    }
    // adversarial: agreement forced deep by construction
    for d in 2..=8u32 {
        pairs.push((
            format!("adv_{}", d),
            add(pow(d), c(1)),
            add(mul(pow(d), c(1)), c(1)),
        ));
    }

    let wmax = 13u32;
    let (mut minform_checked, mut minform_bad) = (0u64, 0u64);
    let (mut checked, mut mis) = (0u64, 0u64);
    let (mut tcnt, mut fcnt) = (0u64, 0u64);
    let (mut branch1, mut branch2) = (0u64, 0u64);
    let (mut wit_gt_d, mut wit_eq_d, mut max_wit_over_d) = (0u64, 0u64, 0f64);
    let mut bad_witness = 0u64;

    for (_n, a, b) in &pairs {
        let d = a.degree().max(b.degree());
        for w in 1..=wmax {
            let m = umax(w);
            // Theorem A, exhaustively over the whole domain
            let mut x = 0u128;
            while x <= m {
                for t in [a, b] {
                    let s = t.sat(x, w);
                    let e = t.exact(x, w);
                    let f = if e > m { m } else { e };
                    minform_checked += 1;
                    if s != f {
                        minform_bad += 1;
                    }
                }
                x += 1;
            }
            // Theorem B against the exhaustive sweep
            let sw = sweep_first_witness(a, b, w);
            let (v, wit) = degree_criterion(a, b, w);
            checked += 1;
            if (sw.is_none()) != v {
                mis += 1;
            }
            if v {
                tcnt += 1
            } else {
                fcnt += 1
            }
            if let Some(x) = wit {
                if a.sat(x, w) == b.sat(x, w) {
                    bad_witness += 1;
                }
            }
            // which branch of the proof does this case exercise?
            let dd = (d as u128).min(m);
            if a.exact(dd, w) > m || b.exact(dd, w) > m {
                branch2 += 1;
            } else {
                branch1 += 1;
            }
            // how deep does the sweep's own first witness sit relative to D?
            if let Some(x) = sw {
                if x > d as u128 {
                    wit_gt_d += 1;
                }
                if x == d as u128 {
                    wit_eq_d += 1;
                }
                let r = x as f64 / (d.max(1) as f64);
                if r > max_wit_over_d {
                    max_wit_over_d = r;
                }
            }
        }
    }

    println!("battery: {} term pairs x widths 1..={}", pairs.len(), wmax);
    println!("\nTHEOREM A (min-form), checked at every point of every domain:");
    println!(
        "  (term, x, width) evaluations compared: {}",
        minform_checked
    );
    println!("  sat(t,x,w) != min(exact,MAX):          {}", minform_bad);

    println!("\nTHEOREM B (degree criterion) against exhaustive sweeps:");
    println!("  (pair, width) verdicts:                {}", checked);
    println!("  true {}, false {}", tcnt, fcnt);
    println!("  criterion vs sweep mismatches:         {}", mis);
    println!("  returned witnesses that fail:          {}", bad_witness);
    println!(
        "  cases exercising proof branch 1 (no clamp at D): {}",
        branch1
    );
    println!(
        "  cases exercising proof branch 2 (clamp at D):    {}",
        branch2
    );
    println!("\n  THE FALSIFICATION TEST: a first witness strictly above D would");
    println!("  refute Theorem B outright.");
    println!(
        "  false cases whose sweep-first-witness  >  D: {}",
        wit_gt_d
    );
    println!(
        "  false cases whose sweep-first-witness ==  D: {}",
        wit_eq_d
    );
    println!(
        "  max (first witness / D) observed:            {:.4}",
        max_wit_over_d
    );

    println!("\nMUTATION CONTROLS (the battery must notice a weakened criterion):");
    for mu in [
        Mut::Ctl,
        Mut::DSamples,
        Mut::StartAtOne,
        Mut::Spread,
        Mut::HalfDegree,
    ] {
        let mut mm = 0u64;
        for (_n, a, b) in &pairs {
            for w in 1..=wmax {
                let s = sweep_first_witness(a, b, w).is_none();
                if mutant(a, b, w, mu) != s {
                    mm += 1;
                }
            }
        }
        let name = match mu {
            Mut::Ctl => "control: 0..=D",
            Mut::DSamples => "0..D (one short)",
            Mut::StartAtOne => "1..=D+1 (skip zero)",
            Mut::Spread => "D+1 points spread over domain",
            Mut::HalfDegree => "0..=D/2",
        };
        println!("  {:<32} mismatches: {}", name, mm);
    }

    assert!(minform_bad == 0, "THEOREM A refuted");
    assert!(mis == 0 && bad_witness == 0, "THEOREM B refuted");
    assert!(wit_gt_d == 0, "THEOREM B refuted by a deep witness");
    assert!(
        branch1 > 100 && branch2 > 100,
        "battery does not exercise both proof branches"
    );

    // ---------------- the shipped width ----------------
    println!("\nwidth-64 verdicts by the degree criterion (evaluations = 2*(D+1)):");
    for (label, a, b) in [
        ("E_63  x^63 == x^64", pow(63), pow(64)),
        ("E_64  x^64 == x^65", pow(64), pow(65)),
        (
            "C-mem x*C == x*(C+1), C = 2^63-1",
            mul(T::X, c((1u64 << 63) - 1)),
            mul(T::X, add(c((1u64 << 63) - 1), c(1))),
        ),
    ] {
        let d = a.degree().max(b.degree());
        let (v, wit) = degree_criterion(&a, &b, 64);
        println!(
            "  {:<34} {}  witness {:?}   D = {}, term-node ops ~ {}",
            label,
            if v { "TRUE " } else { "FALSE" },
            wit,
            d,
            (d + 1) * (a.n_ops() + b.n_ops()) as u64
        );
    }
    let (v, w1) = degree_criterion(&pow(63), &pow(64), 64);
    assert!(!v && w1 == Some(2));
    let (v, _) = degree_criterion(&pow(64), &pow(65), 64);
    assert!(v);
    let (v, _) = degree_criterion(
        &mul(T::X, c((1u64 << 63) - 1)),
        &mul(T::X, add(c((1u64 << 63) - 1), c(1))),
        40,
    );
    assert!(v, "C-member must hold at width 40");

    println!(
        "\n86's p5 spent 516,033 / 616,999 counted evaluation steps on E_63 / E_64 at width 64."
    );
    println!(
        "The degree criterion spends 2*(D+1) term evaluations: 130 evaluations of a 63-op term"
    );
    println!("and 130 of a 64-op term, ~8.2k node operations, for the identical verdicts.");
    println!("\nall checks passed");
}
