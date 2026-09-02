// TEST E: the spec says `Direction: Resolution`, i.e. every rule usable between
// two neighbours is usable past the end of the range. Does the typing then
// accept a rule that names a neighbour which does not exist?
#![allow(dead_code)]
pub trait Resolution {}
pub trait Direction: Resolution {}

macro_rules! dir { ($($n:ident)*) => { $(pub struct $n; impl Resolution for $n {} impl Direction for $n {})* } }
dir!(TowardNegative TowardPositive TowardZero AwayFromZero ToEven ToOdd);
pub struct ReduceModulo;
impl Resolution for ReduceModulo {}
pub struct SubstituteZero;
impl Resolution for SubstituteZero {}
pub struct Refuse;
impl Resolution for Refuse {}

pub trait Quantisation {
    type UnderMidpoint: Direction;
    type OnMidpoint: Direction;
    type OverMidpoint: Direction;
    type OverRange: Resolution;
    type UnderRange: Resolution;
}

/// Above the top of the range the only existing neighbour is MAX, which lies
/// BELOW. `AwayFromZero` names the one above it, which does not exist. Same for
/// `TowardPositive`. The spec's own hierarchy admits both.
pub struct Nonsense;
impl Quantisation for Nonsense {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = AwayFromZero; // return the nonexistent neighbour above MAX
    type UnderRange = AwayFromZero; // and the nonexistent one below MIN
}
fn main() {
    println!("E: a quantisation whose range rules name no value COMPILES");
}
