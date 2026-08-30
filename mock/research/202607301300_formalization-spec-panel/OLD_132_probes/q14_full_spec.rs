//! Full `specialization` (forbidden, but shipping in the engine today) under the flag.
#![no_std]
#![crate_type = "lib"]
#![feature(specialization)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub struct PtrCons<H, T>(PhantomData<(H, T)>);
pub struct Nil;
pub trait TryHead<T> {
    fn find(&self) -> u32;
}
impl<T, H, Tail> TryHead<T> for PtrCons<H, Tail> {
    default fn find(&self) -> u32 {
        0
    }
}
impl<T, Tail> TryHead<T> for PtrCons<T, Tail> {
    fn find(&self) -> u32 {
        1
    }
}
