//! PROBE 7: a law is the bottom rung of a graded fact, and the grade is the
//! DIAMETER of the set of answers a rewrite can reach.
//!
//! Two things this dive has established separately and neither has connected:
//!
//!   McSherry: translation stability is sound and 1024-to-1 over-strict; one
//!   resolution in 65536 is stable, 1024 are genuinely fold-associative, and
//!   "only `Hot` folds for signed values" is an artifact of deriving the fold
//!   law from a strictly stronger property (`13_mcsherry...md:210-244`).
//!
//!   Willsey and McSherry: `arvo-spectral/src/power.rs:71` is arvo's one real
//!   sequential fold over `+`, over a float, so an associativity gate refuses
//!   it at every strategy (`13_mcsherry...md:490-496`).
//!
//! Both are the same complaint: a BOOLEAN law is the wrong instrument. A
//! boolean can only say "every regrouping agrees" or "some regrouping does
//! not", and both `arvo-spectral`'s float fold and a four-way accumulator split
//! sit in the second bucket while being perfectly reasonable things to do.
//!
//! The proposal under test: instead of asking "is this operation associative",
//! ask "over all C(n) groupings of an n-element fold, how far apart can two
//! answers be". Call that number the operation's REGROUPING DIAMETER at that
//! input. Associativity is exactly diameter 0. A combinator then bounds on a
//! diameter budget rather than on a boolean, which is a strictly finer
//! instrument that never states anything false.
//!
//! What is measured here: the diameter over every one of the 14 binary trees on
//! 5 leaves, exhaustively over a fixed-point model and structurally over a
//! float model.
//!
//! Build:  rustc -O 07_the_grade_is_the_diameter.rs -o p7 && ./p7

#![allow(dead_code)]

// ---------------------------------------------------------------- groupings

/// Every binary tree shape on `n` leaves, as a list of reduction schedules.
/// A schedule is a sequence of "combine positions i and i+1" instructions, so
/// applying it to a slice of n values yields one value.
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

/// All DISTINCT groupings, i.e. the Catalan-many tree shapes rather than the
/// (n-1)! schedules, deduplicated by the answer-independent tree they induce.
/// Deduplication is by the parenthesisation string.
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

// ------------------------------------------------------------- fixed models

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Rule {
    Wrap,
    Saturate,
    Refuse,
    Exact,
}

fn add_rule(r: Rule, a: i64, b: i64, lo: i64, hi: i64) -> Option<i64> {
    let x = a + b;
    let span = hi - lo + 1;
    match r {
        Rule::Exact => Some(x),
        Rule::Wrap => Some(lo + (((x - lo) % span) + span) % span),
        Rule::Saturate => Some(x.clamp(lo, hi)),
        Rule::Refuse => {
            if x < lo || x > hi {
                None
            } else {
                Some(x)
            }
        }
    }
}

fn fold_int(r: Rule, xs: &[i64], sched: &[usize], lo: i64, hi: i64) -> Option<i64> {
    let mut v: Vec<Option<i64>> = xs.iter().map(|&x| Some(x)).collect();
    for &i in sched {
        let b = v.remove(i + 1);
        let a = v.remove(i);
        let c = match (a, b) {
            (Some(a), Some(b)) => add_rule(r, a, b, lo, hi),
            _ => None,
        };
        v.insert(i, c);
    }
    v[0]
}

fn fold_f64(xs: &[f64], sched: &[usize]) -> f64 {
    let mut v: Vec<f64> = xs.to_vec();
    for &i in sched {
        let b = v.remove(i + 1);
        let a = v.remove(i);
        v.insert(i, a + b);
    }
    v[0]
}

// ------------------------------------------------------------------ measure

struct Diam {
    /// Largest gap, over all inputs, between two groupings that both returned.
    max_gap: i64,
    /// Inputs where one grouping returned and another refused. Under Kleene
    /// equality these are a disagreement with no numeric distance at all.
    disagree_defined: usize,
    /// How many distinct answers the worst input reached.
    worst_card: usize,
    worst_input: Vec<i64>,
}

fn diameter_int(r: Rule, n: usize, lo: i64, hi: i64) -> Diam {
    let shapes = tree_shapes(n);
    let mut d = Diam {
        max_gap: 0,
        disagree_defined: 0,
        worst_card: 1,
        worst_input: vec![],
    };
    let mut idx = vec![lo; n];
    loop {
        let mut vals: Vec<Option<i64>> = Vec::new();
        for s in &shapes {
            vals.push(fold_int(r, &idx, s, lo, hi));
        }
        let defined: Vec<i64> = vals.iter().filter_map(|v| *v).collect();
        let any_undef = vals.iter().any(|v| v.is_none());
        if any_undef && !defined.is_empty() {
            d.disagree_defined += 1;
        }
        if !defined.is_empty() {
            let gap = defined.iter().max().unwrap() - defined.iter().min().unwrap();
            let mut uniq = defined.clone();
            uniq.sort_unstable();
            uniq.dedup();
            if gap > d.max_gap || (gap == d.max_gap && uniq.len() > d.worst_card) {
                d.max_gap = gap;
                d.worst_card = uniq.len();
                d.worst_input = idx.clone();
            }
        }
        // odometer over [lo, hi]^n
        let mut k = 0;
        loop {
            if k == n {
                return d;
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

fn main() {
    const N: usize = 5;
    let shapes = tree_shapes(N);
    println!(
        "groupings of a {N}-element fold: {} distinct tree shapes (Catalan C_4 = 14)\n",
        shapes.len()
    );

    // ---- fixed point, exhaustive over a 3-bit signed model -----------------
    println!(
        "REGROUPING DIAMETER, signed [-4,3], exhaustive over {} inputs",
        8usize.pow(N as u32)
    );
    println!(
        "{:<26}{:>12}{:>16}{:>26}",
        "arithmetic", "max gap", "worst |answers|", "grouping-dependent refuse"
    );
    for r in [Rule::Exact, Rule::Wrap, Rule::Saturate, Rule::Refuse] {
        let d = diameter_int(r, N, -4, 3);
        let name = match r {
            Rule::Wrap => "Wrap      (Hot)",
            Rule::Saturate => "Saturate  (Warm/Cold)",
            Rule::Refuse => "Refuse    (Precise)",
            Rule::Exact => "Exact     (unbounded)",
        };
        println!(
            "{:<26}{:>12}{:>16}{:>26}",
            name, d.max_gap, d.worst_card, d.disagree_defined
        );
        if d.max_gap > 0 {
            println!("      worst input {:?}", d.worst_input);
        }
    }

    // ---- does the diameter grow with fold length? --------------------------
    // If it does, no fixed budget survives, and the quantitative reading is
    // unusable for that arithmetic however elegant it looks.
    println!();
    println!("does the diameter GROW with fold length? signed [-4,3]");
    println!(
        "{:<26}{:>8}{:>8}{:>8}{:>8}",
        "arithmetic", "n=2", "n=3", "n=4", "n=5"
    );
    for r in [Rule::Wrap, Rule::Saturate, Rule::Refuse] {
        let name = match r {
            Rule::Wrap => "Wrap      (Hot)",
            Rule::Saturate => "Saturate  (Warm/Cold)",
            _ => "Refuse    (Precise)",
        };
        print!("{name:<26}");
        for n in 2..=5 {
            print!("{:>8}", diameter_int(r, n, -4, 3).max_gap);
        }
        println!();
    }

    // ---- and the sign asymmetry Dolan proved, measured on this instrument --
    println!();
    println!("unsigned [0,7], the case Dolan showed is signed-only");
    println!(
        "{:<26}{:>8}{:>8}{:>8}{:>8}",
        "arithmetic", "n=2", "n=3", "n=4", "n=5"
    );
    for r in [Rule::Wrap, Rule::Saturate, Rule::Refuse] {
        let name = match r {
            Rule::Wrap => "Wrap      (Hot)",
            Rule::Saturate => "Saturate  (Warm/Cold)",
            _ => "Refuse    (Precise)",
        };
        print!("{name:<26}");
        for n in 2..=5 {
            print!("{:>8}", diameter_int(r, n, 0, 7).max_gap);
        }
        println!();
    }

    // ---- float, structured inputs -----------------------------------------
    println!();
    println!("REGROUPING DIAMETER, f64, on named inputs (the whole range is [-inf, inf],");
    println!("so an exhaustive sweep is not available and these are constructions)");
    println!(
        "{:<40}{:>16}{:>16}{:>14}",
        "input", "min answer", "max answer", "rel. gap"
    );
    let cases: [(&str, Vec<f64>); 5] = [
        ("all ones", vec![1.0; N]),
        (
            "cancellation: 1e16, -1e16, 1, 1, 1",
            vec![1.0e16, -1.0e16, 1.0, 1.0, 1.0],
        ),
        (
            "cancellation, larger: 1e17, -1e17, 1, 1, 1",
            vec![1.0e17, -1.0e17, 1.0, 1.0, 1.0],
        ),
        (
            "graded magnitudes: 1e8, 1, 1, 1, -1e8",
            vec![1.0e8, 1.0, 1.0, 1.0, -1.0e8],
        ),
        (
            "inf and -inf present",
            vec![f64::INFINITY, f64::NEG_INFINITY, 1.0, 1.0, 1.0],
        ),
    ];
    for (label, xs) in cases {
        let ans: Vec<f64> = shapes.iter().map(|s| fold_f64(&xs, s)).collect();
        let finite: Vec<f64> = ans.iter().copied().filter(|v| v.is_finite()).collect();
        let (lo, hi) = if finite.is_empty() {
            (f64::NAN, f64::NAN)
        } else {
            (
                finite.iter().copied().fold(f64::INFINITY, f64::min),
                finite.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            )
        };
        let nan = ans.iter().filter(|v| v.is_nan()).count();
        let rel = if lo.abs().max(hi.abs()) > 0.0 {
            (hi - lo) / lo.abs().max(hi.abs())
        } else {
            0.0
        };
        println!(
            "{:<40}{:>16}{:>16}{:>13.1}%{}",
            label,
            format!("{lo:.4}"),
            format!("{hi:.4}"),
            rel * 100.0,
            if nan > 0 {
                format!("  ({nan}/{} groupings NaN)", ans.len())
            } else {
                String::new()
            }
        );
    }

    println!();
    println!("readings, and they point in different directions:");
    println!();
    println!("1. Associativity is diameter 0. Every row above with max gap 0 is a row");
    println!("   a boolean `Associative` marker would admit, and the rows with a small");
    println!("   bounded gap are the ones the boolean refuses while a diameter budget");
    println!("   would admit. That is the finer instrument McSherry's 1024-to-1 finding");
    println!("   asks for, and it never states anything false.");
    println!();
    println!("2. But float has NO finite diameter. The relative gap reaches 100% and");
    println!("   the last row reaches NaN, which is not a distance at all. So a");
    println!("   numeric budget cannot be the general form of the grade: for float the");
    println!("   grade can only be the SET of reachable answers, with no metric on it.");
    println!("   `Relaxed` fidelity is therefore not 'equality with slack'. It is the");
    println!("   replacement of a function by a relation, and that is exactly why a");
    println!("   `Relaxed` composition cannot derive `Deterministic`.");
    println!();
    println!("3. And the refuse column is a third kind of disagreement entirely: two");
    println!("   groupings where one returns and the other refuses have no gap, no");
    println!("   ratio, and no set-membership answer either. Kleene equality already");
    println!("   handles it and no quantitative reading extends to it.");
}
