// TEST A: the spec's ACTUAL shape. Axes behind associated-type projections on
// Numeral / Policy, not as direct type parameters of Number as the sketch had.
// Question: does the law bound resolve through the projections, and what does
// the refusal look like?

pub trait Signedness {}
pub struct Unsigned;
impl Signedness for Unsigned {}
pub struct Signed;
impl Signedness for Signed {}

pub trait Resolution {}
pub struct TowardNegative;
impl Resolution for TowardNegative {}
pub struct TowardPositive;
impl Resolution for TowardPositive {}
pub struct ReduceModulo;
impl Resolution for ReduceModulo {}
pub struct SubstituteZero;
impl Resolution for SubstituteZero {}
pub struct Refuse;
impl Resolution for Refuse {}

pub trait Faithful: Resolution {}
impl Faithful for ReduceModulo {}

pub trait Quantisation {
    type OverRange: Resolution;
    type UnderRange: Resolution;
}
pub trait Numeral {
    type Sign: Signedness;
}
pub trait Policy {
    type Quantisation: Quantisation;
}
pub trait Lowering {}

pub struct Number<N: Numeral, S>(core::marker::PhantomData<(N, S)>)
where
    S: Policy + Lowering;

pub trait AddAssoc {}
impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}
impl<A: Faithful, B: Faithful> AddAssoc for ((A, B), Signed) {}

pub struct Add;
pub trait Semigroup<Op> {}

impl<N: Numeral, S: Policy + Lowering, Op> Semigroup<Op> for Number<N, S> where
    (
        (
            <S::Quantisation as Quantisation>::OverRange,
            <S::Quantisation as Quantisation>::UnderRange,
        ),
        <N as Numeral>::Sign,
    ): AddAssoc
{
}

// concrete instances
pub struct Wrapping;
impl Quantisation for Wrapping {
    type OverRange = ReduceModulo;
    type UnderRange = ReduceModulo;
}
pub struct Clamping;
impl Quantisation for Clamping {
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}

pub struct Hot;
impl Policy for Hot {
    type Quantisation = Wrapping;
}
impl Lowering for Hot {}
pub struct Warm;
impl Policy for Warm {
    type Quantisation = Clamping;
}
impl Lowering for Warm {}

pub struct I16;
impl Numeral for I16 {
    type Sign = Signed;
}
pub struct U16;
impl Numeral for U16 {
    type Sign = Unsigned;
}

fn fold<T: Semigroup<Add>>() {}

fn main() {
    fold::<Number<I16, Warm>>(); // signed clamping: must refuse

    println!("A OK");
}
