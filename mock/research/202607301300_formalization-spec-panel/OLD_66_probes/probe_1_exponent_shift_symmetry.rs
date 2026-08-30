// Probe 1. The exponent-offset axis of a `Ranged` numeral transfers by an exact
// symmetry, not by an argument.
//
// Claim under test (E): let F = (r, p, emin, emax, U) and F+k = (r, p, emin+k,
// emax+k, U). Let sigma_k scale an exact value by r^k. Then for every exact
// rational v,
//
//     quantise_{F+k}(sigma_k v) = sigma_k(quantise_F v),   outcomes equal.
//
// If (E) holds, then EMIN and EMAX do not appear in the index set of any claim
// about a `Ranged` numeral: only the span and the operands' positions relative
// to the two ends do. That collapses two of the design's parameters into one and
// it is a proof rather than a sampled check.
//
// Corollary tested separately: addition commutes with sigma_k, so an additive
// claim transfers along this axis. Multiplication does NOT: sigma_k x * sigma_k y
// is sigma_{2k}(x*y), so the equivariant target is a window shifted by 2k, which
// is exactly what the design's `mulnum` already computes.

#[path = "model.rs"]
mod model;
use model::*;

fn shift(f: &Fmt, k: i32) -> Fmt {
    Fmt {
        emin: f.emin + k,
        emax: f.emax + k,
        ..*f
    }
}
fn sigma(v: &Val, k: i32) -> Val {
    Val { m: v.m, q: v.q + k }
}

fn main() {
    let mut checked_e = 0u64;
    let mut fail_e = 0u64;
    let mut checked_add = 0u64;
    let mut fail_add = 0u64;
    let mut checked_mul = 0u64;
    let mut fail_mul = 0u64;

    // The whole matrix of small formats, both underflow policies, both radices
    // the design ships a reason for, and a range of shifts including negative.
    for &r in &[2i128, 10] {
        for p in 2u32..=3 {
            for span in 1i32..=4 {
                for &gradual in &[true, false] {
                    let f = Fmt {
                        r,
                        p,
                        emin: 0,
                        emax: span - 1,
                        gradual,
                    };
                    let vals = enumerate(&f);
                    for &k in &[-3i32, -1, 1, 2, 5] {
                        let fk = shift(&f, k);

                        // (E) on every value and on every exact pairwise sum,
                        // which is the set the quantiser is actually applied to.
                        for x in &vals {
                            for y in &vals {
                                for v in [*x, x.add_exact(y, r)] {
                                    let (a, oa) = quantise(&f, &v);
                                    let (b, ob) = quantise(&fk, &sigma(&v, k));
                                    checked_e += 1;
                                    if oa != ob || !sigma(&a, k).eq_exact(&b, r) {
                                        fail_e += 1;
                                        if fail_e < 4 {
                                            println!(
                                                "  (E) FAIL r={r} p={p} span={span} grad={gradual} \
                                                 k={k} v={:?} -> ({:?},{:?}) vs ({:?},{:?})",
                                                v, a, oa, b, ob
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        // Addition, end to end in the shifted format.
                        for x in &vals {
                            for y in &vals {
                                let (a, oa) = quantise(&f, &x.add_exact(y, r));
                                let sx = sigma(x, k);
                                let sy = sigma(y, k);
                                let (b, ob) = quantise(&fk, &sx.add_exact(&sy, r));
                                checked_add += 1;
                                if oa != ob || !sigma(&a, k).eq_exact(&b, r) {
                                    fail_add += 1;
                                }
                            }
                        }

                        // Multiplication into the mulnum target: window widths add,
                        // so a shift of k on both operands shifts the target by 2k.
                        let fm = Fmt {
                            emin: 2 * f.emin,
                            emax: 2 * f.emax + 1,
                            ..f
                        };
                        let fmk = shift(&fm, 2 * k);
                        for x in &vals {
                            for y in &vals {
                                let (a, oa) = quantise(&fm, &x.mul_exact(y, r));
                                let sx = sigma(x, k);
                                let sy = sigma(y, k);
                                let (b, ob) = quantise(&fmk, &sx.mul_exact(&sy, r));
                                checked_mul += 1;
                                if oa != ob || !sigma(&a, 2 * k).eq_exact(&b, r) {
                                    fail_mul += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("(E) quantiser equivariance : {checked_e} checked, {fail_e} failures");
    println!("(+) addition transfer      : {checked_add} checked, {fail_add} failures");
    println!("(x) mulnum-target transfer : {checked_mul} checked, {fail_mul} failures");

    // Negative control: shift the window but NOT the value. If the check above
    // were vacuous (for instance if quantise ignored emin/emax) this would also
    // pass, and it must not.
    let f = Fmt {
        r: 2,
        p: 3,
        emin: 0,
        emax: 2,
        gradual: false,
    };
    let vals = enumerate(&f);
    let fk = shift(&f, 2);
    let mut control_disagreements = 0u64;
    for x in &vals {
        let (a, oa) = quantise(&f, x);
        let (b, ob) = quantise(&fk, x); // value NOT shifted: deliberately wrong
        if oa != ob || !a.eq_exact(&b, 2) {
            control_disagreements += 1;
        }
    }
    println!(
        "negative control (window shifted, value not): {control_disagreements} disagreements \
         out of {} values; zero here would mean the check above proves nothing",
        vals.len()
    );

    // Second negative control, and it is the load-bearing one: the symmetry needs
    // the value map to be HOMOGENEOUS in the exponent. Add a nonzero additive
    // constant at the value level (a value-level `Bias`, which `Implicit` carries
    // as `B` and `Ranged` does not) and the symmetry dies, because sigma_k does not
    // commute with an affine map whose constant term is not zero.
    // `Ranged` carrying no `Bias` member (63:161-162) is therefore a CONDITION of
    // the transfer argument, not an accident of the table's current shape.
    let mut biased_disagreements = 0u64;
    let mut biased_checked = 0u64;
    let f = Fmt {
        r: 2,
        p: 3,
        emin: 0,
        emax: 3,
        gradual: false,
    };
    let vals = enumerate(&f);
    let bias = Val { m: 1, q: -1 }; // one half, an ordinary representable constant
    for k in [1i32, 2, 3] {
        let fk = shift(&f, k);
        for x in &vals {
            // affine value map: v |-> v + bias, applied before quantising
            let (a, oa) = quantise(&f, &x.add_exact(&bias, 2));
            let (b, ob) = quantise(&fk, &sigma(x, k).add_exact(&bias, 2));
            biased_checked += 1;
            if oa != ob || !sigma(&a, k).eq_exact(&b, 2) {
                biased_disagreements += 1;
            }
        }
    }
    println!(
        "negative control (value map made affine by a nonzero bias):          {biased_disagreements} disagreements out of {biased_checked};          zero here would mean the homogeneity condition is not load-bearing"
    );
    assert!(biased_disagreements > 0);

    assert_eq!(fail_e, 0);
    assert_eq!(fail_add, 0);
    assert_eq!(fail_mul, 0);
    assert!(control_disagreements > 0);
}
