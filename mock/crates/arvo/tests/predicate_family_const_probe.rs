//! Const-context probe for the Predicate family wrapper-impl design.
//!
//! Round 202605021800 resolves the design tension in the Predicate
//! family by introducing wrapper types `IsZeroOf<T>` / `IsPositiveOf<T>`
//! / `IsNonZeroOf<T>` / `IsNonNegativeOf<T>` / `IsZeroOrPositiveOf<T>`.
//! Each wrapper carries its own `Predicate::test()` body and the
//! matching marker subtrait. Generic blankets in arvo-numeric-contracts
//! cover any `T: ConstPartialEq + Identity` (zero-equality predicates)
//! or `T: ConstSign` (sign predicates). Floats impl `ConstSign`
//! directly in arvo facade.
//!
//! This probe validates const-callability of all five wrappers across
//! UFixed, IFixed, and float wrappers.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo::{FastFloat, IFixed, StrictFloat, UFixed, fbits, ibits};
use arvo::strategy::Hot;
use arvo::traits::FromConstant;
use arvo_numeric_contracts::{
    IsNonNegativeOf, IsNonZeroOf, IsPositiveOf, IsZeroOf, IsZeroOrPositiveOf, Predicate,
};
use arvo_storage::{Bool, USize};

type U16 = UFixed<{ ibits(16) }, { fbits(0) }, Hot>;
type I16 = IFixed<{ ibits(15) }, { fbits(0) }, Hot>;

// ---- IsZeroOf via ConstPartialEq + Identity blanket ----

const _U16_ZERO_TEST: Bool = {
    let n = <U16 as FromConstant>::from_constant::<{ USize(0) }>();
    <IsZeroOf<U16> as Predicate>::test(IsZeroOf(n))
};
const _U16_NONZERO_AS_ZERO: Bool = {
    let n = <U16 as FromConstant>::from_constant::<{ USize(5) }>();
    <IsZeroOf<U16> as Predicate>::test(IsZeroOf(n))
};

// ---- IsNonZeroOf ----

const _U16_FIVE_NONZERO: Bool = {
    let n = <U16 as FromConstant>::from_constant::<{ USize(5) }>();
    <IsNonZeroOf<U16> as Predicate>::test(IsNonZeroOf(n))
};
const _U16_ZERO_AS_NONZERO: Bool = {
    let n = <U16 as FromConstant>::from_constant::<{ USize(0) }>();
    <IsNonZeroOf<U16> as Predicate>::test(IsNonZeroOf(n))
};

// ---- IsPositiveOf via ConstSign (auto-blanket from ConstOrd + Identity) ---

const _I16_THREE_POSITIVE: Bool = {
    let n = <I16 as FromConstant>::from_constant::<{ USize(3) }>();
    <IsPositiveOf<I16> as Predicate>::test(IsPositiveOf(n))
};
const _I16_ZERO_NOT_POSITIVE: Bool = {
    let n = <I16 as FromConstant>::from_constant::<{ USize(0) }>();
    <IsPositiveOf<I16> as Predicate>::test(IsPositiveOf(n))
};

// ---- IsNonNegativeOf, IsZeroOrPositiveOf — same body, distinct markers --

const _I16_ZERO_NON_NEGATIVE: Bool = {
    let n = <I16 as FromConstant>::from_constant::<{ USize(0) }>();
    <IsNonNegativeOf<I16> as Predicate>::test(IsNonNegativeOf(n))
};
const _I16_FIVE_ZERO_OR_POSITIVE: Bool = {
    let n = <I16 as FromConstant>::from_constant::<{ USize(5) }>();
    <IsZeroOrPositiveOf<I16> as Predicate>::test(IsZeroOrPositiveOf(n))
};

// ---- Float wrappers via direct ConstSign impl in arvo facade -----------

const _F32_THREE_POSITIVE: Bool = <IsPositiveOf<FastFloat<f32>> as Predicate>::test(
    IsPositiveOf(FastFloat(3.0)),
);
const _F32_NEG_NOT_POSITIVE: Bool = <IsPositiveOf<FastFloat<f32>> as Predicate>::test(
    IsPositiveOf(FastFloat(-3.0)),
);
const _F64_ZERO_NON_NEGATIVE: Bool = <IsNonNegativeOf<StrictFloat<f64>> as Predicate>::test(
    IsNonNegativeOf(StrictFloat(0.0)),
);

// ---- Float zero-equality via ConstPartialEq blanket --------------------

const _F32_ZERO_IS_ZERO: Bool =
    <IsZeroOf<FastFloat<f32>> as Predicate>::test(IsZeroOf(FastFloat(0.0)));
const _F32_NAN_NOT_ZERO: Bool =
    <IsZeroOf<FastFloat<f32>> as Predicate>::test(IsZeroOf(FastFloat(f32::NAN)));

#[test]
fn predicate_family_resolves_at_runtime() {
    // Zero-equality predicates.
    assert_eq!(_U16_ZERO_TEST, Bool(true));
    assert_eq!(_U16_NONZERO_AS_ZERO, Bool(false));
    assert_eq!(_U16_FIVE_NONZERO, Bool(true));
    assert_eq!(_U16_ZERO_AS_NONZERO, Bool(false));

    // Sign predicates on signed fixed.
    assert_eq!(_I16_THREE_POSITIVE, Bool(true));
    assert_eq!(_I16_ZERO_NOT_POSITIVE, Bool(false));
    assert_eq!(_I16_ZERO_NON_NEGATIVE, Bool(true));
    assert_eq!(_I16_FIVE_ZERO_OR_POSITIVE, Bool(true));

    // Float sign predicates via ConstSign in arvo facade.
    assert_eq!(_F32_THREE_POSITIVE, Bool(true));
    assert_eq!(_F32_NEG_NOT_POSITIVE, Bool(false));
    assert_eq!(_F64_ZERO_NON_NEGATIVE, Bool(true));

    // Float zero-equality. NaN never equals 0.0 (correct).
    assert_eq!(_F32_ZERO_IS_ZERO, Bool(true));
    assert_eq!(_F32_NAN_NOT_ZERO, Bool(false));
}

#[test]
fn predicate_family_runtime_float_negatives() {
    // FastFloat<f32> negative: IsPositive false, IsNonNegative false.
    let neg = FastFloat(-3.0_f32);
    let pos_test = <IsPositiveOf<FastFloat<f32>> as Predicate>::test(IsPositiveOf(neg));
    let nn_test = <IsNonNegativeOf<FastFloat<f32>> as Predicate>::test(IsNonNegativeOf(neg));
    assert_eq!(pos_test, Bool(false));
    assert_eq!(nn_test, Bool(false));

    // NaN: every sign predicate returns false (NaN is not >= or > 0).
    let nan = FastFloat(f32::NAN);
    let nan_pos = <IsPositiveOf<FastFloat<f32>> as Predicate>::test(IsPositiveOf(nan));
    let nan_nn = <IsNonNegativeOf<FastFloat<f32>> as Predicate>::test(IsNonNegativeOf(nan));
    let nan_zop = <IsZeroOrPositiveOf<FastFloat<f32>> as Predicate>::test(IsZeroOrPositiveOf(nan));
    assert_eq!(nan_pos, Bool(false));
    assert_eq!(nan_nn, Bool(false));
    assert_eq!(nan_zop, Bool(false));
}
