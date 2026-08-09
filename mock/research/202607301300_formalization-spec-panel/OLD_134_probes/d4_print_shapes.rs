// d4: what does rustc PRINT for each candidate magnitude shape, at the same
// mismatch (26 produced, 16 annotated)? No arithmetic here, only printing, so the
// shapes are directly comparable. Six shapes.
#![no_std]
use core::marker::PhantomData;

pub struct End;

// shape 1: little-endian binary, the shipped a133 encoding
pub struct D0<T>(PhantomData<T>);
pub struct D1<T>(PhantomData<T>);
pub type B16 = D0<D0<D0<D0<D1<End>>>>>;
pub type B26 = D0<D1<D0<D1<D1<End>>>>>;

// shape 2: big-endian decimal digits
pub struct N0<T>(PhantomData<T>);
pub struct N1<T>(PhantomData<T>);
pub struct N2<T>(PhantomData<T>);
pub struct N3<T>(PhantomData<T>);
pub struct N4<T>(PhantomData<T>);
pub struct N5<T>(PhantomData<T>);
pub struct N6<T>(PhantomData<T>);
pub struct N7<T>(PhantomData<T>);
pub struct N8<T>(PhantomData<T>);
pub struct N9<T>(PhantomData<T>);
pub type E16 = N1<N6<End>>;
pub type E26 = N2<N6<End>>;

// shape 3: const-carrying head over a binary tower
pub struct W<const V: u32, D>(PhantomData<D>);
pub type H16 = W<16, B16>;
pub type H26 = W<26, B26>;

// shape 4: big-endian decimal, deep (a 200-bit width)
pub type E200 = N2<N0<N0<End>>>;
pub type E205 = N2<N0<N5<End>>>;

pub struct Hot;
pub struct Fixed<I, F, S>(PhantomData<(I, F, S)>);

pub fn mk<I, F, S>() -> Fixed<I, F, S> {
    Fixed(PhantomData)
}

pub fn s1(_: Fixed<B16, B16, Hot>) -> Fixed<B16, B16, Hot> {
    mk::<B26, B16, Hot>()
}
pub fn s2(_: Fixed<E16, E16, Hot>) -> Fixed<E16, E16, Hot> {
    mk::<E26, E16, Hot>()
}
pub fn s3(_: Fixed<H16, H16, Hot>) -> Fixed<H16, H16, Hot> {
    mk::<H26, H16, Hot>()
}
pub fn s4(_: Fixed<E200, E200, Hot>) -> Fixed<E200, E200, Hot> {
    mk::<E205, E200, Hot>()
}
