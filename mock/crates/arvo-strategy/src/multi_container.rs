//! Multi-value storage primitive for bit widths beyond 128.
//!
//! `MultiContainer<HiT, LoT>` pairs two `BitPrim` halves to back
//! `Bits<N, S, Sign>` at logical widths exceeding any single native
//! primitive. Round 202604280500 ships the storage shape only; the
//! arithmetic surface (`UArith` / `IArith` / `UWidenFrom` /
//! `UNarrowFrom` for `MultiContainer`) is BACKLOG-tracked for a
//! follow-up round.
//!
//! `BitPrim` is the sealed marker trait abstracting the native
//! primitives that may appear as halves: `u8` / `u16` / `u32` / `u64`
//! / `u128` for unsigned, `i8` / `i16` / `i32` / `i64` / `i128` for
//! signed. Mixed-sign pairs are not supported in this round.
//!
//! Re-exported from `arvo-storage` for the documented public surface
//! (`arvo_storage::MultiContainer`); defined here because
//! `arvo-strategy/src/container.rs` references the projection type
//! directly in the `UContainerFor<N>::T` table for `N >= 129`.

use crate::sealed;

/// Sealed marker for primitives that may appear as a `MultiContainer`
/// half.
///
/// Implemented for `u8` / `u16` / `u32` / `u64` / `u128` and
/// `i8` / `i16` / `i32` / `i64` / `i128`. The bound surface matches
/// `UContainerFor<N>::T`'s minimum so any `BitPrim` can flow through
/// the projection.
pub trait BitPrim:
    sealed::Sealed
    + Copy
    + Clone
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Default
    + core::hash::Hash
    + core::fmt::Debug
    + 'static
{
}

impl sealed::Sealed for u8 {}
impl sealed::Sealed for u16 {}
impl sealed::Sealed for u32 {}
impl sealed::Sealed for u64 {}
impl sealed::Sealed for u128 {}
impl sealed::Sealed for i8 {}
impl sealed::Sealed for i16 {}
impl sealed::Sealed for i32 {}
impl sealed::Sealed for i64 {}
impl sealed::Sealed for i128 {}

impl BitPrim for u8 {}
impl BitPrim for u16 {}
impl BitPrim for u32 {}
impl BitPrim for u64 {}
impl BitPrim for u128 {}
impl BitPrim for i8 {}
impl BitPrim for i16 {}
impl BitPrim for i32 {}
impl BitPrim for i64 {}
impl BitPrim for i128 {}

/// Two-half multi-value storage container.
///
/// Pairs two `BitPrim` halves under a stable C layout. `hi` carries
/// the most-significant half, `lo` the least. The pair backs
/// `Bits<N, S, Sign>` at `N >= 129` per the `UContainerFor` /
/// `IContainerFor` table.
///
/// Lex ordering on `(hi, lo)` is sound for storage purposes (stable,
/// total). Arithmetic respecting the logical bit-width contract is
/// BACKLOG-tracked.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct MultiContainer<HiT: BitPrim, LoT: BitPrim> {
    /// Most-significant half.
    pub hi: HiT,
    /// Least-significant half.
    pub lo: LoT,
}

impl<HiT: BitPrim, LoT: BitPrim> MultiContainer<HiT, LoT> {
    /// Construct a `MultiContainer` from two halves.
    #[inline(always)]
    pub const fn new(hi: HiT, lo: LoT) -> Self { Self { hi, lo } }
}
