//! Probe 4: the two things `upward_rank`'s signature does not say.
//!
//! Probe 3's runtime demonstration (`probe_3b_*.rs.txt`) shows both shipped
//! presets returning a wrong ordering from a four-node chain: `Hot` gives
//! `[144, 44, 200, 100]` where the answer is `[400, 300, 200, 100]`, inverting
//! the order between two paths; `Precise` gives `[255, 255, 200, 100]`, tying
//! two nodes the true ranks separate. Neither says anything.
//!
//! Both failures have one cause, and it is in the signature rather than in the
//! body. `upward_rank` returns `C::Array<W>`: the RESULT lives in the OPERAND
//! numeral. The design already refuses that shape everywhere else. `mul_full`
//! is `N1 x N2 -> mulnum(N1, N2)` (`49:269`); the MAC accumulator is a
//! separately-computed numeral (`49:260-265`); the exact-widening family's
//! whole point is that a computed result gets a computed numeral.
//!
//! CLAIM A. The design's answer for a fold-shaped algorithm is the same as for
//! `mul_full`: a result-numeral map. `foldnum(W, A)` is the numeral wide enough
//! to hold `A` operands of `W` summed exactly, its precision is `W`'s plus
//! `ceil(log2 A)`, and the algorithm's signature names it. Written below and
//! compiled at three instances.
//!
//! CLAIM B. The arity that map needs is the one probe 1 established: the
//! capacity's own `Dim`. So `foldnum(W, C::Dim)` is spelled entirely from
//! things the signature already carries, and the caller declares nothing.
//!
//! CLAIM C. `FromConstant` is a partial map declared total.
//! `arvo-numeric-contracts/src/lib.rs:85-88` is
//! `fn from_constant<const C: USize>() -> Self` with no obligation on `C`, and
//! `arvo/src/traits/from_constant.rs:40` implements it as
//! `from_raw((C as $ctype) << $f)` with no check. Measured on the shipped tree:
//! `UFixed<8,16,Hot>::from_constant::<300>()` yields raw 19660800 against a
//! 24-bit ceiling of 16777215, so the transparent container holds a bit pattern
//! the type says cannot exist. `arvo-spectral` reaches this surface with a
//! literal 2 (`fiedler.rs:165`), and every one of the four crates reaches it
//! with a literal 0.
//!
//! CLAIM D. The fix is one const parameter moved from the method to the trait,
//! which costs no unstable feature because a bare const parameter is a
//! standalone argument rather than an expression. Representability becomes a
//! where-clause the algorithm crate writes once, and an unrepresentable
//! constant is `E0277` at the call site instead of a wrong number at runtime.
//!
//! EXPECTED: COMPILES CLEAN. Probe 4b is the refusal that makes CLAIM D real.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_4_the_result_numeral_and_the_constant.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use tower::nat::{Nat, Pos, Pz, H, I, O};

// ---------------------------------------------------------------------------
// CLAIM A + B: the result numeral.
//
// A stand-in `Numeral` carrying only the member this question needs, its
// precision. The real contract has four (`49:109-114`); the other three ride
// along unchanged through a fold, which is why they are absent here rather
// than elided.
// ---------------------------------------------------------------------------

pub trait Numeral {
    type Precision: Pos;
}

/// `ceil(log2(A))`, the extra precision `A` summands cost.
///
/// Spelled at the two instances the call sites below use rather than as
/// general tower arithmetic. The general form is `bit_length(A - 1)`, which is
/// the tower's own `Dec` (`vu_nat.rs:192`) composed with a constructor-depth
/// count, and it is one of the two additions this surface owes the tower; the
/// other is `Inc`. Neither is the object under review here, which is the
/// SIGNATURE, and inventing them badly would put a projection chain in a
/// consumer-facing type, which is the construction the design spent a whole
/// stretch keeping out (`49:1009-1011`).
pub trait BitsFor {
    type Out: Pos;
}

/// `foldnum(W, A)`: the numeral that holds `A` summands of `W` exactly.
pub trait FoldNum<A> {
    type Out: Numeral;
}

/// The concrete numeral shape this probe folds into. `Prec` is the only member
/// the widening touches.
pub struct Num<Prec>(PhantomData<Prec>);
impl<Prec: Pos> Numeral for Num<Prec> {
    type Precision = Prec;
}

/// Addition on the tower, again spelled at the instances used rather than in
/// general, because the object under review is the SIGNATURE.
pub trait AddPos<Rhs> {
    type Out: Pos;
}

type P1 = H; // 1
type P2 = O<H>; // 2
type P3 = I<H>; // 3
type P8 = O<O<O<H>>>; // 8
pub type P8Public = P8;
pub type P10Public = P10;
type P10 = O<I<O<H>>>; // 10
type P14 = O<I<I<H>>>; // 14
type P16 = O<O<O<O<H>>>>; // 16
type P64 = O<O<O<O<O<O<H>>>>>>; // 64
type P6 = O<I<H>>; // 6
type P19 = I<I<O<O<H>>>>; // 19
type P22 = O<I<I<O<H>>>>; // 22

const _: () = assert!(<Pz<P8> as Nat>::VAL == 8);
const _: () = assert!(<Pz<P10> as Nat>::VAL == 10);
const _: () = assert!(<Pz<P14> as Nat>::VAL == 14);
const _: () = assert!(<Pz<P16> as Nat>::VAL == 16);
const _: () = assert!(<Pz<P6> as Nat>::VAL == 6);
const _: () = assert!(<Pz<P19> as Nat>::VAL == 19);
const _: () = assert!(<Pz<P22> as Nat>::VAL == 22);
const _: () = assert!(<Pz<P64> as Nat>::VAL == 64);

// ceil(log2 4) = 2 and ceil(log2 64) = 6, const-asserted above.
impl BitsFor for O<O<H>> {
    type Out = P2;
}
impl BitsFor for P64 {
    type Out = P6;
}

impl AddPos<P2> for P8 {
    type Out = P10;
}
impl AddPos<P6> for P8 {
    type Out = P14;
}
impl AddPos<P6> for P16 {
    type Out = P22;
}

impl<Prec, A> FoldNum<A> for Num<Prec>
where
    Prec: Pos + AddPos<<A as BitsFor>::Out>,
    A: Pos + BitsFor,
{
    type Out = Num<<Prec as AddPos<<A as BitsFor>::Out>>::Out>;
}

// ---------------------------------------------------------------------------
// The rank signature the design gets. Compare with the shipped one
// (`arvo-graph/src/rank.rs:34-37`), whose return type is `C::Array<W>`.
// ---------------------------------------------------------------------------

pub trait CapacityWithNat {
    type Array<T>;
    type Dim: Pos;
}

pub struct Ranks<C: CapacityWithNat, N: Numeral>(PhantomData<(C, N)>);

/// The result numeral is computed from the operand numeral and the capacity,
/// exactly as `mulnum` is computed from its two operands.
pub fn upward_rank_widening<C, W>() -> Ranks<C, <W as FoldNum<C::Dim>>::Out>
where
    C: CapacityWithNat,
    W: Numeral + FoldNum<C::Dim>,
{
    Ranks(PhantomData)
}

pub struct Cap4;
impl CapacityWithNat for Cap4 {
    type Array<T> = [T; 4];
    type Dim = O<O<H>>; // 4
}

pub struct Cap64;
impl CapacityWithNat for Cap64 {
    type Array<T> = [T; 64];
    type Dim = P64;
}

/// Eight-bit weights over four nodes: `ceil(log2 4) = 2`, so the rank numeral
/// carries ten. That is the type the shipped test at
/// `arvo-graph/tests/rank.rs:13` needed and did not get; 400 fits in ten bits.
pub fn rank_of_a_four_node_dag() -> Ranks<Cap4, Num<P10>> {
    upward_rank_widening::<Cap4, Num<P8>>()
}

/// Eight-bit weights over sixty-four nodes: six more bits, so fourteen. The
/// number a maintainer would otherwise have to work out and put in a comment
/// is in the return type.
pub fn rank_of_a_sixty_four_node_dag() -> Ranks<Cap64, Num<P14>> {
    upward_rank_widening::<Cap64, Num<P8>>()
}

/// Q0.16-style operand precision over sixty-four nodes.
pub fn rank_of_a_wide_weight() -> Ranks<Cap64, Num<P22>> {
    upward_rank_widening::<Cap64, Num<P16>>()
}

// ---------------------------------------------------------------------------
// CLAIM C + D: representability as a bound.
// ---------------------------------------------------------------------------

/// Today's shape (`arvo-numeric-contracts/src/lib.rs:85-88`): the constant is a
/// parameter of the METHOD, so no bound can mention it and no impl can be
/// absent for one value of it.
pub trait FromConstantAsShipped {
    fn from_constant<const C: u64>() -> Self;
}

/// The design's shape: the constant keys the TRAIT, so the impl set is the
/// representable set and a where-clause can name exactly the constants an
/// algorithm uses. No unstable feature: `C` appears as a standalone argument,
/// never inside an expression, which is the one thing const generics already
/// allow.
pub trait FromConstantKeyed<const C: u64> {
    fn get() -> Self;
}

/// A numeral with an integer part of eight bits and no fraction: 0..=255.
pub struct U8Num;
/// A purely fractional numeral, range `[0, 1)`: the shape whose `ONE` the
/// review already found unrepresentable, here with its `TWO` and its `ZERO`.
pub struct Q0_15;

impl FromConstantKeyed<0> for U8Num {
    fn get() -> Self {
        U8Num
    }
}
impl FromConstantKeyed<1> for U8Num {
    fn get() -> Self {
        U8Num
    }
}
impl FromConstantKeyed<2> for U8Num {
    fn get() -> Self {
        U8Num
    }
}
// No impl at 300: `U8Num` cannot hold it. Probe 4b is a caller that wants one.

impl FromConstantKeyed<0> for Q0_15 {
    fn get() -> Self {
        Q0_15
    }
}
// No impl at 1 or 2 for `Q0_15`, because `[0, 1)` contains neither. This is the
// `Identity` finding (`UFixed<0, 8>::ONE` held raw zero) generalised from the
// one constant `Identity` names to every constant a consumer can write.

/// `dense_laplacian_lambda_max_bound` (`arvo-spectral/src/fiedler.rs:160-166`)
/// with its two literals promoted into its signature. A reader now knows what
/// the function needs of its weight type without reading its body.
pub fn lambda_max_bound<F>() -> F
where
    F: FromConstantKeyed<0> + FromConstantKeyed<2>,
{
    let _zero = <F as FromConstantKeyed<0>>::get();
    <F as FromConstantKeyed<2>>::get()
}

pub fn bound_on_a_numeral_that_holds_two() -> U8Num {
    lambda_max_bound::<U8Num>()
}

/// The four crates' shared literal, which every numeral holds, so this bound is
/// free wherever it appears.
pub fn zero_init<W: FromConstantKeyed<0>>() -> W {
    <W as FromConstantKeyed<0>>::get()
}

pub fn zero_on_a_fractional_numeral() -> Q0_15 {
    zero_init::<Q0_15>()
}
