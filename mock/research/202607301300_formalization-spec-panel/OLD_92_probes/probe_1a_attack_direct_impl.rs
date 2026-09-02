//! Route 1: a foreign crate implements the sealed trait directly for a
//! local type. Expected: refused, the supertrait bound unsatisfiable.
#![no_std]
extern crate tower;

#[derive(Copy, Clone)]
pub struct Forged(u16);

impl tower::NicheCarrier for Forged {
    type Raw = u16;
    const RAW_BITS: u32 = 16;
    const EXCLUDED: u32 = 1;
}
