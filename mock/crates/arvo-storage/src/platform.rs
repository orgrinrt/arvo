//! Platform-binding wrappers and control-flow primitives.
//!
//! `USize` wraps the platform-pointer-width `usize`; `Cap` wraps a
//! `USize` for const-generic capacity positions. `Bool` wraps `bool`
//! for predicate returns and implements `Try` so consumers can use
//! `?` on it.
//!
//! `AsBool` is the bridge trait for boundary call sites that need a
//! raw `bool`.
//!
//! Per round 202605021800: USize and Cap share the entire canonical
//! const surface (Bounded + Identity + ConstPartialEq + ConstEq +
//! ConstBitEq + ConstOrd + ConstDefault + every core::ops + Deref +
//! AsRef) through one `impl_unsigned_integer_newtype!` macro. The
//! macro emits delegating bodies that read inner values through
//! `<Self as Transparent>::raw(self)` rather than `.0` field access,
//! so the substrate never normalises field-access on its own primitives.

use core::cmp::Ordering;
use core::convert::Infallible;
use core::marker::ConstParamTy;
use core::num::NonZeroUsize;
use core::ops::{
    Add, BitAnd, BitOr, BitXor, ControlFlow, Deref, Div, FromResidual, Mul, Not, Rem, Residual,
    Shl, Shr, Sub, Try,
};

use arvo_strategy::{Bounded, Identity};
use arvo_transparent::Transparent;
use notko::{ConstFromResidual, ConstTry, Maybe, Slot};

use crate::bridges::{
    ConstBitEq, ConstDefault, ConstEq, ConstOrd, ConstOrdering, ConstPartialEq,
};

/// Index / count newtype wrapping `usize`.
///
/// Wraps `usize` for the arvo-types-only lint. `Deref<Target = usize>`
/// gives `array[*idx]` ergonomics. Derives `ConstParamTy` so `USize`
/// values can be used inside other const-generic newtypes (see `Cap`).
///
/// Canonical const surface (Bounded / Identity / ConstPartialEq /
/// ConstEq / ConstBitEq / ConstOrd / ConstDefault / arith ops / bit
/// ops) is supplied by `impl_unsigned_integer_newtype!` below.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct USize(pub usize);

// SAFETY: `repr(transparent)` over `usize`. Layout-identical by Rust
// spec; transmute soundness follows from the repr.
unsafe impl const Transparent for USize {
    type Inner = usize;
}

impl Deref for USize {
    type Target = usize;
    #[inline(always)]
    fn deref(&self) -> &usize {
        &self.0
    }
}

/// Const-generic capacity for fixed-size structures.
///
/// Used wherever a const generic sizes an array. The type prevents
/// mixing capacities with unrelated integers. The const parameter
/// name carries the semantic distinction (`N`, `ROWS`, `NNZ`).
///
/// Canonical const surface comes from `impl_unsigned_integer_newtype!`
/// (same macro that generates USize's surface), so Cap and USize have
/// identical APIs by construction. The asymmetry that previously
/// existed (USize had bit ops, Cap did not) is gone.
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Cap(pub USize);

// SAFETY: `repr(transparent)` over `USize` (which is itself
// `repr(transparent)` over `usize`). Layout-identical by Rust spec.
unsafe impl const Transparent for Cap {
    type Inner = USize;
}

/// Generate the canonical const surface for an unsigned-integer
/// newtype.
///
/// Emits `Bounded`, `Identity`, `ConstPartialEq`, `ConstEq`,
/// `ConstBitEq`, `ConstOrd`, `ConstDefault`, the eleven `core::ops`
/// arith / bit ops (`Add`, `Sub`, `Mul`, `Div`, `Rem`, `Shl`, `Shr`,
/// `BitAnd`, `BitOr`, `BitXor`, `Not`), and `PartialOrd` / `Ord`
/// for `$outer`, where `$outer: repr(transparent)` over `$inner`
/// and `$inner` already provides the same surface. Bodies route
/// through `<$outer as Transparent>::raw` for unwrap and
/// tuple-struct construction for wrap, so the macro never normalises
/// `.0` field access on the wrapper.
///
/// `$inner` must implement: `Bounded + Identity + ConstPartialEq +
/// ConstEq + ConstBitEq + ConstOrd + ConstDefault + Add + Sub + Mul
/// + Div + Rem + Shl + Shr + BitAnd + BitOr + BitXor + Not +
/// PartialOrd + Ord` (every method called by the emitted bodies).
/// `$outer` must implement `[const] Transparent<Inner = $inner>`.
macro_rules! impl_unsigned_integer_newtype {
    ($outer:ty, $inner:ty) => {
        // ---- canonical typed-const surfaces ----
        impl const Bounded for $outer {
            const MIN: Self = Self(<$inner as Bounded>::MIN);
            const MAX: Self = Self(<$inner as Bounded>::MAX);
        }
        impl const Identity for $outer {
            const ZERO: Self = Self(<$inner as Identity>::ZERO);
            const ONE: Self = Self(<$inner as Identity>::ONE);
        }
        impl const ConstPartialEq for $outer {
            #[inline(always)]
            fn const_eq(&self, other: &Self) -> Bool {
                <$inner as ConstPartialEq>::const_eq(
                    &<Self as Transparent>::raw(*self),
                    &<Self as Transparent>::raw(*other),
                )
            }
        }
        impl const ConstEq for $outer {}
        impl const ConstBitEq for $outer {
            #[inline(always)]
            fn const_bit_eq(&self, other: &Self) -> Bool {
                <$inner as ConstBitEq>::const_bit_eq(
                    &<Self as Transparent>::raw(*self),
                    &<Self as Transparent>::raw(*other),
                )
            }
        }
        impl const ConstOrd for $outer {
            #[inline(always)]
            fn const_cmp(&self, other: &Self) -> ConstOrdering {
                <$inner as ConstOrd>::const_cmp(
                    &<Self as Transparent>::raw(*self),
                    &<Self as Transparent>::raw(*other),
                )
            }
        }
        impl const ConstDefault for $outer {
            #[inline(always)]
            fn const_default() -> Self {
                Self(<$inner as ConstDefault>::const_default())
            }
        }

        // ---- core::ops arith ----
        impl const Add<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn add(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) + <Self as Transparent>::raw(rhs))
            }
        }
        impl const Sub<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn sub(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) - <Self as Transparent>::raw(rhs))
            }
        }
        impl const Mul<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn mul(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) * <Self as Transparent>::raw(rhs))
            }
        }
        impl const Div<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn div(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) / <Self as Transparent>::raw(rhs))
            }
        }
        impl const Rem<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn rem(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) % <Self as Transparent>::raw(rhs))
            }
        }

        // ---- core::ops bit ops ----
        impl const Shl<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn shl(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) << <Self as Transparent>::raw(rhs))
            }
        }
        impl const Shr<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn shr(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) >> <Self as Transparent>::raw(rhs))
            }
        }
        impl const BitAnd<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn bitand(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) & <Self as Transparent>::raw(rhs))
            }
        }
        impl const BitOr<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn bitor(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) | <Self as Transparent>::raw(rhs))
            }
        }
        impl const BitXor<$outer> for $outer {
            type Output = $outer;
            #[inline(always)]
            fn bitxor(self, rhs: $outer) -> $outer {
                Self(<Self as Transparent>::raw(self) ^ <Self as Transparent>::raw(rhs))
            }
        }
        impl const Not for $outer {
            type Output = $outer;
            #[inline(always)]
            fn not(self) -> $outer {
                Self(!<Self as Transparent>::raw(self))
            }
        }

        // ---- core::cmp PartialOrd / Ord (non-const, std-compat) ----
        impl PartialOrd<$outer> for $outer {
            #[inline(always)]
            fn partial_cmp(&self, rhs: &$outer) -> Option<Ordering> { // lint:allow(no-bare-option) reason: core::cmp::PartialOrd::partial_cmp trait-method signature returns Option<Ordering>; tracked: #115
                let a = <Self as Transparent>::raw(*self);
                let b = <Self as Transparent>::raw(*rhs);
                a.partial_cmp(&b)
            }
        }
        impl Ord for $outer {
            #[inline(always)]
            fn cmp(&self, rhs: &$outer) -> Ordering {
                let a = <Self as Transparent>::raw(*self);
                let b = <Self as Transparent>::raw(*rhs);
                a.cmp(&b)
            }
        }
    };
}

impl_unsigned_integer_newtype!(USize, usize);
impl_unsigned_integer_newtype!(Cap, USize);

/// Control-flow boolean.
///
/// Returned by predicates (`is_zero`, `bit`). Not a fixed-point
/// type. `Bool` is for branching, `Bit` (defined in arvo-bits) is
/// for 1-bit data storage.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Bool(pub bool);

// SAFETY: `repr(transparent)` over `bool`. Layout-identical by spec.
unsafe impl const Transparent for Bool {
    type Inner = bool;
}

impl Bool {
    /// Constant `Bool(true)`.
    pub const TRUE: Bool = Bool(true);
    /// Constant `Bool(false)`.
    pub const FALSE: Bool = Bool(false);
}

impl Deref for Bool {
    type Target = bool;
    #[inline(always)]
    fn deref(&self) -> &bool {
        &self.0
    }
}

/// Uninhabited residual for [`Bool`]'s infallible `Try`.
///
/// `Bool` never short-circuits (`branch` always returns `Continue`), so
/// its `?` residual is the empty type. This carries the `Residual<bool>`
/// impl that `core::ops::Try::Residual` now requires. Bare
/// `core::convert::Infallible` cannot: the orphan rule forbids
/// implementing `core`'s `Residual` for a foreign type. Not a numeric
/// type, so it carries no `S: Strategy`.
pub enum BoolResidual {}

impl Try for Bool {
    type Output = bool;
    type Residual = BoolResidual;

    #[inline(always)]
    fn from_output(output: bool) -> Self {
        Bool(output)
    }

    #[inline(always)]
    fn branch(self) -> ControlFlow<BoolResidual, bool> {
        ControlFlow::Continue(<Self as Transparent>::raw(self))
    }
}

impl FromResidual<BoolResidual> for Bool {
    #[inline(always)]
    fn from_residual(residual: BoolResidual) -> Self {
        match residual {}
    }
}

impl Residual<bool> for BoolResidual {
    type TryType = Bool;
}

/// Bridge trait for code paths that need a raw `bool`.
///
/// Preferred path in WU code is `?`; `as_bool()` exists for boundary
/// compatibility with libraries that expose `bool` directly.
pub const trait AsBool {
    /// Extract the inner `bool`.
    fn as_bool(&self) -> bool;
}

impl const AsBool for Bool {
    #[inline(always)]
    fn as_bool(&self) -> bool {
        <Self as Transparent>::raw(*self)
    }
}

impl const From<bool> for Bool {
    #[inline(always)]
    fn from(b: bool) -> Self {
        Bool(b)
    }
}

impl const From<Bool> for bool {
    #[inline(always)]
    fn from(b: Bool) -> Self {
        <Bool as Transparent>::raw(b)
    }
}

// ---- Cap <-> USize const From bridges (round 202605112200) ----
//
// Typed ergonomic projection between the `Cap` capacity newtype and
// the `USize` count newtype. `impl const From` makes `.into()`
// resolve at const time when both operands are const-known
// (const-generic computations, `const _: USize = Cap::ONE.into()`
// type-level checks, const-fn bodies that project a Cap-typed
// parameter to a typed count).
//
// Bodies route through `Transparent::raw` (already `const unsafe
// impl const Transparent for Cap` / `USize`) for the unwrap and
// tuple-struct construction for the wrap, so neither direction
// normalises `.0` field-access on its own primitives.
//
// Bare-`usize` projections (`USize` <-> `usize`, `Cap` <-> `usize`)
// stay through the existing paths: `Deref<Target = usize>` on
// `USize`, `Transparent::raw`, the canonical `cap_size: Cap ->
// usize` const fn in `arvo-tensor`. This round only ships the two
// arvo-internal newtype edges.

impl const From<Cap> for USize {
    #[inline(always)]
    fn from(c: Cap) -> Self {
        <Cap as Transparent>::raw(c)
    }
}

impl const From<USize> for Cap {
    #[inline(always)]
    fn from(u: USize) -> Self {
        Cap(u)
    }
}

// ---- Bool canonical surface (subset of integer-newtype macro;
// Bool has only two values so Bounded/Arith are not meaningful).
// ConstPartialEq / ConstEq / ConstBitEq / ConstOrd / ConstDefault
// route through inner `bool`.

impl const ConstPartialEq for Bool {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        <bool as ConstPartialEq>::const_eq(
            &<Self as Transparent>::raw(*self),
            &<Self as Transparent>::raw(*other),
        )
    }
}

impl const ConstEq for Bool {}

impl const ConstBitEq for Bool {
    #[inline(always)]
    fn const_bit_eq(&self, other: &Self) -> Bool {
        <bool as ConstBitEq>::const_bit_eq(
            &<Self as Transparent>::raw(*self),
            &<Self as Transparent>::raw(*other),
        )
    }
}

impl const ConstOrd for Bool {
    #[inline(always)]
    fn const_cmp(&self, other: &Self) -> ConstOrdering {
        <bool as ConstOrd>::const_cmp(
            &<Self as Transparent>::raw(*self),
            &<Self as Transparent>::raw(*other),
        )
    }
}

impl const ConstDefault for Bool {
    #[inline(always)]
    fn const_default() -> Self {
        Bool(<bool as ConstDefault>::const_default())
    }
}

// ---- Bool ConstTry / ConstFromResidual (Round 5, #315) -------------
//
// Mirrors the existing `Try` / `FromResidual` impls above, lifted to
// notko's const-callable bridges. `?` desugars to core::ops::Try and
// stays unchanged; const-context call sites that need const-callable
// fallibility (mockspace lints, narrow_from helpers, future const
// validators) reach the same routing through the const family.

impl const ConstTry for Bool {
    type Output = bool;
    type Residual = Infallible;

    #[inline(always)]
    fn from_output(output: bool) -> Self {
        Bool(output)
    }

    #[inline(always)]
    fn branch(self) -> ControlFlow<Infallible, bool> {
        ControlFlow::Continue(<Self as Transparent>::raw(self))
    }
}

impl const ConstFromResidual<Infallible> for Bool {
    #[inline(always)]
    fn from_residual(residual: Infallible) -> Self {
        match residual {}
    }
}

// ---- NUSize: niche-filled USize alternative (Round 5, #315) --------
//
// `NUSize` wraps `Slot<NonZeroUsize>` with a logical +1 / -1 shift so
// consumers can use it as a 0-indexed `USize` without remembering the
// shift. The transparent layout and Slot's niche-filling mean
// `Array<NUSize, N>` packs to exactly `N * size_of::<usize>()` bytes,
// the same as `Array<USize, N>`. The "absent" marker is `NUSize::NONE`,
// distinct from any logical value (including `USize(0)`).
//
// Domain placement: `arvo-comb::binpack` consumes this for the
// "did this item fit any bin" return shape, replacing the prior
// overloaded `USize(0)` sentinel. See round-5 src CL Topic 2 for
// rationale.

/// Niche-filled optional `USize`.
///
/// `Slot<NonZeroUsize>` with a +1 shift on `some` and a -1 shift on
/// `into_maybe`, so consumers see logical 0..usize::MAX-1 with
/// `NUSize::NONE` distinct from any logical value. Layout-identical
/// to `usize` (no `Array<NUSize, N>` overhead vs `Array<USize, N>`).
///
/// Construction can fail when the logical value is `usize::MAX` (the
/// +1 would overflow); construction returns `NUSize::NONE` in that
/// case. Practical workloads (item counts, bin indices, capacity-bound
/// indices) never hit the cap because `Cap<N>` is bounded well below
/// `usize::MAX`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct NUSize(Slot<NonZeroUsize>);

impl NUSize {
    /// The absent value. Equivalent to `Slot::NONE`.
    pub const NONE: Self = Self(Slot::NONE);

    /// Construct from a logical 0-indexed `USize`. Internally shifts
    /// by +1 so logical 0 maps to `NonZeroUsize::new(1)`. Returns
    /// `NUSize::NONE` if the logical value is `usize::MAX` (the +1
    /// shift would overflow).
    pub const fn some(logical: USize) -> Self {
        match <USize as Transparent>::raw(logical).checked_add(1) {
            Some(shifted) => match NonZeroUsize::new(shifted) {
                Some(nz) => Self(Slot::some(nz)),
                None => Self::NONE,
            },
            None => Self::NONE,
        }
    }

    /// Extract as a logical `Maybe<USize>`. Inner -1 shift maps
    /// `NonZeroUsize::new(1)` back to logical 0. Consuming form
    /// because the shift produces a fresh value (no in-memory
    /// `Maybe<USize>` exists to borrow into); follows notko's
    /// `Slot::into_maybe` convention.
    pub const fn into_maybe(self) -> Maybe<USize> {
        match self.0.into_maybe() {
            Maybe::Is(nz) => Maybe::Is(USize(nz.get() - 1)),
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// Logical value or the supplied default when absent.
    pub const fn unwrap_or(self, default: USize) -> USize {
        match self.into_maybe() {
            Maybe::Is(v) => v,
            Maybe::Isnt => default,
        }
    }

    /// `Bool::TRUE` when carrying a logical value.
    pub const fn is_some(&self) -> Bool {
        Bool(self.0.is_some())
    }

    /// `Bool::TRUE` when absent.
    pub const fn is_none(&self) -> Bool {
        Bool(self.0.is_none())
    }
}

impl Default for NUSize {
    #[inline(always)]
    fn default() -> Self {
        Self::NONE
    }
}

// SAFETY: `repr(transparent)` over `Slot<NonZeroUsize>`, which is
// `repr(transparent)` over `Maybe<NonZeroUsize>`, which niche-fills
// to a single `usize`. Layout-identical to `usize` by transitive
// transparent guarantee plus notko's per-instantiation
// `_LAYOUT_ASSERT` covering `NonZeroUsize`.
unsafe impl const Transparent for NUSize {
    type Inner = Slot<NonZeroUsize>;
}
