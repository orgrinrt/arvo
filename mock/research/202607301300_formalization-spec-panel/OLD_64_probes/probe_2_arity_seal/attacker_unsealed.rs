//! A downstream crate, exactly as an algorithm-crate author (or a careless
//! reviewer of one) would be: it depends on the tower's `InteriorSafety`
//! mechanism and wants to describe its own loop's arity.
//!
//! EXPECTED: COMPILES CLEAN, which is the finding. The orphan rule permits
//! this because `MyOwnArity` (the trait's generic PARAMETER, not `Self`) is
//! local to this crate; that is the standard "uncovered local type parameter"
//! carve-out, the same rule that lets any crate impl a foreign trait for
//! `Vec<MyType>`. The first attempt at this file tried to write the impl
//! generic over `Hd: Pos` and hit `E0210` (`Hd`, uncovered, appears before the
//! local `MyOwnArity`); the orphan rule requires `Self` to be concrete (or
//! covered) once a foreign trait is in play. That does NOT close the hole: it
//! only means the attack has to name a concrete tower type as `Self` rather
//! than being generic over all of them, which is exactly what an algorithm
//! crate author writing a loop over one specific numeral would do anyway.
//!
//! The forged impl below does not merely fail to be caught: it asserts the
//! OPPOSITE of what the design's `Unbounded` marker exists to guarantee, for
//! a real, concrete tower type. A consumer who writes `MyOwnArity` where the
//! design intends `Unbounded` (by accident, by copying an older shape, or by
//! not knowing the marker exists) gets `Safe` for a loop whose trip count is
//! a function of the data, and the type system raises no objection anywhere.

extern crate arity_lib_unsealed as tower;
use tower::{Big, InteriorSafety, Safe};

/// A consumer-defined arity marker. Nothing about its name, its shape, or its
/// crate of origin marks it as standing for "the trip count is unbounded";
/// it could equally be a typo for the design's own `Unbounded`, or a good-
/// faith reinvention by someone who never read this review.
pub struct MyOwnArity;

/// The forgery: claim SAFE for a concrete tower type against this arity, with
/// no obligation to justify it against anything the tower's `Cmp` machinery
/// establishes. This is not routed through `Cmp` at all; it is a bare,
/// independent assertion, for a type (`Big`) that genuinely exists in the
/// tower and that real algorithm code would genuinely be generic over or
/// concrete on.
impl InteriorSafety<MyOwnArity> for Big {
    type Out = Safe;
}

/// This is exactly `55_probes/probe_2b`'s `iterate_until_converged` shape,
/// specialised to `Big`. Swap `Unbounded` for `MyOwnArity` at the call site
/// and the design's own top-of-lattice pessimism (the whole point of file
/// 55's section 2, restated by the consolidation's fixpoint story) never
/// fires, for a real tower type, with no diagnostic anywhere in the chain.
pub fn claims_safe_for_an_unbounded_loop_on_big()
where
    Big: InteriorSafety<MyOwnArity, Out = Safe>,
{
}

pub fn demonstrate() {
    claims_safe_for_an_unbounded_loop_on_big();
}
