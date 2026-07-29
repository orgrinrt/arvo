//! The same shape under the allowed successor.
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub struct Foo<const N: u16>;
pub const fn g(n: u16) -> u16 {
    n + 1
}
pub type A<const N: u16> = Foo<{ g(N) }>;
pub fn use_it() -> A<3> {
    Foo
}
