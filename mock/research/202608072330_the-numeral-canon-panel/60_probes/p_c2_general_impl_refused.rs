// Probe C2: DELIBERATE COMPILE FAILURE, kept as evidence.
//
// The general spelling of the exact-mul width algebra, one impl over all width
// pairs with the output widths computed in type position, requires
// generic_const_exprs, which is FORBIDDEN in this workspace. This file states
// that spelling without the gate so the refusal itself is on record. The
// recorded error (see p_c2.stderr beside this file) is the reason the accepted
// shape in p_c1 is a bounded enumeration or, at scale, a type-level-arithmetic
// trait contract per the refused-bound rule: the derivation lives in impls,
// the bound names a contract.
//
// Expected: E0747-family "generic parameters may not be used in const
// operations", suggesting the forbidden feature.

struct Q<const I: u32, const F: u32>(i128);

trait ExactMul<R> {
    type Out;
}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> ExactMul<Q<I2, F2>> for Q<I1, F1> {
    type Out = Q<{ I1 + I2 }, { F1 + F2 }>;
}

fn main() {}
