//! Probe 3: which reading is a type, which is a value, and what the value one costs in
//! emitted code.
//!
//! WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates the two readings on BINDING TIME,
//! which is the axis the fourth design rule reads (`78:152-166`), and on emitted
//! instructions, which is the axis the pricing pillar says is the measurement. The
//! distinction is nonvacuous here because the loop is over data the compiler cannot see:
//! a loop over a compile-time-known input would let LLVM constant-fold reading B's test
//! away and the two would look identical.
//!
//! It does NOT separate: correctness (both readings compute what they claim), the cause
//! component, or anything about the float door (probe 4).
//!
//! CLAIM A: reading A's count is a function of the operation's type alone and is asserted
//!   in const position, with no value in scope. Reading B's is not: the same const, written
//!   over the same types, cannot be stated, because the count is a function of the data.
//!   The compile-fail companion `probe_3b_reading_b_is_not_a_const.rs` is that half.
//! CLAIM B: over a 64-element fixed-point column, reading A adds ZERO instructions to the
//!   loop (its count is a literal the caller reads from the type), and reading B adds a
//!   test and an accumulate per element. Counted from the disassembly, not asserted.
//! CLAIM C: reading B's per-element test is cheap on the fixed-point side specifically
//!   because the quantiser has already computed the discarded bits. The three loops are
//!   emitted side by side so the difference is one instruction group rather than a claim.
//!
//! Build:
//!   rustc --edition 2021 -O --crate-type lib probe_3_binding_time_and_price.rs --out-dir out
//!   objdump -d out/libprobe_3_binding_time_and_price.rlib
//! Outcome: WORKS. Counts in OUTCOMES.md.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

#![no_std]

/// A quantising operation, described at the type level exactly as the design describes one:
/// whether a quantiser sits in its pipeline, and whether it is total.
pub trait Op {
    /// `43:145` / `40:279-287`: exactness kills quantiser-generated events.
    const IS_EXACT: bool;
    /// `40:279-287`: totality kills causes with no quantiser origin.
    const IS_TOTAL: bool;
    /// The number of quantiser SITES one application of this operation contains.
    const SITES: u32;
}

/// A rounding fixed-point addition: one quantiser site, total.
pub struct RoundingAdd;
impl Op for RoundingAdd {
    const IS_EXACT: bool = false;
    const IS_TOTAL: bool = true;
    const SITES: u32 = 1;
}

/// An exact widening multiply: no quantiser site, total (`43:145-148`, `78:226-230`).
pub struct MulFull;
impl Op for MulFull {
    const IS_EXACT: bool = true;
    const IS_TOTAL: bool = true;
    const SITES: u32 = 0;
}

/// General division: one quantiser site, partial (`84:236-240`, kind 2).
pub struct Div;
impl Op for Div {
    const IS_EXACT: bool = false;
    const IS_TOTAL: bool = false;
    const SITES: u32 = 1;
}

/// Reading A's event count for a fold of `N` applications of `O`. A function of types only.
pub const fn events_reading_a<O: Op>(n: u32) -> u32 {
    O::SITES * n
}

// CLAIM A, positive half: reading A's count is available with no value anywhere in scope,
// in const position, at every operation in the design's own vocabulary.
const _A_ADD_64: () = assert!(events_reading_a::<RoundingAdd>(64) == 64);
const _A_MUL_64: () = assert!(events_reading_a::<MulFull>(64) == 0);
const _A_DIV_64: () = assert!(events_reading_a::<Div>(64) == 64);
// And the round-trip term of probe 2: mul_full then div, one application each.
const _A_ROUNDTRIP: () = assert!(events_reading_a::<MulFull>(1) + events_reading_a::<Div>(1) == 1);
// Against its right-hand side, a leaf, which applies nothing.
const _A_ROUNDTRIP_RHS: () = assert!(0u32 == 0);

// -------------------------------------------------------------------------------------
// CLAIM B and C: the three loops. Fixed-point, FRAC = 4, a rounding add that discards the
// low four bits of the exact sum. `#[no_mangle]` so the disassembly is readable.

const FRAC: u32 = 4;

/// One rounding add: exact sum, then round the low FRAC bits away, ties toward negative
/// (the `Hot` fixed-point row, `78:411`, an arithmetic right shift).
#[inline(always)]
fn radd(acc: i64, x: i64) -> i64 {
    (acc + x) >> FRAC << FRAC
}

/// No grade published. The baseline.
#[no_mangle]
pub extern "C" fn fold_plain(xs: &[i64; 64]) -> i64 {
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < 64 {
        acc = radd(acc, xs[i]);
        i += 1;
    }
    acc
}

/// Reading A's grade published alongside the value. The count is `SITES * 64`, a literal
/// the caller can read from the type; nothing enters the loop.
#[no_mangle]
pub extern "C" fn fold_grade_reading_a(xs: &[i64; 64]) -> (i64, u32) {
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < 64 {
        acc = radd(acc, xs[i]);
        i += 1;
    }
    (acc, events_reading_a::<RoundingAdd>(64))
}

/// Reading B's grade published alongside the value. The count is the number of adds whose
/// rounding actually moved the value, which is a test on the bits the shift discarded.
#[no_mangle]
pub extern "C" fn fold_grade_reading_b(xs: &[i64; 64]) -> (i64, u32) {
    let mut acc = 0i64;
    let mut ev = 0u32;
    let mut i = 0usize;
    while i < 64 {
        let exact = acc + xs[i];
        let discarded = exact & ((1i64 << FRAC) - 1);
        ev += (discarded != 0) as u32;
        acc = exact >> FRAC << FRAC;
        i += 1;
    }
    (acc, ev)
}

/// Reading B at the presence level only: did anything round at all. The design's published
/// grade sits at presence (`50:294-307`, the five-bit word), so this is the shape that
/// would actually ship, and it is cheaper than the counting one.
#[no_mangle]
pub extern "C" fn fold_grade_reading_b_presence(xs: &[i64; 64]) -> (i64, bool) {
    let mut acc = 0i64;
    let mut any = 0i64;
    let mut i = 0usize;
    while i < 64 {
        let exact = acc + xs[i];
        any |= exact & ((1i64 << FRAC) - 1);
        acc = exact >> FRAC << FRAC;
        i += 1;
    }
    (acc, any != 0)
}
