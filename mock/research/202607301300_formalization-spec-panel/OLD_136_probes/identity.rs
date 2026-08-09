//! Is the existence of an identity element in a numeral's value set the same
//! condition as that value set's closure under the operation?
//!
//! MODEL. A numeral's value set is the lattice V = { B + k*A : k in Z } cut to a
//! range, with B the bias and A the adjustment. Bias and adjustment are
//! rationals; writing both over a common denominator, B/A = bn/an, so an integer
//! grid over (bn, an) is fully general for every divisibility question below.
//!
//! CONDITIONS FROM THE STANDING BASE.
//!   additive closure           A | B                     (`110:1472-1473`)
//!   narrowed mult. closure     A,B integers and A | B^2-B (`110:1476-1478`)
//!
//! CONDITIONS THE IDENTITY LAWS NEED.
//!   zero in V                  exists k with B + kA = 0  <=>  A | B
//!   one  in V                  exists k with B + kA = 1  <=>  A | (1 - B)
//!
//! CLAIM A. zero-in-V and additive closure are THE SAME CONDITION, so the
//! additive identity law's existence gate is A1 and needs no new mechanism.
//! CLAIM B. one-in-V and narrowed multiplicative closure are NOT the same
//! condition, in both directions, so the multiplicative identity law's existence
//! gate is genuinely its own.

fn main() {
    let lo = -30i64;
    let hi = 30i64;
    let amax = 30i64;

    let mut checked = 0u64;
    let mut claim_a_fail = 0u64;
    let mut mulclosed_no_one: Vec<(i64, i64)> = Vec::new();
    let mut one_no_mulclosed: Vec<(i64, i64)> = Vec::new();

    for b in lo..=hi {
        for a in 1..=amax {
            checked += 1;

            let zero_in_v = b % a == 0;
            let add_closed = b % a == 0;
            if zero_in_v != add_closed {
                claim_a_fail += 1;
            }

            let one_in_v = (1 - b).rem_euclid(a) == 0;
            // A and B are integers by construction on this grid.
            let mul_narrow_closed = (b * b - b) % a == 0;

            if mul_narrow_closed && !one_in_v && mulclosed_no_one.len() < 6 {
                mulclosed_no_one.push((b, a));
            }
            if one_in_v && !mul_narrow_closed && one_no_mulclosed.len() < 6 {
                one_no_mulclosed.push((b, a));
            }
        }
    }

    println!("(bias, adjustment) pairs checked: {}", checked);
    println!(
        "CLAIM A failures (zero-in-V differs from additive closure): {}",
        claim_a_fail
    );
    println!(
        "CLAIM B, closed under narrowed multiplication with NO representable one: {:?}",
        mulclosed_no_one
    );
    println!(
        "CLAIM B, representable one but NOT closed under narrowed multiplication: {:?}",
        one_no_mulclosed
    );

    // The unbiased case, which is every numeral arvo ships today.
    let b = 0i64;
    println!(
        "unbiased (B=0): zero in V for every A: {}; one in V iff A divides 1, i.e. A == 1: {:?}",
        (1..=amax).all(|a| b % a == 0),
        (1..=amax)
            .filter(|a| (1 - b).rem_euclid(*a) == 0)
            .collect::<Vec<_>>()
    );
}
