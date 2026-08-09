//! Probe 3: second read of `55_probes/p4`'s unsigned-saturation semiring.
//!
//! WHAT IS BEING CHECKED. `55b` section 3.1 reports, exhaustively at 4 bits,
//! that unsigned saturation induces a commutative semiring with distributivity
//! at zero failures, and names it "the result I did not expect and the one I
//! would most want second-read". `55b` also states the width-transfer claim as
//! "argued, unprobed: the algebra is min-clamp against nonnegatives and nothing
//! in it names 15". This probe probes it, and asks two further questions that
//! decide whether the result is a canon-shaped fact or a fact about the integer
//! grid only.
//!
//!   1. WIDTH. Every commutative-semiring axiom, exhaustively, at M = 1, 2, 3,
//!      7, 15, 31, 63, 127, 255. If the axioms hold at nine widths spanning
//!      eight doublings, "coincidence of four bits" is dead. M = 1 is included
//!      deliberately: it should come out as the Boolean semiring, which is a
//!      structurally different instance rather than another size of the same
//!      one, and is therefore worth more than another power of two.
//!
//!   2. THE STRUCTURAL REASON. A measurement at nine widths is still a
//!      measurement. The reason the axioms hold is that "x ~ y iff x == y or
//!      both >= M" is a CONGRUENCE on the semiring of naturals for both
//!      operations, so the saturating algebra is the quotient N/~ and inherits
//!      every axiom for free. That is checkable directly, and it is checked
//!      here, because a congruence check is a statement about all widths at
//!      once in a way an axiom sweep is not.
//!
//!   3. SCALE, which is where I expect it to break. arvo's format concept is
//!      not the integer grid; it has fractional bits. With F fractional bits
//!      the saturating multiply must rescale, r = sat((ra * rb) >> F), and the
//!      truncation in that shift is a second lossy step the integer case does
//!      not have. `42:187-194` reports distributivity and multiplicative
//!      associativity holding "almost exclusively at F == 0" (I opened `42`;
//!      that is its section 3.3). If that is right, the semiring result is a
//!      fact about F == 0 formats and the canon must say so, because a law
//!      layer that states "unsigned saturation is a semiring" without the scale
//!      condition would licence rewrites that are wrong on every fractional
//!      format. Measured here at F = 0, 1, 2, 3.
//!
//! INSTRUMENT VALIDATION. Every axiom checker is also run against a mutant
//! (saturating at M but with the additive clamp replaced by the opposite-bound
//! wrap-round `55b` uses) which must FAIL axioms the real one passes, and
//! against wrapping which must PASS the ones saturation fails (additive
//! inverses). Both are printed. A checker that passed everything, or failed
//! everything, would be visible in that table rather than hidden behind a
//! summary line.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p3 p3_semiring_across_widths_and_scales.rs && ./p3

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    /// saturate into [0, m]
    UnsignedSat,
    /// two's-complement style wrap into [0, m] (m+1 residues)
    Wrap,
    /// mutant: overflow resolves to the opposite bound
    OppositeBound,
}

fn reduce(p: Policy, m: i64, x: i64) -> i64 {
    match p {
        Policy::UnsignedSat => x.clamp(0, m),
        Policy::Wrap => x.rem_euclid(m + 1),
        Policy::OppositeBound => {
            if x < 0 {
                m
            } else if x > m {
                0
            } else {
                x
            }
        }
    }
}

struct Axioms {
    add_assoc: u64,
    add_comm: u64,
    add_ident: u64,
    mul_assoc: u64,
    mul_comm: u64,
    mul_ident: u64,
    distrib: u64,
    zero_annih: u64,
    /// elements with no additive inverse. a semiring has many, a ring has none.
    no_add_inverse: u64,
}

impl Axioms {
    fn is_comm_semiring(&self) -> bool {
        self.add_assoc == 0
            && self.add_comm == 0
            && self.add_ident == 0
            && self.mul_assoc == 0
            && self.mul_comm == 0
            && self.mul_ident == 0
            && self.distrib == 0
            && self.zero_annih == 0
    }
}

/// F is the count of fractional bits. F = 0 is the integer grid.
/// values are raw integers r denoting r / 2^F; addition is raw addition,
/// multiplication rescales by a right shift, which is arvo's fixed-point
/// multiply shape.
fn check(p: Policy, m: i64, f: u32) -> Axioms {
    let scale = 1i64 << f;
    let add = |a: i64, b: i64| reduce(p, m, a + b);
    let mul = |a: i64, b: i64| reduce(p, m, (a * b) / scale);
    let one = scale.min(m); // the raw encoding of the value 1.0

    let mut ax = Axioms {
        add_assoc: 0,
        add_comm: 0,
        add_ident: 0,
        mul_assoc: 0,
        mul_comm: 0,
        mul_ident: 0,
        distrib: 0,
        zero_annih: 0,
        no_add_inverse: 0,
    };

    for a in 0..=m {
        if add(a, 0) != a || add(0, a) != a {
            ax.add_ident += 1;
        }
        if mul(a, one) != a || mul(one, a) != a {
            ax.mul_ident += 1;
        }
        if mul(a, 0) != 0 || mul(0, a) != 0 {
            ax.zero_annih += 1;
        }
        if !(0..=m).any(|b| add(a, b) == 0) {
            ax.no_add_inverse += 1;
        }
        for b in 0..=m {
            if add(a, b) != add(b, a) {
                ax.add_comm += 1;
            }
            if mul(a, b) != mul(b, a) {
                ax.mul_comm += 1;
            }
            for c in 0..=m {
                if add(add(a, b), c) != add(a, add(b, c)) {
                    ax.add_assoc += 1;
                }
                if mul(mul(a, b), c) != mul(a, mul(b, c)) {
                    ax.mul_assoc += 1;
                }
                if mul(a, add(b, c)) != add(mul(a, b), mul(a, c)) {
                    ax.distrib += 1;
                }
            }
        }
    }
    ax
}

/// is "x ~ y iff x == y or both >= m" a congruence on (N, +, *) restricted to
/// a bounded ambient window. this is the structural reason the quotient is a
/// semiring, and it is a statement about all widths rather than one.
fn congruence_violations(m: i64, ambient: i64) -> (u64, u64) {
    let sat = |x: i64| x.clamp(0, m);
    let mut add_v = 0;
    let mut mul_v = 0;
    // x ~ x' and y ~ y' must imply x+y ~ x'+y' and x*y ~ x'*y'
    for x in 0..=ambient {
        for xp in 0..=ambient {
            if sat(x) != sat(xp) {
                continue; // not related
            }
            for y in 0..=ambient {
                if sat(x + y) != sat(xp + y) {
                    add_v += 1;
                }
                if sat(x * y) != sat(xp * y) {
                    mul_v += 1;
                }
            }
        }
    }
    (add_v, mul_v)
}

fn main() {
    let mut ok = true;

    println!("=== 1. commutative-semiring axioms for unsigned saturation, F = 0 ===");
    println!();
    println!(
        "{:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>8} {:>7} {:>10} {:>10}",
        "M",
        "+assoc",
        "+comm",
        "+id",
        "*assoc",
        "*comm",
        "*id",
        "distrib",
        "0-annih",
        "no-add-inv",
        "verdict"
    );
    let widths = [1i64, 2, 3, 7, 15, 31, 63, 127, 255];
    let mut all_semiring = true;
    for &m in &widths {
        let ax = check(Policy::UnsignedSat, m, 0);
        let v = ax.is_comm_semiring();
        all_semiring &= v;
        println!(
            "{:>6} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>8} {:>7} {:>10} {:>10}",
            m,
            ax.add_assoc,
            ax.add_comm,
            ax.add_ident,
            ax.mul_assoc,
            ax.mul_comm,
            ax.mul_ident,
            ax.distrib,
            ax.zero_annih,
            ax.no_add_inverse,
            if v { "semiring" } else { "NOT" }
        );
    }
    ok &= all_semiring;
    println!();
    println!("  M = 1 is the Boolean semiring: two elements, min-clamped add is OR,");
    println!("  min-clamped mul is AND. A structurally different instance, not another size.");

    println!();
    println!("=== 2. the structural reason: saturation at M is a quotient by a congruence ===");
    println!();
    println!("  'x ~ y iff x == y or both >= M' must be preserved by + and * on the naturals.");
    println!("  If it is, the saturating algebra IS N/~ and inherits every semiring axiom,");
    println!("  which is a claim about all widths rather than the nine measured above.");
    println!();
    let mut cong_ok = true;
    for &m in &[1i64, 3, 7, 15, 31] {
        let (a, b) = congruence_violations(m, 4 * m + 3);
        println!(
            "  M={:<4} ambient 0..{:<4}  congruence violations: + {}  * {}",
            m,
            4 * m + 3,
            a,
            b
        );
        cong_ok &= a == 0 && b == 0;
    }
    ok &= cong_ok;

    println!();
    println!("=== 3. scale: does the semiring survive fractional bits ===");
    println!();
    println!("  Raw value r denotes r / 2^F. Add is raw add; multiply rescales by >> F,");
    println!("  which is a SECOND lossy step the integer grid does not have.");
    println!();
    println!(
        "{:>6} {:>4} {:>7} {:>7} {:>8} {:>7} {:>10}",
        "M", "F", "+assoc", "*assoc", "distrib", "*id", "verdict"
    );
    let mut frac_breaks = 0u64;
    for &m in &[15i64, 31, 63] {
        for f in 0..=3u32 {
            let ax = check(Policy::UnsignedSat, m, f);
            let v = ax.is_comm_semiring();
            if f > 0 && !v {
                frac_breaks += 1;
            }
            println!(
                "{:>6} {:>4} {:>7} {:>7} {:>8} {:>7} {:>10}",
                m,
                f,
                ax.add_assoc,
                ax.mul_assoc,
                ax.distrib,
                ax.mul_ident,
                if v { "semiring" } else { "NOT" }
            );
        }
    }
    println!();
    println!(
        "  fractional configurations that are NOT semirings: {}",
        frac_breaks
    );
    println!("  (a nonzero count here is the finding, not a failure: it bounds the");
    println!("   semiring result to F == 0 and is what the canon would have to state.)");

    println!();
    println!("=== 4. instrument validation: the same checkers on wrap and on the mutant ===");
    println!();
    println!(
        "{:>16} {:>6} {:>7} {:>7} {:>8} {:>10} {:>10}",
        "policy", "M", "+assoc", "*assoc", "distrib", "no-add-inv", "verdict"
    );
    let mut saw_pass = false;
    let mut saw_fail = false;
    let mut wrap_has_inverses = false;
    for (name, p) in [
        ("unsigned sat", Policy::UnsignedSat),
        ("wrap", Policy::Wrap),
        ("MUTANT opp-bound", Policy::OppositeBound),
    ] {
        let ax = check(p, 15, 0);
        let v = ax.is_comm_semiring();
        saw_pass |= v;
        saw_fail |= !v;
        if p == Policy::Wrap {
            wrap_has_inverses = ax.no_add_inverse == 0;
        }
        println!(
            "{:>16} {:>6} {:>7} {:>7} {:>8} {:>10} {:>10}",
            name,
            15,
            ax.add_assoc,
            ax.mul_assoc,
            ax.distrib,
            ax.no_add_inverse,
            if v { "semiring" } else { "NOT" }
        );
    }
    println!();
    println!("  a checker that could not fail would put the same verdict on all three rows.");
    println!("  wrap has every additive inverse (a ring), saturation has 15 of 16 missing");
    println!("  (a semiring and not a ring), the mutant fails outright:");
    println!("    checkers reported at least one pass: {}", saw_pass);
    println!("    checkers reported at least one fail: {}", saw_fail);
    println!(
        "    wrap measured with zero missing additive inverses: {}",
        wrap_has_inverses
    );
    ok &= saw_pass && saw_fail && wrap_has_inverses;

    println!();
    println!("{}", if ok { "P3 WORKS" } else { "P3 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
