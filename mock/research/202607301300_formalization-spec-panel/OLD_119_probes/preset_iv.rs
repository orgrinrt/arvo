#![allow(dead_code)]
use core::marker::PhantomData;

// No new vocabulary at all. The key is the exponent form the design
// already declares sealed with exactly two constructors (1.2, 1.23).
pub trait Exponent {}
pub struct EZero;
impl Exponent for EZero {}
pub trait Adjustment {}
pub struct Unit;
impl Adjustment for Unit {}
pub trait Bias {}
pub struct BZero;
impl Bias for BZero {}
pub trait Underflow {}
pub struct Gradual;
impl Underflow for Gradual {}
pub trait Specials {}
pub struct NoSpecials;
impl Specials for NoSpecials {}

pub trait ExponentForm {}
pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>(PhantomData<(E, A, B)>);
pub struct Ranged<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>(
    PhantomData<(EMIN, EMAX, U, S)>,
);
impl<E: Exponent, A: Adjustment, B: Bias> ExponentForm for Implicit<E, A, B> {}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
}

pub trait Numeral {
    type Exponent: ExponentForm;
}
pub type FormOf<N> = <N as Numeral>::Exponent;

pub struct Fix13_3;
impl Numeral for Fix13_3 {
    type Exponent = Implicit<EZero, Unit, BZero>;
}
pub struct Fix7_9;
impl Numeral for Fix7_9 {
    type Exponent = Implicit<EZero, Unit, BZero>;
}
pub struct Binary32;
impl Numeral for Binary32 {
    type Exponent = Ranged<EZero, EZero, Gradual, NoSpecials>;
}

pub trait StoredWidth {}
pub struct Minimum;
impl StoredWidth for Minimum {}
pub struct DoubleLogical;
impl StoredWidth for DoubleLogical {}
pub trait LoweringDoor {}
pub struct Inert;
impl LoweringDoor for Inert {}
pub struct HostFloat;
impl LoweringDoor for HostFloat {}
pub struct Quantised;
impl LoweringDoor for Quantised {}

// SPELLING (iv): keyed on the exponent form, quantified over its parameters.
pub trait Lowering<F: ExponentForm> {
    type StoredWidth: StoredWidth;
    type Door: LoweringDoor;
}
pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

impl<E: Exponent, A: Adjustment, B: Bias> Lowering<Implicit<E, A, B>> for Warm {
    type StoredWidth = DoubleLogical;
    type Door = Inert;
}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> Lowering<Ranged<EMIN, EMAX, U, S>>
    for Warm
{
    type StoredWidth = Minimum;
    type Door = HostFloat;
}

pub struct Number<N: Numeral, S: Lowering<FormOf<N>>>(PhantomData<(N, S)>);

pub fn check() {
    let _: <Warm as Lowering<FormOf<Fix13_3>>>::StoredWidth = DoubleLogical;
    let _: <Warm as Lowering<FormOf<Fix7_9>>>::StoredWidth = DoubleLogical;
    let _: <Warm as Lowering<FormOf<Binary32>>>::StoredWidth = Minimum;
}
