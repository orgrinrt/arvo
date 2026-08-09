//! Probe 1. Does `min_generic_const_args` admit the shipped `UFixed` shape?
//!
//! Reduced to the smallest thing that reproduces `arvo/src/ufixed.rs:35-38`:
//! a computed width from two `ConstParamTy` const params, in type position.
//! The consolidation (63:812) states the minimal successor "refuses
//! identically"; this checks that claim directly rather than inheriting it.
#![no_std]
#![feature(min_generic_const_args)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct IBits(pub u16);
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct FBits(pub u16);

pub const fn ufixed_bits(i: IBits, f: FBits) -> u16 {
    i.0 + f.0
}

pub trait Container<const N: u16> {
    type T: Copy;
}
pub struct Hot;
impl<const N: u16> Container<N> for Hot {
    type T = u64;
}

#[repr(transparent)]
pub struct UFixed<const I: IBits, const F: FBits, S>(<S as Container<{ ufixed_bits(I, F) }>>::T)
where
    S: Container<{ ufixed_bits(I, F) }>;
