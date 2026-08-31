// a3: the container premise, re-derived on a third construction, plus one thing the corpus's
// rows about it do not say.
//
// The question is `question::the_container_premise`: is a declared numeral's footprint
// observable, so that behaviour is stated over the container rather than over the declared width?
//
// This probe does two jobs.
//
// JOB ONE, a third instance of the measured half. `proposal::the_carrier_is_observable_through_
// the_ambient_layout_observation_alone` measures a 13-bit unsigned numeral over `u16` and `u32`
// and reports that the layout observation separates 2 bytes from 4 while nothing the design
// declares separates anything. This rebuilds that on a construction of its own: different
// operations, a different projection, and its own controls. Agreement is worth having because the
// row stands at one expert; disagreement would be worth more.
//
// JOB TWO, and this is the part the rows do not carry. `core::mem::size_of` is a CONST function.
// So the footprint is not merely observable, it is observable AT CONST TIME, which under
// `ruling::the_work_is_predicated_arms_composed` and `ruling::arms_over_regions_are_the_
// fundamental_heart` makes it a legitimate predicate for an arm to be gated on, and under
// `ruling::never_a_runtime_check_and_one_lowered_path` it costs nothing at runtime to read.
// `ruling::the_predicate_is_whatever_is_available_at_const_time` is the sentence that makes this
// the deciding property rather than a curiosity. The probe compiles a const arm keyed on it, so
// the claim is a compile result rather than an argument.
//
// PREDICTIONS, stated before running:
//
// P1. `size_of` and `align_of` separate the two carriers, 2 against 4.
// P2. No operation stated over the declared width separates them, exhaustively over all 8192
//     values and over all 8192 x 8192 ordered pairs for the binary ones.
// P3. A const arm keyed on `size_of` compiles and selects, with no runtime branch, which makes
//     the footprint a gateable axis rather than only an observable one.
//
// THE CASES THAT MUST FAIL, each printed failing rather than asserted:
//   C1 two distinct newtypes over the SAME carrier must not be separated by layout. If they were,
//      the observation would be reading type identity rather than the carrier, and P1 would be
//      about the wrong thing.
//   C2 the arithmetic comparison must be able to report a difference. An operation whose
//      intermediate lives in the CARRIER rather than at the declared width is introduced for
//      exactly this, because a zero from a comparison that cannot produce a nonzero establishes
//      nothing.
//   C3 a zero-sized marker must report size 0, showing the instrument reads real layout and not a
//      constant somebody wrote down.
//
// TWO DEAD ROUTES ON C2, kept and still run rather than repaired away, because between them they
// say exactly when a carrier is arithmetically observable and that is worth more than the working
// mutant alone.
//
//   Dead route one: an ADDITION wrapped at the carrier width. Reported 0 of 67108864. Two 13-bit
//   operands sum to at most 16382, below 2^16, so no carrier in this pair can truncate an
//   addition and the mutant was not a mutant at these widths.
//
//   Dead route two: a MULTIPLY wrapped at the carrier width and then masked to the declared
//   width. Also reported 0 of 67108864, and this one is the more instructive failure. The
//   declared-width mask is a mask of the LOW bits and the carrier mask is a wider mask of the same
//   low bits, so the narrower mask absorbs the wider one and the carrier's truncation is
//   unobservable no matter how large the intermediate gets. Wrapping at the declared width is
//   exactly the projection that cannot see a wider wrap above it.
//
//   What works, and the rule the two dead routes give: the carrier is arithmetically observable
//   only where the intermediate can leave the narrower carrier AND the final projection is not a
//   low-bit mask. Saturation is the smallest such projection, because it reads the intermediate's
//   magnitude rather than its low bits, so a wrap the carrier performed underneath changes which
//   side of the clamp the value lands on.
//
// Deliberately uses `core::mem` rather than `std::mem` throughout the numeric part, because the
// question is whether the observation is available to a `#![no_std]` crate, and `std` would beg it.
//
// Build: rustc --edition 2024 -O.

use core::mem::{align_of, size_of};

const W: u32 = 13;
const MASK: u32 = (1u32 << W) - 1;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct N13U16(u16);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct N13U32(u32);

/// A second newtype over the SAME carrier as `N13U16`, for control C1.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AlsoU16(u16);

/// A zero-sized marker, for control C3.
struct Marker;

trait Numeral: Copy {
    /// The carrier width, named only so the mutants in C2 can reach it.
    const CARRIER_BITS: u32;
    fn from_raw(v: u32) -> Self;
    fn to_raw(self) -> u32;
}

impl Numeral for N13U16 {
    const CARRIER_BITS: u32 = 16;
    fn from_raw(v: u32) -> Self {
        N13U16((v & MASK) as u16)
    }
    fn to_raw(self) -> u32 {
        self.0 as u32 & MASK
    }
}

impl Numeral for N13U32 {
    const CARRIER_BITS: u32 = 32;
    fn from_raw(v: u32) -> Self {
        N13U32(v & MASK)
    }
    fn to_raw(self) -> u32 {
        self.0 & MASK
    }
}

fn carrier_mask<N: Numeral>() -> u32 {
    if N::CARRIER_BITS >= 32 { u32::MAX } else { (1u32 << N::CARRIER_BITS) - 1 }
}

/// Operations stated over the DECLARED width. None of these may see the carrier.
fn wrap_add<N: Numeral>(a: N, b: N) -> u32 {
    (a.to_raw() + b.to_raw()) & MASK
}
fn sat_add<N: Numeral>(a: N, b: N) -> u32 {
    let s = a.to_raw() + b.to_raw();
    if s > MASK { MASK } else { s }
}
fn wrap_mul<N: Numeral>(a: N, b: N) -> u32 {
    a.to_raw().wrapping_mul(b.to_raw()) & MASK
}
/// The declared-width saturating multiply, whose intermediate is the exact product.
fn sat_mul<N: Numeral>(a: N, b: N) -> u32 {
    let p = (a.to_raw() as u64) * (b.to_raw() as u64);
    if p > MASK as u64 { MASK } else { p as u32 }
}
fn xor<N: Numeral>(a: N, b: N) -> u32 {
    a.to_raw() ^ b.to_raw()
}
fn roundtrip<N: Numeral>(v: u32) -> u32 {
    N::from_raw(v).to_raw()
}

/// C2's working mutant: a SATURATING multiply whose intermediate is held in the carrier. The
/// carrier's wrap happens beneath a projection that reads magnitude, so it changes which side of
/// the clamp the result lands on.
fn sat_mul_via_carrier<N: Numeral>(a: N, b: N) -> u32 {
    let intermediate = a.to_raw().wrapping_mul(b.to_raw()) & carrier_mask::<N>();
    if intermediate > MASK { MASK } else { intermediate }
}

/// Dead route one, kept and still run.
fn add_via_carrier<N: Numeral>(a: N, b: N) -> u32 {
    (a.to_raw() + b.to_raw()) & carrier_mask::<N>()
}

/// Dead route two, kept and still run.
fn wrap_mul_via_carrier<N: Numeral>(a: N, b: N) -> u32 {
    (a.to_raw().wrapping_mul(b.to_raw()) & carrier_mask::<N>()) & MASK
}

/// Job two: a const arm keyed on the footprint. If this compiles, the footprint is a const
/// predicate and an arm may be gated on it.
const fn arm_for<const BYTES: usize>() -> &'static str {
    if BYTES <= 2 { "narrow arm" } else { "wide arm" }
}

const ARM_U16: &str = arm_for::<{ size_of::<N13U16>() }>();
const ARM_U32: &str = arm_for::<{ size_of::<N13U32>() }>();

fn main() {
    println!("=== a3: the layout observation, on a third construction, and its const status ===");
    println!();

    println!("--- P1: does the layout observation separate the two carriers? ---");
    println!(
        "  N13U16: size {} align {}   N13U32: size {} align {}",
        size_of::<N13U16>(),
        align_of::<N13U16>(),
        size_of::<N13U32>(),
        align_of::<N13U32>()
    );
    println!(
        "  separated by size: {} (predicted true)",
        size_of::<N13U16>() != size_of::<N13U32>()
    );
    println!();

    println!("--- C1: does it separate two newtypes over the SAME carrier? ---");
    println!(
        "  N13U16: size {}   AlsoU16: size {}   separated: {}",
        size_of::<N13U16>(),
        size_of::<AlsoU16>(),
        size_of::<N13U16>() != size_of::<AlsoU16>()
    );
    println!("     (must be false, or the observation reads type identity rather than the carrier,");
    println!("      and P1 would be measuring the wrong thing)");
    println!();

    println!("--- C3: does it read real layout? ---");
    println!(
        "  zero-sized marker: size {} (must be 0), u8: size {} (must be 1)",
        size_of::<Marker>(),
        size_of::<u8>()
    );
    println!();

    println!("--- P2: does any declared-width operation separate the two carriers? ---");
    let n = 1u32 << W;
    let mut unary_diff = 0u64;
    for v in 0..n {
        if roundtrip::<N13U16>(v) != roundtrip::<N13U32>(v) {
            unary_diff += 1;
        }
    }
    println!("  roundtrip over all {n} values: {unary_diff} differences (predicted 0)");

    let mut add_diff = 0u64;
    let mut sat_diff = 0u64;
    let mut mul_diff = 0u64;
    let mut satmul_diff = 0u64;
    let mut xor_diff = 0u64;
    let mut mutant_satmul = 0u64;
    let mut dead_add = 0u64;
    let mut dead_wrapmul = 0u64;
    let mut pairs = 0u64;
    for a in 0..n {
        let a16 = N13U16::from_raw(a);
        let a32 = N13U32::from_raw(a);
        for b in 0..n {
            let b16 = N13U16::from_raw(b);
            let b32 = N13U32::from_raw(b);
            if wrap_add(a16, b16) != wrap_add(a32, b32) {
                add_diff += 1;
            }
            if sat_add(a16, b16) != sat_add(a32, b32) {
                sat_diff += 1;
            }
            if wrap_mul(a16, b16) != wrap_mul(a32, b32) {
                mul_diff += 1;
            }
            if sat_mul(a16, b16) != sat_mul(a32, b32) {
                satmul_diff += 1;
            }
            if xor(a16, b16) != xor(a32, b32) {
                xor_diff += 1;
            }
            if sat_mul_via_carrier(a16, b16) != sat_mul_via_carrier(a32, b32) {
                mutant_satmul += 1;
            }
            if add_via_carrier(a16, b16) != add_via_carrier(a32, b32) {
                dead_add += 1;
            }
            if wrap_mul_via_carrier(a16, b16) != wrap_mul_via_carrier(a32, b32) {
                dead_wrapmul += 1;
            }
            pairs += 1;
        }
    }
    println!(
        "  over all {pairs} ordered pairs: wrap-add {add_diff}, sat-add {sat_diff}, wrap-mul \
         {mul_diff}, sat-mul {satmul_diff}, xor {xor_diff} (all predicted 0)"
    );
    println!();

    println!("--- C2: the mutant, so the zeros above mean something ---");
    println!(
        "  saturating multiply with a carrier-held intermediate differs on {mutant_satmul} of \
         {pairs} pairs"
    );
    println!(
        "     nonzero: {} (must be true; if this were 0 the five zeros above would establish",
        mutant_satmul > 0
    );
    println!("      nothing, because the comparison could not report a difference at all)");
    println!("  dead route one, addition at the carrier width:      {dead_add} of {pairs}");
    println!("  dead route two, wrap-multiply at the carrier width: {dead_wrapmul} of {pairs}");
    println!("     Both zero, and both for a stated reason rather than by luck: the sum cannot");
    println!("     leave the narrower carrier, and a low-bit mask at the declared width absorbs a");
    println!("     wider low-bit mask above it. Together they say when a carrier is");
    println!("     arithmetically observable: the intermediate must be able to leave the narrower");
    println!("     carrier, and the projection must read magnitude rather than low bits.");
    println!();

    println!("--- P3: is the footprint available as a const predicate? ---");
    println!("  const ARM_U16 = {ARM_U16:?}, const ARM_U32 = {ARM_U32:?}");
    println!("  Both are `const` items whose value came from `core::mem::size_of`, so the arm was");
    println!("  selected at compile time and no runtime branch exists. That is the part the");
    println!("  container-premise rows do not state: the footprint is not only observable, it is");
    println!("  observable in exactly the place a predicated arm is allowed to read.");
    println!();
    println!("  `core::mem` rather than `std::mem` throughout, so the observation is available to");
    println!("  a no_std crate and the answer does not depend on the standard library.");
    println!();
    println!("=== end a3 ===");
}
