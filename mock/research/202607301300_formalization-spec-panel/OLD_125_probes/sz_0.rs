#![no_std]
#![allow(dead_code)]
extern crate tower;
pub use tower::*;
pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Out: Nat;
}
