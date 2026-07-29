//! Escape two: the width stops being a const parameter and becomes a type,
//! the way Capacity carries an array. Nothing computes in type position at
//! all, so no const-generic feature is involved.
pub struct Foo<W: Width> {
    _w: core::marker::PhantomData<W>,
}
pub trait Width {
    const BITS: u16;
    type Wider: Width;
}
pub struct W3;
pub struct W4;
pub struct W8;
impl Width for W3 {
    const BITS: u16 = 3;
    type Wider = W4;
}
impl Width for W4 {
    const BITS: u16 = 4;
    type Wider = W8;
}
impl Width for W8 {
    const BITS: u16 = 8;
    type Wider = W8;
}
// The "computation" is a projection: one wider than W is W::Wider.
pub type Widened<W> = Foo<<W as Width>::Wider>;
pub fn use_it() -> Widened<W3> {
    Foo {
        _w: core::marker::PhantomData,
    }
}
