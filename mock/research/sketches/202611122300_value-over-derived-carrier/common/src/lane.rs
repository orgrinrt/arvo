//! The `Lane` trait as the name page writes it, and the carrier operations an
//! ordinary (not const) generic body reaches through.

use core::fmt::Debug;

use arvo_format::{Policy, Width};

use crate::rung;

/// A number a kernel can add up exactly: the trait kernels generic over a number
/// are bounded by.
///
/// The name page's text: kaski's item names less `wide_div` and `Layer`, methods
/// not const.
pub trait Lane: Copy + Send + Sync + 'static {
    /// The accumulator.
    type Wide: Copy + PartialEq + Debug + Send + Sync + 'static;
    /// The empty sum.
    const WIDE_ZERO: Self::Wide;
    /// The value as an accumulator.
    fn widen(self) -> Self::Wide;
    /// The exact sum of two accumulators.
    fn wide_add(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    /// The exact difference of two accumulators.
    fn wide_sub(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    /// The one adaptation of an accumulated sum.
    fn narrow(wide: Self::Wide) -> Self;
}

/// A ladder carrier's operations, as an ordinary trait.
///
/// Not const, so it serves only a body that is not: the `Lane` impls of the arms
/// and the tests. The question the arms answer is how a `const fn` reaches the
/// same operations.
///
/// Sealed: implemented for the eight ladder integers and nothing else. Arm B's
/// bit copy is sound only while every carrier is one of them, so the closed set
/// is part of its contract rather than a convenience.
pub trait Machine: sealed::Sealed + Copy {
    /// The double-width accumulator.
    type Wide: Copy + Eq + Debug + Send + Sync + 'static;
    /// Zero in the accumulator.
    const WIDE_ZERO: Self::Wide;
    /// The carrier's width in bits.
    const BITS: u32;
    /// Whether the carrier is the signed one of its rung.
    const SIGNED: bool;
    /// The value at the slot index, which must fit the carrier.
    fn from_index(i: i128) -> Self;
    /// The slot index of the value.
    fn index(self) -> i128;
    /// `rung::add_*`.
    fn add(a: Self, b: Self, width: Width, policy: Policy) -> Self;
    /// `rung::adapt_*`.
    fn adapt(x: Self, width: Width, policy: Policy) -> Self;
    /// Into the accumulator.
    fn widen(self) -> Self::Wide;
    /// Exact. Panics on leaving the accumulator in a build with overflow checks.
    fn wide_add(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    /// Exact. Panics on leaving the accumulator in a build with overflow checks.
    fn wide_sub(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    /// `rung::narrow_*`.
    fn narrow(wide: Self::Wide, width: Width, policy: Policy) -> Self;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! machine {
    ($($t:ty, $w:ty, $bits:literal, $signed:literal, $add:path, $adapt:path, $narrow:path;)+) => {$(
        impl sealed::Sealed for $t {}
        impl Machine for $t {
            type Wide = $w;
            const WIDE_ZERO: $w = 0;
            const BITS: u32 = $bits;
            const SIGNED: bool = $signed;
            fn from_index(i: i128) -> Self {
                i as $t
            }
            fn index(self) -> i128 {
                self as i128
            }
            fn add(a: Self, b: Self, width: Width, policy: Policy) -> Self {
                $add(a, b, width, policy)
            }
            fn adapt(x: Self, width: Width, policy: Policy) -> Self {
                $adapt(x, width, policy)
            }
            fn widen(self) -> $w {
                self as $w
            }
            fn wide_add(a: $w, b: $w) -> $w {
                a + b
            }
            fn wide_sub(a: $w, b: $w) -> $w {
                a - b
            }
            fn narrow(wide: $w, width: Width, policy: Policy) -> Self {
                $narrow(wide, width, policy)
            }
        }
    )+};
}

machine! {
    i8, i16, 8, true, rung::add_s8, rung::adapt_s8, rung::narrow_s8;
    u8, i16, 8, false, rung::add_u8, rung::adapt_u8, rung::narrow_u8;
    i16, i32, 16, true, rung::add_s16, rung::adapt_s16, rung::narrow_s16;
    u16, i32, 16, false, rung::add_u16, rung::adapt_u16, rung::narrow_u16;
    i32, i64, 32, true, rung::add_s32, rung::adapt_s32, rung::narrow_s32;
    u32, i64, 32, false, rung::add_u32, rung::adapt_u32, rung::narrow_u32;
    i64, i128, 64, true, rung::add_s64, rung::adapt_s64, rung::narrow_s64;
    u64, i128, 64, false, rung::add_u64, rung::adapt_u64, rung::narrow_u64;
}
