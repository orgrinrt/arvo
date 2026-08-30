// PROBE p3e. p3b showed the const-assert permission is a MONOMORPHISATION-time
// refusal: it fires only where the generic function is actually instantiated in
// codegen. `pub fn total_delta` holding a straddling declaration compiled
// clean in a staticlib, because nothing reached it and the symbol was never
// emitted (p3_compile_output.txt, and `nm` on the archive shows only
// `_agreement_check`).
//
// A TYPE-CHECK-time refusal would be stronger. The obvious spelling is to lift
// the const predicate into a bound. This probe attempts exactly that and
// records the diagnostic, because a refused bound is a result rather than an
// obstacle.
//
// Expected: the anonymous const expression `{ closed_verdict(LO, HI) }` in
// type position uses the generic parameters, which is what
// `generic_const_exprs` exists for, and that feature is FORBIDDEN
// (unstable-features.md, the forbidden table). So this must not compile, and
// the point of the probe is what rustc says about it.

#![no_std]
#![allow(dead_code)]

const fn closed_verdict(lo: i32, hi: i32) -> bool {
    lo >= 0 || hi <= 0
}

pub struct Cond<const B: bool>;
pub trait IsTrue {}
impl IsTrue for Cond<true> {}

pub trait Window {
    const LO: i32;
    const HI: i32;
}
pub struct Win<const LO: i32, const HI: i32>;
impl<const LO: i32, const HI: i32> Window for Win<LO, HI> {
    const LO: i32 = LO;
    const HI: i32 = HI;
}

// ATTEMPT 1: the predicate as an anonymous const in a bound.
pub fn fold_reassociated_a1<const LO: i32, const HI: i32>(xs: &[i8]) -> i8
where
    Cond<{ closed_verdict(LO, HI) }>: IsTrue,
{
    let _ = xs;
    0
}

// ATTEMPT 2: the same thing routed through the trait's associated consts
// rather than through the function's own const parameters, in case the
// indirection changes what the solver will accept.
pub fn fold_reassociated_a2<W: Window>(xs: &[i8]) -> i8
where
    Cond<{ closed_verdict(<W as Window>::LO, <W as Window>::HI) }>: IsTrue,
{
    let _ = xs;
    0
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
