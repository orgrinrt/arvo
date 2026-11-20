//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Each law of the contracts, run against an outside instance that is wrong in that law and must
//! fail it, beside two right ones that must pass.
//!
//! The instance is a four-bit unsigned value of this file's own, using no `Num`. `M` selects how it
//! behaves: `WRAP` and `SATURATE` are right, every other value breaks one law. A law that passed
//! its wrong instance would be asserting nothing.

mod contract_laws;

use core::cmp::Ordering;

use arvo_contracts::{
    Additive,
    Difference,
    Identity,
    Magnitude,
    Multiplicative,
    Order,
    Product,
    Sign,
    Sum,
};
use arvo_format::Bool;
use contract_laws::{
    Members,
    Oracle,
    difference_is_the_oracle,
    one_is_the_multiplicative_identity,
    product_is_associative,
    product_is_the_oracle,
    sum_is_associative,
    sum_is_the_oracle,
    the_magnitude_is_not_negative,
    the_order_is_total,
    the_sign_is_the_order_against_zero,
    the_sum_keeps_the_order,
    zero_is_the_additive_identity,
};

const WRAP: u8 = 0;
const SATURATE: u8 = 1;
const SUM_ASSOCIATIVE: u8 = 2;
const PRODUCT_ASSOCIATIVE: u8 = 3;
const ZERO: u8 = 4;
const ONE: u8 = 5;
const ORDER: u8 = 6;
const MONOTONE: u8 = 7;
const SIGN: u8 = 8;
const MAGNITUDE: u8 = 9;
const ORACLE: u8 = 10;
const DIFFERENCE_ORACLE: u8 = 11;
const PRODUCT_ORACLE: u8 = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Nib<const M: u8>(u8);

const TOP: u8 = 15;

/// The completion of an exact difference, which can fall below zero.
const fn settle<const M: u8>(exact: i16) -> u8 {
    if M == SATURATE || M == MONOTONE {
        if exact < 0 { 0 } else { exact as u8 }
    } else {
        exact.rem_euclid(16) as u8
    }
}

/// The completion the instance declares: saturation for `SATURATE` and `MONOTONE`, wrap otherwise.
const fn complete<const M: u8>(exact: u16) -> u8 {
    if M == SATURATE || M == MONOTONE {
        if exact > TOP as u16 { TOP } else { exact as u8 }
    } else {
        (exact % 16) as u8
    }
}

impl<const M: u8> Sum for Nib<M> {
    fn sum(self, rhs: Self) -> Self {
        let (a, b) = (self.0, rhs.0);
        Nib(match M {
            SUM_ASSOCIATIVE if a == b => a,
            MONOTONE if a == 14 && b == 1 => 0,
            ORACLE if a == 7 => complete::<M>(a as u16 + b as u16 + 1),
            _ => complete::<M>(a as u16 + b as u16),
        })
    }

    fn fits_sum(self, rhs: Self) -> Bool {
        Bool::of(self.0 as u16 + rhs.0 as u16 <= TOP as u16)
    }
}

impl<const M: u8> Difference for Nib<M> {
    fn difference(self, rhs: Self) -> Self {
        let exact = self.0 as i16 - rhs.0 as i16;
        let off = if M == DIFFERENCE_ORACLE && self.0 == 9 { 1 } else { 0 };
        Nib(settle::<M>(exact - off))
    }

    fn fits_difference(self, rhs: Self) -> Bool {
        Bool::of(self.0 >= rhs.0)
    }
}

impl<const M: u8> Product for Nib<M> {
    fn product(self, rhs: Self) -> Self {
        let (a, b) = (self.0, rhs.0);
        Nib(match M {
            PRODUCT_ASSOCIATIVE if a == 3 && b == 3 => 0,
            PRODUCT_ORACLE if a == 5 => complete::<M>(a as u16 * b as u16 + 1),
            _ => complete::<M>(a as u16 * b as u16),
        })
    }

    fn fits_product(self, rhs: Self) -> Bool {
        Bool::of(self.0 as u16 * rhs.0 as u16 <= TOP as u16)
    }
}

impl<const M: u8> Order for Nib<M> {
    fn order(self, rhs: Self) -> Ordering {
        match (M, self.0, rhs.0) {
            (ORDER, 3, 5) | (ORDER, 5, 3) => Ordering::Equal,
            (_, a, b) => a.cmp(&b),
        }
    }
}

impl<const M: u8> Identity<Additive> for Nib<M> {
    const IDENTITY: Self = Nib(if M == ZERO { 1 } else { 0 });
}

impl<const M: u8> Identity<Multiplicative> for Nib<M> {
    const IDENTITY: Self = Nib(if M == ONE { 0 } else { 1 });
}

impl<const M: u8> Sign for Nib<M> {
    fn is_negative(self) -> Bool {
        Bool::FALSE
    }

    fn is_zero(self) -> Bool {
        Bool::of(self.0 == 0)
    }

    fn is_positive(self) -> Bool {
        Bool::of(self.0 > 0 || M == SIGN)
    }
}

impl<const M: u8> Magnitude for Nib<M> {
    fn magnitude(self) -> Self {
        if M == MAGNITUDE && self.0 == 3 { Nib(4) } else { self }
    }
}

impl<const M: u8> Members for Nib<M> {
    fn members() -> impl Iterator<Item = Self> + Clone {
        (0 ..= TOP).map(Nib)
    }
}

impl<const M: u8> Oracle for Nib<M> {
    type Coordinate = u8;

    fn coordinate(self) -> u8 {
        self.0
    }

    fn oracle_sum(a: Self, b: Self) -> u8 {
        complete::<M>(a.0 as u16 + b.0 as u16)
    }

    fn oracle_difference(a: Self, b: Self) -> u8 {
        settle::<M>(a.0 as i16 - b.0 as i16)
    }

    fn oracle_product(a: Self, b: Self) -> u8 {
        complete::<M>(a.0 as u16 * b.0 as u16)
    }

    fn oracle_fits_sum(a: Self, b: Self) -> bool {
        a.0 as u16 + b.0 as u16 <= TOP as u16
    }

    fn oracle_fits_difference(a: Self, b: Self) -> bool {
        a.0 >= b.0
    }

    fn oracle_fits_product(a: Self, b: Self) -> bool {
        a.0 as u16 * b.0 as u16 <= TOP as u16
    }
}

macro_rules! passes_every_law {
    ($m:ident) => {
        assert_eq!(sum_is_the_oracle::<Nib<$m>>(), None);
        assert_eq!(difference_is_the_oracle::<Nib<$m>>(), None);
        assert_eq!(product_is_the_oracle::<Nib<$m>>(), None);
        assert_eq!(sum_is_associative::<Nib<$m>>(), None);
        assert_eq!(product_is_associative::<Nib<$m>>(), None);
        assert_eq!(zero_is_the_additive_identity::<Nib<$m>>(), None);
        assert_eq!(one_is_the_multiplicative_identity::<Nib<$m>>(), None);
        assert_eq!(the_order_is_total::<Nib<$m>>(), None);
        assert_eq!(the_sign_is_the_order_against_zero::<Nib<$m>>(), None);
        assert_eq!(the_magnitude_is_not_negative::<Nib<$m>>(), None);
    };
}

#[test]
fn the_right_instances_pass_every_law_they_are_held_to() {
    passes_every_law!(WRAP);
    passes_every_law!(SATURATE);
    assert_eq!(the_sum_keeps_the_order::<Nib<SATURATE>>(), None);
    assert!(the_sum_keeps_the_order::<Nib<WRAP>>().is_some());
}

#[test]
fn each_wrong_instance_fails_its_law() {
    assert!(sum_is_the_oracle::<Nib<ORACLE>>().is_some());
    assert!(difference_is_the_oracle::<Nib<DIFFERENCE_ORACLE>>().is_some());
    assert!(product_is_the_oracle::<Nib<PRODUCT_ORACLE>>().is_some());
    assert!(sum_is_associative::<Nib<SUM_ASSOCIATIVE>>().is_some());
    assert!(product_is_associative::<Nib<PRODUCT_ASSOCIATIVE>>().is_some());
    assert!(zero_is_the_additive_identity::<Nib<ZERO>>().is_some());
    assert!(one_is_the_multiplicative_identity::<Nib<ONE>>().is_some());
    assert!(the_order_is_total::<Nib<ORDER>>().is_some());
    assert!(the_sum_keeps_the_order::<Nib<MONOTONE>>().is_some());
    assert!(the_sign_is_the_order_against_zero::<Nib<SIGN>>().is_some());
    assert!(the_magnitude_is_not_negative::<Nib<MAGNITUDE>>().is_some());
}
