// Negative control for p3's type-equality assertions: claim the meet of the
// fixed shape and the float shape is the float, which it is not, and confirm
// the assertion is capable of failing. Without this the two `assert_same`
// checks in p3 could be vacuous.
#![no_std]
#![allow(dead_code)]
include!("p3_shapes_include.rs");
pub const fn wrong_meet() {
    assert_same::<<Fixed as Meet<Float>>::Out, Float>();
}
