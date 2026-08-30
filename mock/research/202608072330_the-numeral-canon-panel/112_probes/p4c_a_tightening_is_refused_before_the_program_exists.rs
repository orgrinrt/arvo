// p4c. EXPECTED COMPILE FAILURE.
//
// The direction a refinement does not admit.  `p4`'s `widen` carries a const
// assertion in an associated const, so a tightening is a build failure that
// names the exact instantiation, which is what I15 requires of an invalid:
// it is caught at compile time and never becomes a runtime concern.
//
// Predicted before compiling: E0080, evaluation panicked, naming
// `Widen::<Lit<200>, Lit<100>>`.

use core::marker::PhantomData;

trait Bound {
    const HI: u8;
}
struct Lit<const N: u8>;
impl<const N: u8> Bound for Lit<N> {
    const HI: u8 = N;
}

#[repr(transparent)]
struct Ref<B: Bound>(u8, PhantomData<B>);
impl<B: Bound> Clone for Ref<B> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<B: Bound> Copy for Ref<B> {}

struct Widen<From, To>(PhantomData<(From, To)>);
impl<From: Bound, To: Bound> Widen<From, To> {
    const CHECK: () = assert!(
        From::HI <= To::HI,
        "widening must not tighten the declared bound"
    );
}

fn widen<From: Bound, To: Bound>(x: Ref<From>) -> Ref<To> {
    let () = Widen::<From, To>::CHECK;
    Ref(x.0, PhantomData)
}

fn main() {
    let wide: Ref<Lit<200>> = Ref(150, PhantomData);
    let _narrow: Ref<Lit<100>> = widen(wide);
    println!("{}", _narrow.0);
}
