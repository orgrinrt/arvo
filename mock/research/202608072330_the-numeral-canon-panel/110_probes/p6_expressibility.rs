//! P6. Is "a primitive is an algebra over a declared signature" expressible,
//! and can a definitionally degenerate axis be canonicalised away?
//!
//! P5 established the criterion: an axis belongs to a primitive's identity
//! exactly when the value set or the realisation map reads it, and the only
//! definitional degeneracy found in the swept space is the radix at F = 0,
//! where the grid step is radix^0 = 1 and nothing mentions the radix again.
//!
//! That criterion is worth nothing if it cannot be spelled. Two questions
//! here, both compiled rather than argued:
//!
//!   Q1. Can a primitive be declared as a carrier plus an interpretation of a
//!       declared signature, with the axes in the type, under the forbidden
//!       feature list? No `generic_const_exprs`, no `specialization`, no
//!       `TypeId`, no `dyn`, no `alloc`, no arithmetic in type position.
//!
//!   Q2. Given two spellings that denote the same primitive, can the type
//!       system be made to treat them as one type? This is the hashcons
//!       question. If the answer is no, then a naming discipline cannot repair
//!       a non-canonical parameterisation after the fact, and the
//!       canonicalisation has to be built into how the type is spelled.
//!
//! This is a spike. Its names, arities and field orders are scaffolding chosen
//! to reach the check; none of them is a design decision. It uses bare `u32`
//! and `i128` throughout because it is testing what the trait solver accepts,
//! not what an API surface should look like.
//!
//! Build: rustc --edition 2021 --crate-type lib p6_expressibility.rs

#![no_std]

// ---------------------------------------------------------------------------
// Q1. a primitive as an algebra: a carrier, and an interpretation of a
// declared signature. The signature is the set of required items, so widening
// the signature is an edit to this trait, which is the type-system reflection
// of P3's finding that the primitive count moves with the signature.
// ---------------------------------------------------------------------------

/// The grid step, as a type-level rational with denominator only.
///
/// This is the parameterisation the P5 criterion argues for: the realisation
/// map reads the STEP, and it reads the radix and the fraction width only
/// through it. Taking the step directly means a degenerate combination cannot
/// be written down twice.
pub trait Step {
    /// Denominator of the step. The step is `1 / DEN`.
    const DEN: i128;
}

/// `1 / D`. Binary with eight fraction bits is `OneOver<256>`; decimal with
/// three is `OneOver<1000>`; no fraction at all is `OneOver<1>`.
pub struct OneOver<const D: i128>;
impl<const D: i128> Step for OneOver<D> {
    const DEN: i128 = D;
}

/// The degenerate step has exactly one spelling, and it is an alias rather
/// than a new type, so it unifies with `OneOver<1>` by construction.
pub type Unit = OneOver<1>;

pub trait Rounding {
    fn round(num: i128, den: i128) -> i128;
}
pub struct Near;
pub struct TowardZero;
impl Rounding for Near {
    fn round(num: i128, den: i128) -> i128 {
        let two = 2 * num;
        if two >= 0 {
            (two + den) / (2 * den)
        } else {
            -((-two + den) / (2 * den))
        }
    }
}
impl Rounding for TowardZero {
    fn round(num: i128, den: i128) -> i128 {
        num / den
    }
}

pub trait Policy {
    fn realise(k: i128, lo: i128, hi: i128) -> i128;
}
pub struct Sat;
pub struct Wrap;
impl Policy for Sat {
    fn realise(k: i128, lo: i128, hi: i128) -> i128 {
        if k < lo {
            lo
        } else if k > hi {
            hi
        } else {
            k
        }
    }
}
impl Policy for Wrap {
    fn realise(k: i128, lo: i128, hi: i128) -> i128 {
        let n = hi - lo + 1;
        let mut r = (k - lo) % n;
        if r < 0 {
            r += n;
        }
        r + lo
    }
}

/// A primitive: the carrier, and the interpretation of the signature.
///
/// Every item below is part of the signature. `add` and `mul` are the whole
/// signature this probe declares, deliberately, so that the next section can
/// add one and show what that does.
pub trait Primitive {
    /// Machine-side carrier.
    type Repr: Copy;
    /// The grid step the realisation map reads.
    type S: Step;
    type R: Rounding;
    type P: Policy;

    const LO: i128;
    const HI: i128;

    fn add(a: Self::Repr, b: Self::Repr) -> Self::Repr;
    fn mul(a: Self::Repr, b: Self::Repr) -> Self::Repr;
}

/// A concrete family. The axes it carries are exactly the ones P5's criterion
/// says the value set and R read: the range, the step, the rounding and the
/// overflow policy. There is no radix parameter and no fraction-width
/// parameter, because neither is read except through the step.
pub struct Fx<const LO: i128, const HI: i128, S, R, P>(core::marker::PhantomData<(S, R, P)>);

impl<const LO: i128, const HI: i128, S: Step, R: Rounding, P: Policy> Primitive
    for Fx<LO, HI, S, R, P>
{
    type Repr = i128;
    type S = S;
    type R = R;
    type P = P;

    const LO: i128 = LO;
    const HI: i128 = HI;

    fn add(a: i128, b: i128) -> i128 {
        P::realise(a + b, LO, HI)
    }

    fn mul(a: i128, b: i128) -> i128 {
        // exact product is (a * b) / DEN in grid units, then rounded, then
        // realised. One realisation map, both regions, as P2 argues.
        let k = R::round(a * b, S::DEN);
        P::realise(k, LO, HI)
    }
}

/// A value of a primitive. Nominal in the primitive, which is what makes the
/// substitution question below have an answer at all.
#[repr(transparent)]
pub struct Num<T: Primitive>(pub T::Repr);

impl<T: Primitive> Clone for Num<T> {
    fn clone(&self) -> Self {
        Num(self.0)
    }
}
impl<T: Primitive> Copy for Num<T> where T::Repr: Copy {}

pub fn add<T: Primitive>(a: Num<T>, b: Num<T>) -> Num<T> {
    Num(T::add(a.0, b.0))
}
pub fn mul<T: Primitive>(a: Num<T>, b: Num<T>) -> Num<T> {
    Num(T::mul(a.0, b.0))
}

// ---------------------------------------------------------------------------
// Q1 answer, part two: widening the signature is an edit to the trait, and a
// primitive that does not interpret the new symbol is no longer a primitive
// over that signature. This is P3's signature-relativity, in the type system.
// ---------------------------------------------------------------------------

/// The wider signature P5 used to separate the reachability degeneracies:
/// adding an operation that leaves the grid.
pub trait PrimitiveWithHalf: Primitive {
    fn half(a: Self::Repr) -> Self::Repr;
}

impl<const LO: i128, const HI: i128, S: Step, R: Rounding, P: Policy> PrimitiveWithHalf
    for Fx<LO, HI, S, R, P>
{
    fn half(a: i128) -> i128 {
        // a/2 in grid units: this is the operation that made the rounding mode
        // observable at a step of 1, which no grid-closed operation could do.
        P::realise(R::round(a, 2), LO, HI)
    }
}

// ---------------------------------------------------------------------------
// Q2. the degenerate axis, canonicalised by construction.
//
// Under the P5 criterion the radix is not read at all when the step is 1. In
// this parameterisation that is not a rule to enforce, it is a spelling that
// does not exist: there is no radix parameter to disagree about, and every
// "no fraction" primitive is `OneOver<1>` whatever radix its author had in
// mind. The two aliases below are the same type, and the assertion compiles.
// ---------------------------------------------------------------------------

/// "Eight-bit unsigned, binary, no fraction."
pub type U8BinaryNoFraction = Fx<0, 255, Unit, Near, Sat>;
/// "Eight-bit unsigned, decimal, no fraction." A different intent, and the
/// same primitive, because at a step of 1 nothing reads the radix.
pub type U8DecimalNoFraction = Fx<0, 255, OneOver<1>, Near, Sat>;

/// The two spellings unify. If they did not, this function would not typecheck
/// at the call in `canonical_by_construction`.
pub fn same_primitive_accepts_both(
    a: Num<U8BinaryNoFraction>,
    b: Num<U8DecimalNoFraction>,
) -> Num<U8BinaryNoFraction> {
    add(a, b)
}

pub fn canonical_by_construction() -> i128 {
    let a: Num<U8BinaryNoFraction> = Num(200);
    let b: Num<U8DecimalNoFraction> = Num(100);
    same_primitive_accepts_both(a, b).0
}

// ---------------------------------------------------------------------------
// And the contrast: where the axes ARE read, the types do not unify, and that
// is the type system holding the identity criterion rather than fighting it.
// The companion file p6_noncanonical_wall.rs is the compile-fail half.
// ---------------------------------------------------------------------------

/// Binary with eight fraction bits.
pub type Q8Binary = Fx<0, 65535, OneOver<256>, Near, Sat>;
/// Decimal with roughly the same precision. A different value set, so a
/// different primitive, and the criterion says so before any test runs.
pub type Q3Decimal = Fx<0, 65535, OneOver<1000>, Near, Sat>;

pub fn distinct_primitives_are_distinct_types() -> (i128, i128) {
    let a: Num<Q8Binary> = Num(1 << 8);
    let b: Num<Q3Decimal> = Num(1000);
    // both denote 1.0 in their own primitive, and they are not the same type
    (mul(a, a).0, mul(b, b).0)
}

/// The saturating and wrapping primitives differ at the same width and step,
/// which P1 separated by `add`. The interpretation differs, so the type
/// differs, so nothing can substitute one for the other silently.
pub type U8Sat = Fx<0, 255, Unit, Near, Sat>;
pub type U8Wrap = Fx<0, 255, Unit, Near, Wrap>;

pub fn overflow_policy_is_identity_bearing() -> (i128, i128) {
    let a: Num<U8Sat> = Num(200);
    let b: Num<U8Sat> = Num(100);
    let c: Num<U8Wrap> = Num(200);
    let d: Num<U8Wrap> = Num(100);
    (add(a, b).0, add(c, d).0) // (255, 44)
}

/// The reachability degeneracy, in the type system.
///
/// At a step of 1 these two differ only in a rounding mode that no grid-closed
/// operation reaches, so over the signature `{add, mul}` they compute
/// identical tables. They are still different types, and that is correct:
/// `half` separates them, and a design that had merged them on observed
/// equality would have to un-merge them the day `half` was added.
pub type U8NearStep1 = Fx<0, 255, Unit, Near, Sat>;
pub type U8TruncStep1 = Fx<0, 255, Unit, TowardZero, Sat>;

pub fn reachability_degeneracy_is_not_merged() -> (i128, i128) {
    // identical over {add, mul}
    let x = <U8NearStep1 as Primitive>::mul(7, 9);
    let y = <U8TruncStep1 as Primitive>::mul(7, 9);
    // and separated the moment the signature widens
    let hx = <U8NearStep1 as PrimitiveWithHalf>::half(7);
    let hy = <U8TruncStep1 as PrimitiveWithHalf>::half(7);
    debug_assert_eq!(x, y);
    (hx, hy) // (4, 3): the rounding mode became observable
}
