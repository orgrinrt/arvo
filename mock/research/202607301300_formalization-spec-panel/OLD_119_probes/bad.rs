#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

// ---- Pos ::= H | O<P> | I<P> ------------------------------------------
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

// ---- Pos addition, the carry chain ------------------------------------
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

// ---- Nat ::= Z | Pz<P> ------------------------------------------------
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

pub trait Precision: Nat {}
impl<T: Nat> Precision for T {}

// ---- the bridge: one impl per admitted width --------------------------
pub struct Idx<const N: u16>;
pub trait NatIndex {
    type Out: Nat;
}
pub type NatOf<const N: u16> = <Idx<N> as NatIndex>::Out;
pub type PrecisionOf<const A: u16, const B: u16> = <NatOf<{ A }> as NatAdd<NatOf<{ B }>>>::Out;

impl NatIndex for Idx<0> {
    type Out = Z;
}
const _: () = assert!(<<Idx<0> as NatIndex>::Out as Nat>::VAL == 0);
impl NatIndex for Idx<1> {
    type Out = Pz<H>;
}
const _: () = assert!(<<Idx<1> as NatIndex>::Out as Nat>::VAL == 1);
impl NatIndex for Idx<2> {
    type Out = Pz<O<H>>;
}
const _: () = assert!(<<Idx<2> as NatIndex>::Out as Nat>::VAL == 2);
impl NatIndex for Idx<3> {
    type Out = Pz<I<H>>;
}
const _: () = assert!(<<Idx<3> as NatIndex>::Out as Nat>::VAL == 3);
impl NatIndex for Idx<4> {
    type Out = Pz<O<O<H>>>;
}
const _: () = assert!(<<Idx<4> as NatIndex>::Out as Nat>::VAL == 4);
impl NatIndex for Idx<5> {
    type Out = Pz<I<O<H>>>;
}
const _: () = assert!(<<Idx<5> as NatIndex>::Out as Nat>::VAL == 5);
impl NatIndex for Idx<6> {
    type Out = Pz<O<I<H>>>;
}
const _: () = assert!(<<Idx<6> as NatIndex>::Out as Nat>::VAL == 6);
impl NatIndex for Idx<7> {
    type Out = Pz<I<I<H>>>;
}
const _: () = assert!(<<Idx<7> as NatIndex>::Out as Nat>::VAL == 7);
impl NatIndex for Idx<8> {
    type Out = Pz<O<O<O<H>>>>;
}
const _: () = assert!(<<Idx<8> as NatIndex>::Out as Nat>::VAL == 8);
impl NatIndex for Idx<9> {
    type Out = Pz<I<O<O<H>>>>;
}
const _: () = assert!(<<Idx<9> as NatIndex>::Out as Nat>::VAL == 9);
impl NatIndex for Idx<10> {
    type Out = Pz<O<I<O<H>>>>;
}
const _: () = assert!(<<Idx<10> as NatIndex>::Out as Nat>::VAL == 10);
impl NatIndex for Idx<11> {
    type Out = Pz<I<I<O<H>>>>;
}
const _: () = assert!(<<Idx<11> as NatIndex>::Out as Nat>::VAL == 11);
impl NatIndex for Idx<12> {
    type Out = Pz<O<O<I<H>>>>;
}
const _: () = assert!(<<Idx<12> as NatIndex>::Out as Nat>::VAL == 12);
impl NatIndex for Idx<13> {
    type Out = Pz<O<I<I<H>>>>;
}
const _: () = assert!(<<Idx<13> as NatIndex>::Out as Nat>::VAL == 13);
impl NatIndex for Idx<14> {
    type Out = Pz<O<I<I<H>>>>;
}
const _: () = assert!(<<Idx<14> as NatIndex>::Out as Nat>::VAL == 14);
impl NatIndex for Idx<15> {
    type Out = Pz<I<I<I<H>>>>;
}
const _: () = assert!(<<Idx<15> as NatIndex>::Out as Nat>::VAL == 15);
impl NatIndex for Idx<16> {
    type Out = Pz<O<O<O<O<H>>>>>;
}
const _: () = assert!(<<Idx<16> as NatIndex>::Out as Nat>::VAL == 16);
impl NatIndex for Idx<17> {
    type Out = Pz<I<O<O<O<H>>>>>;
}
const _: () = assert!(<<Idx<17> as NatIndex>::Out as Nat>::VAL == 17);
impl NatIndex for Idx<18> {
    type Out = Pz<O<I<O<O<H>>>>>;
}
const _: () = assert!(<<Idx<18> as NatIndex>::Out as Nat>::VAL == 18);
impl NatIndex for Idx<19> {
    type Out = Pz<I<I<O<O<H>>>>>;
}
const _: () = assert!(<<Idx<19> as NatIndex>::Out as Nat>::VAL == 19);
impl NatIndex for Idx<20> {
    type Out = Pz<O<O<I<O<H>>>>>;
}
const _: () = assert!(<<Idx<20> as NatIndex>::Out as Nat>::VAL == 20);
impl NatIndex for Idx<21> {
    type Out = Pz<I<O<I<O<H>>>>>;
}
const _: () = assert!(<<Idx<21> as NatIndex>::Out as Nat>::VAL == 21);
impl NatIndex for Idx<22> {
    type Out = Pz<O<I<I<O<H>>>>>;
}
const _: () = assert!(<<Idx<22> as NatIndex>::Out as Nat>::VAL == 22);
impl NatIndex for Idx<23> {
    type Out = Pz<I<I<I<O<H>>>>>;
}
const _: () = assert!(<<Idx<23> as NatIndex>::Out as Nat>::VAL == 23);
impl NatIndex for Idx<24> {
    type Out = Pz<O<O<O<I<H>>>>>;
}
const _: () = assert!(<<Idx<24> as NatIndex>::Out as Nat>::VAL == 24);
impl NatIndex for Idx<25> {
    type Out = Pz<I<O<O<I<H>>>>>;
}
const _: () = assert!(<<Idx<25> as NatIndex>::Out as Nat>::VAL == 25);
impl NatIndex for Idx<26> {
    type Out = Pz<O<I<O<I<H>>>>>;
}
const _: () = assert!(<<Idx<26> as NatIndex>::Out as Nat>::VAL == 26);
impl NatIndex for Idx<27> {
    type Out = Pz<I<I<O<I<H>>>>>;
}
const _: () = assert!(<<Idx<27> as NatIndex>::Out as Nat>::VAL == 27);
impl NatIndex for Idx<28> {
    type Out = Pz<O<O<I<I<H>>>>>;
}
const _: () = assert!(<<Idx<28> as NatIndex>::Out as Nat>::VAL == 28);
impl NatIndex for Idx<29> {
    type Out = Pz<I<O<I<I<H>>>>>;
}
const _: () = assert!(<<Idx<29> as NatIndex>::Out as Nat>::VAL == 29);
impl NatIndex for Idx<30> {
    type Out = Pz<O<I<I<I<H>>>>>;
}
const _: () = assert!(<<Idx<30> as NatIndex>::Out as Nat>::VAL == 30);
impl NatIndex for Idx<31> {
    type Out = Pz<I<I<I<I<H>>>>>;
}
const _: () = assert!(<<Idx<31> as NatIndex>::Out as Nat>::VAL == 31);
impl NatIndex for Idx<32> {
    type Out = Pz<O<O<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<32> as NatIndex>::Out as Nat>::VAL == 32);
impl NatIndex for Idx<33> {
    type Out = Pz<I<O<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<33> as NatIndex>::Out as Nat>::VAL == 33);
impl NatIndex for Idx<34> {
    type Out = Pz<O<I<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<34> as NatIndex>::Out as Nat>::VAL == 34);
impl NatIndex for Idx<35> {
    type Out = Pz<I<I<O<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<35> as NatIndex>::Out as Nat>::VAL == 35);
impl NatIndex for Idx<36> {
    type Out = Pz<O<O<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<36> as NatIndex>::Out as Nat>::VAL == 36);
impl NatIndex for Idx<37> {
    type Out = Pz<I<O<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<37> as NatIndex>::Out as Nat>::VAL == 37);
impl NatIndex for Idx<38> {
    type Out = Pz<O<I<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<38> as NatIndex>::Out as Nat>::VAL == 38);
impl NatIndex for Idx<39> {
    type Out = Pz<I<I<I<O<O<H>>>>>>;
}
const _: () = assert!(<<Idx<39> as NatIndex>::Out as Nat>::VAL == 39);
impl NatIndex for Idx<40> {
    type Out = Pz<O<O<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<40> as NatIndex>::Out as Nat>::VAL == 40);
impl NatIndex for Idx<41> {
    type Out = Pz<I<O<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<41> as NatIndex>::Out as Nat>::VAL == 41);
impl NatIndex for Idx<42> {
    type Out = Pz<O<I<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<42> as NatIndex>::Out as Nat>::VAL == 42);
impl NatIndex for Idx<43> {
    type Out = Pz<I<I<O<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<43> as NatIndex>::Out as Nat>::VAL == 43);
impl NatIndex for Idx<44> {
    type Out = Pz<O<O<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<44> as NatIndex>::Out as Nat>::VAL == 44);
impl NatIndex for Idx<45> {
    type Out = Pz<I<O<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<45> as NatIndex>::Out as Nat>::VAL == 45);
impl NatIndex for Idx<46> {
    type Out = Pz<O<I<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<46> as NatIndex>::Out as Nat>::VAL == 46);
impl NatIndex for Idx<47> {
    type Out = Pz<I<I<I<I<O<H>>>>>>;
}
const _: () = assert!(<<Idx<47> as NatIndex>::Out as Nat>::VAL == 47);
impl NatIndex for Idx<48> {
    type Out = Pz<O<O<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<48> as NatIndex>::Out as Nat>::VAL == 48);
impl NatIndex for Idx<49> {
    type Out = Pz<I<O<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<49> as NatIndex>::Out as Nat>::VAL == 49);
impl NatIndex for Idx<50> {
    type Out = Pz<O<I<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<50> as NatIndex>::Out as Nat>::VAL == 50);
impl NatIndex for Idx<51> {
    type Out = Pz<I<I<O<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<51> as NatIndex>::Out as Nat>::VAL == 51);
impl NatIndex for Idx<52> {
    type Out = Pz<O<O<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<52> as NatIndex>::Out as Nat>::VAL == 52);
impl NatIndex for Idx<53> {
    type Out = Pz<I<O<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<53> as NatIndex>::Out as Nat>::VAL == 53);
impl NatIndex for Idx<54> {
    type Out = Pz<O<I<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<54> as NatIndex>::Out as Nat>::VAL == 54);
impl NatIndex for Idx<55> {
    type Out = Pz<I<I<I<O<I<H>>>>>>;
}
const _: () = assert!(<<Idx<55> as NatIndex>::Out as Nat>::VAL == 55);
impl NatIndex for Idx<56> {
    type Out = Pz<O<O<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<56> as NatIndex>::Out as Nat>::VAL == 56);
impl NatIndex for Idx<57> {
    type Out = Pz<I<O<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<57> as NatIndex>::Out as Nat>::VAL == 57);
impl NatIndex for Idx<58> {
    type Out = Pz<O<I<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<58> as NatIndex>::Out as Nat>::VAL == 58);
impl NatIndex for Idx<59> {
    type Out = Pz<I<I<O<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<59> as NatIndex>::Out as Nat>::VAL == 59);
impl NatIndex for Idx<60> {
    type Out = Pz<O<O<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<60> as NatIndex>::Out as Nat>::VAL == 60);
impl NatIndex for Idx<61> {
    type Out = Pz<I<O<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<61> as NatIndex>::Out as Nat>::VAL == 61);
impl NatIndex for Idx<62> {
    type Out = Pz<O<I<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<62> as NatIndex>::Out as Nat>::VAL == 62);
impl NatIndex for Idx<63> {
    type Out = Pz<I<I<I<I<I<H>>>>>>;
}
const _: () = assert!(<<Idx<63> as NatIndex>::Out as Nat>::VAL == 63);
impl NatIndex for Idx<64> {
    type Out = Pz<O<O<O<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<64> as NatIndex>::Out as Nat>::VAL == 64);
impl NatIndex for Idx<65> {
    type Out = Pz<I<O<O<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<65> as NatIndex>::Out as Nat>::VAL == 65);
impl NatIndex for Idx<66> {
    type Out = Pz<O<I<O<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<66> as NatIndex>::Out as Nat>::VAL == 66);
impl NatIndex for Idx<67> {
    type Out = Pz<I<I<O<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<67> as NatIndex>::Out as Nat>::VAL == 67);
impl NatIndex for Idx<68> {
    type Out = Pz<O<O<I<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<68> as NatIndex>::Out as Nat>::VAL == 68);
impl NatIndex for Idx<69> {
    type Out = Pz<I<O<I<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<69> as NatIndex>::Out as Nat>::VAL == 69);
impl NatIndex for Idx<70> {
    type Out = Pz<O<I<I<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<70> as NatIndex>::Out as Nat>::VAL == 70);
impl NatIndex for Idx<71> {
    type Out = Pz<I<I<I<O<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<71> as NatIndex>::Out as Nat>::VAL == 71);
impl NatIndex for Idx<72> {
    type Out = Pz<O<O<O<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<72> as NatIndex>::Out as Nat>::VAL == 72);
impl NatIndex for Idx<73> {
    type Out = Pz<I<O<O<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<73> as NatIndex>::Out as Nat>::VAL == 73);
impl NatIndex for Idx<74> {
    type Out = Pz<O<I<O<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<74> as NatIndex>::Out as Nat>::VAL == 74);
impl NatIndex for Idx<75> {
    type Out = Pz<I<I<O<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<75> as NatIndex>::Out as Nat>::VAL == 75);
impl NatIndex for Idx<76> {
    type Out = Pz<O<O<I<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<76> as NatIndex>::Out as Nat>::VAL == 76);
impl NatIndex for Idx<77> {
    type Out = Pz<I<O<I<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<77> as NatIndex>::Out as Nat>::VAL == 77);
impl NatIndex for Idx<78> {
    type Out = Pz<O<I<I<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<78> as NatIndex>::Out as Nat>::VAL == 78);
impl NatIndex for Idx<79> {
    type Out = Pz<I<I<I<I<O<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<79> as NatIndex>::Out as Nat>::VAL == 79);
impl NatIndex for Idx<80> {
    type Out = Pz<O<O<O<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<80> as NatIndex>::Out as Nat>::VAL == 80);
impl NatIndex for Idx<81> {
    type Out = Pz<I<O<O<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<81> as NatIndex>::Out as Nat>::VAL == 81);
impl NatIndex for Idx<82> {
    type Out = Pz<O<I<O<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<82> as NatIndex>::Out as Nat>::VAL == 82);
impl NatIndex for Idx<83> {
    type Out = Pz<I<I<O<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<83> as NatIndex>::Out as Nat>::VAL == 83);
impl NatIndex for Idx<84> {
    type Out = Pz<O<O<I<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<84> as NatIndex>::Out as Nat>::VAL == 84);
impl NatIndex for Idx<85> {
    type Out = Pz<I<O<I<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<85> as NatIndex>::Out as Nat>::VAL == 85);
impl NatIndex for Idx<86> {
    type Out = Pz<O<I<I<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<86> as NatIndex>::Out as Nat>::VAL == 86);
impl NatIndex for Idx<87> {
    type Out = Pz<I<I<I<O<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<87> as NatIndex>::Out as Nat>::VAL == 87);
impl NatIndex for Idx<88> {
    type Out = Pz<O<O<O<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<88> as NatIndex>::Out as Nat>::VAL == 88);
impl NatIndex for Idx<89> {
    type Out = Pz<I<O<O<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<89> as NatIndex>::Out as Nat>::VAL == 89);
impl NatIndex for Idx<90> {
    type Out = Pz<O<I<O<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<90> as NatIndex>::Out as Nat>::VAL == 90);
impl NatIndex for Idx<91> {
    type Out = Pz<I<I<O<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<91> as NatIndex>::Out as Nat>::VAL == 91);
impl NatIndex for Idx<92> {
    type Out = Pz<O<O<I<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<92> as NatIndex>::Out as Nat>::VAL == 92);
impl NatIndex for Idx<93> {
    type Out = Pz<I<O<I<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<93> as NatIndex>::Out as Nat>::VAL == 93);
impl NatIndex for Idx<94> {
    type Out = Pz<O<I<I<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<94> as NatIndex>::Out as Nat>::VAL == 94);
impl NatIndex for Idx<95> {
    type Out = Pz<I<I<I<I<I<O<H>>>>>>>;
}
const _: () = assert!(<<Idx<95> as NatIndex>::Out as Nat>::VAL == 95);
impl NatIndex for Idx<96> {
    type Out = Pz<O<O<O<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<96> as NatIndex>::Out as Nat>::VAL == 96);
impl NatIndex for Idx<97> {
    type Out = Pz<I<O<O<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<97> as NatIndex>::Out as Nat>::VAL == 97);
impl NatIndex for Idx<98> {
    type Out = Pz<O<I<O<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<98> as NatIndex>::Out as Nat>::VAL == 98);
impl NatIndex for Idx<99> {
    type Out = Pz<I<I<O<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<99> as NatIndex>::Out as Nat>::VAL == 99);
impl NatIndex for Idx<100> {
    type Out = Pz<O<O<I<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<100> as NatIndex>::Out as Nat>::VAL == 100);
impl NatIndex for Idx<101> {
    type Out = Pz<I<O<I<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<101> as NatIndex>::Out as Nat>::VAL == 101);
impl NatIndex for Idx<102> {
    type Out = Pz<O<I<I<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<102> as NatIndex>::Out as Nat>::VAL == 102);
impl NatIndex for Idx<103> {
    type Out = Pz<I<I<I<O<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<103> as NatIndex>::Out as Nat>::VAL == 103);
impl NatIndex for Idx<104> {
    type Out = Pz<O<O<O<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<104> as NatIndex>::Out as Nat>::VAL == 104);
impl NatIndex for Idx<105> {
    type Out = Pz<I<O<O<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<105> as NatIndex>::Out as Nat>::VAL == 105);
impl NatIndex for Idx<106> {
    type Out = Pz<O<I<O<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<106> as NatIndex>::Out as Nat>::VAL == 106);
impl NatIndex for Idx<107> {
    type Out = Pz<I<I<O<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<107> as NatIndex>::Out as Nat>::VAL == 107);
impl NatIndex for Idx<108> {
    type Out = Pz<O<O<I<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<108> as NatIndex>::Out as Nat>::VAL == 108);
impl NatIndex for Idx<109> {
    type Out = Pz<I<O<I<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<109> as NatIndex>::Out as Nat>::VAL == 109);
impl NatIndex for Idx<110> {
    type Out = Pz<O<I<I<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<110> as NatIndex>::Out as Nat>::VAL == 110);
impl NatIndex for Idx<111> {
    type Out = Pz<I<I<I<I<O<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<111> as NatIndex>::Out as Nat>::VAL == 111);
impl NatIndex for Idx<112> {
    type Out = Pz<O<O<O<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<112> as NatIndex>::Out as Nat>::VAL == 112);
impl NatIndex for Idx<113> {
    type Out = Pz<I<O<O<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<113> as NatIndex>::Out as Nat>::VAL == 113);
impl NatIndex for Idx<114> {
    type Out = Pz<O<I<O<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<114> as NatIndex>::Out as Nat>::VAL == 114);
impl NatIndex for Idx<115> {
    type Out = Pz<I<I<O<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<115> as NatIndex>::Out as Nat>::VAL == 115);
impl NatIndex for Idx<116> {
    type Out = Pz<O<O<I<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<116> as NatIndex>::Out as Nat>::VAL == 116);
impl NatIndex for Idx<117> {
    type Out = Pz<I<O<I<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<117> as NatIndex>::Out as Nat>::VAL == 117);
impl NatIndex for Idx<118> {
    type Out = Pz<O<I<I<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<118> as NatIndex>::Out as Nat>::VAL == 118);
impl NatIndex for Idx<119> {
    type Out = Pz<I<I<I<O<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<119> as NatIndex>::Out as Nat>::VAL == 119);
impl NatIndex for Idx<120> {
    type Out = Pz<O<O<O<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<120> as NatIndex>::Out as Nat>::VAL == 120);
impl NatIndex for Idx<121> {
    type Out = Pz<I<O<O<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<121> as NatIndex>::Out as Nat>::VAL == 121);
impl NatIndex for Idx<122> {
    type Out = Pz<O<I<O<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<122> as NatIndex>::Out as Nat>::VAL == 122);
impl NatIndex for Idx<123> {
    type Out = Pz<I<I<O<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<123> as NatIndex>::Out as Nat>::VAL == 123);
impl NatIndex for Idx<124> {
    type Out = Pz<O<O<I<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<124> as NatIndex>::Out as Nat>::VAL == 124);
impl NatIndex for Idx<125> {
    type Out = Pz<I<O<I<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<125> as NatIndex>::Out as Nat>::VAL == 125);
impl NatIndex for Idx<126> {
    type Out = Pz<O<I<I<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<126> as NatIndex>::Out as Nat>::VAL == 126);
impl NatIndex for Idx<127> {
    type Out = Pz<I<I<I<I<I<I<H>>>>>>>;
}
const _: () = assert!(<<Idx<127> as NatIndex>::Out as Nat>::VAL == 127);
impl NatIndex for Idx<128> {
    type Out = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<128> as NatIndex>::Out as Nat>::VAL == 128);
impl NatIndex for Idx<129> {
    type Out = Pz<I<O<O<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<129> as NatIndex>::Out as Nat>::VAL == 129);
impl NatIndex for Idx<130> {
    type Out = Pz<O<I<O<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<130> as NatIndex>::Out as Nat>::VAL == 130);
impl NatIndex for Idx<131> {
    type Out = Pz<I<I<O<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<131> as NatIndex>::Out as Nat>::VAL == 131);
impl NatIndex for Idx<132> {
    type Out = Pz<O<O<I<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<132> as NatIndex>::Out as Nat>::VAL == 132);
impl NatIndex for Idx<133> {
    type Out = Pz<I<O<I<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<133> as NatIndex>::Out as Nat>::VAL == 133);
impl NatIndex for Idx<134> {
    type Out = Pz<O<I<I<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<134> as NatIndex>::Out as Nat>::VAL == 134);
impl NatIndex for Idx<135> {
    type Out = Pz<I<I<I<O<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<135> as NatIndex>::Out as Nat>::VAL == 135);
impl NatIndex for Idx<136> {
    type Out = Pz<O<O<O<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<136> as NatIndex>::Out as Nat>::VAL == 136);
impl NatIndex for Idx<137> {
    type Out = Pz<I<O<O<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<137> as NatIndex>::Out as Nat>::VAL == 137);
impl NatIndex for Idx<138> {
    type Out = Pz<O<I<O<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<138> as NatIndex>::Out as Nat>::VAL == 138);
impl NatIndex for Idx<139> {
    type Out = Pz<I<I<O<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<139> as NatIndex>::Out as Nat>::VAL == 139);
impl NatIndex for Idx<140> {
    type Out = Pz<O<O<I<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<140> as NatIndex>::Out as Nat>::VAL == 140);
impl NatIndex for Idx<141> {
    type Out = Pz<I<O<I<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<141> as NatIndex>::Out as Nat>::VAL == 141);
impl NatIndex for Idx<142> {
    type Out = Pz<O<I<I<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<142> as NatIndex>::Out as Nat>::VAL == 142);
impl NatIndex for Idx<143> {
    type Out = Pz<I<I<I<I<O<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<143> as NatIndex>::Out as Nat>::VAL == 143);
impl NatIndex for Idx<144> {
    type Out = Pz<O<O<O<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<144> as NatIndex>::Out as Nat>::VAL == 144);
impl NatIndex for Idx<145> {
    type Out = Pz<I<O<O<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<145> as NatIndex>::Out as Nat>::VAL == 145);
impl NatIndex for Idx<146> {
    type Out = Pz<O<I<O<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<146> as NatIndex>::Out as Nat>::VAL == 146);
impl NatIndex for Idx<147> {
    type Out = Pz<I<I<O<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<147> as NatIndex>::Out as Nat>::VAL == 147);
impl NatIndex for Idx<148> {
    type Out = Pz<O<O<I<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<148> as NatIndex>::Out as Nat>::VAL == 148);
impl NatIndex for Idx<149> {
    type Out = Pz<I<O<I<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<149> as NatIndex>::Out as Nat>::VAL == 149);
impl NatIndex for Idx<150> {
    type Out = Pz<O<I<I<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<150> as NatIndex>::Out as Nat>::VAL == 150);
impl NatIndex for Idx<151> {
    type Out = Pz<I<I<I<O<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<151> as NatIndex>::Out as Nat>::VAL == 151);
impl NatIndex for Idx<152> {
    type Out = Pz<O<O<O<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<152> as NatIndex>::Out as Nat>::VAL == 152);
impl NatIndex for Idx<153> {
    type Out = Pz<I<O<O<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<153> as NatIndex>::Out as Nat>::VAL == 153);
impl NatIndex for Idx<154> {
    type Out = Pz<O<I<O<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<154> as NatIndex>::Out as Nat>::VAL == 154);
impl NatIndex for Idx<155> {
    type Out = Pz<I<I<O<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<155> as NatIndex>::Out as Nat>::VAL == 155);
impl NatIndex for Idx<156> {
    type Out = Pz<O<O<I<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<156> as NatIndex>::Out as Nat>::VAL == 156);
impl NatIndex for Idx<157> {
    type Out = Pz<I<O<I<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<157> as NatIndex>::Out as Nat>::VAL == 157);
impl NatIndex for Idx<158> {
    type Out = Pz<O<I<I<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<158> as NatIndex>::Out as Nat>::VAL == 158);
impl NatIndex for Idx<159> {
    type Out = Pz<I<I<I<I<I<O<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<159> as NatIndex>::Out as Nat>::VAL == 159);
impl NatIndex for Idx<160> {
    type Out = Pz<O<O<O<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<160> as NatIndex>::Out as Nat>::VAL == 160);
impl NatIndex for Idx<161> {
    type Out = Pz<I<O<O<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<161> as NatIndex>::Out as Nat>::VAL == 161);
impl NatIndex for Idx<162> {
    type Out = Pz<O<I<O<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<162> as NatIndex>::Out as Nat>::VAL == 162);
impl NatIndex for Idx<163> {
    type Out = Pz<I<I<O<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<163> as NatIndex>::Out as Nat>::VAL == 163);
impl NatIndex for Idx<164> {
    type Out = Pz<O<O<I<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<164> as NatIndex>::Out as Nat>::VAL == 164);
impl NatIndex for Idx<165> {
    type Out = Pz<I<O<I<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<165> as NatIndex>::Out as Nat>::VAL == 165);
impl NatIndex for Idx<166> {
    type Out = Pz<O<I<I<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<166> as NatIndex>::Out as Nat>::VAL == 166);
impl NatIndex for Idx<167> {
    type Out = Pz<I<I<I<O<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<167> as NatIndex>::Out as Nat>::VAL == 167);
impl NatIndex for Idx<168> {
    type Out = Pz<O<O<O<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<168> as NatIndex>::Out as Nat>::VAL == 168);
impl NatIndex for Idx<169> {
    type Out = Pz<I<O<O<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<169> as NatIndex>::Out as Nat>::VAL == 169);
impl NatIndex for Idx<170> {
    type Out = Pz<O<I<O<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<170> as NatIndex>::Out as Nat>::VAL == 170);
impl NatIndex for Idx<171> {
    type Out = Pz<I<I<O<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<171> as NatIndex>::Out as Nat>::VAL == 171);
impl NatIndex for Idx<172> {
    type Out = Pz<O<O<I<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<172> as NatIndex>::Out as Nat>::VAL == 172);
impl NatIndex for Idx<173> {
    type Out = Pz<I<O<I<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<173> as NatIndex>::Out as Nat>::VAL == 173);
impl NatIndex for Idx<174> {
    type Out = Pz<O<I<I<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<174> as NatIndex>::Out as Nat>::VAL == 174);
impl NatIndex for Idx<175> {
    type Out = Pz<I<I<I<I<O<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<175> as NatIndex>::Out as Nat>::VAL == 175);
impl NatIndex for Idx<176> {
    type Out = Pz<O<O<O<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<176> as NatIndex>::Out as Nat>::VAL == 176);
impl NatIndex for Idx<177> {
    type Out = Pz<I<O<O<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<177> as NatIndex>::Out as Nat>::VAL == 177);
impl NatIndex for Idx<178> {
    type Out = Pz<O<I<O<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<178> as NatIndex>::Out as Nat>::VAL == 178);
impl NatIndex for Idx<179> {
    type Out = Pz<I<I<O<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<179> as NatIndex>::Out as Nat>::VAL == 179);
impl NatIndex for Idx<180> {
    type Out = Pz<O<O<I<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<180> as NatIndex>::Out as Nat>::VAL == 180);
impl NatIndex for Idx<181> {
    type Out = Pz<I<O<I<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<181> as NatIndex>::Out as Nat>::VAL == 181);
impl NatIndex for Idx<182> {
    type Out = Pz<O<I<I<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<182> as NatIndex>::Out as Nat>::VAL == 182);
impl NatIndex for Idx<183> {
    type Out = Pz<I<I<I<O<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<183> as NatIndex>::Out as Nat>::VAL == 183);
impl NatIndex for Idx<184> {
    type Out = Pz<O<O<O<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<184> as NatIndex>::Out as Nat>::VAL == 184);
impl NatIndex for Idx<185> {
    type Out = Pz<I<O<O<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<185> as NatIndex>::Out as Nat>::VAL == 185);
impl NatIndex for Idx<186> {
    type Out = Pz<O<I<O<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<186> as NatIndex>::Out as Nat>::VAL == 186);
impl NatIndex for Idx<187> {
    type Out = Pz<I<I<O<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<187> as NatIndex>::Out as Nat>::VAL == 187);
impl NatIndex for Idx<188> {
    type Out = Pz<O<O<I<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<188> as NatIndex>::Out as Nat>::VAL == 188);
impl NatIndex for Idx<189> {
    type Out = Pz<I<O<I<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<189> as NatIndex>::Out as Nat>::VAL == 189);
impl NatIndex for Idx<190> {
    type Out = Pz<O<I<I<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<190> as NatIndex>::Out as Nat>::VAL == 190);
impl NatIndex for Idx<191> {
    type Out = Pz<I<I<I<I<I<I<O<H>>>>>>>>;
}
const _: () = assert!(<<Idx<191> as NatIndex>::Out as Nat>::VAL == 191);
impl NatIndex for Idx<192> {
    type Out = Pz<O<O<O<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<192> as NatIndex>::Out as Nat>::VAL == 192);
impl NatIndex for Idx<193> {
    type Out = Pz<I<O<O<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<193> as NatIndex>::Out as Nat>::VAL == 193);
impl NatIndex for Idx<194> {
    type Out = Pz<O<I<O<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<194> as NatIndex>::Out as Nat>::VAL == 194);
impl NatIndex for Idx<195> {
    type Out = Pz<I<I<O<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<195> as NatIndex>::Out as Nat>::VAL == 195);
impl NatIndex for Idx<196> {
    type Out = Pz<O<O<I<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<196> as NatIndex>::Out as Nat>::VAL == 196);
impl NatIndex for Idx<197> {
    type Out = Pz<I<O<I<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<197> as NatIndex>::Out as Nat>::VAL == 197);
impl NatIndex for Idx<198> {
    type Out = Pz<O<I<I<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<198> as NatIndex>::Out as Nat>::VAL == 198);
impl NatIndex for Idx<199> {
    type Out = Pz<I<I<I<O<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<199> as NatIndex>::Out as Nat>::VAL == 199);
impl NatIndex for Idx<200> {
    type Out = Pz<O<O<O<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<200> as NatIndex>::Out as Nat>::VAL == 200);
impl NatIndex for Idx<201> {
    type Out = Pz<I<O<O<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<201> as NatIndex>::Out as Nat>::VAL == 201);
impl NatIndex for Idx<202> {
    type Out = Pz<O<I<O<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<202> as NatIndex>::Out as Nat>::VAL == 202);
impl NatIndex for Idx<203> {
    type Out = Pz<I<I<O<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<203> as NatIndex>::Out as Nat>::VAL == 203);
impl NatIndex for Idx<204> {
    type Out = Pz<O<O<I<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<204> as NatIndex>::Out as Nat>::VAL == 204);
impl NatIndex for Idx<205> {
    type Out = Pz<I<O<I<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<205> as NatIndex>::Out as Nat>::VAL == 205);
impl NatIndex for Idx<206> {
    type Out = Pz<O<I<I<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<206> as NatIndex>::Out as Nat>::VAL == 206);
impl NatIndex for Idx<207> {
    type Out = Pz<I<I<I<I<O<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<207> as NatIndex>::Out as Nat>::VAL == 207);
impl NatIndex for Idx<208> {
    type Out = Pz<O<O<O<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<208> as NatIndex>::Out as Nat>::VAL == 208);
impl NatIndex for Idx<209> {
    type Out = Pz<I<O<O<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<209> as NatIndex>::Out as Nat>::VAL == 209);
impl NatIndex for Idx<210> {
    type Out = Pz<O<I<O<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<210> as NatIndex>::Out as Nat>::VAL == 210);
impl NatIndex for Idx<211> {
    type Out = Pz<I<I<O<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<211> as NatIndex>::Out as Nat>::VAL == 211);
impl NatIndex for Idx<212> {
    type Out = Pz<O<O<I<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<212> as NatIndex>::Out as Nat>::VAL == 212);
impl NatIndex for Idx<213> {
    type Out = Pz<I<O<I<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<213> as NatIndex>::Out as Nat>::VAL == 213);
impl NatIndex for Idx<214> {
    type Out = Pz<O<I<I<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<214> as NatIndex>::Out as Nat>::VAL == 214);
impl NatIndex for Idx<215> {
    type Out = Pz<I<I<I<O<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<215> as NatIndex>::Out as Nat>::VAL == 215);
impl NatIndex for Idx<216> {
    type Out = Pz<O<O<O<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<216> as NatIndex>::Out as Nat>::VAL == 216);
impl NatIndex for Idx<217> {
    type Out = Pz<I<O<O<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<217> as NatIndex>::Out as Nat>::VAL == 217);
impl NatIndex for Idx<218> {
    type Out = Pz<O<I<O<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<218> as NatIndex>::Out as Nat>::VAL == 218);
impl NatIndex for Idx<219> {
    type Out = Pz<I<I<O<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<219> as NatIndex>::Out as Nat>::VAL == 219);
impl NatIndex for Idx<220> {
    type Out = Pz<O<O<I<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<220> as NatIndex>::Out as Nat>::VAL == 220);
impl NatIndex for Idx<221> {
    type Out = Pz<I<O<I<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<221> as NatIndex>::Out as Nat>::VAL == 221);
impl NatIndex for Idx<222> {
    type Out = Pz<O<I<I<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<222> as NatIndex>::Out as Nat>::VAL == 222);
impl NatIndex for Idx<223> {
    type Out = Pz<I<I<I<I<I<O<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<223> as NatIndex>::Out as Nat>::VAL == 223);
impl NatIndex for Idx<224> {
    type Out = Pz<O<O<O<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<224> as NatIndex>::Out as Nat>::VAL == 224);
impl NatIndex for Idx<225> {
    type Out = Pz<I<O<O<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<225> as NatIndex>::Out as Nat>::VAL == 225);
impl NatIndex for Idx<226> {
    type Out = Pz<O<I<O<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<226> as NatIndex>::Out as Nat>::VAL == 226);
impl NatIndex for Idx<227> {
    type Out = Pz<I<I<O<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<227> as NatIndex>::Out as Nat>::VAL == 227);
impl NatIndex for Idx<228> {
    type Out = Pz<O<O<I<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<228> as NatIndex>::Out as Nat>::VAL == 228);
impl NatIndex for Idx<229> {
    type Out = Pz<I<O<I<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<229> as NatIndex>::Out as Nat>::VAL == 229);
impl NatIndex for Idx<230> {
    type Out = Pz<O<I<I<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<230> as NatIndex>::Out as Nat>::VAL == 230);
impl NatIndex for Idx<231> {
    type Out = Pz<I<I<I<O<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<231> as NatIndex>::Out as Nat>::VAL == 231);
impl NatIndex for Idx<232> {
    type Out = Pz<O<O<O<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<232> as NatIndex>::Out as Nat>::VAL == 232);
impl NatIndex for Idx<233> {
    type Out = Pz<I<O<O<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<233> as NatIndex>::Out as Nat>::VAL == 233);
impl NatIndex for Idx<234> {
    type Out = Pz<O<I<O<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<234> as NatIndex>::Out as Nat>::VAL == 234);
impl NatIndex for Idx<235> {
    type Out = Pz<I<I<O<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<235> as NatIndex>::Out as Nat>::VAL == 235);
impl NatIndex for Idx<236> {
    type Out = Pz<O<O<I<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<236> as NatIndex>::Out as Nat>::VAL == 236);
impl NatIndex for Idx<237> {
    type Out = Pz<I<O<I<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<237> as NatIndex>::Out as Nat>::VAL == 237);
impl NatIndex for Idx<238> {
    type Out = Pz<O<I<I<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<238> as NatIndex>::Out as Nat>::VAL == 238);
impl NatIndex for Idx<239> {
    type Out = Pz<I<I<I<I<O<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<239> as NatIndex>::Out as Nat>::VAL == 239);
impl NatIndex for Idx<240> {
    type Out = Pz<O<O<O<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<240> as NatIndex>::Out as Nat>::VAL == 240);
impl NatIndex for Idx<241> {
    type Out = Pz<I<O<O<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<241> as NatIndex>::Out as Nat>::VAL == 241);
impl NatIndex for Idx<242> {
    type Out = Pz<O<I<O<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<242> as NatIndex>::Out as Nat>::VAL == 242);
impl NatIndex for Idx<243> {
    type Out = Pz<I<I<O<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<243> as NatIndex>::Out as Nat>::VAL == 243);
impl NatIndex for Idx<244> {
    type Out = Pz<O<O<I<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<244> as NatIndex>::Out as Nat>::VAL == 244);
impl NatIndex for Idx<245> {
    type Out = Pz<I<O<I<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<245> as NatIndex>::Out as Nat>::VAL == 245);
impl NatIndex for Idx<246> {
    type Out = Pz<O<I<I<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<246> as NatIndex>::Out as Nat>::VAL == 246);
impl NatIndex for Idx<247> {
    type Out = Pz<I<I<I<O<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<247> as NatIndex>::Out as Nat>::VAL == 247);
impl NatIndex for Idx<248> {
    type Out = Pz<O<O<O<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<248> as NatIndex>::Out as Nat>::VAL == 248);
impl NatIndex for Idx<249> {
    type Out = Pz<I<O<O<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<249> as NatIndex>::Out as Nat>::VAL == 249);
impl NatIndex for Idx<250> {
    type Out = Pz<O<I<O<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<250> as NatIndex>::Out as Nat>::VAL == 250);
impl NatIndex for Idx<251> {
    type Out = Pz<I<I<O<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<251> as NatIndex>::Out as Nat>::VAL == 251);
impl NatIndex for Idx<252> {
    type Out = Pz<O<O<I<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<252> as NatIndex>::Out as Nat>::VAL == 252);
impl NatIndex for Idx<253> {
    type Out = Pz<I<O<I<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<253> as NatIndex>::Out as Nat>::VAL == 253);
impl NatIndex for Idx<254> {
    type Out = Pz<O<I<I<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<254> as NatIndex>::Out as Nat>::VAL == 254);
impl NatIndex for Idx<255> {
    type Out = Pz<I<I<I<I<I<I<I<H>>>>>>>>;
}
const _: () = assert!(<<Idx<255> as NatIndex>::Out as Nat>::VAL == 255);

// the load-bearing claim
const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
