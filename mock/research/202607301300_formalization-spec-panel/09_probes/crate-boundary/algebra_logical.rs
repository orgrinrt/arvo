//! the closure attempt that actually works: the type the law trait targets
//! carries NO Lowering bound at all, so it can be defined AND have its ONE
//! law impl written entirely inside the Lowering-blind crate. No forwarding
//! step, no crate with both Policy and Lowering in scope ever touches the
//! law's impl header.
#![crate_type = "rlib"]
#![crate_name = "algebra_logical"]

extern crate numeral;
extern crate policy;

use numeral::Numeral;
use policy::Policy;

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
    type Out = False;
}
impl StableUnderTranslation for policy::Refuse {
    type Out = True;
}

pub trait AddAssoc {}

/// L is a FREE type parameter here: no bound, because nothing in this
/// crate can name a bound for it. That absence of a bound is not a
/// simplification, it is the enforcement mechanism: there is no `where`
/// clause slot this impl could ever attach an `L::Layout` condition to,
/// because `L` has no methods or associated items visible at all.
pub struct LogicalNumber<N, P, L>(core::marker::PhantomData<(N, P, L)>);

impl<N: Numeral, P: Policy, L> AddAssoc for LogicalNumber<N, P, L>
where
    P::OverRange: StableUnderTranslation,
    <P::OverRange as StableUnderTranslation>::Out: IsTrue,
{
}
