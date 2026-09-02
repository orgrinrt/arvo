// p8b. The third prediction of p8: a widening in the wrong direction is refused
// at compile time rather than at runtime, which is what I15 requires of a
// validation.
//
// Expected: FAILS to build, at the const assertion inside `widen`, naming the
// offending instantiation.

use core::marker::PhantomData;

pub trait Bound {
    const HI: u32;
}

pub struct Lit<const H: u32>;
impl<const H: u32> Bound for Lit<H> {
    const HI: u32 = H;
}

#[repr(transparent)]
pub struct Fx<B: Bound>(u8, PhantomData<B>);

impl<B: Bound> Fx<B> {
    #[inline(always)]
    pub const fn assume(v: u8) -> Self {
        Fx(v, PhantomData)
    }
    #[inline(always)]
    pub const fn get(&self) -> u8 {
        self.0
    }
}

#[inline(always)]
pub fn widen<A: Bound, B: Bound>(x: Fx<A>) -> Fx<B> {
    const { assert!(A::HI <= B::HI, "widening must not tighten the bound") }
    Fx::assume(x.get())
}

#[inline(never)]
#[no_mangle]
pub fn tighten_200_to_100(a: u8) -> u8 {
    // 200 does not fit in 100. There is no value of `a` for which this is the
    // wrong thing to reject, and nothing about it is visible at runtime.
    widen::<Lit<200>, Lit<100>>(Fx::assume(a)).get()
}

fn main() {
    println!("{}", tighten_200_to_100(150));
}
