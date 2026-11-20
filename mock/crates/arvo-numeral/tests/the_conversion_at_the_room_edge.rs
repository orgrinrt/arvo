//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion at the edge of the room the value forms it in, against the
//! tests' own arithmetic.
//!
//! From exponent zero onto a deeper exponent, the value forms the position in
//! the host's 128-bit integer while the source's widest member times the radix
//! to minus the target's exponent stays below `2^127`, and the format's four
//! limbs take it one exponent past. Each radix is taken at 32 and 64 bits from
//! both source ranges onto both target ranges, at the last exponent inside the
//! room and the first past it, under every adaptation of the target. The
//! members of a source include both ends of its range.

mod common;

use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use common::edges::host_bound;
use common::shapes::{Shapes, widest};

/// The deepest target exponent the value converts onto in the host's integer
/// from a source of `w` bits at exponent zero.
const fn room(w: u32, signed: bool, radix: i128) -> i32 {
    -host_bound(widest(w, signed) as u128, radix)
}

/// One source range at one width onto both target ranges at the room's last
/// exponent and the one past it.
macro_rules! from_one_range {
    ($w:literal, $radix:literal, $source:ident, $is_signed:literal, $signed:ident,
     $unsigned:ident, $shapes:expr) => {{
        type Source = Signature<$source!($w, 0), Adapt<TowardZero, Wrap>>;
        let members = common::crossings::source::<Source>();
        const IN: i32 = room($w, $is_signed, $radix);
        const PAST: i32 = IN - 1;
        onto_every_adaptation!(Source, $signed!($w, IN), &members, $shapes);
        onto_every_adaptation!(Source, $unsigned!($w, IN), &members, $shapes);
        onto_every_adaptation!(Source, $signed!($w, PAST), &members, $shapes);
        onto_every_adaptation!(Source, $unsigned!($w, PAST), &members, $shapes);
    }};
}

/// Both source ranges of one radix family at one width.
macro_rules! at_width {
    ($w:literal, $radix:literal, $signed:ident, $unsigned:ident, $shapes:expr) => {{
        from_one_range!($w, $radix, $signed, true, $signed, $unsigned, $shapes);
        from_one_range!($w, $radix, $unsigned, false, $signed, $unsigned, $shapes);
    }};
}

/// One radix family at 32 and 64 bits.
macro_rules! room_edge {
    ($name:ident, $radix:literal, $signed:ident, $unsigned:ident) => {
        #[test]
        fn $name() {
            let mut shapes = Shapes::default();
            at_width!(32, $radix, $signed, $unsigned, &mut shapes);
            at_width!(64, $radix, $signed, $unsigned, &mut shapes);
            // Two widths, two source ranges, two target ranges, two exponents
            // and ten adaptations of the target.
            shapes.agree("`convert`", 2 * 2 * 2 * 2 * 10);
        }
    };
}

room_edge!(
    binary_conversions_at_the_room_edge,
    2,
    binary_signed,
    binary_unsigned
);
room_edge!(
    decimal_conversions_at_the_room_edge,
    10,
    decimal_signed,
    decimal_unsigned
);
room_edge!(
    ternary_conversions_at_the_room_edge,
    3,
    ternary_signed,
    ternary_unsigned
);

/// The edges written out, so the room's arithmetic is held to numbers too.
#[test]
fn the_conversion_room_ends_where_arithmetic_puts_it() {
    assert_eq!(room(64, true, 2), -63);
    assert_eq!(room(64, false, 2), -63);
    assert_eq!(room(64, true, 10), -19);
    assert_eq!(room(64, false, 10), -18);
    assert_eq!(room(64, true, 3), -40);
    assert_eq!(room(64, false, 3), -39);
    assert_eq!(room(32, true, 2), -95);
}
