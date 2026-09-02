//! PROBE 1: separate the equality from the quantifiers, and measure each
//! resolution against all three equalities at every arity from 2 to 6.
//!
//! WHAT THE DESIGN CURRENTLY SAYS
//!
//! `11_current_shape_draft.md:266-270` states its law under Kleene equality
//! ("both refuse, or both return and agree") and reports `Refuse` as unstable
//! two-sided. File 17 measured that `Refuse`'s regrouping diameter is 0 and its
//! entire disagreement is definedness, and proposed "partially associative" as
//! the missing name.
//!
//! There are three relations on partial values in ordinary use and they are not
//! interchangeable. Written for `x`, `y` in `V + {undefined}`:
//!
//!   KLEENE (strong) equality      x ~= y  :  both undefined, or both defined
//!                                            and equal.  An equivalence relation.
//!   EXISTENTIAL (weak) equality   x =e y  :  both defined and equal.  Symmetric
//!                                            and transitive but NOT reflexive,
//!                                            so it is a partial equivalence.
//!   REFINEMENT order              x <= y  :  x undefined, or both defined and
//!                                            equal.  A partial order, and Kleene
//!                                            equality is exactly `<=` both ways.
//!
//! A law stated under each of these says something different, and the difference
//! is the whole of what `Precise` needs.
//!
//! AND THERE ARE TWO QUANTIFIERS THE DESIGN NEVER WRITES DOWN
//!
//!   ARITY: the binary law, or "every grouping of an n-element fold agrees"?
//!          Probe 2 shows these are not the same statement for partial operations.
//!   ACCUMULATOR: `(a . b) . c` is only the law's subject if the intermediate is
//!          held at the SAME numeral as the operands. A fold whose accumulator is
//!          wider is a different operation with a different law, and nothing in
//!          the draft's ten axes names the accumulator at all.
//!
//! Sections 3 and 4 below measure the accumulator quantifier directly, because it
//! is the one that moves the answer for `Warm` and `Cold`.
//!
//! Build:  rustc -O 01_three_equalities_and_two_quantifiers.rs -o p1 && ./p1

#![allow(clippy::needless_range_loop)]

// -------------------------------------------------------------- tree shapes

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

/// Catalan-many distinct groupings, deduplicated by parenthesisation.
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

/// The left-nested grouping, which is what a plain sequential fold computes.
fn left_nested(n: usize) -> Vec<usize> {
    vec![0; n - 1]
}

// ------------------------------------------------------------- resolutions

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

/// One binary step with the result landed back into `[lo, hi]`.
fn step(r: Rule, a: i64, b: i64, lo: i64, hi: i64) -> Option<i64> {
    let x = a + b;
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

/// Fold under one grouping, with the accumulator held at `[alo, ahi]` and the
/// final settle back into `[lo, hi]`. When `alo == lo` and `ahi == hi` this is
/// the ordinary same-width fold the draft's law is implicitly about.
fn fold(r: Rule, xs: &[i64], sched: &[usize], lo: i64, hi: i64, alo: i64, ahi: i64) -> Option<i64> {
    let mut v: Vec<Option<i64>> = xs.iter().map(|&x| Some(x)).collect();
    for &i in sched {
        let b = v.remove(i + 1);
        let a = v.remove(i);
        let c = match (a, b) {
            (Some(a), Some(b)) => step(r, a, b, alo, ahi),
            _ => None,
        };
        v.insert(i, c);
    }
    match v[0] {
        None => None,
        Some(x) => step(r, x, 0, lo, hi), // settle: quantise into the numeral once
    }
}

// ------------------------------------------------------------------ verdict

#[derive(Default)]
struct Verdict {
    kleene: bool,
    existential: bool,
    left_refines_all: bool,
    all_refine_left: bool,
    diameter: i64,
    defined_disagreements: usize,
    inputs: usize,
}

fn measure(r: Rule, n: usize, lo: i64, hi: i64, acc_scale: i64) -> Verdict {
    let shapes = tree_shapes(n);
    let ln = left_nested(n);
    // Accumulator window: scale 1 is the operand numeral itself; larger scales
    // widen the intermediate. `i64::MAX`-ish scale stands for an exact accumulator.
    let (alo, ahi) = (lo * acc_scale, hi * acc_scale);

    let mut v = Verdict {
        kleene: true,
        existential: true,
        left_refines_all: true,
        all_refine_left: true,
        ..Default::default()
    };

    let mut idx = vec![lo; n];
    loop {
        v.inputs += 1;
        let left = fold(r, &idx, &ln, lo, hi, alo, ahi);
        let vals: Vec<Option<i64>> = shapes
            .iter()
            .map(|s| fold(r, &idx, s, lo, hi, alo, ahi))
            .collect();

        let defined: Vec<i64> = vals.iter().filter_map(|x| *x).collect();
        let any_undef = vals.iter().any(|x| x.is_none());

        if any_undef && !defined.is_empty() {
            v.defined_disagreements += 1;
        }
        // Kleene: every grouping identical as an Option.
        if vals.iter().any(|x| *x != vals[0]) {
            v.kleene = false;
        }
        // Existential: every DEFINED grouping equal.
        if defined.iter().any(|x| *x != defined[0]) {
            v.existential = false;
        }
        // Refinement, both directions, against the sequential fold.
        for g in &vals {
            // left <= g
            if let Some(l) = left {
                if *g != Some(l) {
                    v.left_refines_all = false;
                }
            }
            // g <= left
            if let Some(gv) = *g {
                if left != Some(gv) {
                    v.all_refine_left = false;
                }
            }
        }
        if !defined.is_empty() {
            let gap = defined.iter().max().unwrap() - defined.iter().min().unwrap();
            if gap > v.diameter {
                v.diameter = gap;
            }
        }

        let mut k = 0;
        loop {
            if k == n {
                return v;
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

fn yn(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "NO"
    }
}

fn main() {
    let (lo, hi) = (-4i64, 3i64);

    println!("=================================================================");
    println!("1. THE THREE EQUALITIES, at every arity, signed [{lo},{hi}]");
    println!("   accumulator held at the operand numeral (scale 1), which is the");
    println!("   reading the draft's law is implicitly about");
    println!("=================================================================\n");
    println!(
        "{:<24}{:>4}{:>9}{:>13}{:>10}{:>10}{:>10}{:>12}",
        "rule", "n", "Kleene", "existential", "L<=all", "all<=L", "diameter", "def.disagree"
    );
    for (r, name) in RULES {
        for n in 2..=6 {
            let v = measure(r, n, lo, hi, 1);
            println!(
                "{:<24}{:>4}{:>9}{:>13}{:>10}{:>10}{:>10}{:>12}",
                if n == 2 { name } else { "" },
                n,
                yn(v.kleene),
                yn(v.existential),
                yn(v.left_refines_all),
                yn(v.all_refine_left),
                v.diameter,
                v.defined_disagreements
            );
        }
        println!();
    }

    println!("=================================================================");
    println!("2. THE SAME, UNSIGNED [0,7]");
    println!("=================================================================\n");
    println!(
        "{:<24}{:>4}{:>9}{:>13}{:>10}{:>10}{:>10}{:>12}",
        "rule", "n", "Kleene", "existential", "L<=all", "all<=L", "diameter", "def.disagree"
    );
    for (r, name) in RULES {
        for n in [2usize, 5] {
            let v = measure(r, n, 0, 7, 1);
            println!(
                "{:<24}{:>4}{:>9}{:>13}{:>10}{:>10}{:>10}{:>12}",
                if n == 2 { name } else { "" },
                n,
                yn(v.kleene),
                yn(v.existential),
                yn(v.left_refines_all),
                yn(v.all_refine_left),
                v.diameter,
                v.defined_disagreements
            );
        }
        println!();
    }

    println!("=================================================================");
    println!("3. THE ACCUMULATOR QUANTIFIER: the same operand numeral, four");
    println!("   accumulator widths, n = 5, signed [{lo},{hi}]");
    println!();
    println!("   scale 1 holds the running value at the operand numeral. scale 5");
    println!("   is wide enough that no intermediate of a 5-element fold can leave");
    println!("   it, so the only quantisation is the single settle at the end.");
    println!("=================================================================\n");
    println!(
        "{:<24}{:>8}{:>9}{:>13}{:>10}{:>12}",
        "rule", "acc", "Kleene", "existential", "diameter", "def.disagree"
    );
    for (r, name) in RULES {
        for scale in [1i64, 2, 3, 5] {
            let v = measure(r, 5, lo, hi, scale);
            println!(
                "{:<24}{:>8}{:>9}{:>13}{:>10}{:>12}",
                if scale == 1 { name } else { "" },
                format!("x{scale}"),
                yn(v.kleene),
                yn(v.existential),
                v.diameter,
                v.defined_disagreements
            );
        }
        println!();
    }

    println!("=================================================================");
    println!("READINGS");
    println!("=================================================================");
    println!();
    println!("A. The three relations sort the four resolutions into four classes,");
    println!("   and no single relation separates them. Any law text that names one");
    println!("   relation and stops is describing one of four columns.");
    println!();
    println!("B. `Refuse` is existentially associative at EVERY arity measured, and");
    println!("   Kleene-associative at none. Probe 2 shows that is not an instance of");
    println!("   the binary law: for partial operations in general the binary law does");
    println!("   not lift. So the reason `Refuse` lifts is something else, and probe 3");
    println!("   names it.");
    println!();
    println!("C. Neither direction of the refinement order holds for `Refuse`. The");
    println!("   sequential fold is neither always the most defined grouping nor always");
    println!("   the least, so 'the parallel answer refines the sequential one' is not");
    println!("   available as the contract, and a combinator cannot promise it.");
    println!();
    println!("D. The accumulator is a quantifier, not a detail. Signed saturating goes");
    println!("   from diameter 7 at scale 1 to diameter 0 at scale 5 with no axis");
    println!("   changed, so `Warm`'s associativity verdict is not a fact about `Warm`.");
    println!("   It is a fact about a (numeral, accumulator) pair the design does not");
    println!("   currently name in either the axis table or the law key.");
}
