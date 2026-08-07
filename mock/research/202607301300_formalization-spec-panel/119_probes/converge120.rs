#![allow(dead_code)]
#![feature(const_trait_impl)]
use core::marker::PhantomData;

pub trait ExponentForm {}
pub struct Implicit;
impl ExponentForm for Implicit {}
pub struct Ranged;
impl ExponentForm for Ranged {}
pub trait Numeral {
    type Exponent: ExponentForm;
}
pub type FormOf<N> = <N as Numeral>::Exponent;

pub trait Quantisation {}
pub struct QFix;
impl Quantisation for QFix {}
pub trait StorageLayout {}
pub struct Dense;
impl StorageLayout for Dense {}

// the two contracts, kind-keyed, as pub const trait (the design's spelling)
pub const trait Policy<F: ExponentForm> {
    type Quantisation: Quantisation;
}
pub const trait Lowering<F: ExponentForm> {
    type Layout: StorageLayout;
}

pub struct Warm;
impl Policy<Implicit> for Warm {
    type Quantisation = QFix;
}
impl Lowering<Implicit> for Warm {
    type Layout = Dense;
}

pub struct Fact<N, S>(PhantomData<(N, S)>);
pub trait AddAssoc {}

// 120 section 2's central result, re-run under the parameterised contracts:
// does a bound on Policy project a Lowering member?
impl<N: Numeral, S: Policy<FormOf<N>>> AddAssoc for Fact<N, S> where S::Layout: StorageLayout {}
