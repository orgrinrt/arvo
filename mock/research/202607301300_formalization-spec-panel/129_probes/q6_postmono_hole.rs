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

// A generic wrapper that is WRONG for every instantiation: it claims the product
// of two P-wide numerals is P wide. Never instantiated in this crate.
pub fn square_wrong<const P: u32>(x: Number<P, Warm>) -> Number<P, Warm> {
    mul::<P, P, P>(x, x)
}
