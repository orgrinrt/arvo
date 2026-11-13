// The twin: an outside overflow policy that declares wrapping is held.
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Overflow, Policy};
use arvo_numeral::Int;

/// An overflow policy of this crate's own.
struct Mine;

impl Overflow for Mine {
    const POLICY: Policy = Policy::Wrap;
}

fn main() {
    let _ = Int::<8, Adapt<TowardZero, Mine>>::constant::<0>();
}
