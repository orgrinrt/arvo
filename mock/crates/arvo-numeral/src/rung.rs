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
//! The product here is the slots' product, which is the members' product only
//! at a quantum exponent of zero. `Num::mul` calls it nowhere else.
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
        $sub_s:ident, $sub_u:ident,
        $neg_s:ident, $neg_u:ident,
        $mul_s:ident, $mul_u:ident,
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

        /// The difference of two signed members, in this rung.
        ///
        /// Below the carrier width the wrapping difference is exact and the
        /// adaptation completes it. At the carrier width the machine's own
        /// wrapping or saturating difference is the answer.
        #[inline]
        #[must_use]
        pub(crate) const fn $sub_s(a: $s, b: $s, width: u32, policy: Policy) -> $s {
            if width < $bits {
                $adapt_s(a.wrapping_sub(b), width, policy)
            } else {
                match policy {
                    Policy::Wrap => a.wrapping_sub(b),
                    _ => a.saturating_sub(b),
                }
            }
        }

        /// The difference of two unsigned members, in this rung.
        ///
        /// A difference below zero leaves the carrier as a wrapped bit pattern,
        /// which the unsigned adaptation would read as a large number. So under
        /// saturation it is the machine's own difference, pinned at zero, and
        /// only wrapping passes through the adaptation. The operation's mutant
        /// pins it to the top.
        #[inline]
        #[must_use]
        pub(crate) const fn $sub_u(a: $u, b: $u, width: u32, policy: Policy) -> $u {
            match policy {
                Policy::Wrap => $adapt_u(a.wrapping_sub(b), width, policy),
                _ => {
                    if cfg!(feature = "mutant-sub") && a < b {
                        return $u::MAX >> ($bits - width);
                    }
                    a.saturating_sub(b)
                },
            }
        }

        /// The negation of a signed member, in this rung.
        ///
        /// Below the carrier width the negation cannot leave the carrier. At the
        /// carrier width the lowest member's does, and the machine's wrapping
        /// or saturating negation completes it. The operation's mutant leaves
        /// that member where it is under saturation.
        #[inline]
        #[must_use]
        pub(crate) const fn $neg_s(a: $s, width: u32, policy: Policy) -> $s {
            if width < $bits {
                $adapt_s(a.wrapping_neg(), width, policy)
            } else {
                match policy {
                    Policy::Wrap => a.wrapping_neg(),
                    _ => {
                        if cfg!(feature = "mutant-neg") {
                            return a.wrapping_neg();
                        }
                        a.saturating_neg()
                    },
                }
            }
        }

        /// The negation of an unsigned member, in this rung.
        ///
        /// Below the range unless the member is zero: the two's complement
        /// negation at the declared width under wrapping, zero under
        /// saturation.
        #[inline]
        #[must_use]
        pub(crate) const fn $neg_u(a: $u, width: u32, policy: Policy) -> $u {
            match policy {
                Policy::Wrap => $adapt_u(a.wrapping_neg(), width, policy),
                _ => 0,
            }
        }

        /// The product of two signed members' slots, in this rung.
        ///
        /// Under wrapping the low bits of the product depend only on the low
        /// bits of the factors, so the wrapping product and the adaptation are
        /// the answer at every width. Under saturation a product that fits the
        /// machine integer is completed by the adaptation, and one that does
        /// not is past the range on its sign. The operation's mutant pins that
        /// one to the top whatever its sign.
        #[inline]
        #[must_use]
        pub(crate) const fn $mul_s(a: $s, b: $s, width: u32, policy: Policy) -> $s {
            match policy {
                Policy::Wrap => $adapt_s(a.wrapping_mul(b), width, policy),
                _ => {
                    let (product, left) = a.overflowing_mul(b);
                    if !left {
                        $adapt_s(product, width, policy)
                    } else if (a < 0) != (b < 0) && !cfg!(feature = "mutant-mul") {
                        $s::MIN >> ($bits - width)
                    } else {
                        $s::MAX >> ($bits - width)
                    }
                },
            }
        }

        /// The product of two unsigned members' slots, in this rung.
        #[inline]
        #[must_use]
        pub(crate) const fn $mul_u(a: $u, b: $u, width: u32, policy: Policy) -> $u {
            match policy {
                Policy::Wrap => $adapt_u(a.wrapping_mul(b), width, policy),
                _ => {
                    let (product, left) = a.overflowing_mul(b);
                    if left { $u::MAX >> ($bits - width) } else { $adapt_u(product, width, policy) }
                },
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
    adapt_s8, adapt_u8, add_s8, add_u8, sub_s8, sub_u8, neg_s8, neg_u8, mul_s8, mul_u8,
    from_s8, from_u8, index_s8, index_u8, cmp_s8, cmp_u8
);
rung!(
    bits 16, mutant "mutant-16", i16, u16,
    adapt_s16, adapt_u16, add_s16, add_u16, sub_s16, sub_u16, neg_s16, neg_u16, mul_s16, mul_u16,
    from_s16, from_u16, index_s16, index_u16, cmp_s16, cmp_u16
);
rung!(
    bits 32, mutant "mutant-32", i32, u32,
    adapt_s32, adapt_u32, add_s32, add_u32, sub_s32, sub_u32, neg_s32, neg_u32, mul_s32, mul_u32,
    from_s32, from_u32, index_s32, index_u32, cmp_s32, cmp_u32
);
rung!(
    bits 64, mutant "mutant-64", i64, u64,
    adapt_s64, adapt_u64, add_s64, add_u64, sub_s64, sub_u64, neg_s64, neg_u64, mul_s64, mul_u64,
    from_s64, from_u64, index_s64, index_u64, cmp_s64, cmp_u64
);
