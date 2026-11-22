//! Sketch: a proof that `I` is below the width of `B`, made only by `NEW`, in
//! two shapes. `Covariant` holds `PhantomData<B>`, the shape `Within` had;
//! `Invariant` holds `PhantomData<fn(B) -> B>`. `Wrap<for<'a> fn(&'a ())>` is
//! a subtype of `Wrap<fn(&'static ())>`, and each declares its own width, so a
//! covariant proof made for the wide one coerces to the narrow one.
#![no_std]
#![allow(coherence_leak_check)]

use core::marker::PhantomData;

pub trait Width {
    const WIDTH: u32;
}

pub struct Covariant<B, const I: u32>(PhantomData<B>);

impl<B: Width, const I: u32> Covariant<B, I> {
    pub const NEW: Self = {
        assert!(I < B::WIDTH, "an index at or past the width");
        Self(PhantomData)
    };
}

pub struct Invariant<B, const I: u32>(PhantomData<fn(B) -> B>);

impl<B: Width, const I: u32> Invariant<B, I> {
    pub const NEW: Self = {
        assert!(I < B::WIDTH, "an index at or past the width");
        Self(PhantomData)
    };
}

/// A container whose storage holds `bytes` bytes, typed by `T`.
pub struct Wrap<T> {
    pub bytes: &'static [u8],
    pub kind:  PhantomData<T>,
}

pub type Wide = Wrap<for<'a> fn(&'a ())>;
pub type Narrow = Wrap<fn(&'static ())>;

impl Width for Wide {
    const WIDTH: u32 = 128;
}

impl Width for Narrow {
    const WIDTH: u32 = 8;
}

/// What an implementor writes, trusting the proof: no check of its own.
pub fn bit_covariant<T, const I: u32>(of: &Wrap<T>, _: Covariant<Wrap<T>, I>) -> bool {
    of.bytes[(I / 8) as usize] >> (I % 8) & 1 == 1
}

pub fn bit_invariant<T, const I: u32>(of: &Wrap<T>, _: Invariant<Wrap<T>, I>) -> bool {
    of.bytes[(I / 8) as usize] >> (I % 8) & 1 == 1
}
