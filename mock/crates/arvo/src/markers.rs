//! Type-family marker traits.
//!
//! Five markers classify arvo's type families. Used by arvo-bits for
//! blanket impls and by consumers for generic constraints.
//!
//! | Marker            | Types                              |
//! |-------------------|------------------------------------|
//! | `IntegerLike`     | UFixed / IFixed with `F = 0`       |
//! | `FractionLike`    | UFixed / IFixed with `F > 0`       |
//! | `BitPresentation` | all UFixed, all IFixed             |
//! | `FloatLike`       | FastFloat, StrictFloat             |
//! | `BoolLike`        | Bool                               |
//!
//! Concrete impls for these markers live alongside the types they
//! classify (see `ufixed`, `ifixed`, `float`, `newtype`).

use arvo_storage::{Bool, FBits, IBits, USize};
use crate::strategy::Hot;

/// Marker for types that behave like whole numbers.
///
/// UFixed / IFixed with `F = 0`. No fractional part.
pub trait IntegerLike: Copy {}

/// Marker for types that carry a fractional component.
///
/// UFixed / IFixed with `F > 0`.
pub trait FractionLike: Copy {}

/// Marker for types whose raw bits are accessible.
///
/// Every UFixed and IFixed implements this. `LOGICAL_WIDTH` is the
/// bit count addressable through `BitAccess` in arvo-bits. Container
/// size (= physical width) may be larger than `LOGICAL_WIDTH`
/// depending on the strategy.
pub trait BitPresentation: Copy {
    /// Logical bit width of this type.
    ///
    /// For `UFixed<I, F, S>`: `I + F`.
    /// For `IFixed<I, F, S>`: `1 + I + F` (sign bit + magnitude).
    const LOGICAL_WIDTH: USize;
}

/// Marker for IEEE-float-wrapping types.
///
/// `FastFloat` and `StrictFloat`. These do not carry a `Strategy`
/// marker — float precision is defined by the IEEE width, not by
/// fixed-point container rules.
pub trait FloatLike: Copy {}

/// Marker for truth-value types.
///
/// Currently `Bool`. `pack()` converts a control-flow `Bool` into
/// a 1-bit data value. The returned type is the `UFixed<1, 0, Hot>`
/// shape that arvo-bits aliases as `Bit`.
pub trait BoolLike: Copy {
    /// Concrete 1-bit packed storage type produced by `pack`.
    ///
    /// Defined as an associated type so arvo L0 does not need to
    /// name `arvo-bits::Bit` directly; arvo-bits pins this to its
    /// own `Bit = UFixed<{ibits(1)}, {fbits(0)}, Hot>` alias.
    type Packed: BitPresentation;

    /// Convert a truth value to a 1-bit data value.
    fn pack(self) -> Self::Packed;
}

// --- Bool conversion -------------------------------------------------------
//
// `Bool: BoolLike` but `pack()` needs a UFixed<1, 0, Hot>. The concrete
// type is produced via the container trait; arvo-bits re-exports the
// Bit alias. Here we only need the value 0/1 mapping.
//
// The packed type is `UFixed<{ibits(1)}, {fbits(0)}, Hot>` whose
// storage container is `u8` (Hot, 1-bit-range bucket).

// `Bool` blanket impl is placed in this module (not `newtype`) to keep
// the marker-trait blanket-impl surface together. The concrete packed
// type is forward-declared: arvo-bits aliases it as `Bit`.
use crate::ufixed::UFixed;

impl BoolLike for Bool {
    type Packed = UFixed<{ IBits::ONE }, { FBits::ZERO }, Hot>;

    #[inline(always)]
    fn pack(self) -> Self::Packed {
        // Hot container for 1 logical bit is `u8`. `bool as u8` is
        // 0 or 1, inside the 1-bit range.
        UFixed::<{ IBits::ONE }, { FBits::ZERO }, Hot>::from_raw(self.0 as u8)
    }
}
