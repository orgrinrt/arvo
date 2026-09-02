//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Does the widened shape survive const evaluation, and does it fold away?
//!
//! An ad-hoc quick spike. Nothing here is timed and it decides no fork. It asks
//! three yes-or-no questions the widened `has_additive_identity` depends on:
//!
//! 1. Can a `const fn` carry a bounded `while` loop and a `match` on `Option`,
//!    on the pinned toolchain, with no feature gate?
//! 2. Does it evaluate in a `const` binding, which is what makes it a
//!    compile-time predicate rather than a runtime one?
//! 3. Does it fold to a constant at `-O`, so
//!    `ruling::never_a_runtime_check_and_one_lowered_path` is satisfied at the
//!    lowering rather than only in the const position?
//!
//! Question 3 is the one that needs the emitted assembly rather than an
//! assertion, so the spike exposes a `#[no_mangle]` caller to read.
//!
//! Run: `rustc --edition 2021 -O --emit asm const_shape.rs -o const_shape.s`
//!      `rustc --edition 2021 -O const_shape.rs -o cs && ./cs`

#![allow(dead_code)]

trait Quantum {
    const BASE: i32;
    const SLOPE: i32;
    const MAGNITUDES: u32;
    const ADMITTED: () = {
        assert!(
            Self::MAGNITUDES >= 1,
            "a law over no magnitudes describes no values"
        );
    };
}

trait Slots {
    const MIN: i64;
    const MAX: i64;
}

trait Format {
    type Quantum: Quantum;
    type Slots: Slots;
    const RADIX: u32;
    const PHASE_NUM: i64;
    const PHASE_DEN: i64;
}

const fn slot_in_range<S: Slots>(slot: i64) -> bool {
    slot >= S::MIN && slot <= S::MAX
}

/// How far the magnitude search can run before no larger magnitude can succeed.
///
/// The cancelling slot at magnitude `m` solves `i * PD * r^(SLOPE*m) = -PN`.
/// With `k = SLOPE*m` and a radix of at least two:
///
/// - `k > 0` needs `PD * r^k` to divide `PN`, so `r^k <= |PN| < 2^63` and
///   `k <= 62`.
/// - `k < 0` gives `|i| * PD = |PN| * r^(-k)`, and both `|i|` and `PD` are
///   bounded by `2^63`, so `r^(-k) <= 2^126` and `-k <= 126`.
///
/// `|SLOPE| >= 1` whenever `SLOPE != 0`, so `|k| >= m` and 126 bounds `m` too.
/// At `SLOPE == 0` every magnitude gives the same equation, so one is enough.
/// At a radix below two the powers never grow, which is the same case.
const MAGNITUDE_SEARCH_BOUND: u32 = 127;

/// The slot that cancels the phase at one magnitude, if there is one.
///
/// `None` means no integer slot cancels the phase there, or that deciding it
/// would leave the range the arithmetic is carried in, which is the same answer
/// for the purpose: a slot that far out is not in any admitted range.
const fn cancelling_slot<F: Format>(magnitude: u32) -> Option<i64> {
    if F::PHASE_DEN == 0 {
        return None;
    }
    if F::PHASE_NUM == 0 {
        return Some(0);
    }
    let radix = F::RADIX as i128;
    let k = (<F::Quantum as Quantum>::SLOPE as i64) * (magnitude as i64);
    let pn = -(F::PHASE_NUM as i128);
    let pd = F::PHASE_DEN as i128;

    // i * PD * r^k = -PN, solved for i in whichever direction keeps it whole.
    let (num, den) = if k >= 0 {
        let mut d = pd;
        let mut n = 0;
        while n < k {
            match d.checked_mul(radix) {
                Some(v) => d = v,
                None => return None,
            }
            n += 1;
        }
        (pn, d)
    } else {
        let mut n2 = pn;
        let mut n = 0;
        while n < -k {
            match n2.checked_mul(radix) {
                Some(v) => n2 = v,
                None => return None,
            }
            n += 1;
        }
        (n2, pd)
    };

    if den == 0 || num % den != 0 {
        return None;
    }
    let slot = num / den;
    if slot < i64::MIN as i128 || slot > i64::MAX as i128 {
        return None;
    }
    Some(slot as i64)
}

const fn has_additive_identity<F: Format>() -> bool {
    let () = <F::Quantum as Quantum>::ADMITTED;
    if F::PHASE_DEN == 0 {
        return false;
    }
    let magnitudes = <F::Quantum as Quantum>::MAGNITUDES;
    let bound = if magnitudes < MAGNITUDE_SEARCH_BOUND {
        magnitudes
    } else {
        MAGNITUDE_SEARCH_BOUND
    };
    let mut m = 0;
    while m < bound {
        if let Some(slot) = cancelling_slot::<F>(m) {
            if slot_in_range::<F::Slots>(slot) {
                return true;
            }
        }
        m += 1;
    }
    false
}

// --- instances ---------------------------------------------------------------

struct Constant<const EXP: i32>;
impl<const EXP: i32> Quantum for Constant<EXP> {
    const BASE: i32 = EXP;
    const SLOPE: i32 = 0;
    const MAGNITUDES: u32 = 1;
}

struct Indexed<const MIN_EXP: i32, const COUNT: u32>;
impl<const MIN_EXP: i32, const COUNT: u32> Quantum for Indexed<MIN_EXP, COUNT> {
    const BASE: i32 = MIN_EXP;
    const SLOPE: i32 = 1;
    const MAGNITUDES: u32 = COUNT;
}

struct Shrinking<const COUNT: u32>;
impl<const COUNT: u32> Quantum for Shrinking<COUNT> {
    const BASE: i32 = 0;
    const SLOPE: i32 = -1;
    const MAGNITUDES: u32 = COUNT;
}

struct Signed<const BITS: u32>;
impl<const BITS: u32> Slots for Signed<BITS> {
    const MIN: i64 = -(1i64 << (BITS - 1));
    const MAX: i64 = (1i64 << (BITS - 1)) - 1;
}

struct Zero;
impl Format for Zero {
    type Quantum = Constant<0>;
    type Slots = Signed<4>;
    const RADIX: u32 = 2;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

struct HalfStep;
impl Format for HalfStep {
    type Quantum = Constant<0>;
    type Slots = Signed<4>;
    const RADIX: u32 = 2;
    const PHASE_NUM: i64 = 1;
    const PHASE_DEN: i64 = 2;
}

/// The case reachable with shipped coordinates: growing quantum, whole phase,
/// cancelling slot out of range at magnitude zero and in range at magnitude one.
struct WholeOutOfReach;
impl Format for WholeOutOfReach {
    type Quantum = Indexed<0, 2>;
    type Slots = Signed<2>;
    const RADIX: u32 = 2;
    const PHASE_NUM: i64 = 4;
    const PHASE_DEN: i64 = 1;
}

/// The case needing an outside quantum: shrinking quantum, fractional phase,
/// which becomes whole at a higher magnitude.
struct FractionalBecomesWhole;
impl Format for FractionalBecomesWhole {
    type Quantum = Shrinking<2>;
    type Slots = Signed<4>;
    const RADIX: u32 = 2;
    const PHASE_NUM: i64 = 1;
    const PHASE_DEN: i64 = 2;
}

// Question 2: does it evaluate in a const binding? If any of these is not const
// this file does not compile, which is the assertion.
const ZERO: bool = has_additive_identity::<Zero>();
const HALF: bool = has_additive_identity::<HalfStep>();
const OUT_OF_REACH: bool = has_additive_identity::<WholeOutOfReach>();
const BECOMES_WHOLE: bool = has_additive_identity::<FractionalBecomesWhole>();

// Question 3: read the emitted assembly for these. Each should be a `mov` of an
// immediate and a return, with no loop and no call.
#[no_mangle]
pub extern "C" fn folds_zero() -> bool {
    has_additive_identity::<Zero>()
}

#[no_mangle]
pub extern "C" fn folds_half() -> bool {
    has_additive_identity::<HalfStep>()
}

#[no_mangle]
pub extern "C" fn folds_out_of_reach() -> bool {
    has_additive_identity::<WholeOutOfReach>()
}

#[no_mangle]
pub extern "C" fn folds_becomes_whole() -> bool {
    has_additive_identity::<FractionalBecomesWhole>()
}

fn main() {
    println!("Zero (phase 0, constant)                    {ZERO}");
    println!("HalfStep (phase 1/2, constant)              {HALF}");
    println!("WholeOutOfReach (phase 4, Indexed+Signed<2>) {OUT_OF_REACH}");
    println!("FractionalBecomesWhole (phase 1/2, slope -1) {BECOMES_WHOLE}");

    assert!(ZERO, "a zero phase lost the identity");
    assert!(!HALF, "a half-step constant grid gained an identity");
    assert!(
        OUT_OF_REACH,
        "the whole phase out of reach at magnitude zero was not found at magnitude one"
    );
    assert!(
        BECOMES_WHOLE,
        "the fractional phase that becomes whole at magnitude one was not found"
    );

    // The control for the search itself: with only one magnitude admitted, both
    // of the two cases above go back to false. If they did not, the search is
    // not what found them and the witnesses above prove nothing.
    struct OneMagnitudeGrowing;
    impl Format for OneMagnitudeGrowing {
        type Quantum = Indexed<0, 1>;
        type Slots = Signed<2>;
        const RADIX: u32 = 2;
        const PHASE_NUM: i64 = 4;
        const PHASE_DEN: i64 = 1;
    }
    struct OneMagnitudeShrinking;
    impl Format for OneMagnitudeShrinking {
        type Quantum = Shrinking<1>;
        type Slots = Signed<4>;
        const RADIX: u32 = 2;
        const PHASE_NUM: i64 = 1;
        const PHASE_DEN: i64 = 2;
    }
    assert!(
        !has_additive_identity::<OneMagnitudeGrowing>(),
        "cutting the magnitude range to one did not take the identity back"
    );
    assert!(
        !has_additive_identity::<OneMagnitudeShrinking>(),
        "cutting the magnitude range to one did not take the identity back"
    );
    println!("control: cutting the magnitude range to one takes both back to false");
}
