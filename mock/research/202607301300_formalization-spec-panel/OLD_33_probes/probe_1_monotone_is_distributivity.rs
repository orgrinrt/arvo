//! Probe 1: on a totally ordered value set, "distributes over the lattice
//! operations" and "monotone in each argument" are the same fact, and no arvo
//! preset is a dioid.
//!
//! Three claims, each checked exhaustively at compile time over a sixteen-value
//! signed model (`AsymmetricLow`, -8 through 7, quantum 1), which is the
//! `SignDomain` instance the settled identity contract names
//! (31_arntzen_settling_the_identity_contract.md:332).
//!
//! CLAIM A. For every binary operation on the model, monotonicity in each
//! argument holds if and only if distributivity over `max` holds, if and only if
//! distributivity over `min` holds. Checked over all three ops below, both
//! directions, so a single op accidentally satisfying both proves nothing: the
//! non-monotone control must fail distributivity, and it does.
//!
//! CLAIM B. The measured inversion in 26_consolidation_two.md:126-137 is
//! reproduced independently here: wrapping addition is associative and does not
//! distribute; saturating addition distributes and is not associative.
//!
//! CLAIM C. Neither is a dioid over (max, op). Wrapping fails distributivity.
//! Saturating fails associativity AND fails the annihilation axiom (the
//! additive identity of max, the bottom element, must annihilate the
//! multiplicative operation). Two independent failures for saturating, so a
//! reader who repaired associativity would still not have a dioid.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1_monotone_is_distributivity.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// The model numeral: AsymmetricLow, sixteen values, quantum 1.
const LO: i32 = -8;
const HI: i32 = 7;
const N: i32 = 16;

const fn nth(i: i32) -> i32 {
    LO + i
}

// ---------------------------------------------------------------------------
// The three operations.
// ---------------------------------------------------------------------------

/// Hot: wrapping addition. A group homomorphism onto Z/16Z.
const fn wrap(a: i32, b: i32) -> i32 {
    let s = a + b;
    let m = (s - LO).rem_euclid(N);
    LO + m
}

/// Warm/Cold: saturating addition. A monotone retraction onto the range.
const fn sat(a: i32, b: i32) -> i32 {
    let s = a + b;
    if s < LO {
        LO
    } else if s > HI {
        HI
    } else {
        s
    }
}

/// Negative control: deliberately non-monotone, and deliberately NOT a
/// quantiser of anything. Exists so that claim A is an equivalence with a
/// witnessed failing side rather than a vacuous agreement between two
/// properties that happen to hold everywhere tested.
const fn control(a: i32, b: i32) -> i32 {
    // Fold the top half of the second argument back down: order-breaking.
    let bb = if b > 0 { HI - b } else { b };
    sat(a, bb)
}

/// Const-tag dispatch. Function pointers cannot be called in a `const fn`
/// (`error: function pointer calls are not allowed in constant functions`), so
/// the operation is selected by a const tag, which is the same Pattern C shape
/// arvo-strategy already uses for container projection.
const OP_WRAP: u8 = 0;
const OP_SAT: u8 = 1;
const OP_CONTROL: u8 = 2;

const fn apply(op: u8, a: i32, b: i32) -> i32 {
    match op {
        OP_WRAP => wrap(a, b),
        OP_SAT => sat(a, b),
        _ => control(a, b),
    }
}

const fn max2(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
const fn min2(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// ---------------------------------------------------------------------------
// The four properties, each an exhaustive const check.
// ---------------------------------------------------------------------------

const fn associative(op: u8) -> bool {
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                let (a, b, c) = (nth(i), nth(j), nth(k));
                if apply(op, apply(op, a, b), c) != apply(op, a, apply(op, b, c)) {
                    return false;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

/// Monotone in each argument separately, which on a chain is the same as
/// monotone in the product order.
const fn monotone(op: u8) -> bool {
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                let (a, b, c) = (nth(i), nth(j), nth(k));
                if b <= c {
                    // monotone in the right argument
                    if apply(op, a, b) > apply(op, a, c) {
                        return false;
                    }
                    // monotone in the left argument
                    if apply(op, b, a) > apply(op, c, a) {
                        return false;
                    }
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const fn distributes_over_max(op: u8) -> bool {
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                let (a, b, c) = (nth(i), nth(j), nth(k));
                if apply(op, a, max2(b, c)) != max2(apply(op, a, b), apply(op, a, c)) {
                    return false;
                }
                if apply(op, max2(b, c), a) != max2(apply(op, b, a), apply(op, c, a)) {
                    return false;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const fn distributes_over_min(op: u8) -> bool {
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                let (a, b, c) = (nth(i), nth(j), nth(k));
                if apply(op, a, min2(b, c)) != min2(apply(op, a, b), apply(op, a, c)) {
                    return false;
                }
                if apply(op, min2(b, c), a) != min2(apply(op, b, a), apply(op, c, a)) {
                    return false;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

/// The dioid annihilation axiom: the additive identity (bottom, for max) must
/// annihilate the multiplicative operation.
const fn bottom_annihilates(op: u8) -> bool {
    let mut i = 0;
    while i < N {
        if apply(op, LO, nth(i)) != LO {
            return false;
        }
        i += 1;
    }
    true
}

/// `max`'s identity on this chain really is the bottom element, so the
/// annihilation check above is asking about the right element.
const fn bottom_is_max_identity() -> bool {
    let mut i = 0;
    while i < N {
        if max2(LO, nth(i)) != nth(i) {
            return false;
        }
        i += 1;
    }
    true
}

// ---------------------------------------------------------------------------
// CLAIM A: monotone <=> distributes over max <=> distributes over min.
// ---------------------------------------------------------------------------

const _: () = assert!(bottom_is_max_identity());

const _: () = assert!(monotone(OP_WRAP) == distributes_over_max(OP_WRAP));
const _: () = assert!(monotone(OP_WRAP) == distributes_over_min(OP_WRAP));
const _: () = assert!(monotone(OP_SAT) == distributes_over_max(OP_SAT));
const _: () = assert!(monotone(OP_SAT) == distributes_over_min(OP_SAT));
const _: () = assert!(monotone(OP_CONTROL) == distributes_over_max(OP_CONTROL));
const _: () = assert!(monotone(OP_CONTROL) == distributes_over_min(OP_CONTROL));

// The equivalence is not vacuous: it has a true instance and a false instance.
const _: () = assert!(monotone(OP_SAT));
const _: () = assert!(!monotone(OP_WRAP));
const _: () = assert!(!monotone(OP_CONTROL));

// ---------------------------------------------------------------------------
// CLAIM B: the inversion.
// ---------------------------------------------------------------------------

const _: () = assert!(associative(OP_WRAP));
const _: () = assert!(!distributes_over_max(OP_WRAP));

const _: () = assert!(!associative(OP_SAT));
const _: () = assert!(distributes_over_max(OP_SAT));
const _: () = assert!(distributes_over_min(OP_SAT));

// ---------------------------------------------------------------------------
// CLAIM C: neither is a dioid over (max, op), and saturating fails twice.
// ---------------------------------------------------------------------------

const fn is_dioid_over_max(op: u8) -> bool {
    associative(op) && distributes_over_max(op) && bottom_annihilates(op)
}

const _: () = assert!(!is_dioid_over_max(OP_WRAP));
const _: () = assert!(!is_dioid_over_max(OP_SAT));

// Saturating's two independent failures, named separately so that repairing
// one is visibly not enough.
const _: () = assert!(!associative(OP_SAT));
const _: () = assert!(!bottom_annihilates(OP_SAT));

// And the specific annihilation witness, so the failure is a number rather
// than a boolean: bottom plus a positive value escapes bottom.
const _: () = assert!(sat(LO, 3) == -5);
const _: () = assert!(sat(LO, 3) != LO);
