//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion between every pair of widths of one radix family.
//!
//! A source of each width from 1 to 64, signed and unsigned, onto a target of
//! each width from 1 to 64, signed and unsigned, under both policies of the
//! target: 16384 pairs of formats. A matrix is taken at one pair of exponents
//! and one mode, which its test file names. The five modes and the exponents
//! are crossed at each width in the exponent sweeps, and a build cannot
//! instantiate their product with this one. A matrix is one test binary, in
//! four bands of source widths.

/// One source width onto one target width, all four pairs of ranges.
#[macro_export]
macro_rules! width_pair {
    ($wt:literal, $ws:literal, $signed:ident, $unsigned:ident, $es:literal, $et:literal,
     $mode:ident, $shapes:expr) => {{
        use arvo_format::overflow::Wrap;
        use arvo_format::rounding::TowardZero;
        use arvo_format::{Adapt, Signature};
        type From<F> = Signature<F, Adapt<TowardZero, Wrap>>;
        {
            let members = $crate::common::crossings::source::<From<$signed!($ws, $es)>>();
            $crate::onto_both_policies!(
                From<$signed!($ws, $es)>,
                $signed!($wt, $et),
                $mode,
                &members,
                $shapes
            );
            $crate::onto_both_policies!(
                From<$signed!($ws, $es)>,
                $unsigned!($wt, $et),
                $mode,
                &members,
                $shapes
            );
        }
        {
            let members = $crate::common::crossings::source::<From<$unsigned!($ws, $es)>>();
            $crate::onto_both_policies!(
                From<$unsigned!($ws, $es)>,
                $signed!($wt, $et),
                $mode,
                &members,
                $shapes
            );
            $crate::onto_both_policies!(
                From<$unsigned!($ws, $es)>,
                $unsigned!($wt, $et),
                $mode,
                &members,
                $shapes
            );
        }
    }};
}

/// One source width onto every target width, in a function of its own, so
/// that no test holds a band's crossings in one frame.
#[macro_export]
macro_rules! onto_every_width {
    ($ws:literal, $signed:ident, $unsigned:ident, $es:literal, $et:literal, $mode:ident,
     $shapes:expr) => {{
        fn from_one_width(shapes: &mut $crate::common::shapes::Shapes) {
            $crate::widths_1_to_16!(width_pair!(
                $ws, $signed, $unsigned, $es, $et, $mode, shapes
            ));
            $crate::widths_17_to_32!(width_pair!(
                $ws, $signed, $unsigned, $es, $et, $mode, shapes
            ));
            $crate::widths_33_to_48!(width_pair!(
                $ws, $signed, $unsigned, $es, $et, $mode, shapes
            ));
            $crate::widths_49_to_64!(width_pair!(
                $ws, $signed, $unsigned, $es, $et, $mode, shapes
            ));
        }
        from_one_width($shapes);
    }};
}

/// The four bands of source widths of one family, each a test: the formats'
/// two macros, the source's exponent, the target's, and the target's mode.
#[macro_export]
macro_rules! width_matrix {
    (@band $name:ident, $widths:ident, $($family:tt)*) => {
        #[test]
        fn $name() {
            let mut shapes = $crate::common::shapes::Shapes::default();
            $widths!(onto_every_width!($($family)*, &mut shapes));
            // Sixteen source widths, sixty-four target widths, four pairs of
            // ranges, both policies.
            shapes.agree("`convert`", 16 * 64 * 4 * 2);
            assert!(shapes.planted > 0, "the planted policy was seen nowhere");
        }
    };
    ($($family:tt)*) => {
        $crate::width_matrix!(@band from_widths_1_to_16, widths_1_to_16, $($family)*);
        $crate::width_matrix!(@band from_widths_17_to_32, widths_17_to_32, $($family)*);
        $crate::width_matrix!(@band from_widths_33_to_48, widths_33_to_48, $($family)*);
        $crate::width_matrix!(@band from_widths_49_to_64, widths_49_to_64, $($family)*);
    };
}
