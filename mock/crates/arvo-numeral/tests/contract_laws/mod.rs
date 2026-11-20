//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws of the contracts, generic over an instance, each returning the first counterexample
//! rather than asserting, so the same law runs against `Num`, where it must hold inside its
//! region, and against a wrong instance, where it must fail.
//!
//! A contract exposes no slot and no format, so each instance supplies a test-side vocabulary:
//! `Members` enumerates its members, and `Oracle` gives the coordinate of a member and what the
//! instance's own definition says each operation returns.

#![allow(dead_code)]

use core::cmp::Ordering;
use core::fmt::Debug;

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

/// Every member of an instance, in its order.
pub trait Members: Copy + Eq + Debug {
    /// The members.
    fn members() -> impl Iterator<Item = Self> + Clone;
}

/// What the instance's own definition says, in coordinates the contract does not expose.
pub trait Oracle: Members {
    /// A coordinate naming one member.
    type Coordinate: Eq + Debug;

    /// The member's coordinate.
    fn coordinate(self) -> Self::Coordinate;

    /// The coordinate the definition gives for the sum.
    fn oracle_sum(a: Self, b: Self) -> Self::Coordinate;

    /// The coordinate the definition gives for the difference.
    fn oracle_difference(a: Self, b: Self) -> Self::Coordinate;

    /// The coordinate the definition gives for the product.
    fn oracle_product(a: Self, b: Self) -> Self::Coordinate;

    /// Whether the definition's exact sum is a member.
    fn oracle_fits_sum(a: Self, b: Self) -> bool;

    /// Whether the definition's exact difference is a member.
    fn oracle_fits_difference(a: Self, b: Self) -> bool;

    /// Whether the definition's exact product is a member.
    fn oracle_fits_product(a: Self, b: Self) -> bool;
}

/// A law's first counterexample, or none where it held over every member.
pub type Verdict<T> = Option<T>;

fn pairs<T: Members>() -> impl Iterator<Item = (T, T)> {
    T::members().flat_map(|a| T::members().map(move |b| (a, b)))
}

fn triples<T: Members>() -> impl Iterator<Item = (T, T, T)> {
    pairs::<T>().flat_map(|(a, b)| T::members().map(move |c| (a, b, c)))
}

/// The sum through the contract is the definition's sum, and `fits_sum` its exactness.
pub fn sum_is_the_oracle<T: Sum + Oracle>() -> Verdict<(T, T)> {
    pairs::<T>().find(|&(a, b)| {
        a.sum(b).coordinate() != T::oracle_sum(a, b)
            || a.fits_sum(b).get() != T::oracle_fits_sum(a, b)
    })
}

/// The difference through the contract is the definition's difference.
pub fn difference_is_the_oracle<T: Difference + Oracle>() -> Verdict<(T, T)> {
    pairs::<T>().find(|&(a, b)| {
        a.difference(b).coordinate() != T::oracle_difference(a, b)
            || a.fits_difference(b).get() != T::oracle_fits_difference(a, b)
    })
}

/// The product through the contract is the definition's product.
pub fn product_is_the_oracle<T: Product + Oracle>() -> Verdict<(T, T)> {
    pairs::<T>().find(|&(a, b)| {
        a.product(b).coordinate() != T::oracle_product(a, b)
            || a.fits_product(b).get() != T::oracle_fits_product(a, b)
    })
}

/// `(a + b) + c == a + (b + c)`.
pub fn sum_is_associative<T: Sum + Members>() -> Verdict<(T, T, T)> {
    triples::<T>().find(|&(a, b, c)| a.sum(b).sum(c) != a.sum(b.sum(c)))
}

/// `(a * b) * c == a * (b * c)`.
pub fn product_is_associative<T: Product + Members>() -> Verdict<(T, T, T)> {
    triples::<T>().find(|&(a, b, c)| a.product(b).product(c) != a.product(b.product(c)))
}

/// `a + 0 == a == 0 + a`.
pub fn zero_is_the_additive_identity<T: Sum + Identity<Additive> + Members>() -> Verdict<T> {
    let zero = <T as Identity<Additive>>::IDENTITY;
    T::members().find(|&a| a.sum(zero) != a || zero.sum(a) != a)
}

/// `a * 1 == a == 1 * a`.
pub fn one_is_the_multiplicative_identity<T: Product + Identity<Multiplicative> + Members>()
-> Verdict<T> {
    let one = <T as Identity<Multiplicative>>::IDENTITY;
    T::members().find(|&a| a.product(one) != a || one.product(a) != a)
}

/// A total order: equal exactly on one member, antisymmetric, transitive, and the provided
/// comparisons, `min` and `max` read off it.
pub fn the_order_is_total<T: Order + Members>() -> Verdict<(T, T, T)> {
    let pair_wrong = |a: T, b: T| {
        let o = a.order(b);
        (o == Ordering::Equal) != (a == b)
            || o != b.order(a).reverse()
            || a.lt(b).get() != (o == Ordering::Less)
            || a.le(b).get() != (o != Ordering::Greater)
            || a.gt(b).get() != (o == Ordering::Greater)
            || a.ge(b).get() != (o != Ordering::Less)
            || a.min(b) != if o == Ordering::Greater { b } else { a }
            || a.max(b) != if o == Ordering::Less { b } else { a }
    };
    triples::<T>()
        .find(|&(a, b, c)| pair_wrong(a, b) || (a.lt(b).get() && b.lt(c).get() && !a.lt(c).get()))
}

/// `a <= b` gives `a + c <= b + c`.
pub fn the_sum_keeps_the_order<T: Sum + Order + Members>() -> Verdict<(T, T, T)> {
    triples::<T>().find(|&(a, b, c)| a.le(b).get() && !a.sum(c).le(b.sum(c)).get())
}

/// The sign is the order against zero.
pub fn the_sign_is_the_order_against_zero<T: Sign + Order + Identity<Additive> + Members>()
-> Verdict<T> {
    let zero = <T as Identity<Additive>>::IDENTITY;
    T::members().find(|&a| {
        a.is_negative().get() != a.lt(zero).get()
            || a.is_zero().get() != (a == zero)
            || a.is_positive().get() != a.gt(zero).get()
    })
}

/// The magnitude of a member not below zero is the member, and of one below zero is not below
/// zero, except the least member, whose magnitude is itself where the completion wraps.
pub fn the_magnitude_is_not_negative<T: Magnitude + Sign + Order + Members>() -> Verdict<T> {
    let least = T::members().next();
    T::members().find(|&a| {
        let m = a.magnitude();
        if !a.is_negative().get() {
            m != a
        } else {
            m.is_negative().get() && !(Some(a) == least && m == a)
        }
    })
}
