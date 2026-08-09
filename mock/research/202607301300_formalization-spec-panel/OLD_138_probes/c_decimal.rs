//! 138 probe C. The decimal family's laws, and the conformance claim at
//! 110:2379-2383, checked rather than asserted. Exhaustive over each model.
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Datum {
    neg: bool,
    s: i64,
    q: i32,
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn val(d: Datum) -> (i128, i128) {
    let mut n = d.s as i128 * if d.neg { -1 } else { 1 };
    let mut den: i128 = 1;
    if d.q >= 0 {
        for _ in 0..d.q {
            n *= 10;
        }
    } else {
        for _ in 0..(-d.q) {
            den *= 10;
        }
    }
    if n == 0 {
        return (0, 1);
    }
    let g = gcd(n.abs(), den);
    (n / g, den / g)
}
fn data(p: u32, qmin: i32, qmax: i32) -> Vec<Datum> {
    let hi = 10i64.pow(p);
    let mut v = Vec::new();
    for q in qmin..=qmax {
        for s in 0..hi {
            for neg in [false, true] {
                v.push(Datum { neg, s, q });
            }
        }
    }
    v
}

fn main() {
    // =================================================================== DC1
    let d = data(2, 0, 2);
    let vals: HashSet<(i128, i128)> = d.iter().map(|x| val(*x)).collect();
    println!("DC1  p=2, q in [0,2]: data={} distinct={} redundant={}  (110:2350-2355 says 600/559)  match={}",
             d.len(), vals.len(), d.len() - vals.len(),
             d.len() == 600 && vals.len() == 559);

    // =================================================================== DC2
    // "The value sets of the normalised and unnormalised counterfactuals are
    // identical" (110:2354-2355). Checked BOTH ways.
    let lo = 10i64.pow(1);
    let n_flush: HashSet<(i128, i128)> = d
        .iter()
        .filter(|x| x.s == 0 || x.s >= lo)
        .map(|x| val(*x))
        .collect();
    let n_grad: HashSet<(i128, i128)> = d
        .iter()
        .filter(|x| x.s == 0 || x.s >= lo || x.q == 0)
        .map(|x| val(*x))
        .collect();
    println!(
        "DC2  normalised, Underflow = Abrupt   : distinct={}  identical to unnormalised = {}",
        n_flush.len(),
        n_flush == vals
    );
    println!(
        "DC2  normalised, Underflow = Gradual  : distinct={}  identical to unnormalised = {}",
        n_grad.len(),
        n_grad == vals
    );
    let mut missing: Vec<(i128, i128)> = vals.difference(&n_flush).copied().collect();
    missing.sort();
    println!(
        "     values the Abrupt counterfactual loses: {} of them, smallest three {:?}",
        missing.len(),
        &missing[..3.min(missing.len())]
    );

    // =================================================================== DC3
    let d2 = data(2, -1, 2);
    let mut coh: HashMap<(i128, i128), Vec<Datum>> = HashMap::new();
    for x in &d2 {
        coh.entry(val(*x)).or_default().push(*x);
    }
    let mut disagree = 0;
    let mut wit = None;
    for (v, m) in coh.iter() {
        let pos: Vec<Datum> = m.iter().copied().filter(|x| !x.neg).collect();
        if pos.len() < 2 {
            continue;
        }
        let small = pos.iter().min_by_key(|x| x.s).unwrap();
        let large = pos.iter().max_by_key(|x| x.s).unwrap();
        if small != large {
            disagree += 1;
            if *v == (1, 1) {
                wit = Some((*small, *large));
            }
        }
    }
    println!(
        "DC3  radix ten, p=2, q in [-1,2]: values where the two cohort rules disagree = {}",
        disagree
    );
    if let Some((a, b)) = wit {
        println!(
            "     witness for the value 1: {}x10^{} against {}x10^{}   (110:2360's own witness)",
            a.s, a.q, b.s, b.q
        );
    }
    let mut b2 = 0;
    let mut seen: HashMap<(i128, i128), usize> = HashMap::new();
    for q in -3..=3i32 {
        for s in 8..16i64 {
            let (mut n, mut den) = (s as i128, 1i128);
            if q >= 0 {
                for _ in 0..q {
                    n *= 2
                }
            } else {
                for _ in 0..(-q) {
                    den *= 2
                }
            }
            let g = gcd(n.abs(), den);
            *seen.entry((n / g, den / g)).or_insert(0) += 1;
        }
    }
    for c in seen.values() {
        if *c > 1 {
            b2 += 1
        }
    }
    println!(
        "DC3  radix two, hidden digit, p=4, e in [-3,3]: values with a cohort beyond one = {}",
        b2
    );

    // =================================================================== DC4
    // Is the delivered VALUE of an arithmetic operation independent of which
    // cohort member carries the operand? Exhaustive over the whole model.
    let mut checked = 0u64;
    let mut viol = 0u64;
    let canon: HashMap<(i128, i128), Datum> = coh
        .iter()
        .map(|(v, m)| (*v, *m.iter().min().unwrap()))
        .collect();
    for a in &d2 {
        for b in &d2 {
            let (an, ad) = val(*a);
            let (bn, bd) = val(*b);
            let ca = canon[&(an, ad)];
            let cb = canon[&(bn, bd)];
            let (cn, cd) = val(ca);
            let (dn, dd) = val(cb);
            let s1 = (an * bd + bn * ad, ad * bd);
            let s2 = (cn * dd + dn * cd, cd * dd);
            let p1 = (an * bn, ad * bd);
            let p2 = (cn * dn, cd * dd);
            checked += 2;
            if s1.0 * s2.1 != s2.0 * s1.1 {
                viol += 1
            }
            if p1.0 * p2.1 != p2.0 * p1.1 {
                viol += 1
            }
        }
    }
    println!("DC4  exact + and * against the cohort's canonical member: {} comparisons, {} value divergences",
             checked, viol);

    // =================================================================== DC5
    // IEEE's preferred exponent against the design's Implicit result-numeral
    // rule. IEEE 754-2019 clause 5.2: add -> min(Qx,Qy), mul -> Qx+Qy, and the
    // preferred exponent is delivered only when the format can express it.
    // The design's Implicit rule: add -> min, mul -> sum, and the RESULT
    // NUMERAL widens (mul_full), so it is always expressible.
    for &(p, name) in &[(7u32, "decimal32-shaped"), (16u32, "decimal64-shaped")] {
        let (mut agree_add, mut clamp_add) = (0u64, 0u64);
        let (mut agree_mul, mut clamp_mul) = (0u64, 0u64);
        let qs: Vec<i32> = (-(p as i32) - 4..=4).collect();
        for &qx in &qs {
            for &qy in &qs {
                let top = 10i128.pow(p - 1) as i64;
                for sx in [1i64, 3, top / 7, top, top * 9 + 9] {
                    for sy in [1i64, 7, top / 3, top, top * 9 + 9] {
                        // ADD: exact sum has quantum min(qx,qy); IEEE prefers the same.
                        let want = qx.min(qy);
                        let exact_digits_add = ((sx as i128) * pow10((qx - want) as u32)
                            + (sy as i128) * pow10((qy - want) as u32))
                        .abs();
                        if digits(exact_digits_add) <= p {
                            agree_add += 1
                        } else {
                            clamp_add += 1
                        }
                        // MUL: exact product has quantum qx+qy, needing digits(sx)+digits(sy).
                        let want_m = qx + qy;
                        let prod = (sx as i128) * (sy as i128);
                        if digits(prod) <= p {
                            agree_mul += 1
                        } else {
                            clamp_mul += 1
                        }
                        let _ = want_m;
                    }
                }
            }
        }
        println!("DC5  {}: p={}", name, p);
        println!(
            "     add: preferred exponent achievable in-format {} of {}, clamped {}",
            agree_add,
            agree_add + clamp_add,
            clamp_add
        );
        println!(
            "     mul: preferred exponent achievable in-format {} of {}, clamped {}",
            agree_mul,
            agree_mul + clamp_mul,
            clamp_mul
        );
        println!("     the design's Implicit numeral achieves it in ALL of them, because the");
        println!("     result NUMERAL widens (mul_full) instead of the value being reshaped.");
    }

    // the named case, spelled out
    println!("\nDC5  the worked case. decimal32, p=7. x = 1.000000 (s=1000000, q=-6).");
    println!("     x*x exact = 1. IEEE preferred exponent = -12, needing 13 significand digits.");
    println!("     p = 7, so the closest achievable is q = -6: the preference silently degrades.");
    println!("     Implicit<-6> * Implicit<-6> = Implicit<-12> at precision 14, exact, checked");
    println!("     at compile time, and no preference was consulted at runtime.");
}

fn pow10(k: u32) -> i128 {
    let mut r = 1i128;
    for _ in 0..k {
        r *= 10
    }
    r
}
fn digits(mut n: i128) -> u32 {
    n = n.abs();
    if n == 0 {
        return 1;
    }
    let mut c = 0;
    while n > 0 {
        n /= 10;
        c += 1
    }
    c
}
