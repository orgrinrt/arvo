// TEST C: the alternative encoding. Instead of "the law is the absence of an
// impl over a hand-partitioned index set", compute the law's truth value with
// a total type-level function whose per-constructor obligations the compiler
// forces, then condition ONE impl on it.
//
// Claims under test:
//   C1 the whole law table becomes one impl, so the coherence wall of TEST B
//      cannot be hit;
//   C2 adding a Resolution constructor is a compile error until it classifies
//      itself, so the SubstituteZero slip is unwriteable;
//   C3 the signedness-uniform fact composes with no conflict;
//   C4 the diagnostic is recoverable via on_unimplemented at the composition.

#![allow(dead_code)]

pub trait TruthMarker {}
pub struct True;
impl TruthMarker for True {}
pub struct False;
impl TruthMarker for False {}

pub trait And<Rhs> {
    type Out: TruthMarker;
}
impl And<True> for True {
    type Out = True;
}
impl And<False> for True {
    type Out = False;
}
impl And<True> for False {
    type Out = False;
}
impl And<False> for False {
    type Out = False;
}

pub trait Signedness {
    /// does this domain reach BOTH ends of the range?
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

/// Every resolution states the two lemmas the law derivation needs. The
/// obligation is TOTAL: a new constructor cannot implement Resolution without
/// answering both, which is the property the blanket-over-Resolution shape
/// does not have.
pub trait Resolution {
    /// phi(phi(x) + c) == phi(x + c) for translations that cannot cross the
    /// far end. Enough for a one-sided domain.
    type StableOneSided: TruthMarker;
    /// the same with translations in both directions.
    type StableTwoSided: TruthMarker;
}

pub struct TowardNegative;
impl Resolution for TowardNegative {
    type StableOneSided = True;
    type StableTwoSided = False;
}
pub struct TowardPositive;
impl Resolution for TowardPositive {
    type StableOneSided = True;
    type StableTwoSided = False;
}
pub struct ReduceModulo;
impl Resolution for ReduceModulo {
    type StableOneSided = True;
    type StableTwoSided = True;
}
pub struct Refuse;
impl Resolution for Refuse {
    type StableOneSided = True;
    type StableTwoSided = False;
}
// the constructor the spec's blanket impl silently swept in as associative:
pub struct SubstituteZero;
impl Resolution for SubstituteZero {
    type StableOneSided = False;
    type StableTwoSided = False;
}

/// Pick the lemma the domain needs. A type-level `if`.
pub trait StableFor<Dom> {
    type Out: TruthMarker;
}
impl<R: Resolution> StableFor<Unsigned> for R {
    type Out = R::StableOneSided;
}
impl<R: Resolution> StableFor<Signed> for R {
    type Out = R::StableTwoSided;
}

/// The law, as a computed truth value rather than as a partitioned impl set.
pub trait AddAssocOf {
    type Out: TruthMarker;
}
impl<A, B, D> AddAssocOf for ((A, B), D)
where
    A: StableFor<D>,
    B: StableFor<D>,
    <A as StableFor<D>>::Out: And<<B as StableFor<D>>::Out>,
{
    type Out = <<A as StableFor<D>>::Out as And<<B as StableFor<D>>::Out>>::Out;
}

pub trait Quantisation {
    type OverRange: Resolution;
    type UnderRange: Resolution;
}
pub trait Numeral {
    type Sign: Signedness;
}
pub trait Policy {
    type Quantisation: Quantisation;
}
pub trait Lowering {}

pub struct Number<N: Numeral, P: Policy, L: Lowering>(core::marker::PhantomData<(N, P, L)>);

pub struct Add;
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no associative addition, so it cannot be folded",
    label = "the composition's quantisation is not translation-stable on this domain"
)]
pub trait Semigroup<Op> {}

// ONE impl. Note it mentions N and P and NOT L, which is now a typing fact.
impl<N: Numeral, P: Policy, L: Lowering> Semigroup<Add> for Number<N, P, L> where
    (
        (
            <P::Quantisation as Quantisation>::OverRange,
            <P::Quantisation as Quantisation>::UnderRange,
        ),
        <N as Numeral>::Sign,
    ): AddAssocOf<Out = True>
{
}

pub struct Wrapping;
impl Quantisation for Wrapping {
    type OverRange = ReduceModulo;
    type UnderRange = ReduceModulo;
}
pub struct Clamping;
impl Quantisation for Clamping {
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
pub struct ZeroingUp;
impl Quantisation for ZeroingUp {
    type OverRange = SubstituteZero;
    type UnderRange = SubstituteZero;
}

pub struct Hot;
impl Policy for Hot {
    type Quantisation = Wrapping;
}
pub struct Warm;
impl Policy for Warm {
    type Quantisation = Clamping;
}
pub struct Sc0;
impl Policy for Sc0 {
    type Quantisation = ZeroingUp;
}
pub struct Dense;
impl Lowering for Dense {}

pub struct I16;
impl Numeral for I16 {
    type Sign = Signed;
}
pub struct U16;
impl Numeral for U16 {
    type Sign = Unsigned;
}

fn fold<T: Semigroup<Add>>() {}

fn main() {
    fold::<Number<I16, Hot, Dense>>(); // signed wrapping   -> holds
    fold::<Number<U16, Hot, Dense>>(); // unsigned wrapping -> holds
    fold::<Number<U16, Warm, Dense>>(); // unsigned clamping -> holds
                                        // fold::<Number<I16, Warm, Dense>>();  // signed clamping -> refused
    fold::<Number<U16, Sc0, Dense>>(); // SC_SAT_ZERO     -> refused, the bug
    println!("C OK: one impl, whole table, no coherence wall");
}
