// Both answers in one artifact: the precision bridge and the preset key,
// composed, so that UFixed<13, 3, Warm> resolves end to end.
// Compiled under the pin, no feature gate.
#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

// ======================= the sealed bottom carrier =====================
pub trait Pos: Sealed {
    const VAL: u64;
    type Succ: Pos;
}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
impl Sealed for H {}
impl<P: Pos> Sealed for O<P> {}
impl<P: Pos> Sealed for I<P> {}
impl Pos for H {
    const VAL: u64 = 1;
    type Succ = O<H>;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
    type Succ = I<P>;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
    type Succ = O<P::Succ>;
}

pub trait PosAdd<R: Pos>: Pos {
    type Out: Pos;
}
impl PosAdd<H> for H {
    type Out = O<H>;
}
impl<Q: Pos> PosAdd<O<Q>> for H {
    type Out = I<Q>;
}
impl<Q: Pos> PosAdd<I<Q>> for H {
    type Out = O<Q::Succ>;
}
impl<P: Pos> PosAdd<H> for O<P> {
    type Out = I<P>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<O<Q>> for O<P> {
    type Out = O<<P as PosAdd<Q>>::Out>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<I<Q>> for O<P> {
    type Out = I<<P as PosAdd<Q>>::Out>;
}
impl<P: Pos> PosAdd<H> for I<P> {
    type Out = O<P::Succ>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<O<Q>> for I<P> {
    type Out = I<<P as PosAdd<Q>>::Out>;
}
impl<P: PosAdd<Q>, Q: Pos> PosAdd<I<Q>> for I<P> {
    type Out = O<<<P as PosAdd<Q>>::Out as Pos>::Succ>;
}

pub trait Nat: Sealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P: Pos>(PhantomData<P>);
impl Sealed for Z {}
impl<P: Pos> Sealed for Pz<P> {}
impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}
pub trait NatAdd<R: Nat>: Nat {
    type Out: Nat;
}
impl<R: Nat> NatAdd<R> for Z {
    type Out = R;
}
impl<P: Pos> NatAdd<Z> for Pz<P> {
    type Out = Pz<P>;
}
impl<P: PosAdd<Q>, Q: Pos> NatAdd<Pz<Q>> for Pz<P> {
    type Out = Pz<<P as PosAdd<Q>>::Out>;
}

// ======================= the precision bridge ==========================
// One impl per admitted width. The escape from a const parameter to a
// type has no other spelling under the permitted feature set: recursion
// on the const needs a const expression in type position (forbidden) and
// a base case needs a specialising impl (forbidden).
pub struct Width<const N: u16>;

#[diagnostic::on_unimplemented(
    message = "{Self} is outside the widths arvo admits",
    label = "no numeral exists at this width",
    note = "admitted widths run 0 through 63, and I + F must also lie in that range"
)]
pub trait AdmittedWidth {
    type Nat: Nat;
}
pub trait AdmittedPrecision: Nat {}
pub type NatOf<const N: u16> = <Width<N> as AdmittedWidth>::Nat;
pub type SumOf<const A: u16, const B: u16> = <NatOf<A> as NatAdd<NatOf<B>>>::Out;

impl AdmittedWidth for Width<0> {
    type Nat = Z;
}
const _: () = assert!(<<Width<0> as AdmittedWidth>::Nat as Nat>::VAL == 0);
impl AdmittedWidth for Width<1> {
    type Nat = Pz<H>;
}
const _: () = assert!(<<Width<1> as AdmittedWidth>::Nat as Nat>::VAL == 1);
impl AdmittedWidth for Width<2> {
    type Nat = Pz<O<H>>;
}
const _: () = assert!(<<Width<2> as AdmittedWidth>::Nat as Nat>::VAL == 2);
impl AdmittedWidth for Width<3> {
    type Nat = Pz<I<H>>;
}
const _: () = assert!(<<Width<3> as AdmittedWidth>::Nat as Nat>::VAL == 3);
impl AdmittedWidth for Width<4> {
    type Nat = Pz<O<O<H>>>;
}
const _: () = assert!(<<Width<4> as AdmittedWidth>::Nat as Nat>::VAL == 4);
impl AdmittedWidth for Width<5> {
    type Nat = Pz<I<O<H>>>;
}
const _: () = assert!(<<Width<5> as AdmittedWidth>::Nat as Nat>::VAL == 5);
impl AdmittedWidth for Width<6> {
    type Nat = Pz<O<I<H>>>;
}
const _: () = assert!(<<Width<6> as AdmittedWidth>::Nat as Nat>::VAL == 6);
impl AdmittedWidth for Width<7> {
    type Nat = Pz<I<I<H>>>;
}
const _: () = assert!(<<Width<7> as AdmittedWidth>::Nat as Nat>::VAL == 7);
impl AdmittedWidth for Width<8> {
    type Nat = Pz<O<O<O<H>>>>;
}
const _: () = assert!(<<Width<8> as AdmittedWidth>::Nat as Nat>::VAL == 8);
impl AdmittedWidth for Width<9> {
    type Nat = Pz<I<O<O<H>>>>;
}
const _: () = assert!(<<Width<9> as AdmittedWidth>::Nat as Nat>::VAL == 9);
impl AdmittedWidth for Width<10> {
    type Nat = Pz<O<I<O<H>>>>;
}
const _: () = assert!(<<Width<10> as AdmittedWidth>::Nat as Nat>::VAL == 10);
impl AdmittedWidth for Width<11> {
    type Nat = Pz<I<I<O<H>>>>;
}
const _: () = assert!(<<Width<11> as AdmittedWidth>::Nat as Nat>::VAL == 11);
impl AdmittedWidth for Width<12> {
    type Nat = Pz<O<O<I<H>>>>;
}
const _: () = assert!(<<Width<12> as AdmittedWidth>::Nat as Nat>::VAL == 12);
impl AdmittedWidth for Width<13> {
    type Nat = Pz<I<O<I<H>>>>;
}
const _: () = assert!(<<Width<13> as AdmittedWidth>::Nat as Nat>::VAL == 13);
impl AdmittedWidth for Width<14> {
    type Nat = Pz<O<I<I<H>>>>;
}
const _: () = assert!(<<Width<14> as AdmittedWidth>::Nat as Nat>::VAL == 14);
impl AdmittedWidth for Width<15> {
    type Nat = Pz<I<I<I<H>>>>;
}
const _: () = assert!(<<Width<15> as AdmittedWidth>::Nat as Nat>::VAL == 15);
impl AdmittedWidth for Width<16> {
    type Nat = Pz<O<O<O<O<H>>>>>;
}
const _: () = assert!(<<Width<16> as AdmittedWidth>::Nat as Nat>::VAL == 16);
impl AdmittedWidth for Width<17> {
    type Nat = Pz<I<O<O<O<H>>>>>;
}
const _: () = assert!(<<Width<17> as AdmittedWidth>::Nat as Nat>::VAL == 17);
impl AdmittedWidth for Width<18> {
    type Nat = Pz<O<I<O<O<H>>>>>;
}
const _: () = assert!(<<Width<18> as AdmittedWidth>::Nat as Nat>::VAL == 18);
impl AdmittedWidth for Width<19> {
    type Nat = Pz<I<I<O<O<H>>>>>;
}
const _: () = assert!(<<Width<19> as AdmittedWidth>::Nat as Nat>::VAL == 19);
impl AdmittedWidth for Width<20> {
    type Nat = Pz<O<O<I<O<H>>>>>;
}
const _: () = assert!(<<Width<20> as AdmittedWidth>::Nat as Nat>::VAL == 20);
impl AdmittedWidth for Width<21> {
    type Nat = Pz<I<O<I<O<H>>>>>;
}
const _: () = assert!(<<Width<21> as AdmittedWidth>::Nat as Nat>::VAL == 21);
impl AdmittedWidth for Width<22> {
    type Nat = Pz<O<I<I<O<H>>>>>;
}
const _: () = assert!(<<Width<22> as AdmittedWidth>::Nat as Nat>::VAL == 22);
impl AdmittedWidth for Width<23> {
    type Nat = Pz<I<I<I<O<H>>>>>;
}
const _: () = assert!(<<Width<23> as AdmittedWidth>::Nat as Nat>::VAL == 23);
impl AdmittedWidth for Width<24> {
    type Nat = Pz<O<O<O<I<H>>>>>;
}
const _: () = assert!(<<Width<24> as AdmittedWidth>::Nat as Nat>::VAL == 24);
impl AdmittedWidth for Width<25> {
    type Nat = Pz<I<O<O<I<H>>>>>;
}
const _: () = assert!(<<Width<25> as AdmittedWidth>::Nat as Nat>::VAL == 25);
impl AdmittedWidth for Width<26> {
    type Nat = Pz<O<I<O<I<H>>>>>;
}
const _: () = assert!(<<Width<26> as AdmittedWidth>::Nat as Nat>::VAL == 26);
impl AdmittedWidth for Width<27> {
    type Nat = Pz<I<I<O<I<H>>>>>;
}
const _: () = assert!(<<Width<27> as AdmittedWidth>::Nat as Nat>::VAL == 27);
impl AdmittedWidth for Width<28> {
    type Nat = Pz<O<O<I<I<H>>>>>;
}
const _: () = assert!(<<Width<28> as AdmittedWidth>::Nat as Nat>::VAL == 28);
impl AdmittedWidth for Width<29> {
    type Nat = Pz<I<O<I<I<H>>>>>;
}
const _: () = assert!(<<Width<29> as AdmittedWidth>::Nat as Nat>::VAL == 29);
impl AdmittedWidth for Width<30> {
    type Nat = Pz<O<I<I<I<H>>>>>;
}
const _: () = assert!(<<Width<30> as AdmittedWidth>::Nat as Nat>::VAL == 30);
impl AdmittedWidth for Width<31> {
    type Nat = Pz<I<I<I<I<H>>>>>;
}
const _: () = assert!(<<Width<31> as AdmittedWidth>::Nat as Nat>::VAL == 31);
impl AdmittedWidth for Width<32> {
    type Nat = Pz<O<O<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<32> as AdmittedWidth>::Nat as Nat>::VAL == 32);
impl AdmittedWidth for Width<33> {
    type Nat = Pz<I<O<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<33> as AdmittedWidth>::Nat as Nat>::VAL == 33);
impl AdmittedWidth for Width<34> {
    type Nat = Pz<O<I<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<34> as AdmittedWidth>::Nat as Nat>::VAL == 34);
impl AdmittedWidth for Width<35> {
    type Nat = Pz<I<I<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<35> as AdmittedWidth>::Nat as Nat>::VAL == 35);
impl AdmittedWidth for Width<36> {
    type Nat = Pz<O<O<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<36> as AdmittedWidth>::Nat as Nat>::VAL == 36);
impl AdmittedWidth for Width<37> {
    type Nat = Pz<I<O<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<37> as AdmittedWidth>::Nat as Nat>::VAL == 37);
impl AdmittedWidth for Width<38> {
    type Nat = Pz<O<I<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<38> as AdmittedWidth>::Nat as Nat>::VAL == 38);
impl AdmittedWidth for Width<39> {
    type Nat = Pz<I<I<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Width<39> as AdmittedWidth>::Nat as Nat>::VAL == 39);
impl AdmittedWidth for Width<40> {
    type Nat = Pz<O<O<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<40> as AdmittedWidth>::Nat as Nat>::VAL == 40);
impl AdmittedWidth for Width<41> {
    type Nat = Pz<I<O<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<41> as AdmittedWidth>::Nat as Nat>::VAL == 41);
impl AdmittedWidth for Width<42> {
    type Nat = Pz<O<I<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<42> as AdmittedWidth>::Nat as Nat>::VAL == 42);
impl AdmittedWidth for Width<43> {
    type Nat = Pz<I<I<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<43> as AdmittedWidth>::Nat as Nat>::VAL == 43);
impl AdmittedWidth for Width<44> {
    type Nat = Pz<O<O<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<44> as AdmittedWidth>::Nat as Nat>::VAL == 44);
impl AdmittedWidth for Width<45> {
    type Nat = Pz<I<O<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<45> as AdmittedWidth>::Nat as Nat>::VAL == 45);
impl AdmittedWidth for Width<46> {
    type Nat = Pz<O<I<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<46> as AdmittedWidth>::Nat as Nat>::VAL == 46);
impl AdmittedWidth for Width<47> {
    type Nat = Pz<I<I<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Width<47> as AdmittedWidth>::Nat as Nat>::VAL == 47);
impl AdmittedWidth for Width<48> {
    type Nat = Pz<O<O<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<48> as AdmittedWidth>::Nat as Nat>::VAL == 48);
impl AdmittedWidth for Width<49> {
    type Nat = Pz<I<O<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<49> as AdmittedWidth>::Nat as Nat>::VAL == 49);
impl AdmittedWidth for Width<50> {
    type Nat = Pz<O<I<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<50> as AdmittedWidth>::Nat as Nat>::VAL == 50);
impl AdmittedWidth for Width<51> {
    type Nat = Pz<I<I<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<51> as AdmittedWidth>::Nat as Nat>::VAL == 51);
impl AdmittedWidth for Width<52> {
    type Nat = Pz<O<O<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<52> as AdmittedWidth>::Nat as Nat>::VAL == 52);
impl AdmittedWidth for Width<53> {
    type Nat = Pz<I<O<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<53> as AdmittedWidth>::Nat as Nat>::VAL == 53);
impl AdmittedWidth for Width<54> {
    type Nat = Pz<O<I<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<54> as AdmittedWidth>::Nat as Nat>::VAL == 54);
impl AdmittedWidth for Width<55> {
    type Nat = Pz<I<I<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Width<55> as AdmittedWidth>::Nat as Nat>::VAL == 55);
impl AdmittedWidth for Width<56> {
    type Nat = Pz<O<O<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<56> as AdmittedWidth>::Nat as Nat>::VAL == 56);
impl AdmittedWidth for Width<57> {
    type Nat = Pz<I<O<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<57> as AdmittedWidth>::Nat as Nat>::VAL == 57);
impl AdmittedWidth for Width<58> {
    type Nat = Pz<O<I<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<58> as AdmittedWidth>::Nat as Nat>::VAL == 58);
impl AdmittedWidth for Width<59> {
    type Nat = Pz<I<I<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<59> as AdmittedWidth>::Nat as Nat>::VAL == 59);
impl AdmittedWidth for Width<60> {
    type Nat = Pz<O<O<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<60> as AdmittedWidth>::Nat as Nat>::VAL == 60);
impl AdmittedWidth for Width<61> {
    type Nat = Pz<I<O<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<61> as AdmittedWidth>::Nat as Nat>::VAL == 61);
impl AdmittedWidth for Width<62> {
    type Nat = Pz<O<I<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<62> as AdmittedWidth>::Nat as Nat>::VAL == 62);
impl AdmittedWidth for Width<63> {
    type Nat = Pz<I<I<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Width<63> as AdmittedWidth>::Nat as Nat>::VAL == 63);
impl AdmittedPrecision for Z {}
impl AdmittedPrecision for Pz<H> {}
impl AdmittedPrecision for Pz<O<H>> {}
impl AdmittedPrecision for Pz<I<H>> {}
impl AdmittedPrecision for Pz<O<O<H>>> {}
impl AdmittedPrecision for Pz<I<O<H>>> {}
impl AdmittedPrecision for Pz<O<I<H>>> {}
impl AdmittedPrecision for Pz<I<I<H>>> {}
impl AdmittedPrecision for Pz<O<O<O<H>>>> {}
impl AdmittedPrecision for Pz<I<O<O<H>>>> {}
impl AdmittedPrecision for Pz<O<I<O<H>>>> {}
impl AdmittedPrecision for Pz<I<I<O<H>>>> {}
impl AdmittedPrecision for Pz<O<O<I<H>>>> {}
impl AdmittedPrecision for Pz<I<O<I<H>>>> {}
impl AdmittedPrecision for Pz<O<I<I<H>>>> {}
impl AdmittedPrecision for Pz<I<I<I<H>>>> {}
impl AdmittedPrecision for Pz<O<O<O<O<H>>>>> {}
impl AdmittedPrecision for Pz<I<O<O<O<H>>>>> {}
impl AdmittedPrecision for Pz<O<I<O<O<H>>>>> {}
impl AdmittedPrecision for Pz<I<I<O<O<H>>>>> {}
impl AdmittedPrecision for Pz<O<O<I<O<H>>>>> {}
impl AdmittedPrecision for Pz<I<O<I<O<H>>>>> {}
impl AdmittedPrecision for Pz<O<I<I<O<H>>>>> {}
impl AdmittedPrecision for Pz<I<I<I<O<H>>>>> {}
impl AdmittedPrecision for Pz<O<O<O<I<H>>>>> {}
impl AdmittedPrecision for Pz<I<O<O<I<H>>>>> {}
impl AdmittedPrecision for Pz<O<I<O<I<H>>>>> {}
impl AdmittedPrecision for Pz<I<I<O<I<H>>>>> {}
impl AdmittedPrecision for Pz<O<O<I<I<H>>>>> {}
impl AdmittedPrecision for Pz<I<O<I<I<H>>>>> {}
impl AdmittedPrecision for Pz<O<I<I<I<H>>>>> {}
impl AdmittedPrecision for Pz<I<I<I<I<H>>>>> {}
impl AdmittedPrecision for Pz<O<O<O<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<O<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<O<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<O<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<I<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<I<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<I<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<I<O<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<O<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<O<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<O<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<O<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<I<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<I<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<I<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<I<I<O<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<O<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<O<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<O<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<O<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<I<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<I<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<I<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<I<O<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<O<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<O<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<O<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<O<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<O<I<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<O<I<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<O<I<I<I<I<H>>>>>> {}
impl AdmittedPrecision for Pz<I<I<I<I<I<H>>>>>> {}

// ======================= the identity contract =========================
pub trait Exponent: Sealed {}
pub struct EZero;
impl Sealed for EZero {}
impl Exponent for EZero {}
pub trait Adjustment: Sealed {}
pub struct Unit;
impl Sealed for Unit {}
impl Adjustment for Unit {}
pub trait BiasT: Sealed {}
pub struct BZero;
impl Sealed for BZero {}
impl BiasT for BZero {}
pub trait Underflow: Sealed {}
pub struct Gradual;
impl Sealed for Gradual {}
impl Underflow for Gradual {}
pub trait Specials: Sealed {}
pub struct NoSpecials;
impl Sealed for NoSpecials {}
impl Specials for NoSpecials {}

pub trait ExponentForm: Sealed {}
pub struct Implicit<E: Exponent, A: Adjustment, B: BiasT>(PhantomData<(E, A, B)>);
pub struct Ranged<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>(
    PhantomData<(EMIN, EMAX, U, S)>,
);
impl<E: Exponent, A: Adjustment, B: BiasT> Sealed for Implicit<E, A, B> {}
impl<E: Exponent, A: Adjustment, B: BiasT> ExponentForm for Implicit<E, A, B> {}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> Sealed
    for Ranged<EMIN, EMAX, U, S>
{
}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
}

pub trait Numeral {
    type Precision: Nat;
    type Exponent: ExponentForm;
}
pub type FormOf<N> = <N as Numeral>::Exponent;

/// The fixed-point numeral a UFixed alias builds: precision I + F, one
/// implicit exponent.
pub struct FixedNumeral<P: AdmittedPrecision>(PhantomData<P>);
impl<P: AdmittedPrecision> Numeral for FixedNumeral<P> {
    type Precision = P;
    type Exponent = Implicit<EZero, Unit, BZero>;
}
/// The float numeral a StrictFloat alias builds.
pub struct FloatNumeral<P: Nat>(PhantomData<P>);
impl<P: Nat> Numeral for FloatNumeral<P> {
    type Precision = P;
    type Exponent = Ranged<EZero, EZero, Gradual, NoSpecials>;
}

// ======================= the preset key ================================
pub trait StoredWidth: Sealed {}
pub struct Minimum;
pub struct DoubleLogical;
impl Sealed for Minimum {}
impl StoredWidth for Minimum {}
impl Sealed for DoubleLogical {}
impl StoredWidth for DoubleLogical {}

pub trait StorageLayout: Sealed {}
pub struct Dense;
pub struct Bitpacked;
impl Sealed for Dense {}
impl StorageLayout for Dense {}
impl Sealed for Bitpacked {}
impl StorageLayout for Bitpacked {}

pub trait LoweringDoor: Sealed {}
pub struct Inert;
pub struct Quantised;
pub struct HostFloat;
impl Sealed for Inert {}
impl LoweringDoor for Inert {}
impl Sealed for Quantised {}
impl LoweringDoor for Quantised {}
impl Sealed for HostFloat {}
impl LoweringDoor for HostFloat {}

pub trait Resolution {}
pub trait Direction: Resolution {}
pub struct TowardNegative;
pub struct ToEven;
pub struct TowardPositive;
pub struct ReduceModulo;
pub struct Refuse;
impl Resolution for TowardNegative {}
impl Direction for TowardNegative {}
impl Resolution for ToEven {}
impl Direction for ToEven {}
impl Resolution for TowardPositive {}
impl Direction for TowardPositive {}
impl Resolution for ReduceModulo {}
impl Resolution for Refuse {}
/// The far point, section 1.16's rule, standing in for the float rows.
pub struct FarPoint;
impl Resolution for FarPoint {}

pub trait Quantisation {
    type UnderMidpoint: Direction;
    type OnMidpoint: Direction;
    type OverMidpoint: Direction;
    type OverRange: Resolution;
    type UnderRange: Resolution;
}

/// Both contracts key on the exponent form and quantify over its
/// parameters, so a row finer than the form is refused by coherence.
pub trait Policy<F: ExponentForm> {
    type Quantisation: Quantisation;
}
pub trait Lowering<F: ExponentForm> {
    type StoredWidth: StoredWidth;
    type Layout: StorageLayout;
    type Door: LoweringDoor;
}

pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

pub struct Q<U, M, V, OR, UR>(PhantomData<(U, M, V, OR, UR)>);
impl<U: Direction, M: Direction, V: Direction, OR: Resolution, UR: Resolution> Quantisation
    for Q<U, M, V, OR, UR>
{
    type UnderMidpoint = U;
    type OnMidpoint = M;
    type OverMidpoint = V;
    type OverRange = OR;
    type UnderRange = UR;
}

// --- section 1.21's fixed-point table, cell for cell -------------------
macro_rules! fixed_row {
    ($preset:ty, $dir:ty, $over:ty, $under:ty, $sw:ty, $lay:ty) => {
        impl<E: Exponent, A: Adjustment, B: BiasT> Policy<Implicit<E, A, B>> for $preset {
            type Quantisation = Q<$dir, $dir, $dir, $over, $under>;
        }
        impl<E: Exponent, A: Adjustment, B: BiasT> Lowering<Implicit<E, A, B>> for $preset {
            type StoredWidth = $sw;
            type Layout = $lay;
            type Door = Inert;
        }
    };
}
fixed_row!(
    Hot,
    TowardNegative,
    ReduceModulo,
    ReduceModulo,
    Minimum,
    Dense
);
fixed_row!(
    Cold,
    ToEven,
    TowardNegative,
    TowardPositive,
    Minimum,
    Bitpacked
);
fixed_row!(
    Warm,
    ToEven,
    TowardNegative,
    TowardPositive,
    DoubleLogical,
    Dense
);
fixed_row!(Precise, ToEven, Refuse, Refuse, DoubleLogical, Dense);

// --- section 1.21's float table, cell for cell -------------------------
macro_rules! float_row {
    ($preset:ty, $over:ty, $under:ty, $sw:ty, $lay:ty, $door:ty) => {
        impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>
            Policy<Ranged<EMIN, EMAX, U, S>> for $preset
        {
            type Quantisation = Q<ToEven, ToEven, ToEven, $over, $under>;
        }
        impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>
            Lowering<Ranged<EMIN, EMAX, U, S>> for $preset
        {
            type StoredWidth = $sw;
            type Layout = $lay;
            type Door = $door;
        }
    };
}
float_row!(Hot, FarPoint, FarPoint, Minimum, Dense, HostFloat);
float_row!(Cold, FarPoint, FarPoint, Minimum, Bitpacked, Quantised);
float_row!(Warm, FarPoint, FarPoint, Minimum, Dense, HostFloat);
float_row!(Precise, Refuse, Refuse, DoubleLogical, Dense, Quantised);

// ======================= the composed type =============================
pub struct Number<N: Numeral, S: Policy<FormOf<N>> + Lowering<FormOf<N>>>(PhantomData<(N, S)>);

/// The public spelling D48 and D31 require, unchanged.
pub type UFixed<const INT: u16, const FRAC: u16, S> = Number<FixedNumeral<SumOf<INT, FRAC>>, S>;
pub type StrictFloat<const PREC: u16, S> = Number<FloatNumeral<NatOf<PREC>>, S>;

// ======================= what the whole thing claims ===================
const _: () = assert!(<<FixedNumeral<SumOf<13, 3>> as Numeral>::Precision as Nat>::VAL == 16);

pub fn what_a_consumer_gets() {
    // One preset name, two rows, selected by the numeral rather than chosen.
    let _: <Warm as Lowering<FormOf<FixedNumeral<SumOf<13, 3>>>>>::StoredWidth = DoubleLogical;
    let _: <Warm as Lowering<FormOf<FloatNumeral<NatOf<24>>>>>::StoredWidth = Minimum;
    let _: <Hot as Lowering<FormOf<FixedNumeral<SumOf<13, 3>>>>>::Door = Inert;
    let _: <Hot as Lowering<FormOf<FloatNumeral<NatOf<24>>>>>::Door = HostFloat;
    // and the composed public spelling resolves
    let _: Option<UFixed<13, 3, Warm>> = None;
    let _: Option<StrictFloat<24, Warm>> = None;
}

pub fn over_ceiling(_x: UFixed<40, 30, Warm>) {}
