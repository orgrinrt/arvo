//! crate `algebra`: minimal stand-in for arvo-algebra-contracts (D72's crate
//! table), compiled with `--extern numeral --extern policy` and, load-bearing,
//! WITHOUT `--extern lowering`. `Lowering` is not merely unused here: the
//! symbol does not exist in this compilation unit at all.
#![crate_type = "rlib"]
#![crate_name = "algebra"]

extern crate numeral;
extern crate policy;

use numeral::Numeral;
use policy::Policy;

// 01's translation-stability fact, computed purely over a `Resolution`.
// This is the leaf classification 01/02/03 spent their files on; here it is
// declared, not derived (the const-eval witness from 07/08 is a separate,
// orthogonal mechanism this probe does not need to answer the boundary
// question).
pub trait IsTrue {}
pub struct True;
pub struct False;
impl IsTrue for True {}

pub trait StableUnderTranslation {
    type Out;
}
impl StableUnderTranslation for policy::ReduceModulo {
    type Out = True;
}
impl StableUnderTranslation for policy::SubstituteZero {
    // 01 finding 1: false under substitute-to-zero.
    type Out = False;
}
impl StableUnderTranslation for policy::Refuse {
    // stable one-sided under Kleene equality (01 finding 1's Refuse row).
    type Out = True;
}

/// The law trait. Its impls are meant to be exhaustive over (N, P) and
/// never conditioned on anything algebra cannot name.
pub trait AddAssoc {}

/// The (N, P)-only carrier the fact is actually proved about. `Number`
/// itself cannot live here: it needs `Lowering` for its physical layout,
/// and `Lowering` is not a dependency of this crate.
pub struct Fact<N, P>(core::marker::PhantomData<(N, P)>);

impl<N: Numeral, P: Policy> AddAssoc for Fact<N, P>
where
    P::OverRange: StableUnderTranslation,
    <P::OverRange as StableUnderTranslation>::Out: IsTrue,
{
}

// The block below is the one this probe exists to make impossible. It is
// commented out because IT DOES NOT COMPILE, and that is the result:
// uncommenting it and rebuilding this crate (still without --extern
// lowering) produces:
//   error[E0433]: failed to resolve: use of undeclared crate or module
//   `lowering`
//
// use lowering::Lowering;
// impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Fact<N, P> where
//     L::Layout: core::any::Any
// {
// }
