//! P1. Does I15 ("Never any runtime checks, ever... unused paths we clear out
//! when lowered") force a primitive to be *saturated*, i.e. to carry exactly one
//! runtime degree of freedom (its value) and no other?
//!
//! The claim under test: a construction with a parameter that is NOT resolved
//! before lowering cannot reach one lowered path. Three arms, same arithmetic:
//!
//!   A. `sat_const`   -- width is a const generic. Saturated.
//!   B. `sat_field`   -- width is a runtime field of the value. Unsaturated.
//!   C. `sat_dispatch`-- width is a runtime enum, matched. Unsaturated, "one
//!                       lowered path per case" rather than one path.
//!
//! NEGATIVE CONTROL, stated before the run. Arm A must show the width's derived
//! mask as a *literal* in its own emitted code, and must contain no
//! variable-operand shift. If A shows a variable shift, the whole mechanism I am
//! claiming (const parameters fold, runtime ones do not) is false and this probe
//! establishes nothing; the run is then a refutation of my thesis, not support
//! for it. Arm B must show a variable shift. If B also folds, the two arms are
//! not distinguishable and the probe is void.
//!
//! `no_std`, no alloc, per I14.
#![no_std]
#![allow(dead_code)]

// ---- Arm A: saturated. Width in the type. ----
#[repr(transparent)]
pub struct Sat<const W: u32>(pub u64);

impl<const W: u32> Sat<W> {
    pub const MASK: u64 = if W >= 64 { u64::MAX } else { (1u64 << W) - 1 };
}

#[unsafe(no_mangle)]
pub fn a_sat_13(x: u64) -> u64 {
    x & Sat::<13>::MASK
}

#[unsafe(no_mangle)]
pub fn a_sat_47(x: u64) -> u64 {
    x & Sat::<47>::MASK
}

// ---- Arm B: unsaturated. Width travels with the value. ----
pub struct Unsat {
    pub bits: u64,
    pub w: u32,
}

#[unsafe(no_mangle)]
pub fn b_unsat(v: &Unsat) -> u64 {
    let mask = if v.w >= 64 { u64::MAX } else { (1u64 << v.w) - 1 };
    v.bits & mask
}

// ---- Arm C: unsaturated by dispatch. A closed set, matched at runtime. ----
#[derive(Clone, Copy)]
pub enum Width {
    W13,
    W47,
}

#[unsafe(no_mangle)]
pub fn c_dispatch(x: u64, w: Width) -> u64 {
    match w {
        Width::W13 => x & Sat::<13>::MASK,
        Width::W47 => x & Sat::<47>::MASK,
    }
}

// ---- The composite question: does a SECOND unresolved parameter multiply? ----
// Arm A2 adds a strategy-shaped second index, still const. If saturation is the
// property that matters, A2 must fold exactly as flat as A.
pub struct Wrap;
pub struct Sat2;

pub trait Pol {
    fn apply(x: u64, mask: u64) -> u64;
}
impl Pol for Wrap {
    fn apply(x: u64, mask: u64) -> u64 {
        x & mask
    }
}
impl Pol for Sat2 {
    fn apply(x: u64, mask: u64) -> u64 {
        if x > mask { mask } else { x }
    }
}

#[unsafe(no_mangle)]
pub fn a2_sat_13_wrap(x: u64) -> u64 {
    <Wrap as Pol>::apply(x, Sat::<13>::MASK)
}

#[unsafe(no_mangle)]
pub fn a2_sat_13_satur(x: u64) -> u64 {
    <Sat2 as Pol>::apply(x, Sat::<13>::MASK)
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
