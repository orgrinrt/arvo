// Probe B. Is "strategy" one thing, or several things wearing one name?
//
// Probes A, C and D separate three questions that the single marker currently
// answers all at once:
//
//   COST      which of the shared arms to select in this region. Probe A.
//   POLICY    what the operation does at the boundary: wrap, clamp, round.
//   LICENCE   which algebraic rewrites may be invoked. Probe C shows these are
//             several separate permissions with separate regions, not one bit.
//
// This probe asks whether they can be carried as three independent components
// with the named markers recovered as POINTS in the product, rather than as a
// closed set of four bundles. Three things are checked:
//
//   1. it is expressible with no unstable features;
//   2. a licence really gates emitted code, so it is not decoration;
//   3. the monomorphisation cost is driven by the points a consumer actually
//      instantiates, not by the size of the product, which is what decides
//      whether opening the axes is affordable.
//
// Build:
//   rustc --edition 2024 -O --emit asm -C panic=abort -o b_unbundled.s b_unbundled.rs
//   rustc --edition 2024 -O --emit obj -C panic=abort -o b_unbundled.o  b_unbundled.rs

#![no_std]
#![crate_type = "lib"]

// ---------------------------------------------------------------------------
// Axis 1: POLICY. What an operation does at the declared boundary.
// ---------------------------------------------------------------------------

pub trait Policy {
    /// 0 = wrap, 1 = saturate. A tag rather than a method, so it is readable
    /// from a const position without needing a const-callable trait method.
    const TAG: u32;
}

pub struct Wrap;
pub struct Saturate;
impl Policy for Wrap {
    const TAG: u32 = 0;
}
impl Policy for Saturate {
    const TAG: u32 = 1;
}

// ---------------------------------------------------------------------------
// Axis 2: LICENCE. Which rewrites may be invoked. Probe C established that
// these are separate permissions over separate regions, so this is a vector of
// bits rather than a single "may I be clever" flag.
//
// REASSOCIATE is the lane-splitting licence. RETRACT_EARLY is the licence to
// apply the boundary policy at each step rather than once at the end. They are
// independent: probe C's part 1 and part 3 disagree on `wrap`/`sub`, which
// retracts and does not associate.
// ---------------------------------------------------------------------------

pub trait Licence {
    const REASSOCIATE: bool;
    const RETRACT_EARLY: bool;
}

pub struct NoRewrites;
pub struct LawfulRewrites;
pub struct AnyRewrite;

impl Licence for NoRewrites {
    const REASSOCIATE: bool = false;
    const RETRACT_EARLY: bool = false;
}
impl Licence for LawfulRewrites {
    // Granted only where probe C found the law holds. The predicate that gates
    // this at a call site is the consumer's; what the axis carries is the
    // permission.
    const REASSOCIATE: bool = true;
    const RETRACT_EARLY: bool = true;
}
impl Licence for AnyRewrite {
    // The speed-first point: rewrites are taken whether or not the law holds,
    // which is the shape a strategy that may trade soundness for a measured
    // gain would use.
    const REASSOCIATE: bool = true;
    const RETRACT_EARLY: bool = true;
}

// ---------------------------------------------------------------------------
// Axis 3: COST. Which measurement the selection weighs, as an ordering over
// the shared arms, indexed by the region. Probe A's shape.
// ---------------------------------------------------------------------------

pub trait Cost<const W: u32, const ARITY: u32> {
    /// 0 = smallest working set, 1 = widest accumulator, 2 = fewest cycles.
    const PREFER: u32;
}

pub struct Cycles;
pub struct Bytes;
pub struct Answer;

impl<const W: u32, const ARITY: u32> Cost<W, ARITY> for Cycles {
    const PREFER: u32 = 2;
}
impl<const W: u32, const ARITY: u32> Cost<W, ARITY> for Bytes {
    const PREFER: u32 = 0;
}
impl<const W: u32, const ARITY: u32> Cost<W, ARITY> for Answer {
    const PREFER: u32 = 1;
}

// ---------------------------------------------------------------------------
// A strategy is a POINT in the product. The named markers are impls; nothing
// stops a consumer naming another point, and nothing about the design has to
// change when one does.
// ---------------------------------------------------------------------------

pub trait Strategy {
    type P: Policy;
    type L: Licence;
    type C;
}

pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

impl Strategy for Hot {
    type P = Wrap;
    type L = AnyRewrite;
    type C = Cycles;
}
impl Strategy for Warm {
    type P = Wrap; // what a native Rust primitive does in release
    type L = LawfulRewrites;
    type C = Cycles;
}
impl Strategy for Cold {
    type P = Saturate;
    type L = LawfulRewrites;
    type C = Bytes;
}
impl Strategy for Precise {
    type P = Saturate;
    type L = NoRewrites;
    type C = Answer;
}

/// A point nobody named in advance: the storage-minimising carrier with the
/// accuracy-first licence. Under a closed set of four this has no spelling.
pub struct ColdExact;
impl Strategy for ColdExact {
    type P = Saturate;
    type L = NoRewrites;
    type C = Bytes;
}

// ---------------------------------------------------------------------------
// The arms. Shared, as in probe A.
// ---------------------------------------------------------------------------

#[inline(never)]
pub fn arm_seq_wrap(v: &[u32]) -> u32 {
    let mut a: u32 = 0;
    for &x in v {
        a = a.wrapping_add(x);
    }
    a
}

#[inline(never)]
pub fn arm_lanes_wrap(v: &[u32]) -> u32 {
    let mut a: [u32; 4] = [0; 4];
    let c = v.len() / 4;
    for i in 0..c {
        a[0] = a[0].wrapping_add(v[i * 4]);
        a[1] = a[1].wrapping_add(v[i * 4 + 1]);
        a[2] = a[2].wrapping_add(v[i * 4 + 2]);
        a[3] = a[3].wrapping_add(v[i * 4 + 3]);
    }
    let mut acc = a[0]
        .wrapping_add(a[1])
        .wrapping_add(a[2])
        .wrapping_add(a[3]);
    let mut i = c * 4;
    while i < v.len() {
        acc = acc.wrapping_add(v[i]);
        i += 1;
    }
    acc
}

#[inline(never)]
pub fn arm_seq_sat(v: &[u32], limit: u32) -> u32 {
    let mut a: u32 = 0;
    for &x in v {
        a = a.saturating_add(x).min(limit);
    }
    a
}

#[inline(never)]
pub fn arm_wide_sat(v: &[u32], limit: u32) -> u32 {
    let mut a: u64 = 0;
    for &x in v {
        a += x as u64;
    }
    if a > limit as u64 {
        limit
    } else {
        a as u32
    }
}

// ---------------------------------------------------------------------------
// The dispatcher reads all three axes. Every branch below is over a
// monomorphisation-time constant.
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn fold<S, const W: u32, const ARITY: u32>(v: &[u32], limit: u32) -> u32
where
    S: Strategy,
    S::C: Cost<W, ARITY>,
{
    let policy = <S::P as Policy>::TAG;
    let may_reassoc = <S::L as Licence>::REASSOCIATE;
    let prefer = <S::C as Cost<W, ARITY>>::PREFER;

    if policy == 0 {
        // wrapping: reassociation is licensed by probe C part 3 for add
        if may_reassoc && prefer == 2 {
            arm_lanes_wrap(v)
        } else {
            arm_seq_wrap(v)
        }
    } else {
        // saturating: the wide accumulator is the deferred form
        if prefer == 0 {
            arm_seq_sat(v, limit)
        } else {
            arm_wide_sat(v, limit)
        }
    }
}

#[unsafe(no_mangle)]
pub fn p_hot(v: &[u32], l: u32) -> u32 {
    fold::<Hot, 13, 64>(v, l)
}
#[unsafe(no_mangle)]
pub fn p_warm(v: &[u32], l: u32) -> u32 {
    fold::<Warm, 13, 64>(v, l)
}
#[unsafe(no_mangle)]
pub fn p_cold(v: &[u32], l: u32) -> u32 {
    fold::<Cold, 13, 64>(v, l)
}
#[unsafe(no_mangle)]
pub fn p_precise(v: &[u32], l: u32) -> u32 {
    fold::<Precise, 13, 64>(v, l)
}

/// The unnamed point. Its presence in the emitted object is what says the
/// product is open rather than the set closed.
#[unsafe(no_mangle)]
pub fn p_cold_exact(v: &[u32], l: u32) -> u32 {
    fold::<ColdExact, 13, 64>(v, l)
}
