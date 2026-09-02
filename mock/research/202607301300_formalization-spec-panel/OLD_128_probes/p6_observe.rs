//! P6: can GCA let an impl OBSERVE its instantiation and behave differently?
//! That is the property the specialization/TypeId bans protect.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub trait Op {
    fn run() -> u32;
}
pub struct S<const N: u16>;

// attempt A: two impls partitioned by a const-computed predicate
impl<const N: u16> Op for S<N> {
    fn run() -> u32 {
        1
    }
}
impl Op for S<8> {
    fn run() -> u32 {
        2
    }
} // must be refused as overlapping
