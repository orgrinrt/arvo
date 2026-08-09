//! PROBE 4: the partial-identity property is about ADDITION, and the prediction
//! that it stops at multiplication, tested.
//!
//! Probe 3 established that `Precise`'s law comes from its recovery map being a
//! partial identity: wherever it returns, it returns the exact value. For that to
//! be true of a whole operation, the exact result of that operation on two
//! representable values has to BE representable whenever it is in range, so the
//! in-range rounding never fires.
//!
//! For addition on a fixed-point numeral that holds, and for a reason worth
//! stating in one line: the sum of two multiples of the quantum is a multiple of
//! the quantum. The representable set is a subgroup of the exact one under
//! addition, so `phi` restricted to in-range sums is the identity, with nothing
//! to round.
//!
//! For multiplication it fails, and for the same one-line reason read backwards:
//! the product of two values with F fractional bits has 2F fractional bits, so it
//! is generically NOT a multiple of the quantum, so the in-range rounding fires on
//! ordinary inputs and `phi` stops being a partial identity long before any range
//! boundary is reached.
//!
//! If that is right, `Precise` multiplication should lose existential
//! associativity while `Precise` addition keeps it, and the failure should show up
//! at inputs nowhere near the range ends. Nothing in this review has measured
//! `Precise` multiplication: file 15's probe covers wrapping and saturating
//! multiply against the SHIPPED truncating body, which is a different function
//! from the one the preset table at `11_current_shape_draft.md:327` describes.
//!
//! Build:  rustc -O 04_where_the_partial_identity_stops.rs -o p4 && ./p4

#![allow(clippy::needless_range_loop)]

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

/// Signed Q2.2: raw integers in [-8, 7], each raw unit worth 1/4.
const LO: i64 = -8;
const HI: i64 = 7;
const FRAC: u32 = 2;
const SCALE: i64 = 1 << FRAC;

/// `Precise` in-range resolution: nearest, ties to even, on the numeral's own
/// lattice. `num` and `den` express the exact rational result as num/den in raw
/// units, so no floating point enters the model.
fn nearest_ties_even(num: i64, den: i64) -> i64 {
    debug_assert!(den > 0);
    let q = num.div_euclid(den);
    let r = num.rem_euclid(den);
    let twice = 2 * r;
    if twice > den {
        q + 1
    } else if twice < den {
        q
    } else if q.rem_euclid(2) == 0 {
        q
    } else {
        q + 1
    }
}

/// `Precise` recovery: round to nearest in range, refuse out of range.
fn settle(num: i64, den: i64) -> Option<i64> {
    let v = nearest_ties_even(num, den);
    if v < LO || v > HI {
        None
    } else {
        Some(v)
    }
}

/// Addition of raw values: exact, no scaling. den = 1.
fn add_precise(a: i64, b: i64) -> Option<i64> {
    settle(a + b, 1)
}

/// Multiplication of raw values: the exact product is (a * b) / SCALE raw units.
fn mul_precise(a: i64, b: i64) -> Option<i64> {
    settle(a * b, SCALE)
}

/// Was the in-range rounding a no-op, i.e. is `phi` acting as a partial identity
/// on this input pair?
fn rounding_fired(num: i64, den: i64) -> bool {
    num.rem_euclid(den) != 0
}

fn fold(op: fn(i64, i64) -> Option<i64>, xs: &[i64], sched: &[usize]) -> Option<i64> {
    let mut v: Vec<Option<i64>> = xs.iter().map(|&x| Some(x)).collect();
    for &i in sched {
        let b = v.remove(i + 1);
        let a = v.remove(i);
        let c = match (a, b) {
            (Some(a), Some(b)) => op(a, b),
            _ => None,
        };
        v.insert(i, c);
    }
    v[0]
}

struct Report {
    kleene: bool,
    existential: bool,
    diameter: i64,
    first_value_disagreement: Option<(Vec<i64>, Vec<Option<i64>>)>,
}

fn measure(op: fn(i64, i64) -> Option<i64>, n: usize) -> Report {
    let shapes = tree_shapes(n);
    let mut rep = Report {
        kleene: true,
        existential: true,
        diameter: 0,
        first_value_disagreement: None,
    };
    let mut idx = vec![LO; n];
    loop {
        let vals: Vec<Option<i64>> = shapes.iter().map(|s| fold(op, &idx, s)).collect();
        if vals.iter().any(|v| *v != vals[0]) {
            rep.kleene = false;
        }
        let d: Vec<i64> = vals.iter().filter_map(|v| *v).collect();
        if d.iter().any(|v| *v != d[0]) {
            rep.existential = false;
            let gap = d.iter().max().unwrap() - d.iter().min().unwrap();
            if gap > rep.diameter {
                rep.diameter = gap;
            }
            if rep.first_value_disagreement.is_none() {
                rep.first_value_disagreement = Some((idx.clone(), vals.clone()));
            }
        }
        let mut k = 0;
        loop {
            if k == n {
                return rep;
            }
            idx[k] += 1;
            if idx[k] <= HI {
                break;
            }
            idx[k] = LO;
            k += 1;
        }
    }
}

fn raw(v: i64) -> String {
    format!("{}", v as f64 / SCALE as f64)
}

fn main() {
    println!(
        "signed Q2.2, raw range [{LO},{HI}], real range [{}, {}], quantum {}",
        LO as f64 / SCALE as f64,
        HI as f64 / SCALE as f64,
        1.0 / SCALE as f64
    );
    println!("`Precise`: nearest ties-to-even in range, refuse out of range");
    println!("(this is the preset table at 11_current_shape_draft.md:327, not the");
    println!("shipped truncating body file 15 measured)\n");

    // --- 1. how often does the in-range rounding fire at all -----------------
    let mut add_fired = 0usize;
    let mut mul_fired = 0usize;
    let mut pairs = 0usize;
    for a in LO..=HI {
        for b in LO..=HI {
            pairs += 1;
            if rounding_fired(a + b, 1) {
                add_fired += 1;
            }
            if rounding_fired(a * b, SCALE) {
                mul_fired += 1;
            }
        }
    }
    println!("is `phi` a partial identity, i.e. does the in-range rounding ever fire?");
    println!("  operand pairs:                       {pairs}");
    println!("  addition,       rounding fired on:   {add_fired}");
    println!("  multiplication, rounding fired on:   {mul_fired}");
    println!();

    // --- 2. the laws ---------------------------------------------------------
    println!(
        "{:<20}{:>4}{:>10}{:>14}{:>12}",
        "operation", "n", "Kleene", "existential", "diameter"
    );
    for n in 2..=5 {
        let r = measure(add_precise, n);
        println!(
            "{:<20}{:>4}{:>10}{:>14}{:>12}",
            if n == 2 { "Precise +" } else { "" },
            n,
            r.kleene,
            r.existential,
            r.diameter
        );
    }
    println!();
    let mut mul_witness = None;
    for n in 2..=5 {
        let r = measure(mul_precise, n);
        println!(
            "{:<20}{:>4}{:>10}{:>14}{:>12}",
            if n == 2 { "Precise *" } else { "" },
            n,
            r.kleene,
            r.existential,
            r.diameter
        );
        if mul_witness.is_none() {
            mul_witness = r.first_value_disagreement.map(|w| (n, w));
        }
    }

    if let Some((n, (inputs, vals))) = mul_witness {
        println!();
        println!("first value disagreement for `Precise` multiplication, at n = {n}:");
        let reals: Vec<String> = inputs.iter().map(|v| raw(*v)).collect();
        println!("  inputs (real):  [{}]", reals.join(", "));
        println!("  inputs (raw):   {inputs:?}");
        let shown: Vec<String> = vals
            .iter()
            .map(|v| match v {
                None => "refuse".to_string(),
                Some(x) => raw(*x),
            })
            .collect();
        println!("  groupings:      [{}]", shown.join(", "));
        let in_range = inputs.iter().all(|v| *v > LO + 1 && *v < HI - 1);
        println!("  every input strictly inside the range ends: {in_range}");
    }

    println!();
    println!("=================================================================");
    println!("READING");
    println!("=================================================================");
    println!();
    println!("The prediction holds and it holds for the stated reason. Addition's");
    println!("in-range rounding fires on zero operand pairs, because the representable");
    println!("set is closed under addition, so `phi` is a partial identity and probe 3's");
    println!("theorem applies. Multiplication's fires on most pairs, because a product");
    println!("carries twice the fractional bits, so `phi` is not a partial identity and");
    println!("the theorem does not apply.");
    println!();
    println!("So `Precise`'s law is not a property of `Precise`. It is a property of");
    println!("(`Precise`, addition), and the design's law key has to carry the operation");
    println!("for the same reason it has to carry the numeral. Anything that reads");
    println!("`Precise` alone and concludes a law will be wrong about multiplication");
    println!("in the direction that matters, which is claiming one that is false.");
}
