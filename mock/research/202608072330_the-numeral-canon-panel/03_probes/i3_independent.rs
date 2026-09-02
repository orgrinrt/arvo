// Instrument 3: an independent re-derivation of the four claims instruments 1 and 2
// carry, in a different language and a different number representation.
//
// Instruments 1 and 2 are Python and use exact rationals (fractions.Fraction) with
// frozenset operations. This one is Rust and uses integers scaled by a fixed power of
// two, with sorted vectors and a merge test. Nothing is shared: not the language, not
// the number type, not the containment algorithm, not the enumeration order. Where the
// two agree, they agree from different directions.
//
//   rustc +nightly-2026-05-28 --edition 2024 i3_independent.rs --out-dir out
//   ./out/i3_independent
//
// What it checks:
//   C1  the four-condition predicate against true inclusion, split by how many values
//       the source carries (02_carried section 1.6's claim)
//   C2  whether the unsigned fixed-point family's meet needs the origin shape
//   C3  whether UF<0,1> and UF<2,0> have two minimal upper bounds once a float shape
//       is in the same space, which is the join disappearing
//   C4  whether the exact meet across two sign domains needs negative integer width

const SCALE: i64 = 1 << 12;

#[derive(Clone, Debug)]
struct Shape {
    name: String,
    vals: Vec<i64>, // scaled by SCALE, sorted, deduped
    step: i64,      // DECLARED step, scaled. meaningless for an empty set.
    bias: i64,      // DECLARED bias, scaled
    n: i64,         // declared value count
}

fn uf(i: i32, f: i32, bias_num: i64, bias_den: i64) -> Shape {
    let step = SCALE >> f.max(0) << (-f).max(0);
    let n = if i + f >= 0 { 1i64 << (i + f) } else { 0 };
    let bias = bias_num * SCALE / bias_den;
    let mut vals: Vec<i64> = (0..n).map(|k| bias + k * step).collect();
    vals.sort_unstable();
    vals.dedup();
    let b = if bias_num == 0 {
        String::new()
    } else {
        format!("+{bias_num}/{bias_den}")
    };
    Shape {
        name: format!("U<{i},{f}>{b}"),
        vals,
        step,
        bias,
        n,
    }
}

fn sym(i: i32, f: i32) -> Shape {
    let step = SCALE >> f.max(0) << (-f).max(0);
    let n = if i + f >= 0 { 1i64 << (i + f) } else { 0 };
    let half = (n - 1) / 2;
    let mut vals: Vec<i64> = (-half..=half).map(|k| k * step).collect();
    vals.sort_unstable();
    vals.dedup();
    Shape {
        name: format!("S<{i},{f}>"),
        vals,
        step,
        bias: -half * step,
        n: 2 * half + 1,
    }
}

fn flt(p: i32, emin: i32, emax: i32) -> Shape {
    let mut vals = vec![0i64];
    for e in emin..=emax {
        for m in (1i64 << (p - 1))..(1i64 << p) {
            let sh = e - p + 1;
            let v = if sh >= 0 {
                m * SCALE << sh
            } else {
                m * SCALE >> (-sh)
            };
            // exactness of the shift, checked rather than assumed
            if sh < 0 {
                assert_eq!(
                    v << (-sh),
                    m * SCALE,
                    "float value lost bits at the chosen scale"
                );
            }
            vals.push(v);
        }
    }
    vals.sort_unstable();
    vals.dedup();
    let n = vals_len(&vals);
    Shape {
        name: format!("F<p{p},e{emin}..{emax}>"),
        vals,
        step: 0,
        bias: 0,
        n,
    }
}

fn vals_len(v: &[i64]) -> i64 {
    v.len() as i64
}

/// True set inclusion, by merge. No shape parameter is consulted.
fn included(a: &[i64], b: &[i64]) -> bool {
    let mut j = 0usize;
    for &x in a {
        while j < b.len() && b[j] < x {
            j += 1;
        }
        if j == b.len() || b[j] != x {
            return false;
        }
    }
    true
}

/// The four-condition predicate, read off DECLARED parameters only.
fn four_condition(a: &Shape, b: &Shape) -> bool {
    let (l1, g1) = (a.bias, a.bias + (a.n - 1) * a.step);
    let (l2, g2) = (b.bias, b.bias + (b.n - 1) * b.step);
    let grid = b.step != 0 && a.step % b.step == 0;
    let phase = b.step != 0 && (a.bias - b.bias).rem_euclid(b.step) == 0;
    grid && phase && l2 <= l1 && g1 <= g2
}

fn c1() {
    println!("C1  four-condition predicate against true inclusion, split by source size");
    let mut shapes = Vec::new();
    for i in -3..=3 {
        for f in 0..=3 {
            if i + f < 0 {
                continue;
            }
            shapes.push(uf(i, f, 0, 1));
        }
    }
    let (mut tot, mut agree, mut small, mut big) = (0, 0, 0, 0);
    let mut witness = None;
    for a in &shapes {
        for b in &shapes {
            tot += 1;
            let truth = included(&a.vals, &b.vals);
            let pred = four_condition(a, b);
            if truth == pred {
                agree += 1;
            } else if a.n < 2 {
                small += 1;
                if witness.is_none() {
                    witness = Some((a.name.clone(), b.name.clone(), truth, pred));
                }
            } else {
                big += 1;
                println!(
                    "      UNEXPLAINED {} into {}: truth {truth}, predicate {pred}",
                    a.name, b.name
                );
            }
        }
    }
    println!(
        "      ordered pairs {tot}, agree {agree}, disagree {}",
        tot - agree
    );
    println!("        source carries fewer than two values: {small}");
    println!("        source carries two or more values:    {big}");
    if let Some((x, y, t, p)) = witness {
        println!("      witness: {x} into {y}: really {t}, predicate {p}");
    }
    println!();
}

fn greatest_lower(space: &[Shape], a: &Shape, b: &Shape) -> Option<usize> {
    let cand: Vec<usize> = (0..space.len())
        .filter(|&k| included(&space[k].vals, &a.vals) && included(&space[k].vals, &b.vals))
        .collect();
    cand.iter().copied().find(|&k| {
        cand.iter()
            .all(|&m| included(&space[m].vals, &space[k].vals))
    })
}

fn c2() {
    println!("C2  does the unsigned family's meet need the origin shape?");
    let mut with: Vec<Shape> = Vec::new();
    for i in 0..=6 {
        for f in 0..=6 {
            if i + f <= 6 {
                with.push(uf(i, f, 0, 1));
            }
        }
    }
    let without: Vec<Shape> = with.iter().filter(|s| s.vals.len() > 1).cloned().collect();
    for (tag, sp) in [("origin admitted", &with), ("origin refused ", &without)] {
        let mut ok = 0;
        let mut fail = 0;
        let mut first = None;
        for x in 0..sp.len() {
            for y in (x + 1)..sp.len() {
                match greatest_lower(sp, &sp[x], &sp[y]) {
                    Some(_) => ok += 1,
                    None => {
                        fail += 1;
                        if first.is_none() {
                            first = Some((sp[x].name.clone(), sp[y].name.clone()));
                        }
                    }
                }
            }
        }
        print!(
            "      {tag}: points {}, meets present {ok}, meets absent {fail}",
            sp.len()
        );
        match first {
            Some((x, y)) => println!("   first absence: {x} and {y}"),
            None => println!(),
        }
    }
    println!("      the intersection of U<0,1> and U<1,0> is the single value zero, which is");
    println!("      the origin shape and nothing else in the family.");
    println!();
}

fn minimal_uppers(space: &[Shape], a: &Shape, b: &Shape) -> Vec<String> {
    let cand: Vec<usize> = (0..space.len())
        .filter(|&k| included(&a.vals, &space[k].vals) && included(&b.vals, &space[k].vals))
        .collect();
    cand.iter()
        .copied()
        .filter(|&k| {
            !cand
                .iter()
                .any(|&m| m != k && included(&space[m].vals, &space[k].vals))
        })
        .map(|k| space[k].name.clone())
        .collect()
}

fn c3() {
    println!("C3  the join of U<0,1> and U<2,0>, before and after a float joins the space");
    let mut fx: Vec<Shape> = Vec::new();
    for i in 0..=5 {
        for f in 0..=5 {
            if i + f <= 5 {
                fx.push(uf(i, f, 0, 1));
            }
        }
    }
    let a = uf(0, 1, 0, 1);
    let b = uf(2, 0, 0, 1);
    println!("      U<0,1> = {:?}", scaled(&a.vals));
    println!("      U<2,0> = {:?}", scaled(&b.vals));
    println!(
        "      fixed-point alone, minimal upper bounds: {:?}",
        minimal_uppers(&fx, &a, &b)
    );
    let mut both = fx.clone();
    for p in 1..=3 {
        for emin in -3..=0 {
            for emax in 0..=2 {
                if emin <= emax {
                    both.push(flt(p, emin, emax));
                }
            }
        }
    }
    let mins = minimal_uppers(&both, &a, &b);
    println!("      with floats present, minimal upper bounds: {mins:?}");
    if mins.len() > 1 {
        println!("      two or more minimal upper bounds and no least one: the join is gone.");
        let f = both.iter().find(|s| s.name.starts_with("F<p2,e-1..1>"));
        if let Some(f) = f {
            println!("      F<p2,e-1..1> = {:?}", scaled(&f.vals));
            let u21 = uf(2, 1, 0, 1);
            println!("      U<2,1>       = {:?}", scaled(&u21.vals));
            println!(
                "      neither contains the other: U<2,1> into float {}, float into U<2,1> {}",
                included(&u21.vals, &f.vals),
                included(&f.vals, &u21.vals)
            );
        }
    }
    println!();
}

fn scaled(v: &[i64]) -> Vec<String> {
    v.iter()
        .map(|&x| {
            let whole = x / SCALE;
            let rem = x % SCALE;
            if rem == 0 {
                format!("{whole}")
            } else {
                let mut num = rem;
                let mut den = SCALE;
                while num % 2 == 0 && den % 2 == 0 {
                    num /= 2;
                    den /= 2;
                }
                if whole == 0 {
                    format!("{num}/{den}")
                } else {
                    format!("{whole}+{num}/{den}")
                }
            }
        })
        .collect()
}

fn c4() {
    println!("C4  the exact meet across two sign domains, with and without negative width");
    for lo in [0i32, -4] {
        let mut sp: Vec<Shape> = Vec::new();
        for i in lo..=5 {
            for f in 0..=5 {
                if i + f >= 0 && i + f <= 5 {
                    sp.push(uf(i, f, 0, 1));
                    sp.push(sym(i, f));
                }
            }
        }
        let mut exact = 0;
        let mut under = 0;
        let mut absent = 0;
        let mut first = None;
        for x in 0..sp.len() {
            for y in (x + 1)..sp.len() {
                let inter: Vec<i64> = sp[x]
                    .vals
                    .iter()
                    .copied()
                    .filter(|v| sp[y].vals.binary_search(v).is_ok())
                    .collect();
                match greatest_lower(&sp, &sp[x], &sp[y]) {
                    None => absent += 1,
                    Some(k) => {
                        if sp[k].vals == inter {
                            exact += 1;
                        } else {
                            under += 1;
                            if first.is_none() {
                                first = Some((
                                    sp[x].name.clone(),
                                    sp[y].name.clone(),
                                    scaled(&inter),
                                    scaled(&sp[k].vals),
                                ));
                            }
                        }
                    }
                }
            }
        }
        let tag = if lo == 0 { "I >= 0 " } else { "I >= -4" };
        println!("      {tag}: exact {exact}, undershooting {under}, absent {absent}");
        if let Some((x, y, i, g)) = first {
            println!("        first undershoot {x} and {y}: intersection {i:?}, greatest lower bound {g:?}");
        }
    }
    println!();
}

fn main() {
    println!("instrument 3: independent re-derivation in Rust over scaled integers");
    println!("{}", "=".repeat(74));
    c1();
    c2();
    c3();
    c4();
}
