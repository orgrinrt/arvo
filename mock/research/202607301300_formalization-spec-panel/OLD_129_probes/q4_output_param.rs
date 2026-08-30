#![no_std]
#![allow(dead_code)]
// NO feature gates at all.
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

/// The output precision is a parameter, not an expression. The relation it must
/// satisfy is stated as a const assertion in the body, which is legal because it
/// is a value computation rather than a const argument.
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

pub fn add<const P: u32, const Q: u32, const R: u32>(
    a: Number<P, Warm>,
    b: Number<Q, Warm>,
) -> Number<R, Warm> {
    const {
        assert!(
            R == if P > Q { P } else { Q } + 1,
            "add: output precision must be one above the wider input"
        )
    }
    Number(a.0 + b.0, core::marker::PhantomData)
}

// R inferred backwards from the annotation.
pub fn use_inferred() {
    let a: Number<16, Warm> = Number::new(1);
    let b: Number<16, Warm> = Number::new(2);
    let c: Number<32, Warm> = mul(a, b);
    let _d: Number<17, Warm> = add(a, b);
    let _ = c;
}

// R inferred from a downstream call's parameter type, with no annotation at all.
pub fn wants32(_: Number<32, Warm>) {}
pub fn use_inferred_through_call() {
    let a: Number<16, Warm> = Number::new(1);
    let b: Number<16, Warm> = Number::new(2);
    wants32(mul(a, b));
}

// and the canonical-form property still holds: two routes to 32 agree.
pub fn agree(x: Number<{ 16 + 16 }, Warm>, y: Number<{ 24 + 8 }, Warm>) {
    wants32(x);
    wants32(y);
}
