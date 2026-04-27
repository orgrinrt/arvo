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

/// Generates a bitfield struct wrapping `Bits<N, Hot>` with
/// `Bits<W, Hot>`-typed accessors and setters per named sub-range.
///
/// See the module-level docs for grammar and examples.
#[macro_export]
macro_rules! bitfield {
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident: $n:literal {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_bits:literal at $lo:literal
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
                Self($crate::Bits::<$n, $crate::Hot>::new(0))
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
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pre-shifted mask at parent width, computed from macro-expanded sub-range literals; tracked: #127
                    let mask: u64 =
                        if $field_bits == 64 { u64::MAX }
                        else { (1u64 << $field_bits) - 1 };
                    $crate::Bits::<$n, $crate::Hot>::new(mask << $lo)
                };

                $(#[$field_attr])*
                pub const fn $field(self) -> $crate::Bits<$field_bits, $crate::Hot> {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-field shift/mask dispatches on macro-expanded literals; tracked: #127
                    let mask: u64 =
                        if $field_bits == 64 { u64::MAX }
                        else { (1u64 << $field_bits) - 1 };
                    let shifted = (self.0.bits() >> $lo) & mask;
                    $crate::Bits::<$field_bits, $crate::Hot>::new(shifted)
                }

                $(#[$field_attr])*
                pub const fn ${concat(with_, $field)}(
                    self,
                    value: $crate::Bits<$field_bits, $crate::Hot>,
                ) -> Self {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-field shift/mask dispatches on macro-expanded literals; tracked: #127
                    let field_mask: u64 =
                        if $field_bits == 64 { u64::MAX }
                        else { (1u64 << $field_bits) - 1 };
                    let in_place_mask: u64 = field_mask << $lo;
                    let cleared = self.0.bits() & !in_place_mask;
                    let masked_value = value.bits() & field_mask;
                    let shifted_value = masked_value << $lo;
                    Self($crate::Bits::<$n, $crate::Hot>::new(cleared | shifted_value))
                }
            )*
        }
    };
}
