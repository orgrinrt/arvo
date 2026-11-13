// An outside overflow policy is held on what it declares: one that clamps is
// refused as the shipped `Clamp` is.
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Overflow, Policy};
use arvo_numeral::Int;

/// An overflow policy of this crate's own.
struct Mine;

impl Overflow for Mine {
    const POLICY: Policy = Policy::Clamp;
}

fn main() {
    let _ = Int::<8, Adapt<TowardZero, Mine>>::constant::<0>();
}
