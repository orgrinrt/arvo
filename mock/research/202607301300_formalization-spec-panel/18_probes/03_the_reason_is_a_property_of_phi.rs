//! PROBE 3: the laws are properties of the recovery map, derivable once, and the
//! one law implementation the design has actually written down is stated over the
//! wrong domain.
//!
//! THREE THINGS MEASURED, IN ORDER
//!
//! A. Classify each recovery map `phi` structurally: is it a HOMOMORPHISM (it
//!    commutes with the exact operation), a PARTIAL IDENTITY (wherever it returns,
//!    it returns its argument unchanged), a RETRACTION (total, order-preserving,
//!    fixing the representable set), or none of those.
//!
//! B. Check that each structural class implies exactly the law probe 1 measured,
//!    at every arity, WITHOUT any per-arity search. If it does, then two rows of
//!    `11_current_shape_draft.md:826-828`'s ledger move from "machine-checked by
//!    bounded exhaustion at a model width" to "true by construction at every
//!    width", and the width-uniformity transfer argument the draft records as
//!    permanently unmechanised (`11_current_shape_draft.md:840-842`) stops being
//!    load-bearing for them.
//!
//!    The partial-identity case is checked over ARBITRARY subsets, not intervals,
//!    because if the theorem needs the interval shape it is a weaker theorem than
//!    the one worth stating.
//!
//! C. The `Monotone` impl. The design's only written law implementation is
//!    `mock/design_rounds/202607301100_topic.the-formalization-talk.md:1113`:
//!
//!        impl<T: Direction> Monotone for (TowardNegative, T, TowardPositive) {}
//!
//!    quantified over the three MIDPOINT positions of a five-member quantiser. The
//!    two range positions are not mentioned. This section holds the triple fixed at
//!    exactly the rows that impl admits and varies only the range members.
//!
//! Build:  rustc -O 03_the_reason_is_a_property_of_phi.rs -o p3 && ./p3

#![allow(clippy::needless_range_loop)]

// ============================================================ shared: shapes

fn schedules(n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for i in 0..n - 1 {
        for mut rest in schedules(n - 1) {
            let mut s = vec![i];
            s.append(&mut rest);
            out.push(s);
        }
    }
    out
}

fn tree_shapes(n: usize) -> Vec<Vec<usize>> {
    let mut seen: Vec<String> = Vec::new();
    let mut out = Vec::new();
    for s in schedules(n) {
        let mut syms: Vec<String> = (0..n).map(|i| format!("{i}")).collect();
        for &i in &s {
            let r = syms.remove(i + 1);
            let l = syms.remove(i);
            syms.insert(i, format!("({l}.{r})"));
        }
        if !seen.contains(&syms[0]) {
            seen.push(syms[0].clone());
            out.push(s);
        }
    }
    out
}

// =========================================================== A: classify phi

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Rule {
    Wrap,
    Saturate,
    Refuse,
    SubZero,
}

const RULES: [(Rule, &str); 4] = [
    (Rule::Wrap, "Wrap      (Hot)"),
    (Rule::Saturate, "Saturate  (Warm/Cold)"),
    (Rule::Refuse, "Refuse    (Precise)"),
    (Rule::SubZero, "SubstituteZero"),
];

/// The recovery map itself, from an exact value to a representable one.
fn phi(r: Rule, x: i64, lo: i64, hi: i64) -> Option<i64> {
    let span = hi - lo + 1;
    match r {
        Rule::Wrap => Some(lo + (((x - lo) % span) + span) % span),
        Rule::Saturate => Some(x.clamp(lo, hi)),
        Rule::Refuse => {
            if x < lo || x > hi {
                None
            } else {
                Some(x)
            }
        }
        Rule::SubZero => {
            if x < lo || x > hi {
                Some(0)
            } else {
                Some(x)
            }
        }
    }
}

/// `phi` commutes with exact addition: phi(x + y) == phi(phi x + phi y),
/// for every exact pair. This is the two-sided form of the draft's
/// translation-stability identity and is exactly "phi is a homomorphism".
fn is_homomorphism(r: Rule, lo: i64, hi: i64, ex: i64) -> bool {
    for x in -ex..=ex {
        for y in -ex..=ex {
            let l = phi(r, x + y, lo, hi);
            let rr = match (phi(r, x, lo, hi), phi(r, y, lo, hi)) {
                (Some(a), Some(b)) => phi(r, a + b, lo, hi),
                _ => None,
            };
            if l != rr {
                return false;
            }
        }
    }
    true
}

/// `phi` returns its argument unchanged wherever it returns at all.
fn is_partial_identity(r: Rule, lo: i64, hi: i64, ex: i64) -> bool {
    (-ex..=ex).all(|x| match phi(r, x, lo, hi) {
        None => true,
        Some(v) => v == x,
    })
}

/// total, fixes the representable set pointwise, and order-preserving.
fn is_retraction(r: Rule, lo: i64, hi: i64, ex: i64) -> bool {
    if (-ex..=ex).any(|x| phi(r, x, lo, hi).is_none()) {
        return false;
    }
    if (lo..=hi).any(|a| phi(r, a, lo, hi) != Some(a)) {
        return false;
    }
    for x in -ex..ex {
        let (a, b) = (phi(r, x, lo, hi).unwrap(), phi(r, x + 1, lo, hi).unwrap());
        if a > b {
            return false;
        }
    }
    true
}

// ========================== B: does the class imply the law, at every arity?

/// General partial fold: `dom` decides where the operation is defined, and the
/// value returned where it is defined is always the exact sum.
fn fold_restricted(xs: &[i64], sched: &[usize], dom: &dyn Fn(i64) -> bool) -> Option<i64> {
    let mut v: Vec<Option<i64>> = xs.iter().map(|&x| Some(x)).collect();
    for &i in sched {
        let b = v.remove(i + 1);
        let a = v.remove(i);
        let c = match (a, b) {
            (Some(a), Some(b)) => {
                let s = a + b;
                if dom(s) {
                    Some(s)
                } else {
                    None
                }
            }
            _ => None,
        };
        v.insert(i, c);
    }
    v[0]
}

fn fold_rule(r: Rule, xs: &[i64], sched: &[usize], lo: i64, hi: i64) -> Option<i64> {
    let mut v: Vec<Option<i64>> = xs.iter().map(|&x| Some(x)).collect();
    for &i in sched {
        let b = v.remove(i + 1);
        let a = v.remove(i);
        let c = match (a, b) {
            (Some(a), Some(b)) => phi(r, a + b, lo, hi),
            _ => None,
        };
        v.insert(i, c);
    }
    v[0]
}

struct Law {
    kleene: bool,
    existential: bool,
}

fn law_of_rule(r: Rule, n: usize, lo: i64, hi: i64) -> Law {
    let shapes = tree_shapes(n);
    let mut out = Law {
        kleene: true,
        existential: true,
    };
    let mut idx = vec![lo; n];
    loop {
        let vals: Vec<Option<i64>> = shapes
            .iter()
            .map(|s| fold_rule(r, &idx, s, lo, hi))
            .collect();
        if vals.iter().any(|v| *v != vals[0]) {
            out.kleene = false;
        }
        let d: Vec<i64> = vals.iter().filter_map(|v| *v).collect();
        if d.iter().any(|v| *v != d[0]) {
            out.existential = false;
        }
        let mut k = 0;
        loop {
            if k == n {
                return out;
            }
            idx[k] += 1;
            if idx[k] <= hi {
                break;
            }
            idx[k] = lo;
            k += 1;
        }
    }
}

/// Same measurement for an ARBITRARY partial-identity domain, so the theorem is
/// not secretly a fact about intervals.
fn law_of_subset(mask: u32, n: usize, lo: i64, hi: i64) -> Law {
    let shapes = tree_shapes(n);
    let dom = move |s: i64| -> bool {
        if s < lo || s > hi {
            return false;
        }
        (mask >> ((s - lo) as u32)) & 1 == 1
    };
    let mut out = Law {
        kleene: true,
        existential: true,
    };
    let mut idx = vec![lo; n];
    loop {
        // Only feed inputs that are themselves in the domain, since a numeral's
        // own values are representable by definition.
        if idx.iter().all(|&x| dom(x)) {
            let vals: Vec<Option<i64>> = shapes
                .iter()
                .map(|s| fold_restricted(&idx, s, &dom))
                .collect();
            if vals.iter().any(|v| *v != vals[0]) {
                out.kleene = false;
            }
            let d: Vec<i64> = vals.iter().filter_map(|v| *v).collect();
            if d.iter().any(|v| *v != d[0]) {
                out.existential = false;
            }
        }
        let mut k = 0;
        loop {
            if k == n {
                return out;
            }
            idx[k] += 1;
            if idx[k] <= hi {
                break;
            }
            idx[k] = lo;
            k += 1;
        }
    }
}

// ====================================================== C: the Monotone impl

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    TowardNegative,
    TowardPositive,
    ToEven,
    ToOdd,
    AwayFromZero,
    TowardZero,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum RangeRule {
    Clamp,
    ReduceModulo,
    SubstituteZero,
    Refuse,
}

const Q: i64 = 4; // quantum, so the midpoint triple has something to decide
const NLO: i64 = -8; // least representable
const NHI: i64 = 8; // greatest representable

fn resolve(d: Dir, n0: i64, n1: i64) -> i64 {
    match d {
        Dir::TowardNegative => n0,
        Dir::TowardPositive => n1,
        Dir::TowardZero => {
            if n0.abs() <= n1.abs() {
                n0
            } else {
                n1
            }
        }
        Dir::AwayFromZero => {
            if n0.abs() >= n1.abs() {
                n0
            } else {
                n1
            }
        }
        Dir::ToEven => {
            if (n0 / Q).rem_euclid(2) == 0 {
                n0
            } else {
                n1
            }
        }
        Dir::ToOdd => {
            if (n0 / Q).rem_euclid(2) == 1 {
                n0
            } else {
                n1
            }
        }
    }
}

/// A full five-member quantiser, exactly the shape of `Quantisation` at
/// `11_current_shape_draft.md:212-219`.
#[derive(Copy, Clone)]
struct Quantiser {
    below: Dir,
    at: Dir,
    above: Dir,
    over: RangeRule,
    under: RangeRule,
}

fn quantise(q: Quantiser, x: i64) -> Option<i64> {
    if x > NHI {
        return match q.over {
            RangeRule::Clamp => Some(NHI),
            RangeRule::SubstituteZero => Some(0),
            RangeRule::Refuse => None,
            RangeRule::ReduceModulo => {
                let span = NHI - NLO + Q;
                let y = NLO + (((x - NLO) % span) + span) % span;
                quantise_in_range(q, y.min(NHI))
            }
        };
    }
    if x < NLO {
        return match q.under {
            RangeRule::Clamp => Some(NLO),
            RangeRule::SubstituteZero => Some(0),
            RangeRule::Refuse => None,
            RangeRule::ReduceModulo => {
                let span = NHI - NLO + Q;
                let y = NLO + (((x - NLO) % span) + span) % span;
                quantise_in_range(q, y.min(NHI))
            }
        };
    }
    quantise_in_range(q, x)
}

fn quantise_in_range(q: Quantiser, x: i64) -> Option<i64> {
    let n0 = (x as f64 / Q as f64).floor() as i64 * Q;
    let n1 = (n0 + Q).min(NHI);
    let r = x - n0;
    let mid = Q / 2;
    let d = if r < mid {
        q.below
    } else if r == mid {
        q.at
    } else {
        q.above
    };
    Some(resolve(d, n0, n1))
}

/// Order-preserving over the exact domain, ignoring pairs where either side
/// refuses (the existential reading, which is the generous one).
fn quantiser_is_monotone(q: Quantiser, ex: i64) -> Option<(i64, i64, i64, i64)> {
    for x in -ex..ex {
        for y in (x + 1)..=ex {
            if let (Some(a), Some(b)) = (quantise(q, x), quantise(q, y)) {
                if a > b {
                    return Some((x, y, a, b));
                }
            }
        }
    }
    None
}

// ==================================================================== driver

fn main() {
    let (lo, hi, ex) = (-4i64, 3i64, 24i64);

    println!("=================================================================");
    println!("A. WHAT KIND OF MAP IS EACH RECOVERY RULE, signed [{lo},{hi}]");
    println!("=================================================================\n");
    println!(
        "{:<24}{:>16}{:>20}{:>14}",
        "rule", "homomorphism", "partial identity", "retraction"
    );
    for (r, name) in RULES {
        println!(
            "{:<24}{:>16}{:>20}{:>14}",
            name,
            is_homomorphism(r, lo, hi, ex),
            is_partial_identity(r, lo, hi, ex),
            is_retraction(r, lo, hi, ex)
        );
    }

    println!();
    println!("=================================================================");
    println!("B. DOES THE CLASS PREDICT THE LAW, at every arity");
    println!("=================================================================\n");
    println!(
        "{:<24}{:>4}{:>10}{:>14}",
        "rule", "n", "Kleene", "existential"
    );
    for (r, name) in RULES {
        for n in 2..=6 {
            let l = law_of_rule(r, n, lo, hi);
            println!(
                "{:<24}{:>4}{:>10}{:>14}",
                if n == 2 { name } else { "" },
                n,
                l.kleene,
                l.existential
            );
        }
        println!();
    }

    println!("and the partial-identity theorem over ARBITRARY domains, not intervals:");
    println!("every subset of [{lo},{hi}] as the defined region, n = 2 to 5\n");
    let width = (hi - lo + 1) as u32;
    let total_subsets = 1u32 << width;
    let mut checked = 0usize;
    let mut exist_all = true;
    let mut kleene_any = 0usize;
    for mask in 0..total_subsets {
        for n in 2..=5 {
            let l = law_of_subset(mask, n, lo, hi);
            if !l.existential {
                exist_all = false;
            }
            if l.kleene {
                kleene_any += 1;
            }
            checked += 1;
        }
    }
    println!("  subsets tried:                              {total_subsets}");
    println!("  (subset, arity) pairs checked:              {checked}");
    println!("  every defined grouping agreed, every time:  {exist_all}");
    println!("  of those pairs, also Kleene-associative:    {kleene_any}");

    println!();
    println!("=================================================================");
    println!("C. THE `Monotone` BLANKET IMPL'S DOMAIN");
    println!();
    println!("   design_rounds/202607301100_topic.the-formalization-talk.md:1113");
    println!("     impl<T: Direction> Monotone for (TowardNegative, T, TowardPositive)");
    println!();
    println!("   The triple below is held at exactly what that impl admits. Only the");
    println!("   two range members vary, and they are the members the impl does not");
    println!("   mention. Quantum {Q}, representable {{{NLO}, .., {NHI}}}.");
    println!("=================================================================\n");
    println!(
        "{:<18}{:<18}{:<16}{:>12}   {}",
        "over-range", "under-range", "midpoint tie", "monotone", "counterexample"
    );
    for tie in [Dir::ToEven, Dir::ToOdd, Dir::AwayFromZero] {
        for (over, under) in [
            (RangeRule::Clamp, RangeRule::Clamp),
            (RangeRule::ReduceModulo, RangeRule::ReduceModulo),
            (RangeRule::SubstituteZero, RangeRule::SubstituteZero),
            (RangeRule::Refuse, RangeRule::Refuse),
            (RangeRule::Clamp, RangeRule::ReduceModulo),
        ] {
            let q = Quantiser {
                below: Dir::TowardNegative,
                at: tie,
                above: Dir::TowardPositive,
                over,
                under,
            };
            let m = quantiser_is_monotone(q, 24);
            println!(
                "{:<18}{:<18}{:<16}{:>12}   {}",
                format!("{over:?}"),
                format!("{under:?}"),
                format!("{tie:?}"),
                if m.is_none() { "yes" } else { "NO" },
                match m {
                    None => String::new(),
                    Some((x, y, a, b)) => format!("phi({x})={a} > phi({y})={b}, and {x} < {y}"),
                }
            );
        }
    }

    println!();
    println!("=================================================================");
    println!("READINGS");
    println!("=================================================================");
    println!();
    println!("A/B. Each law measured in probe 1 is predicted by a structural property");
    println!("     of `phi` alone. A homomorphism gives Kleene associativity at every");
    println!("     arity because every grouping equals phi of the exact sum. A partial");
    println!("     identity gives existential associativity at every arity because every");
    println!("     grouping that returns IS the exact sum. Neither argument mentions a");
    println!("     width, an arity, or a search, so neither needs one.");
    println!();
    println!("     And the partial-identity result holds over every subset, not only");
    println!("     intervals, so `Precise`'s law is not a fact about ranges. Probe 2");
    println!("     showed the binary law does not lift; this shows what does the lifting,");
    println!("     and it is a property the design can read off a constructor.");
    println!();
    println!("C.   The impl asserts a five-member property from a three-member premise.");
    println!("     Holding the triple at exactly the rows that impl admits, monotonicity");
    println!("     flips on the range members it never looks at. The impl is not");
    println!("     imprecise. It is false for compositions it admits, and nothing checks");
    println!("     it, because a marker impl with no associated items has no body to be");
    println!("     wrong in. That is the same defect file 17 section 4.3 found on the");
    println!("     fidelity licence, arriving here on the only law the design has");
    println!("     actually written down.");
}
