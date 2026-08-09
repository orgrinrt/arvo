//! crate `downstream_hostile`: depends on numeral, policy, lowering, algebra
//! AND numeric_honest, but owns neither `AddAssoc` nor `Number`. Tries to add
//! a SECOND impl of `AddAssoc` for `Number<N, P, L>`, conditioned on L, to
//! test whether the orphan rule (not the crate split) is what actually
//! protects against an outside actor. This was never in doubt and this
//! crate confirms it costs nothing new: it was already true before this
//! round, for any trait/type pair from two different crates.
#![crate_type = "rlib"]
#![crate_name = "downstream_hostile"]

extern crate algebra;
extern crate lowering;
extern crate numeral;
extern crate numeric_honest;
extern crate policy;

use algebra::AddAssoc;
use lowering::{Lowering, StorageLayout};
use numeral::Numeral;
use numeric_honest::Number;
use policy::Policy;

pub trait Marker {}
impl Marker for () {}

// neither `AddAssoc` (foreign, from algebra) nor `Number` (foreign, from
// numeric_honest) is local to this crate. this should be E0117.
impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Number<N, P, L> where L::Layout: StorageLayout {}
