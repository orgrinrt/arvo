//! P4. What does a CROSS-FAMILY common target look like to the type system?
//!
//! P3 established that within one family both the product formula and the
//! coordinatewise join are ordinary associated-type folds, gate-free, erasing
//! to a constant. So "formula versus extremum" is not a feasibility distinction
//! there.
//!
//! This probe asks the question one layer out: when the two operands are of
//! DIFFERENT families, whose members are not the same coordinates, is a common
//! target expressible at all?
//!
//! The three arms are built deliberately, and the interesting outcomes are the
//! REFUSALS. A contract that has no expressible form is a stronger result than
//! one that computes a wrong answer.
//!
//!   ARM 1  a product formula, within one family          expect: compiles
//!   ARM 2  a product formula, across two families        expect: ?
//!   ARM 3  a JOIN across two families                    expect: ?
//!
//! Toggle with --cfg arm1 / arm2 / arm3. Nothing is gated; the header of every
//! run is recorded in RUN.md with its exit code either way.

#![no_std]

pub struct End;
pub struct Zero<T>(core::marker::PhantomData<T>);
pub struct One<T>(core::marker::PhantomData<T>);

pub trait Nat {
    const VAL: u32;
}
impl Nat for End {
    const VAL: u32 = 0;
}
impl<T: Nat> Nat for Zero<T> {
    const VAL: u32 = 2 * T::VAL;
}
impl<T: Nat> Nat for One<T> {
    const VAL: u32 = 2 * T::VAL + 1;
}

pub trait AddN<R> {
    type Out: Nat;
}
impl AddN<End> for End {
    type Out = End;
}
impl<T: Nat> AddN<Zero<T>> for End {
    type Out = Zero<T>;
}
impl<T: Nat> AddN<One<T>> for End {
    type Out = One<T>;
}
impl<T: Nat> AddN<End> for Zero<T> {
    type Out = Zero<T>;
}
impl<T: Nat> AddN<End> for One<T> {
    type Out = One<T>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<Zero<B>> for Zero<A> {
    type Out = Zero<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<One<B>> for Zero<A> {
    type Out = One<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<Zero<B>> for One<A> {
    type Out = One<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<One<B>> for One<A>
where
    <A as AddN<B>>::Out: Succ,
{
    type Out = Zero<<<A as AddN<B>>::Out as Succ>::Out>;
}
pub trait Succ {
    type Out: Nat;
}
impl Succ for End {
    type Out = One<End>;
}
impl<T: Nat> Succ for Zero<T> {
    type Out = One<T>;
}
impl<T: Nat + Succ> Succ for One<T> {
    type Out = Zero<<T as Succ>::Out>;
}

type N1 = One<End>;
type N2 = Zero<One<End>>;
type N3 = One<One<End>>;
type N5 = One<Zero<One<End>>>;
type N8 = Zero<Zero<Zero<One<End>>>>;

// ---------------------------------------------------------------------------
// TWO FAMILIES, whose members are genuinely different coordinates. This is the
// whole point: a uniform-grid numeral is (integer width, fraction width); a
// float-shaped numeral is (precision, exponent low, exponent high). There is no
// coordinate they share.
// ---------------------------------------------------------------------------

/// Uniform-grid family: value set is an arithmetic progression.
pub struct Uni<I, F>(core::marker::PhantomData<(I, F)>);

/// Float-shaped family: value set is a union of progressions whose step
/// coarsens with magnitude.
pub struct Flt<P, ELo, EHi>(core::marker::PhantomData<(P, ELo, EHi)>);

// --- ARM 1: the product formula inside the uniform family ---
#[cfg(arm1)]
mod arm1 {
    use super::*;
    pub trait MulNum<R> {
        type Out;
    }
    impl<I1: Nat + AddN<I2>, F1: Nat + AddN<F2>, I2: Nat, F2: Nat> MulNum<Uni<I2, F2>> for Uni<I1, F1> {
        type Out = Uni<<I1 as AddN<I2>>::Out, <F1 as AddN<F2>>::Out>;
    }
    pub type R = <Uni<N3, N5> as MulNum<Uni<N2, N1>>>::Out;
    pub trait W {
        const I: u32;
        const F: u32;
    }
    impl<I: Nat, F: Nat> W for Uni<I, F> {
        const I: u32 = I::VAL;
        const F: u32 = F::VAL;
    }
    const _: () = assert!(<R as W>::I == 5 && <R as W>::F == 6, "arm1");
}

// --- ARM 2: the product formula ACROSS the two families ---
// The product of a uniform numeral and a float numeral. What are the result's
// members? There is no answer written down anywhere, so the honest attempt is
// to declare the impl and see what the compiler asks for.
#[cfg(arm2)]
mod arm2 {
    use super::*;
    pub trait MulNum<R> {
        type Out;
    }
    // the within-family case, for coherence company
    impl<I1: Nat + AddN<I2>, F1: Nat + AddN<F2>, I2: Nat, F2: Nat> MulNum<Uni<I2, F2>> for Uni<I1, F1> {
        type Out = Uni<<I1 as AddN<I2>>::Out, <F1 as AddN<F2>>::Out>;
    }
    // the cross-family case. `Out` has to be SOMETHING. There is no shared
    // coordinate, so every candidate below is a DESIGN CHOICE wearing a type.
    // Written as the most defensible one: the product of a uniform and a float
    // is a float whose precision is the sum and whose exponent range grows.
    impl<I1: Nat, F1: Nat, P2: Nat, ELo: Nat, EHi: Nat> MulNum<Flt<P2, ELo, EHi>> for Uni<I1, F1>
    where
        I1: AddN<F1>,
        <I1 as AddN<F1>>::Out: AddN<P2>,
    {
        // the uniform operand's precision is I1+F1; the product's is that plus P2.
        type Out = Flt<<<I1 as AddN<F1>>::Out as AddN<P2>>::Out, ELo, EHi>;
    }
    pub trait P {
        const PREC: u32;
    }
    impl<Pp: Nat, A, B> P for Flt<Pp, A, B> {
        const PREC: u32 = Pp::VAL;
    }
    pub type R = <Uni<N3, N5> as MulNum<Flt<N8, N1, N2>>>::Out;
    const _: () = assert!(<R as P>::PREC == 16, "arm2: 3+5+8");
}

// --- ARM 3: a JOIN across the two families ---
// The least numeral containing both operands' value sets. `03` section 3.2
// reports, from three instruments, that this pair has TWO minimal upper bounds
// and no least one. So the question here is not "what does it compute" but
// "what can even be written down".
#[cfg(arm3)]
mod arm3 {
    use super::*;
    pub trait JoinNum<R> {
        type Out;
    }
    // The two minimal upper bounds of U<0,1> and U<2,0>, per `03` section 3.2:
    //   Uni<2,1>                      (the uniform candidate)
    //   Flt<prec 2, e -1 .. 1>        (the float candidate)
    // Neither contains the other. An associated type must name exactly ONE.
    // Both impls below are honest readings of "the join". Declaring both is the
    // question this arm asks: can the type system hold an antichain?
    impl<I1: Nat, F1: Nat, P2: Nat, ELo: Nat, EHi: Nat> JoinNum<Flt<P2, ELo, EHi>> for Uni<I1, F1> {
        type Out = Uni<I1, F1>; // "prefer the uniform candidate"
    }
    impl<I1: Nat, F1: Nat, P2: Nat, ELo: Nat, EHi: Nat> JoinNum<Flt<P2, ELo, EHi>> for Uni<I1, F1> {
        type Out = Flt<P2, ELo, EHi>; // "prefer the float candidate"
    }
}
