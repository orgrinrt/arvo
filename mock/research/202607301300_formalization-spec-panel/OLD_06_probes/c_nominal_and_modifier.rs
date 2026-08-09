// PROBE C (sections 4 and 5): nominal on both sides, plus a delegating modifier
// that changes exactly one axis. Self-contained, no dependencies, no gates.
//
// Two questions. First, does a composition render completely when the numeral
// and the strategy are both NAMES rather than structural records? Second, what
// does a consumer write to change one of ten axes without spelling the other
// nine, and does the law derivation still resolve through it?
//
// RESULT: yes to both. Every rendered type below is complete, untruncated, and
// spills to no long-type file, and every token in it is a token the consumer
// wrote. Outputs at the bottom.

#![allow(dead_code)]
use core::marker::PhantomData;

pub struct Unsigned;
pub struct Signed;
pub struct ToEven;
pub struct TowardNegative;
pub struct TowardPositive;
pub struct ReduceModulo;
pub struct Refuse;
pub struct TowardZero;
pub struct Exact;
pub struct NarrowedToOperand;
pub struct Minimum;
pub struct DoubleLogical;
pub struct NoWidening;
pub struct InContainer;
pub struct PerOperation;
pub struct Dense;
pub struct Bitpacked;

pub trait Quantisation {
    type UnderMidpoint;
    type OnMidpoint;
    type OverMidpoint;
    type OverRange;
    type UnderRange;
}
pub trait Policy {
    type Quantisation: Quantisation;
    type Growth;
}
pub trait Lowering {
    type StoredWidth;
    type Widening;
    type Layout;
}
pub trait Numeral {
    type Sign;
}

pub struct Fix<const I: u16, const F: u16, S>(PhantomData<S>);
impl<const I: u16, const F: u16, S> Numeral for Fix<I, F, S> {
    type Sign = S;
}

pub struct Warm;
pub struct WarmQ;
impl Quantisation for WarmQ {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
impl Policy for Warm {
    type Quantisation = WarmQ;
    type Growth = Exact;
}
impl Lowering for Warm {
    type StoredWidth = DoubleLogical;
    type Widening = InContainer;
    type Layout = Dense;
}

pub struct Hot;
pub struct HotQ;
impl Quantisation for HotQ {
    type UnderMidpoint = TowardZero;
    type OnMidpoint = TowardZero;
    type OverMidpoint = TowardZero;
    type OverRange = ReduceModulo;
    type UnderRange = ReduceModulo;
}
impl Policy for Hot {
    type Quantisation = HotQ;
    type Growth = NarrowedToOperand;
}
impl Lowering for Hot {
    type StoredWidth = Minimum;
    type Widening = NoWidening;
    type Layout = Dense;
}

// ---- the one-axis override --------------------------------------------
// `OverRangeOf<S, R>` is `S` in every respect except its out-of-range rule.
// The consumer changes one axis by naming one axis.
pub struct OverRangeOf<S, R>(PhantomData<(S, R)>);
pub struct OverRangeQ<Q, R>(PhantomData<(Q, R)>);
impl<Q: Quantisation, R> Quantisation for OverRangeQ<Q, R> {
    type UnderMidpoint = Q::UnderMidpoint;
    type OnMidpoint = Q::OnMidpoint;
    type OverMidpoint = Q::OverMidpoint;
    type OverRange = R;
    type UnderRange = Q::UnderRange;
}
impl<S: Policy, R> Policy for OverRangeOf<S, R> {
    type Quantisation = OverRangeQ<S::Quantisation, R>;
    type Growth = S::Growth;
}
impl<S: Lowering, R> Lowering for OverRangeOf<S, R> {
    type StoredWidth = S::StoredWidth;
    type Widening = S::Widening;
    type Layout = S::Layout;
}

// the same shape on a lowering axis
pub struct LayoutOf<S, L>(PhantomData<(S, L)>);
impl<S: Policy, L> Policy for LayoutOf<S, L> {
    type Quantisation = S::Quantisation;
    type Growth = S::Growth;
}
impl<S: Lowering, L> Lowering for LayoutOf<S, L> {
    type StoredWidth = S::StoredWidth;
    type Widening = S::Widening;
    type Layout = L;
}

pub struct Number<N, S>(PhantomData<(N, S)>)
where
    N: Numeral,
    S: Policy + Lowering;

#[diagnostic::on_unimplemented(
    message = "`{Self}` has no associative addition",
    note = "Its out-of-range rule is not translation-stable on a signed domain."
)]
pub trait AddAssoc {}
pub fn fold<T: AddAssoc>() {}

pub type IFixed<const I: u16, const F: u16, S> = Number<Fix<I, F, Signed>, S>;

// ---- the derivation, to check it still resolves through a modifier ------
pub struct True;
pub struct False;
pub trait Stable {
    type TwoSided;
}
impl Stable for TowardNegative {
    type TwoSided = False;
}
impl Stable for TowardPositive {
    type TwoSided = False;
}
impl Stable for ReduceModulo {
    type TwoSided = True;
}
impl Stable for Refuse {
    type TwoSided = False;
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

pub trait IsTrue {}
impl IsTrue for True {}

impl<const I: u16, const F: u16, S> AddAssoc for Number<Fix<I, F, Signed>, S>
where
    S: Policy + Lowering,
    <S::Quantisation as Quantisation>::OverRange: Stable,
    <S::Quantisation as Quantisation>::UnderRange: Stable,
    <<S::Quantisation as Quantisation>::OverRange as Stable>::TwoSided:
        And<<<S::Quantisation as Quantisation>::UnderRange as Stable>::TwoSided>,
    <<<S::Quantisation as Quantisation>::OverRange as Stable>::TwoSided as And<
        <<S::Quantisation as Quantisation>::UnderRange as Stable>::TwoSided,
    >>::Out: IsTrue,
{
}

// COMPILES: signed wrapping folds.
pub fn ok_hot() {
    fold::<IFixed<13, 3, Hot>>()
}
// COMPILES: Warm with only its out-of-range rule swapped, through the modifier.
pub fn ok_modified() {
    fold::<IFixed<13, 3, OverRangeOf<Warm, ReduceModulo>>>()
}

// REFUSED, deliberately. The three rendered types are the result.
pub fn case_a() {
    fold::<IFixed<13, 3, Warm>>()
}
pub fn case_b() {
    fold::<IFixed<13, 3, OverRangeOf<Warm, Refuse>>>()
}
pub fn case_c() {
    fold::<IFixed<13, 3, LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>>>()
}

// RESULTS.
//
// With the derivation bodies REMOVED (so `AddAssoc` has no impl and the
// attribute on it is what fires), the three refusals render as:
//
//   `Number<Fix<13, 3, Signed>, Warm>` has no associative addition
//   `Number<Fix<13, 3, Signed>, OverRangeOf<Warm, Refuse>>` has no associative addition
//   `Number<Fix<13, 3, Signed>, LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>>`
//       has no associative addition
//
// Complete, untruncated, no long-type spill, and the third one reads as the
// sentence the consumer meant.
//
// With the derivation bodies PRESENT, as written above, the same three refusals
// render as:
//
//   error[E0277]: the trait bound `False: IsTrue` is not satisfied
//     |
//   pub fn case_a() { fold::<IFixed<13, 3, Warm>>() }
//     |                       ^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
//   help: the trait `IsTrue` is not implemented for `False`
//   note: required for `Number<Fix<13, 3, Signed>, Warm>` to implement `AddAssoc`
//
// which is the finding: the attribute on `AddAssoc` never fires, because the
// failing obligation is `IsTrue`, not `AddAssoc`. The computed-truth-value
// encoding forfeits the diagnostic attribute on the law trait. Probe D is the
// repair.
