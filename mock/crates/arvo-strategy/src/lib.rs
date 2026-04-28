#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]

//! arvo-strategy. Strategy markers + container-projection traits.
//!
//! `Hot` / `Warm` / `Cold` / `Precise` ZSTs. `Strategy` marker
//! trait. `UContainerFor<N>` / `IContainerFor<N>` const traits
//! projecting strategy + bit-width to bare primitive containers.
//! `Resolve<S1, S2>` strategy resolution.
//!
//! Four strategies determine container width, arithmetic semantics,
//! and cross-width rules for `UFixed` / `IFixed`:
//!
//! | Strategy | Container          | Arithmetic           |
//! |----------|--------------------|----------------------|
//! | `Hot`    | min aligned        | wrapping             |
//! | `Warm`   | 2x logical         | wrapping (safe 1-op) |
//! | `Cold`   | min (bitpacked)    | widen-op-narrow      |
//! | `Precise`| 2x logical         | saturating           |
//!
//! Container selection is handled by the sealed `UContainerFor` /
//! `IContainerFor` const traits: one impl per (strategy, bit-range)
//! pair. `Warm` is not implemented for the `33..=64` bit range,
//! making `UFixed<_, _, Warm>` with `I + F > 32` a compile-time
//! error per the doc CL D2 resolution.

use core::marker::ConstParamTy;

mod sealed {
    pub trait Sealed {}
}

mod arith;
mod axes;
mod container;
mod cross_strategy;
mod multi_container;
mod widen;

pub use arith::{IArith, ISaturating, UArith, USaturating};
pub use cross_strategy::CrossStrategyOp;
pub use axes::{
    Bitpacked, ContainerWidth, Dense, DoubleLogical, HasAxes, Min, OverflowPolicy, Saturating,
    StorageLayout, Wrapping,
};
pub use container::{BitsContainerFor, IContainerFor, UContainerFor};
pub use multi_container::{BitPrim, MultiContainer};
pub use widen::{INarrowFrom, IWidenFrom, UNarrowFrom, UWidenFrom};

/// Strategy marker trait.
///
/// Implemented by the four zero-sized markers `Hot`, `Warm`, `Cold`,
/// and `Precise`. Sealed: consumers cannot add new strategies.
pub const trait Strategy: sealed::Sealed + Copy + Clone + Default + 'static {
    /// Human-readable name of the strategy.
    ///
    /// Debug-only: static strings are gated out of release builds
    /// (zero `.rodata` footprint). Runtime strategy identity flows
    /// through `RANK`; `NAME` is for diagnostics and tests.
    #[cfg(debug_assertions)]
    const NAME: &'static str;

    /// Conservativeness rank. Higher is more conservative. Used by
    /// cross-strategy operation resolution:
    /// `Precise > Cold > Warm > Hot`.
    const RANK: u16;
}

/// Optimised for L1 density and operation throughput.
///
/// Container is the minimum byte-aligned standard width that fits
/// `I + F` bits. Arithmetic is wrapping. Single instruction per op;
/// LLVM vectorises freely.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Hot;

/// Store big, operate fast. The development-friendly default.
///
/// Container is 2x the logical bit width. A single `add` / `sub` /
/// `mul` of two values within their logical range cannot overflow
/// the container. Bounded to `I + F <= 32` per doc CL D2: no u128
/// container is available, so Warm is forbidden at logical widths
/// beyond 32 bits rather than degrading silently.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Warm;

/// Store small, operate carefully. Optimised for storage density.
///
/// Minimum container, bitpacked for sub-byte values. Arithmetic
/// widens to 2x before operating, narrows back on store.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Cold;

/// Store exactly, operate exactly. Correctness above all.
///
/// Container is 2x the logical width (same physical layout as Warm).
/// Arithmetic is saturating: overflow clamps to logical min/max
/// rather than wrapping.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Precise;

impl sealed::Sealed for Hot {}
impl sealed::Sealed for Warm {}
impl sealed::Sealed for Cold {}
impl sealed::Sealed for Precise {}

impl const Strategy for Hot {
    #[cfg(debug_assertions)]
    const NAME: &'static str = "Hot";
    const RANK: u16 = 0;
}
impl const Strategy for Warm {
    #[cfg(debug_assertions)]
    const NAME: &'static str = "Warm";
    const RANK: u16 = 1;
}
impl const Strategy for Cold {
    #[cfg(debug_assertions)]
    const NAME: &'static str = "Cold";
    const RANK: u16 = 2;
}
impl const Strategy for Precise {
    #[cfg(debug_assertions)]
    const NAME: &'static str = "Precise";
    const RANK: u16 = 3;
}

// --- Sign axis markers ----------------------------------------------------
//
// `Signedness` is the sealed marker trait carried as the third
// const-generic on `Bits<N, S, Sign>`. Default `Sign = Unsigned`
// keeps every existing call site unchanged. `IFixed<I, F, S>` reaches
// for `Sign = Signed` internally; consumers normally write `IFixed`
// rather than `Bits<N, S, Signed>` directly.

/// Sign-axis marker trait. Sealed; consumers cannot add new variants.
///
/// The two implementors are `Unsigned` and `Signed` (zero-sized
/// markers). Used as the `Sign` const-generic on `Bits<N, S, Sign>`
/// and routed through `BitsContainerFor<N, Sign>` to either the
/// `UContainerFor<N>` or `IContainerFor<N>` table.
pub trait Signedness: sealed::Sealed + Copy + Clone + Default + 'static {}

/// Unsigned bit pattern. Default `Sign` on `Bits<N, S, Sign>`.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Unsigned;

/// Signed bit pattern. Used by `IFixed<I, F, S>` internally.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Signed;

impl sealed::Sealed for Unsigned {}
impl sealed::Sealed for Signed {}

impl Signedness for Unsigned {}
impl Signedness for Signed {}

// --- Strategy resolution for cross-strategy ops ----------------------------
//
// `Precise > Cold > Warm > Hot`. The more conservative strategy wins.
// Encoded via trait-level selection: `Resolve<S1, S2>` picks the winner.
//
// Implemented as a nested table so blanket impls don't collide.

/// Resolve the more conservative of two strategies.
///
/// `Resolve<S1, S2>::Out` is the higher-rank strategy; see
/// `Strategy::RANK`.
pub const trait Resolve<Other: Strategy>: Strategy {
    /// The resolved strategy: more conservative of `Self` and `Other`.
    type Out: Strategy;
}

macro_rules! impl_resolve {
    ($lhs:ty, $rhs:ty, $out:ty) => {
        impl const Resolve<$rhs> for $lhs {
            type Out = $out;
        }
    };
}

// Self: identity.
impl_resolve!(Hot, Hot, Hot);
impl_resolve!(Warm, Warm, Warm);
impl_resolve!(Cold, Cold, Cold);
impl_resolve!(Precise, Precise, Precise);

// Hot with others.
impl_resolve!(Hot, Warm, Warm);
impl_resolve!(Hot, Cold, Cold);
impl_resolve!(Hot, Precise, Precise);
impl_resolve!(Warm, Hot, Warm);
impl_resolve!(Cold, Hot, Cold);
impl_resolve!(Precise, Hot, Precise);

// Warm with others.
impl_resolve!(Warm, Cold, Cold);
impl_resolve!(Warm, Precise, Precise);
impl_resolve!(Cold, Warm, Cold);
impl_resolve!(Precise, Warm, Precise);

// Cold with Precise.
impl_resolve!(Cold, Precise, Precise);
impl_resolve!(Precise, Cold, Precise);
