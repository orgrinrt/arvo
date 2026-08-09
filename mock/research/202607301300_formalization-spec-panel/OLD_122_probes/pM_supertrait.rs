#![no_std]
#![allow(dead_code)]
extern crate base;
use base::*;
use core::marker::PhantomData;

pub trait Policy {
    type Quantisation: Quantisation;
}
pub trait Lowering {
    type Layout: StorageLayout;
    type Container;
}

// The convenience trait 117:330-333 warns about and 120:470-476 forbids.
pub trait Strategy: Policy + Lowering {}
impl<T: Policy + Lowering> Strategy for T {}

pub struct Fact<N, S>(PhantomData<(N, S)>);
pub trait AddAssoc {}
pub trait WideEnough {}
impl WideEnough for u16 {}

// A law crate bounding on the roof trait, naming NOTHING from the lowering
// vocabulary, discriminating on the cost axis anyway (120:281-291).
impl<N: Numeral, S: Strategy> AddAssoc for Fact<N, S> where S::Container: WideEnough {}

pub struct WarmDense;
pub struct ColdPacked;
pub struct Q;
impl Quantisation for Q {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = Refuse;
    type UnderRange = Refuse;
}
impl Policy for WarmDense {
    type Quantisation = Q;
}
impl Policy for ColdPacked {
    type Quantisation = Q;
}
impl Lowering for WarmDense {
    type Layout = Dense;
    type Container = u16;
}
impl Lowering for ColdPacked {
    type Layout = Bitpacked;
    type Container = u8;
}

fn needs<T: AddAssoc>() {}
pub fn dense() {
    needs::<Fact<U13F3, WarmDense>>();
}
pub fn packed() {
    needs::<Fact<U13F3, ColdPacked>>();
}
