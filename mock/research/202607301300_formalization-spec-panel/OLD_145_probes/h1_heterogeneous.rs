//! The conversion the consumer never writes, because the operation takes both numerals.
//!
//! `131:239` shows `add(a, b)` over two operands of ONE numeral. This checks the
//! heterogeneous form: operands of two different numerals, output coordinates named by
//! the annotation, law checked at the signature. If this holds, mixing Q13.3 and Q8.8
//! in arithmetic needs no conversion at all, and the conversion chapter is only about
//! storing into a named format.
//!
//! The sum numeral is the JOIN of the two operand numerals plus one carry digit:
//! addnum((I1,F1),(I2,F2)) = (max(I1,I2) + 1, max(F1,F2)). The product numeral is
//! mulnum = (I1+I2, F1+F2), which is not a join.
//!
//! Also checks the strategy resolution as a type-level join, since a heterogeneous
//! operation can differ on the strategy as well as on the numeral.
//!
//! Build:
//!   rustc --edition 2021 --crate-type lib -Znext-solver=globally h1_heterogeneous.rs
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct Unsigned;
pub struct Signed;
#[derive(Clone, Copy)]
pub struct Hot;
#[derive(Clone, Copy)]
pub struct Warm;
#[derive(Clone, Copy)]
pub struct Cold;
#[derive(Clone, Copy)]
pub struct Precise;

pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}

// ------------------------------------------------------- the numeral-level maps
pub const fn max_u32(a: u32, b: u32) -> u32 {
    if a >= b {
        a
    } else {
        b
    }
}

/// The sum numeral: the join of the two, plus one carry digit.
pub struct SumFormat<
    const I1: u32,
    const F1: u32,
    const I2: u32,
    const F2: u32,
    const IR: u32,
    const FR: u32,
>;
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, const IR: u32, const FR: u32>
    SumFormat<I1, F1, I2, F2, IR, FR>
{
    pub const HOLDS: () = {
        assert!(
            IR == max_u32(I1, I2) + 1,
            "add: the result's integer digits must be one past the join of the operands'"
        );
        assert!(
            FR == max_u32(F1, F2),
            "add: the result's fraction digits must be the join of the operands'"
        );
    };
}

/// The product numeral: coordinatewise sum, and not a join.
pub struct ProductFormat<
    const I1: u32,
    const F1: u32,
    const I2: u32,
    const F2: u32,
    const IR: u32,
    const FR: u32,
>;
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, const IR: u32, const FR: u32>
    ProductFormat<I1, F1, I2, F2, IR, FR>
{
    pub const HOLDS: () = {
        assert!(
            IR == I1 + I2,
            "mul: the result's integer digits must be the sum of the operands'"
        );
        assert!(
            FR == F1 + F2,
            "mul: the result's fraction digits must be the sum of the operands'"
        );
    };
}

// ------------------------------------------------- the strategy order, as a join
pub trait Resolve<Other> {
    type Out;
}
macro_rules! resolve {
    ($a:ty, $b:ty, $o:ty) => {
        impl Resolve<$b> for $a {
            type Out = $o;
        }
    };
}
resolve!(Hot, Hot, Hot);
resolve!(Warm, Warm, Warm);
resolve!(Cold, Cold, Cold);
resolve!(Precise, Precise, Precise);
resolve!(Hot, Warm, Warm);
resolve!(Warm, Hot, Warm);
resolve!(Hot, Cold, Cold);
resolve!(Cold, Hot, Cold);
resolve!(Hot, Precise, Precise);
resolve!(Precise, Hot, Precise);
resolve!(Warm, Cold, Cold);
resolve!(Cold, Warm, Cold);
resolve!(Warm, Precise, Precise);
resolve!(Precise, Warm, Precise);
resolve!(Cold, Precise, Precise);
resolve!(Precise, Cold, Precise);

// ------------------------------------------------------------------- the laws
pub fn add<
    const I1: u32,
    const F1: u32,
    const I2: u32,
    const F2: u32,
    const IR: u32,
    const FR: u32,
    G,
    SA,
    SB,
>(
    _a: Fixed<I1, F1, G, SA>,
    _b: Fixed<I2, F2, G, SB>,
) -> Fixed<IR, FR, G, <SA as Resolve<SB>>::Out>
where
    SA: Resolve<SB>,
{
    let () = SumFormat::<I1, F1, I2, F2, IR, FR>::HOLDS;
    Fixed(PhantomData)
}

pub fn mul<
    const I1: u32,
    const F1: u32,
    const I2: u32,
    const F2: u32,
    const IR: u32,
    const FR: u32,
    G,
    SA,
    SB,
>(
    _a: Fixed<I1, F1, G, SA>,
    _b: Fixed<I2, F2, G, SB>,
) -> Fixed<IR, FR, G, <SA as Resolve<SB>>::Out>
where
    SA: Resolve<SB>,
{
    let () = ProductFormat::<I1, F1, I2, F2, IR, FR>::HOLDS;
    Fixed(PhantomData)
}

// ------------------------------------------------------------------ consumers
/// The antichain pair, added, with no conversion written anywhere.
pub fn mixed_add(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    let _s: UFixed<14, 8, Warm> = add(a, b);
}

/// The same pair, multiplied.
pub fn mixed_mul(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    let _p: UFixed<21, 11, Warm> = mul(a, b);
}

/// Homogeneous, which is the case `131:239` exercises. Still one law.
pub fn same_add(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) {
    let _s: UFixed<14, 3, Warm> = add(a, b);
}

/// Signed, where precision is sign-free and the sign marker is carried through.
pub fn signed_mul(a: IFixed<12, 3, Warm>, b: IFixed<12, 3, Warm>) {
    let _p: IFixed<24, 6, Warm> = mul(a, b);
}

/// Two numerals AND two strategies. The result takes the strategy join.
pub fn mixed_strategy(a: UFixed<13, 3, Hot>, b: UFixed<8, 8, Precise>) {
    let _s: UFixed<14, 8, Precise> = add(a, b);
}

pub fn mixed_strategy_cold(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Cold>) {
    let _s: UFixed<14, 8, Cold> = add(a, b);
}
