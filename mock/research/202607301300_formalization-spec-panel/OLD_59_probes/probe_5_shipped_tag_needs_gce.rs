//! Probe 5. Whether the shipped `BitsContainerFor` dispatch shape, which
//! probe_3 deliberately did not copy, is load-bearing on a forbidden feature.
//!
//! The shape is `arvo-strategy/src/container.rs:254-258`:
//!
//!     const impl<const N: u16, Sign> BitsContainerFor<N, Sign> for Hot
//!     where Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>
//!
//! A const expression over a generic const parameter, in a bound. Reduced to
//! its essentials here with no arvo dependency.
//!
//! NEGATIVE CONTROL, run three ways: bare, under `min_generic_const_args`, and
//! under `generic_const_exprs`. Only the third is expected to compile, and the
//! third is forbidden (`unstable-features.md`, op 2026-07-28).

pub struct Picker;
pub trait Project<const TAG: u8> {
    type T;
}
impl Project<0> for Picker {
    type T = u8;
}
impl Project<1> for Picker {
    type T = u64;
}

pub const fn tag(n: u16) -> u8 {
    if n <= 8 {
        0
    } else {
        1
    }
}

pub trait ContainerFor<const N: u16> {
    type T;
}
pub struct Hot;

impl<const N: u16> ContainerFor<N> for Hot
where
    Picker: Project<{ tag(N) }>,
{
    type T = <Picker as Project<{ tag(N) }>>::T;
}

fn main() {}
