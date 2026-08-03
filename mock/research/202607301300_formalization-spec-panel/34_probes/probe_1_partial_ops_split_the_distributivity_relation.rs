//! Probe 1: file 33's section 6.1 theorem ("on a chain, distributes over the
//! lattice operations iff monotone") has an empty relation slot, and filling it
//! splits the theorem.
//!
//! File 33's own probe 1 quantifies the equivalence over three TOTAL operations
//! (wrap, sat, and a non-monotone control). Its stated hypothesis names
//! totality of the ORDER (`33:526-527`), never totality of the OPERATION. This
//! probe checks the design's one partial operation shape, `Precise` addition
//! (`Refuse` past both range ends), against the three relations file 33 itself
//! imported in its section 1, and finds:
//!
//! CLAIM A. `Precise` addition is monotone where defined (exhaustive).
//!
//! CLAIM B. Distributivity over max holds as a WEAK equation (both sides
//! defined implies equal), exhaustively, under both readings of max below.
//!
//! CLAIM C. Distributivity over max FAILS as a KLEENE equation under the
//! strict reading of max (an undefined operand poisons the max), with the
//! witness a = -5, b = -4, c = 0: the left side is defined (-5) and the right
//! side is undefined because a + b refuses low.
//!
//! CLAIM D. It also fails as a Kleene equation under the suppressing reading
//! of max (an undefined operand is dropped, IEEE maximumNumber's shape), in
//! the complementary direction, witness a = 5, b = -1, c = 4: the left side
//! refuses high (5 + 4 = 9 past the top) while the right side survives via the
//! suppressed branch (5 + -1 = 4).
//!
//! So for a partial operation the two IEEE 754-2019 min/max families (5.10
//! maximum/minimum, strict; maximumNumber/minimumNumber, suppressing) give the
//! law DIFFERENT truth values at the Kleene level, in different directions,
//! and the same truth value at the weak level. The theorem's correct statement
//! carries both a relation slot and a lattice-operation-variant slot.
//!
//! CLAIM E (reification). Replacing `Refuse` with an absorbing error VALUE
//! (the `Specials = WithInfNaN` shape: what refused now delivers NaN) converts
//! the Kleene failure into a WEAK failure on the same witness: both sides are
//! now "defined" and unequal. So the weak/Kleene split is not stable under the
//! refusal-to-special reification the identity contract makes available; the
//! relation a law is stated under must be keyed on the resolution, which file
//! 33's key table already requires for the fold laws and which this extends to
//! the distributivity family.
//!
//! CLAIM F (control, tying to file 33's probe 1). For the TOTAL saturating
//! operation the strict and suppressing max readings coincide and the Kleene
//! and weak relations coincide, exhaustively: the splits above are properties
//! of partiality, not of the model.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1_partial_ops_split_the_distributivity_relation.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

// Same model numeral as file 33's probe 1: sixteen values, quantum 1.
const LO: i32 = -8;
const HI: i32 = 7;
const N: i32 = 16;

const fn nth(i: i32) -> i32 {
    LO + i
}

// A partial result: (defined, value). `value` is meaningless when !defined.
const UNDEF: (bool, i32) = (false, 0);

const fn def(v: i32) -> (bool, i32) {
    (true, v)
}

/// `Precise` addition: exact where the exact sum is representable, `Refuse`
/// past either end.
const fn precise_add(a: i32, b: i32) -> (bool, i32) {
    let s = a + b;
    if s < LO || s > HI {
        UNDEF
    } else {
        def(s)
    }
}

/// Total control: saturating addition (Warm/Cold).
const fn sat_add(a: i32, b: i32) -> (bool, i32) {
    let s = a + b;
    if s < LO {
        def(LO)
    } else if s > HI {
        def(HI)
    } else {
        def(s)
    }
}

const OP_PRECISE: u8 = 0;
const OP_SAT: u8 = 1;

const fn apply(op: u8, a: i32, b: i32) -> (bool, i32) {
    match op {
        OP_PRECISE => precise_add(a, b),
        _ => sat_add(a, b),
    }
}

const fn max2(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Strict max over partial results: an undefined operand poisons the result.
/// IEEE 754-2019 `maximum` shape.
const fn max_strict(x: (bool, i32), y: (bool, i32)) -> (bool, i32) {
    if x.0 && y.0 {
        def(max2(x.1, y.1))
    } else {
        UNDEF
    }
}

/// Suppressing max over partial results: an undefined operand is dropped if
/// the other is defined. IEEE 754-2019 `maximumNumber` shape.
const fn max_suppress(x: (bool, i32), y: (bool, i32)) -> (bool, i32) {
    match (x.0, y.0) {
        (true, true) => def(max2(x.1, y.1)),
        (true, false) => x,
        (false, true) => y,
        (false, false) => UNDEF,
    }
}

/// Kleene equality on partial results: both defined and equal, or both
/// undefined.
const fn kleene_eq(x: (bool, i32), y: (bool, i32)) -> bool {
    if x.0 != y.0 {
        return false;
    }
    !x.0 || x.1 == y.1
}

/// Weak equality: if both defined, equal.
const fn weak_eq(x: (bool, i32), y: (bool, i32)) -> bool {
    !x.0 || !y.0 || x.1 == y.1
}

// ---------------------------------------------------------------------------
// CLAIM A: monotone where defined, exhaustively.
// ---------------------------------------------------------------------------

const fn monotone_where_defined(op: u8) -> bool {
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                let (a, b, c) = (nth(i), nth(j), nth(k));
                if b <= c {
                    let xr = apply(op, a, b);
                    let yr = apply(op, a, c);
                    if xr.0 && yr.0 && xr.1 > yr.1 {
                        return false;
                    }
                    let xl = apply(op, b, a);
                    let yl = apply(op, c, a);
                    if xl.0 && yl.0 && xl.1 > yl.1 {
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

const _: () = assert!(monotone_where_defined(OP_PRECISE));

// ---------------------------------------------------------------------------
// The two sides of distributivity over max, under each max reading.
// LHS: op(a, max(b, c)) with b, c total values, so the max is on values.
// RHS: max(op(a, b), op(a, c)) with the chosen partial-max reading.
// ---------------------------------------------------------------------------

const MAX_STRICT: u8 = 0;
const MAX_SUPPRESS: u8 = 1;

const fn pmax(reading: u8, x: (bool, i32), y: (bool, i32)) -> (bool, i32) {
    match reading {
        MAX_STRICT => max_strict(x, y),
        _ => max_suppress(x, y),
    }
}

const fn lhs(op: u8, a: i32, b: i32, c: i32) -> (bool, i32) {
    apply(op, a, max2(b, c))
}

const fn rhs(op: u8, reading: u8, a: i32, b: i32, c: i32) -> (bool, i32) {
    pmax(reading, apply(op, a, b), apply(op, a, c))
}

/// Count the triples where the law fails under a given relation.
/// relation: 0 = weak, 1 = Kleene.
const fn count_failures(op: u8, reading: u8, relation: u8) -> i32 {
    let mut fails = 0;
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                let (a, b, c) = (nth(i), nth(j), nth(k));
                let l = lhs(op, a, b, c);
                let r = rhs(op, reading, a, b, c);
                let ok = match relation {
                    0 => weak_eq(l, r),
                    _ => kleene_eq(l, r),
                };
                if !ok {
                    fails += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    fails
}

// ---------------------------------------------------------------------------
// CLAIM B: the weak equation holds exhaustively under both max readings.
// ---------------------------------------------------------------------------

const _: () = assert!(count_failures(OP_PRECISE, MAX_STRICT, 0) == 0);
const _: () = assert!(count_failures(OP_PRECISE, MAX_SUPPRESS, 0) == 0);

// ---------------------------------------------------------------------------
// CLAIM C: the Kleene equation fails under strict max, and the witness is the
// low-refusing branch.
// ---------------------------------------------------------------------------

const _: () = assert!(count_failures(OP_PRECISE, MAX_STRICT, 1) > 0);

// The witness, spelled out: a = -5, b = -4, c = 0.
// LHS: precise_add(-5, max(-4, 0)) = precise_add(-5, 0) = -5, defined.
// RHS: max_strict(precise_add(-5, -4), precise_add(-5, 0))
//    = max_strict(UNDEF /* -9 refuses low */, def(-5)) = UNDEF.
const _: () = assert!(lhs(OP_PRECISE, -5, -4, 0).0);
const _: () = assert!(lhs(OP_PRECISE, -5, -4, 0).1 == -5);
const _: () = assert!(!rhs(OP_PRECISE, MAX_STRICT, -5, -4, 0).0);
const _: () = assert!(!kleene_eq(
    lhs(OP_PRECISE, -5, -4, 0),
    rhs(OP_PRECISE, MAX_STRICT, -5, -4, 0)
));

// ---------------------------------------------------------------------------
// CLAIM D: the Kleene equation fails under suppressing max too, in the
// complementary direction: the left side refuses high while the right side
// survives via the suppressed branch.
// ---------------------------------------------------------------------------

const _: () = assert!(count_failures(OP_PRECISE, MAX_SUPPRESS, 1) > 0);

// The witness: a = 5, b = -1, c = 4.
// LHS: precise_add(5, max(-1, 4)) = precise_add(5, 4) = 9, refuses high.
// RHS: max_suppress(precise_add(5, -1), precise_add(5, 4))
//    = max_suppress(def(4), UNDEF) = def(4).
const _: () = assert!(!lhs(OP_PRECISE, 5, -1, 4).0);
const _: () = assert!(rhs(OP_PRECISE, MAX_SUPPRESS, 5, -1, 4).0);
const _: () = assert!(rhs(OP_PRECISE, MAX_SUPPRESS, 5, -1, 4).1 == 4);
const _: () = assert!(!kleene_eq(
    lhs(OP_PRECISE, 5, -1, 4),
    rhs(OP_PRECISE, MAX_SUPPRESS, 5, -1, 4)
));

// ---------------------------------------------------------------------------
// CLAIM E: reifying `Refuse` as an absorbing error value (the Specials shape)
// converts the Kleene failure into a weak failure on the same witness. NAN is
// modelled as a sentinel outside the numeral; add and max both absorb it.
// ---------------------------------------------------------------------------

const NAN: i32 = i32::MIN;

const fn reified_add(a: i32, b: i32) -> i32 {
    if a == NAN || b == NAN {
        return NAN;
    }
    let s = a + b;
    if s < LO || s > HI {
        NAN
    } else {
        s
    }
}

/// Strict (NaN-propagating) max on reified values.
const fn reified_max(a: i32, b: i32) -> i32 {
    if a == NAN || b == NAN {
        NAN
    } else {
        max2(a, b)
    }
}

// The same witness as CLAIM C: a = -5, b = -4, c = 0. Both sides are now
// total, and they disagree on the VALUE: the definedness split became a value
// split, which is a weak-equation failure.
const REIFIED_LHS: i32 = reified_add(-5, max2(-4, 0));
const REIFIED_RHS: i32 = reified_max(reified_add(-5, -4), reified_add(-5, 0));
const _: () = assert!(REIFIED_LHS == -5);
const _: () = assert!(REIFIED_RHS == NAN);
const _: () = assert!(REIFIED_LHS != REIFIED_RHS);

// ---------------------------------------------------------------------------
// CLAIM F: for the total control, everything above coincides: strict and
// suppressing max agree, and the Kleene and weak relations agree, exhaustively.
// ---------------------------------------------------------------------------

const _: () = assert!(count_failures(OP_SAT, MAX_STRICT, 0) == 0);
const _: () = assert!(count_failures(OP_SAT, MAX_STRICT, 1) == 0);
const _: () = assert!(count_failures(OP_SAT, MAX_SUPPRESS, 0) == 0);
const _: () = assert!(count_failures(OP_SAT, MAX_SUPPRESS, 1) == 0);
