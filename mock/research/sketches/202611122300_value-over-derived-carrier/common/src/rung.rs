//! The arithmetic in one ladder carrier, written once and expanded per rung.
//!
//! Every function takes its rung from its name and the width from an argument,
//! independently: the public map picks the narrowest rung, and the tests compute
//! a narrow width in every rung. A value passed in is a member of the width's
//! range, held in the rung's integer.
//!
//! Total: no assertion and no panic on any pair of members. `Policy::Clamp` is
//! never passed, since `ValuePolicy` refuses it on a signature; it takes the
//! saturating branch here.

use arvo_format::{Policy, Width};

macro_rules! rung {
    (
        bits $bits:literal, mutant $mutant:literal,
        $s:ident, $u:ident, $w:ident,
        $adapt_s:ident, $adapt_u:ident,
        $add_s:ident, $add_u:ident,
        $narrow_s:ident, $narrow_u:ident
    ) => {
        /// A signed integer of this rung adapted onto the `width`-bit signed range.
        ///
        /// The step that re-canonicalises a sub-width result. The mutant removes it.
        #[inline]
        #[must_use]
        pub const fn $adapt_s(x: $s, width: Width, policy: Policy) -> $s {
            if cfg!(feature = $mutant) {
                return x;
            }
            let shift = $bits - width.count();
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

        /// An unsigned integer of this rung adapted onto the `width`-bit unsigned range.
        #[inline]
        #[must_use]
        pub const fn $adapt_u(x: $u, width: Width, policy: Policy) -> $u {
            if cfg!(feature = $mutant) {
                return x;
            }
            let hi = $u::MAX >> ($bits - width.count());
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
        /// Where the width is the rung's own, the sum can leave the rung; the
        /// wrapping and saturating machine adds are then already the answer.
        #[inline]
        #[must_use]
        pub const fn $add_s(a: $s, b: $s, width: Width, policy: Policy) -> $s {
            let sum = match policy {
                Policy::Wrap => a.wrapping_add(b),
                _ => a.saturating_add(b),
            };
            $adapt_s(sum, width, policy)
        }

        /// The sum of two unsigned members, in this rung.
        #[inline]
        #[must_use]
        pub const fn $add_u(a: $u, b: $u, width: Width, policy: Policy) -> $u {
            let sum = match policy {
                Policy::Wrap => a.wrapping_add(b),
                _ => a.saturating_add(b),
            };
            $adapt_u(sum, width, policy)
        }

        /// An exact sum held in the double-width integer, adapted onto the
        /// signed range: the one adaptation of an accumulated sum.
        #[inline]
        #[must_use]
        pub const fn $narrow_s(wide: $w, width: Width, policy: Policy) -> $s {
            let shift = $bits - width.count();
            match policy {
                // The low bits of the exact sum are its residue; the sign
                // extension inside `adapt` puts it in range.
                Policy::Wrap => $adapt_s(wide as $s, width, policy),
                _ => {
                    let lo = ($s::MIN >> shift) as $w;
                    let hi = ($s::MAX >> shift) as $w;
                    if wide < lo {
                        lo as $s
                    } else if wide > hi {
                        hi as $s
                    } else {
                        wide as $s
                    }
                },
            }
        }

        /// An exact sum held in the double-width integer, adapted onto the
        /// unsigned range.
        #[inline]
        #[must_use]
        pub const fn $narrow_u(wide: $w, width: Width, policy: Policy) -> $u {
            match policy {
                Policy::Wrap => $adapt_u(wide as $u, width, policy),
                _ => {
                    let hi = ($u::MAX >> ($bits - width.count())) as $w;
                    if wide < 0 {
                        0
                    } else if wide > hi {
                        hi as $u
                    } else {
                        wide as $u
                    }
                },
            }
        }
    };
}

rung!(bits 8, mutant "mutant-8", i8, u8, i16, adapt_s8, adapt_u8, add_s8, add_u8, narrow_s8, narrow_u8);
rung!(bits 16, mutant "mutant-16", i16, u16, i32, adapt_s16, adapt_u16, add_s16, add_u16, narrow_s16, narrow_u16);
rung!(bits 32, mutant "mutant-32", i32, u32, i64, adapt_s32, adapt_u32, add_s32, add_u32, narrow_s32, narrow_u32);
rung!(bits 64, mutant "mutant-64", i64, u64, i128, adapt_s64, adapt_u64, add_s64, add_u64, narrow_s64, narrow_u64);
