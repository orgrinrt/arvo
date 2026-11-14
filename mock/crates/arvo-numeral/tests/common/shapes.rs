//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a sweep over every shape shares: the widths, the formats of a radix
//! family at one width, the adaptations of a format, and the members taken.
//!
//! A shape is a width from 1 to 64, a signed or an unsigned range, an overflow
//! policy, a rounding mode, a radix and an exponent. Every width, both ranges,
//! both policies and the five modes are taken, at radix two through the shipped
//! points, at radix ten, and at radix three, which `arvo-format` names no
//! ambient for. The exponent is an integer with no end, so each operation names
//! a family of five per width, ending at the last exponents its obligation in
//! the format admits.

use arvo_numeral::Sole;

use super::{SplitMix, lattice, range};

/// How many seeded members each signature of a sweep over every shape draws,
/// beside its edge lattice.
pub const SEEDED_MEMBERS: usize = 192;

/// The members a sweep over every shape takes of one signature: every member
/// up to eight bits, and above that the edge lattice with seeded members.
pub fn members<S: Sole>() -> Vec<i128> {
    let (min, max) = range::<S>();
    if max - min < 256 {
        return (min ..= max).collect();
    }
    let mut out = lattice::<S>().as_slice().to_vec();
    let mut rng = SplitMix::seeded();
    out.extend((0 .. SEEDED_MEMBERS).map(|_| rng.member(min, max)));
    out
}

/// What a sweep over every shape found.
#[derive(Default)]
pub struct Shapes {
    /// Signatures the sweep ran.
    pub signatures: u64,
    pub checked:    u64,
    pub wrong:      u64,
    /// Members on which a planted wrong expectation disagreed with the value,
    /// where a sweep plants one.
    pub planted:    u64,
    /// Checks whose expected result left the range and was completed by the
    /// policy, where a sweep counts them.
    pub unfit:      u64,
}

impl Shapes {
    /// Nothing disagreed, and the sweep ran the signatures it names.
    pub fn agree(&self, what: &str, signatures: u64) {
        assert_eq!(
            self.wrong, 0,
            "{what} disagrees with the oracle on {} of {} checks",
            self.wrong, self.checked
        );
        assert_eq!(self.signatures, signatures, "{what}: the signatures swept");
        assert!(self.checked > 0, "{what}: nothing was checked");
    }
}

/// A macro called once at each width of a band, the width first.
#[macro_export]
macro_rules! widths_1_to_16 {
    ($m:ident ! ($($args:tt)*)) => {
        $m!(1, $($args)*); $m!(2, $($args)*); $m!(3, $($args)*); $m!(4, $($args)*);
        $m!(5, $($args)*); $m!(6, $($args)*); $m!(7, $($args)*); $m!(8, $($args)*);
        $m!(9, $($args)*); $m!(10, $($args)*); $m!(11, $($args)*); $m!(12, $($args)*);
        $m!(13, $($args)*); $m!(14, $($args)*); $m!(15, $($args)*); $m!(16, $($args)*);
    };
}

#[macro_export]
macro_rules! widths_17_to_32 {
    ($m:ident ! ($($args:tt)*)) => {
        $m!(17, $($args)*); $m!(18, $($args)*); $m!(19, $($args)*); $m!(20, $($args)*);
        $m!(21, $($args)*); $m!(22, $($args)*); $m!(23, $($args)*); $m!(24, $($args)*);
        $m!(25, $($args)*); $m!(26, $($args)*); $m!(27, $($args)*); $m!(28, $($args)*);
        $m!(29, $($args)*); $m!(30, $($args)*); $m!(31, $($args)*); $m!(32, $($args)*);
    };
}

#[macro_export]
macro_rules! widths_33_to_48 {
    ($m:ident ! ($($args:tt)*)) => {
        $m!(33, $($args)*); $m!(34, $($args)*); $m!(35, $($args)*); $m!(36, $($args)*);
        $m!(37, $($args)*); $m!(38, $($args)*); $m!(39, $($args)*); $m!(40, $($args)*);
        $m!(41, $($args)*); $m!(42, $($args)*); $m!(43, $($args)*); $m!(44, $($args)*);
        $m!(45, $($args)*); $m!(46, $($args)*); $m!(47, $($args)*); $m!(48, $($args)*);
    };
}

#[macro_export]
macro_rules! widths_49_to_64 {
    ($m:ident ! ($($args:tt)*)) => {
        $m!(49, $($args)*); $m!(50, $($args)*); $m!(51, $($args)*); $m!(52, $($args)*);
        $m!(53, $($args)*); $m!(54, $($args)*); $m!(55, $($args)*); $m!(56, $($args)*);
        $m!(57, $($args)*); $m!(58, $($args)*); $m!(59, $($args)*); $m!(60, $($args)*);
        $m!(61, $($args)*); $m!(62, $($args)*); $m!(63, $($args)*); $m!(64, $($args)*);
    };
}

/// One format under both policies and the five modes, for a body that takes
/// the signature as a type and any further arguments after it.
#[macro_export]
macro_rules! every_adaptation {
    ($format:ty, $body:ident ( $($args:expr),* )) => {{
        use arvo_format::overflow::{Saturate, Wrap};
        use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
        use arvo_format::{Adapt, Signature};
        $body::<Signature<$format, Adapt<TowardZero, Wrap>>>($($args),*);
        $body::<Signature<$format, Adapt<Floor, Wrap>>>($($args),*);
        $body::<Signature<$format, Adapt<Ceil, Wrap>>>($($args),*);
        $body::<Signature<$format, Adapt<HalfUp, Wrap>>>($($args),*);
        $body::<Signature<$format, Adapt<HalfEven, Wrap>>>($($args),*);
        $body::<Signature<$format, Adapt<TowardZero, Saturate>>>($($args),*);
        $body::<Signature<$format, Adapt<Floor, Saturate>>>($($args),*);
        $body::<Signature<$format, Adapt<Ceil, Saturate>>>($($args),*);
        $body::<Signature<$format, Adapt<HalfUp, Saturate>>>($($args),*);
        $body::<Signature<$format, Adapt<HalfEven, Saturate>>>($($args),*);
    }};
}

/// The formats of a radix family, each named from a width and an exponent.
#[macro_export]
macro_rules! binary_signed {
    ($w:literal, $e:expr) => { arvo_format::points::Fixed<$w, { $e }> };
}
#[macro_export]
macro_rules! binary_unsigned {
    ($w:literal, $e:expr) => { arvo_format::points::UFixed<$w, { $e }> };
}
#[macro_export]
macro_rules! decimal_signed {
    ($w:literal, $e:expr) => {
        $crate::common::points::Scaled<
            arvo_format::DecimalRationals,
            arvo_format::slots::Signed<$w>,
            { $e },
        >
    };
}
#[macro_export]
macro_rules! decimal_unsigned {
    ($w:literal, $e:expr) => {
        $crate::common::points::Scaled<
            $crate::common::points::UnsignedDecimalRationals,
            arvo_format::slots::Unsigned<$w>,
            { $e },
        >
    };
}
#[macro_export]
macro_rules! ternary_signed {
    ($w:literal, $e:expr) => {
        $crate::common::points::Scaled<
            $crate::common::points::TernaryRationals,
            arvo_format::slots::Signed<$w>,
            { $e },
        >
    };
}
#[macro_export]
macro_rules! ternary_unsigned {
    ($w:literal, $e:expr) => {
        $crate::common::points::Scaled<
            $crate::common::points::UnsignedTernaryRationals,
            arvo_format::slots::Unsigned<$w>,
            { $e },
        >
    };
}

/// The formats of one family at one width, signed and unsigned, at each
/// exponent of the operation's family. An exponent is a `const fn` of the
/// width, whether the range is signed, and the radix. `$use` is called once a
/// format with `signed` or `unsigned`, the format, the width, the radix, the
/// exponent and `$args`.
#[macro_export]
macro_rules! formats_at {
    ($w:literal, $radix:literal, $signed:ident, $unsigned:ident, [$($e:ident),+],
     $use:ident, $args:tt) => {
        $(
            {
                const E: i32 = $e($w, true, $radix);
                $use!(signed, $signed!($w, E), $w, $radix, E, $args);
            }
            {
                const E: i32 = $e($w, false, $radix);
                $use!(unsigned, $unsigned!($w, E), $w, $radix, E, $args);
            }
        )+
    };
}

/// The three radix families at one width.
#[macro_export]
macro_rules! binary_at {
    ($w:literal, $exponents:tt, $use:ident, $args:tt) => {
        $crate::formats_at!(
            $w,
            2,
            binary_signed,
            binary_unsigned,
            $exponents,
            $use,
            $args
        );
    };
}
#[macro_export]
macro_rules! decimal_at {
    ($w:literal, $exponents:tt, $use:ident, $args:tt) => {
        $crate::formats_at!(
            $w,
            10,
            decimal_signed,
            decimal_unsigned,
            $exponents,
            $use,
            $args
        );
    };
}
#[macro_export]
macro_rules! ternary_at {
    ($w:literal, $exponents:tt, $use:ident, $args:tt) => {
        $crate::formats_at!(
            $w,
            3,
            ternary_signed,
            ternary_unsigned,
            $exponents,
            $use,
            $args
        );
    };
}

/// The widest magnitude of a range of `w` bits.
pub const fn widest(w: u32, signed: bool) -> i128 {
    if signed { 1i128 << (w - 1) } else { (1i128 << w) - 1 }
}

/// The exponent zero, at every shape.
pub const fn zero(_w: u32, _signed: bool, _radix: i128) -> i32 {
    0
}

/// The exponent one.
pub const fn one(_w: u32, _signed: bool, _radix: i128) -> i32 {
    1
}

/// The exponent minus one.
pub const fn minus_one(_w: u32, _signed: bool, _radix: i128) -> i32 {
    -1
}

/// Sixteen widths, two ranges, five exponents and the ten adaptations.
pub const SIGNATURES_OF_A_BAND: u64 = 16 * 2 * 5 * 10;

/// The pairs a sweep over every shape takes of one signature: every pair up
/// to six bits, and above that the edge lattice against the ends of the range
/// and the neighbours of zero, both ways round, with seeded pairs.
pub fn pairs<S: Sole>() -> Vec<(i128, i128)> {
    let (min, max) = range::<S>();
    if max - min < 64 {
        return (min ..= max)
            .flat_map(|a| (min ..= max).map(move |b| (a, b)))
            .collect();
    }
    let edges = lattice::<S>();
    let core: Vec<i128> = [min, min + 1, -1, 0, 1, max - 1, max]
        .into_iter()
        .filter(|x| *x >= min && *x <= max)
        .collect();
    let mut out = Vec::new();
    for &a in edges.as_slice() {
        for &b in &core {
            out.push((a, b));
            out.push((b, a));
        }
    }
    let mut rng = SplitMix::seeded();
    out.extend((0 .. SEEDED_MEMBERS).map(|_| (rng.member(min, max), rng.member(min, max))));
    out
}
