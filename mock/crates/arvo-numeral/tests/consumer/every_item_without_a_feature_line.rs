//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A consumer crate with no feature line, reaching every public item of `arvo-contracts`: a generic
//! function over the contracts run against `Num`, and an outside instance that uses no `Num`.

use core::cmp::Ordering;

use arvo_contracts::{
    Additive,
    Constant,
    Difference,
    Identity,
    Magnitude,
    MonotoneCompletion,
    MonotoneSum,
    Multiplicative,
    Order,
    Product,
    Reciprocal,
    Sign,
    SquareRoot,
    Sum,
};
use arvo_format::overflow::{Overflow, Policy, Saturate};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Bool};
use arvo_numeral::{Int, Quiet};

/// The highest running sum, with every contract that has a method in play.
fn best_running_sum<T>(xs: [T; 4]) -> T
where
    T: Sum + Difference + Product + Order + Sign + Magnitude + MonotoneSum,
    T: Identity<Additive> + Identity<Multiplicative> + Constant<1>,
{
    let zero = <T as Identity<Additive>>::IDENTITY;
    let one = <T as Identity<Multiplicative>>::IDENTITY;
    assert!(one.eq_member(<T as Constant<1>>::MEMBER));
    let mut running = zero;
    let mut best = zero;
    for x in xs {
        let x = x.magnitude().product(one).difference(zero);
        assert!(
            x.fits_sum(zero).get() && x.fits_difference(zero).get() && x.fits_product(one).get()
        );
        assert!(!x.is_negative().get() && (x.is_zero().get() || x.is_positive().get()));
        running = running.sum(x);
        best = best.max(running);
    }
    assert!(
        best.ge(zero).get() && zero.le(best).get() && !best.lt(zero).get() && !zero.gt(best).get()
    );
    assert_eq!(best.min(zero).order(zero), Ordering::Equal);
    best
}

/// Equality read through the order, so the bound needs nothing beyond the contracts.
trait EqMember: Order {
    fn eq_member(self, rhs: Self) -> bool {
        matches!(self.order(rhs), Ordering::Equal)
    }
}

impl<T: Order> EqMember for T {}

/// A saturating eight-bit tally of this crate's own, using no `Num`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Tally(u8);

impl Sum for Tally {
    fn sum(self, rhs: Self) -> Self {
        Tally(self.0.saturating_add(rhs.0))
    }

    fn fits_sum(self, rhs: Self) -> Bool {
        Bool::of(self.0.checked_add(rhs.0).is_some())
    }
}

impl Difference for Tally {
    fn difference(self, rhs: Self) -> Self {
        Tally(self.0.saturating_sub(rhs.0))
    }

    fn fits_difference(self, rhs: Self) -> Bool {
        Bool::of(self.0 >= rhs.0)
    }
}

impl Product for Tally {
    fn product(self, rhs: Self) -> Self {
        Tally(self.0.saturating_mul(rhs.0))
    }

    fn fits_product(self, rhs: Self) -> Bool {
        Bool::of(self.0.checked_mul(rhs.0).is_some())
    }
}

impl Order for Tally {
    fn order(self, rhs: Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl Sign for Tally {
    fn is_negative(self) -> Bool {
        Bool::FALSE
    }

    fn is_zero(self) -> Bool {
        Bool::of(self.0 == 0)
    }

    fn is_positive(self) -> Bool {
        Bool::of(self.0 > 0)
    }
}

impl Magnitude for Tally {
    fn magnitude(self) -> Self {
        self
    }
}

impl Identity<Additive> for Tally {
    const IDENTITY: Self = Tally(0);
}

impl Identity<Multiplicative> for Tally {
    const IDENTITY: Self = Tally(1);
}

impl Constant<1> for Tally {
    const MEMBER: Self = Tally(1);
}

impl MonotoneSum for Tally {}

/// A tally other than zero.
#[derive(Clone, Copy)]
struct NonzeroTally(Tally);

impl Reciprocal for Tally {
    type Nonzero = NonzeroTally;

    fn reciprocal(operand: NonzeroTally) -> Self {
        Tally(u8::from(operand.0.0 == 1))
    }
}

impl SquareRoot for Tally {
    fn square_root(self) -> Self {
        Tally(self.0.isqrt())
    }
}

/// A saturating policy of this crate's own, which carries the monotone marker from here.
struct Pinned;

impl Overflow for Pinned {
    const POLICY: Policy = Policy::Saturate;
}

impl MonotoneCompletion for Pinned {}

type I8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;
type P8 = Int<8, Adapt<TowardZero, Pinned>, Quiet>;

fn main() {
    let xs = [
        I8::constant::<3>(),
        I8::constant::<-5>(),
        I8::constant::<9>(),
        I8::constant::<-1>(),
    ];
    // Magnitudes 3, 5, 9, 1, running 3, 8, 17, 18.
    assert_eq!(best_running_sum(xs), I8::constant::<18>());
    let ps = [
        P8::constant::<100>(),
        P8::constant::<100>(),
        P8::constant::<1>(),
        P8::constant::<0>(),
    ];
    assert_eq!(best_running_sum(ps), P8::constant::<127>());
    assert_eq!(
        best_running_sum([Tally(200), Tally(50), Tally(30), Tally(0)]),
        Tally(255)
    );
    assert_eq!(Tally::reciprocal(NonzeroTally(Tally(1))), Tally(1));
    assert_eq!(Tally(9).square_root(), Tally(3));
}
