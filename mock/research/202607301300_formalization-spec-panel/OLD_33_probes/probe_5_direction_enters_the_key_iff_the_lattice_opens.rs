//! Probe 5: `Direction` belongs in a law's key exactly when the operation's
//! exact result can leave the operand lattice, and that is a computable
//! predicate over the numerals rather than a judgement.
//!
//! The settled identity contract splits quantisation into two stages: round on
//! the unbounded-exponent extension of the grid, then classify the rounded
//! result against the range and resolve
//! (31_arntzen_settling_the_identity_contract.md:378-384). The two stages read
//! different axes. `Direction` drives the first; `Resolution` drives the second.
//!
//! The consolidation observes, for one case each, that `Precise` addition never
//! rounds in range while `Precise` multiplication rounds on roughly half of
//! pairs (26_consolidation_two.md:167-172), and treats it as a measurement about
//! two operations. It is a consequence of one predicate.
//!
//! CLAIM A. For addition on a common numeral with `adjustment | bias`, the four
//! `Direction` instances produce identical results at every operand pair. The
//! round stage is the identity, so `Direction` cannot appear in any additive
//! law's key, and this is provable per numeral rather than measured per preset.
//!
//! CLAIM B. For multiplication narrowed back into the operand numeral, the four
//! directions disagree on a large fraction of operand pairs. `Direction` is in
//! the key.
//!
//! CLAIM C. For `mul_full`, which lands in the product numeral rather than the
//! operand numeral, the round stage is the identity again and `Direction` leaves
//! the key. The multiplicative half's headline (relocate the quantiser and the
//! laws come back) is the same predicate flipping, not a separate finding.
//!
//! CLAIM D. Both closure conditions are one-line predicates over the numeral,
//! checked against direct exhaustive computation in both directions. Additive
//! closure is `bias/adjustment is an integer`, of which the shipped `AddClosed`
//! gate on `Bias = Zero` (26_consolidation_two.md:326-331) is the special case.
//! Narrowed-multiplicative closure additionally requires the adjustment itself
//! to be an integer, which no fixed-point numeral with a fractional digit
//! satisfies. That single condition is why multiplication needs `mul_full` and
//! addition does not.
//!
//! CLAIM F. Every `Direction` instance is monotone, and a wrapping resolution is
//! not. So monotonicity of a whole quantiser is computable from its axis
//! instances, which is what lets `Monotone` be a derived safe impl under D51
//! rather than an asserted `unsafe impl` under D16.
//!
//! Values are handled as exact rationals in units of the numeral's own quantum,
//! so nothing here depends on a radix and no floating point appears.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_5_direction_enters_the_key_iff_the_lattice_opens.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Rounding a rational n/d to an integer, one arm per Direction instance.
// ---------------------------------------------------------------------------

const DIR_NEG: u8 = 0; // TowardNegative
const DIR_POS: u8 = 1; // TowardPositive
const DIR_ZERO: u8 = 2; // TowardZero
const DIR_EVEN: u8 = 3; // ToEven (ties to even)
const DIR_COUNT: u8 = 4;

const fn floor_div(n: i64, d: i64) -> i64 {
    // d > 0 throughout.
    let q = n / d;
    if n % d != 0 && n < 0 {
        q - 1
    } else {
        q
    }
}

const fn round_rational(dir: u8, n: i64, d: i64) -> i64 {
    match dir {
        DIR_NEG => floor_div(n, d),
        DIR_POS => {
            let f = floor_div(n, d);
            if f * d == n {
                f
            } else {
                f + 1
            }
        }
        DIR_ZERO => n / d, // Rust integer division truncates toward zero
        _ => {
            // Nearest, ties to even.
            let f = floor_div(n, d);
            let r = n - f * d; // 0 <= r < d
            let twice = 2 * r;
            if twice < d {
                f
            } else if twice > d {
                f + 1
            } else if f % 2 == 0 {
                f
            } else {
                f + 1
            }
        }
    }
}

// Sanity: the four directions are genuinely four different functions, and each
// agrees with the others on exact values. Without this the claims below could
// pass by every direction being the same code.
const _: () = assert!(round_rational(DIR_NEG, 7, 4) == 1);
const _: () = assert!(round_rational(DIR_POS, 7, 4) == 2);
const _: () = assert!(round_rational(DIR_ZERO, -7, 4) == -1);
const _: () = assert!(round_rational(DIR_NEG, -7, 4) == -2);
const _: () = assert!(round_rational(DIR_EVEN, 6, 4) == 2); // tie at 1.5 -> 2
const _: () = assert!(round_rational(DIR_EVEN, 2, 4) == 0); // tie at 0.5 -> 0
const _: () = assert!(round_rational(DIR_NEG, 8, 4) == 2);
const _: () = assert!(round_rational(DIR_POS, 8, 4) == 2);

// ---------------------------------------------------------------------------
// The numeral. Values are `A*k + B`; k ranges over a window. Everything below is
// in units of the quantum where that is exact, and as an explicit rational where
// it is not.
// ---------------------------------------------------------------------------

const KLO: i64 = -8;
const KHI: i64 = 7;
const KSPAN: i64 = KHI - KLO + 1;

/// The predicates, over a numeral whose adjustment (quantum) and bias are
/// rationals. Both are derived below rather than asserted, and both are checked
/// against direct exhaustive computation in claim D.
///
/// Additive closure: `(q k1 + b) + (q k2 + b) = q(k1+k2) + 2b` lies in
/// `{q m + b}` exactly when `b/q` is an integer.
///
/// Narrowed-multiplicative closure: `(q k1 + b)(q k2 + b)` lies in `{q m + b}`
/// exactly when `q` is an integer, `b` is an integer, and `q` divides `b^2 - b`.
/// Setting k1 = k2 = 0, then (1,0), then (1,1) in
/// `m = q*k1*k2 + b*(k1+k2) + (b^2-b)/q` forces the three conditions in turn.
///
/// The consequence the design cares about needs no case analysis: every
/// fixed-point numeral with at least one fractional digit has `q < 1`, so `q` is
/// not an integer, so narrowed multiplication is never lattice-closed for any of
/// them. Addition is closed for all of them at zero bias. That is the whole
/// asymmetry, in one line each.
const fn add_lattice_closed(qn: i64, qd: i64, bn: i64, bd: i64) -> bool {
    // b/q integer  <=>  (bn*qd) divisible by (bd*qn)
    (bn * qd) % (bd * qn) == 0
}

const fn is_integer(n: i64, d: i64) -> bool {
    n % d == 0
}

const fn mul_narrow_lattice_closed(qn: i64, qd: i64, bn: i64, bd: i64) -> bool {
    if !is_integer(qn, qd) || !is_integer(bn, bd) {
        return false;
    }
    let q = qn / qd;
    let b = bn / bd;
    (b * b - b) % q == 0
}

// ---------------------------------------------------------------------------
// CLAIM A: addition, zero bias. All four directions agree everywhere.
// ---------------------------------------------------------------------------

/// Addition in units of the quantum: the exact sum of `A*k1` and `A*k2` is
/// `A*(k1+k2)`, so as a rational in quantum units it is `(k1+k2)/1`.
const fn add_units(k1: i64, k2: i64) -> (i64, i64) {
    (k1 + k2, 1)
}

const fn directions_agree_on_add() -> bool {
    let mut k1 = KLO;
    while k1 <= KHI {
        let mut k2 = KLO;
        while k2 <= KHI {
            let (n, d) = add_units(k1, k2);
            let base = round_rational(DIR_NEG, n, d);
            let mut dir = 1u8;
            while dir < DIR_COUNT {
                if round_rational(dir, n, d) != base {
                    return false;
                }
                dir += 1;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

const _: () = assert!(add_lattice_closed(1, 4, 0, 1));
const _: () = assert!(directions_agree_on_add());

// ---------------------------------------------------------------------------
// CLAIM B: narrowed multiplication. The directions disagree, a lot.
// ---------------------------------------------------------------------------

/// Narrowed multiplication in units of the quantum, zero bias. The exact product
/// of `A*k1` and `A*k2` is `A^2*k1*k2`, which is `A*k1*k2` quanta, and forcing it
/// back into the operand numeral asks for `k1*k2*A` expressed as an integer
/// number of quanta after dividing by the quantum's own denominator. With the
/// quantum written as `1/DEN` of the integer unit, that rational is
/// `k1*k2 / DEN`.
const DEN: i64 = 4;

const fn mul_narrow_units(k1: i64, k2: i64) -> (i64, i64) {
    (k1 * k2, DEN)
}

const fn direction_disagreements_on_mul() -> i64 {
    let mut count = 0;
    let mut k1 = KLO;
    while k1 <= KHI {
        let mut k2 = KLO;
        while k2 <= KHI {
            let (n, d) = mul_narrow_units(k1, k2);
            let base = round_rational(DIR_NEG, n, d);
            let mut differs = false;
            let mut dir = 1u8;
            while dir < DIR_COUNT {
                if round_rational(dir, n, d) != base {
                    differs = true;
                }
                dir += 1;
            }
            if differs {
                count += 1;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    count
}

const MUL_DISAGREEMENTS: i64 = direction_disagreements_on_mul();
const PAIRS: i64 = KSPAN * KSPAN;

const _: () = assert!(MUL_DISAGREEMENTS > 0);
// A substantial fraction, not a corner: over a third of all operand pairs.
const _: () = assert!(MUL_DISAGREEMENTS * 3 > PAIRS);

// ---------------------------------------------------------------------------
// CLAIM C: mul_full lands in the product numeral, and the round stage is the
// identity again.
// ---------------------------------------------------------------------------

/// The exact product measured in units of the PRODUCT numeral's own quantum
/// (`A^2` rather than `A`) is the integer `k1*k2`, denominator one.
const fn mul_full_units(k1: i64, k2: i64) -> (i64, i64) {
    (k1 * k2, 1)
}

const fn directions_agree_on_mul_full() -> bool {
    let mut k1 = KLO;
    while k1 <= KHI {
        let mut k2 = KLO;
        while k2 <= KHI {
            let (n, d) = mul_full_units(k1, k2);
            let base = round_rational(DIR_NEG, n, d);
            let mut dir = 1u8;
            while dir < DIR_COUNT {
                if round_rational(dir, n, d) != base {
                    return false;
                }
                dir += 1;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

const _: () = assert!(directions_agree_on_mul_full());

// The same operation, the same operands, two destinations: `Direction` is in the
// key of one and not of the other. So it is not keyed on the operation alone,
// and not on the numeral alone, but on the pair, which is exactly the shape the
// consolidation reached for the recovery-map classification by measurement
// (26_consolidation_two.md:765-768).
const _: () = assert!(directions_agree_on_mul_full() && MUL_DISAGREEMENTS > 0);

// ---------------------------------------------------------------------------
// CLAIM D: the two closure predicates, checked against direct computation.
// ---------------------------------------------------------------------------

/// Direct check for addition over a grid of the free integer k, with the numeral
/// held as exact rationals scaled by a common denominator so no division is
/// approximate anywhere.
const fn add_closure_holds(qn: i64, qd: i64, bn: i64, bd: i64) -> bool {
    // Work in units of 1/(qd*bd) so every value below is an exact integer.
    let u = qd * bd;
    let q = qn * bd; // q in units of 1/u
    let b = bn * qd; // b in units of 1/u
    let _ = u;
    let mut k1 = KLO;
    while k1 <= KHI {
        let mut k2 = KLO;
        while k2 <= KHI {
            let s = (q * k1 + b) + (q * k2 + b);
            if (s - b) % q != 0 {
                return false;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

/// Direct check for narrowed multiplication. The product of two quantities
/// measured in units of `1/u` is measured in units of `1/u^2`, so it is scaled
/// back by `u` before being asked whether it lands on the operand lattice. That
/// rescaling is the whole reason multiplication behaves differently from
/// addition, and writing it explicitly is what keeps the check honest.
const fn mul_closure_holds(qn: i64, qd: i64, bn: i64, bd: i64) -> bool {
    let u = qd * bd;
    let q = qn * bd;
    let b = bn * qd;
    let mut k1 = KLO;
    while k1 <= KHI {
        let mut k2 = KLO;
        while k2 <= KHI {
            let p = (q * k1 + b) * (q * k2 + b); // units of 1/u^2
            if p % u != 0 {
                return false; // not even on the fine grid
            }
            let p = p / u; // units of 1/u
            if (p - b) % q != 0 {
                return false;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

/// Both predicates agree with direct computation across a grid of rational
/// numerals, in both directions, so each is a characterisation rather than a
/// one-sided sufficient condition.
const fn predicates_match_reality() -> bool {
    let mut qn = 1;
    while qn <= 4 {
        let mut qd = 1;
        while qd <= 4 {
            let mut bn = 0;
            while bn <= 8 {
                let mut bd = 1;
                while bd <= 2 {
                    if add_lattice_closed(qn, qd, bn, bd) != add_closure_holds(qn, qd, bn, bd) {
                        return false;
                    }
                    if mul_narrow_lattice_closed(qn, qd, bn, bd)
                        != mul_closure_holds(qn, qd, bn, bd)
                    {
                        return false;
                    }
                    bd += 1;
                }
                bn += 1;
            }
            qd += 1;
        }
        qn += 1;
    }
    true
}

const _: () = assert!(predicates_match_reality());

// The fixed-point case, named directly: quantum 1/4, zero bias. Additively
// closed, multiplicatively open. This is every `UFixed<I, F>` with F > 0.
const _: () = assert!(add_lattice_closed(1, 4, 0, 1));
const _: () = assert!(!mul_narrow_lattice_closed(1, 4, 0, 1));

// An integer-quantum numeral is closed under both, which is why the asymmetry is
// a fact about the numeral and not a fact about multiplication as such.
const _: () = assert!(add_lattice_closed(4, 1, 0, 1));
const _: () = assert!(mul_narrow_lattice_closed(4, 1, 0, 1));

// The shipped `AddClosed` gate on `Bias = Zero` is the special case: there are
// numerals with nonzero bias that are additively closed and that the shipped
// gate would refuse.
const _: () = assert!(add_lattice_closed(4, 1, 8, 1));
const _: () = assert!(!add_lattice_closed(4, 1, 2, 1));

// ---------------------------------------------------------------------------
// CLAIM F: every `Direction` instance is monotone as a map from the exact line
// onto the grid. This is what makes `Monotone` a DERIVED property of a
// composition (D51's blanket-impl shape, a plain safe impl under D16) for every
// quantiser arvo builds out of a Direction plus a clamping Resolution, rather
// than an asserted one requiring `unsafe impl`.
// ---------------------------------------------------------------------------

const fn direction_is_monotone(dir: u8, d: i64) -> bool {
    let mut n = -64;
    while n <= 64 {
        let mut m = n;
        while m <= 64 {
            if round_rational(dir, n, d) > round_rational(dir, m, d) {
                return false;
            }
            m += 1;
        }
        n += 1;
    }
    true
}

const fn all_directions_monotone() -> bool {
    let mut dir = 0u8;
    while dir < DIR_COUNT {
        let mut d = 1;
        while d <= 8 {
            if !direction_is_monotone(dir, d) {
                return false;
            }
            d += 1;
        }
        dir += 1;
    }
    true
}

const _: () = assert!(all_directions_monotone());

// The negative control: a rule that is not order preserving, so the check above
// is capable of failing. This is `ReduceModulo`'s shape at the range ends, which
// is exactly the resolution that costs a composition its monotonicity.
const fn wrap_to_window(n: i64, d: i64) -> i64 {
    let v = floor_div(n, d);
    ((v + 8).rem_euclid(16)) - 8
}

const fn wrap_is_monotone() -> bool {
    let mut n = -64;
    while n <= 64 {
        let mut m = n;
        while m <= 64 {
            if wrap_to_window(n, 1) > wrap_to_window(m, 1) {
                return false;
            }
            m += 1;
        }
        n += 1;
    }
    true
}

const _: () = assert!(!wrap_is_monotone());

/// Present so the same file can be compiled as a binary and print the counts.
fn main() {
    println!(
        "narrowed-multiply direction disagreements: {} of {} operand pairs",
        MUL_DISAGREEMENTS, PAIRS
    );
    println!("addition: all four directions agree at every pair");
    println!("mul_full: all four directions agree at every pair");
}
