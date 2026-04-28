//! Internal strategy axis decomposition.
//!
//! Round 202604280806 (Round C) decomposes the bundled `Strategy`
//! axis into three orthogonal sub-axes via sealed marker traits:
//!
//! - `OverflowPolicy`: how arithmetic handles overflow
//!   (`Wrapping` / `Saturating`).
//! - `ContainerWidth`: the strategy's container width relative to
//!   logical width (`Min` / `DoubleLogical`).
//! - `StorageLayout`: how multiple values pack in column storage
//!   (`Dense` / `Bitpacked`).
//!
//! The four canonical strategy markers (`Hot`, `Warm`, `Cold`,
//! `Precise`) implement specific axis combinations via the
//! `HasAxes` trait. Round C ships the axis traits as documentation
//! + future-proofing primitives; the arithmetic / widen / narrow
//! impls in `arith.rs` / `widen.rs` continue to dispatch via the
//! bundled `Strategy` marker.
//!
//! A follow-up round will refactor those impls to dispatch via the
//! axis traits, allowing future experimentation with mixed axes
//! (saturating bitpacked, wrapping double-logical with bitpacked,
//! etc.) without breaking the public surface.
//!
//! Per the `arvo-toolbox-not-policer` workspace rule, the public
//! surface is intentionally narrow: the four markers stay as the
//! consumer interface. The axis decomposition is internal.

use core::marker::ConstParamTy;

use crate::sealed;
use crate::{Cold, Hot, Precise, Warm};

// --- OverflowPolicy axis --------------------------------------------------

/// How arithmetic handles overflow.
///
/// Sealed marker trait. The two implementors are `Wrapping` and
/// `Saturating` ZST markers. Each carries a `DISCRIMINANT` const for
/// compile-time `HasAxes` cross-checks (Pass A.1 of round 202604281000).
pub trait OverflowPolicy: sealed::Sealed + Copy + Clone + Default + 'static {
    /// Stable per-marker discriminant for compile-time projection.
    const DISCRIMINANT: u8;
}

/// Overflow policy: arithmetic wraps modulo container width.
///
/// Used by `Hot`, `Warm`, `Cold`. LLVM emits `wrapping_*` ops which
/// vectorise freely.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Wrapping;

/// Overflow policy: arithmetic saturates at logical min/max.
///
/// Used by `Precise`. Overflow clamps rather than wraps; correctness
/// guaranteed at the cost of vectorisation in some cases.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Saturating;

impl sealed::Sealed for Wrapping {}
impl sealed::Sealed for Saturating {}

impl OverflowPolicy for Wrapping {
    const DISCRIMINANT: u8 = 0;
}
impl OverflowPolicy for Saturating {
    const DISCRIMINANT: u8 = 1;
}

// --- ContainerWidth axis --------------------------------------------------

/// Container width relative to logical width.
///
/// Sealed marker trait. The two implementors are `Min` and
/// `DoubleLogical` ZST markers. Each carries a `DISCRIMINANT` const
/// for compile-time `HasAxes` cross-checks.
pub trait ContainerWidth: sealed::Sealed + Copy + Clone + Default + 'static {
    /// Stable per-marker discriminant for compile-time projection.
    const DISCRIMINANT: u8;
}

/// Container width: minimum byte-aligned width that fits the logical
/// bit count.
///
/// Used by `Hot`, `Cold`. Storage is dense; arithmetic dispatches
/// at native widths (`u8` / `u16` / `u32` / `u64` / `u128`).
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Min;

/// Container width: 2x the logical bit width (one bucket up).
///
/// Used by `Warm`, `Precise`. A single `add` / `sub` / `mul` of two
/// values within their logical range cannot overflow the container.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct DoubleLogical;

impl sealed::Sealed for Min {}
impl sealed::Sealed for DoubleLogical {}

impl ContainerWidth for Min {
    const DISCRIMINANT: u8 = 0;
}
impl ContainerWidth for DoubleLogical {
    const DISCRIMINANT: u8 = 1;
}

// --- StorageLayout axis ---------------------------------------------------

/// How multiple values pack in column storage.
///
/// Sealed marker trait. The two implementors are `Dense` and
/// `Bitpacked` ZST markers. Each carries a `DISCRIMINANT` const for
/// compile-time `HasAxes` cross-checks.
pub trait StorageLayout: sealed::Sealed + Copy + Clone + Default + 'static {
    /// Stable per-marker discriminant for compile-time projection.
    const DISCRIMINANT: u8;
}

/// Storage layout: each value occupies a full container slot.
///
/// Used by `Hot`, `Warm`, `Precise`. Column storage is a flat array
/// of containers; element access is a direct index.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Dense;

/// Storage layout: values pack into shared container slots, one bit
/// per logical bit-position.
///
/// Used by `Cold`. Element access is a load-and-mask sequence;
/// element store is a load-mask-or-store sequence. Trades CPU cycles
/// for storage density on workloads with many entities.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Bitpacked;

impl sealed::Sealed for Dense {}
impl sealed::Sealed for Bitpacked {}

impl StorageLayout for Dense {
    const DISCRIMINANT: u8 = 0;
}
impl StorageLayout for Bitpacked {
    const DISCRIMINANT: u8 = 1;
}

// --- HasAxes: bundled axis projection on Strategy markers -----------------

/// Project a `Strategy` marker onto its three axis components.
///
/// Each strategy marker (`Hot`, `Warm`, `Cold`, `Precise`)
/// implements `HasAxes` with its specific axis combination per the
/// design table:
///
/// | Marker  | Overflow   | Width          | Layout    |
/// |---------|------------|----------------|-----------|
/// | Hot     | Wrapping   | Min            | Dense     |
/// | Warm    | Wrapping   | DoubleLogical  | Dense     |
/// | Cold    | Wrapping   | Min            | Bitpacked |
/// | Precise | Saturating | DoubleLogical  | Dense     |
pub trait HasAxes {
    /// Overflow policy of this strategy.
    type Overflow: OverflowPolicy;
    /// Container width of this strategy.
    type Width: ContainerWidth;
    /// Storage layout of this strategy.
    type Layout: StorageLayout;
}

impl HasAxes for Hot {
    type Overflow = Wrapping;
    type Width = Min;
    type Layout = Dense;
}

impl HasAxes for Warm {
    type Overflow = Wrapping;
    type Width = DoubleLogical;
    type Layout = Dense;
}

impl HasAxes for Cold {
    type Overflow = Wrapping;
    type Width = Min;
    type Layout = Bitpacked;
}

impl HasAxes for Precise {
    type Overflow = Saturating;
    type Width = DoubleLogical;
    type Layout = Dense;
}
