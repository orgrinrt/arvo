//! Probe 5: what a law over division claims, worked through the finest-view lattice.
//!
//! Division has no associativity to lose (it is not associative in the rationals), so
//! the interesting laws are structural. The strongest candidate is the round-trip
//! against the exact product: div(mul_full(a, b), b) = a. This probe computes that
//! law's verdict at the three grade components file 37's lattice is built from,
//! exhaustively at a model width, and finds its finest view is the weak-equation
//! corner: values agree wherever both sides are defined, definedness does NOT agree
//! (the left side refuses at b = 0, the right side is just `a`), and quantisation
//! events do NOT agree (the left side's divider fires its quantiser once, the right
//! side never computes). Nothing division-specific had to be added to the lattice
//! vocabulary to state this, which is the compiled form of the dispatch's question
//! about whether the finest-view mechanism handles a quotient: it does, unchanged.
//!
//! One reading is assumed and marked: an event is counted per quantiser APPLICATION,
//! not per value actually moved. That is the type-level reading the design already
//! commits to (`40:279-287`: the operation marker's IS_EXACT, a type-level constant,
//! is what trivialises the grade monoid, so grade content cannot depend on which
//! values a run happened to see), and it is the over-approximating direction the
//! design takes everywhere on lattice containment (`40:308-312`). Under the
//! per-value-moved reading the event component of this law would hold vacuously on
//! this model (the quotient is always on-grid), and the law would sit one lattice
//! point higher; the value and definedness components are unaffected either way.
//!
//! Model: unsigned p=4, F=2 (values k/4, k in 0..=15). mul_full lands exactly on the
//! 1/16 grid (product numeral, `40:359-369`), no event, total. div is
//! quantize(exact quotient) onto the 1/4 grid, RNE, one event, refusing at divisor
//! zero. All arithmetic exact and integer.
//!
//! CLAIM A: value agreement wherever both defined, all 16*15 pairs (the exact
//!   quotient of the exact product by b is a's value, on-grid, so the quantiser is
//!   the identity there; this is why the law holds at all).
//! CLAIM B: definedness disagrees at exactly the 16 pairs with b = 0.
//! CLAIM C: the event counts are 1 against 0 on every defined pair, so any
//!   event-preserving view refuses the law.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_5_the_roundtrip_law_and_its_view.rs --out-dir <dir>
//! Outcome: WORKS (all three claims assert exhaustively).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

/// A term's meaning on this model: defined?, value in 1/16 units (exact for both
/// sides here), quantisation events, refusal causes.
#[derive(Clone, Copy)]
struct Graded {
    defined: bool,
    val16: i128,
    events: u32,
    causes: u32,
}

const fn rne(num: i128, den: i128, g: i128) -> i128 {
    let t = num * g;
    let q0 = t / den;
    let r = t % den;
    if 2 * r > den || (2 * r == den && q0 % 2 == 1) {
        q0 + 1
    } else {
        q0
    }
}

/// mul_full: exact into the product numeral (1/16 grid). No event, no cause.
const fn mul_full(k1: i128, k2: i128) -> Graded {
    Graded {
        defined: true,
        val16: k1 * k2,
        events: 0,
        causes: 0,
    }
}

/// div: quantize(exact quotient) onto the 1/4 grid. One event; a cause at b = 0.
/// Input value in 1/16 units; divisor index k2 (value k2/4).
const fn div_q(a: Graded, k2: i128) -> Graded {
    if !a.defined || k2 == 0 {
        return Graded {
            defined: false,
            val16: 0,
            events: a.events,
            causes: a.causes + if k2 == 0 { 1 } else { 0 },
        };
    }
    // exact quotient = (a.val16/16) / (k2/4) = a.val16 / (4*k2); RNE onto 1/4 grid.
    let n = rne(a.val16, 4 * k2, 4);
    Graded {
        defined: true,
        val16: n * 4,
        events: a.events + 1,
        causes: a.causes,
    }
}

const fn law_verdicts() -> (bool, i128, bool) {
    // (weak-equation holds, definedness disagreements, events ever agree)
    let mut weak = true;
    let mut defin_disagreements: i128 = 0;
    let mut events_ever_agree = false;
    let mut k1: i128 = 0;
    while k1 <= 15 {
        let mut k2: i128 = 0;
        while k2 <= 15 {
            let lhs = div_q(mul_full(k1, k2), k2);
            let rhs = Graded {
                defined: true,
                val16: k1 * 4,
                events: 0,
                causes: 0,
            };
            if lhs.defined != rhs.defined {
                defin_disagreements += 1;
            }
            if lhs.defined && rhs.defined {
                if lhs.val16 != rhs.val16 {
                    weak = false;
                }
                if lhs.events == rhs.events {
                    events_ever_agree = true;
                }
            }
            k2 += 1;
        }
        k1 += 1;
    }
    (weak, defin_disagreements, events_ever_agree)
}

const VERDICTS: (bool, i128, bool) = law_verdicts();

// CLAIM A: values agree wherever both sides are defined.
const _CLAIM_A: () = assert!(VERDICTS.0);
// CLAIM B: definedness disagrees at exactly the 16 zero-divisor pairs.
const _CLAIM_B: () = assert!(VERDICTS.1 == 16);
// CLAIM C: the event components never agree on a defined pair (1 vs 0), so every
// event-preserving view refuses the law; its finest view collapses events AND
// definedness, the weak-equation corner of the lattice.
const _CLAIM_C: () = assert!(!VERDICTS.2);
