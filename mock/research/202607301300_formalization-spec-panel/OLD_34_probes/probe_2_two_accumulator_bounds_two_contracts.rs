//! Probe 2: the consolidation's two accumulator formulas are not "two
//! spellings of one condition" (file 33, section 4.2); they are two different
//! contracts, one digit apart, and the design needs both, named.
//!
//! `26_consolidation_two.md:157-158` gives `acc >= width + ceil(log2(n-1))`
//! (interior safety). `26:269` gives `acc >= width + ceil(log2 n)`. File 33
//! reads the second as a one-digit-wider respelling of the first and proposes
//! shipping only the first as the definition. This probe shows they back
//! different promises:
//!
//! CLAIM A (lawfulness). At the n-1 bound, every grouping of the fold agrees
//! with every other, exhaustively: interior safety delivers grouping
//! invariance, exactly as file 33's section 4.3 proves. This is the contract
//! the LAW needs.
//!
//! CLAIM B (specification). At the n-1 bound, the fold is NOT the function
//! `quantize . exact_sum`: there is an input whose exact total escapes the
//! accumulator, so the fold refuses (grouping-independently), while the
//! destination's own clamp resolution specifies delivery of the clamped
//! total. Witness: four elements of 7 on a [0,7] destination; total 28,
//! accumulator top 21; every grouping refuses; the specification says 7.
//!
//! CLAIM C. At the n bound, the fold IS `quantize . exact_sum`, exhaustively.
//! This is the contract the OPERATION'S SPECIFICATION needs, and it is the
//! contract the DSP-silicon sizing in `26:269-271` (eight guard bits for 256
//! MAC steps, `ceil(log2 256)`) actually encodes.
//!
//! So: interior safety (n-1) is the side condition on the fold LAW; total
//! safety (n) is the side condition on the fold's agreement with its own
//! specification. A consumer who wants both pays the extra digit. A spec that
//! ships only the n-1 form as "the definition" silently weakens the second
//! promise for every total resolution, because a refusal the destination
//! would have absorbed (clamp) surfaces as an accumulator refusal instead.
//!
//! Model: destination numeral [0, 7], quantum 1 (an eight-value model, matching the consolidation's measured const-eval budget: the first draft of this probe at [0, 15] was refused by #[deny(long_running_const_eval)], reproducing the 26 section 1.3 cliff), clamp-at-top resolution.
//! Fold arity 4, all five bracketings of four leaves, exact interior addition
//! in an accumulator [0, ACC_HI]; an interior or root sum past ACC_HI refuses.
//! Exhaustive over all 8^4 = 4096 inputs, both accumulator widths.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2_two_accumulator_bounds_two_contracts.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

const N_LO: i32 = 0;
const N_HI: i32 = 7;
const ARITY: i32 = 4;

// The two accumulator tops under test. Interior safety: (n-1) * max = 21.
// Total safety: n * max = 28.
const ACC_INTERIOR: i32 = (ARITY - 1) * N_HI; // 21
const ACC_TOTAL: i32 = ARITY * N_HI; // 28

const UNDEF: (bool, i32) = (false, 0);

/// Exact addition in the accumulator: refuses past the accumulator top.
/// (The low end cannot be reached on a nonnegative model.)
const fn acc_add(acc_hi: i32, x: (bool, i32), y: (bool, i32)) -> (bool, i32) {
    if !x.0 || !y.0 {
        return UNDEF;
    }
    let s = x.1 + y.1;
    if s > acc_hi {
        UNDEF
    } else {
        (true, s)
    }
}

/// The store quantiser: clamp at the top (Warm's shape on this model).
const fn quantize_clamp(x: (bool, i32)) -> (bool, i32) {
    if !x.0 {
        return UNDEF;
    }
    if x.1 > N_HI {
        (true, N_HI)
    } else {
        (true, x.1)
    }
}

/// The five bracketings of a four-element fold, interior in the accumulator,
/// one quantisation at the root.
const fn fold(acc_hi: i32, shape: u8, a: i32, b: i32, c: i32, d: i32) -> (bool, i32) {
    let (pa, pb, pc, pd) = ((true, a), (true, b), (true, c), (true, d));
    let raw = match shape {
        0 => acc_add(acc_hi, acc_add(acc_hi, acc_add(acc_hi, pa, pb), pc), pd),
        1 => acc_add(acc_hi, acc_add(acc_hi, pa, acc_add(acc_hi, pb, pc)), pd),
        2 => acc_add(acc_hi, acc_add(acc_hi, pa, pb), acc_add(acc_hi, pc, pd)),
        3 => acc_add(acc_hi, pa, acc_add(acc_hi, acc_add(acc_hi, pb, pc), pd)),
        _ => acc_add(acc_hi, pa, acc_add(acc_hi, pb, acc_add(acc_hi, pc, pd))),
    };
    quantize_clamp(raw)
}

/// The operation's own specification: clamp the exact total.
const fn spec(a: i32, b: i32, c: i32, d: i32) -> (bool, i32) {
    quantize_clamp((true, a + b + c + d))
}

const fn kleene_eq(x: (bool, i32), y: (bool, i32)) -> bool {
    if x.0 != y.0 {
        return false;
    }
    !x.0 || x.1 == y.1
}

/// All five groupings agree with each other, over every input.
const fn grouping_invariant(acc_hi: i32) -> bool {
    let mut a = N_LO;
    while a <= N_HI {
        let mut b = N_LO;
        while b <= N_HI {
            let mut c = N_LO;
            while c <= N_HI {
                let mut d = N_LO;
                while d <= N_HI {
                    let r0 = fold(acc_hi, 0, a, b, c, d);
                    let mut s = 1;
                    while s < 5 {
                        if !kleene_eq(r0, fold(acc_hi, s, a, b, c, d)) {
                            return false;
                        }
                        s += 1;
                    }
                    d += 1;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

/// Every grouping agrees with the specification, over every input.
const fn matches_spec(acc_hi: i32) -> bool {
    let mut a = N_LO;
    while a <= N_HI {
        let mut b = N_LO;
        while b <= N_HI {
            let mut c = N_LO;
            while c <= N_HI {
                let mut d = N_LO;
                while d <= N_HI {
                    let sp = spec(a, b, c, d);
                    let mut s = 0;
                    while s < 5 {
                        if !kleene_eq(sp, fold(acc_hi, s, a, b, c, d)) {
                            return false;
                        }
                        s += 1;
                    }
                    d += 1;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

// ---------------------------------------------------------------------------
// CLAIM A: the n-1 bound delivers grouping invariance, exhaustively.
// ---------------------------------------------------------------------------

const _: () = assert!(grouping_invariant(ACC_INTERIOR));

// ---------------------------------------------------------------------------
// CLAIM B: the n-1 bound does not deliver the specification. The witness:
// four elements of 7. Total 28 escapes the 21-top accumulator under every
// grouping (grouping-independently), so the fold refuses; the specification
// clamps to 7.
// ---------------------------------------------------------------------------

const _: () = assert!(!matches_spec(ACC_INTERIOR));

const _: () = assert!(!fold(ACC_INTERIOR, 0, 7, 7, 7, 7).0);
const _: () = assert!(!fold(ACC_INTERIOR, 1, 7, 7, 7, 7).0);
const _: () = assert!(!fold(ACC_INTERIOR, 2, 7, 7, 7, 7).0);
const _: () = assert!(!fold(ACC_INTERIOR, 3, 7, 7, 7, 7).0);
const _: () = assert!(!fold(ACC_INTERIOR, 4, 7, 7, 7, 7).0);
const _: () = assert!(spec(7, 7, 7, 7).0);
const _: () = assert!(spec(7, 7, 7, 7).1 == 7);

// ---------------------------------------------------------------------------
// CLAIM C: the n bound delivers the specification (and therefore also
// grouping invariance), exhaustively.
// ---------------------------------------------------------------------------

const _: () = assert!(matches_spec(ACC_TOTAL));
const _: () = assert!(grouping_invariant(ACC_TOTAL));
