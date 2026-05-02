//! Platform-binding wrappers and control-flow primitives.
//!
//! `USize` wraps the platform-pointer-width `usize`; `Cap` wraps a
//! `USize` for const-generic capacity positions. `Bool` wraps `bool`
//! for predicate returns and implements `Try` so consumers can use
//! `?` on it.
//!
//! `AsBool` is the bridge trait for boundary call sites that need a
//! raw `bool`.

use core::cmp::Ordering;
use core::convert::Infallible;
use core::marker::ConstParamTy;
use core::ops::{
    Add, BitAnd, BitOr, BitXor, ControlFlow, Deref, Div, FromResidual, Mul, Not, Rem, Shl, Shr,
    Sub, Try,
};

use arvo_strategy::{Bounded, Identity};

use crate::bridges::{ConstDefault, ConstEq, ConstOrd, ConstOrdering};

/// Index / count newtype wrapping `usize`.
///
/// Wraps `usize` for the arvo-types-only lint. `Deref<Target = usize>`
/// gives `array[*idx]` ergonomics. Derives `ConstParamTy` so `USize`
/// values can be used inside other const-generic newtypes (see `Cap`).
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct USize(pub usize);

impl USize {
    /// Constant `USize(0)`.
    pub const ZERO: USize = USize(0);
    /// Constant `USize(1)`.
    pub const ONE: USize = USize(1);
    /// Constant `USize(usize::MAX)`.
    pub const MAX: USize = USize(usize::MAX);
}

impl Deref for USize {
    type Target = usize;
    #[inline(always)]
    fn deref(&self) -> &usize {
        &self.0
    }
}

impl const Add<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn add(self, rhs: USize) -> USize {
        USize(self.0 + rhs.0)
    }
}

impl const Sub<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn sub(self, rhs: USize) -> USize {
        USize(self.0 - rhs.0)
    }
}

impl const Mul<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn mul(self, rhs: USize) -> USize {
        USize(self.0 * rhs.0)
    }
}

impl const Div<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn div(self, rhs: USize) -> USize {
        USize(self.0 / rhs.0)
    }
}

impl const Rem<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn rem(self, rhs: USize) -> USize {
        USize(self.0 % rhs.0)
    }
}

impl const Shl<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn shl(self, rhs: USize) -> USize {
        USize(self.0 << rhs.0)
    }
}

impl const Shr<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn shr(self, rhs: USize) -> USize {
        USize(self.0 >> rhs.0)
    }
}

impl const BitAnd<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn bitand(self, rhs: USize) -> USize {
        USize(self.0 & rhs.0)
    }
}

impl const BitOr<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn bitor(self, rhs: USize) -> USize {
        USize(self.0 | rhs.0)
    }
}

impl const BitXor<USize> for USize {
    type Output = USize;
    #[inline(always)]
    fn bitxor(self, rhs: USize) -> USize {
        USize(self.0 ^ rhs.0)
    }
}

impl const Not for USize {
    type Output = USize;
    #[inline(always)]
    fn not(self) -> USize {
        USize(!self.0)
    }
}

impl PartialOrd<USize> for USize {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &USize) -> Option<Ordering> { // lint:allow(no-bare-option) reason: core::cmp::PartialOrd::partial_cmp trait-method signature returns Option<Ordering>; tracked: #115
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for USize {
    #[inline(always)]
    fn cmp(&self, rhs: &USize) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

/// Const-generic capacity for fixed-size structures.
///
/// Used wherever a const generic sizes an array. The type prevents
/// mixing capacities with unrelated integers. The const parameter
/// name carries the semantic distinction (`N`, `ROWS`, `NNZ`).
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Cap(pub USize);

impl Cap {
    /// Constant `Cap(USize::ZERO)`.
    pub const ZERO: Cap = Cap(USize::ZERO);
    /// Constant `Cap(USize::ONE)`.
    pub const ONE: Cap = Cap(USize::ONE);
}

impl const Add<Cap> for Cap {
    type Output = Cap;
    #[inline(always)]
    fn add(self, rhs: Cap) -> Cap {
        Cap(self.0.add(rhs.0))
    }
}

impl const Sub<Cap> for Cap {
    type Output = Cap;
    #[inline(always)]
    fn sub(self, rhs: Cap) -> Cap {
        Cap(self.0.sub(rhs.0))
    }
}

impl const Mul<Cap> for Cap {
    type Output = Cap;
    #[inline(always)]
    fn mul(self, rhs: Cap) -> Cap {
        Cap(self.0.mul(rhs.0))
    }
}

impl const Div<Cap> for Cap {
    type Output = Cap;
    #[inline(always)]
    fn div(self, rhs: Cap) -> Cap {
        Cap(self.0.div(rhs.0))
    }
}

impl const Rem<Cap> for Cap {
    type Output = Cap;
    #[inline(always)]
    fn rem(self, rhs: Cap) -> Cap {
        Cap(self.0.rem(rhs.0))
    }
}

impl PartialOrd<Cap> for Cap {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Cap) -> Option<Ordering> { // lint:allow(no-bare-option) reason: core::cmp::PartialOrd::partial_cmp trait-method signature returns Option<Ordering>; tracked: #115
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for Cap {
    #[inline(always)]
    fn cmp(&self, rhs: &Cap) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

/// Control-flow boolean.
///
/// Returned by predicates (`is_zero`, `bit`). Not a fixed-point
/// type. `Bool` is for branching, `Bit` (defined in arvo-bits) is
/// for 1-bit data storage.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Bool(pub bool);

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

impl Try for Bool {
    type Output = bool;
    type Residual = Infallible;

    #[inline(always)]
    fn from_output(output: bool) -> Self {
        Bool(output)
    }

    #[inline(always)]
    fn branch(self) -> ControlFlow<Infallible, bool> {
        ControlFlow::Continue(self.0)
    }
}

impl FromResidual<Infallible> for Bool {
    #[inline(always)]
    fn from_residual(residual: Infallible) -> Self {
        match residual {}
    }
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
        self.0
    }
}

impl From<bool> for Bool {
    #[inline(always)]
    fn from(b: bool) -> Self {
        Bool(b)
    }
}

impl From<Bool> for bool {
    #[inline(always)]
    fn from(b: Bool) -> Self {
        b.0
    }
}

// --- Canonical typed-const surfaces (round 202605021800) ----------------
//
// `Bounded`, `Identity`, `ConstEq`, `ConstOrd`, `ConstDefault` are the
// trait-canonical const surfaces. The substrate routes consumer code
// through these traits rather than facade-mirror inherent constants.
// Per the user's pushback in round 202605021800: re-defining ZERO/ONE/MAX
// as inherent constants on USize/Cap would duplicate Identity::ZERO etc.
// and create fragile one-off redirects. The trait IS the canonical
// surface; consumers reach for `<USize as Identity>::ZERO` (which
// `USize::ZERO` resolves to via trait associated-const lookup when
// `Identity` is in scope).

impl const Bounded for USize {
    const MIN: Self = USize(<usize as Bounded>::MIN);
    const MAX: Self = USize(<usize as Bounded>::MAX);
}

impl const Identity for USize {
    const ZERO: Self = USize(<usize as Identity>::ZERO);
    const ONE: Self = USize(<usize as Identity>::ONE);
}

impl const ConstEq for USize {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        Bool(self.0 == other.0)
    }
}

impl const ConstOrd for USize {
    #[inline(always)]
    fn const_cmp(&self, other: &Self) -> ConstOrdering {
        if self.0 < other.0 {
            ConstOrdering::Less
        } else if self.0 > other.0 {
            ConstOrdering::Greater
        } else {
            ConstOrdering::Equal
        }
    }
}

impl const ConstDefault for USize {
    #[inline(always)]
    fn const_default() -> Self {
        USize(0)
    }
}

impl const Bounded for Cap {
    const MIN: Self = Cap(<USize as Bounded>::MIN);
    const MAX: Self = Cap(<USize as Bounded>::MAX);
}

impl const Identity for Cap {
    const ZERO: Self = Cap(<USize as Identity>::ZERO);
    const ONE: Self = Cap(<USize as Identity>::ONE);
}

impl const ConstEq for Cap {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        self.0.const_eq(&other.0)
    }
}

impl const ConstOrd for Cap {
    #[inline(always)]
    fn const_cmp(&self, other: &Self) -> ConstOrdering {
        self.0.const_cmp(&other.0)
    }
}

impl const ConstDefault for Cap {
    #[inline(always)]
    fn const_default() -> Self {
        Cap(<USize as ConstDefault>::const_default())
    }
}

impl const ConstEq for Bool {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        Bool(self.0 == other.0)
    }
}

impl const ConstDefault for Bool {
    #[inline(always)]
    fn const_default() -> Self {
        Bool(false)
    }
}
