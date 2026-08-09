//! Does the borrow in `(&a).into()` cost an instruction?
//!
//! op: "It is a cast and we have all we need to do it on compile time, all lowered
//! inlined." The by-reference spelling is the one that compiles; this checks whether
//! the reference survives lowering. Two pairs of functions, each pair computing the
//! same thing, one through the conversion and one by hand.
//!
//!   rustc -O --emit asm --crate-type lib -Znext-solver=globally f12_codegen.rs
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

pub struct Unsigned;
pub struct Warm;

#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, G, S>(pub u32, PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}
impl<const I: u32, const F: u32, G, S> Fixed<I, F, G, S> {
    pub const fn from_raw(r: u32) -> Self {
        Fixed(r, PhantomData)
    }
    pub const fn raw(self) -> u32 {
        self.0
    }
}

pub const fn tag_embeds(i1: u32, f1: u32, i2: u32, f2: u32) -> usize {
    if i1 <= i2 && f1 <= f2 {
        0
    } else {
        1
    }
}
pub struct Picker;
pub struct Pair<const I1: u32, const F1: u32, const I2: u32, const F2: u32>;
pub trait Tagged {
    type const TAG: usize;
}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> Tagged for Pair<I1, F1, I2, F2> {
    type const TAG: usize = const { tag_embeds(I1, F1, I2, F2) };
}
pub trait EmbedWitness<const TAG: usize> {}
impl EmbedWitness<0> for Picker {}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<&Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
{
    #[inline]
    fn from(src: &Fixed<I1, F1, G, S>) -> Self {
        Fixed(src.0 << (F2 - F1), PhantomData)
    }
}

#[no_mangle]
pub fn scalar_via_conversion(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    (&a).into()
}

#[no_mangle]
pub fn scalar_by_hand(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    UFixed::<20, 8, Warm>::from_raw(a.raw() << 5)
}

#[no_mangle]
pub fn loop_via_conversion(src: &[UFixed<13, 3, Warm>], dst: &mut [UFixed<20, 8, Warm>]) {
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = s.into();
    }
}

#[no_mangle]
pub fn loop_by_hand(src: &[UFixed<13, 3, Warm>], dst: &mut [UFixed<20, 8, Warm>]) {
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = UFixed::<20, 8, Warm>::from_raw(s.raw() << 5);
    }
}
