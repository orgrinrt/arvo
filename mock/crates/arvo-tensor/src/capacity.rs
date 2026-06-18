//! `Capacity`, the GCE-free fixed-capacity foundation.

use arvo::{Cap, USize};

use crate::cap::cap;

/// A fixed storage capacity expressed as a type.
///
/// The capacity is a TYPE, not a `Cap` const generic. The backing storage is
/// a GAT bound to a literal-length array in the concrete impl, so no const
/// expression sits in type position and `generic_const_exprs` never runs over
/// capacity arithmetic. This is the GCE-free replacement for the
/// `[T; cap_size(N)]` pattern that ICEs when a generic consumer threads the
/// capacity through its own generic code (the `cap_size(cap(N))` overflow).
///
/// The `AsRef<[T]> + AsMut<[T]>` bound gives slice access and iteration
/// without a const-generic `Index` HKT, and lets two-dimensional storage be
/// the composition `R::Array<C::Array<T>>` with no separate trait.
pub trait Capacity {
    /// The backing array for `T` at this capacity. `[T; N]` for `Dim<N>`.
    type Array<T>: AsRef<[T]> + AsMut<[T]>;

    /// The typed capacity. `Dim<N>` maps to `cap(N)`.
    const CAP: Cap;

    /// Build the backing array with `v` in every slot.
    fn filled<T: Copy>(v: T) -> Self::Array<T>;

    /// Build the backing array by invoking `f(i)` for each slot index.
    ///
    /// The GAT-returning replacement for `core::array::from_fn`, which cannot
    /// produce the opaque `Self::Array<T>`. Every index-built container and
    /// algorithm routes its per-slot construction through this method.
    fn from_fn<T, F: FnMut(USize) -> T>(f: F) -> Self::Array<T>;
}

/// A capacity of exactly `N` slots.
///
/// The implementing type is generic over `const N: usize`; the `Capacity`
/// trait stays non-generic, so consumers bind `C: Capacity` with no const
/// parameter and keep the type-dispatch property. `[T; N]` is plain
/// min-const-generics (no `cap_size`, no `generic_const_exprs`); `cap(N)`
/// appears only in the associated-const value position.
pub struct Dim<const N: usize>; // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: array-length-grammar root, the one permitted bare primitive in the capacity-as-type convention (array lengths are usize at the language level, and a Cap const generic would reintroduce the cap_size(N) GCE); tracked: #649

impl<const N: usize> Capacity for Dim<N> { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: array-length-grammar root, see the Dim declaration; tracked: #649
    type Array<T> = [T; N];
    const CAP: Cap = cap(N);

    #[inline(always)]
    fn filled<T: Copy>(v: T) -> [T; N] {
        [v; N]
    }

    #[inline]
    fn from_fn<T, F: FnMut(USize) -> T>(mut f: F) -> [T; N] {
        core::array::from_fn(|i| f(USize(i)))
    }
}

/// The const-callable capacity surface, the const analog of [`Capacity`].
///
/// Carries the minimal surface a `const fn` needs to build and index a
/// fixed-capacity backing array in const context: a const `filled` constructor
/// and const indexed `get` / `set`. A const fn generic over
/// `C: [const] ConstCapacity` allocates and walks scratch arrays at compile
/// time, which is what compile-time DAG analysis (a const `waist_detect`)
/// requires.
///
/// `Capacity` itself cannot be a `const trait`: its `from_fn` takes an
/// `F: FnMut(USize) -> T` closure and calls `core::array::from_fn` in its body,
/// neither const-callable, and `impl const Trait` must provide a const body for
/// every method. `ConstCapacity` is therefore a sibling trait, not an
/// extension: `Capacity` keeps its `AsRef` / `AsMut` / `from_fn` runtime
/// surface verbatim, and this adds the const surface alongside. `Dim<N>`
/// implements both, so a consumer needing both binds both and shares the one
/// `Dim<N>` impl.
///
/// `ConstCapacity::Array` is a distinct GAT from `Capacity::Array`, carrying the
/// `Copy` bound the const surface needs (the array is returned and indexed by
/// value in const context, dealing in `Copy` scalars: counts, bit-words). For
/// `Dim<N>` both resolve to `[T; N]`, so a const consumer's
/// `ConstCapacity::Array<USize>` is the same `[USize; N]` a runtime consumer's
/// `Capacity::Array<USize>` is.
pub const trait ConstCapacity {
    /// The backing array for `T` at this capacity. `[T; N]` for `Dim<N>`.
    type Array<T: Copy>: Copy;

    /// The typed capacity. `Dim<N>` maps to `cap(N)`.
    const CAP: Cap;

    /// Build the backing array with `v` in every slot.
    fn filled<T: Copy>(v: T) -> Self::Array<T>;

    /// Read the slot at `i` by value.
    fn get<T: Copy>(a: &Self::Array<T>, i: USize) -> T;

    /// Write `v` to the slot at `i`.
    fn set<T: Copy>(a: &mut Self::Array<T>, i: USize, v: T);

    /// View the backing array as a slice in const context.
    ///
    /// The const analog of [`Capacity`]'s runtime `AsRef`. Bridges a built
    /// `Self::Array<T>` scratch to a slice-taking `[const]` helper, which is
    /// how a const fn hands its GAT scratch to a slice-filling routine
    /// (compile-time DAG grouping fills `&mut [USize]` mask scratch this way).
    fn slice<T: Copy>(a: &Self::Array<T>) -> &[T];

    /// View the backing array as a mutable slice in const context.
    ///
    /// The const analog of [`Capacity`]'s runtime `AsMut`.
    fn slice_mut<T: Copy>(a: &mut Self::Array<T>) -> &mut [T];
}

impl<const N: usize> const ConstCapacity for Dim<N> { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: array-length-grammar root, see the Dim declaration; tracked: #649
    type Array<T: Copy> = [T; N];
    const CAP: Cap = cap(N);

    #[inline(always)]
    fn filled<T: Copy>(v: T) -> [T; N] {
        [v; N]
    }

    #[inline(always)]
    fn get<T: Copy>(a: &[T; N], i: USize) -> T {
        a[i.0]
    }

    #[inline(always)]
    fn set<T: Copy>(a: &mut [T; N], i: USize, v: T) {
        a[i.0] = v;
    }

    #[inline(always)]
    fn slice<T: Copy>(a: &[T; N]) -> &[T] {
        a
    }

    #[inline(always)]
    fn slice_mut<T: Copy>(a: &mut [T; N]) -> &mut [T] {
        a
    }
}
