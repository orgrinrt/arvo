//! Probe p1: the signed cell at F = 0. Does the unsigned integer-grid story
//! (semiring for saturation, ring for wrap) have any signed analogue at all,
//! before the fraction axis is even in play?
//!
//! WHY. `59` section 2c: "signed, multiplication: nothing has been measured
//! and both known failure mechanisms apply." `57`'s congruence argument
//! (`57_probes/p3` section 2) explains the UNSIGNED semiring structurally:
//! "x ~ y iff x == y or both >= M" is a congruence on (N, +, *), so unsigned
//! saturation is the quotient N/~. Nobody has asked whether the signed
//! two-sided clamp has the same property. This probe asks, and it asks the
//! multiplicative half separately from the additive half, because the two
//! break by different mechanisms in the signed domain.
//!
//! HYPOTHESIS, stated before running (worked by hand first):
//!
//!   1. Signed saturating MULTIPLICATION at F = 0 under the two's-complement
//!      asymmetric clamp [lo, hi] = [-2^(w-1), 2^(w-1)-1] is NOT associative.
//!      Hand witness at w = 4 (Q = [-8, 7]): (7 * 7) * -1 = sat(49) * -1
//!      = 7 * -1 = -7, against 7 * (7 * -1) = 7 * -7 = sat(-49) = -8.
//!      Mechanism: negation maps the ceiling's absorbed class asymmetrically.
//!      -hi = -7 is interior while -(hi + k) is beyond the floor for k >= 2,
//!      so "both above hi, or both below lo" is not preserved by
//!      multiplication by -1, and the congruence argument that carried the
//!      unsigned semiring cannot even be stated.
//!
//!   2. Under a SYMMETRIC clamp [-h, h] with h = 2^(w-1)-1 the relation
//!      "x == y or both >= h or both <= -h" IS preserved by multiplication
//!      (integer multiplication by a nonzero value never shrinks magnitude,
//!      and negation maps the ceiling class exactly onto the floor class when
//!      the bounds are mirror images), so signed saturating multiplication
//!      under a symmetric clamp should be exactly associative at F = 0.
//!      Distributivity should still fail, because it routes through the
//!      additive side, and signed saturating ADDITION is broken by the
//!      pullback mechanism (`55b`/`57`) regardless of clamp symmetry.
//!
//!   3. Signed WRAP at F = 0 with the signed representative section is the
//!      ring Z/2^w, zero failures on every axiom, exactly as the unsigned
//!      section is: at F = 0 the section is a relabelling (`56` section 4).
//!
//! The mechanism check is run as multiplicative COHERENCE over an ambient
//! window (sat(x*y) == sat(sat(x)*sat(y))): at F = 0 the ambient multiply is
//! exactly associative, so `57`'s sufficiency theorem has its precondition
//! and coherence implies associativity of the induced operation. The
//! asymmetric clamp must show coherence violations; the symmetric clamp must
//! show zero.
//!
//! INSTRUMENT VALIDATION. The same checkers run on unsigned saturation
//! (must report the semiring `57_probes/p3` measured) and on the
//! opposite-bound mutant (must fail), so the instrument demonstrably fires
//! both ways before any signed row is trusted.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p1 p1_signed_grid_multiplication.rs && ./p1

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    /// clamp into [lo, hi]
    Sat,
    /// wrap into [lo, hi] (hi - lo + 1 residues)
    Wrap,
    /// mutant: overflow resolves to the opposite bound
    OppositeBound,
}

#[derive(Clone, Copy)]
struct Fmt {
    lo: i64,
    hi: i64,
    p: Policy,
}

impl Fmt {
    fn reduce(&self, x: i64) -> i64 {
        let m = self.hi - self.lo + 1;
        match self.p {
            Policy::Sat => x.clamp(self.lo, self.hi),
            Policy::Wrap => {
                let r = (x - self.lo).rem_euclid(m) + self.lo;
                r
            }
            Policy::OppositeBound => {
                if x < self.lo {
                    self.hi
                } else if x > self.hi {
                    self.lo
                } else {
                    x
                }
            }
        }
    }
    fn add(&self, a: i64, b: i64) -> i64 {
        self.reduce(a + b)
    }
    fn mul(&self, a: i64, b: i64) -> i64 {
        self.reduce(a * b)
    }
}

#[derive(Default)]
struct Axioms {
    add_assoc: u64,
    add_comm: u64,
    add_ident: u64,
    mul_assoc: u64,
    mul_comm: u64,
    mul_ident: u64,
    distrib: u64,
    zero_annih: u64,
    no_add_inverse: u64,
}

fn check(f: Fmt) -> Axioms {
    let mut ax = Axioms::default();
    // one = 1 is representable in every format this probe sweeps (F = 0).
    let one = 1i64;
    assert!(f.lo <= 0 && f.hi >= 1, "probe precondition: 0 and 1 in Q");
    for a in f.lo..=f.hi {
        if f.add(a, 0) != a || f.add(0, a) != a {
            ax.add_ident += 1;
        }
        if f.mul(a, one) != a || f.mul(one, a) != a {
            ax.mul_ident += 1;
        }
        if f.mul(a, 0) != 0 || f.mul(0, a) != 0 {
            ax.zero_annih += 1;
        }
        if !(f.lo..=f.hi).any(|b| f.add(a, b) == 0) {
            ax.no_add_inverse += 1;
        }
        for b in f.lo..=f.hi {
            if f.add(a, b) != f.add(b, a) {
                ax.add_comm += 1;
            }
            if f.mul(a, b) != f.mul(b, a) {
                ax.mul_comm += 1;
            }
            for c in f.lo..=f.hi {
                if f.add(f.add(a, b), c) != f.add(a, f.add(b, c)) {
                    ax.add_assoc += 1;
                }
                if f.mul(f.mul(a, b), c) != f.mul(a, f.mul(b, c)) {
                    ax.mul_assoc += 1;
                }
                if f.mul(a, f.add(b, c)) != f.add(f.mul(a, b), f.mul(a, c)) {
                    ax.distrib += 1;
                }
            }
        }
    }
    ax
}

/// multiplicative coherence of the clamp over an ambient window:
/// sat(x*y) == sat(sat(x)*sat(y)). at F = 0 the ambient multiply is exactly
/// associative, so per 57's sufficiency argument coherence here implies
/// associativity of the induced multiplication on Q. returns (violations,
/// first witness).
fn mul_coherence_violations(f: Fmt, win: i64) -> (u64, Option<(i64, i64)>) {
    let mut v = 0u64;
    let mut wit = None;
    for x in -win..=win {
        for y in -win..=win {
            if f.reduce(x * y) != f.reduce(f.reduce(x) * f.reduce(y)) {
                v += 1;
                if wit.is_none() {
                    wit = Some((x, y));
                }
            }
        }
    }
    (v, wit)
}

fn print_row(name: &str, lo: i64, hi: i64, ax: &Axioms) {
    println!(
        "{:>26} [{:>4},{:>4}] {:>7} {:>6} {:>5} {:>7} {:>6} {:>5} {:>8} {:>8} {:>7}",
        name,
        lo,
        hi,
        ax.add_assoc,
        ax.add_comm,
        ax.add_ident,
        ax.mul_assoc,
        ax.mul_comm,
        ax.mul_ident,
        ax.distrib,
        ax.zero_annih,
        ax.no_add_inverse
    );
}

fn header() {
    println!(
        "{:>26} {:>11} {:>7} {:>6} {:>5} {:>7} {:>6} {:>5} {:>8} {:>8} {:>7}",
        "format",
        "Q",
        "+assoc",
        "+comm",
        "+id",
        "*assoc",
        "*comm",
        "*id",
        "distrib",
        "0-annih",
        "no-inv"
    );
}

fn main() {
    let mut ok = true;

    println!("=== section 1: instrument reproduction (unsigned rows must match 57_probes/p3) ===");
    println!();
    header();
    let usat = Fmt {
        lo: 0,
        hi: 15,
        p: Policy::Sat,
    };
    let ax = check(usat);
    print_row("unsigned sat M=15", 0, 15, &ax);
    let usat_ok =
        ax.add_assoc == 0 && ax.mul_assoc == 0 && ax.distrib == 0 && ax.no_add_inverse == 15;
    println!(
        "  matches 57_probes/p3's F=0 semiring row (all-zero laws, 15 without inverse): {}",
        usat_ok
    );
    ok &= usat_ok;
    let mutant = Fmt {
        lo: 0,
        hi: 15,
        p: Policy::OppositeBound,
    };
    let axm = check(mutant);
    print_row("MUTANT opp-bound M=15", 0, 15, &axm);
    let mutant_fires = axm.add_assoc > 0 || axm.mul_assoc > 0;
    println!(
        "  mutant fails at least one law (checker can fail): {}",
        mutant_fires
    );
    ok &= mutant_fires;
    println!();

    println!("=== section 2: signed saturation at F = 0, two's-complement asymmetric clamp ===");
    println!();
    header();
    let mut asym_mul_assoc_all_nonzero = true;
    let mut asym_add_assoc_all_nonzero = true;
    for w in [3u32, 4, 5, 6] {
        let hi = (1i64 << (w - 1)) - 1;
        let lo = -(1i64 << (w - 1));
        let f = Fmt {
            lo,
            hi,
            p: Policy::Sat,
        };
        let ax = check(f);
        print_row(&format!("signed sat 2c w={}", w), lo, hi, &ax);
        asym_mul_assoc_all_nonzero &= ax.mul_assoc > 0;
        asym_add_assoc_all_nonzero &= ax.add_assoc > 0;
    }
    println!();
    println!(
        "  every width fails multiplicative associativity: {}",
        asym_mul_assoc_all_nonzero
    );
    println!(
        "  every width fails additive associativity too (pullback, known from 55b/35): {}",
        asym_add_assoc_all_nonzero
    );
    ok &= asym_mul_assoc_all_nonzero && asym_add_assoc_all_nonzero;
    // the hand witness, checked explicitly so it is on the record
    let f4 = Fmt {
        lo: -8,
        hi: 7,
        p: Policy::Sat,
    };
    let l = f4.mul(f4.mul(7, 7), -1);
    let r = f4.mul(7, f4.mul(7, -1));
    println!(
        "  hand witness w=4: (7*7)*-1 = {}  vs  7*(7*-1) = {}   (predicted -7 vs -8)",
        l, r
    );
    ok &= l == -7 && r == -8;
    println!();

    println!("=== section 3: signed saturation at F = 0, SYMMETRIC clamp [-h, h] ===");
    println!();
    header();
    let mut sym_mul_assoc_all_zero = true;
    let mut sym_distrib_all_nonzero = true;
    for w in [3u32, 4, 5, 6] {
        let h = (1i64 << (w - 1)) - 1;
        let f = Fmt {
            lo: -h,
            hi: h,
            p: Policy::Sat,
        };
        let ax = check(f);
        print_row(&format!("signed sat sym w={}", w), -h, h, &ax);
        sym_mul_assoc_all_zero &= ax.mul_assoc == 0;
        sym_distrib_all_nonzero &= ax.distrib > 0;
    }
    println!();
    println!(
        "  multiplicative associativity EXACT at every width under the symmetric clamp: {}",
        sym_mul_assoc_all_zero
    );
    println!(
        "  distributivity still fails at every width (routes through broken addition): {}",
        sym_distrib_all_nonzero
    );
    ok &= sym_mul_assoc_all_zero && sym_distrib_all_nonzero;
    println!();

    println!(
        "=== section 4: the mechanism, as multiplicative coherence over an ambient window ==="
    );
    println!();
    println!("  sat(x*y) == sat(sat(x)*sat(y)) over [-win, win]; at F = 0 the ambient multiply is");
    println!(
        "  exactly associative, so coherence here implies induced associativity (57's theorem"
    );
    println!("  with its precondition actually met).");
    println!();
    let mut asym_incoherent = true;
    let mut sym_coherent = true;
    for w in [3u32, 4, 5] {
        let hi = (1i64 << (w - 1)) - 1;
        let lo = -(1i64 << (w - 1));
        let h = hi;
        let win = 4 * (1i64 << (w - 1));
        let (va, wa) = mul_coherence_violations(
            Fmt {
                lo,
                hi,
                p: Policy::Sat,
            },
            win,
        );
        let (vs, _) = mul_coherence_violations(
            Fmt {
                lo: -h,
                hi: h,
                p: Policy::Sat,
            },
            win,
        );
        println!(
            "  w={}  win=[-{},{}]  asymmetric [{},{}]: {} violations (first witness {:?});  symmetric [-{},{}]: {} violations",
            w, win, win, lo, hi, va, wa, h, h, vs
        );
        asym_incoherent &= va > 0;
        sym_coherent &= vs == 0;
    }
    println!();
    println!(
        "  asymmetric clamp multiplicatively incoherent at every width: {}",
        asym_incoherent
    );
    println!(
        "  symmetric clamp multiplicatively coherent at every width:    {}",
        sym_coherent
    );
    ok &= asym_incoherent && sym_coherent;
    println!();

    println!("=== section 5: signed wrap at F = 0, signed representative section ===");
    println!();
    header();
    let mut wrap_all_ring = true;
    for w in [3u32, 4, 5, 6] {
        let hi = (1i64 << (w - 1)) - 1;
        let lo = -(1i64 << (w - 1));
        let f = Fmt {
            lo,
            hi,
            p: Policy::Wrap,
        };
        let ax = check(f);
        print_row(&format!("signed wrap w={}", w), lo, hi, &ax);
        let ring = ax.add_assoc == 0
            && ax.add_comm == 0
            && ax.add_ident == 0
            && ax.mul_assoc == 0
            && ax.mul_comm == 0
            && ax.mul_ident == 0
            && ax.distrib == 0
            && ax.zero_annih == 0
            && ax.no_add_inverse == 0;
        wrap_all_ring &= ring;
    }
    println!();
    println!("  signed-section wrap is the full ring Z/2^w at every width (as the unsigned",);
    println!(
        "  section is; at F = 0 the section is a relabelling): {}",
        wrap_all_ring
    );
    ok &= wrap_all_ring;

    println!();
    println!("{}", if ok { "P1 WORKS" } else { "P1 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
