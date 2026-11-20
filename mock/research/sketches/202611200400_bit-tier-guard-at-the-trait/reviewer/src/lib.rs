//! Spike: can a trait enforce the index refusal for every implementor, so no
//! implementor owes an assertion of its own? A token whose constructor asserts
//! `I < B::WIDTH` over the implementor's own `BitWidth`.
#![no_std]
#![feature(const_trait_impl)]
use core::marker::PhantomData;

use arvo_bits::{BitWidth, Bits};
use arvo_format::{Bool, Width};

/// An index proven below `B::WIDTH`, only constructible through `NEW`.
pub struct At<B, const I: u32>(PhantomData<B>);
impl<B: BitWidth, const I: u32> At<B, I> {
    pub const NEW: Self = {
        assert!(I < B::WIDTH.count(), "index at or past the width");
        At(PhantomData)
    };
}

/// The trait owns the refusal: the provided method forces the token, the
/// required one receives it already proven.
pub const trait Access: BitWidth + Copy {
    fn read_at<const I: u32>(self, at: At<Self, I>) -> Bool;
    fn bit<const I: u32>(self) -> Bool {
        self.read_at::<I>(At::<Self, I>::NEW)
    }
}

/// A 256-bit container whose implementor writes no assertion at all.
#[derive(Clone, Copy)]
pub struct Wide(pub [Bits<64>; 4]);
impl BitWidth for Wide {
    const WIDTH: Width = Width::bits(256);
}
impl Access for Wide {
    fn read_at<const I: u32>(self, _: At<Self, I>) -> Bool {
        Bool::of((self.0[(I / 64) as usize].raw() >> (I % 64)) & 1 == 1)
    }
}
