//! Compile-time wrapper layout assertions for `UFixed` and `IFixed`.
//!
//! Each strategy primitive in arvo is `repr(transparent)` over a Bits
//! shape. The bottom of that chain projects to a native primitive via
//! `BitsContainerFor`. The Rust type system can't cross-check that
//! the wrapper, the Bits intermediate, and the native primitive all
//! share the same layout end-to-end.
//!
//! These assertions pin the chain at representative cells. If a
//! future round changes the Bits projection or the wrapper's
//! transparency, the build breaks at compile time before any runtime
//! soundness violation could occur via `UFixed::from_raw` /
//! `to_raw` transmutes.
//!
//! arvo-storage's `layout_assertions.rs` covers the lower-level
//! `Bits` ↔ projection target chain; this module covers the wrapper
//! ↔ Bits chain.
//!
//! The module is private (no `pub use`); assertions fire at compile
//! time without exporting any surface.

use arvo_storage::{Bits, fbits, ibits};
use arvo_strategy::{Cold, Hot, Signed, Unsigned, Warm};

use crate::ifixed::IFixed;
use crate::ufixed::UFixed;

/// Assert that two types have identical size and alignment at compile
/// time. Fails the build on mismatch; the error location pinpoints the
/// failing assertion site, which is more useful than embedding
/// stringified type names into a format-string-interpreted panic
/// message (type names like `UFixed<{ ibits(8) }, ...>` contain `{`
/// which trips the format-string parser inside `assert!`).
macro_rules! assert_layout_eq {
    ($a:ty, $b:ty $(,)?) => {
        const _: () = {
            assert!(
                core::mem::size_of::<$a>() == core::mem::size_of::<$b>(),
                "layout drift: size_of mismatch (see file:line for offending pair)",
            );
            assert!(
                core::mem::align_of::<$a>() == core::mem::align_of::<$b>(),
                "layout drift: align_of mismatch (see file:line for offending pair)",
            );
        };
    };
}

// --- UFixed: wrapper ↔ Bits + Bits ↔ primitive ---------------------------
//
// 8-bit at Hot: u8 native.
assert_layout_eq!(UFixed<{ ibits(8) }, { fbits(0) }, Hot>, u8);
assert_layout_eq!(UFixed<{ ibits(8) }, { fbits(0) }, Hot>, Bits<8, Hot, Unsigned>);

// 16-bit at Hot via I+F: u16 native.
assert_layout_eq!(UFixed<{ ibits(13) }, { fbits(3) }, Hot>, u16);

// 8-bit at Warm: u16 (2x).
assert_layout_eq!(UFixed<{ ibits(8) }, { fbits(0) }, Warm>, u16);

// 8-bit at Cold: u8 (Cold = Min container).
assert_layout_eq!(UFixed<{ ibits(8) }, { fbits(0) }, Cold>, u8);

// 32-bit at Hot.
assert_layout_eq!(UFixed<{ ibits(32) }, { fbits(0) }, Hot>, u32);

// 64-bit at Hot.
assert_layout_eq!(UFixed<{ ibits(64) }, { fbits(0) }, Hot>, u64);

// --- IFixed: 1+I+F logical width ----------------------------------------
//
// IFixed reserves one bit for sign; logical width = 1 + I + F.
// 1+7+0 = 8 → i8.
assert_layout_eq!(IFixed<{ ibits(7) }, { fbits(0) }, Hot>, i8);
assert_layout_eq!(IFixed<{ ibits(7) }, { fbits(0) }, Hot>, Bits<8, Hot, Signed>);

// 1+15+0 = 16 → i16.
assert_layout_eq!(IFixed<{ ibits(15) }, { fbits(0) }, Hot>, i16);

// 1+31+0 = 32 → i32.
assert_layout_eq!(IFixed<{ ibits(31) }, { fbits(0) }, Hot>, i32);

// 1+63+0 = 64 → i64.
assert_layout_eq!(IFixed<{ ibits(63) }, { fbits(0) }, Hot>, i64);

// 1+7+0 = 8 at Warm → 2x → i16.
assert_layout_eq!(IFixed<{ ibits(7) }, { fbits(0) }, Warm>, i16);
