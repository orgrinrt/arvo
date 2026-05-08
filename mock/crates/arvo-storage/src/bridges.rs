//! Const-trait predicate bridges returning typed `Bool`.
//!
//! Three families:
//!
//! `ConstPartialEq` / `ConstEq`: equality. `ConstPartialEq` gives
//! NaN-aware partial equality (`a.const_eq(&a)` is FALSE when `a` is
//! NaN). `ConstEq` is a marker supertrait that promises reflexivity:
//! implementations of `ConstEq` guarantee `a.const_eq(&a) == TRUE`
//! for every value. Floats deliberately do not implement `ConstEq`
//! because of NaN; integer-like types implement both.
//!
//! `ConstBitEq`: bit-pattern equality. For float types the natural
//! reflexive equality reads `to_bits() == to_bits()`. The substrate
//! ships `ConstBitEq` separate from `ConstEq` so float-bit-pattern
//! equality is reachable without conflating it with value equality.
//!
//! `ConstOrd`: total ordering. Returns substrate-local
//! `ConstOrdering` (mirror of `core::cmp::Ordering`, locally defined
//! because constructing `core::cmp::Ordering` values inside `impl
//! const` bodies is not const-stable on rustc 1.96.0-nightly).
//!
//! `ConstDefault`: const-callable default construction.
//!
//! These traits return `Bool` rather than bare `bool` per the
//! no-bare-primitives discipline. `Bool` is reachable from this
//! crate (defined in `platform.rs`); `arvo-strategy` cannot return
//! `Bool` without inverting the layering, which is why this family
//! lives one layer below the canonical typed-const surfaces
//! (`Bounded`, `Identity`, `SignedIdentity`) that stay in
//! `arvo-strategy`.

use crate::platform::Bool;

/// Const-callable partial equality.
///
/// `pub const trait`. Returns `Bool`. NaN-aware: a `NaN` value
/// compared to itself returns `FALSE`. Use `ConstEq` (this trait's
/// marker supertrait) when reflexivity is required.
pub const trait ConstPartialEq {
    /// Const-callable `==`. Partial: `NaN == NaN` returns `FALSE`.
    fn const_eq(&self, other: &Self) -> Bool;

    /// Const-callable `!=`. Default body negates `const_eq`.
    #[inline(always)]
    fn const_ne(&self, other: &Self) -> Bool {
        let eq = self.const_eq(other);
        Bool(!eq.0)
    }
}

/// Const-callable equality with reflexivity guarantee.
///
/// `pub const trait`. Marker supertrait of `ConstPartialEq`.
/// Implementations of `ConstEq` promise that `a.const_eq(&a) ==
/// TRUE` for every representable value. Integer-like types impl
/// `ConstEq`; float-bearing types do NOT (NaN breaks reflexivity)
/// and stop at `ConstPartialEq`.
///
/// No methods on `ConstEq` itself; the call surface is inherited
/// from `ConstPartialEq`. The trait exists purely as a typed promise
/// the substrate can bound on when reflexivity matters.
pub const trait ConstEq: [const] ConstPartialEq {}

/// Const-callable bit-pattern equality.
///
/// `pub const trait`. Always reflexive: `a.const_bit_eq(&a) ==
/// TRUE` regardless of value. For integer-like types this coincides
/// with `ConstEq::const_eq`. For float types this compares
/// `to_bits()` representations, which makes `NaN.const_bit_eq(&NaN)`
/// return `TRUE` for any NaN with the same bit pattern (and `FALSE`
/// across NaN encodings).
pub const trait ConstBitEq {
    /// Const-callable bit-pattern equality. Always reflexive.
    fn const_bit_eq(&self, other: &Self) -> Bool;

    /// Const-callable bit-pattern inequality. Default body negates
    /// `const_bit_eq`.
    #[inline(always)]
    fn const_bit_ne(&self, other: &Self) -> Bool {
        let eq = self.const_bit_eq(other);
        Bool(!eq.0)
    }
}

/// Total-ordering result for `ConstOrd::const_cmp`.
///
/// Substrate-local mirror of `core::cmp::Ordering`. Used inside the
/// const ordering bridge because constructing `core::cmp::Ordering`
/// values inside `impl const` bodies is not const-stable on rustc
/// 1.96.0-nightly. Three discriminants: `Less`, `Equal`, `Greater`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum ConstOrdering {
    /// `self < other`.
    Less = -1,
    /// `self == other`.
    Equal = 0,
    /// `self > other`.
    Greater = 1,
}

impl From<core::cmp::Ordering> for ConstOrdering {
    #[inline(always)]
    fn from(o: core::cmp::Ordering) -> Self {
        match o {
            core::cmp::Ordering::Less => ConstOrdering::Less,
            core::cmp::Ordering::Equal => ConstOrdering::Equal,
            core::cmp::Ordering::Greater => ConstOrdering::Greater,
        }
    }
}

impl From<ConstOrdering> for core::cmp::Ordering {
    #[inline(always)]
    fn from(o: ConstOrdering) -> Self {
        match o {
            ConstOrdering::Less => core::cmp::Ordering::Less,
            ConstOrdering::Equal => core::cmp::Ordering::Equal,
            ConstOrdering::Greater => core::cmp::Ordering::Greater,
        }
    }
}

/// Const-callable total ordering.
///
/// `pub const trait`. Supertrait-bounded on `ConstEq` (which carries
/// reflexivity). Returns `Bool` from the predicate methods.
/// `const_lt` / `const_le` / `const_gt` / `const_ge` have default
/// bodies keyed on `const_cmp`.
///
/// Float-bearing types do NOT implement `ConstOrd` because NaN
/// breaks total ordering. The substrate's float total-ordering
/// surface routes through `arvo-numeric-contracts::TotalOrd`, which
/// folds NaN handling into a separate contract.
pub const trait ConstOrd: [const] ConstEq {
    /// Const-callable `cmp`.
    fn const_cmp(&self, other: &Self) -> ConstOrdering;

    /// Const-callable `<`.
    #[inline(always)]
    fn const_lt(&self, other: &Self) -> Bool {
        Bool(matches!(self.const_cmp(other), ConstOrdering::Less))
    }
    /// Const-callable `<=`.
    #[inline(always)]
    fn const_le(&self, other: &Self) -> Bool {
        Bool(!matches!(self.const_cmp(other), ConstOrdering::Greater))
    }
    /// Const-callable `>`.
    #[inline(always)]
    fn const_gt(&self, other: &Self) -> Bool {
        Bool(matches!(self.const_cmp(other), ConstOrdering::Greater))
    }
    /// Const-callable `>=`.
    #[inline(always)]
    fn const_ge(&self, other: &Self) -> Bool {
        Bool(!matches!(self.const_cmp(other), ConstOrdering::Less))
    }
}

/// Const-callable default construction.
///
/// `pub const trait`. The substrate's const-callable equivalent to
/// `core::default::Default::default`, which is not const-stable on
/// rustc 1.96.0-nightly.
pub const trait ConstDefault: Sized {
    /// Const-callable default value.
    fn const_default() -> Self;
}

// ---- bare-primitive impls ----------------------------------------------

macro_rules! impl_partial_eq_eq_ord_default_int {
    ($($ty:ty),+) => {
        $(
            impl const ConstPartialEq for $ty {
                #[inline(always)]
                fn const_eq(&self, other: &Self) -> Bool { Bool(*self == *other) }
            }
            impl const ConstEq for $ty {}
            impl const ConstBitEq for $ty {
                #[inline(always)]
                fn const_bit_eq(&self, other: &Self) -> Bool { Bool(*self == *other) }
            }
            impl const ConstOrd for $ty {
                #[inline(always)]
                fn const_cmp(&self, other: &Self) -> ConstOrdering {
                    if *self < *other { ConstOrdering::Less }
                    else if *self > *other { ConstOrdering::Greater }
                    else { ConstOrdering::Equal }
                }
            }
            impl const ConstDefault for $ty {
                #[inline(always)]
                fn const_default() -> Self { 0 }
            }
        )+
    };
}

impl_partial_eq_eq_ord_default_int!(u8, u16, u32, u64, u128, usize);
impl_partial_eq_eq_ord_default_int!(i8, i16, i32, i64, i128, isize);

// ---- bool: ConstPartialEq + ConstEq (reflexive) + ConstBitEq +
// ConstOrd (false < true) + ConstDefault. ConstOrd ships per the
// audit's H2 finding: silent absence is worst-case; ship a
// principled false<true ordering. -----------------------------------

impl const ConstPartialEq for bool {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        Bool(*self == *other)
    }
}

impl const ConstEq for bool {}

impl const ConstBitEq for bool {
    #[inline(always)]
    fn const_bit_eq(&self, other: &Self) -> Bool {
        Bool(*self == *other)
    }
}

impl const ConstOrd for bool {
    #[inline(always)]
    fn const_cmp(&self, other: &Self) -> ConstOrdering {
        match (*self, *other) {
            (false, false) | (true, true) => ConstOrdering::Equal,
            (false, true) => ConstOrdering::Less,
            (true, false) => ConstOrdering::Greater,
        }
    }
}

impl const ConstDefault for bool {
    #[inline(always)]
    fn const_default() -> Self {
        false
    }
}

// ---- float ConstPartialEq + ConstBitEq + ConstDefault.
//
// Floats deliberately do NOT implement ConstEq, ConstOrd. NaN breaks
// reflexivity (NaN != NaN under PartialEq), and total ordering on
// NaN-bearing types routes through arvo-numeric-contracts::TotalOrd.
// ConstBitEq on floats compares to_bits() representations, which is
// reflexive (NaN.to_bits() == NaN.to_bits() for the same NaN
// encoding). -----------------------------------------------------

macro_rules! impl_partial_eq_bit_eq_default_float {
    ($($ty:ty),+) => {
        $(
            impl const ConstPartialEq for $ty {
                #[inline(always)]
                fn const_eq(&self, other: &Self) -> Bool { Bool(*self == *other) }
            }
            impl const ConstBitEq for $ty {
                #[inline(always)]
                fn const_bit_eq(&self, other: &Self) -> Bool {
                    Bool(self.to_bits() == other.to_bits())
                }
            }
            impl const ConstDefault for $ty {
                #[inline(always)]
                fn const_default() -> Self { 0.0 }
            }
        )+
    };
}

impl_partial_eq_bit_eq_default_float!(f32, f64);
