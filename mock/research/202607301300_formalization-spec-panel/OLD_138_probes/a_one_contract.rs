//! 138 probe A. Does ONE contract cover the numeral families, with the
//! container DERIVED rather than written (130b:39-48, 110:3251)?
//!
//! Redoes 130 section 10, whose own version wrote the container as a
//! parameter and whose cited sources do not exist in the tree.
//!
//! Gate-free: no #![feature], no -Z flag, no_std.
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");
include!("nats.rs");

// ---------------------------------------------------------------- the contract
// 110:911-916, four members. NOT five: the container is not a member.
pub trait Numeral {
    type Radix: Nat; // Rad<P>, a sealed Pos; here its value
    type Precision: Nat; // significand digit count, in DIGITS of the radix
    type Exponent: ExponentForm;
    type Domain: SignDomain;
}
// The container is NOT a member of Numeral (110:911-916 has four). It lives on
// Lowering (110:3246-3252) and is derived there, never declared as an axis.
// 130:684 put `type Store: Container` on the numeral, which is what op refused.
pub trait Lowering<N: Numeral> {
    type StoredWidth: Nat;
    type Container;
}

pub trait ExponentForm {
    /// The number of distinct grids the value set is a union of.
    const GRIDS: u32;
    /// Whether `Underflow` has a bottom to fall off (110:1038-1039).
    const HAS_BOTTOM: bool;
}
pub trait SignDomain {
    const SIGN_BITS: u32;
    type N;
}

pub struct NonNegative;
pub struct Symmetric;
pub struct AsymmetricLow;
impl SignDomain for NonNegative {
    const SIGN_BITS: u32 = 0;
    type N = Term;
}
impl SignDomain for Symmetric {
    const SIGN_BITS: u32 = 1;
    type N = N1;
}
impl SignDomain for AsymmetricLow {
    const SIGN_BITS: u32 = 1;
    type N = N1;
}

// ------------------------------------------------------- the two exponent forms
// 110:918-921. Implicit<E, A, B>; Ranged<EMIN, EMAX, U, S>.
// The exponent's VALUE is a read; the FORM is the type (130:699-703).
pub struct Implicit<E, A, B>(PhantomData<(E, A, B)>);
pub struct Ranged<EMIN, EMAX, U, S>(PhantomData<(EMIN, EMAX, U, S)>);

pub struct Gradual;
pub struct Abrupt;
pub struct NoSpecials;
pub struct IeeeSpecials;

impl<E, A, B> ExponentForm for Implicit<E, A, B> {
    const GRIDS: u32 = 1;
    const HAS_BOTTOM: bool = false;
}
impl<EMIN: Spanning<EMAX>, EMAX, U, S> ExponentForm for Ranged<EMIN, EMAX, U, S>
where
    Span<EMIN, EMAX>: Nat,
{
    const GRIDS: u32 = <Span<EMIN, EMAX> as Nat>::V;
    const HAS_BOTTOM: bool = true;
}

// EMIN and EMAX are carried as the SPAN nat plus a sign-free offset, since a
// signed subtraction in type position is exactly what the design refuses.
// Span is written by whoever writes the bounds, which is the same act.
pub type Span<EMIN, EMAX> = <EMIN as Spanning<EMAX>>::S;
pub trait Spanning<EMAX> {
    type S;
}

// --------------------------------------------------- the derived storage widths
// Implicit: W_S = sign + precision. One grid, one quantum, no exponent field.
// Ranged, radix two, hidden bit: W_S = sign + (p - 1) + bitlen(span).
// The bitlen is Len, which the ladder already has.
// Len gives a unary tally; turn it back into a binary nat to add it.
pub trait ToBin {
    type B;
}
impl ToBin for Z {
    type B = Term;
}
impl<T: ToBin> ToBin for S<T>
where
    <T as ToBin>::B: AddC<Term>,
{
    type B = <<T as ToBin>::B as AddC<Term>>::O;
}

pub type ExpBits<SPAN> = <<SPAN as Len>::L as ToBin>::B;

// ------------------------------------------------------------- the five families
// UFixed and IFixed differ ONLY in Domain. FastFloat differs from UFixed only
// in ExponentForm. Decimal differs only in Radix. DecFixed is radix ten with an
// Implicit exponent, and it is the numeral 110:2380 names and 130 has no slot for.

pub struct Num<R, P, E, D>(PhantomData<(R, P, E, D)>);

impl<R: Nat, P: Nat, E: ExponentForm, D: SignDomain> Numeral for Num<R, P, E, D> {
    type Radix = R;
    type Precision = P;
    type Exponent = E;
    type Domain = D;
}

// --------------------------------------------------- the width derivation, keyed
// The significand's BIT width is ceil(P * log2(R)). At radix two it is P and the
// derivation is arithmetic. At any other radix it is not, and the design already
// says where that fact lives: Encoding::Fields, nested inside Lowering (110:1047-1051).
// So `SigBits` has one blanket impl at radix two and one bridge row per decimal
// precision, and the SHAPE of that split is the finding, not the rows.
pub trait SigBits {
    type B;
}
pub struct Radix2Sig<P>(PhantomData<P>);
impl<P> SigBits for Radix2Sig<P> {
    type B = P;
}

pub struct DecSig<P>(PhantomData<P>);
impl SigBits for DecSig<N7> {
    type B = N24;
} // decimal32,  ceil(7*log2 10)
impl SigBits for DecSig<N16> {
    type B = N54;
} // decimal64,  ceil(16*log2 10)
impl SigBits for DecSig<N34> {
    type B = N113;
} // decimal128, ceil(34*log2 10)

// The lowering for a radix-two numeral. Nothing else is written.
pub struct Bin;
impl<P: Nat, E, A, B, D: SignDomain> Lowering<Num<N2, P, Implicit<E, A, B>, D>> for Bin
where
    SignNat<D>: Add<P>,
    <SignNat<D> as Add<P>>::O: Nat + Container,
{
    type StoredWidth = <SignNat<D> as Add<P>>::O;
    type Container = <<SignNat<D> as Add<P>>::O as Container>::C;
}

impl<P: Nat, EMIN, EMAX, U, S, D: SignDomain> Lowering<Num<N2, P, Ranged<EMIN, EMAX, U, S>, D>>
    for Bin
where
    EMIN: Spanning<EMAX>,
    Span<EMIN, EMAX>: Nat,
    Span<EMIN, EMAX>: Len,
    <Span<EMIN, EMAX> as Len>::L: ToBin,
    P: Dec,
    SignNat<D>: Add<<P as Dec>::O>,
    <SignNat<D> as Add<<P as Dec>::O>>::O: Add<ExpBits<Span<EMIN, EMAX>>>,
    FW<P, D, EMIN, EMAX>: Nat + Container,
{
    type StoredWidth = FW<P, D, EMIN, EMAX>;
    type Container = <FW<P, D, EMIN, EMAX> as Container>::C;
}
pub type FW<P, D, EMIN, EMAX> =
    <<SignNat<D> as Add<<P as Dec>::O>>::O as Add<ExpBits<Span<EMIN, EMAX>>>>::O;

pub type SignNat<D> = <D as SignDomain>::N;
pub type N34 = D0<D1<D0<D0<D0<D1<Term>>>>>>;
pub type N113 = D1<D0<D0<D0<D1<D1<D1<Term>>>>>>>;

// ------------------------------------------------------------------- the spans
impl Spanning<N127> for ENeg126 {
    type S = N254;
} // binary32
impl Spanning<N1023> for ENeg1022 {
    type S = N2046;
} // binary64
impl Spanning<N15> for ENeg14 {
    type S = N30;
} // binary16
impl Spanning<N8> for ENeg6 {
    type S = N15;
} // E4M3
impl Spanning<N15> for ENeg14b {
    type S = N30;
} // E5M2 (same span, distinct EMIN)
impl Spanning<N384> for ENeg383 {
    type S = N768;
} // decimal64
pub struct ENeg126;
pub struct ENeg1022;
pub struct ENeg14;
pub struct ENeg6;
pub struct ENeg14b;
pub struct ENeg383;

// ------------------------------------------------------------------ the aliases
pub type UFixed<PREC> = Num<N2, PREC, Implicit<ENeg6, N1, Term>, NonNegative>;
pub type IFixed<PREC> = Num<N2, PREC, Implicit<ENeg6, N1, Term>, Symmetric>;
pub type FastFloat<PREC, EMIN, EMAX> =
    Num<N2, PREC, Ranged<EMIN, EMAX, Gradual, IeeeSpecials>, Symmetric>;
pub type Decimal<PREC, EMIN, EMAX> =
    Num<N10, PREC, Ranged<EMIN, EMAX, Gradual, IeeeSpecials>, Symmetric>;
pub type DecFixed<PREC, E> = Num<N10, PREC, Implicit<E, N1, Term>, Symmetric>;

// ------------------------------------------------- every coordinate reads as itself
pub type B32 = FastFloat<N24, ENeg126, N127>;
pub type B64 = FastFloat<N53, ENeg1022, N1023>;
pub type B16 = FastFloat<N11, ENeg14, N15>;
pub type E4M3 = FastFloat<N4, ENeg6, N8>;
pub type E5M2 = FastFloat<N3, ENeg14b, N15>;
pub type Q13_3 = UFixed<N16>;
pub type S12_3 = IFixed<N16>;
pub type Money = DecFixed<N16, ENeg2>;
pub struct ENeg2;

const _: () = assert!(<<B32 as Numeral>::Radix as Nat>::V == 2);
const _: () = assert!(<<B32 as Numeral>::Precision as Nat>::V == 24);
const _: () = assert!(<<Decimal<N16, ENeg383, N384> as Numeral>::Radix as Nat>::V == 10);
const _: () = assert!(<<Money as Numeral>::Radix as Nat>::V == 10);
const _: () = assert!(<<Q13_3 as Numeral>::Precision as Nat>::V == 16);

// The float storage-width law, checked as a VALUE, radix two, hidden bit:
//   W_S = sign + (p - 1) + bitlen(EMAX - EMIN + 1)
// and the two reserved exponent codes are exactly the slack the bit length leaves.
const _: () = assert!(<<Bin as Lowering<B32>>::StoredWidth as Nat>::V == 32);
const _: () = assert!(<<Bin as Lowering<B64>>::StoredWidth as Nat>::V == 64);
const _: () = assert!(<<Bin as Lowering<B16>>::StoredWidth as Nat>::V == 16);
const _: () = assert!(<<Bin as Lowering<E4M3>>::StoredWidth as Nat>::V == 8);
const _: () = assert!(<<Bin as Lowering<E5M2>>::StoredWidth as Nat>::V == 8);
const _: () = assert!(<<Bin as Lowering<Q13_3>>::StoredWidth as Nat>::V == 16);
const _: () = assert!(<<Bin as Lowering<S12_3>>::StoredWidth as Nat>::V == 17);

// and the container derived from it, checked as a TYPE EQUALITY, so the file
// does not build if a rung is wrong.
pub fn c_b32(x: <Bin as Lowering<B32>>::Container) -> u32 {
    x
}
pub fn c_b64(x: <Bin as Lowering<B64>>::Container) -> u64 {
    x
}
pub fn c_b16(x: <Bin as Lowering<B16>>::Container) -> u16 {
    x
}
pub fn c_e4m3(x: <Bin as Lowering<E4M3>>::Container) -> u8 {
    x
}
pub fn c_q133(x: <Bin as Lowering<Q13_3>>::Container) -> u16 {
    x
}
pub fn c_s123(x: <Bin as Lowering<S12_3>>::Container) -> u32 {
    x
}

// One generic algorithm binding the contract and naming no family (130:691).
pub const fn quantum_decades<N: Numeral>() -> u32 {
    <<N as Numeral>::Radix as Nat>::V
}
const _: () = assert!(quantum_decades::<B32>() == 2);
const _: () = assert!(quantum_decades::<Money>() == 10);
