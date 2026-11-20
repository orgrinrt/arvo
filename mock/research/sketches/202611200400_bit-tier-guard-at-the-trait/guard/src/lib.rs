//! Sketch: the index guard owned by the tier, and sealed so no implementor
//! can skip it. The implementor's trait takes a proof `Within<Self, I>` that
//! only `Within::NEW` makes, and `NEW` asserts `I` below `Self::WIDTH`. The
//! caller's trait has the implementor's as its supertrait and is implemented
//! once, over every implementor, so any other impl of it overlaps that one.
#![no_std]
#![feature(const_trait_impl)]

use core::marker::PhantomData;

use arvo_bits::{BitWidth, Bits};
use arvo_format::{Bool, Width};

/// A proof that `I` is below the width of `B`, made only by `NEW`.
pub struct Within<B, const I: u32>(PhantomData<B>);

impl<B: BitWidth, const I: u32> Within<B, I> {
    pub const NEW: Self = {
        assert!(
            I < B::WIDTH.count(),
            "an index or an amount at or past the width"
        );
        Self(PhantomData)
    };
}

/// What an implementor writes: the read and the write at a proven index.
pub const trait BitAccessAt: BitWidth + Copy {
    fn bit_at<const I: u32>(self, at: Within<Self, I>) -> Bool;
    fn with_bit_at<const I: u32>(self, at: Within<Self, I>, to: Bool) -> Self;
}

/// What a caller names: the index alone, proven by the one impl below.
pub const trait BitAccess: BitAccessAt {
    fn bit<const I: u32>(self) -> Bool;
    fn with_bit<const I: u32>(self, to: Bool) -> Self;
}

const impl<B: [const] BitAccessAt> BitAccess for B {
    fn bit<const I: u32>(self) -> Bool {
        self.bit_at(Within::<Self, I>::NEW)
    }

    fn with_bit<const I: u32>(self, to: Bool) -> Self {
        self.with_bit_at(Within::<Self, I>::NEW, to)
    }
}

/// A 256-bit set written with no assertion of its own.
#[derive(Clone, Copy)]
pub struct Wide(pub [Bits<64>; 4]);

const impl BitWidth for Wide {
    const WIDTH: Width = Width::bits(256);
}

const impl BitAccessAt for Wide {
    fn bit_at<const I: u32>(self, _: Within<Self, I>) -> Bool {
        Bool::of((self.0[(I / 64) as usize].raw() >> (I % 64)) & 1 == 1)
    }

    fn with_bit_at<const I: u32>(self, _: Within<Self, I>, to: Bool) -> Self {
        let mut out = self;
        let word = self.0[(I / 64) as usize].raw() & !(1u64 << (I % 64));
        out.0[(I / 64) as usize] = Bits::<64>::masked(word | ((to.get() as u64) << (I % 64)));
        out
    }
}

/// `Bits<8>` through the same shape, as `Bits<N>` would be.
#[derive(Clone, Copy)]
pub struct Eight(pub Bits<8>);

const impl BitWidth for Eight {
    const WIDTH: Width = Width::bits(8);
}

const impl BitAccessAt for Eight {
    fn bit_at<const I: u32>(self, _: Within<Self, I>) -> Bool {
        Bool::of((self.0.raw() >> I) & 1 == 1)
    }

    fn with_bit_at<const I: u32>(self, _: Within<Self, I>, to: Bool) -> Self {
        Self(Bits::<8>::masked(
            (self.0.raw() & !(1u64 << I)) | ((to.get() as u64) << I),
        ))
    }
}
