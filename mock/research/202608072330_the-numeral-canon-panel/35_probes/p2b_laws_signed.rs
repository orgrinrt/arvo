// p2b: the same law sweep as p2, on SIGNED numerals.
//
// p2 found that unsigned saturating addition is associative at every cell in
// the box, which contradicted the prediction that saturation breaks
// associativity. The reason is that unsigned saturation clamps at one end
// only, and addition of non-negatives is monotone, so both association orders
// collapse to min(a+b+c, MAX).
//
// A signed numeral clamps at both ends, and the classic counterexample
// (MAX + 1) - 1 against MAX + (1 - 1) needs exactly that. So the question is
// whether the reassociation licence p2 established is a property of the
// arithmetic or a property of the sign domain. This probe is the negative
// control for p2's positive result, on the axis p2 held fixed.
//
// Model. Total width W, F fractional bits, two's complement range
// [-2^(W-1), 2^(W-1) - 1]. Same two policies.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p2b p2b_laws_signed.rs && ./p2b

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
}

#[inline(always)]
fn lo(w: u32) -> i128 {
    -(1i128 << (w - 1))
}
#[inline(always)]
fn hi(w: u32) -> i128 {
    (1i128 << (w - 1)) - 1
}

#[inline(always)]
fn clip(x: i128, w: u32, p: Policy) -> i128 {
    match p {
        Policy::Wrap => {
            let m: i128 = 1i128 << w;
            let mut r = x % m;
            if r > hi(w) {
                r -= m;
            }
            if r < lo(w) {
                r += m;
            }
            r
        }
        Policy::Saturate => {
            if x > hi(w) {
                hi(w)
            } else if x < lo(w) {
                lo(w)
            } else {
                x
            }
        }
    }
}

#[inline(always)]
fn add(a: i128, b: i128, w: u32, p: Policy) -> i128 {
    clip(a + b, w, p)
}
#[inline(always)]
fn sub(a: i128, b: i128, w: u32, p: Policy) -> i128 {
    clip(a - b, w, p)
}
#[inline(always)]
fn mul(a: i128, b: i128, w: u32, f: u32, p: Policy) -> i128 {
    // Arithmetic shift right, which floors rather than truncating toward
    // zero. Named because it is a rounding choice and a different one would
    // move the multiplicative counts.
    clip((a * b) >> f, w, p)
}

fn pct(f: u64, t: u64) -> f64 {
    if t == 0 {
        0.0
    } else {
        100.0 * (f as f64) / (t as f64)
    }
}

fn main() {
    println!("w,f,policy,law,failures,total,pct");

    let mut sat_add_assoc_total_fail: u64 = 0;
    let mut wrap_add_assoc_total_fail: u64 = 0;
    let mut first_sat_witness: Option<(u32, u32, i128, i128, i128, i128, i128)> = None;

    for w in 2..=7u32 {
        for f in 0..=w {
            for (p, name) in [(Policy::Wrap, "wrap"), (Policy::Saturate, "saturate")] {
                let (mut aa, mut ma, mut di, mut mo, mut iv) = (0u64, 0u64, 0u64, 0u64, 0u64);
                let (mut aat, mut mat, mut dit, mut mot, mut ivt) = (0u64, 0u64, 0u64, 0u64, 0u64);

                for a in lo(w)..=hi(w) {
                    for b in lo(w)..=hi(w) {
                        for c in lo(w)..=hi(w) {
                            aat += 1;
                            let l = add(add(a, b, w, p), c, w, p);
                            let r = add(a, add(b, c, w, p), w, p);
                            if l != r {
                                aa += 1;
                                if p == Policy::Saturate && first_sat_witness.is_none() {
                                    first_sat_witness = Some((w, f, a, b, c, l, r));
                                }
                            }

                            mat += 1;
                            if mul(mul(a, b, w, f, p), c, w, f, p)
                                != mul(a, mul(b, c, w, f, p), w, f, p)
                            {
                                ma += 1;
                            }

                            dit += 1;
                            if mul(a, add(b, c, w, p), w, f, p)
                                != add(mul(a, b, w, f, p), mul(a, c, w, f, p), w, p)
                            {
                                di += 1;
                            }

                            // monotonicity: a <= b implies a+c <= b+c
                            if a <= b {
                                mot += 1;
                                if add(a, c, w, p) > add(b, c, w, p) {
                                    mo += 1;
                                }
                            }
                        }

                        ivt += 1;
                        if sub(add(a, b, w, p), b, w, p) != a {
                            iv += 1;
                        }
                    }
                }

                if p == Policy::Saturate {
                    sat_add_assoc_total_fail += aa;
                } else {
                    wrap_add_assoc_total_fail += aa;
                }

                println!(
                    "{},{},{},add_assoc,{},{},{:.4}",
                    w,
                    f,
                    name,
                    aa,
                    aat,
                    pct(aa, aat)
                );
                println!(
                    "{},{},{},mul_assoc,{},{},{:.4}",
                    w,
                    f,
                    name,
                    ma,
                    mat,
                    pct(ma, mat)
                );
                println!(
                    "{},{},{},distributivity,{},{},{:.4}",
                    w,
                    f,
                    name,
                    di,
                    dit,
                    pct(di, dit)
                );
                println!(
                    "{},{},{},monotonicity_add,{},{},{:.4}",
                    w,
                    f,
                    name,
                    mo,
                    mot,
                    pct(mo, mot)
                );
                println!(
                    "{},{},{},additive_inverse,{},{},{:.4}",
                    w,
                    f,
                    name,
                    iv,
                    ivt,
                    pct(iv, ivt)
                );
            }
        }
    }

    eprintln!("--- the axis p2 held fixed ---");
    eprintln!(
        "SIGNED saturating addition associativity failures, summed over every (w,f): {}",
        sat_add_assoc_total_fail
    );
    eprintln!(
        "SIGNED wrapping addition associativity failures, summed over every (w,f): {}",
        wrap_add_assoc_total_fail
    );
    if let Some((w, f, a, b, c, l, r)) = first_sat_witness {
        eprintln!(
            "first saturating witness at w={} f={}: (a+b)+c = {} but a+(b+c) = {}, with a={} b={} c={} (raw units)",
            w, f, l, r, a, b, c
        );
    }
}
