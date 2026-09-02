//! crate `numeric_honest`: stand-in for arvo-numeric. Depends on all four:
//! numeral, policy, lowering, algebra. Owns `Number<N, P, L>`, whose PHYSICAL
//! layout genuinely needs `L: Lowering` (that is the entire reason Lowering
//! exists as an axis), so this crate cannot avoid having `Lowering` in scope
//! wherever `Number` is declared.
//!
//! The forwarding impl of `algebra::AddAssoc` for `Number<N, P, L>` lives
//! here, is the ONLY impl of that trait for `Number` anywhere (checked by
//! `downstream_hostile.rs`), and does not mention `L`.
#![crate_type = "rlib"]
#![crate_name = "numeric_honest"]

extern crate algebra;
extern crate lowering;
extern crate numeral;
extern crate policy;

use algebra::AddAssoc;
use lowering::Lowering;
use numeral::Numeral;
use policy::Policy;

/// The physically real numeral. Its storage genuinely depends on `L`
/// (the field width in a real implementation would be `L::StoredWidth`
/// bytes; here a phantom stands in, since only the *type-level* dependency
/// on L matters for this probe).
pub struct Number<N: Numeral, P: Policy, L: Lowering>(core::marker::PhantomData<(N, P, L)>);

// the one authorized forwarding impl. no L bound in the where clause.
impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Number<N, P, L> where
    algebra::Fact<N, P>: AddAssoc
{
}

pub fn fold<T: AddAssoc>() {}

pub fn check_warm_folds() {
    fold::<Number<numeral::Fix13_3Signed, policy::Warm, lowering::MinWidth>>();
}

// uncommenting this should refuse to compile, and does (see the .stderr
// captured in the panel file): SubstituteZero's fact is False, so
// `algebra::Fact<N, Hot>: AddAssoc` is unsatisfied.
//
// pub fn check_hot_refuses() {
//     fold::<Number<numeral::Fix13_3Signed, policy::Hot, lowering::MinWidth>>();
// }
