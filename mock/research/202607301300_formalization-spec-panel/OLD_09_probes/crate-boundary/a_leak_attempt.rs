//! Same as algebra_leak_attempt.rs but without an explicit `extern crate`
//! line, since edition 2024 crates reference dependencies directly by name.
//! This is the realistic shape: someone just writes `use lowering::Lowering;`
//! inside the algebra crate's own source, with no --extern lowering= passed
//! at all (because Cargo, per D72, would simply never list arvo-lowering as
//! a dependency of arvo-algebra-contracts).
#![crate_type = "rlib"]
#![crate_name = "algebra_leak_attempt2"]

use lowering::Lowering;
use numeral::Numeral;
use policy::Policy;

pub trait IsTrue {}
pub struct True;
impl IsTrue for True {}

pub trait AddAssoc {}
pub struct Fact<N, P, L>(core::marker::PhantomData<(N, P, L)>);

impl<N: Numeral, P: Policy, L: Lowering> AddAssoc for Fact<N, P, L> where L::Layout: core::any::Any {}
