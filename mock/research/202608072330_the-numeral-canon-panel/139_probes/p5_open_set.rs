// p5: can the strategy set be OPEN, so a consumer supplies a strategy the
// library never heard of, without breaking "everything reaches one lowered
// path"?
//
// The canon has to be able to say whether a thing is doable. This is a
// feasibility check on the openness claim, under the constraints that are in
// force: no dyn, no TypeId, no alloc, monomorphisation is the dispatch, and no
// runtime validation. If a consumer-defined strategy either fails to compile
// or survives as a runtime branch, "the set is open" is not a thing the canon
// may say.
//
// The library half declares a strategy as the two-component object: a policy
// (answer-visible) and a weighting (cost-visible). The consumer half lives in a
// separate module and defines its own point. Nothing in the library names it.
//
// PREDICTIONS, recorded before the first run:
//   T1 the consumer-defined strategy compiles with no change to the library.
//   T2 the emitted code for a monomorphised call contains ONE arm, so the
//      weighting's choice is resolved before runtime.
//   T3 the runtime-selected control contains BOTH arms, proving the check in
//      T2 can tell the difference.
//
// CONTROLS:
//   C1 each arm carries a distinct magic constant. The asm scan looks for the
//      magic, so an arm that survives cannot hide.
//   C2 the runtime-selected function MUST show both magics. A scan that
//      reports one arm everywhere is not measuring erasure, it is measuring
//      nothing, and this is the case that must fail.
//   C3 the two arms must AGREE on every input over an exhaustive sweep. If
//      they disagree, the weighting is changing the answer and the firewall is
//      already broken, whatever the asm says.

#![allow(dead_code)]

// ============================================================ library half

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Ov {
    Wrap,
    Sat,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Rd {
    Trunc,
    Floor,
}

/// The answer-visible component.
pub trait Policy {
    const OVERFLOW: Ov;
    const ROUNDING: Rd;
    /// Raw units of slack the policy declares around the pinned answer.
    const SLACK: i32;
}

/// The cost-visible component. Weights are const integers so the comparison
/// is a const expression; nothing here is read at runtime.
pub trait Weighting {
    const W_TIME: u32;
    const W_CODE: u32;
    const W_DATA: u32;
}

/// A strategy is the pair, and nothing else.
pub trait Strategy {
    type P: Policy;
    type W: Weighting;
}

pub const MAGIC_FAST: i32 = 0x5BD1_E995u32 as i32;
pub const MAGIC_SMALL: i32 = 0x27D4_EB2Du32 as i32;

// Two arms that compute the SAME function by different routes. The magic
// constant is xored in and back out, so it is observable in the emitted code
// and cannot change the result.
#[inline(never)]
fn arm_fast(a: i32, b: i32, shift: u32, sat: bool) -> i32 {
    let t = (a as i64) * (b as i64);
    let t = t >> shift;
    let t = ((t as i32) ^ MAGIC_FAST) ^ MAGIC_FAST;
    if sat {
        t.clamp(-128, 127)
    } else {
        ((t as u32) & 0xFF) as i32 - if (t as u32) & 0x80 != 0 { 256 } else { 0 }
    }
}

#[inline(never)]
fn arm_small(a: i32, b: i32, shift: u32, sat: bool) -> i32 {
    // long multiplication, one bit at a time: the arm a size-weighted
    // selection would pick.
    let mut acc: i64 = 0;
    let mut m = b as i64;
    let mut n = a as i64;
    let neg = n < 0;
    if neg {
        n = -n;
    }
    while n != 0 {
        if n & 1 != 0 {
            acc += m;
        }
        m <<= 1;
        n >>= 1;
    }
    if neg {
        acc = -acc;
    }
    let t = acc >> shift;
    let t = ((t as i32) ^ MAGIC_SMALL) ^ MAGIC_SMALL;
    if sat {
        t.clamp(-128, 127)
    } else {
        ((t as u32) & 0xFF) as i32 - if (t as u32) & 0x80 != 0 { 256 } else { 0 }
    }
}

/// The whole point: the arm is selected by a const comparison on the
/// weighting, and the semantics come from the policy. No value is consulted.
#[inline(never)]
pub fn mul<S: Strategy>(a: i32, b: i32) -> i32 {
    const SHIFT: u32 = 3;
    let sat = matches!(<S::P as Policy>::OVERFLOW, Ov::Sat);
    if <S::W as Weighting>::W_TIME > <S::W as Weighting>::W_DATA {
        arm_fast(a, b, SHIFT, sat)
    } else {
        arm_small(a, b, SHIFT, sat)
    }
}

/// C2's control: the same selection made at runtime. Both arms must survive.
#[inline(never)]
pub fn mul_runtime_selected(a: i32, b: i32, prefer_time: bool, sat: bool) -> i32 {
    const SHIFT: u32 = 3;
    if prefer_time {
        arm_fast(a, b, SHIFT, sat)
    } else {
        arm_small(a, b, SHIFT, sat)
    }
}

// The library ships two presets, and that is ALL it ships.
pub struct SatTrunc;
impl Policy for SatTrunc {
    const OVERFLOW: Ov = Ov::Sat;
    const ROUNDING: Rd = Rd::Trunc;
    const SLACK: i32 = 0;
}
pub struct TimeFirst;
impl Weighting for TimeFirst {
    const W_TIME: u32 = 100;
    const W_CODE: u32 = 1;
    const W_DATA: u32 = 1;
}
pub struct LibraryPreset;
impl Strategy for LibraryPreset {
    type P = SatTrunc;
    type W = TimeFirst;
}

// ============================================================ consumer half
// A separate module standing in for a separate crate. It defines a point the
// library never named: the library's policy, but a weighting that is not the
// library's, and a slack the library never declared.

mod consumer {
    use super::{Ov, Policy, Rd, Strategy, Weighting};

    pub struct MyPolicy;
    impl Policy for MyPolicy {
        const OVERFLOW: Ov = Ov::Sat;
        const ROUNDING: Rd = Rd::Floor;
        // the consumer declares one raw unit of slack, which the library has
        // no preset for
        const SLACK: i32 = 1;
    }

    pub struct SpaceFirst;
    impl Weighting for SpaceFirst {
        const W_TIME: u32 = 1;
        const W_CODE: u32 = 3;
        const W_DATA: u32 = 90;
    }

    pub struct MyStrategy;
    impl Strategy for MyStrategy {
        type P = MyPolicy;
        type W = SpaceFirst;
    }
}

// Monomorphised entry points, one per strategy, so the asm scan has a symbol
// to look at.
#[inline(never)]
#[no_mangle]
pub extern "C" fn call_library_preset(a: i32, b: i32) -> i32 {
    mul::<LibraryPreset>(a, b)
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn call_consumer_strategy(a: i32, b: i32) -> i32 {
    mul::<consumer::MyStrategy>(a, b)
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn call_runtime_selected(a: i32, b: i32, p: bool) -> i32 {
    mul_runtime_selected(a, b, p, true)
}

fn main() {
    // T1 is established by this file compiling at all.
    println!("T1 the consumer-defined strategy compiles against an unchanged library: PASS");

    // C3: the two arms must agree on every input, or the weighting is moving
    // the answer and nothing else here matters.
    let mut disagreements = 0u64;
    let mut nonzero = 0u64;
    let mut n = 0u64;
    for a in -128i32..=127 {
        for b in -128i32..=127 {
            for sat in [false, true] {
                n += 1;
                let f = arm_fast(a, b, 3, sat);
                let s = arm_small(a, b, 3, sat);
                if f != s {
                    disagreements += 1;
                }
                if f != 0 {
                    nonzero += 1;
                }
            }
        }
    }
    println!(
        "C3 the two arms agree on {} of {} inputs, disagreements={}, nonzero results={}",
        n - disagreements,
        n,
        disagreements,
        nonzero
    );
    if disagreements == 0 && nonzero > 0 {
        println!("C3 PASS: the arms are two routes to one answer, and the check is non-vacuous");
    } else {
        println!("C3 FAIL");
        std::process::exit(1);
    }

    // Values printed so nothing is dead-code eliminated before the asm scan.
    println!(
        "sample outputs: library={} consumer={} runtime={}",
        call_library_preset(100, 100),
        call_consumer_strategy(100, 100),
        call_runtime_selected(100, 100, true)
    );
    println!();
    println!("T2 and T3 are answered by the asm scan in p5_scan.sh, not by this program.");
}
