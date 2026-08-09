//! Probe 6. The third route, which neither the panel nor the 2026-07-28 sketch
//! states, and which decides the price.
//!
//! Probe 2 showed the width cannot become a type while `I` and `F` stay consts,
//! because the ADDITION is what sits in type position. That conclusion assumes
//! the addition has to happen. It does not: `UFixed`'s first parameter can BE
//! the total width, with the fraction point as the second, so nothing is ever
//! computed in type position at all.
//!
//! Today:  UFixed<const I: IBits, const F: FBits, S>  ->  Bits<{ I + F }, S>
//! Here:   UFixed<const W: Width,  const F: FBits, S>  ->  Bits<Wid<W>, S>
//!
//! `W` is a standalone const argument, which the grammar already permits.
#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use core::marker::{ConstParamTy, PhantomData};

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Width(pub u16);
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct FBits(pub u16);

// Width as typestate, the 2026-07-28 sketch's own `Wid<N>`.
pub struct Wid<const N: Width>;
pub struct B8;
pub struct B16;
pub struct B32;
pub trait Bucket {}
impl Bucket for B8 {}
impl Bucket for B16 {}
impl Bucket for B32 {}
pub trait WidthFor {
    type Bkt: Bucket;
}
macro_rules! widths { ($($n:literal => $b:ty),* $(,)?) => {
    $( impl WidthFor for Wid<{ Width($n) }> { type Bkt = $b; } )* }; }
widths!(1 => B8, 7 => B8, 8 => B8, 9 => B16, 13 => B16, 16 => B16, 17 => B32, 24 => B32, 32 => B32);

pub trait Strategy {}
pub struct Hot;
impl Strategy for Hot {}
pub trait Project<B: Bucket, S: Strategy> {
    type T: Copy;
}
pub struct Picker;
impl Project<B8, Hot> for Picker {
    type T = u8;
}
impl Project<B16, Hot> for Picker {
    type T = u16;
}
impl Project<B32, Hot> for Picker {
    type T = u32;
}

pub trait BitsContainerFor<W>: Strategy {
    type T: Copy;
}
impl<W: WidthFor> BitsContainerFor<W> for Hot
where
    Picker: Project<<W as WidthFor>::Bkt, Hot>,
{
    type T = <Picker as Project<<W as WidthFor>::Bkt, Hot>>::T;
}

#[repr(transparent)]
pub struct Bits<W, S: Strategy>(<S as BitsContainerFor<W>>::T, PhantomData<(W, S)>)
where
    S: BitsContainerFor<W>;

// The facade type, re-parameterised. NOTE: no expression anywhere in type
// position. `W` is passed straight through.
#[repr(transparent)]
pub struct UFixed<const W: Width, const F: FBits, S: Strategy>(Bits<Wid<W>, S>)
where
    S: BitsContainerFor<Wid<W>>;

// A method body still needs I and F separately; that is value position, where
// arithmetic has always been free.
impl<const W: Width, const F: FBits, S: Strategy> UFixed<W, F, S>
where
    S: BitsContainerFor<Wid<W>>,
{
    pub const INT_BITS: u16 = W.0 - F.0;
    pub const FRAC_BITS: u16 = F.0;
    pub const TOTAL: u16 = W.0;
}

// Signed counterpart: the sign bit is folded into the declared total, so the
// `1 +` that forced `ifixed_bits` into type position disappears entirely.
#[repr(transparent)]
pub struct IFixed<const W: Width, const F: FBits, S: Strategy>(Bits<Wid<W>, S>)
where
    S: BitsContainerFor<Wid<W>>;

// Call sites.
pub type Byte = UFixed<{ Width(8) }, { FBits(0) }, Hot>;
pub type Q13_3 = UFixed<{ Width(16) }, { FBits(3) }, Hot>; // was UFixed<13, 3>
pub type S7 = IFixed<{ Width(8) }, { FBits(0) }, Hot>; // was IFixed<7, 0>

const _: () = {
    assert!(<Q13_3>::INT_BITS == 13);
    assert!(<Q13_3>::FRAC_BITS == 3);
    assert!(<Byte>::TOTAL == 8);
};

// The generic-threading case: a caller threading its own const through.
pub fn threaded<const W: Width, const F: FBits, S: Strategy>(_x: UFixed<W, F, S>)
where
    S: BitsContainerFor<Wid<W>>,
{
}
const _: () = {
    let _ = threaded::<{ Width(13) }, { FBits(3) }, Hot>;
};
