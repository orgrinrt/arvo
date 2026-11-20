//! A const trait with a const-generic index, implemented by delegating to an inherent
//! `const fn` that forces a private assertion on the index.
#![no_std]
#![feature(const_trait_impl)]

use arvo_format::{Bool, Width};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bits<const N: u32>(u64);

struct Within<const N: u32, const I: u32>;
impl<const N: u32, const I: u32> Within<N, I> {
    const ADMITTED: () = assert!(I < N, "a bit index at or past the width");
}

impl<const N: u32> Bits<N> {
    pub const fn masked(raw: u64) -> Self {
        Self(if N == 64 { raw } else { raw & ((1u64 << N) - 1) })
    }

    pub const fn bit<const I: u32>(self) -> Bool {
        let () = Within::<N, I>::ADMITTED;
        Bool::of((self.0 >> I) & 1 == 1)
    }

    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn is_empty(self) -> Bool {
        Bool::of(self.0 == 0)
    }
}

pub const trait BitWidth {
    const WIDTH: Width;
}
pub const trait BitAccess: BitWidth + Copy {
    fn bit<const I: u32>(self) -> Bool;
}
pub const trait BitLogic: Copy {
    fn and(self, other: Self) -> Self;
    fn is_empty(self) -> Bool;
}

const impl<const N: u32> BitWidth for Bits<N> {
    const WIDTH: Width = Width::bits(N);
}
const impl<const N: u32> BitAccess for Bits<N> {
    fn bit<const I: u32>(self) -> Bool {
        Bits::<N>::bit::<I>(self)
    }
}
const impl<const N: u32> BitLogic for Bits<N> {
    fn and(self, other: Self) -> Self {
        Bits::<N>::and(self, other)
    }

    fn is_empty(self) -> Bool {
        Bits::<N>::is_empty(self)
    }
}

/// Generic over the bound, in a const context.
pub const fn first_bit<B: [const] BitAccess>(b: B) -> Bool {
    b.bit::<0>()
}
pub const fn disjoint<B: [const] BitLogic>(a: B, b: B) -> Bool {
    a.and(b).is_empty()
}

pub const OK_INHERENT: Bool = Bits::<8>::masked(3).bit::<7>();
pub const OK_GENERIC: Bool = first_bit(Bits::<8>::masked(3));
pub const OK_DISJOINT: Bool = disjoint(Bits::<8>::masked(1), Bits::<8>::masked(2));
pub const W: Width = <Bits<8> as BitWidth>::WIDTH;
