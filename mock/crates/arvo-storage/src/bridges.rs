//! Const-trait predicate bridges returning typed `Bool`.
//!
//! `ConstEq`, `ConstOrd`, `ConstDefault` are the substrate's
//! const-callable equivalents to `core::cmp::PartialEq`,
//! `core::cmp::Ord`, and `core::default::Default`. Stdlib equivalents
//! are not const-callable on rustc 1.96.0-nightly; the substrate
//! ships its own surface so const contexts can still compare,
//! order, and default-construct primitive and substrate types.
//!
//! The traits return `Bool` rather than bare `bool` per the
//! no-bare-primitives discipline. `Bool` is reachable from this
//! crate (defined in `platform.rs`); `arvo-strategy` cannot return
//! `Bool` without inverting the layering, which is why this family
//! lives one layer below the canonical typed-const surfaces
//! (`Bounded`, `Identity`, `SignedIdentity`) that stay in
//! `arvo-strategy`.
//!
//! Per round 202605021800: every bare primitive (u8..u128, i8..i128,
//! usize, isize) has impls for ConstEq, ConstOrd, ConstDefault.
//! Substrate types (USize, Cap, Bits, UFixed, IFixed, FastFloat,
//! StrictFloat, MetaCarrier, IBits, FBits, Width) gain blanket impls
//! routed through the inner primitive's impl, in their owning files.

use crate::platform::Bool;

/// Const-callable equality.
///
/// `pub const trait`. Returns `Bool` rather than `bool` per the
/// no-bare-primitives discipline. `const_ne` has a default body
/// keyed on `const_eq`; impls only need to provide `const_eq`.
pub const trait ConstEq {
    /// Const-callable `==`.
    fn const_eq(&self, other: &Self) -> Bool;

    /// Const-callable `!=`. Default body negates `const_eq`.
    #[inline(always)]
    fn const_ne(&self, other: &Self) -> Bool {
        Bool(!self.const_eq(other).0)
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

/// Const-callable total ordering.
///
/// `pub const trait`. Supertrait-bounded on `ConstEq`. Returns
/// `Bool` from the predicate methods. `const_lt` / `const_le` /
/// `const_gt` / `const_ge` have default bodies keyed on `const_cmp`.
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

macro_rules! impl_const_eq_ord_default_unsigned {
    ($($ty:ty),+) => {
        $(
            impl const ConstEq for $ty {
                #[inline(always)]
                fn const_eq(&self, other: &Self) -> Bool { Bool(*self == *other) }
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

macro_rules! impl_const_eq_ord_default_signed {
    ($($ty:ty),+) => {
        $(
            impl const ConstEq for $ty {
                #[inline(always)]
                fn const_eq(&self, other: &Self) -> Bool { Bool(*self == *other) }
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

impl_const_eq_ord_default_unsigned!(u8, u16, u32, u64, u128, usize);
impl_const_eq_ord_default_signed!(i8, i16, i32, i64, i128, isize);

// ---- bool ConstEq + ConstDefault (no ConstOrd; only two values) -------

impl const ConstEq for bool {
    #[inline(always)]
    fn const_eq(&self, other: &Self) -> Bool {
        Bool(*self == *other)
    }
}

impl const ConstDefault for bool {
    #[inline(always)]
    fn const_default() -> Self {
        false
    }
}

// ---- float ConstEq + ConstDefault (no ConstOrd; NaN ordering is total
// only via the substrate's `TotalOrd` from numeric-contracts) ----------

macro_rules! impl_const_eq_default_float {
    ($($ty:ty),+) => {
        $(
            impl const ConstEq for $ty {
                #[inline(always)]
                fn const_eq(&self, other: &Self) -> Bool { Bool(*self == *other) }
            }
            impl const ConstDefault for $ty {
                #[inline(always)]
                fn const_default() -> Self { 0.0 }
            }
        )+
    };
}

impl_const_eq_default_float!(f32, f64);
