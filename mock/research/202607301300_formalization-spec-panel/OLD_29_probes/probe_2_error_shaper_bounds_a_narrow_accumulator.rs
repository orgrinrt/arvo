//! Probe 2: a combinator-owned, stateful error-feedback shaper compiles
//! under the same `const trait` discipline the design already uses
//! (`Quantisation::Fallibility<T>` is already a GAT; this needs less), and
//! bounds a narrow accumulator's cumulative error where a plain per-step
//! `Direction` does not, for the identical constant-bias input the
//! consolidation's own DC-ramp finding names
//! (`26_consolidation_two.md:289-292`).
//!
//! Model: quantum Q = 10, five steps of exact input 2 (a value under half a
//! quantum, so plain nearest-rounding sends every step to 0 and drifts by
//! -2 per step). `ErrorShaper::State` is the carried residual: what the
//! shaper wanted to deliver this step but did not, fed forward as an
//! addition to next step's exact value before rounding. `FirstOrderFeedback`
//! is the plainest instance (order 1). Nothing here claims this is the
//! whole design; it is the smallest instance that exercises the shape.
//!
//! What this shows: the trait shape is buildable today, with a plain
//! associated type (no GAT needed for the base case), `Default`-free (an
//! explicit `INIT` const stands in, since `Self::State` here is a bare
//! primitive with a nonzero-cost `Default` question that does not matter to
//! the mechanism), no allocation, no heap, sizes const, and it fits inside a
//! `const fn` fold exactly like the accumulator-sufficiency check the
//! multiplicative half already ships (`26_consolidation_two.md:278-286`).
//! Whether THIS shape is the right one for arvo (associated const vs
//! `Default`, order-N generalisation, how it composes with `Quantisation`)
//! is argued in the deliverable; this probe checks only that the mechanism
//! computes what it claims to.

#![no_std]
#![feature(const_trait_impl)]

const Q: i32 = 10;

const fn round_to_nearest_multiple(v: i32, q: i32) -> i32 {
    let down = v.div_euclid(q) * q;
    let rem = v - down;
    if rem * 2 < q {
        down
    } else {
        down + q
    }
}

/// A feedback kernel threaded through a strictly sequential fold. Not a
/// `Policy`: nothing here is keyed on the composition alone, because the
/// state is a property of the SEQUENCE of quantisation events, not of any
/// one value. See the deliverable, section on where shaping lives.
pub const trait ErrorShaper {
    type State: Copy;
    const INIT: Self::State;

    /// The exact value this step, adjusted by whatever the prior step could
    /// not deliver. Feeds the SAME rounder the undithered path uses; no new
    /// `Direction` vocabulary is needed, only an extra input to it.
    fn shape(state: Self::State, exact: i32) -> i32;

    /// What was owed after this step's rounding: the shaped exact value
    /// minus what actually got delivered, carried forward as next step's
    /// correction.
    fn update(state: Self::State, exact: i32, delivered: i32) -> Self::State;
}

pub struct FirstOrderFeedback;

const impl ErrorShaper for FirstOrderFeedback {
    type State = i32;
    const INIT: i32 = 0;

    fn shape(state: i32, exact: i32) -> i32 {
        exact + state
    }

    fn update(state: i32, exact: i32, delivered: i32) -> i32 {
        (exact + state) - delivered
    }
}

const fn shaped_fold<const N: usize>(inputs: [i32; N], q: i32) -> ([i32; N], i32) {
    let mut state = FirstOrderFeedback::INIT;
    let mut out = [0i32; N];
    let mut i = 0;
    while i < N {
        let shaped_exact = FirstOrderFeedback::shape(state, inputs[i]);
        let delivered = round_to_nearest_multiple(shaped_exact, q);
        state = FirstOrderFeedback::update(state, inputs[i], delivered);
        out[i] = delivered;
        i += 1;
    }
    (out, state)
}

const fn plain_fold<const N: usize>(inputs: [i32; N], q: i32) -> [i32; N] {
    let mut out = [0i32; N];
    let mut i = 0;
    while i < N {
        out[i] = round_to_nearest_multiple(inputs[i], q);
        i += 1;
    }
    out
}

const fn sum<const N: usize>(xs: [i32; N]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < N {
        s += xs[i];
        i += 1;
    }
    s
}

const INPUTS: [i32; 5] = [2, 2, 2, 2, 2];
const EXACT_SUM: i32 = 10; // 5 * 2

// negative control: plain per-step rounding of a constant sub-half-quantum
// bias drifts without bound. every step rounds to zero; the DC ramp the
// consolidation already measured, reproduced here at N = 5.
const PLAIN_DELIVERED: [i32; 5] = plain_fold(INPUTS, Q);
const PLAIN_SUM: i32 = sum(PLAIN_DELIVERED);
const _: () = assert!(PLAIN_SUM == 0);
const PLAIN_TOTAL_ERROR: i32 = PLAIN_SUM - EXACT_SUM;
const _: () = assert!(PLAIN_TOTAL_ERROR == -10); // one full quantum of drift at N = 5

// shaped: the carried residual delivers a 10 on the third step, exactly
// balancing the four zero-steps around it, and the final state returns to
// zero, matching "bounds total error within one quantum forever."
const SHAPED: ([i32; 5], i32) = shaped_fold(INPUTS, Q);
const SHAPED_DELIVERED: [i32; 5] = SHAPED.0;
const SHAPED_FINAL_STATE: i32 = SHAPED.1;

const _: () = assert!(SHAPED_DELIVERED[0] == 0);
const _: () = assert!(SHAPED_DELIVERED[1] == 0);
const _: () = assert!(SHAPED_DELIVERED[2] == 10);
const _: () = assert!(SHAPED_DELIVERED[3] == 0);
const _: () = assert!(SHAPED_DELIVERED[4] == 0);

const SHAPED_SUM: i32 = sum(SHAPED_DELIVERED);
const _: () = assert!(SHAPED_SUM == 10);
const SHAPED_TOTAL_ERROR: i32 = SHAPED_SUM - EXACT_SUM;
const _: () = assert!(SHAPED_TOTAL_ERROR == 0);
const _: () = assert!(SHAPED_FINAL_STATE == 0);

// the finding: identical input, identical rounder, the only difference is
// whether the residual is carried across steps. shaped total error (0) is
// strictly better than plain total error's magnitude (10, one full quantum
// at N = 5, unbounded as N grows) for a workload the interior-safety fix
// does not reach at all: a narrow, per-step-quantised accumulator, which is
// exactly the regime real DSP silicon reaches for noise shaping in rather
// than widening the accumulator.
const _: () = assert!(SHAPED_TOTAL_ERROR.abs() < PLAIN_TOTAL_ERROR.abs());
