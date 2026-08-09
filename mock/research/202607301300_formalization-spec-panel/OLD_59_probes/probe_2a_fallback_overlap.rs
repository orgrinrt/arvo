//! Probe 2a. The door as a projection from the strategy alone, with a software
//! fallback refined by a hardware impl where the numeral is host-implemented.
//!
//! This is the shape the presumptive per-preset table (58:820-822) implies when
//! read literally: "`Hot` carries the receipt-carrying hardware lowering",
//! which cannot be total, because a `Ranged` numeral the host has no
//! instruction for still has to lower somehow.
//!
//! NEGATIVE CONTROL. Expected: E0119.

pub trait Numeral {}
/// The numerals this target's silicon implements. binary16/32/64 here.
pub trait HostFormat: Numeral {}

pub struct Binary32;
pub struct Ranged11
// p=11, emin=-14, emax=15, Underflow=Abrupt: no instruction;
impl Numeral for Binary32 {}
impl Numeral for Ranged11 {}
impl HostFormat for Binary32 {}

pub struct Quantised;
pub struct HostFloat;
pub struct Hot;

pub trait DoorFor<N: Numeral> {
    type Out;
}

impl<N: Numeral> DoorFor<N> for Hot {
    type Out = Quantised;
}

impl<N: Numeral + HostFormat> DoorFor<N> for Hot {
    type Out = HostFloat;
}

fn main() {}
