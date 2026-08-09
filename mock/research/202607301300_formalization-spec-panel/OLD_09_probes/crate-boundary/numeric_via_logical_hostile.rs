//! attempts to add a second, L-conditioned impl of AddAssoc for
//! LogicalNumber from a crate that has Lowering in scope but does not own
//! either the trait or the type.
#![crate_type = "rlib"]
#![crate_name = "numeric_via_logical_hostile"]
extern crate algebra_logical;
extern crate lowering;
extern crate numeral;
extern crate policy;

use algebra_logical::{AddAssoc, LogicalNumber};
use lowering::{Dense, Lowering, StorageLayout};

pub trait IsDense {}
impl IsDense for Dense {}

impl<N: numeral::Numeral, P: policy::Policy, L: Lowering> AddAssoc for LogicalNumber<N, P, L> where
    L::Layout: IsDense
{
}
