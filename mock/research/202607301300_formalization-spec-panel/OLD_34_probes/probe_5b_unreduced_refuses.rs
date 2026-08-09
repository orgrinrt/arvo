//! Probe 5b (the refusing half): under the unnormalised rational-adjustment
//! encoding, "the same numeral" is spelling-dependent, so a law stated as a
//! type-level equation over numerals is not even well formed: whether a
//! product inhabits "the" numeral of its quantum depends on how that numeral
//! was constructed, not on which numeral it is.
//!
//! The witness needs no contrivance: two perfectly reduced operand numerals,
//! quantum 2/3 and quantum 3/4, whose product formula yields the unreduced
//! spelling 6/12 of the numeral a consumer writes directly as 1/2. Value-equal
//! as rationals; distinct as types; a signature naming one refuses the other.
//!
//! This file is committed refusing, on purpose, per
//! `a-test-that-cannot-compile-is-the-finding.md`: the E0308 below is the
//! result, not an obstacle. Do not "fix" this file; the fix is a
//! normalisation obligation in the spec (see probe 5's header), after which
//! this encoding shape is not written at all.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_5b_unreduced_refuses.rs
//! Outcome: FAILS WITH E0308 (mismatched types), recorded verbatim in
//! OUTCOMES.md, against rustc 1.98.0-nightly (57d06900f 2026-05-27). The
//! diagnostic itself states the finding: expected `Adj { num: 6, den: 12 }`,
//! found `Adj { num: 1, den: 2 }`.

#![allow(dead_code)]
#![no_std]
#![feature(adt_const_params)]

use core::marker::ConstParamTy;
use core::marker::PhantomData;

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct Adj {
    pub num: u64,
    pub den: u64,
}

pub struct Numeral<const A: Adj>;

/// The unnormalised product: multiply componentwise, do not reduce. This is
/// file 28's rational-pair adjustment as literally stated, with no normal
/// form required anywhere in files 28, 30, 31 or 33. Note componentwise pair
/// multiplication is itself associative, so pure product chains do unify
/// with each other; the failure below is between a product's spelling and
/// the directly-written numeral it denotes, which is where a generic
/// consumer actually stands.
const fn mul_adj_unreduced(a: Adj, b: Adj) -> Adj {
    Adj {
        num: a.num * b.num,
        den: a.den * b.den,
    }
}

pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

// Two reduced operand numerals whose product spelling is unreduced.
const A1: Adj = Adj { num: 2, den: 3 };
const A2: Adj = Adj { num: 3, den: 4 };

// The product numeral as the formula spells it: 6/12.
const PRODUCT: Adj = mul_adj_unreduced(A1, A2);

// The numeral a consumer writes for the same quantum: 1/2.
const HALF: Adj = Adj { num: 1, den: 2 };

// Value-equal (6/12 == 1/2 as rationals), type-distinct. This call is the
// probe: it does not type-check.
pub fn product_does_not_inhabit_its_own_numeral() {
    same_type(
        PhantomData::<Numeral<PRODUCT>>,
        PhantomData::<Numeral<HALF>>,
    );
}
