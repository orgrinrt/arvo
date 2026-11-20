//! The second seat's shape over the real `Bits`, `Bool` and `notko-hlist`:
//! positions derived from the list order, low bit first, and the const impl
//! required once on the declaring traits' associated types, so the accessors
//! are plain `const fn`s a consumer with no feature line calls in a constant.

#![no_std]
#![feature(const_trait_impl)]

use core::marker::PhantomData;

use arvo_bits::{BitWidth, Bits};
use arvo_format::{Bool, Width};
use notko_hlist::{Cons, Empty, Here, Place, PlaceOf, There};

pub const trait Lane: Copy {
    const WIDTH: Width;
    fn from_low(low: Bits<64>) -> Self;
    fn low(self) -> Bits<64>;
}

const impl<const N: u32> Lane for Bits<N> {
    const WIDTH: Width = <Self as BitWidth>::WIDTH;
    fn from_low(low: Bits<64>) -> Self {
        low.cast::<N>()
    }
    fn low(self) -> Bits<64> {
        self.cast::<64>()
    }
}

const impl Lane for Bool {
    const WIDTH: Width = Width::bits(1);
    fn from_low(low: Bits<64>) -> Self {
        low.bit::<0>()
    }
    fn low(self) -> Bits<64> {
        Bits::<64>::masked(0).with_bit::<0>(self)
    }
}

pub trait Field: Copy {
    type Value: const Lane;
}

pub trait Layout {
    type Container: const Lane;
    type Fields: Fields;
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for notko_hlist::Empty {}
    impl<H: super::Field, T: Sealed> Sealed for notko_hlist::Cons<H, T> {}
}

pub trait Fields: sealed::Sealed {
    const SUM: Width;
}

impl Fields for Empty {
    const SUM: Width = Width::bits(0);
}

impl<H: Field, T: Fields> Fields for Cons<H, T> {
    const SUM: Width = <H::Value as Lane>::WIDTH.add(T::SUM);
}

pub trait Below<P: Place>: Fields {
    const BELOW: Width;
}

impl<H: Field, T: Fields> Below<Here> for Cons<H, T> {
    const BELOW: Width = Width::bits(0);
}

impl<H: Field, P: Place, T: Below<P>> Below<There<P>> for Cons<H, T> {
    const BELOW: Width = <H::Value as Lane>::WIDTH.add(T::BELOW);
}

const fn low_bits(width: Width) -> u64 {
    u64::MAX >> (64 - width.count())
}

#[repr(transparent)]
pub struct Word<L: Layout>(L::Container, PhantomData<L>);

impl<L: Layout> Clone for Word<L> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<L: Layout> Copy for Word<L> {}

impl<L: Layout> Word<L> {
    const TILED: () = assert!(
        <L::Fields as Fields>::SUM.equals(<L::Container as Lane>::WIDTH).get(),
        "the field widths of a packed word do not sum to the width of its container"
    );

    pub const fn of(bits: L::Container) -> Self {
        let () = Self::TILED;
        Self(bits, PhantomData)
    }

    pub const fn bits(self) -> L::Container {
        self.0
    }

    pub const fn get<F: Field, P: Place>(self, _: F) -> F::Value
    where
        L::Fields: PlaceOf<F, P> + Below<P>,
    {
        let () = Self::TILED;
        let below = <L::Fields as Below<P>>::BELOW.count();
        let down = (self.0.low().raw() >> below) & low_bits(<F::Value as Lane>::WIDTH);
        F::Value::from_low(Bits::<64>::masked(down))
    }

    pub const fn with<F: Field, P: Place>(self, _: F, to: F::Value) -> Self
    where
        L::Fields: PlaceOf<F, P> + Below<P>,
    {
        let () = Self::TILED;
        let below = <L::Fields as Below<P>>::BELOW.count();
        let field = low_bits(<F::Value as Lane>::WIDTH) << below;
        let put = (to.low().raw() << below) & field;
        let kept = self.0.low().raw() & !field;
        Self(L::Container::from_low(Bits::<64>::masked(kept | put)), PhantomData)
    }
}
