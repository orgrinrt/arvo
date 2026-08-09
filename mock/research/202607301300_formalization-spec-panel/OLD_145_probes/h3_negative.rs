//! Negative control for `h1`: a wrong output annotation must be refused, and the
//! message must name the law and the coordinates in the law's own order.
//!
//! Expected to fail. The failure text is the artifact.
//!
//! Build:
//!   rustc --edition 2021 --crate-type lib h3_negative.rs
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
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
/// Wrong: the join of 3 and 8 is 8, not 3. A consumer who forgot that the finer
/// grid wins writes this.
pub fn wrong_fraction(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    let _s: UFixed<14, 3, Warm> = add(a, b);
}

/// Wrong: no carry digit.
pub fn wrong_carry(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) {
    let _s: UFixed<13, 8, Warm> = add(a, b);
}
