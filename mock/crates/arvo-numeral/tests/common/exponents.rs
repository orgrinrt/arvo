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

pub use super::shapes::{minus_one, one, zero};

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
/// one step of exponent from `$from` onto `$onto`, a signed range onto a signed
/// one and an unsigned onto an unsigned.
#[macro_export]
macro_rules! source_signatures_at {
    ($w:literal, $from:literal, $onto:literal, $signed:ident, $unsigned:ident, $shapes:expr) => {{
        fn at_one_width(shapes: &mut $crate::common::shapes::Shapes) {
            $crate::from_every_adaptation!($signed!($w, $from), $signed!($w, $onto), shapes);
            $crate::from_every_adaptation!($unsigned!($w, $from), $unsigned!($w, $onto), shapes);
        }
        at_one_width($shapes);
    }};
}

/// The source signatures of one family at every width across one step of
/// exponent, one test. `$leaves` says whether some member leaves the target's
/// range on that step, which the test asserts: where none does no policy is
/// consulted, and where some do every pair of policies is.
#[macro_export]
macro_rules! source_signatures {
    ($name:ident, $from:literal => $onto:literal, leaves $leaves:literal,
     $signed:ident, $unsigned:ident) => {
        #[test]
        fn $name() {
            let mut shapes = $crate::common::shapes::Shapes::default();
            $crate::widths_1_to_16!(source_signatures_at!(
                $from,
                $onto,
                $signed,
                $unsigned,
                &mut shapes
            ));
            $crate::widths_17_to_32!(source_signatures_at!(
                $from,
                $onto,
                $signed,
                $unsigned,
                &mut shapes
            ));
            $crate::widths_33_to_48!(source_signatures_at!(
                $from,
                $onto,
                $signed,
                $unsigned,
                &mut shapes
            ));
            $crate::widths_49_to_64!(source_signatures_at!(
                $from,
                $onto,
                $signed,
                $unsigned,
                &mut shapes
            ));
            // Sixty-four widths, two ranges, ten signatures a side.
            shapes.agree("`convert`", 64 * 2 * 10 * 10);
            assert_eq!(
                shapes.unfit > 0,
                $leaves,
                "whether a member left the target's range, {} of {} did",
                shapes.unfit,
                shapes.checked
            );
        }
    };
}

/// The conversion at the positive edge of either side, at one width: each
/// source range at the highest exponent it converts from onto both target
/// ranges at exponent zero, and each source range at exponent zero onto both
/// target ranges at the highest exponent they convert onto, under every
/// adaptation of the target. Each edge is held against the format's verdict,
/// admitted at it and refused one past it.
#[macro_export]
macro_rules! positive_edges_at {
    ($w:literal, $radix:literal, $signed:ident, $unsigned:ident, $shapes:expr) => {{
        fn at_one_width(shapes: &mut $crate::common::shapes::Shapes) {
            $crate::positive_edges_at!(@from $w, $radix, $signed, true, $signed, $unsigned, shapes);
            $crate::positive_edges_at!(@from $w, $radix, $unsigned, false, $signed, $unsigned,
                shapes);
            $crate::positive_edges_at!(@onto $w, $radix, $signed, $signed, true, shapes);
            $crate::positive_edges_at!(@onto $w, $radix, $signed, $unsigned, false, shapes);
            $crate::positive_edges_at!(@onto $w, $radix, $unsigned, $signed, true, shapes);
            $crate::positive_edges_at!(@onto $w, $radix, $unsigned, $unsigned, false, shapes);
        }
        at_one_width($shapes);
    }};
    (@from $w:literal, $radix:literal, $source:ident, $is_signed:literal, $signed:ident,
     $unsigned:ident, $shapes:expr) => {{
        use arvo_format::overflow::Wrap;
        use arvo_format::rounding::TowardZero;
        use arvo_format::{Adapt, Signature};
        const ES: i32 = $crate::common::edges::source_highest($w, $is_signed, $radix);
        $crate::conversion_edge!($source!($w, ES) => $signed!($w, 0),
            past $source!($w, { ES + 1 }) => $signed!($w, 0), $w, "source's");
        $crate::conversion_edge!($source!($w, ES) => $unsigned!($w, 0),
            past $source!($w, { ES + 1 }) => $unsigned!($w, 0), $w, "source's");
        type Source = Signature<$source!($w, ES), Adapt<TowardZero, Wrap>>;
        let members = $crate::common::crossings::source::<Source>();
        $crate::onto_every_adaptation!(Source, $signed!($w, 0), &members, $shapes);
        $crate::onto_every_adaptation!(Source, $unsigned!($w, 0), &members, $shapes);
    }};
    (@onto $w:literal, $radix:literal, $source:ident, $target:ident, $is_signed:literal,
     $shapes:expr) => {{
        use arvo_format::overflow::Wrap;
        use arvo_format::rounding::TowardZero;
        use arvo_format::{Adapt, Signature};
        const ET: i32 = $crate::common::edges::target_highest($w, $is_signed, $radix);
        $crate::conversion_edge!($source!($w, 0) => $target!($w, ET),
            past $source!($w, 0) => $target!($w, { ET + 1 }), $w, "target's");
        type Source = Signature<$source!($w, 0), Adapt<TowardZero, Wrap>>;
        let members = $crate::common::crossings::source::<Source>();
        $crate::onto_every_adaptation!(Source, $target!($w, ET), &members, $shapes);
    }};
}

/// One radix family's positive edges, each width band of sixteen a test.
#[macro_export]
macro_rules! positive_edges {
    ($radix:literal, $signed:ident, $unsigned:ident, $a:ident, $b:ident, $c:ident, $d:ident) => {
        $crate::positive_edges!(@band $a, widths_1_to_16, $radix, $signed, $unsigned);
        $crate::positive_edges!(@band $b, widths_17_to_32, $radix, $signed, $unsigned);
        $crate::positive_edges!(@band $c, widths_33_to_48, $radix, $signed, $unsigned);
        $crate::positive_edges!(@band $d, widths_49_to_64, $radix, $signed, $unsigned);
    };
    (@band $name:ident, $widths:ident, $radix:literal, $signed:ident, $unsigned:ident) => {
        #[test]
        fn $name() {
            let mut shapes = $crate::common::shapes::Shapes::default();
            $widths!(positive_edges_at!($radix, $signed, $unsigned, &mut shapes));
            // Sixteen widths; on the source's side two source ranges onto two
            // target ranges, on the target's side the same, ten signatures of
            // the target each.
            shapes.agree("`convert`", 16 * (2 * 2 + 2 * 2) * 10);
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
