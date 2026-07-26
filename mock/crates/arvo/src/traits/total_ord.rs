//! Total ordering over the numeric family.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use core::cmp::Ordering;

use crate::float::{FastFloat, StrictFloat};
use crate::ifixed::IFixed;
use crate::strategy::{ifixed_bits, ufixed_bits, BitsContainerFor, Signed, Strategy, Unsigned};
use crate::ufixed::UFixed;
pub use arvo_numeric_contracts::TotalOrd;
use arvo_storage::{FBits, IBits};
use arvo_transparent::Transparent;

// --- TotalOrd --------------------------------------------------------------
//
// Bodies use direct `<` / `>` / `==` comparison rather than the inherent
// `cmp` / `total_cmp` methods because those are not const-stable on
// rustc 1.96.0-nightly. The bare-primitive comparison operators ARE
// const-stable for every integer width, and the float bit-reinterpret
// XOR trick (see `total_cmp_f32` / `total_cmp_f64`) gives the same
// total ordering as `f*::total_cmp` while staying const-callable.

/// Const-callable equivalent of `f32::total_cmp`. Reinterprets the float
/// bits as i32, applies the standard XOR mask so positive floats sort
/// after negative floats by bit pattern, then compares as signed.
#[inline(always)]
const fn total_cmp_f32(a: f32, b: f32) -> Ordering {
    let mut left = a.to_bits() as i32;
    let mut right = b.to_bits() as i32;
    left ^= (((left >> 31) as u32) >> 1) as i32;
    right ^= (((right >> 31) as u32) >> 1) as i32;
    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Const-callable equivalent of `f64::total_cmp`. Same XOR-mask trick
/// as `total_cmp_f32`, widened to i64.
#[inline(always)]
const fn total_cmp_f64(a: f64, b: f64) -> Ordering {
    let mut left = a.to_bits() as i64;
    let mut right = b.to_bits() as i64;
    left ^= (((left >> 63) as u64) >> 1) as i64;
    right ^= (((right >> 63) as u64) >> 1) as i64;
    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

const impl<const I: IBits, const F: FBits, S: Strategy> TotalOrd for UFixed<I, F, S>
where
    S: [const] BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>,
    Self: [const] arvo_storage::ConstOrd,
{
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        const_ordering_to_core(<Self as arvo_storage::ConstOrd>::const_cmp(&self, &other))
    }
}

const impl<const I: IBits, const F: FBits, S: Strategy> TotalOrd for IFixed<I, F, S>
where
    S: [const] BitsContainerFor<{ ifixed_bits(I, F) }, Signed>,
    Self: [const] arvo_storage::ConstOrd,
{
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        const_ordering_to_core(<Self as arvo_storage::ConstOrd>::const_cmp(&self, &other))
    }
}

/// Const-callable bridge from substrate `ConstOrdering` to core
/// `Ordering`. Wraps the existing bidirectional `From` impl in a
/// const fn since `From::from` is not yet const-stable as a trait
/// method on rustc 1.96.0-nightly.
#[inline(always)]
const fn const_ordering_to_core(o: arvo_storage::ConstOrdering) -> Ordering {
    match o {
        arvo_storage::ConstOrdering::Less => Ordering::Less,
        arvo_storage::ConstOrdering::Equal => Ordering::Equal,
        arvo_storage::ConstOrdering::Greater => Ordering::Greater,
    }
}

const impl TotalOrd for FastFloat<f32> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f32(
            <Self as Transparent>::raw(self),
            <Self as Transparent>::raw(other),
        )
    }
}

const impl TotalOrd for FastFloat<f64> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f64(
            <Self as Transparent>::raw(self),
            <Self as Transparent>::raw(other),
        )
    }
}

const impl TotalOrd for StrictFloat<f32> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f32(
            <Self as Transparent>::raw(self),
            <Self as Transparent>::raw(other),
        )
    }
}

const impl TotalOrd for StrictFloat<f64> {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        total_cmp_f64(
            <Self as Transparent>::raw(self),
            <Self as Transparent>::raw(other),
        )
    }
}
