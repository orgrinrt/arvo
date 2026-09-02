//! Does the order for `Ranged` numerals repeat the coupling that broke the
//! componentwise reading for fixed point?
//!
//! A `Ranged` numeral's value set is not an arithmetic progression: it is a union of
//! them, one per exponent. So section 2's four conditions do not decide it, and the
//! question is what does. The obvious reading is componentwise on the declared
//! coordinates: significand precision up, exponent range out at both ends.
//!
//! Normal values only. Subnormals and specials are omitted and the omission is
//! recorded rather than hidden: they would only add cases.
//!
//!   rustc -O o2_ranged_order.rs -o o2_ranged_order && ./o2_ranged_order

use std::collections::HashSet;

/// value m * 2^e represented exactly as (odd_mantissa, exponent), a canonical form
fn canon(m: i64, e: i32) -> (i64, i32) {
    if m == 0 {
        return (0, 0);
    }
    let (mut m, mut e) = (m, e);
    while m % 2 == 0 {
        m /= 2;
        e += 1;
    }
    (m, e)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct R {
    p: u32,
    emin: i32,
    emax: i32,
}

fn values(r: R) -> HashSet<(i64, i32)> {
    let mut s = HashSet::new();
    s.insert((0, 0));
    let lo = 1i64 << (r.p - 1);
    let hi = 1i64 << r.p;
    for m in lo..hi {
        for e in r.emin..=r.emax {
            s.insert(canon(m, e));
            s.insert(canon(-m, e));
        }
    }
    s
}

fn main() {
    let mut fam = Vec::new();
    for p in 2..=4u32 {
        for emin in -3..=0i32 {
            for emax in 0..=3i32 {
                fam.push(R { p, emin, emax });
            }
        }
    }
    let vs: Vec<_> = fam.iter().map(|&r| (r, values(r))).collect();

    // ---- the componentwise reading on the declared coordinates -------------
    let mut cw_fail = 0usize;
    let mut pairs = 0usize;
    let mut first: Option<String> = None;
    for (a, av) in &vs {
        for (b, bv) in &vs {
            pairs += 1;
            let cw = a.p <= b.p && b.emin <= a.emin && a.emax <= b.emax;
            let real = av.is_subset(bv);
            if cw != real {
                cw_fail += 1;
                if first.is_none() {
                    first = Some(format!(
                        "p{} [{},{}] into p{} [{},{}]: componentwise says {}, inclusion is {}",
                        a.p, a.emin, a.emax, b.p, b.emin, b.emax, cw, real
                    ));
                }
            }
        }
    }
    println!("R. Ranged numerals {} ordered pairs {}", fam.len(), pairs);
    println!("R. componentwise failures {}", cw_fail);
    if let Some(s) = first {
        println!("R. first counterexample: {}", s);
    }

    // ---- the shifted reading: the exponent window moves with the precision --
    // adding d digits to the significand shifts every value's exponent down by d,
    // so the target's window must cover the source's window shifted by d.
    let mut sh_fail = 0usize;
    let mut sh_first: Option<String> = None;
    for (a, av) in &vs {
        for (b, bv) in &vs {
            let d = b.p as i32 - a.p as i32;
            let sh = a.p <= b.p && b.emin <= a.emin - d && a.emax - d <= b.emax;
            let real = av.is_subset(bv);
            if sh != real {
                sh_fail += 1;
                if sh_first.is_none() {
                    sh_first = Some(format!(
                        "p{} [{},{}] into p{} [{},{}]: shifted says {}, inclusion is {}",
                        a.p, a.emin, a.emax, b.p, b.emin, b.emax, sh, real
                    ));
                }
            }
        }
    }
    println!("R. shifted-window failures {}", sh_fail);
    if let Some(s) = sh_first {
        println!("R. first: {}", s);
    }

    // ---- equal precision, comparable ---------------------------------------
    let mut eq_prec_comparable = 0usize;
    for (a, av) in &vs {
        for (b, bv) in &vs {
            if a.p == b.p && a != b && av.is_subset(bv) {
                eq_prec_comparable += 1;
            }
        }
    }
    println!(
        "R. equal-precision ordered pairs that ARE strictly comparable: {}",
        eq_prec_comparable
    );

    // ---- equal cardinality, comparable (the section 4 theorem) -------------
    let mut eq_card_violations = 0usize;
    let mut eq_card_pairs = 0usize;
    for (_, av) in &vs {
        for (_, bv) in &vs {
            if av.len() == bv.len() {
                eq_card_pairs += 1;
                if av.is_subset(bv) && av != bv {
                    eq_card_violations += 1;
                }
            }
        }
    }
    println!(
        "R. equal-cardinality pairs {} inclusion-without-equality {}",
        eq_card_pairs, eq_card_violations
    );
}
