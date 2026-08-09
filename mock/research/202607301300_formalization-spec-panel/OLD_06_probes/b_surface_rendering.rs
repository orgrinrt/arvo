// PROBE B (section 3): alias, newtype face, and nominal numeral, side by side,
// against one ten-axis composition. Self-contained, no dependencies, no gates.
//
// The question: 04 section 7 proposes concrete newtype faces over an internal
// composition as the structural fix for the diagnostic problem. Does a face
// actually recover the spelling?
//
// RESULT: partially. A face fixes the numeral half and leaves the policy and
// lowering halves rendering in full. All three spellings truncate and spill to
// a long-type file. Quoted outputs are at the bottom.

#![allow(dead_code)]
use core::marker::PhantomData;

pub struct Implicit<const E: i16>;
pub struct Stored<const B: u16, U>(PhantomData<U>);
pub struct Unit;
pub struct Zero;
pub struct Unsigned;

pub struct Bin<X, A, B, S, const W: u16>(PhantomData<(X, A, B, S)>);

pub struct TowardNegative;
pub struct TowardPositive;
pub struct ToEven;
pub struct Exact;
pub struct DoubleLogical;
pub struct InContainer;
pub struct Dense;

pub struct Quant<U, O, V, OR, UR>(PhantomData<(U, O, V, OR, UR)>);
pub struct Pol<Q, G>(PhantomData<(Q, G)>);
pub struct Low<SW, WI, LA>(PhantomData<(SW, WI, LA)>);

pub struct Number<N, P, L>(PhantomData<(N, P, L)>);

type WarmP = Pol<Quant<ToEven, ToEven, ToEven, TowardNegative, TowardPositive>, Exact>;
type WarmL = Low<DoubleLogical, InContainer, Dense>;

// surface 1: alias, which is what the spec proposes
pub type UFixedAlias<const I: u16, const F: u16, P, L> =
    Number<Bin<Implicit<0>, Unit, Zero, Unsigned, I>, P, L>;

// surface 2: newtype face, which is what 04 proposes
#[repr(transparent)]
pub struct UFixedFace<const I: u16, const F: u16, P, L>(
    Number<Bin<Implicit<0>, Unit, Zero, Unsigned, I>, P, L>,
);

// surface 3: the numeral is a NAME rather than a structural record, and its
// axes are recovered as projections of its `Numeral` impl
pub struct Fix<const I: u16, const F: u16, S>(PhantomData<S>);
pub type UFixedNominal<const I: u16, const F: u16, P, L> = Number<Fix<I, F, Unsigned>, P, L>;

pub trait AddAssocPlain {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` has no associative addition",
    note = "Signed clamping is not translation-stable, so folding is refused."
)]
pub trait AddAssocDiag {}

pub fn fold_plain<T: AddAssocPlain>() {}
pub fn fold_diag<T: AddAssocDiag>() {}

pub fn case_alias_plain() {
    fold_plain::<UFixedAlias<13, 3, WarmP, WarmL>>()
}
pub fn case_alias_diag() {
    fold_diag::<UFixedAlias<13, 3, WarmP, WarmL>>()
}
pub fn case_face_plain() {
    fold_plain::<UFixedFace<13, 3, WarmP, WarmL>>()
}
pub fn case_face_diag() {
    fold_diag::<UFixedFace<13, 3, WarmP, WarmL>>()
}
pub fn case_nominal_plain() {
    fold_plain::<UFixedNominal<13, 3, WarmP, WarmL>>()
}
pub fn case_nominal_diag() {
    fold_diag::<UFixedNominal<13, 3, WarmP, WarmL>>()
}

// RESULTS, the message line of each, verbatim:
//
// alias, no attribute:
//   the trait bound `Number<Bin<Implicit<0>, Unit, Zero, ..., 13>, ..., ...>:
//   AddAssocPlain` is not satisfied
//
// alias, with attribute:
//   `Number<Bin<Implicit<0>, Unit, Zero, Unsigned, 13>, Pol<..., ...>, ...>`
//   has no associative addition
//
// face, no attribute:
//   the trait bound `UFixedFace<13, 3, Pol<..., ...>, ...>: AddAssocPlain`
//   is not satisfied
//
// face, with attribute:
//   `UFixedFace<13, 3, Pol<Quant<ToEven, ToEven, ..., ..., ...>, ...>, ...>`
//   has no associative addition
//
// nominal, no attribute:
//   the trait bound `Number<Fix<13, 3, Unsigned>, Pol<..., ...>, ...>:
//   AddAssocPlain` is not satisfied
//
// Every one of the six spills to a long-type file. The face recovers `13, 3`
// and loses everything on the policy side, because the face's own generic
// parameters carry the policy and lowering compositions. The nominal numeral is
// the shortest, and probe C shows what happens when the strategy side is
// nominal too.
