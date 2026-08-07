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
impl<const P: u32, S> Number<P, S> {
    pub const fn new(raw: u128) -> Self {
        Number(raw, core::marker::PhantomData)
    }
}
pub fn mul<const P: u32, const Q: u32, const R: u32>(
    a: Number<P, Warm>,
    b: Number<Q, Warm>,
) -> Number<R, Warm> {
    const {
        assert!(
            R == P + Q,
            "mul: output precision must equal the sum of the input precisions"
        )
    }
    Number(a.0 * b.0, core::marker::PhantomData)
}
pub fn wrong() {
    let a: Number<16, Warm> = Number::new(1);
    let b: Number<16, Warm> = Number::new(2);
    let _c: Number<31, Warm> = mul(a, b);
}
