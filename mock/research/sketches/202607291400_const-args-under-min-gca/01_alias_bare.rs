//! The shape arvo's ergonomic aliases actually use, with no feature gate.
//! Expect: refused, naming generic_const_exprs.
pub struct Foo<const N: u16>;
pub const fn g(n: u16) -> u16 {
    n + 1
}
pub type A<const N: u16> = Foo<{ g(N) }>;
pub fn use_it() -> A<3> {
    Foo
}
