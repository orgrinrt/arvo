// PROBE D (section 6): making the derived law's refusal name the type the
// consumer wrote. Self-contained, no dependencies, no gates.
//
// The problem probe C ends on: under the computed-truth-value encoding that 02
// section 4 proposes and that 03 endorses, the failing obligation is the truth
// marker, so `#[diagnostic::on_unimplemented]` on the law trait never fires and
// the consumer reads `False: IsTrue`. 02's own `c4_diag.rs` puts the attribute
// on `IsTrue` instead, whose `{Self}` is `False`, so the message can say what
// went wrong but cannot say what it went wrong ABOUT.
//
// The repair: parameterise the verdict marker by the composition it is a
// verdict about, purely so the attribute has the consumer's type in scope.
//
// RESULT: it works. Full output at the bottom.

#![allow(dead_code)]
use core::marker::PhantomData;

pub struct Unsigned;
pub struct Signed;
pub struct TowardNegative;
pub struct TowardPositive;
pub struct ReduceModulo;
pub struct True;
pub struct False;

pub trait Stable<Dom> {
    type Out;
}
impl<D> Stable<D> for ReduceModulo {
    type Out = True;
}
impl Stable<Unsigned> for TowardNegative {
    type Out = True;
}
impl Stable<Signed> for TowardNegative {
    type Out = False;
}
impl Stable<Unsigned> for TowardPositive {
    type Out = True;
}
impl Stable<Signed> for TowardPositive {
    type Out = False;
}

pub trait And<B> {
    type Out;
}
impl And<True> for True {
    type Out = True;
}
impl And<False> for True {
    type Out = False;
}
impl<B> And<B> for False {
    type Out = False;
}

/// `True` proves the law for any composition; `False` proves it for none. The
/// parameter `C` is carried only so the diagnostic can name it.
#[diagnostic::on_unimplemented(
    message = "`{C}` has no associative addition",
    label = "this composition cannot be folded",
    note = "Its out-of-range rule is not translation-stable on a signed domain: clamping the intermediate and clamping the result can disagree. Fold under a wrapping out-of-range rule, or accumulate in a wider numeral and quantise once at the end."
)]
pub trait Proves<C> {}
impl<C> Proves<C> for True {}

pub trait Quantisation {
    type OverRange;
    type UnderRange;
}
pub trait Policy {
    type Quantisation: Quantisation;
}
pub trait Numeral {
    type Sign;
}

pub struct Fix<const I: u16, const F: u16, S>(PhantomData<S>);
impl<const I: u16, const F: u16, S> Numeral for Fix<I, F, S> {
    type Sign = S;
}
pub struct Number<N, S>(PhantomData<(N, S)>);

pub trait AddAssoc {}
impl<N: Numeral, S: Policy> AddAssoc for Number<N, S>
where
    <S::Quantisation as Quantisation>::OverRange: Stable<N::Sign>,
    <S::Quantisation as Quantisation>::UnderRange: Stable<N::Sign>,
    <<S::Quantisation as Quantisation>::OverRange as Stable<N::Sign>>::Out:
        And<<<S::Quantisation as Quantisation>::UnderRange as Stable<N::Sign>>::Out>,
    <<<S::Quantisation as Quantisation>::OverRange as Stable<N::Sign>>::Out as And<
        <<S::Quantisation as Quantisation>::UnderRange as Stable<N::Sign>>::Out,
    >>::Out: Proves<Number<N, S>>,
{
}

pub struct WarmQ;
impl Quantisation for WarmQ {
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
pub struct Warm;
impl Policy for Warm {
    type Quantisation = WarmQ;
}
pub struct HotQ;
impl Quantisation for HotQ {
    type OverRange = ReduceModulo;
    type UnderRange = ReduceModulo;
}
pub struct Hot;
impl Policy for Hot {
    type Quantisation = HotQ;
}

pub type IFixed<const I: u16, const F: u16, S> = Number<Fix<I, F, Signed>, S>;
pub fn fold<T: AddAssoc>() {}

pub fn ok() {
    fold::<IFixed<13, 3, Hot>>()
} // COMPILES
pub fn bad() {
    fold::<IFixed<13, 3, Warm>>()
} // REFUSED, deliberately

// does the message survive four frames of generic plumbing?
pub trait TotalOrd {}
pub trait FromConstant {}
impl<N, S> TotalOrd for Number<N, S> {}
impl<N, S> FromConstant for Number<N, S> {}
pub fn upward_rank<W: AddAssoc + TotalOrd + FromConstant>(_n: usize) {}
pub fn plan<W: AddAssoc + TotalOrd + FromConstant>() {
    upward_rank::<W>(0)
}
pub fn schedule<W: AddAssoc + TotalOrd + FromConstant>() {
    plan::<W>()
}
pub fn app() {
    schedule::<IFixed<13, 3, Warm>>()
}

// RESULT, verbatim, for `bad()`:
//
//   error[E0277]: `Number<Fix<13, 3, Signed>, Warm>` has no associative addition
//     --> src/lib.rs:65:23
//      |
//   65 | pub fn bad() { fold::<IFixed<13, 3, Warm>>() }
//      |                       ^^^^^^^^^^^^^^^^^^^ this composition cannot be folded
//      |
//   help: the trait `Proves<Number<Fix<13, 3, Signed>, Warm>>` is not implemented for `False`
//      = note: Its out-of-range rule is not translation-stable on a signed domain:
//              clamping the intermediate and clamping the result can disagree.
//              Fold under a wrapping out-of-range rule, or accumulate in a wider
//              numeral and quantise once at the end.
//   note: required for `Number<Fix<13, 3, Signed>, Warm>` to implement `AddAssoc`
//
// The message names the composition, the label sits on the consumer's own span,
// the note carries the remediation. No truncation, no long-type file.
//
// And for `app()`, four frames deep, the error is reported at the outermost
// concrete instantiation and reads identically:
//
//   error[E0277]: `Number<Fix<13, 3, Signed>, Warm>` has no associative addition
//     |
//   77 | pub fn app() { schedule::<IFixed<13, 3, Warm>>() }
//      |                           ^^^^^^^^^^^^^^^^^^^ this composition cannot be folded
//
// which is evidence against 04 section 6's impression that a projected-bound
// failure lands "eight bounds deep" for the person debugging it.
