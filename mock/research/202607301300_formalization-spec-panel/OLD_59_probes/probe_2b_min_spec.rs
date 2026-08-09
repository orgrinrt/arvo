//! Probe 2b. Probe 2a's identical shape under `min_specialization`, the only
//! specialisation door the workspace permits (`unstable-features.md`: full
//! `specialization` is forbidden, `min_specialization` allowed by the
//! std-internal carve-out).
//!
//! NEGATIVE CONTROL. Expected: refusal, and specifically not a compile.
#![feature(min_specialization)]

pub trait Numeral {}
pub trait HostFormat: Numeral {}

pub struct Binary32;
pub struct Ranged11;
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
    default type Out = Quantised;
}

impl<N: Numeral + HostFormat> DoorFor<N> for Hot {
    type Out = HostFloat;
}

fn main() {}
