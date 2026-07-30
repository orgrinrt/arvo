// TEST D: does the spec's single `S: Policy + Lowering` parameter carry its own
// stated invariant, "conditioning a law on a Lowering member would be
// conditioning correctness on a storage choice"?

#![allow(dead_code)]
pub trait Quantisation {}
pub struct Wrapping;
impl Quantisation for Wrapping {}
pub trait StorageLayout {}
pub struct Dense;
impl StorageLayout for Dense {}
pub struct Bitpacked;
impl StorageLayout for Bitpacked {}

pub trait Numeral {}
pub struct I16;
impl Numeral for I16 {}
pub trait Policy {
    type Quantisation: Quantisation;
}
pub trait Lowering {
    type Layout: StorageLayout;
}

pub struct Number<N: Numeral, S>(core::marker::PhantomData<(N, S)>)
where
    S: Policy + Lowering;

pub struct Add;
pub trait Semigroup<Op> {}

// a law derivation nobody would notice reading the impl header, keyed on the
// STORAGE LAYOUT. The spec forbids this in prose. Nothing types against it.
pub trait LawsOk {}
impl LawsOk for Dense {} // and deliberately not for Bitpacked

impl<N: Numeral, S: Policy + Lowering> Semigroup<Add> for Number<N, S> where
    <S as Lowering>::Layout: LawsOk
{
}

pub struct Hot;
impl Policy for Hot {
    type Quantisation = Wrapping;
}
impl Lowering for Hot {
    type Layout = Dense;
}

fn fold<T: Semigroup<Add>>() {}
fn main() {
    fold::<Number<I16, Hot>>();
    println!("D: a law conditioned on Layout COMPILES");
}
