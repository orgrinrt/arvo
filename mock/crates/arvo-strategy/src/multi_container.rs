//! Multi-value storage primitive for bit widths beyond 128.
//!
//! `MultiContainer<HiT, LoT>` pairs two `MultiContainerHalf` halves to back
//! `Bits<N, S, Sign>` at logical widths exceeding any single native
//! primitive. Round 202604280500 ships the storage shape only; the
//! arithmetic surface (`UArith` / `IArith` / `UWidenFrom` /
//! `UNarrowFrom` for `MultiContainer`) is BACKLOG-tracked for a
//! follow-up round.
//!
//! `MultiContainerHalf` is the sealed marker trait abstracting the
//! native primitives that may appear as halves: `u8` / `u16` / `u32`
//! / `u64` / `u128` for unsigned, `i8` / `i16` / `i32` / `i64` /
//! `i128` for signed. Mixed-sign pairs are not supported in this
//! round.
//!
//! The trait was previously named `BitPrim` here, conflicting in
//! intent (but not in path) with the canonical `BitPrim` in
//! `arvo-bits-contracts`. Renamed to `MultiContainerHalf` (round
//! 202605021800) to free the canonical name and to make the marker's
//! purpose explicit at the call site.
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
/// `UContainerFor<N>::T`'s minimum so any `MultiContainerHalf` can
/// flow through the projection.
pub trait MultiContainerHalf:
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

impl MultiContainerHalf for u8 {}
impl MultiContainerHalf for u16 {}
impl MultiContainerHalf for u32 {}
impl MultiContainerHalf for u64 {}
impl MultiContainerHalf for u128 {}
impl MultiContainerHalf for i8 {}
impl MultiContainerHalf for i16 {}
impl MultiContainerHalf for i32 {}
impl MultiContainerHalf for i64 {}
impl MultiContainerHalf for i128 {}

/// Two-half multi-value storage container.
///
/// Pairs two `MultiContainerHalf` halves under a stable C layout.
/// `hi` carries the most-significant half, `lo` the least. The pair
/// backs `Bits<N, S, Sign>` at `N >= 129` per the `UContainerFor` /
/// `IContainerFor` table.
///
/// Lex ordering on `(hi, lo)` is sound for storage purposes (stable,
/// total). Arithmetic respecting the logical bit-width contract is
/// BACKLOG-tracked.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct MultiContainer<HiT: MultiContainerHalf, LoT: MultiContainerHalf> {
    /// Most-significant half.
    pub hi: HiT,
    /// Least-significant half.
    pub lo: LoT,
}

impl<HiT: MultiContainerHalf, LoT: MultiContainerHalf> MultiContainer<HiT, LoT> {
    /// Construct a `MultiContainer` from two halves.
    #[inline(always)]
    pub const fn new(hi: HiT, lo: LoT) -> Self { Self { hi, lo } }
}

// SAFETY: `MultiContainer<HiT, LoT>` is `repr(C)` over two halves.
// `ConstParamTy_` requires structural eq + bitwise stable
// representation. Both halves are `MultiContainerHalf +
// ConstParamTy_` (every primitive in the seal, u8..u128, has the
// derive); the composite is structurally equal under the standard
// derive. Audit C2: this impl closes the gap that previously made
// `Bits<N, ..., Signed>` unsound at `N >= 129` (where the projection
// resolves to a `MultiContainer<HiT, LoT>` carrier and Bits's own
// `ConstParamTy_` impl bounds on the inner being `ConstParamTy_`).
impl<HiT, LoT> core::marker::ConstParamTy_ for MultiContainer<HiT, LoT>
where
    HiT: MultiContainerHalf + core::marker::ConstParamTy_,
    LoT: MultiContainerHalf + core::marker::ConstParamTy_,
{
}
