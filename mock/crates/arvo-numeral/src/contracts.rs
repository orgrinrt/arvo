//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The value as an instance of every numeric contract.
//!
//! Each method is the inherent `const fn` of the same operation, so its law is the equality the
//! method already stands on, against the format's own function over slots, and a contract adds no
//! behaviour. Under `Abort` a result leaving the range aborts a debug build through a contract as
//! through the method.

use core::cmp::Ordering;

use arvo_contracts::{
    Additive,
    Constant,
    Difference,
    Identity,
    Magnitude,
    MonotoneCompletion,
    MonotoneSum,
    Order,
    Product,
    Sign,
    Sum,
};
use arvo_format::{Adaptation, Bool, DeclaredSignature};

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::num::Num;

const impl<S: Sole, C: DebugCheck> Sum for Num<S, C> {
    fn sum(self, rhs: Self) -> Self {
        self.add(rhs)
    }

    fn fits_sum(self, rhs: Self) -> Bool {
        self.fits_add(rhs)
    }
}

const impl<S: Sole, C: DebugCheck> Difference for Num<S, C> {
    fn difference(self, rhs: Self) -> Self {
        self.sub(rhs)
    }

    fn fits_difference(self, rhs: Self) -> Bool {
        self.fits_sub(rhs)
    }
}

/// Refused at build where the format's product is not admitted, as the inherent product is.
const impl<S: Sole, C: DebugCheck> Product for Num<S, C> {
    fn product(self, rhs: Self) -> Self {
        self.mul(rhs)
    }

    fn fits_product(self, rhs: Self) -> Bool {
        self.fits_mul(rhs)
    }
}

const impl<S: Sole, C: DebugCheck> Order for Num<S, C> {
    fn order(self, rhs: Self) -> Ordering {
        self.cmp(rhs)
    }
}

/// The order against the additive identity, so a format without a zero is refused at build with
/// the identity's own message.
const impl<S: Sole, C: DebugCheck> Sign for Num<S, C> {
    fn is_negative(self) -> Bool {
        self.lt(<Self as Identity<Additive>>::IDENTITY)
    }

    fn is_zero(self) -> Bool {
        self.eq(<Self as Identity<Additive>>::IDENTITY)
    }

    fn is_positive(self) -> Bool {
        self.gt(<Self as Identity<Additive>>::IDENTITY)
    }
}

/// The negation where the member is below zero: under wrap the least member of a signed format
/// negates to itself, under saturation to the greatest.
const impl<S: Sole, C: DebugCheck> Magnitude for Num<S, C> {
    fn magnitude(self) -> Self {
        if self.lt(<Self as Identity<Additive>>::IDENTITY).get() {
            self.neg()
        } else {
            self
        }
    }
}

impl<S: Sole, C: DebugCheck, const V: i128> Constant<V> for Num<S, C> {
    const MEMBER: Self = Num::constant::<V>();
}

impl<S: Sole, C: DebugCheck> MonotoneSum for Num<S, C> where
    <<S as DeclaredSignature>::Adaptation as Adaptation>::Overflow: MonotoneCompletion
{
}
