// p7b. p7 with the control repaired.
//
// p7's wrap arm was not an independent body: wrapping at the container width is
// the bare add, so three of its four symbols aliased for a reason unrelated to
// the bound and only one cell carried the claim. The repair is to declare a
// logical range strictly inside the container, so saturation clamps at 200 and
// wrapping is modulo 201, and neither is what the machine does for free.
//
// The prediction, recorded before running:
//
//   proved_sat   == proved_wrap     one body, the bare add, because the
//                                   propagated bound 100 + 100 discharges
//   unproved_sat != unproved_wrap   two bodies, because 200 + 200 does not
//   unproved_sat == ungated_sat     the gated and ungated repairs agree
//   unproved_wrap == ungated_wrap   likewise
//
// If the first and second both hold, the merge is a fact about the emitted code
// and not only about a model sweep, and the discriminator is the bound rather
// than an accident of which completion happens to be free.
//
// No feature gate.

use core::marker::PhantomData;

/// The declared logical range, strictly inside the container.
const LOGICAL_HI: u32 = 200;

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

pub trait Completion {
    fn repair(x: u32) -> u8;
}

pub struct Sat;
impl Completion for Sat {
    #[inline(always)]
    fn repair(x: u32) -> u8 {
        if x > LOGICAL_HI {
            LOGICAL_HI as u8
        } else {
            x as u8
        }
    }
}

pub struct Wrap;
impl Completion for Wrap {
    #[inline(always)]
    fn repair(x: u32) -> u8 {
        (x % (LOGICAL_HI + 1)) as u8
    }
}

#[repr(transparent)]
pub struct Fx<B: Bound, C: Completion>(u8, PhantomData<(B, C)>);

impl<B: Bound, C: Completion> Fx<B, C> {
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
pub fn add<A: Bound, B: Bound, C: Completion>(a: Fx<A, C>, b: Fx<B, C>) -> Fx<BSum<A, B>, C> {
    let exact = a.get() as u32 + b.get() as u32;
    let v = if const { <BSum<A, B> as Bound>::HI <= LOGICAL_HI } {
        exact as u8
    } else {
        C::repair(exact)
    };
    Fx::assume(v)
}

#[inline(never)]
#[no_mangle]
pub fn proved_sat(a: u8, b: u8) -> u8 {
    add::<Lit<100>, Lit<100>, Sat>(Fx::assume(a), Fx::assume(b)).get()
}

#[inline(never)]
#[no_mangle]
pub fn proved_wrap(a: u8, b: u8) -> u8 {
    add::<Lit<100>, Lit<100>, Wrap>(Fx::assume(a), Fx::assume(b)).get()
}

#[inline(never)]
#[no_mangle]
pub fn unproved_sat(a: u8, b: u8) -> u8 {
    add::<Lit<200>, Lit<200>, Sat>(Fx::assume(a), Fx::assume(b)).get()
}

#[inline(never)]
#[no_mangle]
pub fn unproved_wrap(a: u8, b: u8) -> u8 {
    add::<Lit<200>, Lit<200>, Wrap>(Fx::assume(a), Fx::assume(b)).get()
}

#[inline(never)]
#[no_mangle]
pub fn ungated_sat(a: u8, b: u8) -> u8 {
    Sat::repair(a as u32 + b as u32)
}

#[inline(never)]
#[no_mangle]
pub fn ungated_wrap(a: u8, b: u8) -> u8 {
    Wrap::repair(a as u32 + b as u32)
}

fn main() {
    let mut agree = 0u32;
    let mut pairs = 0u32;
    for a in 0u8..=100 {
        for b in 0u8..=100 {
            pairs += 1;
            if proved_sat(a, b) == proved_wrap(a, b) {
                agree += 1;
            }
        }
    }
    let mut differ = 0u32;
    let mut pairs2 = 0u32;
    for a in 0u8..=200 {
        for b in 0u8..=200 {
            pairs2 += 1;
            if unproved_sat(a, b) != unproved_wrap(a, b) {
                differ += 1;
            }
        }
    }
    println!("declared logical range 0..={LOGICAL_HI}, container u8");
    println!("proved   (operands <= 100, propagated 200 <= {LOGICAL_HI}):");
    println!("  sat and wrap agree on {agree} of {pairs} pairs");
    println!("unproved (operands <= 200, propagated 400 >  {LOGICAL_HI}):");
    println!("  sat and wrap differ on {differ} of {pairs2} pairs");
    println!("witnesses:");
    println!("  proved_sat(100,100)    = {}", proved_sat(100, 100));
    println!("  proved_wrap(100,100)   = {}", proved_wrap(100, 100));
    println!("  unproved_sat(150,150)  = {}", unproved_sat(150, 150));
    println!("  unproved_wrap(150,150) = {}", unproved_wrap(150, 150));
}
