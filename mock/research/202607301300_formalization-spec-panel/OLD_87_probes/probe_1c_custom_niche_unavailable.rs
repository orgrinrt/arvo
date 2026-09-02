// Confirming file 84's finding independently: no stable, general mechanism exists on this
// toolchain for a downstream Lowering author to declare an arbitrary custom validity range
// on their own carrier type. The only route is pattern_types, internal-features-flagged.
#![feature(pattern_types)]
#![feature(core_pattern_type)]
use core::pat::pattern_type;
type Custom = pattern_type!(u16 is 1..=65535);
fn main() {}
