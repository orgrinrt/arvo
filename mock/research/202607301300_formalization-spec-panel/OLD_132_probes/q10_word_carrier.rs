//! The softer opaque carrier: one machine word for everything up to 64 bits.
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Word<const I: u32, const F: u32, S>(u64, PhantomData<S>);
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Proj<const I: u32, const F: u32, S, T>(T, PhantomData<S>);
pub struct Hot;

pub const W_Q3: usize = core::mem::size_of::<Word<3, 0, Hot>>();
pub const P_Q3: usize = core::mem::size_of::<Proj<3, 0, Hot, u8>>();
pub const W_COL: usize = core::mem::size_of::<[Word<3, 0, Hot>; 1_000_000]>();
pub const P_COL: usize = core::mem::size_of::<[Proj<3, 0, Hot, u8>; 1_000_000]>();
// a 13-field record of narrow values, the bitfield case
pub const W_REC: usize = core::mem::size_of::<[Word<3, 0, Hot>; 13]>();
pub const P_REC: usize = core::mem::size_of::<[Proj<3, 0, Hot, u8>; 13]>();
const _: () = assert!(W_Q3 == 8 && P_Q3 == 1);
const _: () = assert!(W_COL == 8_000_000 && P_COL == 1_000_000);
const _: () = assert!(W_REC == 104 && P_REC == 13);
