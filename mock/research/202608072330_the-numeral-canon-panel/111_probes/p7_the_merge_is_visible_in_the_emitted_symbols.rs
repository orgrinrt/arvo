// p7. Under a discharged refinement the two completions compile to one body,
// and without it they do not.
//
// p6 measures in a model that a declared operand bound makes saturating and
// wrapping addition the same function, and that the propagated bound predicts
// the boundary exactly in both directions. This probe asks the operational
// question that model cannot: does the merge reach the compiler, or is it a
// fact about a Python sweep.
//
// `109` P5 established the neighbouring result, that a carried range removes the
// completion from the emitted code, with the clamp present on the unproved path
// and absent on the proved one. What is added here is the MERGE rather than the
// elimination: two functions differing only in which completion they name should
// become one body where the bound is discharged, and stay two where it is not.
// That is the compiled form of "these two primitives are the same here".
//
// No feature gate. The bound propagates through associated consts, which is the
// trait-shaped answer to arithmetic in type position rather than
// `generic_const_exprs`.

use core::marker::PhantomData;

// ---------------------------------------------------------------- the bound

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

// ------------------------------------------------------------ the completion

pub trait Completion {
    /// Applied only where the bound does not discharge. The gate below is what
    /// decides whether this is reachable at all.
    fn repair(x: u32) -> u8;
}

pub struct Sat;
impl Completion for Sat {
    #[inline(always)]
    fn repair(x: u32) -> u8 {
        if x > 255 {
            255
        } else {
            x as u8
        }
    }
}

pub struct Wrap;
impl Completion for Wrap {
    #[inline(always)]
    fn repair(x: u32) -> u8 {
        (x & 0xff) as u8
    }
}

// -------------------------------------------------------------- the primitive

#[repr(transparent)]
pub struct Fx<B: Bound, C: Completion>(u8, PhantomData<(B, C)>);

impl<B: Bound, C: Completion> Fx<B, C> {
    /// The construction site is the perimeter. A value only carries the bound
    /// because it was established here.
    #[inline(always)]
    pub const fn assume(v: u8) -> Self {
        Fx(v, PhantomData)
    }
    #[inline(always)]
    pub const fn get(&self) -> u8 {
        self.0
    }
}

/// Whether the completion can fire at all for a result carrying bound `B`.
pub const fn discharged<B: Bound>() -> bool {
    B::HI <= 255
}

/// The one addition. The gate is const and selects between the bare instruction
/// and the completion; nothing branches at runtime.
#[inline(always)]
pub fn add<A: Bound, B: Bound, C: Completion>(a: Fx<A, C>, b: Fx<B, C>) -> Fx<BSum<A, B>, C> {
    let exact = a.get() as u32 + b.get() as u32;
    let v = if const { <BSum<A, B> as Bound>::HI <= 255 } {
        exact as u8
    } else {
        C::repair(exact)
    };
    Fx::assume(v)
}

// ------------------------------------------------------- the four observables
//
// proved:   operands bounded by 100, so the sum is bounded by 200 and the
//           completion is unreachable. The two completions should collapse.
// unproved: operands bounded by 200, so the sum reaches 400 and the completion
//           decides. The two completions must not collapse.

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

/// The control. The same repair with no gate at all, so neither the merge nor
/// the elimination can be attributed to the optimiser deleting dead arithmetic
/// for unrelated reasons.
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
    // Behaviour, checked rather than assumed, so the symbol comparison below is
    // about two functions that actually compute something.
    let mut agree_proved = 0u32;
    let mut differ_unproved = 0u32;
    for a in 0u8..=100 {
        for b in 0u8..=100 {
            if proved_sat(a, b) == proved_wrap(a, b) {
                agree_proved += 1;
            }
        }
    }
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            if unproved_sat(a, b) != unproved_wrap(a, b) {
                differ_unproved += 1;
            }
        }
    }
    println!("within the proved bound, sat and wrap agree on {agree_proved} of 10201 pairs");
    println!("outside it, they differ on {differ_unproved} of 65536 pairs");
    println!("proved_sat(100,100)   = {}", proved_sat(100, 100));
    println!("unproved_sat(200,200) = {}", unproved_sat(200, 200));
    println!("unproved_wrap(200,200)= {}", unproved_wrap(200, 200));
}
