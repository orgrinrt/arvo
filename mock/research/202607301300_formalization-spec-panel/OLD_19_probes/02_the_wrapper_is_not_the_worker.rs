//! PROBE 2: the fix in probe 1 closes the grant-side hole. It does not, by
//! itself, close a second hole in the SAME mechanism: nothing forces a
//! fidelity-gated combinator to be reachable only through the checked door.
//!
//! `17_probes/06_two_lattices_opposite_variance.rs` splits the mechanism in
//! two: `sum4::<L>` performs the liberty-gated branch and reads `L::LIBERTIES`
//! directly; `sum4_in_context::<A, L>` carries the `ViewC` bound and calls
//! `sum4::<L>`. The entitlement check sits on the WRAPPER. This probe checks
//! whether the WORKER is reachable without it, and it is: `sum4::<Relaxed>`
//! is public, generic only over `L: CGrade`, and nothing stops calling it
//! directly, with no operand-grade check performed anywhere.
//!
//! This is Thread C's original disease, shape-for-shape:
//! `10_leroy...md` section 0 named it "two semantics for one construct,"
//! found because the checked classification and the executed arithmetic
//! were two independently-authored things that happened never to touch.
//! Here the two independently-reachable things are the checked WRAPPER and
//! the unchecked WORKER, and reachability, not authorship, is the seam.
//!
//! The fix probe 1 argues for (fold the door's assertion into the function
//! that actually performs the branch, not a caller-side gate) closes this
//! too, and this file shows both shapes side by side rather than arguing
//! that it does.
//!
//! Build:
//!   rustc -O 02_the_wrapper_is_not_the_worker.rs -o p2 && ./p2

#![allow(dead_code)]

pub const REASSOC: u8 = 0b0001;
pub trait CGrade {
    const LIBERTIES: u8;
    const NAME: &'static str;
}
pub struct Strict;
pub struct Relaxed;
impl CGrade for Strict {
    const LIBERTIES: u8 = 0;
    const NAME: &'static str = "Strict";
}
impl CGrade for Relaxed {
    const LIBERTIES: u8 = REASSOC;
    const NAME: &'static str = "Relaxed";
}
pub const fn liberties_subset(g: u8, holder: u8) -> bool {
    (g & !holder) == 0
}

// ============================================= wrapper-gates, worker-trusts
// This is the file 17 probe's own shape, restated with the same public
// surface it shipped: a marker `ViewC` (kept here only for the wrapper's
// bound; see the note below about what it does and does not guarantee),
// a WORKER generic only over the branch grade, and a WRAPPER that adds the
// entitlement bound and forwards.

pub trait ViewC<G: CGrade>: CGrade {}
impl ViewC<Strict> for Strict {}
impl ViewC<Relaxed> for Relaxed {}
impl ViewC<Strict> for Relaxed {}
// (Deliberately no `impl ViewC<Relaxed> for Strict`, matching the design.)

/// The worker. Reads `L::LIBERTIES` to decide which body to run. Carries no
/// bound at all on what grade any caller's data actually is, because by the
/// time a function takes plain `f64`s there is no type-level record of
/// that left to check against; the type-level record was on the CALLER's
/// side, one level up, and the worker was never told to require it.
pub fn sum4<L: CGrade>(xs: [f64; 4]) -> f64 {
    if L::LIBERTIES & REASSOC != 0 {
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

/// The wrapper. This is where the file 17 probe put the entitlement bound.
pub fn sum4_in_context<A: CGrade + ViewC<L>, L: CGrade>(xs: [f64; 4]) -> f64 {
    sum4::<L>(xs)
}

// ============================================================ single door
// The fix: fold the check into the function that actually branches, keyed
// on the SAME parameter that decides the branch, so there is exactly one
// function anywhere that reads `LIBERTIES` for dispatch, and it is
// unconditionally the checked one. No separate wrapper exists to bypass,
// because there is nothing left for a wrapper to add.
pub fn sum4_doored<A: CGrade, L: CGrade>(xs: [f64; 4]) -> f64 {
    const {
        assert!(
            liberties_subset(L::LIBERTIES, A::LIBERTIES),
            "a value under this licence may not be viewed at a grade granting a liberty it does not hold"
        );
    }
    if L::LIBERTIES & REASSOC != 0 {
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

fn main() {
    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];

    println!("through the wrapper, which does carry the ViewC bound:");
    println!(
        "  sum4_in_context::<Strict, Strict>()   = {:?}",
        sum4_in_context::<Strict, Strict>(xs)
    );
    // sum4_in_context::<Strict, Relaxed>(xs) does not compile: no ViewC<Relaxed> for Strict.

    println!();
    println!("straight to the worker, bypassing the wrapper entirely:");
    println!(
        "  sum4::<Relaxed>(xs)  = {:?}   <-- no entitlement check ran, none was in scope to run",
        sum4::<Relaxed>(xs)
    );
    println!(
        "  sum4::<Strict>(xs)   = {:?}   <-- also fine, also unchecked; the worker cannot tell",
        sum4::<Strict>(xs)
    );

    println!();
    println!("the doored worker: same call shape as sum4, the check travels with it");
    println!(
        "  sum4_doored::<Strict, Strict>(xs)  = {:?}",
        sum4_doored::<Strict, Strict>(xs)
    );
    println!(
        "  sum4_doored::<Relaxed, Relaxed>(xs) = {:?}",
        sum4_doored::<Relaxed, Relaxed>(xs)
    );
    // sum4_doored::<Strict, Relaxed>(xs) would panic at const-eval, the
    // E0080 shape confirmed separately in probe 1's door_refuse_test. There
    // is no second, unchecked path to this same computation to call instead.

    println!();
    println!("reading: `sum4` is directly, unconditionally callable with any grade, checked or");
    println!("not, because the check lived on a caller-side wrapper the worker does not require");
    println!("anyone to go through. `sum4_doored` has no such second path, because there is");
    println!("nothing split across two functions for a caller to reassemble around the check.");
}
