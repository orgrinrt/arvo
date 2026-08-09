//! Identity existence against closure, checked by SEARCH rather than by the
//! closed forms, because the first version of this compared `b % a == 0` to
//! `b % a == 0` and was structurally incapable of failing.
//!
//! MODEL. A numeral's lattice is V = { B + k*A : k in Z }. B and A are
//! rationals; write both over a common denominator d, so a value v is
//! represented by the integer v*d and V = { (bn + k*an)/d }.
//!
//! Nothing below uses a divisibility formula. `zero_in_v` searches for a k.
//! `add_closed` searches for a k3 realising every pairwise sum. `one_in_v`
//! searches for a k. `mul_closed` searches for a k3 realising every pairwise
//! product. The design's stated closed forms (`110:1472-1473`,
//! `110:1476-1478`) are then compared against the searched answers, so a
//! disagreement is reportable rather than definitionally impossible.

const KSEARCH: i64 = 4000;
const KOP: i64 = 8;

/// Is `target*d` a lattice point?
fn in_v(bn: i64, an: i64, d: i64, target_times_d: i64) -> bool {
    let mut k = -KSEARCH;
    while k <= KSEARCH {
        if bn + k * an == target_times_d {
            return true;
        }
        k += 1;
    }
    false
}

/// Is every pairwise sum of lattice points a lattice point?
fn add_closed(bn: i64, an: i64, d: i64) -> bool {
    for k1 in -KOP..=KOP {
        for k2 in -KOP..=KOP {
            let sum = (bn + k1 * an) + (bn + k2 * an);
            if !in_v(bn, an, d, sum) {
                return false;
            }
        }
    }
    true
}

/// Is every pairwise product of lattice points a lattice point? A product of
/// two values each scaled by d is scaled by d*d, so it is renormalised by d.
fn mul_closed(bn: i64, an: i64, d: i64) -> bool {
    for k1 in -KOP..=KOP {
        for k2 in -KOP..=KOP {
            let p = (bn + k1 * an) * (bn + k2 * an);
            if p % d != 0 {
                return false;
            }
            if !in_v(bn, an, d, p / d) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut n = 0u64;
    let mut a_disagree: Vec<(i64, i64, i64)> = Vec::new();
    let mut b_one_no_mul: Vec<(i64, i64, i64)> = Vec::new();
    let mut b_mul_no_one: Vec<(i64, i64, i64)> = Vec::new();

    for d in 1..=4i64 {
        for bn in -12..=12i64 {
            for an in 1..=12i64 {
                n += 1;
                let zero = in_v(bn, an, d, 0);
                let ac = add_closed(bn, an, d);
                if zero != ac {
                    a_disagree.push((bn, an, d));
                }

                let one = in_v(bn, an, d, d);
                let mc = mul_closed(bn, an, d);
                if one && !mc && b_one_no_mul.len() < 8 {
                    b_one_no_mul.push((bn, an, d));
                }
                if mc && !one && b_mul_no_one.len() < 8 {
                    b_mul_no_one.push((bn, an, d));
                }
            }
        }
    }

    println!("(bias_num, adj_num, denom) triples checked: {}", n);
    println!(
        "CLAIM A, zero-in-V disagrees with additive closure at: {:?}  (count {})",
        a_disagree,
        a_disagree.len()
    );
    println!(
        "CLAIM B, representable one but NOT multiplicatively closed: {:?}",
        b_one_no_mul
    );
    println!(
        "CLAIM B, multiplicatively closed but NO representable one: {:?}",
        b_mul_no_one
    );

    // The two shapes the design names by hand.
    // UFixed<0, F> unbiased dyadic: bias 0, adjustment 2^-F. Over denominator
    // d = 2^F the lattice is every integer, so one IS a lattice point and the
    // exclusion is the RANGE, not the lattice.
    for f in 1..=4u32 {
        let d = 1i64 << f;
        println!(
            "UFixed<0,{}>: lattice contains one: {}; range is [0,1) so the value \
set does not, missing it by one quantum",
            f,
            in_v(0, 1, d, d)
        );
    }
}
