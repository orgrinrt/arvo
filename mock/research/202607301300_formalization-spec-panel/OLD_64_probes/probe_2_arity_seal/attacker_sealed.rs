//! The same forgery `attacker_unsealed.rs` ran, against the sealed library.
//!
//! EXPECTED: REFUSES. Two independent routes to the same forged goal, both
//! closed, matching the shape Knuth's file 62 confirmed for `Rad<P>`'s
//! `AtLeastTwo` (four then five routes, "every attack found lands in one of
//! them"): the direct route (implement `Arity` for a local type) refuses on
//! the private supertrait; the bypass route (skip `Arity` and implement
//! `InteriorSafety` directly for the forged type) refuses on the trait's own
//! declared bound. Both are exercised below; comment one out to see the
//! other's error in isolation, or compile as-is to see both errors at once.

extern crate arity_lib_sealed as tower;
use tower::{Arity, Big, InteriorSafety, Safe};

pub struct MyOwnArity;

// Route 1: implement the closed vocabulary trait directly for a local type.
// Refuses: `sealed::Sealed` is not reachable outside the defining crate, so
// there is no way to satisfy `Arity`'s supertrait bound from here at all.
impl Arity for MyOwnArity {}

// Route 2: skip `Arity` and go straight for `InteriorSafety`, the same shape
// that compiled clean in `attacker_unsealed.rs`. Refuses now because the
// trait declaration itself requires `A: Arity`, and `MyOwnArity` cannot
// satisfy that requirement (Route 1's failure is exactly why).
impl InteriorSafety<MyOwnArity> for Big {
    type Out = Safe;
}
