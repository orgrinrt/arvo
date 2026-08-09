//! Probe 2: interior safety is exactly the condition that upgrades the weak
//! equation to the Kleene equation, and the value half holds either way.
//!
//! The three equations of a partial algebra (Burmeister's vocabulary, standard
//! since the 1980s and not invented here):
//!
//!   weak       `t1 =w t2`  : if both sides are defined, they are equal.
//!   existence  `t1 =e t2`  : both sides are defined, and they are equal.
//!   Kleene     `t1 ~= t2`  : both defined and equal, or both undefined.
//!
//! `Precise` addition (out-of-range resolution `Refuse`) is the case the
//! consolidation reports as one fused failed verdict
//! (26_consolidation_two.md:213-221: diameter 0 at every fold length, against
//! 10992 grouping-dependent refusals out of 32768 inputs at a five-element
//! fold). This probe reproduces that at the same model size and then shows the
//! two facts are separable and that one of them is a function of the accumulator
//! alone.
//!
//! CLAIM A. The weak equation holds at every accumulator width: whenever two
//! groupings both return, they return the same number. Checked over all 14
//! parenthesisations of a five-element fold, over all 8^5 = 32768 inputs, at
//! four accumulator ranges.
//!
//! CLAIM B. Below the threshold, definedness is grouping-dependent, so the
//! Kleene equation fails while the weak equation holds. Two accumulators exhibit
//! it, and the wider of the two exhibits it less often, so the effect is a
//! threshold being approached rather than a step at an arbitrary place.
//!
//! CLAIM C. At or above the threshold, definedness is grouping-invariant, so the
//! Kleene equation holds outright. The sufficient condition is stated in the
//! numeral's own value coordinates and is a closed form in the arity alone:
//! the accumulator contains every sum of at most `n-1` operands drawn from the
//! destination numeral, `(n-1)*[min V(N), max V(N)] subset V(M)`.
//!
//! CLAIM D. That closed form is sufficient and NOT necessary, and the gap is
//! measured rather than asserted: an accumulator strictly narrower than the
//! closed form still shows no definedness split, because the destination
//! numeral's own range prunes the inputs that could have produced one. An
//! earlier draft of this probe claimed `n` rather than `n-1` and predicted a
//! split at an accumulator that shows none; the const assertion refused, which
//! is how the arithmetic got corrected.
//!
//! CLAIM E. The residual refusals at sufficient width are grouping-independent
//! and are exactly "the exact sum does not fit the destination numeral", checked
//! against an independent count.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2_interior_safety_upgrades_weak_to_kleene.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).
//! Roughly 90 seconds of const evaluation.

#![allow(dead_code)]
// The bounded exhaustive check below is 32768 inputs x 14 groupings x 4
// accumulator ranges, which trips `long_running_const_eval` exactly as
// 26_consolidation_two.md:72-74 records for this class of check. The lint's own
// note says an allow is safe where the compilation genuinely takes a long time,
// and this one terminates by construction: every loop bound is a literal.
#![allow(long_running_const_eval)]

// The destination numeral N: eight values, AsymmetricLow, quantum 1.
const N_LO: i32 = -4;
const N_HI: i32 = 3;
const NVALS: i32 = 8;
const ARITY: i32 = 5;

/// The sufficient interior-safety condition, in the numeral's own value
/// coordinates. Every proper subtree of a fold over ARITY operands holds a sum
/// of at most ARITY-1 of them, so an accumulator containing that span leaves
/// every proper subtree exact and defined; the root then receives the same exact
/// total under every grouping, and both its value and its definedness are
/// grouping-independent.
const NEED_LO: i32 = (ARITY - 1) * N_LO; // -16
const NEED_HI: i32 = (ARITY - 1) * N_HI; //  12

const fn interior_safe(acc_lo: i32, acc_hi: i32) -> bool {
    acc_lo <= NEED_LO && acc_hi >= NEED_HI
}

// Four accumulators. Ranges, not bit counts: the condition is a statement about
// which values exist, which is a mathematical-coordinate fact, and the width
// formula in bits is its radix-two encoding.
const ACC: [(i32, i32); 4] = [
    (-8, 7),   // clearly below
    (-9, 8),   // one step below the observed boundary
    (-10, 9),  // the observed boundary for this N and this arity
    (-16, 12), // the closed-form sufficient bound
];

const _: () = assert!(!interior_safe(ACC[0].0, ACC[0].1));
const _: () = assert!(!interior_safe(ACC[1].0, ACC[1].1));
const _: () = assert!(!interior_safe(ACC[2].0, ACC[2].1));
const _: () = assert!(interior_safe(ACC[3].0, ACC[3].1));

// ---------------------------------------------------------------------------
// A partial value: (number, defined). `Refuse` is the whole fallibility story
// here, which is `Precise`'s own out-of-range resolution.
// ---------------------------------------------------------------------------

type PV = (i32, bool);

const fn leaf(v: i32) -> PV {
    (v, true)
}

/// Interior addition, in the accumulator numeral. Exact when it lands, refusing
/// when it does not. No rounding: two multiples of the quantum sum to a multiple
/// of the quantum, so the round stage of the quantiser is the identity here,
/// which is itself the reason the value half never diverges (claim A).
const fn addi(x: PV, y: PV, lo: i32, hi: i32) -> PV {
    if !x.1 || !y.1 {
        return (0, false);
    }
    let s = x.0 + y.0;
    if s < lo || s > hi {
        (0, false)
    } else {
        (s, true)
    }
}

/// The single store at the root: quantise the accumulated value into the
/// destination numeral N.
const fn store(x: PV) -> PV {
    if !x.1 {
        return (0, false);
    }
    if x.0 < N_LO || x.0 > N_HI {
        (0, false)
    } else {
        (x.0, true)
    }
}

/// Evaluate one of the fourteen parenthesisations of a five-element fold.
/// Element order is identical in every tree: only the grouping changes, which is
/// the point (26_consolidation_two.md:705-708, order is the wrong axis).
const fn shape(tree: u8, a: i32, b: i32, c: i32, d: i32, e: i32, lo: i32, hi: i32) -> PV {
    let (a, b, c, d, e) = (leaf(a), leaf(b), leaf(c), leaf(d), leaf(e));
    let r = match tree {
        0 => addi(
            addi(addi(addi(a, b, lo, hi), c, lo, hi), d, lo, hi),
            e,
            lo,
            hi,
        ),
        1 => addi(
            addi(addi(a, addi(b, c, lo, hi), lo, hi), d, lo, hi),
            e,
            lo,
            hi,
        ),
        2 => addi(
            addi(addi(a, b, lo, hi), addi(c, d, lo, hi), lo, hi),
            e,
            lo,
            hi,
        ),
        3 => addi(
            addi(a, addi(addi(b, c, lo, hi), d, lo, hi), lo, hi),
            e,
            lo,
            hi,
        ),
        4 => addi(
            addi(a, addi(b, addi(c, d, lo, hi), lo, hi), lo, hi),
            e,
            lo,
            hi,
        ),
        5 => addi(
            addi(addi(a, b, lo, hi), c, lo, hi),
            addi(d, e, lo, hi),
            lo,
            hi,
        ),
        6 => addi(
            addi(a, addi(b, c, lo, hi), lo, hi),
            addi(d, e, lo, hi),
            lo,
            hi,
        ),
        7 => addi(
            addi(a, b, lo, hi),
            addi(addi(c, d, lo, hi), e, lo, hi),
            lo,
            hi,
        ),
        8 => addi(
            addi(a, b, lo, hi),
            addi(c, addi(d, e, lo, hi), lo, hi),
            lo,
            hi,
        ),
        9 => addi(
            a,
            addi(addi(addi(b, c, lo, hi), d, lo, hi), e, lo, hi),
            lo,
            hi,
        ),
        10 => addi(
            a,
            addi(addi(b, addi(c, d, lo, hi), lo, hi), e, lo, hi),
            lo,
            hi,
        ),
        11 => addi(
            a,
            addi(addi(b, c, lo, hi), addi(d, e, lo, hi), lo, hi),
            lo,
            hi,
        ),
        12 => addi(
            a,
            addi(b, addi(addi(c, d, lo, hi), e, lo, hi), lo, hi),
            lo,
            hi,
        ),
        _ => addi(
            a,
            addi(b, addi(c, addi(d, e, lo, hi), lo, hi), lo, hi),
            lo,
            hi,
        ),
    };
    store(r)
}

const TREES: u8 = 14;

/// Returns (value_disagreements, definedness_splits, inputs_defined_under_all).
const fn survey(lo: i32, hi: i32) -> (u32, u32, u32) {
    let mut value_bad: u32 = 0;
    let mut defined_bad: u32 = 0;
    let mut all_defined: u32 = 0;

    let mut ia = 0;
    while ia < NVALS {
        let mut ib = 0;
        while ib < NVALS {
            let mut ic = 0;
            while ic < NVALS {
                let mut id = 0;
                while id < NVALS {
                    let mut ie = 0;
                    while ie < NVALS {
                        let (a, b, c, d, e) =
                            (N_LO + ia, N_LO + ib, N_LO + ic, N_LO + id, N_LO + ie);

                        let mut any_def = false;
                        let mut any_undef = false;
                        let mut first_val = 0i32;
                        let mut have_first = false;
                        let mut mismatch = false;

                        let mut t = 0u8;
                        while t < TREES {
                            let r = shape(t, a, b, c, d, e, lo, hi);
                            if r.1 {
                                any_def = true;
                                if have_first {
                                    if r.0 != first_val {
                                        mismatch = true;
                                    }
                                } else {
                                    first_val = r.0;
                                    have_first = true;
                                }
                            } else {
                                any_undef = true;
                            }
                            t += 1;
                        }

                        if mismatch {
                            value_bad += 1;
                        }
                        if any_def && any_undef {
                            defined_bad += 1;
                        }
                        if any_def && !any_undef {
                            all_defined += 1;
                        }

                        ie += 1;
                    }
                    id += 1;
                }
                ic += 1;
            }
            ib += 1;
        }
        ia += 1;
    }
    (value_bad, defined_bad, all_defined)
}

const S0: (u32, u32, u32) = survey(ACC[0].0, ACC[0].1);
const S1: (u32, u32, u32) = survey(ACC[1].0, ACC[1].1);
const S2: (u32, u32, u32) = survey(ACC[2].0, ACC[2].1);
const S3: (u32, u32, u32) = survey(ACC[3].0, ACC[3].1);

// CLAIM A: the weak equation holds at every width, including deep below the
// threshold. Zero value disagreements anywhere.
const _: () = assert!(S0.0 == 0);
const _: () = assert!(S1.0 == 0);
const _: () = assert!(S2.0 == 0);
const _: () = assert!(S3.0 == 0);

// CLAIM B: below the boundary, Kleene fails while weak holds, and the count
// shrinks as the accumulator widens.
const _: () = assert!(S0.1 > 0);
const _: () = assert!(S1.1 > 0);
const _: () = assert!(S1.1 < S0.1);

// CLAIM C: at the closed-form sufficient bound, definedness is
// grouping-invariant.
const _: () = assert!(S3.1 == 0);

// CLAIM D: sufficiency is not necessity. An accumulator strictly narrower than
// the closed form already shows no split, because the destination numeral's own
// range prunes the inputs that could produce one. The design should still state
// the closed form, because it is a bound in the arity alone and needs no
// reasoning about which inputs can occur, but it should say that it is
// conservative rather than implying it is tight.
const _: () = assert!(S2.1 == 0);
const _: () = assert!(ACC[2].0 > NEED_LO && ACC[2].1 < NEED_HI);

// CLAIM E: the residual refusals at sufficient width are exactly the inputs
// whose exact sum does not fit the destination numeral, computed independently
// of the fold machinery.
const fn exact_fits_count() -> u32 {
    let mut n: u32 = 0;
    let mut ia = 0;
    while ia < NVALS {
        let mut ib = 0;
        while ib < NVALS {
            let mut ic = 0;
            while ic < NVALS {
                let mut id = 0;
                while id < NVALS {
                    let mut ie = 0;
                    while ie < NVALS {
                        let s = (N_LO + ia) + (N_LO + ib) + (N_LO + ic) + (N_LO + id) + (N_LO + ie);
                        if s >= N_LO && s <= N_HI {
                            n += 1;
                        }
                        ie += 1;
                    }
                    id += 1;
                }
                ic += 1;
            }
            ib += 1;
        }
        ia += 1;
    }
    n
}

const _: () = assert!(S3.2 == exact_fits_count());
const _: () = assert!(S2.2 == exact_fits_count());

// The negative control for claim E: the count is neither zero nor everything, so
// the equality above is a real agreement between two independent computations
// rather than two ways of writing 32768.
const _: () = assert!(exact_fits_count() > 0);
const _: () = assert!(exact_fits_count() < 32768);

/// Present so the same file can be compiled as a binary to print the counts the
/// assertions above only bound. Compiling as `--crate-type lib` leaves it unused.
fn main() {
    let rows = [S0, S1, S2, S3];
    println!("acc_range     value_disagreements  definedness_splits  defined_under_all");
    let mut i = 0;
    while i < 4 {
        println!(
            "[{:>3},{:>3}]     {:>6}               {:>6}              {:>6}",
            ACC[i].0, ACC[i].1, rows[i].0, rows[i].1, rows[i].2
        );
        i += 1;
    }
    println!("closed-form sufficient bound: [{}, {}]", NEED_LO, NEED_HI);
    println!("exact_fits_count = {}", exact_fits_count());
}
