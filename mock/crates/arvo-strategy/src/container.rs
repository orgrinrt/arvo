//! Unified container projection via Pattern C const-tag dispatch.
//!
//! `BitsContainerFor<const N: u16, Sign: Signedness>` const trait
//! projecting `(strategy, logical_bits, sign)` to a concrete storage
//! type. One user-facing impl per Strategy. Bucket dispatch through
//! the `Project<TAG, Sign, BYTES, S>` helper trait whose impls are
//! Strategy-aware (Hot / Cold use min aligned native; Warm / Precise
//! use 2x logical native).
//!
//! Native bucket boundaries per Strategy:
//!
//! - **Hot / Cold**: minimum aligned per logical bit count.
//!   `1..=8 -> u8`, `9..=16 -> u16`, `17..=32 -> u32`,
//!   `33..=64 -> u64`, `65..=128 -> u128`.
//! - **Warm / Precise**: 2x logical width (one bucket up; carries
//!   single-op overflow headroom for Warm wrapping and Precise
//!   saturating semantics). `1..=8 -> u16`, `9..=16 -> u32`,
//!   `17..=32 -> u64`, `33..=64 -> u128`. **No native bucket above
//!   N=64** by design: Warm / Precise at `N=65..=128` falls into the
//!   wide bucket directly (no native u256 ladder).
//!
//! Wide bucket (above the native ladder) projects to
//! `WideBits<bytes_for(N), A>`:
//!
//! - **Hot**: `A = A16` (SSE2 / NEON 16-byte aligned baseline).
//!   #320 lands AVX-2 (`A32`) and AVX-512 (`A64`) tiers behind cfg
//!   gates and consumer opt-in per audit H1.
//! - **Cold / Warm / Precise**: `A = A1` (align-1 byte-exact, no
//!   alignment padding).
//!
//! Round 202605031400 (#316) replaces the per-N
//! `UContainerFor<N>` + `IContainerFor<N>` tables (~512 entries
//! across N x Sign x Strategy plus the older Round D MultiContainer
//! cells) with this Pattern C trait family. 48 impls total: 40
//! native (4 strategies x 5/4 buckets x 2 Sign, asymmetric per the
//! Strategy-aware boundaries above) + 8 wide (4 strategies x 2 Sign)
//! + 4 user-facing `BitsContainerFor` impls. ~98% reduction in
//! impl-block count vs. the per-N table.
//!
//! Validated by sketch 07 for the dispatch mechanism (TAG-keyed
//! Project impls dodge E0119 conflicts) and sketch 08 (production-
//! semantics audit: confirms the Strategy-aware bucket structure
//! cannot be Strategy-erased; sketch 07's simplification did not
//! match production and was repaired here).

use crate::{
    A1, A16, Align, Cold, Hot, Precise, Signed, Signedness, Strategy, Unsigned, Warm, WideBits,
};

// ---------------------------------------------------------------------------
// Bucket tag functions (Strategy-aware).
//
// 0..=4 are native primitive buckets. 5 is the wide bucket.
// Hot / Cold span 0..=4 native (N=1..=128). Warm / Precise span 0..=3
// native (N=1..=64); their bucket 4 is undefined and N=65..=128 falls
// straight to wide.
// ---------------------------------------------------------------------------

#[inline(always)]
pub const fn tag_hot_cold(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else if n <= 128 {
        4
    } else {
        5
    }
}

#[inline(always)]
pub const fn tag_warm_precise(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else {
        5
    }
}

#[inline(always)]
pub const fn bytes_for_u16(n: u16) -> usize {
    (n as usize).div_ceil(8)
}

// ---------------------------------------------------------------------------
// User-facing trait.
// ---------------------------------------------------------------------------

/// Sign-aware container dispatch: `(strategy, logical_bits, sign) -> type`.
///
/// One impl per Strategy. Each defers to the
/// `Project<TAG, Sign, BYTES, S>` helper trait through a
/// Strategy-specific `tag` const fn. Absence of a Project impl for
/// a given `(TAG, Sign, S)` triple is how `Uint<100, Warm>`
/// (N=100, no native u256) becomes a compile error pointing at the
/// `#[diagnostic::on_unimplemented]` note below.
#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` does not provide a container for {N}-bit width",
    note = "Hot / Cold cover 1..=128 directly via native u8/u16/u32/u64/u128. Warm / Precise cover 1..=64 via 2x-logical primitives (u16/u32/u64/u128); for 65..=128 they have no native bucket and must use Hot or Cold. Above 128 bits all four strategies dispatch through `WideBits<bytes_for(N), A>` (Hot uses align-16 baseline, others align-1). If this fires on Warm/Precise at N=65..=128, choose Hot or Cold explicitly."
)]
pub const trait BitsContainerFor<const N: u16, Sign: Signedness>: Strategy {
    /// Concrete storage type for this `(strategy, bit-width, sign)`
    /// triple. Either a native primitive (native bucket) or a
    /// `WideBits<BYTES, A>` (wide bucket).
    type T: Copy
        + Clone
        + PartialEq
        + Eq
        + Default
        + core::hash::Hash
        + core::fmt::Debug
        + 'static;
}

// ---------------------------------------------------------------------------
// Pattern C helper trait.
//
// Distinct impls keyed on the const-generic `TAG` value avoid E0119
// overlap. The `BYTES` const generic is unused for native buckets
// (TAG 0..=4) but threaded through so the wide bucket (TAG=5) can
// build `WideBits<BYTES, A>`.
// ---------------------------------------------------------------------------

/// Bucket-keyed projection helper. Per-Strategy impls below cover
/// every reachable `(TAG, Sign, S)` combination for that Strategy.
///
/// Sealed via the `crate::sealed::Sealed` supertrait. Downstream
/// consumers cannot add `impl Project<6, ..., Picker>` etc.; only
/// types in this crate that impl `Sealed` can implement `Project`.
/// Today the sole implementor is `Picker`.
pub trait Project<const TAG: usize, Sign: Signedness, const BYTES: usize, S: Strategy>:
    crate::sealed::Sealed
{
    /// Concrete storage type for this projection slot.
    type T: Copy
        + Clone
        + PartialEq
        + Eq
        + Default
        + core::hash::Hash
        + core::fmt::Debug
        + 'static;
}

/// Single dispatch site for `Project` impls. Zero-sized.
///
/// Stays `pub` because it appears in the public associated-type body
/// of every `BitsContainerFor` impl (`type T = <Picker as Project<...>>::T`).
/// Demoting `Picker` to `pub(crate)` triggers `E0446: private type Picker
/// in public interface` per round 202605031548 sketch 02 finding 2. The
/// gate against downstream impls is the `Project` seal, not Picker's
/// visibility.
pub struct Picker;

impl crate::sealed::Sealed for Picker {}

// ---------------------------------------------------------------------------
// Native bucket Project impls (Strategy-aware), via macro_rules.
//
// Hot / Cold: minimum aligned (5 buckets per Sign).
// Warm / Precise: 2x logical (4 buckets per Sign; no bucket 4).
//
// The `BYTES` generic is unused for native impls; consumers always
// pass `bytes_for_u16(N)` but the projection ignores it at native TAGs.
//
// Per round 202605031548 (#314), the 40 hand-written native-bucket
// impl blocks collapse to 4 macro invocations (one per Strategy).
// Total impl count is unchanged; source-line count drops by ~70%.
// ---------------------------------------------------------------------------

macro_rules! impl_native_bucket {
    (
        $strategy:ty,
        [ $( $u_tag:literal => $u_ty:ty ),+ $(,)? ],
        [ $( $i_tag:literal => $i_ty:ty ),+ $(,)? ]
        $(,)?
    ) => {
        $(
            impl<const BYTES: usize> Project<$u_tag, Unsigned, BYTES, $strategy> for Picker {
                type T = $u_ty;
            }
        )+
        $(
            impl<const BYTES: usize> Project<$i_tag, Signed, BYTES, $strategy> for Picker {
                type T = $i_ty;
            }
        )+
    };
}

// Hot: 5 buckets, min aligned native ladder.
impl_native_bucket!(
    Hot,
    [0 => u8, 1 => u16, 2 => u32, 3 => u64, 4 => u128],
    [0 => i8, 1 => i16, 2 => i32, 3 => i64, 4 => i128],
);

// Cold: same primitive ladder as Hot; bitpacking is an access-path
// concern, not a container-type concern.
impl_native_bucket!(
    Cold,
    [0 => u8, 1 => u16, 2 => u32, 3 => u64, 4 => u128],
    [0 => i8, 1 => i16, 2 => i32, 3 => i64, 4 => i128],
);

// Warm: 4 buckets, 2x-logical native ladder. 1..=8 dispatches to u16, etc.
impl_native_bucket!(
    Warm,
    [0 => u16, 1 => u32, 2 => u64, 3 => u128],
    [0 => i16, 1 => i32, 2 => i64, 3 => i128],
);

// Precise: same primitive ladder as Warm; saturating semantics
// implemented in arith.rs, container shape is identical.
impl_native_bucket!(
    Precise,
    [0 => u16, 1 => u32, 2 => u64, 3 => u128],
    [0 => i16, 1 => i32, 2 => i64, 3 => i128],
);

// ---------------------------------------------------------------------------
// Wide bucket Project impls (TAG=5).
//
// Hot uses A16 (SSE2 / NEON baseline). Cold / Warm / Precise use A1
// (align-1 byte-exact). Sign axis is structurally irrelevant at the
// wide storage layer (bytes are bytes); BitPrim impls on
// `WideBits<BYTES, A>` in arvo-bits-contracts add the sign-aware
// interpretation.
// ---------------------------------------------------------------------------

impl<Sign: Signedness, const BYTES: usize> Project<5, Sign, BYTES, Hot> for Picker {
    type T = WideBits<BYTES, A16>;
}

impl<Sign: Signedness, const BYTES: usize> Project<5, Sign, BYTES, Cold> for Picker {
    type T = WideBits<BYTES, A1>;
}

impl<Sign: Signedness, const BYTES: usize> Project<5, Sign, BYTES, Warm> for Picker {
    type T = WideBits<BYTES, A1>;
}

impl<Sign: Signedness, const BYTES: usize> Project<5, Sign, BYTES, Precise> for Picker {
    type T = WideBits<BYTES, A1>;
}

// ---------------------------------------------------------------------------
// User-facing `BitsContainerFor` impls (4 total, one per Strategy).
//
// Each defers to `<Picker as Project<{ tag(N) }, Sign, { bytes_for_u16(N) }, Self>>::T`
// using its Strategy-specific tag function. The `[(); ...]:` where-
// clauses validate the const expressions at trait-resolution time.
// ---------------------------------------------------------------------------

impl<const N: u16, Sign: Signedness> const BitsContainerFor<N, Sign> for Hot
where
    Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>,
{
    type T = <Picker as Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>>::T;
}

impl<const N: u16, Sign: Signedness> const BitsContainerFor<N, Sign> for Cold
where
    Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Cold>,
{
    type T = <Picker as Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Cold>>::T;
}

impl<const N: u16, Sign: Signedness> const BitsContainerFor<N, Sign> for Warm
where
    Picker: Project<{ tag_warm_precise(N) }, Sign, { bytes_for_u16(N) }, Warm>,
{
    type T = <Picker as Project<{ tag_warm_precise(N) }, Sign, { bytes_for_u16(N) }, Warm>>::T;
}

impl<const N: u16, Sign: Signedness> const BitsContainerFor<N, Sign> for Precise
where
    Picker: Project<{ tag_warm_precise(N) }, Sign, { bytes_for_u16(N) }, Precise>,
{
    type T = <Picker as Project<{ tag_warm_precise(N) }, Sign, { bytes_for_u16(N) }, Precise>>::T;
}

// Mark `Align` as used inside this module so its trait bound on the
// wide-bucket impls keeps the import alive even when only A1 / A16
// are reached at the projection level today.
const _: fn() = || {
    fn _bound<A: Align>() {}
    _bound::<A1>();
    _bound::<A16>();
};
