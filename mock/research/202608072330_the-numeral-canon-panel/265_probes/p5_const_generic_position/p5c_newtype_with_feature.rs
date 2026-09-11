//! p5c: p5b with the allowed-tier feature that admits an ADT there.
//! Differs from p5b in one thing: the gate and the marker it requires.
#![no_std]
#![feature(adt_const_params)]
use core::marker::ConstParamTy;
#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Cap(usize);
pub struct Buf<const N: Cap>;
pub type Four = Buf<{ Cap(4) }>;
