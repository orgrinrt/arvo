//! Probe 4: file 33's law equality ("the equality induced by the composition's
//! total order", `33:196-199`) is only correct if `TotalOrd` is specified at
//! the value level, and the precedent implementation it leans on is not: an
//! IEEE-754-5.10-shaped total order is a DATUM order. The equality the charter
//! actually licenses is the canonical quotient the identity half already
//! built, and it is not the same relation.
//!
//! The charter: "no law may read one [a datum-level operation]" (`31:361-363`).
//! File 33 section 2.2 proposes law equality as the equality induced by
//! `TotalOrd`, citing the shipped trait (`arvo-numeric-contracts/src/lib.rs:65`,
//! "strict-NaN-policy total order"). That trait's declaration does not say
//! which side of the value/datum split it orders. The natural implementation,
//! `f64::total_cmp` / IEEE 754-2019 section 5.10 `totalOrder`, orders DATA:
//! it separates -0 from +0 and orders NaNs by sign and payload. An equality
//! induced by a datum order reads a datum, which the charter forbids in the
//! very definition of law equality.
//!
//! Model: a five-bit sign-magnitude datum (sign, magnitude 0..15). Magnitudes
//! 0..=11 are finite (value = signed magnitude); 12..=15 are NaN data with
//! payload = magnitude - 12. Two zero data (+0, -0) carry one value; eight
//! NaN data carry one value-level special.
//!
//! CLAIM A. The 5.10-shaped total order is total and antisymmetric over the
//! 32 data (a real total order; the model is not a strawman).
//!
//! CLAIM B. Its induced equality separates -0 from +0, and separates two NaN
//! data differing only in payload. Both distinctions are datum facts; a law
//! stated under this equality reads the encoding.
//!
//! CLAIM C. The canonical quotient (compare after `Encoding::Canonical`:
//! -0 canonicalises to +0, every NaN to one canonical NaN) coincides exactly
//! with value equality (equality of decoded values, NaN as one value-level
//! class), over all 32 x 32 datum pairs. This is the crossing contract's
//! idempotent canonicalisation (`31:370-374`) doing double duty: it is not
//! only where signed zero and NaN payloads are collapsed, it is the
//! DEFINITION of law equality, and file 32 already priced it (zero
//! instructions for every `Specials = None` composition, a small branchless
//! constant for a range-based collapse, `32:164-203`).
//!
//! CLAIM D. The difference is observable in a law, not only in a relation
//! table: under a directed rounding the IEEE-mandated result of x - x is -0
//! (roundTowardNegative) against +0 elsewhere (IEEE 754-2019 6.3). The law
//! "x - x equals zero" holds under the canonical quotient for both rounding
//! attributes, and holds under the order-induced equality only for one of
//! them. A law whose truth value depends on the rounding attribute through
//! the SIGN OF A ZERO is reading a datum, which is exactly what the charter
//! exists to forbid.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_4_law_equality_is_the_canonical_quotient.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

// A datum: 0..32. Bit 4 = sign, bits 0..4 = magnitude.
const NDATA: u32 = 32;

const fn sign(d: u32) -> bool {
    d & 16 != 0
}
const fn mag(d: u32) -> u32 {
    d & 15
}
const fn is_nan(d: u32) -> bool {
    mag(d) >= 12
}

// Value level: finite values are signed magnitudes; both zeroes are value 0;
// every NaN is the one value-level special, encoded here as a class tag.
const VAL_NAN: i32 = i32::MAX;

const fn decode(d: u32) -> i32 {
    if is_nan(d) {
        VAL_NAN
    } else if sign(d) {
        -(mag(d) as i32)
    } else {
        mag(d) as i32
    }
}

// ---------------------------------------------------------------------------
// The 5.10-shaped total order: sort by the sign-magnitude reading,
// -NaN < -finite < -0 < +0 < +finite < +NaN, NaNs ordered by payload.
// This is `f64::total_cmp`'s shape on the model.
// ---------------------------------------------------------------------------

const fn order_key(d: u32) -> i32 {
    let m = mag(d) as i32;
    if sign(d) {
        -1 - m // -0 maps to -1: strictly below +0's 0
    } else {
        m
    }
}

const fn total_le(a: u32, b: u32) -> bool {
    order_key(a) <= order_key(b)
}

/// The equality induced by the total order.
const fn order_eq(a: u32, b: u32) -> bool {
    total_le(a, b) && total_le(b, a)
}

// ---------------------------------------------------------------------------
// The canonical quotient: canonicalise, then compare data.
// ---------------------------------------------------------------------------

const CANONICAL_NAN: u32 = 12; // +NaN, payload 0

const fn canonicalize(d: u32) -> u32 {
    if is_nan(d) {
        CANONICAL_NAN
    } else if mag(d) == 0 {
        0 // -0 canonicalises to +0
    } else {
        d
    }
}

const fn canonical_eq(a: u32, b: u32) -> bool {
    canonicalize(a) == canonicalize(b)
}

/// Value equality: equal decoded values (NaN one class).
const fn value_eq(a: u32, b: u32) -> bool {
    decode(a) == decode(b)
}

// ---------------------------------------------------------------------------
// CLAIM A: the order is total and antisymmetric over the data.
// ---------------------------------------------------------------------------

const fn order_is_total_and_antisymmetric() -> bool {
    let mut a = 0;
    while a < NDATA {
        let mut b = 0;
        while b < NDATA {
            if !total_le(a, b) && !total_le(b, a) {
                return false; // not total
            }
            // Antisymmetry over DATA fails only where two data share a key;
            // in this model every datum has a distinct key, which is exactly
            // the 5.10 property (totalOrder distinguishes all data).
            if a != b && order_eq(a, b) {
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

const _: () = assert!(order_is_total_and_antisymmetric());

// ---------------------------------------------------------------------------
// CLAIM B: the induced equality separates one value's data.
// ---------------------------------------------------------------------------

const POS_ZERO: u32 = 0;
const NEG_ZERO: u32 = 16;
const NAN_P0: u32 = 12;
const NAN_P1: u32 = 13;

// One value, two data, and the order-induced equality separates them:
const _: () = assert!(value_eq(POS_ZERO, NEG_ZERO));
const _: () = assert!(!order_eq(POS_ZERO, NEG_ZERO));

// One value-level special, two payloads, separated:
const _: () = assert!(value_eq(NAN_P0, NAN_P1));
const _: () = assert!(!order_eq(NAN_P0, NAN_P1));

// ---------------------------------------------------------------------------
// CLAIM C: the canonical quotient IS value equality, over every datum pair.
// ---------------------------------------------------------------------------

const fn canonical_quotient_is_value_equality() -> bool {
    let mut a = 0;
    while a < NDATA {
        let mut b = 0;
        while b < NDATA {
            if canonical_eq(a, b) != value_eq(a, b) {
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

const _: () = assert!(canonical_quotient_is_value_equality());

// And it is reflexive at NaN, which `PartialEq`-shaped IEEE equality is not;
// reflexivity over all data:
const fn canonical_eq_reflexive() -> bool {
    let mut a = 0;
    while a < NDATA {
        if !canonical_eq(a, a) {
            return false;
        }
        a += 1;
    }
    true
}
const _: () = assert!(canonical_eq_reflexive());

// ---------------------------------------------------------------------------
// CLAIM D: the law "x - x equals zero" under the two equalities, across the
// two rounding attributes' mandated zero signs (IEEE 754-2019 6.3: an exact
// zero result from subtraction of equal operands is +0 except under
// roundTowardNegative, where it is -0).
// ---------------------------------------------------------------------------

const fn sub_self(rounding_toward_negative: bool) -> u32 {
    if rounding_toward_negative {
        NEG_ZERO
    } else {
        POS_ZERO
    }
}

// Canonical quotient: the law holds under both attributes.
const _: () = assert!(canonical_eq(sub_self(false), POS_ZERO));
const _: () = assert!(canonical_eq(sub_self(true), POS_ZERO));

// Order-induced equality: the law's truth value depends on the attribute,
// through nothing but the sign of a zero, a datum fact.
const _: () = assert!(order_eq(sub_self(false), POS_ZERO));
const _: () = assert!(!order_eq(sub_self(true), POS_ZERO));
