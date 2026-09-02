//! The fix Knuth (file 62) named and did not build: a sealed `Arity` kind with
//! the finite constructor wrapping a `Pos`. "The two-line fix is the tower's
//! own idiom" (62:223-224). This file is that fix, compiled, plus the attack
//! from `attacker_unsealed.rs` re-run against it in `attacker_sealed.rs`.
//!
//! Same reduced `Pos`/`Cmp` stand-in as `lib_unsealed.rs`, unchanged, so the
//! only variable between the two libraries is the thing under test: whether
//! `InteriorSafety`'s parameter is closed.

use core::marker::PhantomData;

pub struct Safe;
pub struct Unsafe;
pub trait Safety {}
impl Safety for Safe {}
impl Safety for Unsafe {}

pub struct Small;
pub struct Big;
pub trait Pos {}
impl Pos for Small {}
impl Pos for Big {}

pub trait Cmp<Rhs> {
    type Out: SafetyOf;
}
impl Cmp<Small> for Small {
    type Out = AtOrAbove;
}
impl Cmp<Small> for Big {
    type Out = AtOrAbove;
}
impl Cmp<Big> for Small {
    type Out = Below;
}
impl Cmp<Big> for Big {
    type Out = AtOrAbove;
}

pub struct AtOrAbove;
pub struct Below;
pub trait SafetyOf {
    type Out: Safety;
}
impl SafetyOf for AtOrAbove {
    type Out = Safe;
}
impl SafetyOf for Below {
    type Out = Unsafe;
}

// ---------------------------------------------------------------------------
// The fix: `Arity` is closed, sealed the same way `Rad<P>`'s `AtLeastTwo` is
// (62:229-239): a private supertrait no downstream crate can name, so no
// downstream crate can implement `Arity` for a type of its own, ever.
// ---------------------------------------------------------------------------

mod sealed {
    /// Not `pub`: this module's contents are visible inside this crate only.
    /// A downstream crate cannot write `sealed::Sealed` at all; the path does
    /// not resolve for it, which is what makes the trait below closed.
    pub trait Sealed {}
}

/// The closed vocabulary `InteriorSafety` now quantifies over. Two
/// constructors, matching Knuth's proposal exactly: `Fin<P>` wraps a finite
/// `Pos`, `Unbounded` names the one infinite case the review's fixpoint story
/// needs. Nothing else can ever inhabit it, from any crate.
pub trait Arity: sealed::Sealed {}

pub struct Fin<P: Pos>(PhantomData<P>);
pub struct Unbounded;

impl<P: Pos> sealed::Sealed for Fin<P> {}
impl sealed::Sealed for Unbounded {}
impl<P: Pos> Arity for Fin<P> {}
impl Arity for Unbounded {}

/// The bound this probe adds, and the whole of the fix: `A: Arity`, not bare.
pub trait InteriorSafety<A: Arity> {
    type Out: Safety;
}

impl<Hd: Pos + Cmp<P>, P: Pos> InteriorSafety<Fin<P>> for Hd {
    type Out = <<Hd as Cmp<P>>::Out as SafetyOf>::Out;
}

impl<Hd: Pos> InteriorSafety<Unbounded> for Hd {
    type Out = Unsafe;
}

/// The legitimate mechanism, unchanged in behaviour from the unsealed
/// version, spelled at `Fin<P>` where the unsealed version spelled `P` bare.
pub fn assert_unbounded_is_always_unsafe<Hd: Pos + InteriorSafety<Unbounded, Out = Unsafe>>() {}
pub fn assert_finite_is_checked<Hd: Pos + InteriorSafety<Fin<Big>>>() {}

pub fn witness() {
    assert_unbounded_is_always_unsafe::<Small>();
    assert_unbounded_is_always_unsafe::<Big>();
    assert_finite_is_checked::<Small>();
    assert_finite_is_checked::<Big>();
}
