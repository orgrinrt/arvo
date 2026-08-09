// Probe D. Key both strategy contracts on the exponent form.
// The number kind IS the exponent form: 110:880 bounds Numeral::Exponent by
// ExponentForm, and 110:3181 gives ExponentForm exactly two constructors,
// Implicit and Ranged. There are exactly two ratified tables.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
use base::*;
use core::marker::PhantomData;

pub const trait Policy<F: ExponentForm> {
    type Quantisation: Quantisation;
}
pub const trait Lowering<F: ExponentForm> {
    type StoredWidth: StoredWidth;
    type Layout: StorageLayout;
    type Door: LoweringDoor;
    type Container;
}

// The four ratified preset names stay four names.
pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

pub struct QFixedWarm;
const impl Quantisation for QFixedWarm {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive; // clamp
}
pub struct QFloatWarm;
const impl Quantisation for QFloatWarm {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = FarPoint;
    type UnderRange = FarPoint;
}

// Warm's fixed-point row (110:2674): doubled, dense, inert.
const impl<E: Exponent, A: Adjustment, B: Bias> Lowering<Implicit<E, A, B>> for Warm {
    type StoredWidth = DoubleLogical;
    type Layout = Dense;
    type Door = Inert;
    type Container = u32;
}
const impl<E: Exponent, A: Adjustment, B: Bias> Policy<Implicit<E, A, B>> for Warm {
    type Quantisation = QFixedWarm;
}

// Warm's float row (110:2727): minimum, dense, HostFloat. The divergence
// 110:2738 calls "the sharpest single finding the re-derivation produced".
const impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>
    Lowering<Ranged<EMIN, EMAX, U, S>> for Warm
{
    type StoredWidth = Minimum;
    type Layout = Dense;
    type Door = HostFloat<DefaultEnv>;
    type Container = u32;
}
const impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>
    Policy<Ranged<EMIN, EMAX, U, S>> for Warm
{
    type Quantisation = QFloatWarm;
}

// The kind is PROJECTED out of the numeral, never supplied alongside it.
pub struct Number<N: Numeral, S: Policy<N::Exponent> + Lowering<N::Exponent>> {
    datum: <S as Lowering<N::Exponent>>::Container,
    _numeral: PhantomData<N>,
}

// Both real pairings are writable and each reads its own table's row.
pub type Fixed = Number<U13F3, Warm>;
pub type Float = Number<Binary32, Warm>;

// The rows are recoverable, and they are the ratified ones. If these two
// projections resolved to the same type the whole exercise would be void.
pub fn fixed_row() -> <Warm as Lowering<<U13F3 as Numeral>::Exponent>>::StoredWidth {
    DoubleLogical
}
pub fn float_row() -> <Warm as Lowering<<Binary32 as Numeral>::Exponent>>::StoredWidth {
    Minimum
}

// The second numeral of the same kind gets the same row by construction: there
// is no per-numeral impl to disagree at. U14F2 is Implicit, so it reads the
// fixed row, and this type-checks only because that is so.
pub fn same_kind_same_row() -> <Warm as Lowering<<U14F2 as Numeral>::Exponent>>::StoredWidth {
    DoubleLogical
}

// Crosses keeps its supertrait edge under the parameterisation (110:3110).
pub unsafe trait Crosses<N: Numeral>: Lowering<N::Exponent> {}

pub struct Fact<N, S>(core::marker::PhantomData<(N, S)>);
pub trait AddAssoc {}
pub trait IsDense {}
// The bare projection, which is the spelling file 120 used (120:149).
impl<N: Numeral, S: Policy<N::Exponent>> AddAssoc for Fact<N, S> where S::Layout: IsDense {}
