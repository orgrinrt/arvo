// Probe 4. `Underflow = Abrupt` under an unnormalised significand.
//
// `63:328` records the chain: no radix above two has a constant leading digit to
// hide, so its significand is stored unnormalised and a value has one datum per
// representable exponent shift, a cohort. `63:481-482` names "no unnormalised-
// significand cohort" as one of the three things that make `UFixed`/`IFixed`
// injective for free.
//
// The question is what `Abrupt` means there. At radix two, `Abrupt` leaves a hole
// in (0, r^EMIN) and the encoding cannot name a value inside it, because a
// normalised significand at the minimum exponent field already has magnitude at
// least r^EMIN. Under an unnormalised significand that coincidence is gone.
//
// What is measured: over the whole datum set of each encoding, how many data
// decode to a value that is not in the numeral's value set. The crossing
// contract's three statements (`63:186-191`) quantify over values and over data
// round-trips; none of them says decode lands in V(N), and this probe is about
// whether that is free or owed.

#[path = "model.rs"]
mod model;
use model::*;

/// Every datum of the normalised encoding: (m, e), m in [r^(p-1), r^p),
/// e in [EMIN, EMAX]. Value is m * r^(e-p+1).
fn data_normalised(f: &Fmt) -> Vec<Val> {
    let mut out = Vec::new();
    for e in f.emin..=f.emax {
        for m in ipow(f.r, f.p - 1)..ipow(f.r, f.p) {
            out.push(Val {
                m,
                q: e - f.p as i32 + 1,
            });
        }
    }
    out
}

/// Every datum of the unnormalised encoding: (m, Q), m in [1, r^p), Q the
/// quantum exponent field. Q must reach EMIN-p+1 for the smallest normal value
/// to have a full-precision representation, and EMAX-p+1 for the largest.
/// Value is m * r^Q. This is the decimal shape: a value has a cohort.
fn data_unnormalised(f: &Fmt) -> Vec<Val> {
    let mut out = Vec::new();
    for q in (f.emin - f.p as i32 + 1)..=(f.emax - f.p as i32 + 1) {
        for m in 1..ipow(f.r, f.p) {
            out.push(Val { m, q });
        }
    }
    out
}

fn in_value_set(f: &Fmt, v: &Val) -> bool {
    enumerate(f).iter().any(|w| w.eq_exact(v, f.r))
}

fn main() {
    println!("  r   p   Underflow   encoding        data   escaping   example escape");
    for &r in &[2i128, 10] {
        for p in 2u32..=3 {
            for &gradual in &[false, true] {
                for (name, data) in [
                    (
                        "normalised  ",
                        data_normalised(&Fmt {
                            r,
                            p,
                            emin: 0,
                            emax: 2,
                            gradual,
                        }),
                    ),
                    (
                        "unnormalised",
                        data_unnormalised(&Fmt {
                            r,
                            p,
                            emin: 0,
                            emax: 2,
                            gradual,
                        }),
                    ),
                ] {
                    let f = Fmt {
                        r,
                        p,
                        emin: 0,
                        emax: 2,
                        gradual,
                    };
                    let mut escaping = 0u64;
                    let mut first: Option<Val> = None;
                    for d in &data {
                        if !in_value_set(&f, d) {
                            escaping += 1;
                            if first.is_none() {
                                first = Some(*d);
                            }
                        }
                    }
                    println!(
                        " {r:>2}   {p}   {:<9}   {name}   {:>5}   {escaping:>8}   {}",
                        if gradual { "Gradual" } else { "Abrupt" },
                        data.len(),
                        match first {
                            Some(v) => format!("m={} * r^{} ", v.m, v.q),
                            None => "-".to_string(),
                        }
                    );
                }
            }
        }
    }

    // The assertion. Exactly one cell of the (normalisation x Underflow) matrix
    // has data escaping the value set, and it is Abrupt + unnormalised.
    for &r in &[2i128, 10] {
        for p in 2u32..=3 {
            let abrupt = Fmt {
                r,
                p,
                emin: 0,
                emax: 2,
                gradual: false,
            };
            let gradual = Fmt {
                r,
                p,
                emin: 0,
                emax: 2,
                gradual: true,
            };
            let esc = |f: &Fmt, d: &Vec<Val>| d.iter().filter(|v| !in_value_set(f, v)).count();

            assert_eq!(
                esc(&abrupt, &data_normalised(&abrupt)),
                0,
                "normalised+Abrupt should be total"
            );
            assert_eq!(
                esc(&gradual, &data_normalised(&gradual)),
                0,
                "normalised+Gradual should be total"
            );
            assert_eq!(
                esc(&gradual, &data_unnormalised(&gradual)),
                0,
                "unnormalised+Gradual should be total: the hole is filled"
            );
            assert!(
                esc(&abrupt, &data_unnormalised(&abrupt)) > 0,
                "unnormalised+Abrupt should have data decoding into the hole at r={r} p={p}"
            );
        }
    }
    println!("\nexactly one cell of the matrix leaks: Abrupt with an unnormalised significand.");
    println!(
        "under Gradual the same data decode into the subnormal grid, which is in V(N), \
         so nothing escapes and the axis is invisible."
    );
}
