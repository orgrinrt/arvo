//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The arithmetic in one ladder carrier, written once and expanded per rung.
//!
//! Every function takes its rung from its name and the declared width from an
//! argument, independently: the placement picks the narrowest rung for a value, and
//! the tests compute a narrow width in every rung. A value passed in is a member
//! of the width's range, held in the rung's integer.
//!
//! Total over members: no assertion and no panic. `Policy::Clamp` is never
//! passed, since `Sole::ADMITTED` refuses it on a signature.

use core::cmp::Ordering;

use arvo_format::Policy;

macro_rules! rung {
    (
        bits $bits:literal, mutant $mutant:literal,
        $s:ident, $u:ident,
        $adapt_s:ident, $adapt_u:ident,
        $add_s:ident, $add_u:ident,
        $from_s:ident, $from_u:ident,
        $index_s:ident, $index_u:ident,
        $cmp_s:ident, $cmp_u:ident
    ) => {
        /// A signed integer of this rung put onto the `width`-bit signed range.
        ///
        /// Re-canonicalises a result below the carrier width. The rung's mutant
        /// removes it.
        #[inline]
        #[must_use]
        pub(crate) const fn $adapt_s(x: $s, width: u32, policy: Policy) -> $s {
            if cfg!(feature = $mutant) {
                return x;
            }
            let shift = $bits - width;
            match policy {
                Policy::Wrap => (x << shift) >> shift,
                _ => {
                    let lo = $s::MIN >> shift;
                    let hi = $s::MAX >> shift;
                    if x < lo {
                        lo
                    } else if x > hi {
                        hi
                    } else {
                        x
                    }
                },
            }
        }

        /// An unsigned integer of this rung put onto the `width`-bit unsigned range.
        #[inline]
        #[must_use]
        pub(crate) const fn $adapt_u(x: $u, width: u32, policy: Policy) -> $u {
            if cfg!(feature = $mutant) {
                return x;
            }
            let hi = $u::MAX >> ($bits - width);
            match policy {
                Policy::Wrap => x & hi,
                _ => {
                    if x > hi {
                        hi
                    } else {
                        x
                    }
                },
            }
        }

        /// The sum of two signed members, in this rung.
        ///
        /// Below the carrier width two members cannot leave the carrier, so the
        /// wrapping add is the exact sum and the adaptation completes it. At the
        /// carrier width the machine's own wrapping or saturating add is the
        /// answer.
        #[inline]
        #[must_use]
        pub(crate) const fn $add_s(a: $s, b: $s, width: u32, policy: Policy) -> $s {
            if width < $bits {
                $adapt_s(a.wrapping_add(b), width, policy)
            } else {
                match policy {
                    Policy::Wrap => a.wrapping_add(b),
                    _ => a.saturating_add(b),
                }
            }
        }

        /// The sum of two unsigned members, in this rung.
        #[inline]
        #[must_use]
        pub(crate) const fn $add_u(a: $u, b: $u, width: u32, policy: Policy) -> $u {
            if width < $bits {
                $adapt_u(a.wrapping_add(b), width, policy)
            } else {
                match policy {
                    Policy::Wrap => a.wrapping_add(b),
                    _ => a.saturating_add(b),
                }
            }
        }

        /// The carrier of a member's slot index. The index is a member's.
        #[inline]
        #[must_use]
        pub(crate) const fn $from_s(index: i128) -> $s {
            index as $s
        }

        /// The carrier of a member's slot index. The index is a member's.
        #[inline]
        #[must_use]
        pub(crate) const fn $from_u(index: i128) -> $u {
            index as $u
        }

        /// A member's slot index.
        #[inline]
        #[must_use]
        pub(crate) const fn $index_s(x: $s) -> i128 {
            x as i128
        }

        /// A member's slot index.
        #[inline]
        #[must_use]
        pub(crate) const fn $index_u(x: $u) -> i128 {
            x as i128
        }

        /// The order of two members.
        #[inline]
        #[must_use]
        pub(crate) const fn $cmp_s(a: $s, b: $s) -> Ordering {
            if a < b {
                Ordering::Less
            } else if a > b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

        /// The order of two members.
        #[inline]
        #[must_use]
        pub(crate) const fn $cmp_u(a: $u, b: $u) -> Ordering {
            if a < b {
                Ordering::Less
            } else if a > b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    };
}

rung!(
    bits 8, mutant "mutant-8", i8, u8,
    adapt_s8, adapt_u8, add_s8, add_u8, from_s8, from_u8, index_s8, index_u8, cmp_s8, cmp_u8
);
rung!(
    bits 16, mutant "mutant-16", i16, u16,
    adapt_s16, adapt_u16, add_s16, add_u16, from_s16, from_u16, index_s16, index_u16, cmp_s16, cmp_u16
);
rung!(
    bits 32, mutant "mutant-32", i32, u32,
    adapt_s32, adapt_u32, add_s32, add_u32, from_s32, from_u32, index_s32, index_u32, cmp_s32, cmp_u32
);
rung!(
    bits 64, mutant "mutant-64", i64, u64,
    adapt_s64, adapt_u64, add_s64, add_u64, from_s64, from_u64, index_s64, index_u64, cmp_s64, cmp_u64
);
