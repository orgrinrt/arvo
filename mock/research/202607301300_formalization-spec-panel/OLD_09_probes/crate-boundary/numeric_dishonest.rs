//! Same crate as numeric_honest.rs, except the ONE forwarding impl also
//! conditions on `L::Layout`, exactly the shape `08_probes/c_split_does_not_
//! bind.rs` wrote, but now attempted at the one site that actually owns
//! `Number` and legitimately has `Lowering` in scope. This is the honest
//! test of whether the crate split (D72) closes 08's finding, or only moves
//! it.
//!
//! Made concrete rather than vacuous: `IsDense` holds only for `Dense`, not
//! `Bitpacked`, so two compositions equal in every load-bearing (N, P) way
//! and differing only in the cost axis L now differ in whether the same
//! law, `AddAssoc`, holds for them. That is the thing 02 said typing the
//! split would make impossible.
#![crate_type = "rlib"]
#![crate_name = "numeric_dishonest"]

extern crate algebra;
extern crate lowering;
extern crate numeral;
extern crate policy;

use algebra::AddAssoc;
use lowering::{Dense, Lowering};
use numeral::Numeral;
use policy::Policy;

pub trait IsDense {}
impl IsDense for Dense {}

pub struct Number<N: Numeral, P: Policy, L: Lowering>(core::marker::PhantomData<(N, P, L)>);

// the forwarding impl, now ALSO conditioned on L. this is legal Rust: L is
// a generic parameter of the impl (Number<N,P,L> requires it), so nothing
// stops adding a where-clause on it. this is the finding: the crate split
// does not, and structurally cannot, prevent THIS impl from reading L,
// because this impl is necessarily authored in the one crate where L is in
// scope for Number's own definition.
impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Number<N, P, L>
where
    algebra::Fact<N, P>: AddAssoc,
    L::Layout: IsDense,
{
}

pub fn fold<T: AddAssoc>() {}
pub fn compiles_clean_at_dense() {
    fold::<Number<numeral::Fix13_3Signed, policy::Warm, lowering::MinWidth>>();
}

// same N and P, only L differs, and this refuses: the "same fact" from
// algebra now holds or does not hold depending purely on a Lowering member,
// which is exactly the correctness-conditioned-on-cost violation this
// panel's job one exists to test for.
//
// pub fn refuses_at_bitpacked() {
//     fold::<Number<numeral::Fix13_3Signed, policy::Warm, lowering::DoubleWidth>>();
// }
