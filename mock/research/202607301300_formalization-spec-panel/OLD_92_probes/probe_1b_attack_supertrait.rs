//! Route 2: the foreign crate tries to satisfy the bound by implementing
//! the private supertrait first. Expected: refused, the module is private.
#![no_std]
extern crate tower;

#[derive(Copy, Clone)]
pub struct Forged(u16);

impl tower::sealed::Sealed for Forged {}
