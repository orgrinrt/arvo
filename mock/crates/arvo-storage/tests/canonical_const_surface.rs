//! Canonical const surface smoke test for round 202605021800.
//!
//! Validates that every substrate primitive carries the full
//! canonical const surface (Bounded + Identity<Additive> + ConstPartialEq +
//! ConstEq + ConstBitEq + ConstOrd + ConstDefault) where
//! semantically applicable, and that consumers reach these constants
//! through trait projection without ever needing `.0` field access
//! on the trait-projected value.
//!
//! Float types (f32 / f64 / FastFloat / StrictFloat) impl
//! ConstPartialEq + ConstBitEq but NOT ConstEq (NaN breaks
//! reflexivity) and NOT ConstOrd (NaN breaks total ordering). Bool
//! impls everything except Bounded (only two values; MIN/MAX would
//! be redundant).

#![feature(const_trait_impl)]
#![feature(const_ops)]

use arvo_storage::{
    fbits, ibits, width, Bool, Cap, ConstBitEq, ConstDefault, ConstOrd, ConstOrdering,
    ConstPartialEq, MetaCarrier, USize,
};
use arvo_strategy::{Additive, Bounded, Identity, Multiplicative, SignedIdentity};

// ---- USize: full surface ----

const _USIZE_MIN: USize = <USize as Bounded>::MIN;
const _USIZE_MAX: USize = <USize as Bounded>::MAX;
const _USIZE_ZERO: USize = <USize as Identity<Additive>>::IDENTITY;
const _USIZE_ONE: USize = <USize as Identity<Multiplicative>>::IDENTITY;
const _USIZE_DEFAULT: USize = <USize as ConstDefault>::const_default();
const _USIZE_LT: Bool = <USize as ConstOrd>::const_lt(&USize(3), &USize(5));
const _USIZE_EQ: Bool = <USize as ConstPartialEq>::const_eq(&USize(7), &USize(7));
const _USIZE_BIT_EQ: Bool = <USize as ConstBitEq>::const_bit_eq(&USize(7), &USize(7));

// ---- Cap: full surface (parallel to USize, audit-fixed asymmetry) ----

const _CAP_MIN: Cap = <Cap as Bounded>::MIN;
const _CAP_MAX: Cap = <Cap as Bounded>::MAX;
const _CAP_ZERO: Cap = <Cap as Identity<Additive>>::IDENTITY;
const _CAP_ONE: Cap = <Cap as Identity<Multiplicative>>::IDENTITY;
const _CAP_DEFAULT: Cap = <Cap as ConstDefault>::const_default();
const _CAP_LT: Bool = <Cap as ConstOrd>::const_lt(&_CAP_ZERO, &_CAP_ONE);

// Bit ops landed for Cap as part of the audit's H7 unification: Cap
// previously lacked Shl/Shr/BitAnd/BitOr/BitXor/Not. Now identical to
// USize's surface via the impl_unsigned_integer_newtype! macro.
const _CAP_SHL: Cap = Cap(USize(1)) << Cap(USize(4));
const _CAP_AND: Cap = Cap(USize(0xFF)) & Cap(USize(0x0F));
const _CAP_NOT: Cap = !Cap(USize(0));

// ---- Bool: full surface except Bounded ----

const _BOOL_TRUE: Bool = Bool::TRUE;
const _BOOL_FALSE: Bool = Bool::FALSE;
const _BOOL_DEFAULT: Bool = <Bool as ConstDefault>::const_default();
const _BOOL_EQ: Bool = <Bool as ConstPartialEq>::const_eq(&Bool(true), &Bool(true));
const _BOOL_LT: Bool = <Bool as ConstOrd>::const_lt(&Bool(false), &Bool(true));

// ---- MetaCarrier + meta-bits: full integer-newtype surface ----

const _META_MIN: MetaCarrier = <MetaCarrier as Bounded>::MIN;
const _META_MAX: MetaCarrier = <MetaCarrier as Bounded>::MAX;
const _META_ZERO: MetaCarrier = <MetaCarrier as Identity<Additive>>::IDENTITY;
const _IBITS_ZERO: arvo_storage::IBits = <arvo_storage::IBits as Identity<Additive>>::IDENTITY;
const _IBITS_ONE: arvo_storage::IBits = <arvo_storage::IBits as Identity<Multiplicative>>::IDENTITY;
const _FBITS_DEFAULT: arvo_storage::FBits = <arvo_storage::FBits as ConstDefault>::const_default();
const _WIDTH_MAX: arvo_storage::Width = <arvo_storage::Width as Bounded>::MAX;

// ---- bare primitive: SignedIdentity reaches NEG_ONE through trait ----

const _I8_NEG_ONE: i8 = <i8 as SignedIdentity>::NEG_ONE;
const _I64_NEG_ONE: i64 = <i64 as SignedIdentity>::NEG_ONE;
const _ISIZE_NEG_ONE: isize = <isize as SignedIdentity>::NEG_ONE;
const _F32_NEG_ONE: f32 = <f32 as SignedIdentity>::NEG_ONE;
const _F64_NEG_ONE: f64 = <f64 as SignedIdentity>::NEG_ONE;

// ---- bare primitive: full surface ----

const _U64_BOUNDED_MAX: u64 = <u64 as Bounded>::MAX;
const _U64_IDENTITY_ONE: u64 = <u64 as Identity<Multiplicative>>::IDENTITY;
const _U64_DEFAULT: u64 = <u64 as ConstDefault>::const_default();

// Float canonical (no ConstEq, no ConstOrd; ConstPartialEq +
// ConstBitEq + Bounded + Identity<Additive> + ConstDefault).
const _F32_MIN: f32 = <f32 as Bounded>::MIN;
const _F32_MAX: f32 = <f32 as Bounded>::MAX;
const _F32_ZERO: f32 = <f32 as Identity<Additive>>::IDENTITY;
const _F32_ONE: f32 = <f32 as Identity<Multiplicative>>::IDENTITY;
const _F32_DEFAULT: f32 = <f32 as ConstDefault>::const_default();

#[test]
fn typed_const_surfaces_resolve_at_runtime() {
    // USize trait-projected constants match expected primitive values.
    assert_eq!(_USIZE_ZERO, USize(0));
    assert_eq!(_USIZE_ONE, USize(1));
    assert_eq!(_USIZE_MIN, USize(usize::MIN));
    assert_eq!(_USIZE_MAX, USize(usize::MAX));
    assert_eq!(_USIZE_DEFAULT, USize(0));
    assert_eq!(_USIZE_LT, Bool(true));
    assert_eq!(_USIZE_EQ, Bool(true));
    assert_eq!(_USIZE_BIT_EQ, Bool(true));

    // Cap parallel.
    assert_eq!(_CAP_ZERO, Cap(USize(0)));
    assert_eq!(_CAP_ONE, Cap(USize(1)));
    assert_eq!(_CAP_DEFAULT, Cap(USize(0)));
    assert_eq!(_CAP_LT, Bool(true));
    assert_eq!(_CAP_SHL, Cap(USize(16)));
    assert_eq!(_CAP_AND, Cap(USize(0x0F)));

    // Bool.
    assert_eq!(_BOOL_DEFAULT, Bool(false));
    assert_eq!(_BOOL_EQ, Bool(true));
    assert_eq!(_BOOL_LT, Bool(true));

    // SignedIdentity bottom-out values.
    assert_eq!(_I8_NEG_ONE, -1i8);
    assert_eq!(_I64_NEG_ONE, -1i64);
    assert_eq!(_F32_NEG_ONE, -1.0f32);
    assert_eq!(_F64_NEG_ONE, -1.0f64);
}

#[test]
fn const_partial_eq_reflexivity_for_floats() {
    // Float ConstPartialEq is non-reflexive on NaN: this is the
    // documented contract that motivated the ConstEq vs
    // ConstPartialEq split (audit C1 finding).
    let nan = f32::NAN;
    assert_eq!(<f32 as ConstPartialEq>::const_eq(&nan, &nan), Bool(false));
    // ConstBitEq is reflexive even for NaN (compares to_bits()).
    assert_eq!(<f32 as ConstBitEq>::const_bit_eq(&nan, &nan), Bool(true));
}

#[test]
fn const_ord_const_cmp_works() {
    let lt = <USize as ConstOrd>::const_cmp(&USize(3), &USize(5));
    assert!(matches!(lt, ConstOrdering::Less));
    let eq = <USize as ConstOrd>::const_cmp(&USize(7), &USize(7));
    assert!(matches!(eq, ConstOrdering::Equal));
    let gt = <USize as ConstOrd>::const_cmp(&USize(9), &USize(2));
    assert!(matches!(gt, ConstOrdering::Greater));
}

// Helper-ergonomics check: domain construction goes through ibits /
// fbits / width helpers (the documented exception per
// no-bare-primitives.md). We never reach `.0` on the trait
// projection; the helper IS the boundary.
const _IBITS_FROM_HELPER: arvo_storage::IBits = ibits(8);
const _FBITS_FROM_HELPER: arvo_storage::FBits = fbits(0);
const _WIDTH_FROM_HELPER: arvo_storage::Width = width(32);
