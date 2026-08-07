//! 129's numeral, reproduced: the precision IS the parameter, and the signed
//! surface macro folds the sign bit into it (129:521).  Then the mul law
//! `R == P + Q` (129:127) is applied to two signed numerals.
#![no_std]
use core::marker::PhantomData;
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u16 {
    const BITS: u32 = 16;
}
impl Container for u32 {
    const BITS: u32 = 32;
}
impl Container for u64 {
    const BITS: u32 = 64;
}
impl Container for u128 {
    const BITS: u32 = 128;
}
#[derive(Clone, Copy)]
pub struct Fx<const P: u32, C: Container, Sg, S>(C, PhantomData<(Sg, S)>);
pub struct Signed;
pub struct Warm;
macro_rules! IFixed { ($i:literal, $f:literal, $c:ty, $s:ty) => { Fx<{ 1 + $i + $f }, $c, Signed, $s> }; }

pub fn mul<const P: u32, const Q: u32, const R: u32, C: Container, Sg, S>(
    _a: Fx<P, C, Sg, S>,
    _b: Fx<Q, C, Sg, S>,
) -> Fx<R, C, Sg, S> {
    const {
        assert!(
            R == P + Q,
            "mul: output precision must equal the sum of the input precisions"
        )
    }
    unimplemented!()
}

// Q12.3 signed: 1 sign + 12 integer + 3 fraction = 16 stored bits.
// The product is Q24.6 signed: 1 sign + 24 integer + 6 fraction = 31 stored bits.
pub fn correct(a: IFixed!(12, 3, u32, Warm), b: IFixed!(12, 3, u32, Warm)) {
    let _p: Fx<31, u32, Signed, Warm> = mul(a, b); // the true width
}
