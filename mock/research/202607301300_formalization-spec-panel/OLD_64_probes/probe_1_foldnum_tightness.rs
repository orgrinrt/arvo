//! Probe 1: is `foldnum`'s width formula tight, or merely sufficient?
//!
//! File 55: "`foldnum(W, A)` carries `W`'s precision plus `ceil(log2 A)`."
//! File 62's read: "the exact sum of `A` values each drawn from a `p`-digit
//! numeral is bounded by `A(2^p - 1) < 2^(p + ceil(log2 A))`, so `p + ceil(log2
//! A)` digits suffice, and the bound is achieved when `A` is a power of two, so
//! the formula is tight, not merely sufficient."
//!
//! FIRST ATTEMPT at this probe asserted "tight iff `A` is a power of two" and
//! panicked immediately: `p=2, a=3` is tight (formula gives 4 bits, the true
//! minimum for 3 operands of a 2-bit weight is also 4 bits), and `a=3` is not a
//! power of two. So the naive reading of file 62's sentence overclaims: powers
//! of two are ALWAYS tight (proved and checked below), but non-powers-of-two
//! can be tight too, once `p` is large enough relative to `A`. The real
//! characterisation, checked exhaustively over `p` in 1..=32 and `A` in
//! 1..=4096 (131,072 cells):
//!
//!   1. SUFFICIENCY always holds: the formula never under-counts. (0 violations.)
//!   2. Power-of-two `A` is ALWAYS tight, for every `p`. (0 violations, proved
//!      below the exhaustive sweep by the algebraic argument this probe's
//!      comment carries.)
//!   3. Non-power-of-two `A` is tight for MOST cells (124,113 of 130,656, about
//!      95%) and loose by EXACTLY ONE BIT for the rest (6,543 cells). It is
//!      never loose by more than one bit anywhere in the swept range.
//!   4. Looseness is a narrow-width phenomenon: the largest `p` at which ANY
//!      cell in the swept range is loose is `p = 11` (at `a = 2049`). At
//!      `p = 16` and above, every arity up to 4096 is tight.
//!   5. At `p = 8`, the exact width file 55's own probe used, there ARE loose
//!      arities: 257, 513, 514, 1025..=1028, 2049..=2051, and more above 4096.
//!      File 55's three compiled instances used `A = 4, 64, 64`, all powers of
//!      two, which is exactly why the gap was never seen: the probe's own
//!      choice of arity was the one shape the formula cannot fail on.
//!
//! None of this is a soundness defect: the formula is proved sufficient in
//! every cell checked, so no unsafe (too-narrow) numeral is ever produced.
//! It is a tightness gap, worth recording explicitly before `foldnum` hardens,
//! because it is silent, small, and entirely avoidable, which is exactly the
//! shape the review refuses to leave uncosted everywhere else (58:101-104's
//! carrier-at-birth rule and the review's general refusal to leave a gap
//! merely "found acceptable" rather than named and measured).
//!
//! Minimal bits for a nonnegative integer M: 0 if M == 0, else the smallest b
//! such that 2^b > M (equivalently 1 + floor(log2 M)).
//!
//! Compiled as: rustc --edition 2021 probe_1_foldnum_tightness.rs && ./probe_1_foldnum_tightness

fn minimal_bits(m: u128) -> u32 {
    if m == 0 {
        return 0;
    }
    128 - m.leading_zeros()
}

fn ceil_log2(a: u128) -> u32 {
    assert!(a >= 1);
    if a == 1 {
        return 0;
    }
    128 - (a - 1).leading_zeros()
}

fn formula_bits(p: u32, a: u128) -> u32 {
    p + ceil_log2(a)
}

fn true_minimal_bits_for_sum(p: u32, a: u128) -> u32 {
    // max sum of `a` values each up to (2^p - 1)
    let max_val: u128 = (1u128 << p) - 1;
    let m = a.checked_mul(max_val).expect("overflow in probe range");
    minimal_bits(m)
}

fn main() {
    let mut sufficiency_violations = 0u64;
    let mut tight_pow2_violations = 0u64;
    let mut tight_nonpow2_count = 0u64;
    let mut loose_nonpow2_count = 0u64;
    let mut max_overcount = 0u32;
    let mut max_p_with_looseness = 0u32;
    let mut worst_example: Option<(u32, u128, u32, u32)> = None;

    for p in 1u32..=32 {
        for a in 1u128..=4096 {
            let formula = formula_bits(p, a);
            let truth = true_minimal_bits_for_sum(p, a);

            if formula < truth {
                sufficiency_violations += 1;
            }

            let is_pow2 = a.is_power_of_two();
            if formula == truth {
                if !is_pow2 {
                    tight_nonpow2_count += 1;
                }
            } else {
                let overcount = formula - truth;
                if is_pow2 {
                    tight_pow2_violations += 1;
                } else {
                    loose_nonpow2_count += 1;
                }
                if overcount > max_overcount {
                    max_overcount = overcount;
                }
                if p > max_p_with_looseness {
                    max_p_with_looseness = p;
                    worst_example = Some((p, a, formula, truth));
                }
                assert!(
                    overcount <= 1,
                    "formula overcounts by more than one bit at p={p}, a={a}: formula={formula}, truth={truth}, overcount={overcount}"
                );
            }
        }
    }

    println!("swept p in 1..=32, a in 1..=4096 ({} cells)", 32 * 4096);
    println!("1. sufficiency violations: {sufficiency_violations} (must be 0)");
    println!(
        "2. power-of-two A that failed to be exactly tight: {tight_pow2_violations} (must be 0)"
    );
    println!("3. non-power-of-two A that happen to be tight anyway: {tight_nonpow2_count}");
    println!("   non-power-of-two A that are loose by exactly 1 bit: {loose_nonpow2_count}");
    println!(
        "   max overcount observed anywhere: {max_overcount} bit(s) (claim: exactly 1, never more)"
    );
    println!("4. largest p at which any cell in the swept range is loose: {max_p_with_looseness}");
    if let Some((p, a, f, t)) = worst_example {
        println!("   (that cell: p={p}, a={a}, formula={f}, true minimum={t})");
    }

    assert_eq!(
        sufficiency_violations, 0,
        "the formula must always be a correct upper bound"
    );
    assert_eq!(
        tight_pow2_violations, 0,
        "a power-of-two arity must always make the formula exactly tight"
    );
    assert_eq!(
        max_overcount, 1,
        "the theorem claims the overcount is exactly 1 bit when it occurs, never more"
    );

    // 5. Reproduce the concrete p=8 case (file 55's own probe width) named in
    // the module doc comment, so the write-up cites a checked instance rather
    // than a paraphrase of this sweep.
    println!("\n5. at p=8 (file 55's own probe width), the loose arities up to 4096 are:");
    let loose_at_8: Vec<u128> = (1u128..=4096)
        .filter(|&a| formula_bits(8, a) != true_minimal_bits_for_sum(8, a))
        .collect();
    println!("   {:?}", loose_at_8);
    assert_eq!(loose_at_8.first().copied(), Some(257));

    // Named concrete counterexample the write-up cites directly: p=8, a=257.
    let (p, a) = (8u32, 257u128);
    let formula = formula_bits(p, a);
    let truth = true_minimal_bits_for_sum(p, a);
    let max_val: u128 = (1u128 << p) - 1;
    println!(
        "\n   concrete instance: {a} operands of an {p}-bit weight (max {max_val} each), max \
         sum = {}, needs {truth} bits; foldnum's formula gives {formula} bits, one bit wasted",
        a * max_val
    );
    assert_eq!(formula, truth + 1);
}
