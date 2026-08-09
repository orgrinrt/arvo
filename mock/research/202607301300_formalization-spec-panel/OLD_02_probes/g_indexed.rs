// TEST G: replace the linear `Direction: Resolution` chain with per-position
// capability, which is what the data actually is. The containment the chain
// asserts is false in both directions:
//   AwayFromZero is a Direction and is NOT usable past either end
//   ReduceModulo is usable past an end and is NOT usable between neighbours
// so the true picture is three overlapping sets, not a two-level chain.
#![allow(dead_code)]

pub trait UsableBetween {} // a rule for the interior of a cell
pub trait UsableAbove {} // past the top: only the neighbour BELOW exists
pub trait UsableBelow {} // past the bottom: only the neighbour ABOVE exists

pub struct TowardNegative;
impl UsableBetween for TowardNegative {}
impl UsableAbove for TowardNegative {}
pub struct TowardPositive;
impl UsableBetween for TowardPositive {}
impl UsableBelow for TowardPositive {}
pub struct TowardZero;
impl UsableBetween for TowardZero {}
impl UsableAbove for TowardZero {}
impl UsableBelow for TowardZero {}
pub struct AwayFromZero;
impl UsableBetween for AwayFromZero {} // neither end
pub struct ToEven;
impl UsableBetween for ToEven {} // parity-dependent at an end
pub struct ToOdd;
impl UsableBetween for ToOdd {}
pub struct ReduceModulo;
impl UsableAbove for ReduceModulo {}
impl UsableBelow for ReduceModulo {}
pub struct SubstituteZero;
impl UsableAbove for SubstituteZero {}
impl UsableBelow for SubstituteZero {}
pub struct Refuse;
impl UsableAbove for Refuse {}
impl UsableBelow for Refuse {}

pub trait Quantisation {
    type UnderMidpoint: UsableBetween;
    type OnMidpoint: UsableBetween;
    type OverMidpoint: UsableBetween;
    type OverRange: UsableAbove;
    type UnderRange: UsableBelow;
}

pub struct Saturating;
impl Quantisation for Saturating {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative; // clamp to MAX
    type UnderRange = TowardPositive; // clamp to MIN
}

// each of the two refusals the spec wants, now BOTH by typing:
pub struct ModAtMidpoint;
impl Quantisation for ModAtMidpoint {
    type UnderMidpoint = ReduceModulo; // ERROR 1: the spec already refuses this
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
pub struct AwayAtRange;
impl Quantisation for AwayAtRange {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = AwayFromZero; // ERROR 2: the spec's chain ACCEPTS this
    type UnderRange = TowardPositive;
}
fn main() {}
