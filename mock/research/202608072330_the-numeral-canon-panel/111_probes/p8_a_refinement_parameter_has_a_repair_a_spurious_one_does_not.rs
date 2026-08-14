// p8. The objection to my own answer, and whether it lands.
//
// If a primitive carries a refinement, then two values with different declared
// bounds are different types, and a design that was worried about extra names
// has just acquired a lot of them. `110` F8 prices exactly that hazard: two
// spellings of one primitive are an `E0308` with no in-language repair.
//
// The objection assumes the two cases are alike. This probe checks whether they
// are, and the prediction is that they are not, for a reason with a name:
//
//   A SPURIOUS parameter is one the realisation map does not read. Two types
//   differing only in it denote the same thing, so what is wanted is EQUALITY,
//   and Rust gives no way to make two type constructors applied to different
//   arguments equal. `110` F8.
//
//   A REFINEMENT parameter is one that is read, by the arm selection rather
//   than by R. Two types differing only in it denote different sets, one
//   contained in the other, so what is wanted is not equality but WEAKENING,
//   and weakening is an ordinary total function that is the identity on the
//   representation.
//
// So the prediction, recorded before running:
//
//   1. a widening from a tighter bound to a looser one compiles, with no
//      feature gate
//   2. it is free: the emitted symbol is the identity
//   3. a widening in the wrong direction is refused at COMPILE time, not at
//      runtime, which is what I15 requires
//
// If all three hold, the extra names a refinement introduces are not 110's
// hazard, and the objection does not land.

use core::marker::PhantomData;

pub trait Bound {
    const HI: u32;
}

pub struct Lit<const H: u32>;
impl<const H: u32> Bound for Lit<H> {
    const HI: u32 = H;
}

pub struct BSum<A, B>(PhantomData<(A, B)>);
impl<A: Bound, B: Bound> Bound for BSum<A, B> {
    const HI: u32 = A::HI + B::HI;
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

/// Weakening. The obligation is discharged during monomorphisation, so a
/// widening that is not one is not a program.
#[inline(always)]
pub fn widen<A: Bound, B: Bound>(x: Fx<A>) -> Fx<B> {
    const { assert!(A::HI <= B::HI, "widening must not tighten the bound") }
    Fx::assume(x.get())
}

#[inline(never)]
#[no_mangle]
pub fn widen_7_to_15(a: u8) -> u8 {
    widen::<Lit<7>, Lit<15>>(Fx::assume(a)).get()
}

#[inline(never)]
#[no_mangle]
pub fn widen_100_to_200(a: u8) -> u8 {
    widen::<Lit<100>, Lit<200>>(Fx::assume(a)).get()
}

/// Through a derived bound, which is the case that matters: the result of an
/// addition weakened into a declared column type.
#[inline(never)]
#[no_mangle]
pub fn widen_derived(a: u8) -> u8 {
    widen::<BSum<Lit<40>, Lit<60>>, Lit<200>>(Fx::assume(a)).get()
}

/// The control: an identity function on the same representation, so the symbol
/// comparison is against something rather than against nothing.
#[inline(never)]
#[no_mangle]
pub fn plain_identity(a: u8) -> u8 {
    a
}

fn main() {
    let mut ok = 0u32;
    for a in 0u8..=255 {
        if widen_7_to_15(a) == a && widen_100_to_200(a) == a && widen_derived(a) == a {
            ok += 1;
        }
    }
    println!("widening is the identity on all {ok} of 256 representations");
    println!("widen_7_to_15(7)      = {}", widen_7_to_15(7));
    println!("widen_derived(100)    = {}", widen_derived(100));
    println!("BSum<Lit<40>,Lit<60>>::HI = {}", <BSum<Lit<40>, Lit<60>> as Bound>::HI);
}
