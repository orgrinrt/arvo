//! Packed bitfield with named sub-ranges.
//!
//! The `bitfield!` macro generates a `#[repr(transparent)]` struct
//! over `Bits<N, Hot>` with per-field accessors and setters typed
//! as `Bits<W, Hot>` for sub-range width `W`. Exact-width arvo
//! primitives throughout; no raw u8 / u16 / u32 leaks.
//!
//! Grammar:
//!
//! ```text
//! bitfield! {
//!     $(#[$struct_attr])*
//!     $vis struct $name: $n {
//!         $(
//!             $(#[$field_attr])*
//!             $field: $field_bits at $lo,
//!         )*
//!     }
//! }
//! ```
//!
//! Each sub-range is specified by its `$field_bits` width and its
//! low bit index `$lo`. The occupied range is `$lo..($lo +
//! $field_bits)`. Current implementation supports contiguous
//! sub-ranges at an explicit low-bit index; range syntax
//! (`lo..=hi`) is a future refinement.
//!
//! Compile-time checks: `N <= 64`, and each sub-range fits within N
//! (i.e. `$lo + $field_bits <= N`). Overlap detection is deferred
//! to a future macro version (for now, authors are responsible).
//!
//! Example:
//!
//! ```ignore
//! use arvo::bitfield;
//!
//! bitfield! {
//!     /// 32-bit interned string handle.
//!     pub struct StrHandle: 32 {
//!         /// 1 = runtime-interned, 0 = compile-time.
//!         origin: 1 at 31,
//!         /// Reserved flag bits.
//!         reserved: 3 at 28,
//!         /// 28-bit interned identity.
//!         id: 28 at 0,
//!     }
//! }
//! ```

/// Internal helper: yields the concrete container primitive type for
/// a `Bits<N, Hot>` width literal. Mirrors `<Hot as BitsContainerFor<N, Unsigned>>::T`
/// per-N: `u8` for 1..=8, `u16` for 9..=16, `u32` for 17..=32, `u64`
/// for 33..=64.
///
/// Used by the `bitfield!` macro to bridge between u64-shaped shift and
/// mask arithmetic and the dispatched container type. Hidden from the
/// public surface; consumers go through the typed `Bits::from_raw` /
/// `Bits::to_raw` API directly.
#[doc(hidden)]
#[macro_export]
macro_rules! __bitfield_container_ty {
    (1) => {
        u8
    };
    (2) => {
        u8
    };
    (3) => {
        u8
    };
    (4) => {
        u8
    };
    (5) => {
        u8
    };
    (6) => {
        u8
    };
    (7) => {
        u8
    };
    (8) => {
        u8
    };
    (9) => {
        u16
    };
    (10) => {
        u16
    };
    (11) => {
        u16
    };
    (12) => {
        u16
    };
    (13) => {
        u16
    };
    (14) => {
        u16
    };
    (15) => {
        u16
    };
    (16) => {
        u16
    };
    (17) => {
        u32
    };
    (18) => {
        u32
    };
    (19) => {
        u32
    };
    (20) => {
        u32
    };
    (21) => {
        u32
    };
    (22) => {
        u32
    };
    (23) => {
        u32
    };
    (24) => {
        u32
    };
    (25) => {
        u32
    };
    (26) => {
        u32
    };
    (27) => {
        u32
    };
    (28) => {
        u32
    };
    (29) => {
        u32
    };
    (30) => {
        u32
    };
    (31) => {
        u32
    };
    (32) => {
        u32
    };
    (33) => {
        u64
    };
    (34) => {
        u64
    };
    (35) => {
        u64
    };
    (36) => {
        u64
    };
    (37) => {
        u64
    };
    (38) => {
        u64
    };
    (39) => {
        u64
    };
    (40) => {
        u64
    };
    (41) => {
        u64
    };
    (42) => {
        u64
    };
    (43) => {
        u64
    };
    (44) => {
        u64
    };
    (45) => {
        u64
    };
    (46) => {
        u64
    };
    (47) => {
        u64
    };
    (48) => {
        u64
    };
    (49) => {
        u64
    };
    (50) => {
        u64
    };
    (51) => {
        u64
    };
    (52) => {
        u64
    };
    (53) => {
        u64
    };
    (54) => {
        u64
    };
    (55) => {
        u64
    };
    (56) => {
        u64
    };
    (57) => {
        u64
    };
    (58) => {
        u64
    };
    (59) => {
        u64
    };
    (60) => {
        u64
    };
    (61) => {
        u64
    };
    (62) => {
        u64
    };
    (63) => {
        u64
    };
    (64) => {
        u64
    };
}

/// Generates a bitfield struct wrapping `Bits<N, S>` with
/// `Bits<W, S>`-typed accessors and setters per named sub-range.
///
/// Accepts an optional `<S: Strategy>` after the struct name; default
/// is `Hot`. Cold (which mirrors Hot's container projection across
/// every cell) is also supported. Warm and Precise are not yet
/// supported because the internal container-projection helper is
/// keyed on Hot's table; future round generalises.
///
/// ```ignore
/// // Default Hot
/// bitfield! { pub struct StrHandle: 32 { id: 28 at 0, flags: 4 at 28 } }
///
/// // Explicit Cold for column-store bitpacked layouts
/// bitfield! { pub struct EntityFlags<Cold>: 32 { active: 1 at 0, hidden: 1 at 1 } }
/// ```
///
/// See the module-level docs for grammar and examples.
#[macro_export]
macro_rules! bitfield {
    // Explicit strategy arm: `pub struct Foo<S>: N { ... }`
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident<$strategy:ty>: $n:tt {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_bits:tt at $lo:tt
            ),* $(,)?
        }
    ) => {
        $crate::__bitfield_impl! {
            $(#[$struct_attr])*
            $vis struct $name<$strategy>: $n {
                $(
                    $(#[$field_attr])*
                    $field: $field_bits at $lo
                ),*
            }
        }
    };

    // Default arm: `pub struct Foo: N { ... }` (Hot)
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident: $n:tt {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_bits:tt at $lo:tt
            ),* $(,)?
        }
    ) => {
        $crate::__bitfield_impl! {
            $(#[$struct_attr])*
            $vis struct $name<$crate::Hot>: $n {
                $(
                    $(#[$field_attr])*
                    $field: $field_bits at $lo
                ),*
            }
        }
    };
}

/// Internal expansion shared between the default and explicit-strategy
/// arms of the public `bitfield!` macro. Not part of the public surface.
#[doc(hidden)]
#[macro_export]
macro_rules! __bitfield_impl {
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident<$strategy:ty>: $n:tt {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_bits:tt at $lo:tt
            ),* $(,)?
        }
    ) => {
        $(#[$struct_attr])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Default)]
        $vis struct $name($crate::Bits<$n, $strategy>);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl core::cmp::Eq for $name {}

        impl core::hash::Hash for $name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        // Round 5 (#315): const-callable equality and default routed
        // through the inner `Bits<N, S>`. Parallel to the std impls
        // above; does not replace them.
        impl const $crate::ConstPartialEq for $name {
            fn const_eq(&self, other: &Self) -> $crate::Bool {
                <$crate::Bits<$n, $strategy> as $crate::ConstPartialEq>::const_eq(&self.0, &other.0)
            }
        }

        impl const $crate::ConstEq for $name {}

        impl const $crate::ConstDefault for $name {
            fn const_default() -> Self {
                Self(<$crate::Bits<$n, $strategy> as $crate::Identity<$crate::Additive>>::IDENTITY)
            }
        }

        impl $name {
            const _BOUNDS: () = {
                // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: compile-time bounds checks on macro input; the assertions themselves are arithmetic on the declared literals; tracked: #127
                assert!($n <= 64, "bitfield N must be <= 64");
                $(
                    assert!(
                        ($lo as u16) + ($field_bits as u16) <= ($n as u16),
                        concat!("sub-range ", stringify!($field), " does not fit within N bits"),
                    );
                )*
            };

            /// Total bit width of this bitfield.
            pub const BITS: u16 = $n;

            /// Zero-initialise.
            pub const fn new() -> Self {
                let _ = Self::_BOUNDS;
                Self($crate::Bits::<$n, $strategy>::from_raw(0 as $crate::__bitfield_container_ty!($n)))
            }

            /// Wrap a pre-built `Bits<N, Hot>` value.
            pub const fn from_bits(raw: $crate::Bits<$n, $strategy>) -> Self {
                let _ = Self::_BOUNDS;
                Self(raw)
            }

            /// Project to the underlying `Bits<N, Hot>`.
            pub const fn to_bits(self) -> $crate::Bits<$n, $strategy> {
                self.0
            }

            $(
                $(#[$field_attr])*
                pub const ${concat($field, _MASK)}: $crate::Bits<$n, $strategy> = {
                    // Round 5 (#315): mask construction routed through
                    // `BitPrim::mask_low` on the dispatched container.
                    // The saturating-at-WIDTH semantic of `mask_low`
                    // absorbs the prior `if width == container { MAX }`
                    // branch.
                    let field_mask = <$crate::__bitfield_container_ty!($n) as $crate::BitPrim>::mask_low($crate::USize($field_bits as usize));
                    let parent_mask = <$crate::__bitfield_container_ty!($n) as $crate::BitPrim>::mask_low($crate::USize($n as usize));
                    let shifted = (field_mask << $lo) & parent_mask;
                    $crate::Bits::<$n, $strategy>::from_raw(shifted)
                };

                $(#[$field_attr])*
                pub const fn $field(self) -> $crate::Bits<$field_bits, $strategy> {
                    // Round 5 (#315): extract via container-typed shift
                    // and `BitPrim::mask_low`. The narrow cast at the
                    // boundary stays per D-7 (field container may be
                    // narrower than parent container).
                    let raw_typed: $crate::__bitfield_container_ty!($n) = self.0.to_raw();
                    let mask = <$crate::__bitfield_container_ty!($n) as $crate::BitPrim>::mask_low($crate::USize($field_bits as usize));
                    let shifted = (raw_typed >> $lo) & mask;
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: parent-to-field container narrowing at the bitfield boundary; mask precondition enforces zero high bits per D-7; tracked: #256
                    $crate::Bits::<$field_bits, $strategy>::from_raw(shifted as $crate::__bitfield_container_ty!($field_bits))
                }

                $(#[$field_attr])*
                pub const fn ${concat(with_, $field)}(
                    self,
                    value: $crate::Bits<$field_bits, $strategy>,
                ) -> Self {
                    // Round 5 (#315): clear-and-set via container-typed
                    // operations. Field value widens through `as` cast
                    // (sound: field container is no wider than parent).
                    let parent_typed: $crate::__bitfield_container_ty!($n) = self.0.to_raw();
                    let value_typed: $crate::__bitfield_container_ty!($field_bits) = value.to_raw();
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: field-to-parent container widening for shift-and-or in parent width; lossless by D-7's $field_bits + $lo <= $n invariant; tracked: #256
                    let value_widened: $crate::__bitfield_container_ty!($n) = value_typed as $crate::__bitfield_container_ty!($n);
                    let field_mask = <$crate::__bitfield_container_ty!($n) as $crate::BitPrim>::mask_low($crate::USize($field_bits as usize));
                    let parent_mask = <$crate::__bitfield_container_ty!($n) as $crate::BitPrim>::mask_low($crate::USize($n as usize));
                    let in_place_mask = field_mask << $lo;
                    let cleared = parent_typed & !in_place_mask;
                    let masked_value = value_widened & field_mask;
                    let shifted_value = masked_value << $lo;
                    let combined = (cleared | shifted_value) & parent_mask;
                    Self($crate::Bits::<$n, $strategy>::from_raw(combined))
                }
            )*
        }
    };
}
