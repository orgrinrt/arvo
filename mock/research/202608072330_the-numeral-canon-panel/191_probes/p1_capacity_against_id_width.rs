// Can "this capacity fits in this id's width" be a compile-time refusal?
//
// WHY THIS PROBE EXISTS. hilavitkutin's plan chain rejects an over-wide
// declaration at run time:
//
//   "A registration whose `PlanDims` declares more phases or trunks than the
//    fixed-width `PhaseId` / `TrunkId` can name is rejected up front as
//    `PlanError::PhaseCapacityExceedsIdWidth` ... so an over-wide dims fails
//    loudly instead of silently wrapping ids past the addressable range."
//        -- hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl, step 2
//
// Both quantities are compile-time constants. Op's I15 says "Never any runtime
// checks, ever. We catch invalids on compile time." So either the check can be
// a compile-time refusal and the consumer is carrying a runtime check it does
// not need, or it cannot and the canon has to say so. This probe asks which,
// and at which spellings, because `35`'s section-6 droplist already records
// that one spelling is refused terminally and the others were never separated.
//
// Modelled in miniature, the way `35_probes/p1` models the width algebra: a
// bounded index `Id<W>` naming `2^W` values and a capacity `Cap<N>`. Nothing
// here is a proposal for a shape; it is a question about what the compiler
// admits.
//
// ARMS. Every arm states what it must do, and the run does not count unless
// each does it. Three must compile and three must be refused; a probe where
// everything compiles has measured nothing.
//
//   A  monomorphic, capacity fits          MUST COMPILE
//   B  monomorphic, capacity overflows     MUST BE REFUSED   <- the whole point
//   C  generic over W and N, fits          MUST COMPILE      (if it can at all)
//   D  generic, the bound in a `where`     ? this is 35's closed spelling
//   E  associated-const gate, fits         MUST COMPILE
//   F  associated-const gate, overflows    MUST BE REFUSED
//
// CONTROL. Arm B is the negative control and it is the reason the probe is
// worth running: if B compiles, every other green here means nothing, because
// the mechanism admits the invalid case it exists to reject.
#![allow(dead_code)]

pub struct Id<const W: u32>;
pub struct Cap<const N: usize>;

impl<const W: u32> Id<W> {
    // The number of distinct values a W-bit id can name. Saturating at the
    // pointer width so the const evaluator does not overflow before it can
    // compare, which would refuse for the wrong reason.
    pub const NAMES: usize = if W as usize >= usize::BITS as usize {
        usize::MAX
    } else {
        1usize << W
    };
}

// --- A / B: monomorphic, the plain const assertion --------------------------

const fn fits(names: usize, capacity: usize) -> bool {
    capacity <= names
}

pub struct PlanA;
impl PlanA {
    const _CHECK: () = assert!(fits(Id::<8>::NAMES, 200), "capacity exceeds id width");
    pub fn touch() {
        let _ = Self::_CHECK;
    }
}

#[cfg(arm_b)]
pub struct PlanB;
#[cfg(arm_b)]
impl PlanB {
    // 300 values do not fit in an 8-bit id. This must not build.
    const _CHECK: () = assert!(fits(Id::<8>::NAMES, 300), "capacity exceeds id width");
    pub fn touch() {
        let _ = Self::_CHECK;
    }
}

// --- C: generic, the assertion inside an inherent const ----------------------

pub struct PlanC<const W: u32, const N: usize>;
impl<const W: u32, const N: usize> PlanC<W, N> {
    const _CHECK: () = assert!(fits(Id::<W>::NAMES, N), "capacity exceeds id width");
    pub fn touch() {
        let _ = Self::_CHECK;
    }
}

pub fn use_c_ok() {
    PlanC::<8, 200>::touch();
}

#[cfg(arm_c_bad)]
pub fn use_c_bad() {
    PlanC::<8, 300>::touch();
}

// --- D: the bound written in a `where` clause, which 35 records as closed ----

#[cfg(arm_d)]
pub fn plan_d<const W: u32, const N: usize>()
where
    [(); (N <= Id::<W>::NAMES) as usize - 1]:,
{
}

// --- E / F: an associated-const gate on a trait -----------------------------

pub trait IdWidth {
    const NAMES: usize;
}
impl<const W: u32> IdWidth for Id<W> {
    const NAMES: usize = Id::<W>::NAMES;
}

pub struct Gated<I, const N: usize>(core::marker::PhantomData<I>);
impl<I: IdWidth, const N: usize> Gated<I, N> {
    const _CHECK: () = assert!(N <= I::NAMES, "capacity exceeds id width");
    pub fn touch() {
        let _ = Self::_CHECK;
    }
}

pub fn use_e_ok() {
    Gated::<Id<8>, 200>::touch();
}

#[cfg(arm_f)]
pub fn use_f_bad() {
    Gated::<Id<8>, 300>::touch();
}

fn main() {
    PlanA::touch();
    use_c_ok();
    use_e_ok();
    #[cfg(arm_b)]
    PlanB::touch();
    #[cfg(arm_c_bad)]
    use_c_bad();
    #[cfg(arm_f)]
    use_f_bad();
    println!("base arms compiled");
}

// --- G: the laziness case, and it is the one that decides whether any of the
// above is a guarantee. An inherent associated const is only evaluated where it
// is used, so if a consumer can name `PlanC::<8, 300>` and merely construct or
// store it without ever touching `_CHECK`, the refusal is not a refusal, it is
// a landmine. This arm must be REFUSED for the mechanism to mean anything, and
// if it compiles then arms C-bad and F are measuring the call site rather than
// the type.
#[cfg(arm_g_construct)]
pub fn use_g_construct() -> PlanC<8, 300> {
    PlanC
}

#[cfg(arm_g_typedef)]
pub type GBad = PlanC<8, 300>;

#[cfg(arm_g_field)]
pub struct HoldsIt {
    inner: PlanC<8, 300>,
}
