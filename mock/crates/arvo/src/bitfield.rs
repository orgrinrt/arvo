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
/// a `Bits<N, Hot>` width literal. Mirrors `<Hot as UContainerFor<N>>::T`
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
    (1) => { u8 }; (2) => { u8 }; (3) => { u8 }; (4) => { u8 };
    (5) => { u8 }; (6) => { u8 }; (7) => { u8 }; (8) => { u8 };
    (9) => { u16 }; (10) => { u16 }; (11) => { u16 }; (12) => { u16 };
    (13) => { u16 }; (14) => { u16 }; (15) => { u16 }; (16) => { u16 };
    (17) => { u32 }; (18) => { u32 }; (19) => { u32 }; (20) => { u32 };
    (21) => { u32 }; (22) => { u32 }; (23) => { u32 }; (24) => { u32 };
    (25) => { u32 }; (26) => { u32 }; (27) => { u32 }; (28) => { u32 };
    (29) => { u32 }; (30) => { u32 }; (31) => { u32 }; (32) => { u32 };
    (33) => { u64 }; (34) => { u64 }; (35) => { u64 }; (36) => { u64 };
    (37) => { u64 }; (38) => { u64 }; (39) => { u64 }; (40) => { u64 };
    (41) => { u64 }; (42) => { u64 }; (43) => { u64 }; (44) => { u64 };
    (45) => { u64 }; (46) => { u64 }; (47) => { u64 }; (48) => { u64 };
    (49) => { u64 }; (50) => { u64 }; (51) => { u64 }; (52) => { u64 };
    (53) => { u64 }; (54) => { u64 }; (55) => { u64 }; (56) => { u64 };
    (57) => { u64 }; (58) => { u64 }; (59) => { u64 }; (60) => { u64 };
    (61) => { u64 }; (62) => { u64 }; (63) => { u64 }; (64) => { u64 };
}

/// Generates a bitfield struct wrapping `Bits<N, Hot>` with
/// `Bits<W, Hot>`-typed accessors and setters per named sub-range.
///
/// See the module-level docs for grammar and examples.
#[macro_export]
macro_rules! bitfield {
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident: $n:tt {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_bits:tt at $lo:tt
            ),* $(,)?
        }
    ) => {
        $(#[$struct_attr])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Default)]
        $vis struct $name($crate::Bits<$n, $crate::Hot>);

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
            pub const BITS: u8 = $n;

            /// Zero-initialise.
            pub const fn new() -> Self {
                let _ = Self::_BOUNDS;
                Self($crate::Bits::<$n, $crate::Hot>::from_raw(0 as $crate::__bitfield_container_ty!($n)))
            }

            /// Wrap a pre-built `Bits<N, Hot>` value.
            pub const fn from_bits(raw: $crate::Bits<$n, $crate::Hot>) -> Self {
                let _ = Self::_BOUNDS;
                Self(raw)
            }

            /// Project to the underlying `Bits<N, Hot>`.
            pub const fn to_bits(self) -> $crate::Bits<$n, $crate::Hot> {
                self.0
            }

            $(
                $(#[$field_attr])*
                pub const ${concat($field, _MASK)}: $crate::Bits<$n, $crate::Hot> = {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pre-shifted mask at parent width, computed from macro-expanded sub-range literals; container truncation lives at the bitfield boundary per D-7; tracked: #256
                    let mask: u64 =
                        if $field_bits == 64 { u64::MAX }
                        else { (1u64 << $field_bits) - 1 };
                    let parent_mask: u64 =
                        if $n == 64 { u64::MAX }
                        else { (1u64 << $n) - 1 };
                    let shifted = (mask << $lo) & parent_mask;
                    $crate::Bits::<$n, $crate::Hot>::from_raw(shifted as $crate::__bitfield_container_ty!($n))
                };

                $(#[$field_attr])*
                pub const fn $field(self) -> $crate::Bits<$field_bits, $crate::Hot> {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-field shift/mask in u64; final cast narrows to the dispatched container under the mask precondition per D-7; tracked: #256
                    let raw_typed: $crate::__bitfield_container_ty!($n) = self.0.to_raw();
                    let raw: u64 = raw_typed as u64;
                    let mask: u64 =
                        if $field_bits == 64 { u64::MAX }
                        else { (1u64 << $field_bits) - 1 };
                    let shifted = (raw >> $lo) & mask;
                    $crate::Bits::<$field_bits, $crate::Hot>::from_raw(shifted as $crate::__bitfield_container_ty!($field_bits))
                }

                $(#[$field_attr])*
                pub const fn ${concat(with_, $field)}(
                    self,
                    value: $crate::Bits<$field_bits, $crate::Hot>,
                ) -> Self {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-field shift/mask in u64; final cast narrows to the parent container under the mask precondition per D-7; tracked: #256
                    let parent_typed: $crate::__bitfield_container_ty!($n) = self.0.to_raw();
                    let value_typed: $crate::__bitfield_container_ty!($field_bits) = value.to_raw();
                    let parent_raw: u64 = parent_typed as u64;
                    let value_raw: u64 = value_typed as u64;
                    let field_mask: u64 =
                        if $field_bits == 64 { u64::MAX }
                        else { (1u64 << $field_bits) - 1 };
                    let parent_mask: u64 =
                        if $n == 64 { u64::MAX }
                        else { (1u64 << $n) - 1 };
                    let in_place_mask: u64 = field_mask << $lo;
                    let cleared = parent_raw & !in_place_mask;
                    let masked_value = value_raw & field_mask;
                    let shifted_value = masked_value << $lo;
                    let combined = (cleared | shifted_value) & parent_mask;
                    Self($crate::Bits::<$n, $crate::Hot>::from_raw(combined as $crate::__bitfield_container_ty!($n)))
                }
            )*
        }
    };
}
