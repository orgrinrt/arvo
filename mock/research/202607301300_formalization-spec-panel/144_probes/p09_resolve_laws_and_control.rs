// P9. The precedence function's laws, asserted as type equalities the compiler
// checks, with a live negative control. A law whose control passes is no law.
#![allow(dead_code)]
pub struct Ambient;
pub struct Hot;
pub struct Warm;
pub struct Precise;
pub trait Marker {}
impl Marker for Hot {}
impl Marker for Warm {}
impl Marker for Precise {}
pub trait Resolve<P> {
    type Out;
}
impl<P: Marker> Resolve<P> for Ambient {
    type Out = P;
}
impl<P> Resolve<P> for Hot {
    type Out = Hot;
}
impl<P> Resolve<P> for Warm {
    type Out = Warm;
}
impl<P> Resolve<P> for Precise {
    type Out = Precise;
}

trait Same<T> {}
impl<T> Same<T> for T {}
fn assert_same<A: Same<B>, B>() {}

fn laws() {
    // the elision yields to the scope, for every scope posture
    assert_same::<<Ambient as Resolve<Hot>>::Out, Hot>();
    assert_same::<<Ambient as Resolve<Warm>>::Out, Warm>();
    assert_same::<<Ambient as Resolve<Precise>>::Out, Precise>();
    // a declaration is invariant under every scope posture: 3x3, not a sample
    assert_same::<<Hot as Resolve<Hot>>::Out, Hot>();
    assert_same::<<Hot as Resolve<Warm>>::Out, Hot>();
    assert_same::<<Hot as Resolve<Precise>>::Out, Hot>();
    assert_same::<<Warm as Resolve<Hot>>::Out, Warm>();
    assert_same::<<Warm as Resolve<Warm>>::Out, Warm>();
    assert_same::<<Warm as Resolve<Precise>>::Out, Warm>();
    assert_same::<<Precise as Resolve<Hot>>::Out, Precise>();
    assert_same::<<Precise as Resolve<Warm>>::Out, Precise>();
    assert_same::<<Precise as Resolve<Precise>>::Out, Precise>();
    // idempotence: resolving twice is resolving once, over the whole 4x3 grid
    assert_same::<<<Ambient as Resolve<Hot>>::Out as Resolve<Warm>>::Out, Hot>();
    assert_same::<<<Precise as Resolve<Hot>>::Out as Resolve<Hot>>::Out, Precise>();
}

// NEGATIVE CONTROL: the scope must NOT override a declaration. If this line
// compiles, the precedence rule is not implemented and every law above is void.
fn control() {
    assert_same::<<Precise as Resolve<Hot>>::Out, Hot>();
}
fn main() {
    laws();
    control();
}
