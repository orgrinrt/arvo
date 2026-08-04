//! Probe 4: `Implicit`'s own single exponent, moved to a type. This closes the carve-out
//! file 50 was honest about leaving open (`50:602-604`): the two-expert threshold was met
//! on the `Ranged` bounds and explicitly not on whether `Implicit`'s single exponent moves
//! at the same time.
//!
//! rustc --edition 2021 --crate-type lib probe_4_implicit_exponent_as_type.rs
//!
//! It does move, for the identical reason and not by analogy: `mulnum` over two `Implicit`
//! numerals computes `E1 + E2` and the sum appears in the result numeral's TYPE. The
//! negative control is `probe_4b`, which shows the const route is closed for the `Implicit`
//! case exactly as file 50's `probe_3b` showed it for the `Ranged` case.
//!
//! There is a second question underneath, which nobody has asked and which decides whether
//! `E` is redundant rather than merely const-or-type. `Implicit<E, A, B>` has BOTH an
//! exponent and a rational adjustment, and `A * radix^E` could be folded into a single
//! rational `A'`. If it could be folded at no cost, `E` would be a redundant axis and the
//! spine-rule question about it would not arise. It cannot: folding costs a `Pos`
//! constructor nest proportional to `|E| * log2(radix)` where keeping the axes apart costs
//! one proportional to `log2 |E|`. Probe 5 measures that; here it is stated so that this
//! probe's answer is not read as an isolated preference.

#![allow(dead_code)]

#[path = "vu_bias_sealed_adj.rs"]
pub mod bias;
#[path = "numeral.rs"]
pub mod numeral;

use bias::nat::{Adjustment, Pos, Ratio, H, O};
use bias::BZero;
use numeral::*;

type P2t = O<H>;
type P4t = O<P2t>;
type P8t = O<P4t>;
type P32t = O<O<P8t>>;
type P256t = O<O<O<P32t>>>;

const _: () = assert!(<P32t as Pos>::VAL == 32);
const _: () = assert!(<P256t as Pos>::VAL == 256);

// Dyadic adjustments: quantum 1/4, 1/8, and their product 1/32.
pub type A4 = Ratio<H, P4t>;
pub type A8 = Ratio<H, P8t>;
pub type A32 = Ratio<H, P32t>;
pub type A256 = Ratio<H, P256t>;

/// `AdjProduct` at concrete adjustments, per the projection-chain constraint: `Reduce`
/// never appears in a chain that reaches `MulNum`'s signature. For a unit numerator the
/// product is already reduced, which is why these three lines need no gcd at all and why
/// file 53 measured the same profile in the cheap band (`53:133-136`).
impl AdjProduct<A8> for A4 {
    type Out = A32;
}
impl AdjProduct<A32> for A8 {
    type Out = A256;
}
impl AdjProduct<A4> for A4 {
    type Out = Ratio<H, P16t>;
}
type P16t = O<P8t>;

// ---------------------------------------------------------------------------
// the fixed numerals, with the exponent as a type
// ---------------------------------------------------------------------------

/// A Q-format-shaped numeral: radix two, precision 8, exponent -4, quantum adjustment 1/4,
/// no bias. The exponent and the adjustment are two different quantities and both are types.
pub type Q1 = Fx<Two, P8t, ENeg<P4t>, A4, BZero, Symmetric>;
/// The second operand: exponent -2, adjustment 1/8.
pub type Q2 = Fx<Two, P4t, ENeg<P2t>, A8, BZero, Symmetric>;

pub type Q12 = <Q1 as MulNum<Q2>>::Out;

const _: () = assert!(<Q1 as Numeral>::P == 8);
const _: () = assert!(<Q1 as Numeral>::EMIN == -4);
const _: () = assert!(<Q2 as Numeral>::EMIN == -2);

/// The three quantities the spine rule is about, all computed and all in the result type:
/// the precision sum, the exponent sum, and the adjustment product.
const _: () = assert!(<Q12 as Numeral>::P == 12);
const _: () = assert!(<Q12 as Numeral>::EMIN == -6);
const _: () = assert!(<Q12 as Numeral>::EMAX == -6);

/// The adjustment came out of the projection rather than being asserted: 1/4 times 1/8.
const _: () = assert!(<A32 as Adjustment>::NUM == 1);
const _: () = assert!(<A32 as Adjustment>::DEN == 32);

/// A decimal fixed numeral. Radix ten, exponent -2, which is a currency quantum: the
/// exponent axis carries the whole scale and the adjustment is one.
pub type A1 = Ratio<H, H>;
impl AdjProduct<A1> for A1 {
    type Out = A1;
}
pub type Money = Fx<Ten, P8t, ENeg<P2t>, A1, BZero, Symmetric>;
pub type MoneySq = <Money as MulNum<Money>>::Out;
const _: () = assert!(<Money as Numeral>::R == 10);
const _: () = assert!(<Money as Numeral>::EMIN == -2);
const _: () = assert!(<MoneySq as Numeral>::EMIN == -4);
const _: () = assert!(<MoneySq as Numeral>::P == 16);

/// A three-way composition, so the exponent chain is not one step deep. Exponents -4, -2
/// and -2 sum to -8, and every intermediate result numeral is a type the next step consumes.
pub type Q3 = Fx<Two, P4t, ENeg<P2t>, A4, BZero, Symmetric>;
impl AdjProduct<A4> for A32 {
    type Out = Ratio<H, P128t>;
}
type P128t = O<O<O<O<P8t>>>>;
pub type Q123 = <Q12 as MulNum<Q3>>::Out;
const _: () = assert!(<Q123 as Numeral>::EMIN == -8);
const _: () = assert!(<Q123 as Numeral>::P == 16);

/// Forced through a signature. `MulNum` reaches a consumer-facing position and compiles,
/// which is the projection-chain constraint holding for the `Implicit` half as well as the
/// `Ranged` half.
pub fn mul_fixed<N1, N2>() -> (i64, u64)
where
    N1: Numeral + MulNum<N2>,
    N2: Numeral,
{
    (
        <<N1 as MulNum<N2>>::Out as Numeral>::EMIN,
        <<N1 as MulNum<N2>>::Out as Numeral>::P,
    )
}

pub fn call_mul_fixed() -> ((i64, u64), (i64, u64)) {
    (mul_fixed::<Q1, Q2>(), mul_fixed::<Money, Money>())
}
