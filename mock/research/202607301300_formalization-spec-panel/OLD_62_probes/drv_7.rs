#![feature(const_trait_impl)]
// fresh width-sweep driver, file 62. five constructors, one one-sided
// stability check each, at width 7 (span 0..=127), forced by const items.
#![crate_type = "lib"]
use union::{stable, ReduceModulo, Refuse, SubstituteZero, TowardNegative, TowardPositive};
const A: bool = stable::<ReduceModulo>(0, 127, false);
const B: bool = stable::<TowardNegative>(0, 127, false);
const C: bool = stable::<TowardPositive>(0, 127, false);
const D: bool = stable::<SubstituteZero>(0, 127, false);
const E: bool = stable::<Refuse>(0, 127, false);
const _: [bool; 5] = [A, B, C, D, E];
