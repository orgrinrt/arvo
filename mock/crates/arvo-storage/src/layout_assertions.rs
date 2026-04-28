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
//! - `MultiContainer<HiT, LoT>` uses `repr(C)` two-field layout.
//!   The assertions pin size lower-bound and alignment.
//!
//! The module is private (no `pub use`); assertions fire at compile
//! time without exporting any surface. Adding a new transparency
//! invariant elsewhere in the crate should add a corresponding
//! assertion here.
//!
//! Round 202604280841 origin: insurance for the Round C MetaCarrier
//! workaround and follow-on transparency chains.

use arvo_strategy::{
    Bitpacked, Cold, ContainerWidth, Dense, DoubleLogical, HasAxes, Hot, Min, MultiContainer,
    OverflowPolicy, Precise, Saturating, Signed, StorageLayout, Unsigned, Warm, Wrapping,
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

// --- Tier 2: Bits projection — Hot, unsigned ------------------------------

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
assert_layout_eq!(Bits<129, Hot, Unsigned>, MultiContainer<u64, u128>);
assert_layout_eq!(Bits<192, Hot, Unsigned>, MultiContainer<u64, u128>);
assert_layout_eq!(Bits<193, Hot, Unsigned>, MultiContainer<u128, u128>);
assert_layout_eq!(Bits<255, Hot, Unsigned>, MultiContainer<u128, u128>);

// --- Tier 2: Bits projection — Hot, signed --------------------------------

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
assert_layout_eq!(Bits<129, Hot, Signed>, MultiContainer<i64, i128>);
assert_layout_eq!(Bits<192, Hot, Signed>, MultiContainer<i64, i128>);
assert_layout_eq!(Bits<193, Hot, Signed>, MultiContainer<i128, i128>);
assert_layout_eq!(Bits<255, Hot, Signed>, MultiContainer<i128, i128>);

// --- Tier 2: Bits projection — Cold mirrors Hot ---------------------------

assert_layout_eq!(Bits<1, Cold, Unsigned>, u8);
assert_layout_eq!(Bits<8, Cold, Unsigned>, u8);
assert_layout_eq!(Bits<64, Cold, Unsigned>, u64);
assert_layout_eq!(Bits<65, Cold, Unsigned>, u128);
assert_layout_eq!(Bits<128, Cold, Unsigned>, u128);
assert_layout_eq!(Bits<129, Cold, Unsigned>, MultiContainer<u64, u128>);
assert_layout_eq!(Bits<255, Cold, Unsigned>, MultiContainer<u128, u128>);

assert_layout_eq!(Bits<8, Cold, Signed>, i8);
assert_layout_eq!(Bits<64, Cold, Signed>, i64);
assert_layout_eq!(Bits<128, Cold, Signed>, i128);
assert_layout_eq!(Bits<129, Cold, Signed>, MultiContainer<i64, i128>);
assert_layout_eq!(Bits<255, Cold, Signed>, MultiContainer<i128, i128>);

// --- Tier 2: Bits projection — Warm (2x logical, caps at 64) --------------

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

// --- Tier 2: Bits projection — Precise ------------------------------------
//
// Precise mirrors Warm at 1..=32 (2x doubled) and stays at u64/i64
// at 33..=64 with saturating ops (the 2x rule was never strictly
// enforced at this band in pre-Round-B source; pre-existing semantics
// quirk inherited per Round B notes).

assert_layout_eq!(Bits<1, Precise, Unsigned>, u16);
assert_layout_eq!(Bits<8, Precise, Unsigned>, u16);
assert_layout_eq!(Bits<9, Precise, Unsigned>, u32);
assert_layout_eq!(Bits<16, Precise, Unsigned>, u32);
assert_layout_eq!(Bits<17, Precise, Unsigned>, u64);
assert_layout_eq!(Bits<32, Precise, Unsigned>, u64);
assert_layout_eq!(Bits<33, Precise, Unsigned>, u64);
assert_layout_eq!(Bits<64, Precise, Unsigned>, u64);

assert_layout_eq!(Bits<1, Precise, Signed>, i16);
assert_layout_eq!(Bits<8, Precise, Signed>, i16);
assert_layout_eq!(Bits<32, Precise, Signed>, i64);
assert_layout_eq!(Bits<33, Precise, Signed>, i64);
assert_layout_eq!(Bits<64, Precise, Signed>, i64);

// --- Tier 4: MultiContainer layout invariants -----------------------------
//
// `repr(C)` lays the struct out as `(hi, lo)` with field order. Alignment
// is `max(align_of::<HiT>(), align_of::<LoT>())`. Size is
// `align_up(align_up(size_of::<HiT>(), align_of::<LoT>()) +
// size_of::<LoT>(), struct_align)`. For our canonical pairs:
//   `(u64, u128)`: u64 at offset 0 (8 bytes), 8 bytes padding to align 16,
//     u128 at offset 16 (16 bytes), total 32, align 16.
//   `(u128, u128)`: u128 at offset 0 (16), u128 at offset 16 (16), total
//     32, align 16.
//   Signed mirrors match.

const _: () = {
    // Size lower bound: at least the sum of two halves (padding may make
    // actual size larger; `>=` catches any drift that loses bytes).
    assert!(
        core::mem::size_of::<MultiContainer<u64, u128>>()
            >= core::mem::size_of::<u64>() + core::mem::size_of::<u128>(),
        "MultiContainer<u64, u128> size drift: under sum of halves",
    );
    assert!(
        core::mem::size_of::<MultiContainer<u128, u128>>()
            >= core::mem::size_of::<u128>() + core::mem::size_of::<u128>(),
        "MultiContainer<u128, u128> size drift: under sum of halves",
    );
    assert!(
        core::mem::size_of::<MultiContainer<i64, i128>>()
            >= core::mem::size_of::<i64>() + core::mem::size_of::<i128>(),
        "MultiContainer<i64, i128> size drift: under sum of halves",
    );
    assert!(
        core::mem::size_of::<MultiContainer<i128, i128>>()
            >= core::mem::size_of::<i128>() + core::mem::size_of::<i128>(),
        "MultiContainer<i128, i128> size drift: under sum of halves",
    );
    // Alignment: pinned to LoT's alignment (the larger half in all our
    // canonical pairs).
    assert!(
        core::mem::align_of::<MultiContainer<u64, u128>>() == core::mem::align_of::<u128>(),
        "MultiContainer<u64, u128> alignment drift",
    );
    assert!(
        core::mem::align_of::<MultiContainer<u128, u128>>() == core::mem::align_of::<u128>(),
        "MultiContainer<u128, u128> alignment drift",
    );
    assert!(
        core::mem::align_of::<MultiContainer<i64, i128>>() == core::mem::align_of::<i128>(),
        "MultiContainer<i64, i128> alignment drift",
    );
    assert!(
        core::mem::align_of::<MultiContainer<i128, i128>>() == core::mem::align_of::<i128>(),
        "MultiContainer<i128, i128> alignment drift",
    );
    // Exact sizes for the canonical pairs (32 bytes for all four; computed
    // as align_up(8+16, 16) and align_up(16+16, 16) respectively).
    assert!(
        core::mem::size_of::<MultiContainer<u64, u128>>() == 32,
        "MultiContainer<u64, u128> size drift: expected 32",
    );
    assert!(
        core::mem::size_of::<MultiContainer<u128, u128>>() == 32,
        "MultiContainer<u128, u128> size drift: expected 32",
    );
    assert!(
        core::mem::size_of::<MultiContainer<i64, i128>>() == 32,
        "MultiContainer<i64, i128> size drift: expected 32",
    );
    assert!(
        core::mem::size_of::<MultiContainer<i128, i128>>() == 32,
        "MultiContainer<i128, i128> size drift: expected 32",
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
