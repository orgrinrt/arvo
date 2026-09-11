//! p5b: the same position, with a stack-owned newtype as the parameter's type.
//! Differs from p5a in one thing: the parameter's type is a newtype.
#![no_std]
#[derive(PartialEq, Eq)]
pub struct Cap(usize);
pub struct Buf<const N: Cap>;
