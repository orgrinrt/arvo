//! The fixed-max opaque carrier. What it costs in footprint at realistic widths.
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;

/// One opaque carrier for every numeral, sized for the widest the design admits.
#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct Carrier32([u8; 32]);

#[derive(Clone, Copy)]
pub struct FixedOpaque<const I: u32, const F: u32, S>(Carrier32, PhantomData<S>);

/// The projected form, for comparison: the container the ladder would pick.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct FixedProj<const I: u32, const F: u32, S, T>(T, PhantomData<S>);

pub struct Hot;

pub const OPAQUE_Q3_0: usize = core::mem::size_of::<FixedOpaque<3, 0, Hot>>();
pub const PROJ_Q3_0: usize = core::mem::size_of::<FixedProj<3, 0, Hot, u8>>();
pub const OPAQUE_COL: usize = core::mem::size_of::<[FixedOpaque<3, 0, Hot>; 1_000_000]>();
pub const PROJ_COL: usize = core::mem::size_of::<[FixedProj<3, 0, Hot, u8>; 1_000_000]>();
pub const OPAQUE_ALIGN: usize = core::mem::align_of::<FixedOpaque<3, 0, Hot>>();
pub const PROJ_ALIGN: usize = core::mem::align_of::<FixedProj<3, 0, Hot, u8>>();

const _: () = assert!(OPAQUE_Q3_0 == 32);
const _: () = assert!(PROJ_Q3_0 == 1);
const _: () = assert!(OPAQUE_COL == 32_000_000);
const _: () = assert!(PROJ_COL == 1_000_000);
const _: () = assert!(OPAQUE_ALIGN == 16);
const _: () = assert!(PROJ_ALIGN == 1);
