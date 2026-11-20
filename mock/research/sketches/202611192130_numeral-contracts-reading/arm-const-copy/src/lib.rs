//! Arm three: the contracts are const traits, always, and there are no plain
//! ones. `const_trait_impl` is on in this crate unconditionally.

#![no_std]
#![feature(const_trait_impl)]

use core::cmp::Ordering;

use arvo_numeral::{Additive, DebugCheck, Identity, Num, Sole};

/// The sum under the instance's own completion.
pub const trait Sum: Sized {
    /// The sum.
    fn sum(self, rhs: Self) -> Self;
}

/// The total order of the members.
pub const trait Order: Sized {
    /// The order of the two members.
    fn order(self, rhs: Self) -> Ordering;
}

/// A member that adds to every member without moving it.
pub trait Zero: Sized {
    /// The additive identity.
    const ZERO: Self;
}

const impl<S: Sole, C: DebugCheck> Sum for Num<S, C> {
    fn sum(self, rhs: Self) -> Self {
        self.add(rhs)
    }
}

const impl<S: Sole, C: DebugCheck> Order for Num<S, C> {
    fn order(self, rhs: Self) -> Ordering {
        self.cmp(rhs)
    }
}

impl<S: Sole, C: DebugCheck> Zero for Num<S, C> {
    const ZERO: Self = <Self as Identity<Additive>>::IDENTITY;
}

/// A sum computed at build time through the contract, which only this arm and
/// arm two under its feature can write. Read by the arm's own test.
pub const fn sum_in_const<T: [const] Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}
