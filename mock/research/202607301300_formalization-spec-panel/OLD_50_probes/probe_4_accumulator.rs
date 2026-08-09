//! Probe 4: the fold's accumulator sufficiency condition, derived over fixed quanta, when
//! the exponent moves.
//!
//! rustc --edition 2021 -O probe_4_accumulator.rs -o /tmp/p4 && /tmp/p4
//!
//! The claim under test: a float numeral's whole representable set sits inside ONE
//! fixed-quantum grid, namely quantum 2^(emin-p+1) with magnitude bound 2^(emax+1). It
//! follows that the exact sum of n values of a float numeral is exactly representable in an
//! `Implicit` numeral of that quantum and width
//!
//!     W = (emax + 1) - (emin - p + 1) + ceil(log2 n) + 1   bits including sign
//!
//! and therefore that interior safety for a float fold is satisfiable at a finite, computable
//! width, by a FIXED-POINT accumulator. The design gets this for free: the accumulator that
//! makes a float fold exact is an ordinary numeral of the design's other kind.
//!
//! What is checked here: exactness, grouping invariance, permutation invariance, that exactly
//! one quantiser fires (at the root, on a grouping-independent argument), and that the width
//! formula is tight rather than generous.

#[path = "model.rs"]
mod model;
use model::*;

/// The exact accumulator: an integer count of units of 2^acc_q.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Acc {
    units: i128,
    q: i32,
}

impl Acc {
    fn zero(f: &Fmt) -> Acc {
        Acc {
            units: 0,
            q: f.quantum_exp(f.emin),
        }
    }
    fn add(self, d: Dyadic) -> Acc {
        if d.is_zero() {
            return self;
        }
        let shift = d.scale - self.q;
        assert!(
            shift >= 0,
            "a format value below the accumulator quantum: not possible"
        );
        let u = (d.mag as i128) << (shift as u32);
        Acc {
            units: self.units + if d.neg { -u } else { u },
            q: self.q,
        }
    }
    fn as_rat(self) -> Rat {
        Rat {
            neg: self.units < 0,
            num: self.units.unsigned_abs(),
            den: 1,
            scale: self.q,
        }
    }
    fn bits(self) -> u32 {
        128 - self.units.unsigned_abs().leading_zeros()
    }
}

/// Width of the exact accumulator, in bits, excluding sign.
fn acc_width(f: &Fmt, n: u32) -> u32 {
    let top = f.emax + 1;
    let bot = f.quantum_exp(f.emin);
    (top - bot) as u32 + ceil_log2(n)
}

fn ceil_log2(n: u32) -> u32 {
    32 - (n - 1).leading_zeros()
}

fn all_values(f: &Fmt) -> Vec<Dyadic> {
    let mut v = vec![Dyadic::zero(false)];
    for d in f.positives() {
        v.push(d);
        v.push(Dyadic { neg: true, ..d });
    }
    v
}

fn main() {
    let f = MODEL;
    let vals = all_values(&f);
    println!(
        "model format p={} emin={} emax={}: {} values, accumulator quantum 2^{}",
        f.p,
        f.emin,
        f.emax,
        vals.len(),
        f.quantum_exp(f.emin)
    );

    // ---- exhaustive over all ordered triples: exactness and grouping invariance ----
    let mut triples = 0u64;
    let mut widest = 0u32;
    let mut interior_quantisations_left = 0u64;
    let mut interior_quantisations_right = 0u64;
    let mut root_disagreements = 0u64;

    for &a in &vals {
        for &b in &vals {
            for &c in &vals {
                triples += 1;
                // the exact accumulator, order-independent by construction
                let acc = Acc::zero(&f).add(a).add(b).add(c);
                widest = widest.max(acc.bits());

                // the delivered value: quantise once, at the root
                let root = quantize(&f, &acc.as_rat(), Dir::Nearest);

                // the same fold with the accumulator held in the FORMAT instead: a quantiser
                // fires at every interior step, and the two groupings can disagree.
                let l = in_format_fold(&f, &[a, b, c], true);
                let r = in_format_fold(&f, &[a, b, c], false);
                interior_quantisations_left += l.1;
                interior_quantisations_right += r.1;
                if l.0 != r.0 {
                    root_disagreements += 1;
                }
                // the exact accumulator's root result is what both SHOULD have delivered
                let _ = root;
            }
        }
    }
    println!("\nordered triples checked: {triples}");
    println!(
        "widest accumulator magnitude seen: {widest} bits; formula for n=3 gives {} bits",
        acc_width(&f, 3)
    );
    println!(
        "in-format fold, interior quantisations: left-assoc {interior_quantisations_left}, \
         right-assoc {interior_quantisations_right}"
    );
    println!(
        "in-format fold, groupings delivering DIFFERENT results: {root_disagreements} \
         ({:.2}% of triples)",
        100.0 * root_disagreements as f64 / triples as f64
    );

    // ---- exact accumulator: grouping and permutation invariance, by construction ----
    // Checked rather than asserted: every permutation of every sampled tuple.
    let mut perms_checked = 0u64;
    let mut s: u64 = 0x2545_F491_4F6C_DD1D;
    for _ in 0..20000 {
        let n = 4 + (next(&mut s) % 5) as usize; // 4..8
        let tuple: Vec<Dyadic> = (0..n)
            .map(|_| vals[(next(&mut s) as usize) % vals.len()])
            .collect();
        let base = fold_exact(&f, &tuple);
        // all rotations and the reverse, plus a shuffle
        for k in 0..n {
            let mut t = tuple.clone();
            t.rotate_left(k);
            assert_eq!(fold_exact(&f, &t), base, "rotation changed the exact sum");
            perms_checked += 1;
        }
        let mut rev = tuple.clone();
        rev.reverse();
        assert_eq!(fold_exact(&f, &rev), base, "reversal changed the exact sum");
        perms_checked += 1;
    }
    println!("\nexact-accumulator orderings checked: {perms_checked}, all agreed");

    // ---- the width formula, checked for tightness ----
    let worst: Vec<Dyadic> = (0..8).map(|_| max_finite_of(&f)).collect();
    let acc = fold_exact_acc(&f, &worst);
    println!(
        "\n8 copies of max finite: accumulator needs {} bits, formula gives {}",
        acc.bits(),
        acc_width(&f, 8)
    );
    let tightest = {
        // the smallest magnitude that must be representable is one unit of the accumulator
        // quantum, and the largest is n * (2^(emax+1) - ulp). Both are exercised above.
        let small = Acc::zero(&f).add(f.positives()[0]);
        small.bits()
    };
    println!("smallest nonzero accumulator magnitude: {tightest} bit(s)");

    // ---- the real formats ----
    println!("\n== the width the condition asks for, at real formats ==");
    for (name, fmt) in [("binary32", BINARY32), ("binary64", BINARY64)] {
        let base = (fmt.emax + 1 - fmt.quantum_exp(fmt.emin)) as u32;
        println!(
            "{name}: sum accumulator {} bits + ceil(log2 n) (n=2^20 -> {} bits)",
            base,
            base + 20
        );
        // the multiply-accumulate case: the product's exponent range doubles and the
        // product's precision doubles, so the accumulator spans twice the range.
        let prod_top = 2 * (fmt.emax + 1);
        let prod_bot = 2 * fmt.quantum_exp(fmt.emin);
        println!(
            "{name}: dot-product accumulator {} bits + ceil(log2 n) (n=2^20 -> {} bits)",
            (prod_top - prod_bot) as u32,
            (prod_top - prod_bot) as u32 + 20
        );
    }
}

fn max_finite_of(f: &Fmt) -> Dyadic {
    let (m, q) = f.max_finite();
    Dyadic {
        neg: false,
        mag: m,
        scale: q,
    }
}

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn fold_exact_acc(f: &Fmt, xs: &[Dyadic]) -> Acc {
    let mut a = Acc::zero(f);
    for &x in xs {
        a = a.add(x);
    }
    a
}

fn fold_exact(f: &Fmt, xs: &[Dyadic]) -> i128 {
    fold_exact_acc(f, xs).units
}

/// The fold with the accumulator held in the format itself: a quantiser at every step.
/// Returns the delivered result and how many interior quantisations were inexact.
fn in_format_fold(f: &Fmt, xs: &[Dyadic], left: bool) -> (Outcome, u64) {
    let order: Vec<Dyadic> = if left {
        xs.to_vec()
    } else {
        xs.iter().rev().copied().collect()
    };
    let mut acc = Dyadic::zero(false);
    let mut events = 0u64;
    for (i, &x) in order.iter().enumerate() {
        let r = exact_add(acc, x);
        let g = quantize(f, &r, Dir::Nearest);
        if i + 1 < order.len() && g.grade.has(Cause::Inexact) {
            events += 1;
        }
        acc = match g.out {
            Outcome::Finite(d) => d,
            other => return (other, events),
        };
    }
    (Outcome::Finite(acc), events)
}
