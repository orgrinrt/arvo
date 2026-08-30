//! Probe 4: which factor of the reduction breaks the semiring, and what
//! coherence actually buys.
//!
//! TWO QUESTIONS, both raised by `p3`.
//!
//! ONE: `p3` found the unsigned-saturation semiring holding at nine widths on
//! the integer grid and collapsing at every fractional scale, with
//! multiplicative associativity and distributivity failing while additive
//! associativity survives. That is a count, not an attribution. The reduction
//! at F > 0 is a COMPOSITE of two different maps:
//!
//!     a GRID coarsening  x |-> x >> F   (changes the ulp, leaves the range)
//!     a RANGE clamping   x |-> sat(x)   (changes the bounds, leaves the ulp)
//!
//! and the semiring could be dying from either. This probe runs each factor
//! alone and then the composite, so the blame lands on a named map. If the
//! clamp alone is a semiring and the coarsening alone is not, then `42:187-194`
//! and `p3` section 3 are both really about ROUNDING, and saying "saturation
//! breaks distributivity" would be false in a way that matters, since it would
//! send a law layer to condition on the wrong axis of the format.
//!
//! The stronger corollary if that lands: COHERENCE DOES NOT COMPOSE. Each
//! factor could be individually well behaved and the composite still not be, so
//! a law layer cannot derive a composite operation's laws from its parts'.
//!
//! TWO: what coherence buys, stated so it can be checked rather than admired.
//! `55b` section 3.1 argues coherence is definitionally a homomorphism onto the
//! induced algebra. If that is right, coherence has an operational consequence:
//! reducing after EVERY step gives the same answer as computing exactly and
//! reducing ONCE at the end, for folds of any length. That is the property that
//! decides whether an implementation needs a wider accumulator. This probe
//! measures it at fold lengths 2 through 6, which is `55b`'s stated open edge
//! ("whether the pullback mechanism survives fold lengths past three").
//!
//! INSTRUMENT VALIDATION. The eager-versus-once checker must report zero on the
//! coherent policies and nonzero on the incoherent one at the same fold length,
//! on the same code path. The factor checkers must disagree with each other.
//! Both are printed rather than summarised.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p4 p4_which_factor_breaks_and_what_coherence_buys.rs && ./p4

// ---------------------------------------------------------------- factors

#[derive(Clone, Copy, PartialEq, Eq)]
enum Factor {
    /// range clamping only: saturate into [0, m], exact grid
    ClampOnly,
    /// grid coarsening only: rescale by >> f, no range bound at all
    CoarsenOnly,
    /// the composite arvo actually performs at F > 0
    Composite,
}

struct Alg {
    factor: Factor,
    m: i64,
    f: u32,
}

impl Alg {
    fn one(&self) -> i64 {
        let s = 1i64 << self.f;
        match self.factor {
            Factor::ClampOnly => 1.min(self.m),
            Factor::CoarsenOnly => s,
            Factor::Composite => s.min(self.m),
        }
    }
    fn hi(&self) -> i64 {
        // the element set each factor ranges over
        match self.factor {
            Factor::ClampOnly => self.m,
            // coarsen-only has no range bound; a finite element set is needed to
            // enumerate, so it uses the same count of elements, which keeps the
            // comparison against the other two like for like
            Factor::CoarsenOnly => self.m,
            Factor::Composite => self.m,
        }
    }
    fn add(&self, a: i64, b: i64) -> i64 {
        match self.factor {
            Factor::ClampOnly => (a + b).clamp(0, self.m),
            Factor::CoarsenOnly => a + b, // addition needs no rescale
            Factor::Composite => (a + b).clamp(0, self.m),
        }
    }
    fn mul(&self, a: i64, b: i64) -> i64 {
        let s = 1i64 << self.f;
        match self.factor {
            Factor::ClampOnly => (a * b).clamp(0, self.m),
            Factor::CoarsenOnly => (a * b) / s,
            Factor::Composite => ((a * b) / s).clamp(0, self.m),
        }
    }
}

struct Res {
    add_assoc: u64,
    mul_assoc: u64,
    distrib: u64,
}

fn axioms(alg: &Alg) -> Res {
    let hi = alg.hi();
    let mut r = Res {
        add_assoc: 0,
        mul_assoc: 0,
        distrib: 0,
    };
    for a in 0..=hi {
        for b in 0..=hi {
            for c in 0..=hi {
                if alg.add(alg.add(a, b), c) != alg.add(a, alg.add(b, c)) {
                    r.add_assoc += 1;
                }
                if alg.mul(alg.mul(a, b), c) != alg.mul(a, alg.mul(b, c)) {
                    r.mul_assoc += 1;
                }
                if alg.mul(a, alg.add(b, c)) != alg.add(alg.mul(a, b), alg.mul(a, c)) {
                    r.distrib += 1;
                }
            }
        }
    }
    r
}

// ------------------------------------------------- eager versus once, n-ary

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    UnsignedSat,
    SignedSat,
    Wrap,
}

fn red(p: Policy, x: i64) -> i64 {
    match p {
        Policy::UnsignedSat => x.clamp(0, 15),
        Policy::SignedSat => x.clamp(-8, 7),
        Policy::Wrap => ((x + 8).rem_euclid(16)) - 8,
    }
}

fn domain(p: Policy) -> (i64, i64) {
    match p {
        Policy::UnsignedSat => (0, 15),
        Policy::SignedSat | Policy::Wrap => (-8, 7),
    }
}

/// count operand tuples of length n where reducing after every step differs
/// from summing exactly and reducing once. that difference is precisely the
/// need for a wider accumulator.
fn eager_versus_once(p: Policy, n: usize) -> (u64, u64) {
    let (lo, hi) = domain(p);
    let span = (hi - lo + 1) as usize;
    let total = span.pow(n as u32) as u64;
    let mut diff = 0u64;
    let mut idx = vec![0usize; n];
    loop {
        let mut eager = 0i64;
        let mut exact = 0i64;
        for k in 0..n {
            let v = lo + idx[k] as i64;
            eager = red(p, eager + v);
            exact += v;
        }
        if eager != red(p, exact) {
            diff += 1;
        }
        // odometer
        let mut k = 0;
        loop {
            if k == n {
                return (total, diff);
            }
            idx[k] += 1;
            if idx[k] < span {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
    }
}

fn main() {
    let mut ok = true;

    println!("=== 1. which factor of the reduction breaks the semiring ===");
    println!();
    println!("  clamp-only  = range clamping, exact grid   (F is not applied)");
    println!("  coarsen-only= grid coarsening by >> F, no range bound at all");
    println!("  composite   = what arvo performs at F > 0");
    println!();
    println!(
        "{:>14} {:>5} {:>4} {:>9} {:>9} {:>9}",
        "factor", "M", "F", "+assoc", "*assoc", "distrib"
    );
    let mut clamp_clean = true;
    let mut coarsen_dirty = false;
    for &m in &[15i64, 31] {
        for &f in &[1u32, 2] {
            for (name, fac) in [
                ("clamp-only", Factor::ClampOnly),
                ("coarsen-only", Factor::CoarsenOnly),
                ("composite", Factor::Composite),
            ] {
                let alg = Alg { factor: fac, m, f };
                let r = axioms(&alg);
                println!(
                    "{:>14} {:>5} {:>4} {:>9} {:>9} {:>9}",
                    name, m, f, r.add_assoc, r.mul_assoc, r.distrib
                );
                if fac == Factor::ClampOnly {
                    clamp_clean &= r.add_assoc == 0 && r.mul_assoc == 0 && r.distrib == 0;
                }
                if fac == Factor::CoarsenOnly {
                    coarsen_dirty |= r.mul_assoc > 0 || r.distrib > 0;
                }
            }
            println!();
        }
    }
    println!(
        "  clamp-only clean at every row measured:   {}",
        clamp_clean
    );
    println!(
        "  coarsen-only broke something:             {}",
        coarsen_dirty
    );
    println!();
    println!("  Reading: if the clamp is clean and the coarsening is not, then the");
    println!("  fractional collapse `p3` section 3 measured is a ROUNDING fact, not a");
    println!("  saturation fact, and coherence does not compose: each factor can be");
    println!("  individually well behaved with the composite still not being.");
    ok &= clamp_clean && coarsen_dirty;

    println!();
    println!("=== 2. what coherence buys: eager reduction versus one reduction at the end ===");
    println!();
    println!("  A coherent reduction is a homomorphism, so reducing at every step must");
    println!("  agree with computing exactly and reducing once, at EVERY fold length.");
    println!("  An incoherent one must not. This is the property that decides whether an");
    println!("  implementation needs a wider accumulator.");
    println!();
    println!(
        "{:>4} {:>14} {:>12} {:>14} {:>10}",
        "n", "policy", "tuples", "eager != once", "fraction"
    );
    let mut coherent_zero_everywhere = true;
    let mut incoherent_grows = Vec::new();
    for n in 2..=6usize {
        for (name, p) in [
            ("wrap", Policy::Wrap),
            ("unsigned sat", Policy::UnsignedSat),
            ("signed sat", Policy::SignedSat),
        ] {
            let (total, diff) = eager_versus_once(p, n);
            println!(
                "{:>4} {:>14} {:>12} {:>14} {:>9.2}%",
                n,
                name,
                total,
                diff,
                100.0 * diff as f64 / total as f64
            );
            if p != Policy::SignedSat {
                coherent_zero_everywhere &= diff == 0;
            } else {
                incoherent_grows.push(100.0 * diff as f64 / total as f64);
            }
        }
        println!();
    }
    println!(
        "  coherent policies at zero divergence for every n from 2 to 6: {}",
        coherent_zero_everywhere
    );
    let monotone = incoherent_grows.windows(2).all(|w| w[1] > w[0]);
    println!(
        "  signed saturation's divergence fraction strictly increasing in n: {}",
        monotone
    );
    println!(
        "  fractions by n: {:?}",
        incoherent_grows
            .iter()
            .map(|x| (x * 100.0).round() / 100.0)
            .collect::<Vec<_>>()
    );
    println!();
    println!("  This answers `55b`'s stated open edge ('whether the pullback mechanism");
    println!("  survives fold lengths past three') for the coherence half: the split is");
    println!("  not a length-three artifact, and the incoherent policy gets worse with n");
    println!("  while the coherent ones stay exactly at zero.");
    ok &= coherent_zero_everywhere;

    println!();
    println!("=== 3. instrument validation ===");
    println!();
    println!("  same checker, same code path, opposite verdicts on the same fold length:");
    // n starts at 3, not 2. The first run of this probe asserted a divergence
    // at n = 2 and reported FAILS, correctly: with two operands there is only
    // one association order, so eager and once are the same computation and
    // NOTHING can diverge, for any policy. That run is kept beside this file as
    // `p4_output.v1_failed_assertion.txt`. The measurement was right and the
    // assertion was wrong, and the zero at n = 2 is itself informative: it
    // confirms the divergence is a fact about ASSOCIATION rather than about the
    // reduction, since the reduction is equally present at n = 2 and costs
    // nothing there.
    for n in [2usize, 3, 4, 6] {
        let (_, w) = eager_versus_once(Policy::Wrap, n);
        let (_, s) = eager_versus_once(Policy::SignedSat, n);
        println!(
            "    n={}: wrap {} divergences, signed saturation {} divergences",
            n, w, s
        );
        ok &= w == 0;
        if n == 2 {
            ok &= s == 0; // nothing to associate
        } else {
            ok &= s > 0;
        }
    }
    println!(
        "  factor checkers disagreed with each other rather than agreeing: {}",
        clamp_clean && coarsen_dirty
    );

    println!();
    println!("{}", if ok { "P4 WORKS" } else { "P4 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
