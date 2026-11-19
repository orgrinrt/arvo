//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The contracts held against `Num`.
//!
//! Every method agrees with the format's own function exhaustively at widths 1 to 8, signed and
//! unsigned, under wrap and saturation. An algebraic law is asserted only inside the region a law
//! row states for it: `law::additive_associativity_under_wrapping`,
//! `law::additive_associativity_under_saturation`, `law::multiplicative_associativity_under_wrapping`,
//! `law::multiplicative_associativity_under_saturation` and
//! `law::existence_of_a_translation_invariant_total_order`. What lies outside a row's region is not
//! asserted here.

mod common;
mod contract_laws;

use arvo_contracts::{Constant, Product, Sum};
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{
    Adapt,
    Dither,
    Slot,
    add,
    difference_position,
    mul,
    panic_on_overflow,
    product_position,
    sub,
    sum_position,
};
use arvo_numeral::{Abort, Fix, Int, Num, Quiet, Sole, UFix, Uint};
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

impl<S: Sole> Members for Num<S, Quiet> {
    fn members() -> impl Iterator<Item = Self> + Clone {
        let (lo, hi) = common::range::<S>();
        (lo ..= hi).map(common::value::<S>)
    }
}

impl<S: Sole> Oracle for Num<S, Quiet> {
    type Coordinate = Slot;

    fn coordinate(self) -> Slot {
        self.slot()
    }

    fn oracle_sum(a: Self, b: Self) -> Slot {
        add::<S>(a.slot(), b.slot(), Dither::UNUSED)
    }

    fn oracle_difference(a: Self, b: Self) -> Slot {
        sub::<S>(a.slot(), b.slot(), Dither::UNUSED)
    }

    fn oracle_product(a: Self, b: Self) -> Slot {
        mul::<S>(a.slot(), b.slot(), Dither::UNUSED)
    }

    fn oracle_fits_sum(a: Self, b: Self) -> bool {
        !panic_on_overflow::<S>(
            sum_position::<S::Format>(a.slot(), b.slot()),
            Dither::UNUSED,
        )
        .get()
    }

    fn oracle_fits_difference(a: Self, b: Self) -> bool {
        !panic_on_overflow::<S>(
            difference_position::<S::Format>(a.slot(), b.slot()),
            Dither::UNUSED,
        )
        .get()
    }

    fn oracle_fits_product(a: Self, b: Self) -> bool {
        !panic_on_overflow::<S>(
            product_position::<S::Format>(a.slot(), b.slot()),
            Dither::UNUSED,
        )
        .get()
    }
}

type W<const N: u32> = Uint<N, Adapt<TowardZero, Wrap>, Quiet>;
type SW<const N: u32> = Int<N, Adapt<TowardZero, Wrap>, Quiet>;
type Sat<const N: u32> = Uint<N, Adapt<TowardZero, Saturate>, Quiet>;
type SSat<const N: u32> = Int<N, Adapt<TowardZero, Saturate>, Quiet>;

/// Every law that holds of every member, at one signature.
macro_rules! everywhere {
    ($t:ty) => {{
        assert_eq!(sum_is_the_oracle::<$t>(), None, "sum, {}", stringify!($t));
        assert_eq!(
            difference_is_the_oracle::<$t>(),
            None,
            "difference, {}",
            stringify!($t)
        );
        assert_eq!(
            product_is_the_oracle::<$t>(),
            None,
            "product, {}",
            stringify!($t)
        );
        assert_eq!(
            zero_is_the_additive_identity::<$t>(),
            None,
            "zero, {}",
            stringify!($t)
        );
        assert_eq!(
            the_order_is_total::<$t>(),
            None,
            "order, {}",
            stringify!($t)
        );
        assert_eq!(
            the_sign_is_the_order_against_zero::<$t>(),
            None,
            "sign, {}",
            stringify!($t)
        );
        assert_eq!(
            the_magnitude_is_not_negative::<$t>(),
            None,
            "magnitude, {}",
            stringify!($t)
        );
    }};
}

macro_rules! at_widths {
    ($m:ident, $f:ident; $($n:literal)*) => { $( $m!($f<$n>); )* };
}

#[test]
fn every_method_is_the_formats_own_at_widths_1_to_8() {
    at_widths!(everywhere, W; 1 2 3 4 5 6 7 8);
    at_widths!(everywhere, SW; 1 2 3 4 5 6 7 8);
    at_widths!(everywhere, Sat; 1 2 3 4 5 6 7 8);
    at_widths!(everywhere, SSat; 1 2 3 4 5 6 7 8);
    everywhere!(Fix<4, -1, Adapt<TowardZero, Wrap>, Quiet>);
    everywhere!(UFix<4, -2, Adapt<TowardZero, Wrap>, Quiet>);
}

macro_rules! one {
    ($t:ty) => {
        assert_eq!(
            one_is_the_multiplicative_identity::<$t>(),
            None,
            "one, {}",
            stringify!($t)
        );
    };
}

/// Every format whose range reaches one: every unsigned width, and every signed width from two.
#[test]
fn one_is_the_multiplicative_identity_wherever_the_range_reaches_it() {
    at_widths!(one, W; 1 2 3 4 5 6 7 8);
    at_widths!(one, Sat; 1 2 3 4 5 6 7 8);
    at_widths!(one, SW; 2 3 4 5 6 7 8);
    at_widths!(one, SSat; 2 3 4 5 6 7 8);
}

macro_rules! holds {
    ($law:ident, $t:ty) => {
        assert_eq!(
            $law::<$t>(),
            None,
            "{} holds at {}",
            stringify!($law),
            stringify!($t)
        );
    };
}

macro_rules! fails {
    ($law:ident, $t:ty) => {
        assert!(
            $law::<$t>().is_some(),
            "{} fails at {}",
            stringify!($law),
            stringify!($t)
        );
    };
}

/// `law::additive_associativity_under_wrapping`: holds at `W in {4, 8}`, either signedness.
#[test]
fn the_wrapping_sum_is_associative_inside_its_region() {
    holds!(sum_is_associative, W<4>);
    holds!(sum_is_associative, W<8>);
    holds!(sum_is_associative, SW<4>);
    holds!(sum_is_associative, SW<8>);
    holds!(sum_is_associative, Fix<4, -1, Adapt<TowardZero, Wrap>, Quiet>);
    holds!(sum_is_associative, UFix<4, -2, Adapt<TowardZero, Wrap>, Quiet>);
}

/// `law::additive_associativity_under_saturation`: holds unsigned at `W in {4, 8}`, fails signed
/// at `W = 4`.
#[test]
fn the_saturating_sum_is_associative_unsigned_and_not_signed() {
    holds!(sum_is_associative, Sat<4>);
    holds!(sum_is_associative, Sat<8>);
    fails!(sum_is_associative, SSat<4>);
}

/// `law::multiplicative_associativity_under_wrapping`: holds at `F = 0`, `W in 3..=8`, and fails
/// at the `F > 0` cells measured to fail; the row's region is wider than what was measured, ticket
/// `c7dfe82db2d5`.
#[test]
fn the_wrapping_product_is_associative_at_integer_points_only() {
    holds!(product_is_associative, W<3>);
    holds!(product_is_associative, W<4>);
    holds!(product_is_associative, W<5>);
    holds!(product_is_associative, W<6>);
    holds!(product_is_associative, W<7>);
    holds!(product_is_associative, W<8>);
    holds!(product_is_associative, SW<3>);
    holds!(product_is_associative, SW<4>);
    holds!(product_is_associative, SW<5>);
    holds!(product_is_associative, SW<6>);
    holds!(product_is_associative, SW<7>);
    holds!(product_is_associative, SW<8>);
    fails!(product_is_associative, UFix<4, -1, Adapt<TowardZero, Wrap>, Quiet>);
    fails!(product_is_associative, UFix<4, -2, Adapt<TowardZero, Wrap>, Quiet>);
    fails!(product_is_associative, Fix<4, -1, Adapt<TowardZero, Wrap>, Quiet>);
    fails!(product_is_associative, Fix<4, -2, Adapt<TowardZero, Wrap>, Quiet>);
}

/// `law::multiplicative_associativity_under_saturation`: holds unsigned at `F = 0`,
/// `W in 3..=8`, fails signed at `F = 0`, `W in 3..=6`.
#[test]
fn the_saturating_product_is_associative_unsigned_and_not_signed() {
    holds!(product_is_associative, Sat<3>);
    holds!(product_is_associative, Sat<4>);
    holds!(product_is_associative, Sat<5>);
    holds!(product_is_associative, Sat<6>);
    holds!(product_is_associative, Sat<7>);
    holds!(product_is_associative, Sat<8>);
    fails!(product_is_associative, SSat<3>);
    fails!(product_is_associative, SSat<4>);
    fails!(product_is_associative, SSat<5>);
    fails!(product_is_associative, SSat<6>);
}

macro_rules! fails_monotone {
    ($t:ty) => {
        fails!(the_sum_keeps_the_order, $t);
    };
}

/// `law::existence_of_a_translation_invariant_total_order`: under wrap the sum keeps no total order
/// at any width, either signedness.
#[test]
fn the_wrapping_sum_keeps_no_order() {
    at_widths!(fails_monotone, W; 1 2 3 4 5 6 7 8);
    at_widths!(fails_monotone, SW; 1 2 3 4 5 6 7 8);
}

/// Under the `Abort` check a debug build aborts through the contract as through the method, and a
/// sum that fits returns.
#[test]
fn the_abort_check_aborts_through_the_contract() {
    type A = Int<8, Adapt<TowardZero, Saturate>, Abort>;
    let a = A::constant::<100>();
    assert_eq!(a.sum(A::constant::<20>()), A::constant::<120>());
    let left = std::panic::catch_unwind(|| a.sum(a));
    assert_eq!(left.is_err(), cfg!(debug_assertions));
    let product = std::panic::catch_unwind(|| a.product(a));
    assert_eq!(product.is_err(), cfg!(debug_assertions));
}

/// The member at a constant slot is the member the value's own door gives.
#[test]
fn the_constant_is_the_member_at_its_slot() {
    assert_eq!(
        <SW<8> as Constant<-128>>::MEMBER,
        SW::<8>::constant::<-128>()
    );
    assert_eq!(<W<8> as Constant<255>>::MEMBER.slot().index(), 255);
}
