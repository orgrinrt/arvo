// Probe 3. The repair to the crossing contract is forced onto the datum side.
// It was not a choice, and this derives that rather than asserting it.
//
// Probe 1 showed statement 2 (`encode . decode`) is ill-typed when decode
// escapes V, because encode's domain is V. There are exactly two ways to make a
// composition typecheck when the middle types do not meet:
//
//   (a) SHRINK the source, so decode's image lands in encode's domain. This is
//       the datum-side repair: the encoding excludes the escaping data, and
//       file 66's statement 0 is the obligation that says so.
//
//   (b) WIDEN the target, so encode accepts everything decode produces. The
//       only candidate the design has is the quantiser (`63:200-206`), which
//       maps an exact rational onto the grid. `encode . quantise . decode`
//       would typecheck if `quantise : Q -> V` were total.
//
// (b) is the one worth checking, because if it worked it would be cheaper: it
// needs no change to the encoding and no new obligation. This probe runs the
// quantiser on every escaping datum of file 66's own leaking cell and asks what
// it returns.
//
// If the quantiser refuses on every escaping datum, (b) is unavailable, and the
// datum-side repair is the only one, which turns "add statement 0" from a
// proposal into a derivation.

#![allow(dead_code)]

#[path = "model.rs"]
mod model;
use model::*;

/// The unnormalised datum set, exactly as file 66's probe 4 builds it: the
/// quantum exponent field must reach EMIN-p+1 for the smallest normal value to
/// have a full-precision representation.
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
    v.is_zero() || enumerate(f).iter().any(|w| w.eq_exact(v, f.r))
}

fn main() {
    println!("   r   p    data   escaping   quantise(escaping) outcomes");
    let mut checked_cells = 0;

    for &r in &[2i128, 10] {
        for p in 2u32..=3 {
            let f = Fmt {
                r,
                p,
                emin: 0,
                emax: 2,
                gradual: false, // Abrupt: the leaking cell
            };
            let ds = data_unnormalised(&f);
            let escaping: Vec<Val> = ds
                .iter()
                .copied()
                .filter(|d| !in_value_set(&f, d))
                .collect();

            let mut n_value = 0usize;
            let mut n_overflow = 0usize;
            let mut n_refused = 0usize;
            // For the escaping data the quantiser DOES resolve (if any), does
            // the resolved value differ from what the datum denotes? If it
            // resolves and agrees, there was no escape to begin with.
            let mut n_value_and_agrees = 0usize;
            for d in &escaping {
                let (out, oc) = quantise(&f, d);
                match oc {
                    Outcome::Value => {
                        n_value += 1;
                        if out.eq_exact(d, f.r) {
                            n_value_and_agrees += 1;
                        }
                    }
                    Outcome::Overflow => n_overflow += 1,
                    Outcome::UnderflowRefused => n_refused += 1,
                }
            }

            println!(
                "  {:2}  {:2}   {:5}   {:8}   Value {:3} / Overflow {:3} / Refused {:3}",
                r,
                p,
                ds.len(),
                escaping.len(),
                n_value,
                n_overflow,
                n_refused
            );

            if !escaping.is_empty() {
                checked_cells += 1;
                // THE RESULT: the quantiser refuses on every single escaping
                // datum. `encode . quantise . decode` is exactly as partial as
                // `encode . decode` was, at exactly the same data.
                assert_eq!(
                    n_refused,
                    escaping.len(),
                    "r={r} p={p}: some escaping datum was NOT refused by the quantiser; \
                     the encode-side repair would then be partially available"
                );
                assert_eq!(n_value, 0);
                assert_eq!(n_value_and_agrees, 0);
            }
        }
    }

    assert!(
        checked_cells >= 2,
        "the probe must exercise at least two leaking cells, else it proves nothing"
    );

    // Negative control. The quantiser is not a function that refuses
    // everything: on the value set itself it returns Value for every element,
    // so "refused on every escaping datum" is a statement about the escaping
    // data and not about the quantiser being broken.
    let f = Fmt {
        r: 10,
        p: 3,
        emin: 0,
        emax: 2,
        gradual: false,
    };
    let vs = enumerate(&f);
    assert!(vs.len() > 100);
    for v in &vs {
        let (out, oc) = quantise(&f, v);
        assert_eq!(oc, Outcome::Value, "quantiser refused a member of V(N)");
        assert!(out.eq_exact(v, f.r), "quantiser moved a member of V(N)");
    }
    println!(
        "\n  negative control: quantise is the identity on all {} values of V(N), r=10 p=3",
        vs.len()
    );

    println!("\n  ALL ASSERTIONS PASSED");
    println!("  => `encode . quantise . decode` is partial at exactly the data where");
    println!("     `encode . decode` was. The encode-side repair does not exist.");
}
