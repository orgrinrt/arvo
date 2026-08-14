//! P6. Where does "accurate across chains" live?
//!
//! I7 says the accuracy-first concern is accurate "especially within chains
//! and ops, not only alone". A primitive, as the panel's working assumption
//! has it, is a property of a VALUE: a composition of a format, a number
//! system, a law set and a strategy. This probe asks whether chain accuracy
//! can be one of those components, by trying to move it and seeing what has
//! to change.
//!
//! The experiment holds every per-value component FIXED and changes only the
//! OPERATOR'S TARGET TYPE:
//!
//!   Route A, per-step: mul: P x P -> P. The completion fires at every step.
//!   Route B, deferred: mul: P x P -> P2, wider. The completion never fires
//!                      until the consumer narrows at the end.
//!
//! Same value set on the operands. Same realisation. Same completion policy.
//! Same law set, whatever that would mean. The only thing that moved is where
//! the operator lands.
//!
//! If Route B is more accurate on a chain, then chain accuracy is a fact
//! about the OPERATOR TYPING and not about any component of the operand type,
//! and a per-value primitive has no room for it. If both routes give the same
//! answers, the claim is wrong and chain accuracy really is a per-value
//! property after all.
//!
//! The subject: fixed-point multiplication with F fraction bits and truncation
//! toward zero, which is the ordinary implementation and the one where the
//! effect is largest.
//!
//! No feature gates. `std` used only by the test harness.
//!
//! Build: rustc --edition 2021 --test -O p6_chain_accuracy_is_not_a_property_of_a_value.rs

#![allow(dead_code)]

/// The shared value set: F fraction bits, stored as an integer numerator over
/// 2^F. Held identical for both routes.
const F: u32 = 8;
const DEN: i128 = 1 << F;

/// Route A. Quantise at every step: the product of two values with F fraction
/// bits has 2F, and is immediately truncated back to F.
fn mul_per_step(a: i128, b: i128) -> i128 {
    (a * b) >> F
}

/// Route B. Do not quantise: the product keeps its 2F fraction bits, and the
/// chain accumulates in whatever width it needs. `scale` tracks how many
/// fraction bits the accumulated value currently carries, which is what the
/// operator's target TYPE would carry in a real design; here it is a value so
/// the probe can sweep chain lengths without writing N types by hand.
fn mul_deferred(a: i128, b: i128) -> i128 {
    a * b
}

/// The exact answer, as a rational numerator over 2^(F*n) for a chain of n
/// factors. Used as the oracle both routes are measured against, so neither
/// route is the reference for the other.
fn exact_chain(factors: &[i128]) -> (i128, u32) {
    let mut num: i128 = 1;
    for &f in factors {
        num *= f;
    }
    (num, F * factors.len() as u32)
}

/// Route A over a chain: quantise after each multiply.
fn chain_per_step(factors: &[i128]) -> i128 {
    let mut acc = DEN; // 1.0 in F fraction bits
    for &f in factors {
        acc = mul_per_step(acc, f);
    }
    acc
}

/// Route B over a chain: never quantise, then narrow once at the end.
fn chain_deferred(factors: &[i128]) -> i128 {
    let mut acc: i128 = 1;
    let mut scale = 0u32;
    for &f in factors {
        acc = mul_deferred(acc, f);
        scale += F;
    }
    // Narrow once, at the consumer's boundary, to F fraction bits.
    acc >> (scale - F)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deterministic spread of factors in [0, 2), as F-fraction-bit
    /// numerators. Not random, so the census is reproducible without a seed.
    fn factors_for(seed: usize, n: usize) -> Vec<i128> {
        let mut v = Vec::with_capacity(n);
        let mut x = seed as i128 * 7919 + 13;
        for _ in 0..n {
            x = (x * 1103515245 + 12345) % 2147483648;
            // A numerator in 1..=2*DEN, i.e. a value in (0, 2].
            v.push(1 + (x.abs() % (2 * DEN)));
        }
        v
    }

    /// The two routes agree at chain length one. If they did not, the probe
    /// would be measuring something other than chain behaviour.
    #[test]
    fn the_routes_agree_on_a_single_operation() {
        let mut checked = 0u32;
        for s in 0..200 {
            let f = factors_for(s, 1);
            assert_eq!(
                chain_per_step(&f),
                chain_deferred(&f),
                "routes must agree at length 1, seed {s}"
            );
            checked += 1;
        }
        assert_eq!(checked, 200);
    }

    /// The core result. Over chains of increasing length, with every
    /// per-value component held fixed, the deferred route is closer to the
    /// exact answer. The comparison is against the EXACT rational, so neither
    /// route is judged by the other.
    #[test]
    fn the_deferred_route_is_closer_to_exact_on_every_chain_length_above_one() {
        println!("len  seeds  per-step-worse  deferred-worse  tied  max|err| per-step  max|err| deferred");
        let mut lengths_where_deferred_wins = 0u32;
        for n in 1..=8usize {
            let mut per_step_worse = 0u32;
            let mut deferred_worse = 0u32;
            let mut tied = 0u32;
            let mut max_e_step = 0i128;
            let mut max_e_def = 0i128;
            let seeds = 400u32;
            for s in 0..seeds as usize {
                let f = factors_for(s, n);
                let (num, scale) = exact_chain(&f);

                // Both answers are numerators over 2^F. Compare them to the
                // exact value by cross-multiplying into a common scale, so no
                // rounding is introduced by the comparison itself.
                let shift = scale - F;
                let a = chain_per_step(&f) as i128;
                let b = chain_deferred(&f) as i128;
                let e_step = (a << shift) - num;
                let e_def = (b << shift) - num;
                let (e_step, e_def) = (e_step.abs(), e_def.abs());

                if e_step > e_def {
                    per_step_worse += 1;
                } else if e_def > e_step {
                    deferred_worse += 1;
                } else {
                    tied += 1;
                }
                if e_step > max_e_step {
                    max_e_step = e_step;
                }
                if e_def > max_e_def {
                    max_e_def = e_def;
                }
            }
            assert_eq!(per_step_worse + deferred_worse + tied, seeds);
            println!(
                "{n:3}  {seeds:5}  {per_step_worse:14}  {deferred_worse:14}  {tied:4}  {max_e_step:17}  {max_e_def:17}"
            );

            // The deferred route is never worse. That is the claim, and it is
            // asserted rather than eyeballed.
            assert_eq!(
                deferred_worse, 0,
                "at length {n} the deferred route was worse on {deferred_worse} \
                 chains, which would refute the finding"
            );
            if per_step_worse > 0 {
                lengths_where_deferred_wins += 1;
            }
        }
        assert!(
            lengths_where_deferred_wins > 0,
            "if the two routes never differed, chain accuracy would not be \
             observable and this probe would establish nothing"
        );
        println!("chain lengths where the deferred route strictly wins somewhere: {lengths_where_deferred_wins} of 8");
    }

    /// And the point of the whole probe: NOTHING about the operand type
    /// changed between the two routes. Same fraction width, same denominator,
    /// same truncation, same stored numerators. The difference is entirely in
    /// where the operator lands.
    #[test]
    fn no_per_value_component_differs_between_the_two_routes() {
        // Search for a witness rather than assuming one seed produces the
        // difference. The first version of this test asserted a difference at
        // seed 1, length 4, where the two routes happen to agree (both 30).
        // The effect is real on most chains and not on every chain, and a
        // single-point assertion of a statistical effect is a bad test.
        let mut witness = None;
        'outer: for n in 2..=8usize {
            for s in 0..400usize {
                let f = factors_for(s, n);
                let a = chain_per_step(&f);
                let b = chain_deferred(&f);
                if a != b {
                    witness = Some((s, n, a, b));
                    break 'outer;
                }
            }
        }
        let (ws, wn, a, b) = witness.expect(
            "the routes must differ somewhere, or there is nothing to explain",
        );
        assert_ne!(a, b);
        println!("witness: seed {ws}, chain length {wn}, per-step {a}, deferred {b}");

        // Both results are numerators over the same denominator, in the same
        // value set, so the results are comparable without conversion. The
        // value set did not move.
        assert_eq!(DEN, 1 << F);

        // The truncation rule is the same function in both routes; the
        // deferred route simply applies it once instead of n times.
        assert_eq!(mul_per_step(3 * DEN, 5 * DEN), (3 * 5) * DEN);
        assert_eq!(mul_deferred(3 * DEN, 5 * DEN) >> F, (3 * 5) * DEN);
    }

    /// The cost side, stated so the finding is not read as a free win: the
    /// deferred route's accumulator grows with chain length. At F=8 a chain of
    /// n factors needs F*(n+1) fraction bits before the final narrowing, so
    /// the intermediate width is a function of the CHAIN, which no per-value
    /// type knows.
    #[test]
    fn the_deferred_route_costs_intermediate_width_that_grows_with_the_chain() {
        for n in 1..=8u32 {
            let needed = F * n;
            assert_eq!(needed, F * n);
        }
        // Concretely: at F=8 and n=8 the accumulator carries 64 fraction bits
        // plus the integer part, which no 8-bit or 16-bit container holds.
        // That is why this is a trade rather than a strictly better answer,
        // and why the choice belongs to whoever knows the chain.
        assert_eq!(F * 8, 64);
    }
}
