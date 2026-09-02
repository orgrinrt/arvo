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
    type Out = Pz<I<O<I<H>>>>;
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
impl NatIndex for Idx<256> {
    type Out = Pz<O<O<O<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<256> as NatIndex>::Out as Nat>::VAL == 256);
impl NatIndex for Idx<257> {
    type Out = Pz<I<O<O<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<257> as NatIndex>::Out as Nat>::VAL == 257);
impl NatIndex for Idx<258> {
    type Out = Pz<O<I<O<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<258> as NatIndex>::Out as Nat>::VAL == 258);
impl NatIndex for Idx<259> {
    type Out = Pz<I<I<O<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<259> as NatIndex>::Out as Nat>::VAL == 259);
impl NatIndex for Idx<260> {
    type Out = Pz<O<O<I<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<260> as NatIndex>::Out as Nat>::VAL == 260);
impl NatIndex for Idx<261> {
    type Out = Pz<I<O<I<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<261> as NatIndex>::Out as Nat>::VAL == 261);
impl NatIndex for Idx<262> {
    type Out = Pz<O<I<I<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<262> as NatIndex>::Out as Nat>::VAL == 262);
impl NatIndex for Idx<263> {
    type Out = Pz<I<I<I<O<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<263> as NatIndex>::Out as Nat>::VAL == 263);
impl NatIndex for Idx<264> {
    type Out = Pz<O<O<O<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<264> as NatIndex>::Out as Nat>::VAL == 264);
impl NatIndex for Idx<265> {
    type Out = Pz<I<O<O<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<265> as NatIndex>::Out as Nat>::VAL == 265);
impl NatIndex for Idx<266> {
    type Out = Pz<O<I<O<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<266> as NatIndex>::Out as Nat>::VAL == 266);
impl NatIndex for Idx<267> {
    type Out = Pz<I<I<O<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<267> as NatIndex>::Out as Nat>::VAL == 267);
impl NatIndex for Idx<268> {
    type Out = Pz<O<O<I<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<268> as NatIndex>::Out as Nat>::VAL == 268);
impl NatIndex for Idx<269> {
    type Out = Pz<I<O<I<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<269> as NatIndex>::Out as Nat>::VAL == 269);
impl NatIndex for Idx<270> {
    type Out = Pz<O<I<I<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<270> as NatIndex>::Out as Nat>::VAL == 270);
impl NatIndex for Idx<271> {
    type Out = Pz<I<I<I<I<O<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<271> as NatIndex>::Out as Nat>::VAL == 271);
impl NatIndex for Idx<272> {
    type Out = Pz<O<O<O<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<272> as NatIndex>::Out as Nat>::VAL == 272);
impl NatIndex for Idx<273> {
    type Out = Pz<I<O<O<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<273> as NatIndex>::Out as Nat>::VAL == 273);
impl NatIndex for Idx<274> {
    type Out = Pz<O<I<O<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<274> as NatIndex>::Out as Nat>::VAL == 274);
impl NatIndex for Idx<275> {
    type Out = Pz<I<I<O<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<275> as NatIndex>::Out as Nat>::VAL == 275);
impl NatIndex for Idx<276> {
    type Out = Pz<O<O<I<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<276> as NatIndex>::Out as Nat>::VAL == 276);
impl NatIndex for Idx<277> {
    type Out = Pz<I<O<I<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<277> as NatIndex>::Out as Nat>::VAL == 277);
impl NatIndex for Idx<278> {
    type Out = Pz<O<I<I<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<278> as NatIndex>::Out as Nat>::VAL == 278);
impl NatIndex for Idx<279> {
    type Out = Pz<I<I<I<O<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<279> as NatIndex>::Out as Nat>::VAL == 279);
impl NatIndex for Idx<280> {
    type Out = Pz<O<O<O<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<280> as NatIndex>::Out as Nat>::VAL == 280);
impl NatIndex for Idx<281> {
    type Out = Pz<I<O<O<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<281> as NatIndex>::Out as Nat>::VAL == 281);
impl NatIndex for Idx<282> {
    type Out = Pz<O<I<O<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<282> as NatIndex>::Out as Nat>::VAL == 282);
impl NatIndex for Idx<283> {
    type Out = Pz<I<I<O<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<283> as NatIndex>::Out as Nat>::VAL == 283);
impl NatIndex for Idx<284> {
    type Out = Pz<O<O<I<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<284> as NatIndex>::Out as Nat>::VAL == 284);
impl NatIndex for Idx<285> {
    type Out = Pz<I<O<I<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<285> as NatIndex>::Out as Nat>::VAL == 285);
impl NatIndex for Idx<286> {
    type Out = Pz<O<I<I<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<286> as NatIndex>::Out as Nat>::VAL == 286);
impl NatIndex for Idx<287> {
    type Out = Pz<I<I<I<I<I<O<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<287> as NatIndex>::Out as Nat>::VAL == 287);
impl NatIndex for Idx<288> {
    type Out = Pz<O<O<O<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<288> as NatIndex>::Out as Nat>::VAL == 288);
impl NatIndex for Idx<289> {
    type Out = Pz<I<O<O<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<289> as NatIndex>::Out as Nat>::VAL == 289);
impl NatIndex for Idx<290> {
    type Out = Pz<O<I<O<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<290> as NatIndex>::Out as Nat>::VAL == 290);
impl NatIndex for Idx<291> {
    type Out = Pz<I<I<O<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<291> as NatIndex>::Out as Nat>::VAL == 291);
impl NatIndex for Idx<292> {
    type Out = Pz<O<O<I<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<292> as NatIndex>::Out as Nat>::VAL == 292);
impl NatIndex for Idx<293> {
    type Out = Pz<I<O<I<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<293> as NatIndex>::Out as Nat>::VAL == 293);
impl NatIndex for Idx<294> {
    type Out = Pz<O<I<I<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<294> as NatIndex>::Out as Nat>::VAL == 294);
impl NatIndex for Idx<295> {
    type Out = Pz<I<I<I<O<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<295> as NatIndex>::Out as Nat>::VAL == 295);
impl NatIndex for Idx<296> {
    type Out = Pz<O<O<O<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<296> as NatIndex>::Out as Nat>::VAL == 296);
impl NatIndex for Idx<297> {
    type Out = Pz<I<O<O<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<297> as NatIndex>::Out as Nat>::VAL == 297);
impl NatIndex for Idx<298> {
    type Out = Pz<O<I<O<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<298> as NatIndex>::Out as Nat>::VAL == 298);
impl NatIndex for Idx<299> {
    type Out = Pz<I<I<O<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<299> as NatIndex>::Out as Nat>::VAL == 299);
impl NatIndex for Idx<300> {
    type Out = Pz<O<O<I<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<300> as NatIndex>::Out as Nat>::VAL == 300);
impl NatIndex for Idx<301> {
    type Out = Pz<I<O<I<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<301> as NatIndex>::Out as Nat>::VAL == 301);
impl NatIndex for Idx<302> {
    type Out = Pz<O<I<I<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<302> as NatIndex>::Out as Nat>::VAL == 302);
impl NatIndex for Idx<303> {
    type Out = Pz<I<I<I<I<O<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<303> as NatIndex>::Out as Nat>::VAL == 303);
impl NatIndex for Idx<304> {
    type Out = Pz<O<O<O<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<304> as NatIndex>::Out as Nat>::VAL == 304);
impl NatIndex for Idx<305> {
    type Out = Pz<I<O<O<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<305> as NatIndex>::Out as Nat>::VAL == 305);
impl NatIndex for Idx<306> {
    type Out = Pz<O<I<O<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<306> as NatIndex>::Out as Nat>::VAL == 306);
impl NatIndex for Idx<307> {
    type Out = Pz<I<I<O<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<307> as NatIndex>::Out as Nat>::VAL == 307);
impl NatIndex for Idx<308> {
    type Out = Pz<O<O<I<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<308> as NatIndex>::Out as Nat>::VAL == 308);
impl NatIndex for Idx<309> {
    type Out = Pz<I<O<I<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<309> as NatIndex>::Out as Nat>::VAL == 309);
impl NatIndex for Idx<310> {
    type Out = Pz<O<I<I<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<310> as NatIndex>::Out as Nat>::VAL == 310);
impl NatIndex for Idx<311> {
    type Out = Pz<I<I<I<O<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<311> as NatIndex>::Out as Nat>::VAL == 311);
impl NatIndex for Idx<312> {
    type Out = Pz<O<O<O<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<312> as NatIndex>::Out as Nat>::VAL == 312);
impl NatIndex for Idx<313> {
    type Out = Pz<I<O<O<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<313> as NatIndex>::Out as Nat>::VAL == 313);
impl NatIndex for Idx<314> {
    type Out = Pz<O<I<O<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<314> as NatIndex>::Out as Nat>::VAL == 314);
impl NatIndex for Idx<315> {
    type Out = Pz<I<I<O<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<315> as NatIndex>::Out as Nat>::VAL == 315);
impl NatIndex for Idx<316> {
    type Out = Pz<O<O<I<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<316> as NatIndex>::Out as Nat>::VAL == 316);
impl NatIndex for Idx<317> {
    type Out = Pz<I<O<I<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<317> as NatIndex>::Out as Nat>::VAL == 317);
impl NatIndex for Idx<318> {
    type Out = Pz<O<I<I<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<318> as NatIndex>::Out as Nat>::VAL == 318);
impl NatIndex for Idx<319> {
    type Out = Pz<I<I<I<I<I<I<O<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<319> as NatIndex>::Out as Nat>::VAL == 319);
impl NatIndex for Idx<320> {
    type Out = Pz<O<O<O<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<320> as NatIndex>::Out as Nat>::VAL == 320);
impl NatIndex for Idx<321> {
    type Out = Pz<I<O<O<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<321> as NatIndex>::Out as Nat>::VAL == 321);
impl NatIndex for Idx<322> {
    type Out = Pz<O<I<O<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<322> as NatIndex>::Out as Nat>::VAL == 322);
impl NatIndex for Idx<323> {
    type Out = Pz<I<I<O<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<323> as NatIndex>::Out as Nat>::VAL == 323);
impl NatIndex for Idx<324> {
    type Out = Pz<O<O<I<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<324> as NatIndex>::Out as Nat>::VAL == 324);
impl NatIndex for Idx<325> {
    type Out = Pz<I<O<I<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<325> as NatIndex>::Out as Nat>::VAL == 325);
impl NatIndex for Idx<326> {
    type Out = Pz<O<I<I<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<326> as NatIndex>::Out as Nat>::VAL == 326);
impl NatIndex for Idx<327> {
    type Out = Pz<I<I<I<O<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<327> as NatIndex>::Out as Nat>::VAL == 327);
impl NatIndex for Idx<328> {
    type Out = Pz<O<O<O<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<328> as NatIndex>::Out as Nat>::VAL == 328);
impl NatIndex for Idx<329> {
    type Out = Pz<I<O<O<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<329> as NatIndex>::Out as Nat>::VAL == 329);
impl NatIndex for Idx<330> {
    type Out = Pz<O<I<O<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<330> as NatIndex>::Out as Nat>::VAL == 330);
impl NatIndex for Idx<331> {
    type Out = Pz<I<I<O<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<331> as NatIndex>::Out as Nat>::VAL == 331);
impl NatIndex for Idx<332> {
    type Out = Pz<O<O<I<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<332> as NatIndex>::Out as Nat>::VAL == 332);
impl NatIndex for Idx<333> {
    type Out = Pz<I<O<I<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<333> as NatIndex>::Out as Nat>::VAL == 333);
impl NatIndex for Idx<334> {
    type Out = Pz<O<I<I<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<334> as NatIndex>::Out as Nat>::VAL == 334);
impl NatIndex for Idx<335> {
    type Out = Pz<I<I<I<I<O<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<335> as NatIndex>::Out as Nat>::VAL == 335);
impl NatIndex for Idx<336> {
    type Out = Pz<O<O<O<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<336> as NatIndex>::Out as Nat>::VAL == 336);
impl NatIndex for Idx<337> {
    type Out = Pz<I<O<O<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<337> as NatIndex>::Out as Nat>::VAL == 337);
impl NatIndex for Idx<338> {
    type Out = Pz<O<I<O<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<338> as NatIndex>::Out as Nat>::VAL == 338);
impl NatIndex for Idx<339> {
    type Out = Pz<I<I<O<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<339> as NatIndex>::Out as Nat>::VAL == 339);
impl NatIndex for Idx<340> {
    type Out = Pz<O<O<I<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<340> as NatIndex>::Out as Nat>::VAL == 340);
impl NatIndex for Idx<341> {
    type Out = Pz<I<O<I<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<341> as NatIndex>::Out as Nat>::VAL == 341);
impl NatIndex for Idx<342> {
    type Out = Pz<O<I<I<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<342> as NatIndex>::Out as Nat>::VAL == 342);
impl NatIndex for Idx<343> {
    type Out = Pz<I<I<I<O<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<343> as NatIndex>::Out as Nat>::VAL == 343);
impl NatIndex for Idx<344> {
    type Out = Pz<O<O<O<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<344> as NatIndex>::Out as Nat>::VAL == 344);
impl NatIndex for Idx<345> {
    type Out = Pz<I<O<O<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<345> as NatIndex>::Out as Nat>::VAL == 345);
impl NatIndex for Idx<346> {
    type Out = Pz<O<I<O<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<346> as NatIndex>::Out as Nat>::VAL == 346);
impl NatIndex for Idx<347> {
    type Out = Pz<I<I<O<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<347> as NatIndex>::Out as Nat>::VAL == 347);
impl NatIndex for Idx<348> {
    type Out = Pz<O<O<I<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<348> as NatIndex>::Out as Nat>::VAL == 348);
impl NatIndex for Idx<349> {
    type Out = Pz<I<O<I<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<349> as NatIndex>::Out as Nat>::VAL == 349);
impl NatIndex for Idx<350> {
    type Out = Pz<O<I<I<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<350> as NatIndex>::Out as Nat>::VAL == 350);
impl NatIndex for Idx<351> {
    type Out = Pz<I<I<I<I<I<O<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<351> as NatIndex>::Out as Nat>::VAL == 351);
impl NatIndex for Idx<352> {
    type Out = Pz<O<O<O<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<352> as NatIndex>::Out as Nat>::VAL == 352);
impl NatIndex for Idx<353> {
    type Out = Pz<I<O<O<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<353> as NatIndex>::Out as Nat>::VAL == 353);
impl NatIndex for Idx<354> {
    type Out = Pz<O<I<O<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<354> as NatIndex>::Out as Nat>::VAL == 354);
impl NatIndex for Idx<355> {
    type Out = Pz<I<I<O<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<355> as NatIndex>::Out as Nat>::VAL == 355);
impl NatIndex for Idx<356> {
    type Out = Pz<O<O<I<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<356> as NatIndex>::Out as Nat>::VAL == 356);
impl NatIndex for Idx<357> {
    type Out = Pz<I<O<I<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<357> as NatIndex>::Out as Nat>::VAL == 357);
impl NatIndex for Idx<358> {
    type Out = Pz<O<I<I<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<358> as NatIndex>::Out as Nat>::VAL == 358);
impl NatIndex for Idx<359> {
    type Out = Pz<I<I<I<O<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<359> as NatIndex>::Out as Nat>::VAL == 359);
impl NatIndex for Idx<360> {
    type Out = Pz<O<O<O<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<360> as NatIndex>::Out as Nat>::VAL == 360);
impl NatIndex for Idx<361> {
    type Out = Pz<I<O<O<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<361> as NatIndex>::Out as Nat>::VAL == 361);
impl NatIndex for Idx<362> {
    type Out = Pz<O<I<O<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<362> as NatIndex>::Out as Nat>::VAL == 362);
impl NatIndex for Idx<363> {
    type Out = Pz<I<I<O<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<363> as NatIndex>::Out as Nat>::VAL == 363);
impl NatIndex for Idx<364> {
    type Out = Pz<O<O<I<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<364> as NatIndex>::Out as Nat>::VAL == 364);
impl NatIndex for Idx<365> {
    type Out = Pz<I<O<I<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<365> as NatIndex>::Out as Nat>::VAL == 365);
impl NatIndex for Idx<366> {
    type Out = Pz<O<I<I<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<366> as NatIndex>::Out as Nat>::VAL == 366);
impl NatIndex for Idx<367> {
    type Out = Pz<I<I<I<I<O<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<367> as NatIndex>::Out as Nat>::VAL == 367);
impl NatIndex for Idx<368> {
    type Out = Pz<O<O<O<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<368> as NatIndex>::Out as Nat>::VAL == 368);
impl NatIndex for Idx<369> {
    type Out = Pz<I<O<O<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<369> as NatIndex>::Out as Nat>::VAL == 369);
impl NatIndex for Idx<370> {
    type Out = Pz<O<I<O<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<370> as NatIndex>::Out as Nat>::VAL == 370);
impl NatIndex for Idx<371> {
    type Out = Pz<I<I<O<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<371> as NatIndex>::Out as Nat>::VAL == 371);
impl NatIndex for Idx<372> {
    type Out = Pz<O<O<I<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<372> as NatIndex>::Out as Nat>::VAL == 372);
impl NatIndex for Idx<373> {
    type Out = Pz<I<O<I<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<373> as NatIndex>::Out as Nat>::VAL == 373);
impl NatIndex for Idx<374> {
    type Out = Pz<O<I<I<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<374> as NatIndex>::Out as Nat>::VAL == 374);
impl NatIndex for Idx<375> {
    type Out = Pz<I<I<I<O<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<375> as NatIndex>::Out as Nat>::VAL == 375);
impl NatIndex for Idx<376> {
    type Out = Pz<O<O<O<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<376> as NatIndex>::Out as Nat>::VAL == 376);
impl NatIndex for Idx<377> {
    type Out = Pz<I<O<O<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<377> as NatIndex>::Out as Nat>::VAL == 377);
impl NatIndex for Idx<378> {
    type Out = Pz<O<I<O<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<378> as NatIndex>::Out as Nat>::VAL == 378);
impl NatIndex for Idx<379> {
    type Out = Pz<I<I<O<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<379> as NatIndex>::Out as Nat>::VAL == 379);
impl NatIndex for Idx<380> {
    type Out = Pz<O<O<I<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<380> as NatIndex>::Out as Nat>::VAL == 380);
impl NatIndex for Idx<381> {
    type Out = Pz<I<O<I<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<381> as NatIndex>::Out as Nat>::VAL == 381);
impl NatIndex for Idx<382> {
    type Out = Pz<O<I<I<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<382> as NatIndex>::Out as Nat>::VAL == 382);
impl NatIndex for Idx<383> {
    type Out = Pz<I<I<I<I<I<I<I<O<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<383> as NatIndex>::Out as Nat>::VAL == 383);
impl NatIndex for Idx<384> {
    type Out = Pz<O<O<O<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<384> as NatIndex>::Out as Nat>::VAL == 384);
impl NatIndex for Idx<385> {
    type Out = Pz<I<O<O<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<385> as NatIndex>::Out as Nat>::VAL == 385);
impl NatIndex for Idx<386> {
    type Out = Pz<O<I<O<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<386> as NatIndex>::Out as Nat>::VAL == 386);
impl NatIndex for Idx<387> {
    type Out = Pz<I<I<O<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<387> as NatIndex>::Out as Nat>::VAL == 387);
impl NatIndex for Idx<388> {
    type Out = Pz<O<O<I<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<388> as NatIndex>::Out as Nat>::VAL == 388);
impl NatIndex for Idx<389> {
    type Out = Pz<I<O<I<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<389> as NatIndex>::Out as Nat>::VAL == 389);
impl NatIndex for Idx<390> {
    type Out = Pz<O<I<I<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<390> as NatIndex>::Out as Nat>::VAL == 390);
impl NatIndex for Idx<391> {
    type Out = Pz<I<I<I<O<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<391> as NatIndex>::Out as Nat>::VAL == 391);
impl NatIndex for Idx<392> {
    type Out = Pz<O<O<O<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<392> as NatIndex>::Out as Nat>::VAL == 392);
impl NatIndex for Idx<393> {
    type Out = Pz<I<O<O<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<393> as NatIndex>::Out as Nat>::VAL == 393);
impl NatIndex for Idx<394> {
    type Out = Pz<O<I<O<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<394> as NatIndex>::Out as Nat>::VAL == 394);
impl NatIndex for Idx<395> {
    type Out = Pz<I<I<O<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<395> as NatIndex>::Out as Nat>::VAL == 395);
impl NatIndex for Idx<396> {
    type Out = Pz<O<O<I<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<396> as NatIndex>::Out as Nat>::VAL == 396);
impl NatIndex for Idx<397> {
    type Out = Pz<I<O<I<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<397> as NatIndex>::Out as Nat>::VAL == 397);
impl NatIndex for Idx<398> {
    type Out = Pz<O<I<I<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<398> as NatIndex>::Out as Nat>::VAL == 398);
impl NatIndex for Idx<399> {
    type Out = Pz<I<I<I<I<O<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<399> as NatIndex>::Out as Nat>::VAL == 399);
impl NatIndex for Idx<400> {
    type Out = Pz<O<O<O<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<400> as NatIndex>::Out as Nat>::VAL == 400);
impl NatIndex for Idx<401> {
    type Out = Pz<I<O<O<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<401> as NatIndex>::Out as Nat>::VAL == 401);
impl NatIndex for Idx<402> {
    type Out = Pz<O<I<O<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<402> as NatIndex>::Out as Nat>::VAL == 402);
impl NatIndex for Idx<403> {
    type Out = Pz<I<I<O<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<403> as NatIndex>::Out as Nat>::VAL == 403);
impl NatIndex for Idx<404> {
    type Out = Pz<O<O<I<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<404> as NatIndex>::Out as Nat>::VAL == 404);
impl NatIndex for Idx<405> {
    type Out = Pz<I<O<I<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<405> as NatIndex>::Out as Nat>::VAL == 405);
impl NatIndex for Idx<406> {
    type Out = Pz<O<I<I<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<406> as NatIndex>::Out as Nat>::VAL == 406);
impl NatIndex for Idx<407> {
    type Out = Pz<I<I<I<O<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<407> as NatIndex>::Out as Nat>::VAL == 407);
impl NatIndex for Idx<408> {
    type Out = Pz<O<O<O<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<408> as NatIndex>::Out as Nat>::VAL == 408);
impl NatIndex for Idx<409> {
    type Out = Pz<I<O<O<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<409> as NatIndex>::Out as Nat>::VAL == 409);
impl NatIndex for Idx<410> {
    type Out = Pz<O<I<O<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<410> as NatIndex>::Out as Nat>::VAL == 410);
impl NatIndex for Idx<411> {
    type Out = Pz<I<I<O<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<411> as NatIndex>::Out as Nat>::VAL == 411);
impl NatIndex for Idx<412> {
    type Out = Pz<O<O<I<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<412> as NatIndex>::Out as Nat>::VAL == 412);
impl NatIndex for Idx<413> {
    type Out = Pz<I<O<I<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<413> as NatIndex>::Out as Nat>::VAL == 413);
impl NatIndex for Idx<414> {
    type Out = Pz<O<I<I<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<414> as NatIndex>::Out as Nat>::VAL == 414);
impl NatIndex for Idx<415> {
    type Out = Pz<I<I<I<I<I<O<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<415> as NatIndex>::Out as Nat>::VAL == 415);
impl NatIndex for Idx<416> {
    type Out = Pz<O<O<O<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<416> as NatIndex>::Out as Nat>::VAL == 416);
impl NatIndex for Idx<417> {
    type Out = Pz<I<O<O<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<417> as NatIndex>::Out as Nat>::VAL == 417);
impl NatIndex for Idx<418> {
    type Out = Pz<O<I<O<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<418> as NatIndex>::Out as Nat>::VAL == 418);
impl NatIndex for Idx<419> {
    type Out = Pz<I<I<O<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<419> as NatIndex>::Out as Nat>::VAL == 419);
impl NatIndex for Idx<420> {
    type Out = Pz<O<O<I<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<420> as NatIndex>::Out as Nat>::VAL == 420);
impl NatIndex for Idx<421> {
    type Out = Pz<I<O<I<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<421> as NatIndex>::Out as Nat>::VAL == 421);
impl NatIndex for Idx<422> {
    type Out = Pz<O<I<I<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<422> as NatIndex>::Out as Nat>::VAL == 422);
impl NatIndex for Idx<423> {
    type Out = Pz<I<I<I<O<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<423> as NatIndex>::Out as Nat>::VAL == 423);
impl NatIndex for Idx<424> {
    type Out = Pz<O<O<O<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<424> as NatIndex>::Out as Nat>::VAL == 424);
impl NatIndex for Idx<425> {
    type Out = Pz<I<O<O<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<425> as NatIndex>::Out as Nat>::VAL == 425);
impl NatIndex for Idx<426> {
    type Out = Pz<O<I<O<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<426> as NatIndex>::Out as Nat>::VAL == 426);
impl NatIndex for Idx<427> {
    type Out = Pz<I<I<O<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<427> as NatIndex>::Out as Nat>::VAL == 427);
impl NatIndex for Idx<428> {
    type Out = Pz<O<O<I<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<428> as NatIndex>::Out as Nat>::VAL == 428);
impl NatIndex for Idx<429> {
    type Out = Pz<I<O<I<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<429> as NatIndex>::Out as Nat>::VAL == 429);
impl NatIndex for Idx<430> {
    type Out = Pz<O<I<I<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<430> as NatIndex>::Out as Nat>::VAL == 430);
impl NatIndex for Idx<431> {
    type Out = Pz<I<I<I<I<O<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<431> as NatIndex>::Out as Nat>::VAL == 431);
impl NatIndex for Idx<432> {
    type Out = Pz<O<O<O<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<432> as NatIndex>::Out as Nat>::VAL == 432);
impl NatIndex for Idx<433> {
    type Out = Pz<I<O<O<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<433> as NatIndex>::Out as Nat>::VAL == 433);
impl NatIndex for Idx<434> {
    type Out = Pz<O<I<O<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<434> as NatIndex>::Out as Nat>::VAL == 434);
impl NatIndex for Idx<435> {
    type Out = Pz<I<I<O<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<435> as NatIndex>::Out as Nat>::VAL == 435);
impl NatIndex for Idx<436> {
    type Out = Pz<O<O<I<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<436> as NatIndex>::Out as Nat>::VAL == 436);
impl NatIndex for Idx<437> {
    type Out = Pz<I<O<I<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<437> as NatIndex>::Out as Nat>::VAL == 437);
impl NatIndex for Idx<438> {
    type Out = Pz<O<I<I<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<438> as NatIndex>::Out as Nat>::VAL == 438);
impl NatIndex for Idx<439> {
    type Out = Pz<I<I<I<O<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<439> as NatIndex>::Out as Nat>::VAL == 439);
impl NatIndex for Idx<440> {
    type Out = Pz<O<O<O<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<440> as NatIndex>::Out as Nat>::VAL == 440);
impl NatIndex for Idx<441> {
    type Out = Pz<I<O<O<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<441> as NatIndex>::Out as Nat>::VAL == 441);
impl NatIndex for Idx<442> {
    type Out = Pz<O<I<O<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<442> as NatIndex>::Out as Nat>::VAL == 442);
impl NatIndex for Idx<443> {
    type Out = Pz<I<I<O<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<443> as NatIndex>::Out as Nat>::VAL == 443);
impl NatIndex for Idx<444> {
    type Out = Pz<O<O<I<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<444> as NatIndex>::Out as Nat>::VAL == 444);
impl NatIndex for Idx<445> {
    type Out = Pz<I<O<I<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<445> as NatIndex>::Out as Nat>::VAL == 445);
impl NatIndex for Idx<446> {
    type Out = Pz<O<I<I<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<446> as NatIndex>::Out as Nat>::VAL == 446);
impl NatIndex for Idx<447> {
    type Out = Pz<I<I<I<I<I<I<O<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<447> as NatIndex>::Out as Nat>::VAL == 447);
impl NatIndex for Idx<448> {
    type Out = Pz<O<O<O<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<448> as NatIndex>::Out as Nat>::VAL == 448);
impl NatIndex for Idx<449> {
    type Out = Pz<I<O<O<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<449> as NatIndex>::Out as Nat>::VAL == 449);
impl NatIndex for Idx<450> {
    type Out = Pz<O<I<O<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<450> as NatIndex>::Out as Nat>::VAL == 450);
impl NatIndex for Idx<451> {
    type Out = Pz<I<I<O<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<451> as NatIndex>::Out as Nat>::VAL == 451);
impl NatIndex for Idx<452> {
    type Out = Pz<O<O<I<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<452> as NatIndex>::Out as Nat>::VAL == 452);
impl NatIndex for Idx<453> {
    type Out = Pz<I<O<I<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<453> as NatIndex>::Out as Nat>::VAL == 453);
impl NatIndex for Idx<454> {
    type Out = Pz<O<I<I<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<454> as NatIndex>::Out as Nat>::VAL == 454);
impl NatIndex for Idx<455> {
    type Out = Pz<I<I<I<O<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<455> as NatIndex>::Out as Nat>::VAL == 455);
impl NatIndex for Idx<456> {
    type Out = Pz<O<O<O<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<456> as NatIndex>::Out as Nat>::VAL == 456);
impl NatIndex for Idx<457> {
    type Out = Pz<I<O<O<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<457> as NatIndex>::Out as Nat>::VAL == 457);
impl NatIndex for Idx<458> {
    type Out = Pz<O<I<O<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<458> as NatIndex>::Out as Nat>::VAL == 458);
impl NatIndex for Idx<459> {
    type Out = Pz<I<I<O<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<459> as NatIndex>::Out as Nat>::VAL == 459);
impl NatIndex for Idx<460> {
    type Out = Pz<O<O<I<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<460> as NatIndex>::Out as Nat>::VAL == 460);
impl NatIndex for Idx<461> {
    type Out = Pz<I<O<I<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<461> as NatIndex>::Out as Nat>::VAL == 461);
impl NatIndex for Idx<462> {
    type Out = Pz<O<I<I<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<462> as NatIndex>::Out as Nat>::VAL == 462);
impl NatIndex for Idx<463> {
    type Out = Pz<I<I<I<I<O<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<463> as NatIndex>::Out as Nat>::VAL == 463);
impl NatIndex for Idx<464> {
    type Out = Pz<O<O<O<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<464> as NatIndex>::Out as Nat>::VAL == 464);
impl NatIndex for Idx<465> {
    type Out = Pz<I<O<O<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<465> as NatIndex>::Out as Nat>::VAL == 465);
impl NatIndex for Idx<466> {
    type Out = Pz<O<I<O<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<466> as NatIndex>::Out as Nat>::VAL == 466);
impl NatIndex for Idx<467> {
    type Out = Pz<I<I<O<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<467> as NatIndex>::Out as Nat>::VAL == 467);
impl NatIndex for Idx<468> {
    type Out = Pz<O<O<I<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<468> as NatIndex>::Out as Nat>::VAL == 468);
impl NatIndex for Idx<469> {
    type Out = Pz<I<O<I<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<469> as NatIndex>::Out as Nat>::VAL == 469);
impl NatIndex for Idx<470> {
    type Out = Pz<O<I<I<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<470> as NatIndex>::Out as Nat>::VAL == 470);
impl NatIndex for Idx<471> {
    type Out = Pz<I<I<I<O<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<471> as NatIndex>::Out as Nat>::VAL == 471);
impl NatIndex for Idx<472> {
    type Out = Pz<O<O<O<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<472> as NatIndex>::Out as Nat>::VAL == 472);
impl NatIndex for Idx<473> {
    type Out = Pz<I<O<O<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<473> as NatIndex>::Out as Nat>::VAL == 473);
impl NatIndex for Idx<474> {
    type Out = Pz<O<I<O<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<474> as NatIndex>::Out as Nat>::VAL == 474);
impl NatIndex for Idx<475> {
    type Out = Pz<I<I<O<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<475> as NatIndex>::Out as Nat>::VAL == 475);
impl NatIndex for Idx<476> {
    type Out = Pz<O<O<I<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<476> as NatIndex>::Out as Nat>::VAL == 476);
impl NatIndex for Idx<477> {
    type Out = Pz<I<O<I<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<477> as NatIndex>::Out as Nat>::VAL == 477);
impl NatIndex for Idx<478> {
    type Out = Pz<O<I<I<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<478> as NatIndex>::Out as Nat>::VAL == 478);
impl NatIndex for Idx<479> {
    type Out = Pz<I<I<I<I<I<O<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<479> as NatIndex>::Out as Nat>::VAL == 479);
impl NatIndex for Idx<480> {
    type Out = Pz<O<O<O<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<480> as NatIndex>::Out as Nat>::VAL == 480);
impl NatIndex for Idx<481> {
    type Out = Pz<I<O<O<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<481> as NatIndex>::Out as Nat>::VAL == 481);
impl NatIndex for Idx<482> {
    type Out = Pz<O<I<O<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<482> as NatIndex>::Out as Nat>::VAL == 482);
impl NatIndex for Idx<483> {
    type Out = Pz<I<I<O<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<483> as NatIndex>::Out as Nat>::VAL == 483);
impl NatIndex for Idx<484> {
    type Out = Pz<O<O<I<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<484> as NatIndex>::Out as Nat>::VAL == 484);
impl NatIndex for Idx<485> {
    type Out = Pz<I<O<I<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<485> as NatIndex>::Out as Nat>::VAL == 485);
impl NatIndex for Idx<486> {
    type Out = Pz<O<I<I<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<486> as NatIndex>::Out as Nat>::VAL == 486);
impl NatIndex for Idx<487> {
    type Out = Pz<I<I<I<O<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<487> as NatIndex>::Out as Nat>::VAL == 487);
impl NatIndex for Idx<488> {
    type Out = Pz<O<O<O<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<488> as NatIndex>::Out as Nat>::VAL == 488);
impl NatIndex for Idx<489> {
    type Out = Pz<I<O<O<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<489> as NatIndex>::Out as Nat>::VAL == 489);
impl NatIndex for Idx<490> {
    type Out = Pz<O<I<O<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<490> as NatIndex>::Out as Nat>::VAL == 490);
impl NatIndex for Idx<491> {
    type Out = Pz<I<I<O<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<491> as NatIndex>::Out as Nat>::VAL == 491);
impl NatIndex for Idx<492> {
    type Out = Pz<O<O<I<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<492> as NatIndex>::Out as Nat>::VAL == 492);
impl NatIndex for Idx<493> {
    type Out = Pz<I<O<I<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<493> as NatIndex>::Out as Nat>::VAL == 493);
impl NatIndex for Idx<494> {
    type Out = Pz<O<I<I<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<494> as NatIndex>::Out as Nat>::VAL == 494);
impl NatIndex for Idx<495> {
    type Out = Pz<I<I<I<I<O<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<495> as NatIndex>::Out as Nat>::VAL == 495);
impl NatIndex for Idx<496> {
    type Out = Pz<O<O<O<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<496> as NatIndex>::Out as Nat>::VAL == 496);
impl NatIndex for Idx<497> {
    type Out = Pz<I<O<O<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<497> as NatIndex>::Out as Nat>::VAL == 497);
impl NatIndex for Idx<498> {
    type Out = Pz<O<I<O<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<498> as NatIndex>::Out as Nat>::VAL == 498);
impl NatIndex for Idx<499> {
    type Out = Pz<I<I<O<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<499> as NatIndex>::Out as Nat>::VAL == 499);
impl NatIndex for Idx<500> {
    type Out = Pz<O<O<I<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<500> as NatIndex>::Out as Nat>::VAL == 500);
impl NatIndex for Idx<501> {
    type Out = Pz<I<O<I<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<501> as NatIndex>::Out as Nat>::VAL == 501);
impl NatIndex for Idx<502> {
    type Out = Pz<O<I<I<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<502> as NatIndex>::Out as Nat>::VAL == 502);
impl NatIndex for Idx<503> {
    type Out = Pz<I<I<I<O<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<503> as NatIndex>::Out as Nat>::VAL == 503);
impl NatIndex for Idx<504> {
    type Out = Pz<O<O<O<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<504> as NatIndex>::Out as Nat>::VAL == 504);
impl NatIndex for Idx<505> {
    type Out = Pz<I<O<O<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<505> as NatIndex>::Out as Nat>::VAL == 505);
impl NatIndex for Idx<506> {
    type Out = Pz<O<I<O<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<506> as NatIndex>::Out as Nat>::VAL == 506);
impl NatIndex for Idx<507> {
    type Out = Pz<I<I<O<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<507> as NatIndex>::Out as Nat>::VAL == 507);
impl NatIndex for Idx<508> {
    type Out = Pz<O<O<I<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<508> as NatIndex>::Out as Nat>::VAL == 508);
impl NatIndex for Idx<509> {
    type Out = Pz<I<O<I<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<509> as NatIndex>::Out as Nat>::VAL == 509);
impl NatIndex for Idx<510> {
    type Out = Pz<O<I<I<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<510> as NatIndex>::Out as Nat>::VAL == 510);
impl NatIndex for Idx<511> {
    type Out = Pz<I<I<I<I<I<I<I<I<H>>>>>>>>>;
}
const _: () = assert!(<<Idx<511> as NatIndex>::Out as Nat>::VAL == 511);
impl NatIndex for Idx<512> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<512> as NatIndex>::Out as Nat>::VAL == 512);
impl NatIndex for Idx<513> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<513> as NatIndex>::Out as Nat>::VAL == 513);
impl NatIndex for Idx<514> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<514> as NatIndex>::Out as Nat>::VAL == 514);
impl NatIndex for Idx<515> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<515> as NatIndex>::Out as Nat>::VAL == 515);
impl NatIndex for Idx<516> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<516> as NatIndex>::Out as Nat>::VAL == 516);
impl NatIndex for Idx<517> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<517> as NatIndex>::Out as Nat>::VAL == 517);
impl NatIndex for Idx<518> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<518> as NatIndex>::Out as Nat>::VAL == 518);
impl NatIndex for Idx<519> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<519> as NatIndex>::Out as Nat>::VAL == 519);
impl NatIndex for Idx<520> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<520> as NatIndex>::Out as Nat>::VAL == 520);
impl NatIndex for Idx<521> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<521> as NatIndex>::Out as Nat>::VAL == 521);
impl NatIndex for Idx<522> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<522> as NatIndex>::Out as Nat>::VAL == 522);
impl NatIndex for Idx<523> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<523> as NatIndex>::Out as Nat>::VAL == 523);
impl NatIndex for Idx<524> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<524> as NatIndex>::Out as Nat>::VAL == 524);
impl NatIndex for Idx<525> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<525> as NatIndex>::Out as Nat>::VAL == 525);
impl NatIndex for Idx<526> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<526> as NatIndex>::Out as Nat>::VAL == 526);
impl NatIndex for Idx<527> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<527> as NatIndex>::Out as Nat>::VAL == 527);
impl NatIndex for Idx<528> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<528> as NatIndex>::Out as Nat>::VAL == 528);
impl NatIndex for Idx<529> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<529> as NatIndex>::Out as Nat>::VAL == 529);
impl NatIndex for Idx<530> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<530> as NatIndex>::Out as Nat>::VAL == 530);
impl NatIndex for Idx<531> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<531> as NatIndex>::Out as Nat>::VAL == 531);
impl NatIndex for Idx<532> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<532> as NatIndex>::Out as Nat>::VAL == 532);
impl NatIndex for Idx<533> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<533> as NatIndex>::Out as Nat>::VAL == 533);
impl NatIndex for Idx<534> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<534> as NatIndex>::Out as Nat>::VAL == 534);
impl NatIndex for Idx<535> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<535> as NatIndex>::Out as Nat>::VAL == 535);
impl NatIndex for Idx<536> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<536> as NatIndex>::Out as Nat>::VAL == 536);
impl NatIndex for Idx<537> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<537> as NatIndex>::Out as Nat>::VAL == 537);
impl NatIndex for Idx<538> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<538> as NatIndex>::Out as Nat>::VAL == 538);
impl NatIndex for Idx<539> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<539> as NatIndex>::Out as Nat>::VAL == 539);
impl NatIndex for Idx<540> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<540> as NatIndex>::Out as Nat>::VAL == 540);
impl NatIndex for Idx<541> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<541> as NatIndex>::Out as Nat>::VAL == 541);
impl NatIndex for Idx<542> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<542> as NatIndex>::Out as Nat>::VAL == 542);
impl NatIndex for Idx<543> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<543> as NatIndex>::Out as Nat>::VAL == 543);
impl NatIndex for Idx<544> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<544> as NatIndex>::Out as Nat>::VAL == 544);
impl NatIndex for Idx<545> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<545> as NatIndex>::Out as Nat>::VAL == 545);
impl NatIndex for Idx<546> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<546> as NatIndex>::Out as Nat>::VAL == 546);
impl NatIndex for Idx<547> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<547> as NatIndex>::Out as Nat>::VAL == 547);
impl NatIndex for Idx<548> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<548> as NatIndex>::Out as Nat>::VAL == 548);
impl NatIndex for Idx<549> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<549> as NatIndex>::Out as Nat>::VAL == 549);
impl NatIndex for Idx<550> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<550> as NatIndex>::Out as Nat>::VAL == 550);
impl NatIndex for Idx<551> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<551> as NatIndex>::Out as Nat>::VAL == 551);
impl NatIndex for Idx<552> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<552> as NatIndex>::Out as Nat>::VAL == 552);
impl NatIndex for Idx<553> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<553> as NatIndex>::Out as Nat>::VAL == 553);
impl NatIndex for Idx<554> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<554> as NatIndex>::Out as Nat>::VAL == 554);
impl NatIndex for Idx<555> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<555> as NatIndex>::Out as Nat>::VAL == 555);
impl NatIndex for Idx<556> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<556> as NatIndex>::Out as Nat>::VAL == 556);
impl NatIndex for Idx<557> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<557> as NatIndex>::Out as Nat>::VAL == 557);
impl NatIndex for Idx<558> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<558> as NatIndex>::Out as Nat>::VAL == 558);
impl NatIndex for Idx<559> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<559> as NatIndex>::Out as Nat>::VAL == 559);
impl NatIndex for Idx<560> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<560> as NatIndex>::Out as Nat>::VAL == 560);
impl NatIndex for Idx<561> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<561> as NatIndex>::Out as Nat>::VAL == 561);
impl NatIndex for Idx<562> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<562> as NatIndex>::Out as Nat>::VAL == 562);
impl NatIndex for Idx<563> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<563> as NatIndex>::Out as Nat>::VAL == 563);
impl NatIndex for Idx<564> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<564> as NatIndex>::Out as Nat>::VAL == 564);
impl NatIndex for Idx<565> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<565> as NatIndex>::Out as Nat>::VAL == 565);
impl NatIndex for Idx<566> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<566> as NatIndex>::Out as Nat>::VAL == 566);
impl NatIndex for Idx<567> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<567> as NatIndex>::Out as Nat>::VAL == 567);
impl NatIndex for Idx<568> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<568> as NatIndex>::Out as Nat>::VAL == 568);
impl NatIndex for Idx<569> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<569> as NatIndex>::Out as Nat>::VAL == 569);
impl NatIndex for Idx<570> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<570> as NatIndex>::Out as Nat>::VAL == 570);
impl NatIndex for Idx<571> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<571> as NatIndex>::Out as Nat>::VAL == 571);
impl NatIndex for Idx<572> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<572> as NatIndex>::Out as Nat>::VAL == 572);
impl NatIndex for Idx<573> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<573> as NatIndex>::Out as Nat>::VAL == 573);
impl NatIndex for Idx<574> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<574> as NatIndex>::Out as Nat>::VAL == 574);
impl NatIndex for Idx<575> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<575> as NatIndex>::Out as Nat>::VAL == 575);
impl NatIndex for Idx<576> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<576> as NatIndex>::Out as Nat>::VAL == 576);
impl NatIndex for Idx<577> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<577> as NatIndex>::Out as Nat>::VAL == 577);
impl NatIndex for Idx<578> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<578> as NatIndex>::Out as Nat>::VAL == 578);
impl NatIndex for Idx<579> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<579> as NatIndex>::Out as Nat>::VAL == 579);
impl NatIndex for Idx<580> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<580> as NatIndex>::Out as Nat>::VAL == 580);
impl NatIndex for Idx<581> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<581> as NatIndex>::Out as Nat>::VAL == 581);
impl NatIndex for Idx<582> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<582> as NatIndex>::Out as Nat>::VAL == 582);
impl NatIndex for Idx<583> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<583> as NatIndex>::Out as Nat>::VAL == 583);
impl NatIndex for Idx<584> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<584> as NatIndex>::Out as Nat>::VAL == 584);
impl NatIndex for Idx<585> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<585> as NatIndex>::Out as Nat>::VAL == 585);
impl NatIndex for Idx<586> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<586> as NatIndex>::Out as Nat>::VAL == 586);
impl NatIndex for Idx<587> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<587> as NatIndex>::Out as Nat>::VAL == 587);
impl NatIndex for Idx<588> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<588> as NatIndex>::Out as Nat>::VAL == 588);
impl NatIndex for Idx<589> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<589> as NatIndex>::Out as Nat>::VAL == 589);
impl NatIndex for Idx<590> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<590> as NatIndex>::Out as Nat>::VAL == 590);
impl NatIndex for Idx<591> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<591> as NatIndex>::Out as Nat>::VAL == 591);
impl NatIndex for Idx<592> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<592> as NatIndex>::Out as Nat>::VAL == 592);
impl NatIndex for Idx<593> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<593> as NatIndex>::Out as Nat>::VAL == 593);
impl NatIndex for Idx<594> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<594> as NatIndex>::Out as Nat>::VAL == 594);
impl NatIndex for Idx<595> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<595> as NatIndex>::Out as Nat>::VAL == 595);
impl NatIndex for Idx<596> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<596> as NatIndex>::Out as Nat>::VAL == 596);
impl NatIndex for Idx<597> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<597> as NatIndex>::Out as Nat>::VAL == 597);
impl NatIndex for Idx<598> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<598> as NatIndex>::Out as Nat>::VAL == 598);
impl NatIndex for Idx<599> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<599> as NatIndex>::Out as Nat>::VAL == 599);
impl NatIndex for Idx<600> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<600> as NatIndex>::Out as Nat>::VAL == 600);
impl NatIndex for Idx<601> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<601> as NatIndex>::Out as Nat>::VAL == 601);
impl NatIndex for Idx<602> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<602> as NatIndex>::Out as Nat>::VAL == 602);
impl NatIndex for Idx<603> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<603> as NatIndex>::Out as Nat>::VAL == 603);
impl NatIndex for Idx<604> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<604> as NatIndex>::Out as Nat>::VAL == 604);
impl NatIndex for Idx<605> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<605> as NatIndex>::Out as Nat>::VAL == 605);
impl NatIndex for Idx<606> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<606> as NatIndex>::Out as Nat>::VAL == 606);
impl NatIndex for Idx<607> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<607> as NatIndex>::Out as Nat>::VAL == 607);
impl NatIndex for Idx<608> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<608> as NatIndex>::Out as Nat>::VAL == 608);
impl NatIndex for Idx<609> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<609> as NatIndex>::Out as Nat>::VAL == 609);
impl NatIndex for Idx<610> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<610> as NatIndex>::Out as Nat>::VAL == 610);
impl NatIndex for Idx<611> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<611> as NatIndex>::Out as Nat>::VAL == 611);
impl NatIndex for Idx<612> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<612> as NatIndex>::Out as Nat>::VAL == 612);
impl NatIndex for Idx<613> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<613> as NatIndex>::Out as Nat>::VAL == 613);
impl NatIndex for Idx<614> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<614> as NatIndex>::Out as Nat>::VAL == 614);
impl NatIndex for Idx<615> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<615> as NatIndex>::Out as Nat>::VAL == 615);
impl NatIndex for Idx<616> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<616> as NatIndex>::Out as Nat>::VAL == 616);
impl NatIndex for Idx<617> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<617> as NatIndex>::Out as Nat>::VAL == 617);
impl NatIndex for Idx<618> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<618> as NatIndex>::Out as Nat>::VAL == 618);
impl NatIndex for Idx<619> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<619> as NatIndex>::Out as Nat>::VAL == 619);
impl NatIndex for Idx<620> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<620> as NatIndex>::Out as Nat>::VAL == 620);
impl NatIndex for Idx<621> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<621> as NatIndex>::Out as Nat>::VAL == 621);
impl NatIndex for Idx<622> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<622> as NatIndex>::Out as Nat>::VAL == 622);
impl NatIndex for Idx<623> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<623> as NatIndex>::Out as Nat>::VAL == 623);
impl NatIndex for Idx<624> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<624> as NatIndex>::Out as Nat>::VAL == 624);
impl NatIndex for Idx<625> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<625> as NatIndex>::Out as Nat>::VAL == 625);
impl NatIndex for Idx<626> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<626> as NatIndex>::Out as Nat>::VAL == 626);
impl NatIndex for Idx<627> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<627> as NatIndex>::Out as Nat>::VAL == 627);
impl NatIndex for Idx<628> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<628> as NatIndex>::Out as Nat>::VAL == 628);
impl NatIndex for Idx<629> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<629> as NatIndex>::Out as Nat>::VAL == 629);
impl NatIndex for Idx<630> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<630> as NatIndex>::Out as Nat>::VAL == 630);
impl NatIndex for Idx<631> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<631> as NatIndex>::Out as Nat>::VAL == 631);
impl NatIndex for Idx<632> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<632> as NatIndex>::Out as Nat>::VAL == 632);
impl NatIndex for Idx<633> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<633> as NatIndex>::Out as Nat>::VAL == 633);
impl NatIndex for Idx<634> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<634> as NatIndex>::Out as Nat>::VAL == 634);
impl NatIndex for Idx<635> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<635> as NatIndex>::Out as Nat>::VAL == 635);
impl NatIndex for Idx<636> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<636> as NatIndex>::Out as Nat>::VAL == 636);
impl NatIndex for Idx<637> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<637> as NatIndex>::Out as Nat>::VAL == 637);
impl NatIndex for Idx<638> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<638> as NatIndex>::Out as Nat>::VAL == 638);
impl NatIndex for Idx<639> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<639> as NatIndex>::Out as Nat>::VAL == 639);
impl NatIndex for Idx<640> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<640> as NatIndex>::Out as Nat>::VAL == 640);
impl NatIndex for Idx<641> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<641> as NatIndex>::Out as Nat>::VAL == 641);
impl NatIndex for Idx<642> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<642> as NatIndex>::Out as Nat>::VAL == 642);
impl NatIndex for Idx<643> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<643> as NatIndex>::Out as Nat>::VAL == 643);
impl NatIndex for Idx<644> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<644> as NatIndex>::Out as Nat>::VAL == 644);
impl NatIndex for Idx<645> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<645> as NatIndex>::Out as Nat>::VAL == 645);
impl NatIndex for Idx<646> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<646> as NatIndex>::Out as Nat>::VAL == 646);
impl NatIndex for Idx<647> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<647> as NatIndex>::Out as Nat>::VAL == 647);
impl NatIndex for Idx<648> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<648> as NatIndex>::Out as Nat>::VAL == 648);
impl NatIndex for Idx<649> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<649> as NatIndex>::Out as Nat>::VAL == 649);
impl NatIndex for Idx<650> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<650> as NatIndex>::Out as Nat>::VAL == 650);
impl NatIndex for Idx<651> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<651> as NatIndex>::Out as Nat>::VAL == 651);
impl NatIndex for Idx<652> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<652> as NatIndex>::Out as Nat>::VAL == 652);
impl NatIndex for Idx<653> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<653> as NatIndex>::Out as Nat>::VAL == 653);
impl NatIndex for Idx<654> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<654> as NatIndex>::Out as Nat>::VAL == 654);
impl NatIndex for Idx<655> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<655> as NatIndex>::Out as Nat>::VAL == 655);
impl NatIndex for Idx<656> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<656> as NatIndex>::Out as Nat>::VAL == 656);
impl NatIndex for Idx<657> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<657> as NatIndex>::Out as Nat>::VAL == 657);
impl NatIndex for Idx<658> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<658> as NatIndex>::Out as Nat>::VAL == 658);
impl NatIndex for Idx<659> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<659> as NatIndex>::Out as Nat>::VAL == 659);
impl NatIndex for Idx<660> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<660> as NatIndex>::Out as Nat>::VAL == 660);
impl NatIndex for Idx<661> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<661> as NatIndex>::Out as Nat>::VAL == 661);
impl NatIndex for Idx<662> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<662> as NatIndex>::Out as Nat>::VAL == 662);
impl NatIndex for Idx<663> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<663> as NatIndex>::Out as Nat>::VAL == 663);
impl NatIndex for Idx<664> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<664> as NatIndex>::Out as Nat>::VAL == 664);
impl NatIndex for Idx<665> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<665> as NatIndex>::Out as Nat>::VAL == 665);
impl NatIndex for Idx<666> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<666> as NatIndex>::Out as Nat>::VAL == 666);
impl NatIndex for Idx<667> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<667> as NatIndex>::Out as Nat>::VAL == 667);
impl NatIndex for Idx<668> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<668> as NatIndex>::Out as Nat>::VAL == 668);
impl NatIndex for Idx<669> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<669> as NatIndex>::Out as Nat>::VAL == 669);
impl NatIndex for Idx<670> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<670> as NatIndex>::Out as Nat>::VAL == 670);
impl NatIndex for Idx<671> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<671> as NatIndex>::Out as Nat>::VAL == 671);
impl NatIndex for Idx<672> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<672> as NatIndex>::Out as Nat>::VAL == 672);
impl NatIndex for Idx<673> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<673> as NatIndex>::Out as Nat>::VAL == 673);
impl NatIndex for Idx<674> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<674> as NatIndex>::Out as Nat>::VAL == 674);
impl NatIndex for Idx<675> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<675> as NatIndex>::Out as Nat>::VAL == 675);
impl NatIndex for Idx<676> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<676> as NatIndex>::Out as Nat>::VAL == 676);
impl NatIndex for Idx<677> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<677> as NatIndex>::Out as Nat>::VAL == 677);
impl NatIndex for Idx<678> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<678> as NatIndex>::Out as Nat>::VAL == 678);
impl NatIndex for Idx<679> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<679> as NatIndex>::Out as Nat>::VAL == 679);
impl NatIndex for Idx<680> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<680> as NatIndex>::Out as Nat>::VAL == 680);
impl NatIndex for Idx<681> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<681> as NatIndex>::Out as Nat>::VAL == 681);
impl NatIndex for Idx<682> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<682> as NatIndex>::Out as Nat>::VAL == 682);
impl NatIndex for Idx<683> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<683> as NatIndex>::Out as Nat>::VAL == 683);
impl NatIndex for Idx<684> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<684> as NatIndex>::Out as Nat>::VAL == 684);
impl NatIndex for Idx<685> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<685> as NatIndex>::Out as Nat>::VAL == 685);
impl NatIndex for Idx<686> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<686> as NatIndex>::Out as Nat>::VAL == 686);
impl NatIndex for Idx<687> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<687> as NatIndex>::Out as Nat>::VAL == 687);
impl NatIndex for Idx<688> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<688> as NatIndex>::Out as Nat>::VAL == 688);
impl NatIndex for Idx<689> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<689> as NatIndex>::Out as Nat>::VAL == 689);
impl NatIndex for Idx<690> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<690> as NatIndex>::Out as Nat>::VAL == 690);
impl NatIndex for Idx<691> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<691> as NatIndex>::Out as Nat>::VAL == 691);
impl NatIndex for Idx<692> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<692> as NatIndex>::Out as Nat>::VAL == 692);
impl NatIndex for Idx<693> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<693> as NatIndex>::Out as Nat>::VAL == 693);
impl NatIndex for Idx<694> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<694> as NatIndex>::Out as Nat>::VAL == 694);
impl NatIndex for Idx<695> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<695> as NatIndex>::Out as Nat>::VAL == 695);
impl NatIndex for Idx<696> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<696> as NatIndex>::Out as Nat>::VAL == 696);
impl NatIndex for Idx<697> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<697> as NatIndex>::Out as Nat>::VAL == 697);
impl NatIndex for Idx<698> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<698> as NatIndex>::Out as Nat>::VAL == 698);
impl NatIndex for Idx<699> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<699> as NatIndex>::Out as Nat>::VAL == 699);
impl NatIndex for Idx<700> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<700> as NatIndex>::Out as Nat>::VAL == 700);
impl NatIndex for Idx<701> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<701> as NatIndex>::Out as Nat>::VAL == 701);
impl NatIndex for Idx<702> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<702> as NatIndex>::Out as Nat>::VAL == 702);
impl NatIndex for Idx<703> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<703> as NatIndex>::Out as Nat>::VAL == 703);
impl NatIndex for Idx<704> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<704> as NatIndex>::Out as Nat>::VAL == 704);
impl NatIndex for Idx<705> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<705> as NatIndex>::Out as Nat>::VAL == 705);
impl NatIndex for Idx<706> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<706> as NatIndex>::Out as Nat>::VAL == 706);
impl NatIndex for Idx<707> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<707> as NatIndex>::Out as Nat>::VAL == 707);
impl NatIndex for Idx<708> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<708> as NatIndex>::Out as Nat>::VAL == 708);
impl NatIndex for Idx<709> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<709> as NatIndex>::Out as Nat>::VAL == 709);
impl NatIndex for Idx<710> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<710> as NatIndex>::Out as Nat>::VAL == 710);
impl NatIndex for Idx<711> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<711> as NatIndex>::Out as Nat>::VAL == 711);
impl NatIndex for Idx<712> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<712> as NatIndex>::Out as Nat>::VAL == 712);
impl NatIndex for Idx<713> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<713> as NatIndex>::Out as Nat>::VAL == 713);
impl NatIndex for Idx<714> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<714> as NatIndex>::Out as Nat>::VAL == 714);
impl NatIndex for Idx<715> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<715> as NatIndex>::Out as Nat>::VAL == 715);
impl NatIndex for Idx<716> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<716> as NatIndex>::Out as Nat>::VAL == 716);
impl NatIndex for Idx<717> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<717> as NatIndex>::Out as Nat>::VAL == 717);
impl NatIndex for Idx<718> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<718> as NatIndex>::Out as Nat>::VAL == 718);
impl NatIndex for Idx<719> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<719> as NatIndex>::Out as Nat>::VAL == 719);
impl NatIndex for Idx<720> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<720> as NatIndex>::Out as Nat>::VAL == 720);
impl NatIndex for Idx<721> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<721> as NatIndex>::Out as Nat>::VAL == 721);
impl NatIndex for Idx<722> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<722> as NatIndex>::Out as Nat>::VAL == 722);
impl NatIndex for Idx<723> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<723> as NatIndex>::Out as Nat>::VAL == 723);
impl NatIndex for Idx<724> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<724> as NatIndex>::Out as Nat>::VAL == 724);
impl NatIndex for Idx<725> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<725> as NatIndex>::Out as Nat>::VAL == 725);
impl NatIndex for Idx<726> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<726> as NatIndex>::Out as Nat>::VAL == 726);
impl NatIndex for Idx<727> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<727> as NatIndex>::Out as Nat>::VAL == 727);
impl NatIndex for Idx<728> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<728> as NatIndex>::Out as Nat>::VAL == 728);
impl NatIndex for Idx<729> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<729> as NatIndex>::Out as Nat>::VAL == 729);
impl NatIndex for Idx<730> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<730> as NatIndex>::Out as Nat>::VAL == 730);
impl NatIndex for Idx<731> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<731> as NatIndex>::Out as Nat>::VAL == 731);
impl NatIndex for Idx<732> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<732> as NatIndex>::Out as Nat>::VAL == 732);
impl NatIndex for Idx<733> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<733> as NatIndex>::Out as Nat>::VAL == 733);
impl NatIndex for Idx<734> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<734> as NatIndex>::Out as Nat>::VAL == 734);
impl NatIndex for Idx<735> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<735> as NatIndex>::Out as Nat>::VAL == 735);
impl NatIndex for Idx<736> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<736> as NatIndex>::Out as Nat>::VAL == 736);
impl NatIndex for Idx<737> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<737> as NatIndex>::Out as Nat>::VAL == 737);
impl NatIndex for Idx<738> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<738> as NatIndex>::Out as Nat>::VAL == 738);
impl NatIndex for Idx<739> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<739> as NatIndex>::Out as Nat>::VAL == 739);
impl NatIndex for Idx<740> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<740> as NatIndex>::Out as Nat>::VAL == 740);
impl NatIndex for Idx<741> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<741> as NatIndex>::Out as Nat>::VAL == 741);
impl NatIndex for Idx<742> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<742> as NatIndex>::Out as Nat>::VAL == 742);
impl NatIndex for Idx<743> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<743> as NatIndex>::Out as Nat>::VAL == 743);
impl NatIndex for Idx<744> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<744> as NatIndex>::Out as Nat>::VAL == 744);
impl NatIndex for Idx<745> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<745> as NatIndex>::Out as Nat>::VAL == 745);
impl NatIndex for Idx<746> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<746> as NatIndex>::Out as Nat>::VAL == 746);
impl NatIndex for Idx<747> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<747> as NatIndex>::Out as Nat>::VAL == 747);
impl NatIndex for Idx<748> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<748> as NatIndex>::Out as Nat>::VAL == 748);
impl NatIndex for Idx<749> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<749> as NatIndex>::Out as Nat>::VAL == 749);
impl NatIndex for Idx<750> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<750> as NatIndex>::Out as Nat>::VAL == 750);
impl NatIndex for Idx<751> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<751> as NatIndex>::Out as Nat>::VAL == 751);
impl NatIndex for Idx<752> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<752> as NatIndex>::Out as Nat>::VAL == 752);
impl NatIndex for Idx<753> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<753> as NatIndex>::Out as Nat>::VAL == 753);
impl NatIndex for Idx<754> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<754> as NatIndex>::Out as Nat>::VAL == 754);
impl NatIndex for Idx<755> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<755> as NatIndex>::Out as Nat>::VAL == 755);
impl NatIndex for Idx<756> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<756> as NatIndex>::Out as Nat>::VAL == 756);
impl NatIndex for Idx<757> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<757> as NatIndex>::Out as Nat>::VAL == 757);
impl NatIndex for Idx<758> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<758> as NatIndex>::Out as Nat>::VAL == 758);
impl NatIndex for Idx<759> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<759> as NatIndex>::Out as Nat>::VAL == 759);
impl NatIndex for Idx<760> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<760> as NatIndex>::Out as Nat>::VAL == 760);
impl NatIndex for Idx<761> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<761> as NatIndex>::Out as Nat>::VAL == 761);
impl NatIndex for Idx<762> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<762> as NatIndex>::Out as Nat>::VAL == 762);
impl NatIndex for Idx<763> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<763> as NatIndex>::Out as Nat>::VAL == 763);
impl NatIndex for Idx<764> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<764> as NatIndex>::Out as Nat>::VAL == 764);
impl NatIndex for Idx<765> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<765> as NatIndex>::Out as Nat>::VAL == 765);
impl NatIndex for Idx<766> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<766> as NatIndex>::Out as Nat>::VAL == 766);
impl NatIndex for Idx<767> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<767> as NatIndex>::Out as Nat>::VAL == 767);
impl NatIndex for Idx<768> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<768> as NatIndex>::Out as Nat>::VAL == 768);
impl NatIndex for Idx<769> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<769> as NatIndex>::Out as Nat>::VAL == 769);
impl NatIndex for Idx<770> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<770> as NatIndex>::Out as Nat>::VAL == 770);
impl NatIndex for Idx<771> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<771> as NatIndex>::Out as Nat>::VAL == 771);
impl NatIndex for Idx<772> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<772> as NatIndex>::Out as Nat>::VAL == 772);
impl NatIndex for Idx<773> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<773> as NatIndex>::Out as Nat>::VAL == 773);
impl NatIndex for Idx<774> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<774> as NatIndex>::Out as Nat>::VAL == 774);
impl NatIndex for Idx<775> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<775> as NatIndex>::Out as Nat>::VAL == 775);
impl NatIndex for Idx<776> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<776> as NatIndex>::Out as Nat>::VAL == 776);
impl NatIndex for Idx<777> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<777> as NatIndex>::Out as Nat>::VAL == 777);
impl NatIndex for Idx<778> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<778> as NatIndex>::Out as Nat>::VAL == 778);
impl NatIndex for Idx<779> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<779> as NatIndex>::Out as Nat>::VAL == 779);
impl NatIndex for Idx<780> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<780> as NatIndex>::Out as Nat>::VAL == 780);
impl NatIndex for Idx<781> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<781> as NatIndex>::Out as Nat>::VAL == 781);
impl NatIndex for Idx<782> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<782> as NatIndex>::Out as Nat>::VAL == 782);
impl NatIndex for Idx<783> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<783> as NatIndex>::Out as Nat>::VAL == 783);
impl NatIndex for Idx<784> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<784> as NatIndex>::Out as Nat>::VAL == 784);
impl NatIndex for Idx<785> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<785> as NatIndex>::Out as Nat>::VAL == 785);
impl NatIndex for Idx<786> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<786> as NatIndex>::Out as Nat>::VAL == 786);
impl NatIndex for Idx<787> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<787> as NatIndex>::Out as Nat>::VAL == 787);
impl NatIndex for Idx<788> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<788> as NatIndex>::Out as Nat>::VAL == 788);
impl NatIndex for Idx<789> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<789> as NatIndex>::Out as Nat>::VAL == 789);
impl NatIndex for Idx<790> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<790> as NatIndex>::Out as Nat>::VAL == 790);
impl NatIndex for Idx<791> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<791> as NatIndex>::Out as Nat>::VAL == 791);
impl NatIndex for Idx<792> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<792> as NatIndex>::Out as Nat>::VAL == 792);
impl NatIndex for Idx<793> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<793> as NatIndex>::Out as Nat>::VAL == 793);
impl NatIndex for Idx<794> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<794> as NatIndex>::Out as Nat>::VAL == 794);
impl NatIndex for Idx<795> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<795> as NatIndex>::Out as Nat>::VAL == 795);
impl NatIndex for Idx<796> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<796> as NatIndex>::Out as Nat>::VAL == 796);
impl NatIndex for Idx<797> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<797> as NatIndex>::Out as Nat>::VAL == 797);
impl NatIndex for Idx<798> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<798> as NatIndex>::Out as Nat>::VAL == 798);
impl NatIndex for Idx<799> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<799> as NatIndex>::Out as Nat>::VAL == 799);
impl NatIndex for Idx<800> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<800> as NatIndex>::Out as Nat>::VAL == 800);
impl NatIndex for Idx<801> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<801> as NatIndex>::Out as Nat>::VAL == 801);
impl NatIndex for Idx<802> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<802> as NatIndex>::Out as Nat>::VAL == 802);
impl NatIndex for Idx<803> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<803> as NatIndex>::Out as Nat>::VAL == 803);
impl NatIndex for Idx<804> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<804> as NatIndex>::Out as Nat>::VAL == 804);
impl NatIndex for Idx<805> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<805> as NatIndex>::Out as Nat>::VAL == 805);
impl NatIndex for Idx<806> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<806> as NatIndex>::Out as Nat>::VAL == 806);
impl NatIndex for Idx<807> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<807> as NatIndex>::Out as Nat>::VAL == 807);
impl NatIndex for Idx<808> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<808> as NatIndex>::Out as Nat>::VAL == 808);
impl NatIndex for Idx<809> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<809> as NatIndex>::Out as Nat>::VAL == 809);
impl NatIndex for Idx<810> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<810> as NatIndex>::Out as Nat>::VAL == 810);
impl NatIndex for Idx<811> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<811> as NatIndex>::Out as Nat>::VAL == 811);
impl NatIndex for Idx<812> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<812> as NatIndex>::Out as Nat>::VAL == 812);
impl NatIndex for Idx<813> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<813> as NatIndex>::Out as Nat>::VAL == 813);
impl NatIndex for Idx<814> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<814> as NatIndex>::Out as Nat>::VAL == 814);
impl NatIndex for Idx<815> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<815> as NatIndex>::Out as Nat>::VAL == 815);
impl NatIndex for Idx<816> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<816> as NatIndex>::Out as Nat>::VAL == 816);
impl NatIndex for Idx<817> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<817> as NatIndex>::Out as Nat>::VAL == 817);
impl NatIndex for Idx<818> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<818> as NatIndex>::Out as Nat>::VAL == 818);
impl NatIndex for Idx<819> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<819> as NatIndex>::Out as Nat>::VAL == 819);
impl NatIndex for Idx<820> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<820> as NatIndex>::Out as Nat>::VAL == 820);
impl NatIndex for Idx<821> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<821> as NatIndex>::Out as Nat>::VAL == 821);
impl NatIndex for Idx<822> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<822> as NatIndex>::Out as Nat>::VAL == 822);
impl NatIndex for Idx<823> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<823> as NatIndex>::Out as Nat>::VAL == 823);
impl NatIndex for Idx<824> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<824> as NatIndex>::Out as Nat>::VAL == 824);
impl NatIndex for Idx<825> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<825> as NatIndex>::Out as Nat>::VAL == 825);
impl NatIndex for Idx<826> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<826> as NatIndex>::Out as Nat>::VAL == 826);
impl NatIndex for Idx<827> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<827> as NatIndex>::Out as Nat>::VAL == 827);
impl NatIndex for Idx<828> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<828> as NatIndex>::Out as Nat>::VAL == 828);
impl NatIndex for Idx<829> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<829> as NatIndex>::Out as Nat>::VAL == 829);
impl NatIndex for Idx<830> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<830> as NatIndex>::Out as Nat>::VAL == 830);
impl NatIndex for Idx<831> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<831> as NatIndex>::Out as Nat>::VAL == 831);
impl NatIndex for Idx<832> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<832> as NatIndex>::Out as Nat>::VAL == 832);
impl NatIndex for Idx<833> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<833> as NatIndex>::Out as Nat>::VAL == 833);
impl NatIndex for Idx<834> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<834> as NatIndex>::Out as Nat>::VAL == 834);
impl NatIndex for Idx<835> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<835> as NatIndex>::Out as Nat>::VAL == 835);
impl NatIndex for Idx<836> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<836> as NatIndex>::Out as Nat>::VAL == 836);
impl NatIndex for Idx<837> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<837> as NatIndex>::Out as Nat>::VAL == 837);
impl NatIndex for Idx<838> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<838> as NatIndex>::Out as Nat>::VAL == 838);
impl NatIndex for Idx<839> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<839> as NatIndex>::Out as Nat>::VAL == 839);
impl NatIndex for Idx<840> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<840> as NatIndex>::Out as Nat>::VAL == 840);
impl NatIndex for Idx<841> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<841> as NatIndex>::Out as Nat>::VAL == 841);
impl NatIndex for Idx<842> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<842> as NatIndex>::Out as Nat>::VAL == 842);
impl NatIndex for Idx<843> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<843> as NatIndex>::Out as Nat>::VAL == 843);
impl NatIndex for Idx<844> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<844> as NatIndex>::Out as Nat>::VAL == 844);
impl NatIndex for Idx<845> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<845> as NatIndex>::Out as Nat>::VAL == 845);
impl NatIndex for Idx<846> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<846> as NatIndex>::Out as Nat>::VAL == 846);
impl NatIndex for Idx<847> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<847> as NatIndex>::Out as Nat>::VAL == 847);
impl NatIndex for Idx<848> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<848> as NatIndex>::Out as Nat>::VAL == 848);
impl NatIndex for Idx<849> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<849> as NatIndex>::Out as Nat>::VAL == 849);
impl NatIndex for Idx<850> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<850> as NatIndex>::Out as Nat>::VAL == 850);
impl NatIndex for Idx<851> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<851> as NatIndex>::Out as Nat>::VAL == 851);
impl NatIndex for Idx<852> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<852> as NatIndex>::Out as Nat>::VAL == 852);
impl NatIndex for Idx<853> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<853> as NatIndex>::Out as Nat>::VAL == 853);
impl NatIndex for Idx<854> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<854> as NatIndex>::Out as Nat>::VAL == 854);
impl NatIndex for Idx<855> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<855> as NatIndex>::Out as Nat>::VAL == 855);
impl NatIndex for Idx<856> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<856> as NatIndex>::Out as Nat>::VAL == 856);
impl NatIndex for Idx<857> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<857> as NatIndex>::Out as Nat>::VAL == 857);
impl NatIndex for Idx<858> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<858> as NatIndex>::Out as Nat>::VAL == 858);
impl NatIndex for Idx<859> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<859> as NatIndex>::Out as Nat>::VAL == 859);
impl NatIndex for Idx<860> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<860> as NatIndex>::Out as Nat>::VAL == 860);
impl NatIndex for Idx<861> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<861> as NatIndex>::Out as Nat>::VAL == 861);
impl NatIndex for Idx<862> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<862> as NatIndex>::Out as Nat>::VAL == 862);
impl NatIndex for Idx<863> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<863> as NatIndex>::Out as Nat>::VAL == 863);
impl NatIndex for Idx<864> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<864> as NatIndex>::Out as Nat>::VAL == 864);
impl NatIndex for Idx<865> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<865> as NatIndex>::Out as Nat>::VAL == 865);
impl NatIndex for Idx<866> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<866> as NatIndex>::Out as Nat>::VAL == 866);
impl NatIndex for Idx<867> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<867> as NatIndex>::Out as Nat>::VAL == 867);
impl NatIndex for Idx<868> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<868> as NatIndex>::Out as Nat>::VAL == 868);
impl NatIndex for Idx<869> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<869> as NatIndex>::Out as Nat>::VAL == 869);
impl NatIndex for Idx<870> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<870> as NatIndex>::Out as Nat>::VAL == 870);
impl NatIndex for Idx<871> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<871> as NatIndex>::Out as Nat>::VAL == 871);
impl NatIndex for Idx<872> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<872> as NatIndex>::Out as Nat>::VAL == 872);
impl NatIndex for Idx<873> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<873> as NatIndex>::Out as Nat>::VAL == 873);
impl NatIndex for Idx<874> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<874> as NatIndex>::Out as Nat>::VAL == 874);
impl NatIndex for Idx<875> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<875> as NatIndex>::Out as Nat>::VAL == 875);
impl NatIndex for Idx<876> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<876> as NatIndex>::Out as Nat>::VAL == 876);
impl NatIndex for Idx<877> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<877> as NatIndex>::Out as Nat>::VAL == 877);
impl NatIndex for Idx<878> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<878> as NatIndex>::Out as Nat>::VAL == 878);
impl NatIndex for Idx<879> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<879> as NatIndex>::Out as Nat>::VAL == 879);
impl NatIndex for Idx<880> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<880> as NatIndex>::Out as Nat>::VAL == 880);
impl NatIndex for Idx<881> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<881> as NatIndex>::Out as Nat>::VAL == 881);
impl NatIndex for Idx<882> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<882> as NatIndex>::Out as Nat>::VAL == 882);
impl NatIndex for Idx<883> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<883> as NatIndex>::Out as Nat>::VAL == 883);
impl NatIndex for Idx<884> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<884> as NatIndex>::Out as Nat>::VAL == 884);
impl NatIndex for Idx<885> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<885> as NatIndex>::Out as Nat>::VAL == 885);
impl NatIndex for Idx<886> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<886> as NatIndex>::Out as Nat>::VAL == 886);
impl NatIndex for Idx<887> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<887> as NatIndex>::Out as Nat>::VAL == 887);
impl NatIndex for Idx<888> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<888> as NatIndex>::Out as Nat>::VAL == 888);
impl NatIndex for Idx<889> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<889> as NatIndex>::Out as Nat>::VAL == 889);
impl NatIndex for Idx<890> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<890> as NatIndex>::Out as Nat>::VAL == 890);
impl NatIndex for Idx<891> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<891> as NatIndex>::Out as Nat>::VAL == 891);
impl NatIndex for Idx<892> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<892> as NatIndex>::Out as Nat>::VAL == 892);
impl NatIndex for Idx<893> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<893> as NatIndex>::Out as Nat>::VAL == 893);
impl NatIndex for Idx<894> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<894> as NatIndex>::Out as Nat>::VAL == 894);
impl NatIndex for Idx<895> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<895> as NatIndex>::Out as Nat>::VAL == 895);
impl NatIndex for Idx<896> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<896> as NatIndex>::Out as Nat>::VAL == 896);
impl NatIndex for Idx<897> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<897> as NatIndex>::Out as Nat>::VAL == 897);
impl NatIndex for Idx<898> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<898> as NatIndex>::Out as Nat>::VAL == 898);
impl NatIndex for Idx<899> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<899> as NatIndex>::Out as Nat>::VAL == 899);
impl NatIndex for Idx<900> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<900> as NatIndex>::Out as Nat>::VAL == 900);
impl NatIndex for Idx<901> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<901> as NatIndex>::Out as Nat>::VAL == 901);
impl NatIndex for Idx<902> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<902> as NatIndex>::Out as Nat>::VAL == 902);
impl NatIndex for Idx<903> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<903> as NatIndex>::Out as Nat>::VAL == 903);
impl NatIndex for Idx<904> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<904> as NatIndex>::Out as Nat>::VAL == 904);
impl NatIndex for Idx<905> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<905> as NatIndex>::Out as Nat>::VAL == 905);
impl NatIndex for Idx<906> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<906> as NatIndex>::Out as Nat>::VAL == 906);
impl NatIndex for Idx<907> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<907> as NatIndex>::Out as Nat>::VAL == 907);
impl NatIndex for Idx<908> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<908> as NatIndex>::Out as Nat>::VAL == 908);
impl NatIndex for Idx<909> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<909> as NatIndex>::Out as Nat>::VAL == 909);
impl NatIndex for Idx<910> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<910> as NatIndex>::Out as Nat>::VAL == 910);
impl NatIndex for Idx<911> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<911> as NatIndex>::Out as Nat>::VAL == 911);
impl NatIndex for Idx<912> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<912> as NatIndex>::Out as Nat>::VAL == 912);
impl NatIndex for Idx<913> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<913> as NatIndex>::Out as Nat>::VAL == 913);
impl NatIndex for Idx<914> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<914> as NatIndex>::Out as Nat>::VAL == 914);
impl NatIndex for Idx<915> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<915> as NatIndex>::Out as Nat>::VAL == 915);
impl NatIndex for Idx<916> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<916> as NatIndex>::Out as Nat>::VAL == 916);
impl NatIndex for Idx<917> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<917> as NatIndex>::Out as Nat>::VAL == 917);
impl NatIndex for Idx<918> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<918> as NatIndex>::Out as Nat>::VAL == 918);
impl NatIndex for Idx<919> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<919> as NatIndex>::Out as Nat>::VAL == 919);
impl NatIndex for Idx<920> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<920> as NatIndex>::Out as Nat>::VAL == 920);
impl NatIndex for Idx<921> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<921> as NatIndex>::Out as Nat>::VAL == 921);
impl NatIndex for Idx<922> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<922> as NatIndex>::Out as Nat>::VAL == 922);
impl NatIndex for Idx<923> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<923> as NatIndex>::Out as Nat>::VAL == 923);
impl NatIndex for Idx<924> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<924> as NatIndex>::Out as Nat>::VAL == 924);
impl NatIndex for Idx<925> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<925> as NatIndex>::Out as Nat>::VAL == 925);
impl NatIndex for Idx<926> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<926> as NatIndex>::Out as Nat>::VAL == 926);
impl NatIndex for Idx<927> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<927> as NatIndex>::Out as Nat>::VAL == 927);
impl NatIndex for Idx<928> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<928> as NatIndex>::Out as Nat>::VAL == 928);
impl NatIndex for Idx<929> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<929> as NatIndex>::Out as Nat>::VAL == 929);
impl NatIndex for Idx<930> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<930> as NatIndex>::Out as Nat>::VAL == 930);
impl NatIndex for Idx<931> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<931> as NatIndex>::Out as Nat>::VAL == 931);
impl NatIndex for Idx<932> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<932> as NatIndex>::Out as Nat>::VAL == 932);
impl NatIndex for Idx<933> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<933> as NatIndex>::Out as Nat>::VAL == 933);
impl NatIndex for Idx<934> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<934> as NatIndex>::Out as Nat>::VAL == 934);
impl NatIndex for Idx<935> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<935> as NatIndex>::Out as Nat>::VAL == 935);
impl NatIndex for Idx<936> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<936> as NatIndex>::Out as Nat>::VAL == 936);
impl NatIndex for Idx<937> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<937> as NatIndex>::Out as Nat>::VAL == 937);
impl NatIndex for Idx<938> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<938> as NatIndex>::Out as Nat>::VAL == 938);
impl NatIndex for Idx<939> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<939> as NatIndex>::Out as Nat>::VAL == 939);
impl NatIndex for Idx<940> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<940> as NatIndex>::Out as Nat>::VAL == 940);
impl NatIndex for Idx<941> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<941> as NatIndex>::Out as Nat>::VAL == 941);
impl NatIndex for Idx<942> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<942> as NatIndex>::Out as Nat>::VAL == 942);
impl NatIndex for Idx<943> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<943> as NatIndex>::Out as Nat>::VAL == 943);
impl NatIndex for Idx<944> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<944> as NatIndex>::Out as Nat>::VAL == 944);
impl NatIndex for Idx<945> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<945> as NatIndex>::Out as Nat>::VAL == 945);
impl NatIndex for Idx<946> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<946> as NatIndex>::Out as Nat>::VAL == 946);
impl NatIndex for Idx<947> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<947> as NatIndex>::Out as Nat>::VAL == 947);
impl NatIndex for Idx<948> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<948> as NatIndex>::Out as Nat>::VAL == 948);
impl NatIndex for Idx<949> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<949> as NatIndex>::Out as Nat>::VAL == 949);
impl NatIndex for Idx<950> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<950> as NatIndex>::Out as Nat>::VAL == 950);
impl NatIndex for Idx<951> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<951> as NatIndex>::Out as Nat>::VAL == 951);
impl NatIndex for Idx<952> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<952> as NatIndex>::Out as Nat>::VAL == 952);
impl NatIndex for Idx<953> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<953> as NatIndex>::Out as Nat>::VAL == 953);
impl NatIndex for Idx<954> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<954> as NatIndex>::Out as Nat>::VAL == 954);
impl NatIndex for Idx<955> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<955> as NatIndex>::Out as Nat>::VAL == 955);
impl NatIndex for Idx<956> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<956> as NatIndex>::Out as Nat>::VAL == 956);
impl NatIndex for Idx<957> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<957> as NatIndex>::Out as Nat>::VAL == 957);
impl NatIndex for Idx<958> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<958> as NatIndex>::Out as Nat>::VAL == 958);
impl NatIndex for Idx<959> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<959> as NatIndex>::Out as Nat>::VAL == 959);
impl NatIndex for Idx<960> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<960> as NatIndex>::Out as Nat>::VAL == 960);
impl NatIndex for Idx<961> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<961> as NatIndex>::Out as Nat>::VAL == 961);
impl NatIndex for Idx<962> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<962> as NatIndex>::Out as Nat>::VAL == 962);
impl NatIndex for Idx<963> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<963> as NatIndex>::Out as Nat>::VAL == 963);
impl NatIndex for Idx<964> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<964> as NatIndex>::Out as Nat>::VAL == 964);
impl NatIndex for Idx<965> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<965> as NatIndex>::Out as Nat>::VAL == 965);
impl NatIndex for Idx<966> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<966> as NatIndex>::Out as Nat>::VAL == 966);
impl NatIndex for Idx<967> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<967> as NatIndex>::Out as Nat>::VAL == 967);
impl NatIndex for Idx<968> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<968> as NatIndex>::Out as Nat>::VAL == 968);
impl NatIndex for Idx<969> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<969> as NatIndex>::Out as Nat>::VAL == 969);
impl NatIndex for Idx<970> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<970> as NatIndex>::Out as Nat>::VAL == 970);
impl NatIndex for Idx<971> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<971> as NatIndex>::Out as Nat>::VAL == 971);
impl NatIndex for Idx<972> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<972> as NatIndex>::Out as Nat>::VAL == 972);
impl NatIndex for Idx<973> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<973> as NatIndex>::Out as Nat>::VAL == 973);
impl NatIndex for Idx<974> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<974> as NatIndex>::Out as Nat>::VAL == 974);
impl NatIndex for Idx<975> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<975> as NatIndex>::Out as Nat>::VAL == 975);
impl NatIndex for Idx<976> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<976> as NatIndex>::Out as Nat>::VAL == 976);
impl NatIndex for Idx<977> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<977> as NatIndex>::Out as Nat>::VAL == 977);
impl NatIndex for Idx<978> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<978> as NatIndex>::Out as Nat>::VAL == 978);
impl NatIndex for Idx<979> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<979> as NatIndex>::Out as Nat>::VAL == 979);
impl NatIndex for Idx<980> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<980> as NatIndex>::Out as Nat>::VAL == 980);
impl NatIndex for Idx<981> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<981> as NatIndex>::Out as Nat>::VAL == 981);
impl NatIndex for Idx<982> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<982> as NatIndex>::Out as Nat>::VAL == 982);
impl NatIndex for Idx<983> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<983> as NatIndex>::Out as Nat>::VAL == 983);
impl NatIndex for Idx<984> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<984> as NatIndex>::Out as Nat>::VAL == 984);
impl NatIndex for Idx<985> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<985> as NatIndex>::Out as Nat>::VAL == 985);
impl NatIndex for Idx<986> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<986> as NatIndex>::Out as Nat>::VAL == 986);
impl NatIndex for Idx<987> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<987> as NatIndex>::Out as Nat>::VAL == 987);
impl NatIndex for Idx<988> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<988> as NatIndex>::Out as Nat>::VAL == 988);
impl NatIndex for Idx<989> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<989> as NatIndex>::Out as Nat>::VAL == 989);
impl NatIndex for Idx<990> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<990> as NatIndex>::Out as Nat>::VAL == 990);
impl NatIndex for Idx<991> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<991> as NatIndex>::Out as Nat>::VAL == 991);
impl NatIndex for Idx<992> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<992> as NatIndex>::Out as Nat>::VAL == 992);
impl NatIndex for Idx<993> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<993> as NatIndex>::Out as Nat>::VAL == 993);
impl NatIndex for Idx<994> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<994> as NatIndex>::Out as Nat>::VAL == 994);
impl NatIndex for Idx<995> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<995> as NatIndex>::Out as Nat>::VAL == 995);
impl NatIndex for Idx<996> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<996> as NatIndex>::Out as Nat>::VAL == 996);
impl NatIndex for Idx<997> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<997> as NatIndex>::Out as Nat>::VAL == 997);
impl NatIndex for Idx<998> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<998> as NatIndex>::Out as Nat>::VAL == 998);
impl NatIndex for Idx<999> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<999> as NatIndex>::Out as Nat>::VAL == 999);
impl NatIndex for Idx<1000> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1000> as NatIndex>::Out as Nat>::VAL == 1000);
impl NatIndex for Idx<1001> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1001> as NatIndex>::Out as Nat>::VAL == 1001);
impl NatIndex for Idx<1002> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1002> as NatIndex>::Out as Nat>::VAL == 1002);
impl NatIndex for Idx<1003> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1003> as NatIndex>::Out as Nat>::VAL == 1003);
impl NatIndex for Idx<1004> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1004> as NatIndex>::Out as Nat>::VAL == 1004);
impl NatIndex for Idx<1005> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1005> as NatIndex>::Out as Nat>::VAL == 1005);
impl NatIndex for Idx<1006> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1006> as NatIndex>::Out as Nat>::VAL == 1006);
impl NatIndex for Idx<1007> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1007> as NatIndex>::Out as Nat>::VAL == 1007);
impl NatIndex for Idx<1008> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1008> as NatIndex>::Out as Nat>::VAL == 1008);
impl NatIndex for Idx<1009> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1009> as NatIndex>::Out as Nat>::VAL == 1009);
impl NatIndex for Idx<1010> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1010> as NatIndex>::Out as Nat>::VAL == 1010);
impl NatIndex for Idx<1011> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1011> as NatIndex>::Out as Nat>::VAL == 1011);
impl NatIndex for Idx<1012> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1012> as NatIndex>::Out as Nat>::VAL == 1012);
impl NatIndex for Idx<1013> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1013> as NatIndex>::Out as Nat>::VAL == 1013);
impl NatIndex for Idx<1014> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1014> as NatIndex>::Out as Nat>::VAL == 1014);
impl NatIndex for Idx<1015> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1015> as NatIndex>::Out as Nat>::VAL == 1015);
impl NatIndex for Idx<1016> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1016> as NatIndex>::Out as Nat>::VAL == 1016);
impl NatIndex for Idx<1017> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1017> as NatIndex>::Out as Nat>::VAL == 1017);
impl NatIndex for Idx<1018> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1018> as NatIndex>::Out as Nat>::VAL == 1018);
impl NatIndex for Idx<1019> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1019> as NatIndex>::Out as Nat>::VAL == 1019);
impl NatIndex for Idx<1020> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1020> as NatIndex>::Out as Nat>::VAL == 1020);
impl NatIndex for Idx<1021> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1021> as NatIndex>::Out as Nat>::VAL == 1021);
impl NatIndex for Idx<1022> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1022> as NatIndex>::Out as Nat>::VAL == 1022);
impl NatIndex for Idx<1023> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1023> as NatIndex>::Out as Nat>::VAL == 1023);

// the load-bearing claim
const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
