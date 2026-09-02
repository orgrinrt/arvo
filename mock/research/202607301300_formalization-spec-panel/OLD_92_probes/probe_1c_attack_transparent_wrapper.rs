//! Route 3: a repr(transparent) wrapper around an honest member, claiming
//! membership by layout identity. Expected: refused; layout identity grants
//! nothing at the trait layer, the same asymmetry file 87 found between
//! what repr(transparent) exposes (every bit) and what it proves (nothing).
#![no_std]
extern crate tower;
use core::num::NonZeroU16;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Evil(NonZeroU16);

impl tower::NicheCarrier for Evil {
    type Raw = u16;
    const RAW_BITS: u32 = 16;
    const EXCLUDED: u32 = 1;
}
