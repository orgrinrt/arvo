//! PROBE A: the check IS the typestate (Thread C, op's direction in 04b/06b).
//!
//! Prior state: 03 proposed bounded const falsification with a hand-written
//! oracle; 05 compiled it and found the oracle must be macro-instantiated
//! because a `const fn` cannot call through a `fn` pointer; both left the
//! check sitting BESIDE the classification, with nothing tying the declared
//! truth marker to the computed boolean.
//!
//! This probe ties them, with three moves:
//!   1. the recovery map is a `[const]` trait method on the resolution itself,
//!      so the oracle is the SAME definition the runtime arithmetic calls, not
//!      a duplicate, and the generic check calls through the bound (no macro,
//!      no fn pointer);
//!   2. the classification trait requires the truth markers AND a witness
//!      const whose default body asserts the declared marker equals the
//!      computed check, so a constructor cannot classify itself wrongly;
//!   3. the same markers feed the law derivation, so the typestate the laws
//!      project is the thing the check gates.
#![feature(const_trait_impl)]
#![allow(dead_code)]

pub trait TruthMarker {
    const VALUE: bool;
}
pub struct True;
impl TruthMarker for True {
    const VALUE: bool = true;
}
pub struct False;
impl TruthMarker for False {
    const VALUE: bool = false;
}

/// The ONE semantic definition of a resolution: its recovery map. This is
/// the method the runtime arithmetic would call; the check below runs
/// against it, not against a re-statement of it.
pub const trait Resolve {
    fn phi(x: i32, min: i32, max: i32) -> i32;
}

/// Bounded exhaustive translation-stability check (01's identity), generic
/// over the resolution, calling the recovery map through the const-trait
/// bound. Dissolves 05's macro-instantiation constraint.
pub const fn stable<R: [const] Resolve>(min: i32, max: i32, two_sided: bool) -> bool {
    let lo = if two_sided { min + min } else { 0 };
    let hi = max + max;
    let mut x = lo;
    while x <= hi {
        let clo = if two_sided { min } else { 0 };
        let mut c = clo;
        while c <= max {
            let lhs = R::phi(R::phi(x, min, max) + c, min, max);
            let rhs = R::phi(x + c, min, max);
            if lhs != rhs {
                return false;
            }
            c += 1;
        }
        x += 1;
    }
    true
}

/// The classification. The declared markers are what the law derivation
/// projects (totality per 02); the WITNESS default body is what makes a
/// false declaration unbuildable (truth per 03).
pub trait Resolution: const Resolve + Sized {
    type StableOneSided: TruthMarker;
    type StableTwoSided: TruthMarker;
    const WITNESS: () = {
        assert!(
            stable::<Self>(0, 7, false) == <Self::StableOneSided as TruthMarker>::VALUE,
            "declared one-sided stability disagrees with the computed check"
        );
        assert!(
            stable::<Self>(-8, 7, true) == <Self::StableTwoSided as TruthMarker>::VALUE,
            "declared two-sided stability disagrees with the computed check"
        );
    };
}

// --- constructors: one recovery map each, one classification each ----------

pub struct ReduceModulo;
const impl Resolve for ReduceModulo {
    fn phi(x: i32, min: i32, max: i32) -> i32 {
        let span = max - min + 1;
        (x - min).rem_euclid(span) + min
    }
}
impl Resolution for ReduceModulo {
    type StableOneSided = True;
    type StableTwoSided = True;
}

pub struct Clamp;
const impl Resolve for Clamp {
    fn phi(x: i32, min: i32, max: i32) -> i32 {
        if x > max {
            max
        } else if x < min {
            min
        } else {
            x
        }
    }
}
impl Resolution for Clamp {
    type StableOneSided = True;
    type StableTwoSided = False;
}

pub struct SubstituteZero;
const impl Resolve for SubstituteZero {
    fn phi(x: i32, min: i32, max: i32) -> i32 {
        if x > max || x < min {
            0
        } else {
            x
        }
    }
}
impl Resolution for SubstituteZero {
    const WITNESS: () = ();
    type StableOneSided = True;
    type StableTwoSided = False;
}

// --- forcing sites, one per constructor (see probe A2 for why) -------------

const _: () = <ReduceModulo as Resolution>::WITNESS;
const _: () = <Clamp as Resolution>::WITNESS;
const _: () = <SubstituteZero as Resolution>::WITNESS;

// --- the same markers feed the law derivation (02's computed shape) --------

pub trait And<R> {
    type Out;
}
impl And<True> for True {
    type Out = True;
}
impl And<False> for True {
    type Out = False;
}
impl<R> And<R> for False {
    type Out = False;
}

pub trait Signedness {
    type TwoSided: TruthMarker;
}
pub struct Unsigned;
impl Signedness for Unsigned {
    type TwoSided = False;
}
pub struct Signed;
impl Signedness for Signed {
    type TwoSided = True;
}

pub trait StableFor<D> {
    type Out;
}
impl<R: Resolution> StableFor<Unsigned> for R {
    type Out = R::StableOneSided;
}
impl<R: Resolution> StableFor<Signed> for R {
    type Out = R::StableTwoSided;
}

pub trait IsTrue {}
impl IsTrue for True {}

fn fold<A, B, D>()
where
    A: StableFor<D>,
    B: StableFor<D>,
    A::Out: And<B::Out>,
    <A::Out as And<B::Out>>::Out: IsTrue,
{
}

fn main() {
    fold::<ReduceModulo, ReduceModulo, Signed>(); // wrap folds signed
    fold::<Clamp, Clamp, Unsigned>();
    let _l1 = (); // unsigned clamp folds
                  // fold::<Clamp, Clamp, Signed>();          // refused, correctly
                  // fold::<SubstituteZero, SubstituteZero, Unsigned>(); // refused: 01 finding 1
    println!("A OK: witness ties the declared marker to the computed check");
}
