//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at each width between every pair of exponents, under every
//! source signature, and between every pair of radixes.
//!
//! The exponents of a width are zero, one, minus one, minus half the width
//! rounded up, and the deepest one a conversion from exponent one admits, so
//! every pair of the five is a conversion the format admits and the family ends
//! at the obligation's own edge. Both ranges are on each side.

/// The deepest exponent a target of `w` bits takes from a source of `w` bits at
/// exponent one: minus the largest `k` at which the widest member of the width,
/// the top of the unsigned range, times the radix to the `k + 1` is still a
/// position a slot index holds. The same on either range, so that every pair of
/// ranges is admitted.
pub const fn deep(w: u32, _signed: bool, radix: i128) -> i32 {
    let mut at = ((1i128 << w) - 1) * radix;
    let mut k = 0;
    while at <= i128::MAX / radix {
        at *= radix;
        k += 1;
    }
    -k
}

/// Minus half the width rounded up, or the deepest exponent where that is
/// nearer zero.
pub const fn half(w: u32, signed: bool, radix: i128) -> i32 {
    let half = -((w as i32 + 1) / 2);
    let deep = deep(w, signed, radix);
    if half > deep { half } else { deep }
}

/// The exponent zero, as the family names it.
pub const fn zero(w: u32, signed: bool, radix: i128) -> i32 {
    super::shapes::zero(w, signed, radix)
}

/// The exponent one.
pub const fn one(w: u32, signed: bool, radix: i128) -> i32 {
    super::shapes::one(w, signed, radix)
}

/// The exponent minus one.
pub const fn minus_one(w: u32, signed: bool, radix: i128) -> i32 {
    super::shapes::minus_one(w, signed, radix)
}

/// One source signature at one exponent onto each exponent of the target's
/// family, on one target range, under every adaptation of the target.
#[macro_export]
macro_rules! onto_every_exponent {
    ($w:literal, $radix:literal, $source:ty, $target:ident, $is_signed:literal,
     $members:expr, $shapes:expr, [$($et:ident),+]) => {
        $({
            const ET: i32 = $crate::common::exponents::$et($w, $is_signed, $radix);
            $crate::onto_every_adaptation!($source, $target!($w, ET), $members, $shapes);
        })+
    };
}

/// One source range of one width at each exponent of the family, onto both
/// target ranges at each exponent.
#[macro_export]
macro_rules! from_every_exponent {
    ($w:literal, $radix:literal, $source:ident, $is_signed:literal, $signed:ident,
     $unsigned:ident, $shapes:expr, [$($es:ident),+]) => {
        $({
            use arvo_format::overflow::Wrap;
            use arvo_format::rounding::TowardZero;
            use arvo_format::{Adapt, Signature};
            const ES: i32 = $crate::common::exponents::$es($w, $is_signed, $radix);
            type Source = Signature<$source!($w, ES), Adapt<TowardZero, Wrap>>;
            let members = $crate::common::crossings::source::<Source>();
            $crate::onto_every_exponent!(
                $w, $radix, Source, $signed, true, &members, $shapes,
                [zero, one, minus_one, half, deep]
            );
            $crate::onto_every_exponent!(
                $w, $radix, Source, $unsigned, false, &members, $shapes,
                [zero, one, minus_one, half, deep]
            );
        })+
    };
}

/// Every pair of exponents and of ranges at one width.
#[macro_export]
macro_rules! exponent_pairs_at {
    ($w:literal, $radix:literal, $signed:ident, $unsigned:ident, $shapes:expr) => {{
        // A function a width, so that no test holds a band's crossings in one frame.
        fn at_one_width(shapes: &mut $crate::common::shapes::Shapes) {
            $crate::from_every_exponent!($w, $radix, $signed, true, $signed, $unsigned, shapes, [
                zero, one, minus_one, half, deep
            ]);
            $crate::from_every_exponent!(
                $w,
                $radix,
                $unsigned,
                false,
                $signed,
                $unsigned,
                shapes,
                [zero, one, minus_one, half, deep]
            );
        }
        at_one_width($shapes);
    }};
}

/// Half of one family's exponent pairs, each width band of sixteen a test:
/// `narrow` is widths 1 to 32 and `wide` is 33 to 64. A half is one test
/// binary, since the whole family in one does not build in the memory a
/// container has.
#[macro_export]
macro_rules! exponent_matrix {
    (narrow, $radix:literal, $signed:ident, $unsigned:ident) => {
        $crate::exponent_matrix!(@band exponents_at_widths_1_to_16, widths_1_to_16,
            $radix, $signed, $unsigned);
        $crate::exponent_matrix!(@band exponents_at_widths_17_to_32, widths_17_to_32,
            $radix, $signed, $unsigned);
    };
    (wide, $radix:literal, $signed:ident, $unsigned:ident) => {
        $crate::exponent_matrix!(@band exponents_at_widths_33_to_48, widths_33_to_48,
            $radix, $signed, $unsigned);
        $crate::exponent_matrix!(@band exponents_at_widths_49_to_64, widths_49_to_64,
            $radix, $signed, $unsigned);
    };
    (@band $name:ident, $widths:ident, $radix:literal, $signed:ident, $unsigned:ident) => {
        #[test]
        fn $name() {
            let mut shapes = $crate::common::shapes::Shapes::default();
            $widths!(exponent_pairs_at!($radix, $signed, $unsigned, &mut shapes));
            // Sixteen widths, two source ranges and five source exponents, two
            // target ranges and five target exponents, ten signatures of the
            // target.
            shapes.agree("`convert`", 16 * 2 * 5 * 2 * 5 * 10);
        }
    };
}

/// One source format under every adaptation, onto one target format under
/// every adaptation.
#[macro_export]
macro_rules! from_every_adaptation {
    ($source:ty, $target:ty, $shapes:expr) => {{
        use arvo_format::overflow::{Saturate, Wrap};
        use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
        use arvo_format::{Adapt, Signature};
        macro_rules! from {
            ($mode:ty, $policy:ty) => {{
                type Source = Signature<$source, Adapt<$mode, $policy>>;
                let members = $crate::common::crossings::source::<Source>();
                $crate::onto_every_adaptation!(Source, $target, &members, $shapes);
            }};
        }
        from!(TowardZero, Wrap);
        from!(Floor, Wrap);
        from!(Ceil, Wrap);
        from!(HalfUp, Wrap);
        from!(HalfEven, Wrap);
        from!(TowardZero, Saturate);
        from!(Floor, Saturate);
        from!(Ceil, Saturate);
        from!(HalfUp, Saturate);
        from!(HalfEven, Saturate);
    }};
}

/// Every source signature onto every target signature at one width, across
/// one step of exponent, a signed range onto a signed one and an unsigned onto
/// an unsigned.
#[macro_export]
macro_rules! source_signatures_at {
    ($w:literal, $signed:ident, $unsigned:ident, $shapes:expr) => {{
        fn at_one_width(shapes: &mut $crate::common::shapes::Shapes) {
            $crate::from_every_adaptation!($signed!($w, -1), $signed!($w, 0), shapes);
            $crate::from_every_adaptation!($unsigned!($w, -1), $unsigned!($w, 0), shapes);
        }
        at_one_width($shapes);
    }};
}

/// The source signatures of one family at every width, one test.
#[macro_export]
macro_rules! source_signatures {
    ($name:ident, $signed:ident, $unsigned:ident) => {
        #[test]
        fn $name() {
            let mut shapes = $crate::common::shapes::Shapes::default();
            $crate::widths_1_to_16!(source_signatures_at!($signed, $unsigned, &mut shapes));
            $crate::widths_17_to_32!(source_signatures_at!($signed, $unsigned, &mut shapes));
            $crate::widths_33_to_48!(source_signatures_at!($signed, $unsigned, &mut shapes));
            $crate::widths_49_to_64!(source_signatures_at!($signed, $unsigned, &mut shapes));
            // Sixty-four widths, two ranges, ten signatures a side.
            shapes.agree("`convert`", 64 * 2 * 10 * 10);
        }
    };
}

/// One width of one radix family onto the same width of another, both at
/// exponent minus one, so a source slot is the target's radix over the
/// source's in target slots: all four pairs of ranges, every adaptation of the
/// target.
#[macro_export]
macro_rules! radix_pair_at {
    ($w:literal, $from_signed:ident, $from_unsigned:ident, $onto_signed:ident,
     $onto_unsigned:ident, $shapes:expr) => {{
        fn at_one_width(shapes: &mut $crate::common::shapes::Shapes) {
            use arvo_format::overflow::Wrap;
            use arvo_format::rounding::TowardZero;
            use arvo_format::{Adapt, Signature};
            type From<F> = Signature<F, Adapt<TowardZero, Wrap>>;
            {
                type Source = From<$from_signed!($w, -1)>;
                let members = $crate::common::crossings::source::<Source>();
                $crate::onto_every_adaptation!(Source, $onto_signed!($w, -1), &members, shapes);
                $crate::onto_every_adaptation!(Source, $onto_unsigned!($w, -1), &members, shapes);
            }
            {
                type Source = From<$from_unsigned!($w, -1)>;
                let members = $crate::common::crossings::source::<Source>();
                $crate::onto_every_adaptation!(Source, $onto_signed!($w, -1), &members, shapes);
                $crate::onto_every_adaptation!(Source, $onto_unsigned!($w, -1), &members, shapes);
            }
        }
        at_one_width($shapes);
    }};
}

/// One ordered pair of radix families at every width, one test.
#[macro_export]
macro_rules! radix_pair {
    ($name:ident, $($families:ident),+) => {
        #[test]
        fn $name() {
            let mut shapes = $crate::common::shapes::Shapes::default();
            $crate::widths_1_to_16!(radix_pair_at!($($families),+, &mut shapes));
            $crate::widths_17_to_32!(radix_pair_at!($($families),+, &mut shapes));
            $crate::widths_33_to_48!(radix_pair_at!($($families),+, &mut shapes));
            $crate::widths_49_to_64!(radix_pair_at!($($families),+, &mut shapes));
            // Sixty-four widths, four pairs of ranges, ten signatures of the
            // target.
            shapes.agree("`convert`", 64 * 4 * 10);
        }
    };
}
