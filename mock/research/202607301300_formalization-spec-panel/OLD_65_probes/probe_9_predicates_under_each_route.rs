//! Probe 9. The four const-fn predicates the facade carries today, expressed
//! under the two candidate parameterisations, so the residual can be compared
//! rather than argued.
//!
//! The predicates, from the shipped tree:
//!   OneRepresentable  arvo-strategy/src/identity.rs:70-91   (I >= 1)
//!   IntegerLike       arvo/src/ufixed.rs:263-266            (F == 0)
//!   FractionLike      arvo/src/ufixed.rs:270-275            (F  > 0)
//!   the width itself  arvo/src/strategy.rs:39-50            (I + F)
//!
//! Route Y: `UFixed<const W: u16, const F: FBits, S>`. Everything above except
//! OneRepresentable falls out; OneRepresentable becomes `W > F`, a comparison
//! of two const parameters, which has no expression without either a
//! quadratic (W, F) impl table or type-level arithmetic.
//!
//! Route Z: `UFixed<I: Nat, F: Nat, S>`. Every predicate is a one-line
//! structural fact and the width is a type-level sum. This is the tower's own
//! shape and the migration's stated target.
#![no_std]
use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct Iv<P>(PhantomData<P>);
pub trait Pos {
    const VAL: u128;
}
impl Pos for H {
    const VAL: u128 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u128 = 2 * P::VAL;
}
impl<P: Pos> Pos for Iv<P> {
    const VAL: u128 = 2 * P::VAL + 1;
}
pub struct Z;
pub struct Pz<P>(PhantomData<P>);
pub trait Nat {
    const VAL: u128;
}
impl Nat for Z {
    const VAL: u128 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u128 = P::VAL;
}

// -- Route Z, all four predicates ------------------------------------------
// 1. one is representable exactly when the integer part is nonzero: ONE impl,
//    and the absence at `Z` is what withholds `Identity<Multiplicative>`,
//    which is the guard the review installed for the `UFixed<0, F>::ONE`
//    defect. Under route Z the guard costs one line and cannot be got wrong.
pub trait OneRepresentable {}
impl<P: Pos> OneRepresentable for Pz<P> {}
// deliberately no `impl OneRepresentable for Z`

// 2. and 3. the fraction predicates, one impl each.
pub trait IsZero {}
impl IsZero for Z {}
pub trait NonZero {}
impl<P: Pos> NonZero for Pz<P> {}

// 4. the width, a type-level sum (the adder is probe 3's, elided here to the
//    two cases this file exercises).
pub trait AddN<R> {
    type Out: Nat;
}
impl AddN<Z> for Z {
    type Out = Z;
}
impl<P: Pos> AddN<Z> for Pz<P> {
    type Out = Pz<P>;
}
impl<P: Pos> AddN<Pz<P>> for Z {
    type Out = Pz<P>;
}

// -- what a consumer's bound reads like under route Z -----------------------
pub struct UFixedZ<Ib, Fb>(PhantomData<(Ib, Fb)>);
pub trait MulIdentity {
    const ONE_EXISTS: bool;
}
impl<Ib: Nat + OneRepresentable, Fb: Nat> MulIdentity for UFixedZ<Ib, Fb> {
    const ONE_EXISTS: bool = true;
}

pub type N0 = Z;
pub type N8 = Pz<O<O<O<H>>>>;
const _: () = {
    assert!(<UFixedZ<N8, N0> as MulIdentity>::ONE_EXISTS);
};
// The negative case is a compile-fail, not a value: `UFixedZ<N0, N8>` has no
// `MulIdentity` impl, which is the point. Asserting it here would not compile,
// so it belongs in a trybuild fixture, not in this file.
