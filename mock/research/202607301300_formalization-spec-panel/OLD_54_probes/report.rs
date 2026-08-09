//! A runtime reporter over the same `const fn` model the probes assert against, so the
//! numbers in file 54 are read off a run rather than guessed and then asserted.
//!
//! rustc --edition 2021 -O report.rs -o report && ./report
//!
//! Nothing here is timed. Every figure is a count over an exhaustively enumerated datum
//! space, which is the same computation the `const _: () = assert!(..)` lines in probes 2
//! and 3 perform at compile time; this binary exists only to print it.

#[path = "crossing.rs"]
mod crossing;

use crossing::*;

fn row(name: &str, f: Fmt) {
    println!(
        "{:<34} r={:<3} p={} q=[{},{}] data={:<5} live={:<5} values={:<5} s1={} s2={} s3={} pred={}",
        name,
        f.r,
        f.p,
        f.qmin,
        f.qmax,
        ndata(f),
        live_data(f),
        distinct_values(f),
        s1_decode_after_encode_is_id_on_values(f),
        s2_canonicalisation_is_idempotent(f),
        s3_encode_after_decode_is_id_on_data(f),
        s3_predicted(f),
    );
}

fn base(r: i64, p: u32, qmin: i32, qmax: i32) -> Fmt {
    Fmt {
        r,
        p,
        qmin,
        qmax,
        signed: true,
        inf: false,
        nan_data: 0,
        normalised: r == 2,
        gradual: true,
        neg_zero: true,
        cohort_rule: 0,
    }
}

fn main() {
    println!("== radix two, normalised (hidden digit) ==");
    let mut f = base(2, 3, -4, 1);
    row("binary model, no specials", f);
    f.inf = true;
    row("binary model, InfOnly", f);
    let mut g = base(2, 3, -4, 1);
    g.nan_data = 2;
    row("binary model, NanOnly (2 nan data)", g);
    let mut h = base(2, 3, -4, 1);
    h.inf = true;
    h.nan_data = 2;
    row("binary model, Ieee", h);
    let mut i = base(2, 3, -4, 1);
    i.inf = true;
    i.nan_data = 4;
    row("binary model, Ieee (4 nan data)", i);
    let mut u = base(2, 3, -4, 1);
    u.signed = false;
    u.neg_zero = false;
    row("binary model, unsigned no specials", u);
    let mut z = base(2, 4, -6, 8);
    z.neg_zero = false;
    row("E4M3FNUZ shape (nan on -0 datum)", z);
    let mut a = base(2, 3, -4, 1);
    a.gradual = false;
    row("binary model, abrupt underflow", a);

    println!();
    println!("== radix ten, unnormalised (no hidden digit) ==");
    for p in 1..=3u32 {
        let mut d = base(10, p, -1, 1);
        d.normalised = false;
        row(&format!("decimal model p={p}, cohort=min-sig"), d);
        d.cohort_rule = 1;
        row(&format!("decimal model p={p}, cohort=max-sig"), d);
    }
    let mut d1 = base(10, 2, 0, 0);
    d1.normalised = false;
    row("decimal, single exponent (no cohort)", d1);
    let mut du = base(10, 2, 1, 1);
    du.normalised = false;
    du.signed = false;
    du.neg_zero = false;
    row("decimal, single row, unsigned", du);
    let mut dn = base(10, 2, -1, 1);
    dn.normalised = false;
    dn.inf = true;
    dn.nan_data = 2;
    row("decimal, full IEEE specials", dn);

    println!();
    println!("== radix ten forced normalised (the counterfactual) ==");
    let mut dnorm = base(10, 2, -1, 1);
    dnorm.normalised = true;
    row("decimal p=2 IF normalised", dnorm);

    println!();
    println!("== radix sixteen (IBM hex float shape) ==");
    let mut hx = base(16, 2, -1, 1);
    hx.normalised = false;
    row("hex p=2, unnormalised", hx);

    println!();
    println!("== cohort census: how many data per value, decimal p=3, q in [-2,2] ==");
    let mut c = base(10, 3, -2, 2);
    c.normalised = false;
    let n = ndata(c);
    let mut hist = [0usize; 16];
    let mut d = 0;
    while d < n {
        let v = decode(c, d);
        if v.tag != 0 && encode(c, v) == d {
            // canonical datum: count how many data decode to this value
            let mut k = 0usize;
            let mut e = 0;
            while e < n {
                let w = decode(c, e);
                if w.tag != 0 && val_eq(v, w) {
                    k += 1;
                }
                e += 1;
            }
            if k < 16 {
                hist[k] += 1;
            }
        }
        d += 1;
    }
    for (k, count) in hist.iter().enumerate() {
        if *count > 0 {
            println!("  cohort size {k:>2}: {count} values");
        }
    }
}
