#![feature(const_trait_impl)]
use union::*;

// B1: signed clamping fold. The headline refusal the panel keeps citing.
pub fn b1() {
    fold::<IFixed<13, 3, Warm>>()
}
// B2: same, with a one-axis modifier the consumer wrote.
pub fn b2() {
    fold::<IFixed<13, 3, LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>>>()
}
// B3: 01 finding 1's SubstituteZero counterexample, unsigned.
pub fn b3() {
    fold::<UFixed<13, 3, OverRangeOf<Warm, SubstituteZero>>>()
}
// B4: the arithmetic on a composition whose delivery has no lift.
pub fn b4() {
    let _ =
        add::<Number<Fix<13, 3, Unsigned>, Precise, DeliveredAs<Precise, AsFlag>>>(1, 2, 0, 100);
}
pub struct AsFlag;
fn main() {}
