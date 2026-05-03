//! Compile-time layout-equivalence guards for the substrate's
//! `repr(transparent)` chains and the `MetaCarrier` transmute.
//!
//! Every assertion in this module is a compile-time invariant the
//! type system cannot verify on its own:
//!
//! - `MetaCarrier::as_bits()` transmutes between `MetaCarrier` and
//!   `Bits<9, Hot, Unsigned>`. Soundness depends on layout
//!   equivalence; a future projection change at (Hot, 9, Unsigned)
//!   would silently break the transmute. The assertions below catch
//!   any drift at compile time.
//! - Every `Bits<N, S, Sign>` is `repr(transparent)` over its
//!   `BitsContainerFor<N, Sign>` projection target. The assertions
//!   pin this at boundary cells.
//! - `WideBits<BYTES, A>` uses `repr(C)` over `[A; 0]` + `[u8; BYTES]`.
//!   The assertions pin size and alignment per `A` marker.
//!
//! The module is private (no `pub use`); assertions fire at compile
//! time without exporting any surface. Adding a new transparency
//! invariant elsewhere in the crate should add a corresponding
//! assertion here.
//!
//! Round 202604280841 origin: insurance for the Round C MetaCarrier
//! workaround and follow-on transparency chains.

use arvo_strategy::{
    A1, A16, Bitpacked, Cold, ContainerWidth, Dense, DoubleLogical, HasAxes, Hot, Min,
    OverflowPolicy, Precise, Saturating, Signed, StorageLayout, Unsigned, Warm, WideBits, Wrapping,
};

use crate::{Bits, FBits, IBits, MetaCarrier, Width};

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

// --- Tier 1: meta-layer carrier (load-bearing for as_bits transmute) ------

assert_layout_eq!(MetaCarrier, u16);
assert_layout_eq!(MetaCarrier, Bits<9, Hot, Unsigned>);

assert_layout_eq!(IBits, MetaCarrier);
assert_layout_eq!(FBits, MetaCarrier);
assert_layout_eq!(Width, MetaCarrier);

// --- Tier 2: Bits projection: Hot, unsigned ------------------------------

assert_layout_eq!(Bits<1, Hot, Unsigned>, u8);
assert_layout_eq!(Bits<8, Hot, Unsigned>, u8);
assert_layout_eq!(Bits<9, Hot, Unsigned>, u16);
assert_layout_eq!(Bits<16, Hot, Unsigned>, u16);
assert_layout_eq!(Bits<17, Hot, Unsigned>, u32);
assert_layout_eq!(Bits<32, Hot, Unsigned>, u32);
assert_layout_eq!(Bits<33, Hot, Unsigned>, u64);
assert_layout_eq!(Bits<64, Hot, Unsigned>, u64);
assert_layout_eq!(Bits<65, Hot, Unsigned>, u128);
assert_layout_eq!(Bits<128, Hot, Unsigned>, u128);
// Hot wide bucket projects to WideBits<bytes_for(N), A16>. The
// alignment baseline (A16) produces 32-byte physical size for
// 17..=32 byte payloads, matching the canonical pre-redesign
// envelope at the same width. Audit-validated by sketch 01.
assert_layout_eq!(Bits<129, Hot, Unsigned>, WideBits<17, A16>);
assert_layout_eq!(Bits<192, Hot, Unsigned>, WideBits<24, A16>);
assert_layout_eq!(Bits<193, Hot, Unsigned>, WideBits<25, A16>);
assert_layout_eq!(Bits<255, Hot, Unsigned>, WideBits<32, A16>);

// --- Tier 2: Bits projection: Hot, signed --------------------------------

assert_layout_eq!(Bits<1, Hot, Signed>, i8);
assert_layout_eq!(Bits<8, Hot, Signed>, i8);
assert_layout_eq!(Bits<9, Hot, Signed>, i16);
assert_layout_eq!(Bits<16, Hot, Signed>, i16);
assert_layout_eq!(Bits<17, Hot, Signed>, i32);
assert_layout_eq!(Bits<32, Hot, Signed>, i32);
assert_layout_eq!(Bits<33, Hot, Signed>, i64);
assert_layout_eq!(Bits<64, Hot, Signed>, i64);
assert_layout_eq!(Bits<65, Hot, Signed>, i128);
assert_layout_eq!(Bits<128, Hot, Signed>, i128);
// Hot wide bucket signed mirrors unsigned (Sign axis is structurally
// transparent at the wide storage level; bit interpretation lives in
// arvo-bits-contracts BitPrim impls).
assert_layout_eq!(Bits<129, Hot, Signed>, WideBits<17, A16>);
assert_layout_eq!(Bits<192, Hot, Signed>, WideBits<24, A16>);
assert_layout_eq!(Bits<193, Hot, Signed>, WideBits<25, A16>);
assert_layout_eq!(Bits<255, Hot, Signed>, WideBits<32, A16>);

// --- Tier 2: Bits projection: Cold mirrors Hot ---------------------------

assert_layout_eq!(Bits<1, Cold, Unsigned>, u8);
assert_layout_eq!(Bits<8, Cold, Unsigned>, u8);
assert_layout_eq!(Bits<64, Cold, Unsigned>, u64);
assert_layout_eq!(Bits<65, Cold, Unsigned>, u128);
assert_layout_eq!(Bits<128, Cold, Unsigned>, u128);
// Cold wide bucket projects to WideBits<bytes_for(N), A1>: align-1
// byte-exact (Cold is bitpacked column-store; alignment on the
// per-value container is not load-bearing).
assert_layout_eq!(Bits<129, Cold, Unsigned>, WideBits<17, A1>);
assert_layout_eq!(Bits<255, Cold, Unsigned>, WideBits<32, A1>);

assert_layout_eq!(Bits<8, Cold, Signed>, i8);
assert_layout_eq!(Bits<64, Cold, Signed>, i64);
assert_layout_eq!(Bits<128, Cold, Signed>, i128);
assert_layout_eq!(Bits<129, Cold, Signed>, WideBits<17, A1>);
assert_layout_eq!(Bits<255, Cold, Signed>, WideBits<32, A1>);

// --- Tier 2: Bits projection: Warm (2x logical, caps at 64) --------------

assert_layout_eq!(Bits<1, Warm, Unsigned>, u16);
assert_layout_eq!(Bits<8, Warm, Unsigned>, u16);
assert_layout_eq!(Bits<9, Warm, Unsigned>, u32);
assert_layout_eq!(Bits<16, Warm, Unsigned>, u32);
assert_layout_eq!(Bits<17, Warm, Unsigned>, u64);
assert_layout_eq!(Bits<32, Warm, Unsigned>, u64);
assert_layout_eq!(Bits<33, Warm, Unsigned>, u128);
assert_layout_eq!(Bits<64, Warm, Unsigned>, u128);

assert_layout_eq!(Bits<1, Warm, Signed>, i16);
assert_layout_eq!(Bits<8, Warm, Signed>, i16);
assert_layout_eq!(Bits<32, Warm, Signed>, i64);
assert_layout_eq!(Bits<33, Warm, Signed>, i128);
assert_layout_eq!(Bits<64, Warm, Signed>, i128);

// --- Tier 2: Bits projection: Precise ------------------------------------
//
// Precise mirrors Warm's 2x-logical contract across the entire 1..=64
// range. Round 202604281000 Pass D promoted Precise 33..=64 from u64
// to u128 (signed: i64 to i128) to honor the 2x rule consistently;
// pre-Pass-D Precise inherited a u64 cell at 33..=64 that violated the
// 2x contract relative to Warm's u128 at the same band.

assert_layout_eq!(Bits<1, Precise, Unsigned>, u16);
assert_layout_eq!(Bits<8, Precise, Unsigned>, u16);
assert_layout_eq!(Bits<9, Precise, Unsigned>, u32);
assert_layout_eq!(Bits<16, Precise, Unsigned>, u32);
assert_layout_eq!(Bits<17, Precise, Unsigned>, u64);
assert_layout_eq!(Bits<32, Precise, Unsigned>, u64);
assert_layout_eq!(Bits<33, Precise, Unsigned>, u128);
assert_layout_eq!(Bits<64, Precise, Unsigned>, u128);

assert_layout_eq!(Bits<1, Precise, Signed>, i16);
assert_layout_eq!(Bits<8, Precise, Signed>, i16);
assert_layout_eq!(Bits<32, Precise, Signed>, i64);
assert_layout_eq!(Bits<33, Precise, Signed>, i128);
assert_layout_eq!(Bits<64, Precise, Signed>, i128);

// --- Tier 4: WideBits layout invariants -----------------------------------
//
// `WideBits<BYTES, A>` is `repr(C)` over `([A; 0], [u8; BYTES])`. The
// zero-sized `[A; 0]` marker pins alignment to `A::VALUE`; the byte
// array follows immediately.
//
// `WideBits<BYTES, A1>`: align-1, exact size BYTES.
// `WideBits<BYTES, A16>`: align-16, size = align_up(BYTES, 16).
// `WideBits<BYTES, A32>`: align-32, size = align_up(BYTES, 32). (#320)
// `WideBits<BYTES, A64>`: align-64, size = align_up(BYTES, 64). (#320)

const _: () = {
    // A1: align-1 byte-exact, size = BYTES.
    assert!(
        core::mem::size_of::<WideBits<17, A1>>() == 17,
        "WideBits<17, A1> size drift: expected 17 bytes (align-1)",
    );
    assert!(
        core::mem::size_of::<WideBits<32, A1>>() == 32,
        "WideBits<32, A1> size drift: expected 32 bytes (align-1)",
    );
    assert!(
        core::mem::align_of::<WideBits<17, A1>>() == 1,
        "WideBits<17, A1> alignment drift: expected 1",
    );
    assert!(
        core::mem::align_of::<WideBits<32, A1>>() == 1,
        "WideBits<32, A1> alignment drift: expected 1",
    );

    // A16: align-16, size = align_up(BYTES, 16). The Hot baseline
    // tier covering SSE2 / NEON. Hits the same 32-byte physical
    // envelope at 17..=32 byte payloads as the pre-redesign
    // heterogeneous-pair shape, with an explicit alignment marker
    // rather than relying on a larger-half alignment leak.
    assert!(
        core::mem::size_of::<WideBits<17, A16>>() == 32,
        "WideBits<17, A16> size drift: expected 32 bytes (align_up(17, 16))",
    );
    assert!(
        core::mem::size_of::<WideBits<32, A16>>() == 32,
        "WideBits<32, A16> size drift: expected 32 bytes (align_up(32, 16))",
    );
    assert!(
        core::mem::size_of::<WideBits<24, A16>>() == 32,
        "WideBits<24, A16> size drift: expected 32 bytes (align_up(24, 16))",
    );
    assert!(
        core::mem::align_of::<WideBits<17, A16>>() == 16,
        "WideBits<17, A16> alignment drift: expected 16",
    );
    assert!(
        core::mem::align_of::<WideBits<32, A16>>() == 16,
        "WideBits<32, A16> alignment drift: expected 16",
    );
};

// --- Tier 5: HasAxes correctness (round 202604281000 Pass A.1) ------------
//
// Cross-check that each canonical strategy's `HasAxes` projection
// matches the documented axis combination. The discriminants live on
// the axis sub-markers (Wrapping=0/Saturating=1, Min=0/DoubleLogical=1,
// Dense=0/Bitpacked=1). Each strategy gets three asserts.

const _: () = {
    // Hot = Wrapping / Min / Dense
    assert!(<<Hot as HasAxes>::Overflow as OverflowPolicy>::DISCRIMINANT == Wrapping::DISCRIMINANT);
    assert!(<<Hot as HasAxes>::Width as ContainerWidth>::DISCRIMINANT == Min::DISCRIMINANT);
    assert!(<<Hot as HasAxes>::Layout as StorageLayout>::DISCRIMINANT == Dense::DISCRIMINANT);

    // Warm = Wrapping / DoubleLogical / Dense
    assert!(<<Warm as HasAxes>::Overflow as OverflowPolicy>::DISCRIMINANT == Wrapping::DISCRIMINANT);
    assert!(
        <<Warm as HasAxes>::Width as ContainerWidth>::DISCRIMINANT == DoubleLogical::DISCRIMINANT,
    );
    assert!(<<Warm as HasAxes>::Layout as StorageLayout>::DISCRIMINANT == Dense::DISCRIMINANT);

    // Cold = Wrapping / Min / Bitpacked
    assert!(<<Cold as HasAxes>::Overflow as OverflowPolicy>::DISCRIMINANT == Wrapping::DISCRIMINANT);
    assert!(<<Cold as HasAxes>::Width as ContainerWidth>::DISCRIMINANT == Min::DISCRIMINANT);
    assert!(<<Cold as HasAxes>::Layout as StorageLayout>::DISCRIMINANT == Bitpacked::DISCRIMINANT);

    // Precise = Saturating / DoubleLogical / Dense
    assert!(
        <<Precise as HasAxes>::Overflow as OverflowPolicy>::DISCRIMINANT
            == Saturating::DISCRIMINANT,
    );
    assert!(
        <<Precise as HasAxes>::Width as ContainerWidth>::DISCRIMINANT
            == DoubleLogical::DISCRIMINANT,
    );
    assert!(<<Precise as HasAxes>::Layout as StorageLayout>::DISCRIMINANT == Dense::DISCRIMINANT);
};

// --- Tier 6: ConstParamTy soundness audit (round 202604281000 Pass A.3) ---
//
// Every newtype that flows through const-generic position must satisfy
// `StructuralEq + Copy + Eq + ConstParamTy`. The bound check below
// fails to compile if any of the obligations regress on a future change.

const fn _assert_const_param_ty<T: core::marker::ConstParamTy_ + Copy + Eq>() {}

const _: () = {
    _assert_const_param_ty::<MetaCarrier>();
    _assert_const_param_ty::<IBits>();
    _assert_const_param_ty::<FBits>();
    _assert_const_param_ty::<Width>();
};

// --- Tier 7: Full-cell expansion (round 202604281000 Pass A.2) ------------
//
// Default builds keep the boundary-cell coverage above. CI nightly builds
// running with `--cfg arvo_full_layout_audit` open the full grid: every
// (Strategy, N, Sign) cell where the strategy implements
// `BitsContainerFor<N, Sign>`. Catches any mid-band projection drift the
// boundary set would miss. Compile-time impact is bounded by the cfg gate.
//
// To run the full audit:
//
// ```bash
// RUSTFLAGS='--cfg arvo_full_layout_audit' cargo check -p arvo-storage
// ```

#[cfg(arvo_full_layout_audit)]
mod full_audit {
    use super::*;

    macro_rules! cells_unsigned {
        ($strategy:ty, $primitive:ty, $($n:literal),+) => {
            $( assert_layout_eq!(Bits<$n, $strategy, Unsigned>, $primitive); )+
        };
    }

    macro_rules! cells_signed {
        ($strategy:ty, $primitive:ty, $($n:literal),+) => {
            $( assert_layout_eq!(Bits<$n, $strategy, Signed>, $primitive); )+
        };
    }

    // --- Hot, unsigned: every cell across 1..=255 ---
    cells_unsigned!(Hot, u8, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_unsigned!(Hot, u16, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_unsigned!(
        Hot, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_unsigned!(
        Hot, u64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
    cells_unsigned!(
        Hot, u128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128
    );

    // --- Hot, signed: matching span ---
    cells_signed!(Hot, i8, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_signed!(Hot, i16, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_signed!(
        Hot, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_signed!(
        Hot, i64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
    cells_signed!(
        Hot, i128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128
    );

    // --- Cold mirrors Hot at every cell ---
    cells_unsigned!(Cold, u8, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_unsigned!(Cold, u16, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_unsigned!(
        Cold, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_unsigned!(
        Cold, u64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
    cells_unsigned!(
        Cold, u128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128
    );
    cells_signed!(Cold, i8, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_signed!(Cold, i16, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_signed!(
        Cold, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_signed!(
        Cold, i64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
    cells_signed!(
        Cold, i128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128
    );

    // --- Warm: 2x logical width across 1..=64 ---
    cells_unsigned!(Warm, u16, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_unsigned!(Warm, u32, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_unsigned!(
        Warm, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_unsigned!(
        Warm, u128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
    cells_signed!(Warm, i16, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_signed!(Warm, i32, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_signed!(
        Warm, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_signed!(
        Warm, i128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );

    // --- Precise: same shape as Warm post-Pass-D ---
    cells_unsigned!(Precise, u16, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_unsigned!(Precise, u32, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_unsigned!(
        Precise, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_unsigned!(
        Precise, u128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
    cells_signed!(Precise, i16, 1, 2, 3, 4, 5, 6, 7, 8);
    cells_signed!(Precise, i32, 9, 10, 11, 12, 13, 14, 15, 16);
    cells_signed!(
        Precise, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
    cells_signed!(
        Precise, i128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
}
