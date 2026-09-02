#![feature(const_trait_impl)]
// fresh width-sweep driver, file 62. width 9 (span 0..=511).
#![crate_type = "lib"]
use union::{stable, ReduceModulo, Refuse, SubstituteZero, TowardNegative, TowardPositive};
const A: bool = stable::<ReduceModulo>(0, 511, false);
const B: bool = stable::<TowardNegative>(0, 511, false);
const C: bool = stable::<TowardPositive>(0, 511, false);
const D: bool = stable::<SubstituteZero>(0, 511, false);
const E: bool = stable::<Refuse>(0, 511, false);
const _: [bool; 5] = [A, B, C, D, E];
