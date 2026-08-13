// Probe A. Is a strategy expressible as a compile-time CHOICE FUNCTION over a
// shared arm set, rather than as a bundle of per-strategy implementations?
//
// The claim under test is structural, not numeric: that the arms can be written
// once, shared by every strategy, and that a strategy contributes only a const
// map from region facts (here: logical width and fold arity) to an arm index.
// If that is expressible with NO unstable features and the selection erases,
// then "a strategy generates code" is the wrong model and "a strategy picks
// among shared code" is available.
//
// Build:
//   rustc --edition 2024 -O --emit asm -C panic=abort -o a_choice_function.s \
//         a_choice_function.rs
//
// no_std on purpose, and the absence of any `#![feature(...)]` line is part of
// the result rather than an omission.

#![no_std]
#![crate_type = "lib"]

// ---------------------------------------------------------------------------
// The shared arm set. Three ways to fold a run of W-bit values with a
// saturating add at the declared limit. Every strategy selects from THIS set;
// no strategy owns an arm.
// ---------------------------------------------------------------------------

/// Arm 0: accumulate in the minimum container that holds one element, clamping
/// at every step. Smallest working set, most clamps executed.
#[inline(never)]
pub fn arm_minimum(vals: &[u32], limit: u32) -> u32 {
    let mut acc: u32 = 0;
    for &v in vals {
        let s = acc.wrapping_add(v);
        acc = if s > limit || s < acc { limit } else { s };
    }
    acc
}

/// Arm 1: accumulate in a container wide enough that the interior clamps are
/// provably dead, clamp once at the end.
#[inline(never)]
pub fn arm_accfit(vals: &[u32], limit: u32) -> u32 {
    let mut acc: u64 = 0;
    for &v in vals {
        acc += v as u64;
    }
    if acc > limit as u64 { limit } else { acc as u32 }
}

/// Arm 2: the same deferred fold, split into four independent partial sums so
/// the backend can keep four accumulators live. Legal only where the fold's
/// operator is associative on the accumulator's domain.
#[inline(never)]
pub fn arm_accfit_lanes(vals: &[u32], limit: u32) -> u32 {
    let mut a: [u64; 4] = [0; 4];
    let chunks = vals.len() / 4;
    for c in 0..chunks {
        a[0] += vals[c * 4] as u64;
        a[1] += vals[c * 4 + 1] as u64;
        a[2] += vals[c * 4 + 2] as u64;
        a[3] += vals[c * 4 + 3] as u64;
    }
    let mut acc = a[0] + a[1] + a[2] + a[3];
    let mut i = chunks * 4;
    while i < vals.len() {
        acc += vals[i] as u64;
        i += 1;
    }
    if acc > limit as u64 { limit } else { acc as u32 }
}

// ---------------------------------------------------------------------------
// The strategy. It contributes an associated const INDEXED BY THE REGION, and
// nothing else. No bodies, no arithmetic impls, no per-strategy copy of
// anything above.
//
// The region is in the trait's own parameters rather than in a const fn's
// arguments. That is the shape `a-refused-bound-wants-a-trait-not-a-feature`
// describes: the derivation lives in an impl, where arbitrary const
// expressions are legal, and the bound names a contract.
// ---------------------------------------------------------------------------

pub trait StrategyAt<const W: u32, const ARITY: u32> {
    const ARM: u32;
}

pub struct Speed;
pub struct Footprint;
pub struct Exact;

impl<const W: u32, const ARITY: u32> StrategyAt<W, ARITY> for Speed {
    // Weighs cycles. Takes the lane-split arm wherever the run is long enough
    // for four accumulators to pay, otherwise the deferred fold.
    const ARM: u32 = if ARITY >= 16 { 2 } else { 1 };
}

impl<const W: u32, const ARITY: u32> StrategyAt<W, ARITY> for Footprint {
    // Weighs bytes. The minimum container, which is also the only arm whose
    // working set is one element wide.
    const ARM: u32 = 0;
}

impl<const W: u32, const ARITY: u32> StrategyAt<W, ARITY> for Exact {
    // Weighs the answer. Never reassociates, so never the lane arm.
    const ARM: u32 = 1;
}

// ---------------------------------------------------------------------------
// The dispatcher. One body, generic over the strategy, with the selection as a
// monomorphisation-time constant.
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn fold<S, const W: u32, const ARITY: u32>(vals: &[u32], limit: u32) -> u32
where
    S: StrategyAt<W, ARITY>,
{
    match <S as StrategyAt<W, ARITY>>::ARM {
        0 => arm_minimum(vals, limit),
        1 => arm_accfit(vals, limit),
        _ => arm_accfit_lanes(vals, limit),
    }
}

// Three monomorphisations at one region, so the emitted code can be read for
// whether the match survived.

#[unsafe(no_mangle)]
pub fn entry_speed(vals: &[u32], limit: u32) -> u32 {
    fold::<Speed, 13, 64>(vals, limit)
}

#[unsafe(no_mangle)]
pub fn entry_footprint(vals: &[u32], limit: u32) -> u32 {
    fold::<Footprint, 13, 64>(vals, limit)
}

#[unsafe(no_mangle)]
pub fn entry_exact(vals: &[u32], limit: u32) -> u32 {
    fold::<Exact, 13, 64>(vals, limit)
}

// And the same strategy at a different region, to show the selection moves
// with the region rather than with the marker alone. This is the thing a
// preset table cannot do without being edited.

#[unsafe(no_mangle)]
pub fn entry_speed_short(vals: &[u32], limit: u32) -> u32 {
    fold::<Speed, 13, 4>(vals, limit)
}
