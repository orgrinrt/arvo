#![no_std]
#![allow(dead_code)]
pub struct Warm;
pub struct Number<const P: u32, S>(u128, core::marker::PhantomData<S>);
impl<const P: u32, S> Clone for Number<P, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, S> Copy for Number<P, S> {}

pub fn widen_then<const P: u32, const A: u32, const B: u32, const R: u32>(
    x: Number<P, Warm>,
) -> Number<R, Warm> {
    const {
        assert!(
            R == (P + A) + B,
            "widen: output precision must be the input plus both increments"
        )
    }
    Number(x.0, core::marker::PhantomData)
}
pub fn widen_once<const P: u32, const A: u32, const B: u32, const R: u32>(
    x: Number<P, Warm>,
) -> Number<R, Warm> {
    const {
        assert!(
            R == P + (A + B),
            "widen: output precision must be the input plus the combined increment"
        )
    }
    Number(x.0, core::marker::PhantomData)
}
// under a generic parameter, both produce the SAME type, because the type is R.
pub fn interchange<const P: u32, const A: u32, const B: u32, const R: u32>(
    x: Number<P, Warm>,
) -> Number<R, Warm> {
    let y: Number<R, Warm> = widen_then::<P, A, B, R>(x);
    let z: Number<R, Warm> = widen_once::<P, A, B, R>(x);
    let _ = z;
    y
}
