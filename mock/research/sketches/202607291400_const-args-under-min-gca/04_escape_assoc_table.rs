//! Escape one: move the computation out of type position into an impl table,
//! and project the answer as an associated type. The const parameter appears
//! only as a standalone argument, which is what min_ permits.
//! Needs NO feature gate: verified by compiling it with none.
pub struct Foo<const N: u16>;
pub struct Tbl;
pub trait Sel<const N: u16> {
    type T;
}
// One row per supported N; macro-generated in the real thing.
impl Sel<3> for Tbl {
    type T = Foo<4>;
}
impl Sel<7> for Tbl {
    type T = Foo<8>;
}
pub type A<const N: u16> = <Tbl as Sel<N>>::T;
pub fn use_a() -> A<3> {
    Foo
}
pub fn use_b() -> A<7> {
    Foo
}
