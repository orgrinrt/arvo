//! 138 probe D. The exponent form's own laws, which 130 section 10 left open.
//! The form is the only axis on which Implicit and Ranged differ, so every law
//! keyed on it is a law about the form and about nothing else.

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn norm(n: i128, d: i128) -> (i128, i128) {
    if n == 0 {
        (0, 1)
    } else {
        let g = gcd(n.abs(), d);
        (n / g, d / g)
    }
}

/// Implicit<E>: one grid, quantum radix^E, significand in [0, radix^p).
fn implicit_set(radix: i128, p: u32, e: i32) -> Vec<(i128, i128)> {
    let hi = radix.pow(p);
    let (mut num, mut den) = (1i128, 1i128);
    if e >= 0 {
        for _ in 0..e {
            num *= radix
        }
    } else {
        for _ in 0..(-e) {
            den *= radix
        }
    }
    (0..hi).map(|s| norm(s * num, den)).collect()
}
/// Ranged<EMIN, EMAX, U>: the union over e of the grid with quantum
/// radix^(e-p+1) restricted to [radix^e, radix^(e+1)), plus the bottom grid
/// extended to zero when Gradual (110:1241-1245).
fn ranged_set(radix: i128, p: u32, emin: i32, emax: i32, gradual: bool) -> Vec<(i128, i128)> {
    let mut out = Vec::new();
    for e in emin..=emax {
        let k = e - p as i32 + 1;
        let (mut num, mut den) = (1i128, 1i128);
        if k >= 0 {
            for _ in 0..k {
                num *= radix
            }
        } else {
            for _ in 0..(-k) {
                den *= radix
            }
        }
        let lo = radix.pow((p - 1) as u32);
        let hi = radix.pow(p);
        for s in lo..hi {
            out.push(norm(s * num, den));
        }
    }
    if gradual {
        let k = emin - p as i32 + 1;
        let (mut num, mut den) = (1i128, 1i128);
        if k >= 0 {
            for _ in 0..k {
                num *= radix
            }
        } else {
            for _ in 0..(-k) {
                den *= radix
            }
        }
        for s in 0..radix.pow((p - 1) as u32) {
            out.push(norm(s * num, den));
        }
    }
    out.sort();
    out.dedup();
    out
}
fn add(a: (i128, i128), b: (i128, i128)) -> (i128, i128) {
    norm(a.0 * b.1 + b.0 * a.1, a.1 * b.1)
}
fn cmpv(a: &(i128, i128), b: &(i128, i128)) -> std::cmp::Ordering {
    (a.0 * b.1).cmp(&(b.0 * a.1))
}
fn bounds(s: &[(i128, i128)]) -> ((i128, i128), (i128, i128)) {
    let lo = *s.iter().min_by(|a, b| cmpv(a, b)).unwrap();
    let hi = *s.iter().max_by(|a, b| cmpv(a, b)).unwrap();
    (lo, hi)
}

fn main() {
    // ================================================================== EF1
    // Implicit's value set is an interval of a rank-one subgroup and is closed
    // under addition wherever the sum stays in range (110:1249-1250).
    // Ranged's is a union of intervals of subgroups and is NOT a subgroup.
    for &(radix, p, e) in &[(2i128, 6u32, -3i32), (10, 3, -2), (2, 8, 0), (10, 4, 5)] {
        let s = implicit_set(radix, p, e);
        let set: std::collections::HashSet<_> = s.iter().copied().collect();
        let (lo, hi) = bounds(&s);
        let mut out_of_set = 0u64;
        let mut in_range = 0u64;
        for a in &s {
            for b in &s {
                let c = add(*a, *b);
                let ge = c.0 * lo.1 >= lo.0 * c.1;
                let le = c.0 * hi.1 <= hi.0 * c.1;
                if ge && le {
                    in_range += 1;
                    if !set.contains(&c) {
                        out_of_set += 1
                    }
                }
            }
        }
        println!(
            "EF1  Implicit radix={} p={} E={}: {} in-range sums, {} leaving the value set",
            radix, p, e, in_range, out_of_set
        );
    }
    for &(radix, p, emin, emax) in &[(2i128, 4u32, -3i32, 3i32), (10, 2, -1, 2), (2, 8, -6, 6)] {
        let s = ranged_set(radix, p, emin, emax, true);
        let set: std::collections::HashSet<_> = s.iter().copied().collect();
        let (lo, hi) = bounds(&s);
        let mut out_of_set = 0u64;
        let mut in_range = 0u64;
        for a in &s {
            for b in &s {
                let c = add(*a, *b);
                let ge = c.0 * lo.1 >= lo.0 * c.1;
                let le = c.0 * hi.1 <= hi.0 * c.1;
                if ge && le {
                    in_range += 1;
                    if !set.contains(&c) {
                        out_of_set += 1
                    }
                }
            }
        }
        println!("EF1  Ranged   radix={} p={} E in [{},{}]: |set|={} lo={:?} hi={:?} {} in-range sums, {} leaving the value set",
                 radix, p, emin, emax, s.len(), lo, hi, in_range, out_of_set);
    }
    // the design's own witness, 110:1251
    // 110:1251's own witness, decided by the significand's bit length after
    // reduction: a dyadic n/2^m is a binary32 value only if n's odd part fits
    // in 24 bits (and the exponent is in range, which it is here).
    let n: i128 = (1i128 << 24) + 1; // 1 + 2^-24, times 2^24
    let bits = 128 - n.leading_zeros();
    println!(
        "EF1  110:1251's witness: 1 + 2^-24 needs a {}-bit significand; binary32 has 24,",
        bits
    );
    println!("     so it is a binary32 value: {}", bits <= 24);

    // ================================================================== EF2
    // Underflow is vacuous on Implicit and changes representability on Ranged
    // (110:1038-1039, 110:2205).
    let g = ranged_set(2, 4, -3, 3, true);
    let a = ranged_set(2, 4, -3, 3, false);
    println!(
        "EF2  Ranged p=4 e in [-3,3]: |Gradual| = {}, |Abrupt| = {}, differ = {}",
        g.len(),
        a.len(),
        g.len() != a.len()
    );
    println!("EF2  Implicit has one grid, so there is no bottom to fall off and no");
    println!("     Underflow setting to make: the axis is not merely defaulted, it is absent.");

    // ================================================================== EF3
    // The quantum function. Constant on Implicit, a function of the magnitude
    // on Ranged. This is what makes Direction enter the key on one and not the
    // other (110:1470-1472).
    let mut s = ranged_set(10, 2, 0, 2, true);
    s.sort_by(cmpv);
    let mut quanta = std::collections::BTreeSet::new();
    for i in 1..s.len() {
        let d = add(s[i], (-s[i - 1].0, s[i - 1].1));
        quanta.insert(d);
    }
    println!(
        "EF3  Ranged radix=10 p=2 e in [0,2]: distinct gaps between adjacent values = {}",
        quanta.len()
    );
    let mut s2 = implicit_set(10, 3, -1);
    s2.sort_by(cmpv);
    let mut q2 = std::collections::BTreeSet::new();
    for i in 1..s2.len() {
        let d = add(s2[i], (-s2[i - 1].0, s2[i - 1].1));
        q2.insert(d);
    }
    println!(
        "EF3  Implicit radix=10 p=3 E=-1: distinct gaps = {}",
        q2.len()
    );

    // ================================================================== EF4
    // decimal64 under the design's OWN convention for EMIN (110:1241-1243),
    // against the number 130:690 writes into the EMIN slot.
    let p = 16i32;
    println!("EF4  110:1241-1243 makes the bottom quantum radix^(EMIN - p + 1).");
    println!(
        "     decimal64's bottom quantum is 10^-398, so EMIN = -398 + {} - 1 = {}.",
        p,
        -398 + p - 1
    );
    println!("     130:690 writes Decimal<16, -398, u64, Warm>. Read as EMIN that is a");
    println!(
        "     bottom quantum of 10^{}, which is {} decades below decimal64's.",
        -398 - p + 1,
        p - 1
    );
    ef5();
}

// appended: is Implicit a degenerate Ranged, or a second constructor?
#[allow(dead_code)]
fn ef5() {
    println!(
        "\nEF5  is Implicit<K> the same value set as Ranged<E, E, Gradual> with K = E - p + 1?"
    );
    let mut all = true;
    for &(radix, p) in &[
        (2i128, 3u32),
        (2, 5),
        (2, 8),
        (10, 2),
        (10, 3),
        (3, 4),
        (7, 3),
    ] {
        for e in -4..=4i32 {
            let k = e - p as i32 + 1;
            let mut a = implicit_set(radix, p, k);
            let mut b = ranged_set(radix, p, e, e, true);
            a.sort_by(cmpv);
            a.dedup();
            b.sort_by(cmpv);
            b.dedup();
            let same = a == b;
            if !same {
                all = false;
                println!(
                    "     radix={} p={} E={} K={}: DIFFER  |I|={} |R|={}",
                    radix,
                    p,
                    e,
                    k,
                    a.len(),
                    b.len()
                );
            }
        }
    }
    println!(
        "     identical at every (radix, p, E) in the matrix: {}",
        all
    );
    // and the Abrupt single-binade case, which is what the design says is meaningless
    for &(radix, p) in &[(2i128, 4u32), (10, 2)] {
        let b = ranged_set(radix, p, 0, 0, false);
        println!(
            "     radix={} p={} Ranged<0,0,Abrupt>: |set|={}, contains zero: {}",
            radix,
            p,
            b.len(),
            b.contains(&(0, 1))
        );
    }
}
