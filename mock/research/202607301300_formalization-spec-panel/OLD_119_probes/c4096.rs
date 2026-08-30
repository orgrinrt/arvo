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
impl NatIndex for Idx<1024> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1024> as NatIndex>::Out as Nat>::VAL == 1024);
impl NatIndex for Idx<1025> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1025> as NatIndex>::Out as Nat>::VAL == 1025);
impl NatIndex for Idx<1026> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1026> as NatIndex>::Out as Nat>::VAL == 1026);
impl NatIndex for Idx<1027> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1027> as NatIndex>::Out as Nat>::VAL == 1027);
impl NatIndex for Idx<1028> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1028> as NatIndex>::Out as Nat>::VAL == 1028);
impl NatIndex for Idx<1029> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1029> as NatIndex>::Out as Nat>::VAL == 1029);
impl NatIndex for Idx<1030> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1030> as NatIndex>::Out as Nat>::VAL == 1030);
impl NatIndex for Idx<1031> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1031> as NatIndex>::Out as Nat>::VAL == 1031);
impl NatIndex for Idx<1032> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1032> as NatIndex>::Out as Nat>::VAL == 1032);
impl NatIndex for Idx<1033> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1033> as NatIndex>::Out as Nat>::VAL == 1033);
impl NatIndex for Idx<1034> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1034> as NatIndex>::Out as Nat>::VAL == 1034);
impl NatIndex for Idx<1035> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1035> as NatIndex>::Out as Nat>::VAL == 1035);
impl NatIndex for Idx<1036> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1036> as NatIndex>::Out as Nat>::VAL == 1036);
impl NatIndex for Idx<1037> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1037> as NatIndex>::Out as Nat>::VAL == 1037);
impl NatIndex for Idx<1038> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1038> as NatIndex>::Out as Nat>::VAL == 1038);
impl NatIndex for Idx<1039> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1039> as NatIndex>::Out as Nat>::VAL == 1039);
impl NatIndex for Idx<1040> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1040> as NatIndex>::Out as Nat>::VAL == 1040);
impl NatIndex for Idx<1041> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1041> as NatIndex>::Out as Nat>::VAL == 1041);
impl NatIndex for Idx<1042> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1042> as NatIndex>::Out as Nat>::VAL == 1042);
impl NatIndex for Idx<1043> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1043> as NatIndex>::Out as Nat>::VAL == 1043);
impl NatIndex for Idx<1044> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1044> as NatIndex>::Out as Nat>::VAL == 1044);
impl NatIndex for Idx<1045> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1045> as NatIndex>::Out as Nat>::VAL == 1045);
impl NatIndex for Idx<1046> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1046> as NatIndex>::Out as Nat>::VAL == 1046);
impl NatIndex for Idx<1047> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1047> as NatIndex>::Out as Nat>::VAL == 1047);
impl NatIndex for Idx<1048> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1048> as NatIndex>::Out as Nat>::VAL == 1048);
impl NatIndex for Idx<1049> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1049> as NatIndex>::Out as Nat>::VAL == 1049);
impl NatIndex for Idx<1050> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1050> as NatIndex>::Out as Nat>::VAL == 1050);
impl NatIndex for Idx<1051> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1051> as NatIndex>::Out as Nat>::VAL == 1051);
impl NatIndex for Idx<1052> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1052> as NatIndex>::Out as Nat>::VAL == 1052);
impl NatIndex for Idx<1053> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1053> as NatIndex>::Out as Nat>::VAL == 1053);
impl NatIndex for Idx<1054> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1054> as NatIndex>::Out as Nat>::VAL == 1054);
impl NatIndex for Idx<1055> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1055> as NatIndex>::Out as Nat>::VAL == 1055);
impl NatIndex for Idx<1056> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1056> as NatIndex>::Out as Nat>::VAL == 1056);
impl NatIndex for Idx<1057> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1057> as NatIndex>::Out as Nat>::VAL == 1057);
impl NatIndex for Idx<1058> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1058> as NatIndex>::Out as Nat>::VAL == 1058);
impl NatIndex for Idx<1059> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1059> as NatIndex>::Out as Nat>::VAL == 1059);
impl NatIndex for Idx<1060> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1060> as NatIndex>::Out as Nat>::VAL == 1060);
impl NatIndex for Idx<1061> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1061> as NatIndex>::Out as Nat>::VAL == 1061);
impl NatIndex for Idx<1062> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1062> as NatIndex>::Out as Nat>::VAL == 1062);
impl NatIndex for Idx<1063> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1063> as NatIndex>::Out as Nat>::VAL == 1063);
impl NatIndex for Idx<1064> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1064> as NatIndex>::Out as Nat>::VAL == 1064);
impl NatIndex for Idx<1065> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1065> as NatIndex>::Out as Nat>::VAL == 1065);
impl NatIndex for Idx<1066> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1066> as NatIndex>::Out as Nat>::VAL == 1066);
impl NatIndex for Idx<1067> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1067> as NatIndex>::Out as Nat>::VAL == 1067);
impl NatIndex for Idx<1068> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1068> as NatIndex>::Out as Nat>::VAL == 1068);
impl NatIndex for Idx<1069> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1069> as NatIndex>::Out as Nat>::VAL == 1069);
impl NatIndex for Idx<1070> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1070> as NatIndex>::Out as Nat>::VAL == 1070);
impl NatIndex for Idx<1071> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1071> as NatIndex>::Out as Nat>::VAL == 1071);
impl NatIndex for Idx<1072> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1072> as NatIndex>::Out as Nat>::VAL == 1072);
impl NatIndex for Idx<1073> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1073> as NatIndex>::Out as Nat>::VAL == 1073);
impl NatIndex for Idx<1074> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1074> as NatIndex>::Out as Nat>::VAL == 1074);
impl NatIndex for Idx<1075> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1075> as NatIndex>::Out as Nat>::VAL == 1075);
impl NatIndex for Idx<1076> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1076> as NatIndex>::Out as Nat>::VAL == 1076);
impl NatIndex for Idx<1077> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1077> as NatIndex>::Out as Nat>::VAL == 1077);
impl NatIndex for Idx<1078> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1078> as NatIndex>::Out as Nat>::VAL == 1078);
impl NatIndex for Idx<1079> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1079> as NatIndex>::Out as Nat>::VAL == 1079);
impl NatIndex for Idx<1080> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1080> as NatIndex>::Out as Nat>::VAL == 1080);
impl NatIndex for Idx<1081> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1081> as NatIndex>::Out as Nat>::VAL == 1081);
impl NatIndex for Idx<1082> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1082> as NatIndex>::Out as Nat>::VAL == 1082);
impl NatIndex for Idx<1083> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1083> as NatIndex>::Out as Nat>::VAL == 1083);
impl NatIndex for Idx<1084> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1084> as NatIndex>::Out as Nat>::VAL == 1084);
impl NatIndex for Idx<1085> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1085> as NatIndex>::Out as Nat>::VAL == 1085);
impl NatIndex for Idx<1086> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1086> as NatIndex>::Out as Nat>::VAL == 1086);
impl NatIndex for Idx<1087> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1087> as NatIndex>::Out as Nat>::VAL == 1087);
impl NatIndex for Idx<1088> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1088> as NatIndex>::Out as Nat>::VAL == 1088);
impl NatIndex for Idx<1089> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1089> as NatIndex>::Out as Nat>::VAL == 1089);
impl NatIndex for Idx<1090> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1090> as NatIndex>::Out as Nat>::VAL == 1090);
impl NatIndex for Idx<1091> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1091> as NatIndex>::Out as Nat>::VAL == 1091);
impl NatIndex for Idx<1092> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1092> as NatIndex>::Out as Nat>::VAL == 1092);
impl NatIndex for Idx<1093> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1093> as NatIndex>::Out as Nat>::VAL == 1093);
impl NatIndex for Idx<1094> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1094> as NatIndex>::Out as Nat>::VAL == 1094);
impl NatIndex for Idx<1095> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1095> as NatIndex>::Out as Nat>::VAL == 1095);
impl NatIndex for Idx<1096> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1096> as NatIndex>::Out as Nat>::VAL == 1096);
impl NatIndex for Idx<1097> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1097> as NatIndex>::Out as Nat>::VAL == 1097);
impl NatIndex for Idx<1098> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1098> as NatIndex>::Out as Nat>::VAL == 1098);
impl NatIndex for Idx<1099> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1099> as NatIndex>::Out as Nat>::VAL == 1099);
impl NatIndex for Idx<1100> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1100> as NatIndex>::Out as Nat>::VAL == 1100);
impl NatIndex for Idx<1101> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1101> as NatIndex>::Out as Nat>::VAL == 1101);
impl NatIndex for Idx<1102> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1102> as NatIndex>::Out as Nat>::VAL == 1102);
impl NatIndex for Idx<1103> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1103> as NatIndex>::Out as Nat>::VAL == 1103);
impl NatIndex for Idx<1104> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1104> as NatIndex>::Out as Nat>::VAL == 1104);
impl NatIndex for Idx<1105> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1105> as NatIndex>::Out as Nat>::VAL == 1105);
impl NatIndex for Idx<1106> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1106> as NatIndex>::Out as Nat>::VAL == 1106);
impl NatIndex for Idx<1107> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1107> as NatIndex>::Out as Nat>::VAL == 1107);
impl NatIndex for Idx<1108> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1108> as NatIndex>::Out as Nat>::VAL == 1108);
impl NatIndex for Idx<1109> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1109> as NatIndex>::Out as Nat>::VAL == 1109);
impl NatIndex for Idx<1110> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1110> as NatIndex>::Out as Nat>::VAL == 1110);
impl NatIndex for Idx<1111> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1111> as NatIndex>::Out as Nat>::VAL == 1111);
impl NatIndex for Idx<1112> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1112> as NatIndex>::Out as Nat>::VAL == 1112);
impl NatIndex for Idx<1113> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1113> as NatIndex>::Out as Nat>::VAL == 1113);
impl NatIndex for Idx<1114> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1114> as NatIndex>::Out as Nat>::VAL == 1114);
impl NatIndex for Idx<1115> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1115> as NatIndex>::Out as Nat>::VAL == 1115);
impl NatIndex for Idx<1116> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1116> as NatIndex>::Out as Nat>::VAL == 1116);
impl NatIndex for Idx<1117> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1117> as NatIndex>::Out as Nat>::VAL == 1117);
impl NatIndex for Idx<1118> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1118> as NatIndex>::Out as Nat>::VAL == 1118);
impl NatIndex for Idx<1119> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1119> as NatIndex>::Out as Nat>::VAL == 1119);
impl NatIndex for Idx<1120> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1120> as NatIndex>::Out as Nat>::VAL == 1120);
impl NatIndex for Idx<1121> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1121> as NatIndex>::Out as Nat>::VAL == 1121);
impl NatIndex for Idx<1122> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1122> as NatIndex>::Out as Nat>::VAL == 1122);
impl NatIndex for Idx<1123> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1123> as NatIndex>::Out as Nat>::VAL == 1123);
impl NatIndex for Idx<1124> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1124> as NatIndex>::Out as Nat>::VAL == 1124);
impl NatIndex for Idx<1125> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1125> as NatIndex>::Out as Nat>::VAL == 1125);
impl NatIndex for Idx<1126> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1126> as NatIndex>::Out as Nat>::VAL == 1126);
impl NatIndex for Idx<1127> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1127> as NatIndex>::Out as Nat>::VAL == 1127);
impl NatIndex for Idx<1128> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1128> as NatIndex>::Out as Nat>::VAL == 1128);
impl NatIndex for Idx<1129> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1129> as NatIndex>::Out as Nat>::VAL == 1129);
impl NatIndex for Idx<1130> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1130> as NatIndex>::Out as Nat>::VAL == 1130);
impl NatIndex for Idx<1131> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1131> as NatIndex>::Out as Nat>::VAL == 1131);
impl NatIndex for Idx<1132> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1132> as NatIndex>::Out as Nat>::VAL == 1132);
impl NatIndex for Idx<1133> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1133> as NatIndex>::Out as Nat>::VAL == 1133);
impl NatIndex for Idx<1134> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1134> as NatIndex>::Out as Nat>::VAL == 1134);
impl NatIndex for Idx<1135> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1135> as NatIndex>::Out as Nat>::VAL == 1135);
impl NatIndex for Idx<1136> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1136> as NatIndex>::Out as Nat>::VAL == 1136);
impl NatIndex for Idx<1137> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1137> as NatIndex>::Out as Nat>::VAL == 1137);
impl NatIndex for Idx<1138> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1138> as NatIndex>::Out as Nat>::VAL == 1138);
impl NatIndex for Idx<1139> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1139> as NatIndex>::Out as Nat>::VAL == 1139);
impl NatIndex for Idx<1140> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1140> as NatIndex>::Out as Nat>::VAL == 1140);
impl NatIndex for Idx<1141> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1141> as NatIndex>::Out as Nat>::VAL == 1141);
impl NatIndex for Idx<1142> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1142> as NatIndex>::Out as Nat>::VAL == 1142);
impl NatIndex for Idx<1143> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1143> as NatIndex>::Out as Nat>::VAL == 1143);
impl NatIndex for Idx<1144> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1144> as NatIndex>::Out as Nat>::VAL == 1144);
impl NatIndex for Idx<1145> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1145> as NatIndex>::Out as Nat>::VAL == 1145);
impl NatIndex for Idx<1146> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1146> as NatIndex>::Out as Nat>::VAL == 1146);
impl NatIndex for Idx<1147> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1147> as NatIndex>::Out as Nat>::VAL == 1147);
impl NatIndex for Idx<1148> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1148> as NatIndex>::Out as Nat>::VAL == 1148);
impl NatIndex for Idx<1149> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1149> as NatIndex>::Out as Nat>::VAL == 1149);
impl NatIndex for Idx<1150> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1150> as NatIndex>::Out as Nat>::VAL == 1150);
impl NatIndex for Idx<1151> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1151> as NatIndex>::Out as Nat>::VAL == 1151);
impl NatIndex for Idx<1152> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1152> as NatIndex>::Out as Nat>::VAL == 1152);
impl NatIndex for Idx<1153> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1153> as NatIndex>::Out as Nat>::VAL == 1153);
impl NatIndex for Idx<1154> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1154> as NatIndex>::Out as Nat>::VAL == 1154);
impl NatIndex for Idx<1155> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1155> as NatIndex>::Out as Nat>::VAL == 1155);
impl NatIndex for Idx<1156> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1156> as NatIndex>::Out as Nat>::VAL == 1156);
impl NatIndex for Idx<1157> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1157> as NatIndex>::Out as Nat>::VAL == 1157);
impl NatIndex for Idx<1158> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1158> as NatIndex>::Out as Nat>::VAL == 1158);
impl NatIndex for Idx<1159> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1159> as NatIndex>::Out as Nat>::VAL == 1159);
impl NatIndex for Idx<1160> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1160> as NatIndex>::Out as Nat>::VAL == 1160);
impl NatIndex for Idx<1161> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1161> as NatIndex>::Out as Nat>::VAL == 1161);
impl NatIndex for Idx<1162> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1162> as NatIndex>::Out as Nat>::VAL == 1162);
impl NatIndex for Idx<1163> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1163> as NatIndex>::Out as Nat>::VAL == 1163);
impl NatIndex for Idx<1164> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1164> as NatIndex>::Out as Nat>::VAL == 1164);
impl NatIndex for Idx<1165> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1165> as NatIndex>::Out as Nat>::VAL == 1165);
impl NatIndex for Idx<1166> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1166> as NatIndex>::Out as Nat>::VAL == 1166);
impl NatIndex for Idx<1167> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1167> as NatIndex>::Out as Nat>::VAL == 1167);
impl NatIndex for Idx<1168> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1168> as NatIndex>::Out as Nat>::VAL == 1168);
impl NatIndex for Idx<1169> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1169> as NatIndex>::Out as Nat>::VAL == 1169);
impl NatIndex for Idx<1170> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1170> as NatIndex>::Out as Nat>::VAL == 1170);
impl NatIndex for Idx<1171> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1171> as NatIndex>::Out as Nat>::VAL == 1171);
impl NatIndex for Idx<1172> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1172> as NatIndex>::Out as Nat>::VAL == 1172);
impl NatIndex for Idx<1173> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1173> as NatIndex>::Out as Nat>::VAL == 1173);
impl NatIndex for Idx<1174> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1174> as NatIndex>::Out as Nat>::VAL == 1174);
impl NatIndex for Idx<1175> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1175> as NatIndex>::Out as Nat>::VAL == 1175);
impl NatIndex for Idx<1176> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1176> as NatIndex>::Out as Nat>::VAL == 1176);
impl NatIndex for Idx<1177> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1177> as NatIndex>::Out as Nat>::VAL == 1177);
impl NatIndex for Idx<1178> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1178> as NatIndex>::Out as Nat>::VAL == 1178);
impl NatIndex for Idx<1179> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1179> as NatIndex>::Out as Nat>::VAL == 1179);
impl NatIndex for Idx<1180> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1180> as NatIndex>::Out as Nat>::VAL == 1180);
impl NatIndex for Idx<1181> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1181> as NatIndex>::Out as Nat>::VAL == 1181);
impl NatIndex for Idx<1182> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1182> as NatIndex>::Out as Nat>::VAL == 1182);
impl NatIndex for Idx<1183> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1183> as NatIndex>::Out as Nat>::VAL == 1183);
impl NatIndex for Idx<1184> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1184> as NatIndex>::Out as Nat>::VAL == 1184);
impl NatIndex for Idx<1185> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1185> as NatIndex>::Out as Nat>::VAL == 1185);
impl NatIndex for Idx<1186> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1186> as NatIndex>::Out as Nat>::VAL == 1186);
impl NatIndex for Idx<1187> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1187> as NatIndex>::Out as Nat>::VAL == 1187);
impl NatIndex for Idx<1188> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1188> as NatIndex>::Out as Nat>::VAL == 1188);
impl NatIndex for Idx<1189> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1189> as NatIndex>::Out as Nat>::VAL == 1189);
impl NatIndex for Idx<1190> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1190> as NatIndex>::Out as Nat>::VAL == 1190);
impl NatIndex for Idx<1191> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1191> as NatIndex>::Out as Nat>::VAL == 1191);
impl NatIndex for Idx<1192> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1192> as NatIndex>::Out as Nat>::VAL == 1192);
impl NatIndex for Idx<1193> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1193> as NatIndex>::Out as Nat>::VAL == 1193);
impl NatIndex for Idx<1194> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1194> as NatIndex>::Out as Nat>::VAL == 1194);
impl NatIndex for Idx<1195> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1195> as NatIndex>::Out as Nat>::VAL == 1195);
impl NatIndex for Idx<1196> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1196> as NatIndex>::Out as Nat>::VAL == 1196);
impl NatIndex for Idx<1197> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1197> as NatIndex>::Out as Nat>::VAL == 1197);
impl NatIndex for Idx<1198> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1198> as NatIndex>::Out as Nat>::VAL == 1198);
impl NatIndex for Idx<1199> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1199> as NatIndex>::Out as Nat>::VAL == 1199);
impl NatIndex for Idx<1200> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1200> as NatIndex>::Out as Nat>::VAL == 1200);
impl NatIndex for Idx<1201> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1201> as NatIndex>::Out as Nat>::VAL == 1201);
impl NatIndex for Idx<1202> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1202> as NatIndex>::Out as Nat>::VAL == 1202);
impl NatIndex for Idx<1203> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1203> as NatIndex>::Out as Nat>::VAL == 1203);
impl NatIndex for Idx<1204> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1204> as NatIndex>::Out as Nat>::VAL == 1204);
impl NatIndex for Idx<1205> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1205> as NatIndex>::Out as Nat>::VAL == 1205);
impl NatIndex for Idx<1206> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1206> as NatIndex>::Out as Nat>::VAL == 1206);
impl NatIndex for Idx<1207> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1207> as NatIndex>::Out as Nat>::VAL == 1207);
impl NatIndex for Idx<1208> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1208> as NatIndex>::Out as Nat>::VAL == 1208);
impl NatIndex for Idx<1209> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1209> as NatIndex>::Out as Nat>::VAL == 1209);
impl NatIndex for Idx<1210> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1210> as NatIndex>::Out as Nat>::VAL == 1210);
impl NatIndex for Idx<1211> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1211> as NatIndex>::Out as Nat>::VAL == 1211);
impl NatIndex for Idx<1212> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1212> as NatIndex>::Out as Nat>::VAL == 1212);
impl NatIndex for Idx<1213> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1213> as NatIndex>::Out as Nat>::VAL == 1213);
impl NatIndex for Idx<1214> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1214> as NatIndex>::Out as Nat>::VAL == 1214);
impl NatIndex for Idx<1215> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1215> as NatIndex>::Out as Nat>::VAL == 1215);
impl NatIndex for Idx<1216> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1216> as NatIndex>::Out as Nat>::VAL == 1216);
impl NatIndex for Idx<1217> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1217> as NatIndex>::Out as Nat>::VAL == 1217);
impl NatIndex for Idx<1218> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1218> as NatIndex>::Out as Nat>::VAL == 1218);
impl NatIndex for Idx<1219> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1219> as NatIndex>::Out as Nat>::VAL == 1219);
impl NatIndex for Idx<1220> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1220> as NatIndex>::Out as Nat>::VAL == 1220);
impl NatIndex for Idx<1221> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1221> as NatIndex>::Out as Nat>::VAL == 1221);
impl NatIndex for Idx<1222> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1222> as NatIndex>::Out as Nat>::VAL == 1222);
impl NatIndex for Idx<1223> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1223> as NatIndex>::Out as Nat>::VAL == 1223);
impl NatIndex for Idx<1224> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1224> as NatIndex>::Out as Nat>::VAL == 1224);
impl NatIndex for Idx<1225> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1225> as NatIndex>::Out as Nat>::VAL == 1225);
impl NatIndex for Idx<1226> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1226> as NatIndex>::Out as Nat>::VAL == 1226);
impl NatIndex for Idx<1227> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1227> as NatIndex>::Out as Nat>::VAL == 1227);
impl NatIndex for Idx<1228> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1228> as NatIndex>::Out as Nat>::VAL == 1228);
impl NatIndex for Idx<1229> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1229> as NatIndex>::Out as Nat>::VAL == 1229);
impl NatIndex for Idx<1230> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1230> as NatIndex>::Out as Nat>::VAL == 1230);
impl NatIndex for Idx<1231> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1231> as NatIndex>::Out as Nat>::VAL == 1231);
impl NatIndex for Idx<1232> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1232> as NatIndex>::Out as Nat>::VAL == 1232);
impl NatIndex for Idx<1233> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1233> as NatIndex>::Out as Nat>::VAL == 1233);
impl NatIndex for Idx<1234> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1234> as NatIndex>::Out as Nat>::VAL == 1234);
impl NatIndex for Idx<1235> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1235> as NatIndex>::Out as Nat>::VAL == 1235);
impl NatIndex for Idx<1236> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1236> as NatIndex>::Out as Nat>::VAL == 1236);
impl NatIndex for Idx<1237> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1237> as NatIndex>::Out as Nat>::VAL == 1237);
impl NatIndex for Idx<1238> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1238> as NatIndex>::Out as Nat>::VAL == 1238);
impl NatIndex for Idx<1239> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1239> as NatIndex>::Out as Nat>::VAL == 1239);
impl NatIndex for Idx<1240> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1240> as NatIndex>::Out as Nat>::VAL == 1240);
impl NatIndex for Idx<1241> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1241> as NatIndex>::Out as Nat>::VAL == 1241);
impl NatIndex for Idx<1242> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1242> as NatIndex>::Out as Nat>::VAL == 1242);
impl NatIndex for Idx<1243> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1243> as NatIndex>::Out as Nat>::VAL == 1243);
impl NatIndex for Idx<1244> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1244> as NatIndex>::Out as Nat>::VAL == 1244);
impl NatIndex for Idx<1245> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1245> as NatIndex>::Out as Nat>::VAL == 1245);
impl NatIndex for Idx<1246> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1246> as NatIndex>::Out as Nat>::VAL == 1246);
impl NatIndex for Idx<1247> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1247> as NatIndex>::Out as Nat>::VAL == 1247);
impl NatIndex for Idx<1248> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1248> as NatIndex>::Out as Nat>::VAL == 1248);
impl NatIndex for Idx<1249> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1249> as NatIndex>::Out as Nat>::VAL == 1249);
impl NatIndex for Idx<1250> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1250> as NatIndex>::Out as Nat>::VAL == 1250);
impl NatIndex for Idx<1251> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1251> as NatIndex>::Out as Nat>::VAL == 1251);
impl NatIndex for Idx<1252> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1252> as NatIndex>::Out as Nat>::VAL == 1252);
impl NatIndex for Idx<1253> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1253> as NatIndex>::Out as Nat>::VAL == 1253);
impl NatIndex for Idx<1254> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1254> as NatIndex>::Out as Nat>::VAL == 1254);
impl NatIndex for Idx<1255> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1255> as NatIndex>::Out as Nat>::VAL == 1255);
impl NatIndex for Idx<1256> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1256> as NatIndex>::Out as Nat>::VAL == 1256);
impl NatIndex for Idx<1257> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1257> as NatIndex>::Out as Nat>::VAL == 1257);
impl NatIndex for Idx<1258> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1258> as NatIndex>::Out as Nat>::VAL == 1258);
impl NatIndex for Idx<1259> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1259> as NatIndex>::Out as Nat>::VAL == 1259);
impl NatIndex for Idx<1260> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1260> as NatIndex>::Out as Nat>::VAL == 1260);
impl NatIndex for Idx<1261> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1261> as NatIndex>::Out as Nat>::VAL == 1261);
impl NatIndex for Idx<1262> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1262> as NatIndex>::Out as Nat>::VAL == 1262);
impl NatIndex for Idx<1263> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1263> as NatIndex>::Out as Nat>::VAL == 1263);
impl NatIndex for Idx<1264> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1264> as NatIndex>::Out as Nat>::VAL == 1264);
impl NatIndex for Idx<1265> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1265> as NatIndex>::Out as Nat>::VAL == 1265);
impl NatIndex for Idx<1266> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1266> as NatIndex>::Out as Nat>::VAL == 1266);
impl NatIndex for Idx<1267> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1267> as NatIndex>::Out as Nat>::VAL == 1267);
impl NatIndex for Idx<1268> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1268> as NatIndex>::Out as Nat>::VAL == 1268);
impl NatIndex for Idx<1269> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1269> as NatIndex>::Out as Nat>::VAL == 1269);
impl NatIndex for Idx<1270> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1270> as NatIndex>::Out as Nat>::VAL == 1270);
impl NatIndex for Idx<1271> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1271> as NatIndex>::Out as Nat>::VAL == 1271);
impl NatIndex for Idx<1272> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1272> as NatIndex>::Out as Nat>::VAL == 1272);
impl NatIndex for Idx<1273> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1273> as NatIndex>::Out as Nat>::VAL == 1273);
impl NatIndex for Idx<1274> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1274> as NatIndex>::Out as Nat>::VAL == 1274);
impl NatIndex for Idx<1275> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1275> as NatIndex>::Out as Nat>::VAL == 1275);
impl NatIndex for Idx<1276> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1276> as NatIndex>::Out as Nat>::VAL == 1276);
impl NatIndex for Idx<1277> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1277> as NatIndex>::Out as Nat>::VAL == 1277);
impl NatIndex for Idx<1278> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1278> as NatIndex>::Out as Nat>::VAL == 1278);
impl NatIndex for Idx<1279> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1279> as NatIndex>::Out as Nat>::VAL == 1279);
impl NatIndex for Idx<1280> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1280> as NatIndex>::Out as Nat>::VAL == 1280);
impl NatIndex for Idx<1281> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1281> as NatIndex>::Out as Nat>::VAL == 1281);
impl NatIndex for Idx<1282> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1282> as NatIndex>::Out as Nat>::VAL == 1282);
impl NatIndex for Idx<1283> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1283> as NatIndex>::Out as Nat>::VAL == 1283);
impl NatIndex for Idx<1284> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1284> as NatIndex>::Out as Nat>::VAL == 1284);
impl NatIndex for Idx<1285> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1285> as NatIndex>::Out as Nat>::VAL == 1285);
impl NatIndex for Idx<1286> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1286> as NatIndex>::Out as Nat>::VAL == 1286);
impl NatIndex for Idx<1287> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1287> as NatIndex>::Out as Nat>::VAL == 1287);
impl NatIndex for Idx<1288> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1288> as NatIndex>::Out as Nat>::VAL == 1288);
impl NatIndex for Idx<1289> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1289> as NatIndex>::Out as Nat>::VAL == 1289);
impl NatIndex for Idx<1290> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1290> as NatIndex>::Out as Nat>::VAL == 1290);
impl NatIndex for Idx<1291> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1291> as NatIndex>::Out as Nat>::VAL == 1291);
impl NatIndex for Idx<1292> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1292> as NatIndex>::Out as Nat>::VAL == 1292);
impl NatIndex for Idx<1293> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1293> as NatIndex>::Out as Nat>::VAL == 1293);
impl NatIndex for Idx<1294> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1294> as NatIndex>::Out as Nat>::VAL == 1294);
impl NatIndex for Idx<1295> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1295> as NatIndex>::Out as Nat>::VAL == 1295);
impl NatIndex for Idx<1296> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1296> as NatIndex>::Out as Nat>::VAL == 1296);
impl NatIndex for Idx<1297> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1297> as NatIndex>::Out as Nat>::VAL == 1297);
impl NatIndex for Idx<1298> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1298> as NatIndex>::Out as Nat>::VAL == 1298);
impl NatIndex for Idx<1299> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1299> as NatIndex>::Out as Nat>::VAL == 1299);
impl NatIndex for Idx<1300> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1300> as NatIndex>::Out as Nat>::VAL == 1300);
impl NatIndex for Idx<1301> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1301> as NatIndex>::Out as Nat>::VAL == 1301);
impl NatIndex for Idx<1302> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1302> as NatIndex>::Out as Nat>::VAL == 1302);
impl NatIndex for Idx<1303> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1303> as NatIndex>::Out as Nat>::VAL == 1303);
impl NatIndex for Idx<1304> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1304> as NatIndex>::Out as Nat>::VAL == 1304);
impl NatIndex for Idx<1305> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1305> as NatIndex>::Out as Nat>::VAL == 1305);
impl NatIndex for Idx<1306> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1306> as NatIndex>::Out as Nat>::VAL == 1306);
impl NatIndex for Idx<1307> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1307> as NatIndex>::Out as Nat>::VAL == 1307);
impl NatIndex for Idx<1308> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1308> as NatIndex>::Out as Nat>::VAL == 1308);
impl NatIndex for Idx<1309> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1309> as NatIndex>::Out as Nat>::VAL == 1309);
impl NatIndex for Idx<1310> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1310> as NatIndex>::Out as Nat>::VAL == 1310);
impl NatIndex for Idx<1311> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1311> as NatIndex>::Out as Nat>::VAL == 1311);
impl NatIndex for Idx<1312> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1312> as NatIndex>::Out as Nat>::VAL == 1312);
impl NatIndex for Idx<1313> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1313> as NatIndex>::Out as Nat>::VAL == 1313);
impl NatIndex for Idx<1314> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1314> as NatIndex>::Out as Nat>::VAL == 1314);
impl NatIndex for Idx<1315> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1315> as NatIndex>::Out as Nat>::VAL == 1315);
impl NatIndex for Idx<1316> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1316> as NatIndex>::Out as Nat>::VAL == 1316);
impl NatIndex for Idx<1317> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1317> as NatIndex>::Out as Nat>::VAL == 1317);
impl NatIndex for Idx<1318> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1318> as NatIndex>::Out as Nat>::VAL == 1318);
impl NatIndex for Idx<1319> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1319> as NatIndex>::Out as Nat>::VAL == 1319);
impl NatIndex for Idx<1320> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1320> as NatIndex>::Out as Nat>::VAL == 1320);
impl NatIndex for Idx<1321> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1321> as NatIndex>::Out as Nat>::VAL == 1321);
impl NatIndex for Idx<1322> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1322> as NatIndex>::Out as Nat>::VAL == 1322);
impl NatIndex for Idx<1323> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1323> as NatIndex>::Out as Nat>::VAL == 1323);
impl NatIndex for Idx<1324> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1324> as NatIndex>::Out as Nat>::VAL == 1324);
impl NatIndex for Idx<1325> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1325> as NatIndex>::Out as Nat>::VAL == 1325);
impl NatIndex for Idx<1326> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1326> as NatIndex>::Out as Nat>::VAL == 1326);
impl NatIndex for Idx<1327> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1327> as NatIndex>::Out as Nat>::VAL == 1327);
impl NatIndex for Idx<1328> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1328> as NatIndex>::Out as Nat>::VAL == 1328);
impl NatIndex for Idx<1329> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1329> as NatIndex>::Out as Nat>::VAL == 1329);
impl NatIndex for Idx<1330> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1330> as NatIndex>::Out as Nat>::VAL == 1330);
impl NatIndex for Idx<1331> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1331> as NatIndex>::Out as Nat>::VAL == 1331);
impl NatIndex for Idx<1332> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1332> as NatIndex>::Out as Nat>::VAL == 1332);
impl NatIndex for Idx<1333> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1333> as NatIndex>::Out as Nat>::VAL == 1333);
impl NatIndex for Idx<1334> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1334> as NatIndex>::Out as Nat>::VAL == 1334);
impl NatIndex for Idx<1335> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1335> as NatIndex>::Out as Nat>::VAL == 1335);
impl NatIndex for Idx<1336> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1336> as NatIndex>::Out as Nat>::VAL == 1336);
impl NatIndex for Idx<1337> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1337> as NatIndex>::Out as Nat>::VAL == 1337);
impl NatIndex for Idx<1338> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1338> as NatIndex>::Out as Nat>::VAL == 1338);
impl NatIndex for Idx<1339> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1339> as NatIndex>::Out as Nat>::VAL == 1339);
impl NatIndex for Idx<1340> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1340> as NatIndex>::Out as Nat>::VAL == 1340);
impl NatIndex for Idx<1341> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1341> as NatIndex>::Out as Nat>::VAL == 1341);
impl NatIndex for Idx<1342> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1342> as NatIndex>::Out as Nat>::VAL == 1342);
impl NatIndex for Idx<1343> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1343> as NatIndex>::Out as Nat>::VAL == 1343);
impl NatIndex for Idx<1344> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1344> as NatIndex>::Out as Nat>::VAL == 1344);
impl NatIndex for Idx<1345> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1345> as NatIndex>::Out as Nat>::VAL == 1345);
impl NatIndex for Idx<1346> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1346> as NatIndex>::Out as Nat>::VAL == 1346);
impl NatIndex for Idx<1347> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1347> as NatIndex>::Out as Nat>::VAL == 1347);
impl NatIndex for Idx<1348> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1348> as NatIndex>::Out as Nat>::VAL == 1348);
impl NatIndex for Idx<1349> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1349> as NatIndex>::Out as Nat>::VAL == 1349);
impl NatIndex for Idx<1350> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1350> as NatIndex>::Out as Nat>::VAL == 1350);
impl NatIndex for Idx<1351> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1351> as NatIndex>::Out as Nat>::VAL == 1351);
impl NatIndex for Idx<1352> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1352> as NatIndex>::Out as Nat>::VAL == 1352);
impl NatIndex for Idx<1353> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1353> as NatIndex>::Out as Nat>::VAL == 1353);
impl NatIndex for Idx<1354> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1354> as NatIndex>::Out as Nat>::VAL == 1354);
impl NatIndex for Idx<1355> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1355> as NatIndex>::Out as Nat>::VAL == 1355);
impl NatIndex for Idx<1356> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1356> as NatIndex>::Out as Nat>::VAL == 1356);
impl NatIndex for Idx<1357> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1357> as NatIndex>::Out as Nat>::VAL == 1357);
impl NatIndex for Idx<1358> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1358> as NatIndex>::Out as Nat>::VAL == 1358);
impl NatIndex for Idx<1359> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1359> as NatIndex>::Out as Nat>::VAL == 1359);
impl NatIndex for Idx<1360> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1360> as NatIndex>::Out as Nat>::VAL == 1360);
impl NatIndex for Idx<1361> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1361> as NatIndex>::Out as Nat>::VAL == 1361);
impl NatIndex for Idx<1362> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1362> as NatIndex>::Out as Nat>::VAL == 1362);
impl NatIndex for Idx<1363> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1363> as NatIndex>::Out as Nat>::VAL == 1363);
impl NatIndex for Idx<1364> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1364> as NatIndex>::Out as Nat>::VAL == 1364);
impl NatIndex for Idx<1365> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1365> as NatIndex>::Out as Nat>::VAL == 1365);
impl NatIndex for Idx<1366> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1366> as NatIndex>::Out as Nat>::VAL == 1366);
impl NatIndex for Idx<1367> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1367> as NatIndex>::Out as Nat>::VAL == 1367);
impl NatIndex for Idx<1368> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1368> as NatIndex>::Out as Nat>::VAL == 1368);
impl NatIndex for Idx<1369> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1369> as NatIndex>::Out as Nat>::VAL == 1369);
impl NatIndex for Idx<1370> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1370> as NatIndex>::Out as Nat>::VAL == 1370);
impl NatIndex for Idx<1371> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1371> as NatIndex>::Out as Nat>::VAL == 1371);
impl NatIndex for Idx<1372> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1372> as NatIndex>::Out as Nat>::VAL == 1372);
impl NatIndex for Idx<1373> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1373> as NatIndex>::Out as Nat>::VAL == 1373);
impl NatIndex for Idx<1374> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1374> as NatIndex>::Out as Nat>::VAL == 1374);
impl NatIndex for Idx<1375> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1375> as NatIndex>::Out as Nat>::VAL == 1375);
impl NatIndex for Idx<1376> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1376> as NatIndex>::Out as Nat>::VAL == 1376);
impl NatIndex for Idx<1377> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1377> as NatIndex>::Out as Nat>::VAL == 1377);
impl NatIndex for Idx<1378> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1378> as NatIndex>::Out as Nat>::VAL == 1378);
impl NatIndex for Idx<1379> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1379> as NatIndex>::Out as Nat>::VAL == 1379);
impl NatIndex for Idx<1380> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1380> as NatIndex>::Out as Nat>::VAL == 1380);
impl NatIndex for Idx<1381> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1381> as NatIndex>::Out as Nat>::VAL == 1381);
impl NatIndex for Idx<1382> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1382> as NatIndex>::Out as Nat>::VAL == 1382);
impl NatIndex for Idx<1383> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1383> as NatIndex>::Out as Nat>::VAL == 1383);
impl NatIndex for Idx<1384> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1384> as NatIndex>::Out as Nat>::VAL == 1384);
impl NatIndex for Idx<1385> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1385> as NatIndex>::Out as Nat>::VAL == 1385);
impl NatIndex for Idx<1386> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1386> as NatIndex>::Out as Nat>::VAL == 1386);
impl NatIndex for Idx<1387> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1387> as NatIndex>::Out as Nat>::VAL == 1387);
impl NatIndex for Idx<1388> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1388> as NatIndex>::Out as Nat>::VAL == 1388);
impl NatIndex for Idx<1389> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1389> as NatIndex>::Out as Nat>::VAL == 1389);
impl NatIndex for Idx<1390> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1390> as NatIndex>::Out as Nat>::VAL == 1390);
impl NatIndex for Idx<1391> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1391> as NatIndex>::Out as Nat>::VAL == 1391);
impl NatIndex for Idx<1392> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1392> as NatIndex>::Out as Nat>::VAL == 1392);
impl NatIndex for Idx<1393> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1393> as NatIndex>::Out as Nat>::VAL == 1393);
impl NatIndex for Idx<1394> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1394> as NatIndex>::Out as Nat>::VAL == 1394);
impl NatIndex for Idx<1395> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1395> as NatIndex>::Out as Nat>::VAL == 1395);
impl NatIndex for Idx<1396> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1396> as NatIndex>::Out as Nat>::VAL == 1396);
impl NatIndex for Idx<1397> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1397> as NatIndex>::Out as Nat>::VAL == 1397);
impl NatIndex for Idx<1398> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1398> as NatIndex>::Out as Nat>::VAL == 1398);
impl NatIndex for Idx<1399> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1399> as NatIndex>::Out as Nat>::VAL == 1399);
impl NatIndex for Idx<1400> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1400> as NatIndex>::Out as Nat>::VAL == 1400);
impl NatIndex for Idx<1401> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1401> as NatIndex>::Out as Nat>::VAL == 1401);
impl NatIndex for Idx<1402> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1402> as NatIndex>::Out as Nat>::VAL == 1402);
impl NatIndex for Idx<1403> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1403> as NatIndex>::Out as Nat>::VAL == 1403);
impl NatIndex for Idx<1404> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1404> as NatIndex>::Out as Nat>::VAL == 1404);
impl NatIndex for Idx<1405> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1405> as NatIndex>::Out as Nat>::VAL == 1405);
impl NatIndex for Idx<1406> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1406> as NatIndex>::Out as Nat>::VAL == 1406);
impl NatIndex for Idx<1407> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1407> as NatIndex>::Out as Nat>::VAL == 1407);
impl NatIndex for Idx<1408> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1408> as NatIndex>::Out as Nat>::VAL == 1408);
impl NatIndex for Idx<1409> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1409> as NatIndex>::Out as Nat>::VAL == 1409);
impl NatIndex for Idx<1410> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1410> as NatIndex>::Out as Nat>::VAL == 1410);
impl NatIndex for Idx<1411> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1411> as NatIndex>::Out as Nat>::VAL == 1411);
impl NatIndex for Idx<1412> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1412> as NatIndex>::Out as Nat>::VAL == 1412);
impl NatIndex for Idx<1413> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1413> as NatIndex>::Out as Nat>::VAL == 1413);
impl NatIndex for Idx<1414> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1414> as NatIndex>::Out as Nat>::VAL == 1414);
impl NatIndex for Idx<1415> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1415> as NatIndex>::Out as Nat>::VAL == 1415);
impl NatIndex for Idx<1416> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1416> as NatIndex>::Out as Nat>::VAL == 1416);
impl NatIndex for Idx<1417> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1417> as NatIndex>::Out as Nat>::VAL == 1417);
impl NatIndex for Idx<1418> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1418> as NatIndex>::Out as Nat>::VAL == 1418);
impl NatIndex for Idx<1419> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1419> as NatIndex>::Out as Nat>::VAL == 1419);
impl NatIndex for Idx<1420> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1420> as NatIndex>::Out as Nat>::VAL == 1420);
impl NatIndex for Idx<1421> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1421> as NatIndex>::Out as Nat>::VAL == 1421);
impl NatIndex for Idx<1422> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1422> as NatIndex>::Out as Nat>::VAL == 1422);
impl NatIndex for Idx<1423> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1423> as NatIndex>::Out as Nat>::VAL == 1423);
impl NatIndex for Idx<1424> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1424> as NatIndex>::Out as Nat>::VAL == 1424);
impl NatIndex for Idx<1425> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1425> as NatIndex>::Out as Nat>::VAL == 1425);
impl NatIndex for Idx<1426> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1426> as NatIndex>::Out as Nat>::VAL == 1426);
impl NatIndex for Idx<1427> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1427> as NatIndex>::Out as Nat>::VAL == 1427);
impl NatIndex for Idx<1428> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1428> as NatIndex>::Out as Nat>::VAL == 1428);
impl NatIndex for Idx<1429> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1429> as NatIndex>::Out as Nat>::VAL == 1429);
impl NatIndex for Idx<1430> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1430> as NatIndex>::Out as Nat>::VAL == 1430);
impl NatIndex for Idx<1431> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1431> as NatIndex>::Out as Nat>::VAL == 1431);
impl NatIndex for Idx<1432> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1432> as NatIndex>::Out as Nat>::VAL == 1432);
impl NatIndex for Idx<1433> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1433> as NatIndex>::Out as Nat>::VAL == 1433);
impl NatIndex for Idx<1434> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1434> as NatIndex>::Out as Nat>::VAL == 1434);
impl NatIndex for Idx<1435> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1435> as NatIndex>::Out as Nat>::VAL == 1435);
impl NatIndex for Idx<1436> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1436> as NatIndex>::Out as Nat>::VAL == 1436);
impl NatIndex for Idx<1437> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1437> as NatIndex>::Out as Nat>::VAL == 1437);
impl NatIndex for Idx<1438> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1438> as NatIndex>::Out as Nat>::VAL == 1438);
impl NatIndex for Idx<1439> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1439> as NatIndex>::Out as Nat>::VAL == 1439);
impl NatIndex for Idx<1440> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1440> as NatIndex>::Out as Nat>::VAL == 1440);
impl NatIndex for Idx<1441> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1441> as NatIndex>::Out as Nat>::VAL == 1441);
impl NatIndex for Idx<1442> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1442> as NatIndex>::Out as Nat>::VAL == 1442);
impl NatIndex for Idx<1443> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1443> as NatIndex>::Out as Nat>::VAL == 1443);
impl NatIndex for Idx<1444> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1444> as NatIndex>::Out as Nat>::VAL == 1444);
impl NatIndex for Idx<1445> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1445> as NatIndex>::Out as Nat>::VAL == 1445);
impl NatIndex for Idx<1446> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1446> as NatIndex>::Out as Nat>::VAL == 1446);
impl NatIndex for Idx<1447> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1447> as NatIndex>::Out as Nat>::VAL == 1447);
impl NatIndex for Idx<1448> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1448> as NatIndex>::Out as Nat>::VAL == 1448);
impl NatIndex for Idx<1449> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1449> as NatIndex>::Out as Nat>::VAL == 1449);
impl NatIndex for Idx<1450> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1450> as NatIndex>::Out as Nat>::VAL == 1450);
impl NatIndex for Idx<1451> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1451> as NatIndex>::Out as Nat>::VAL == 1451);
impl NatIndex for Idx<1452> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1452> as NatIndex>::Out as Nat>::VAL == 1452);
impl NatIndex for Idx<1453> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1453> as NatIndex>::Out as Nat>::VAL == 1453);
impl NatIndex for Idx<1454> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1454> as NatIndex>::Out as Nat>::VAL == 1454);
impl NatIndex for Idx<1455> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1455> as NatIndex>::Out as Nat>::VAL == 1455);
impl NatIndex for Idx<1456> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1456> as NatIndex>::Out as Nat>::VAL == 1456);
impl NatIndex for Idx<1457> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1457> as NatIndex>::Out as Nat>::VAL == 1457);
impl NatIndex for Idx<1458> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1458> as NatIndex>::Out as Nat>::VAL == 1458);
impl NatIndex for Idx<1459> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1459> as NatIndex>::Out as Nat>::VAL == 1459);
impl NatIndex for Idx<1460> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1460> as NatIndex>::Out as Nat>::VAL == 1460);
impl NatIndex for Idx<1461> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1461> as NatIndex>::Out as Nat>::VAL == 1461);
impl NatIndex for Idx<1462> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1462> as NatIndex>::Out as Nat>::VAL == 1462);
impl NatIndex for Idx<1463> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1463> as NatIndex>::Out as Nat>::VAL == 1463);
impl NatIndex for Idx<1464> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1464> as NatIndex>::Out as Nat>::VAL == 1464);
impl NatIndex for Idx<1465> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1465> as NatIndex>::Out as Nat>::VAL == 1465);
impl NatIndex for Idx<1466> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1466> as NatIndex>::Out as Nat>::VAL == 1466);
impl NatIndex for Idx<1467> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1467> as NatIndex>::Out as Nat>::VAL == 1467);
impl NatIndex for Idx<1468> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1468> as NatIndex>::Out as Nat>::VAL == 1468);
impl NatIndex for Idx<1469> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1469> as NatIndex>::Out as Nat>::VAL == 1469);
impl NatIndex for Idx<1470> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1470> as NatIndex>::Out as Nat>::VAL == 1470);
impl NatIndex for Idx<1471> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1471> as NatIndex>::Out as Nat>::VAL == 1471);
impl NatIndex for Idx<1472> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1472> as NatIndex>::Out as Nat>::VAL == 1472);
impl NatIndex for Idx<1473> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1473> as NatIndex>::Out as Nat>::VAL == 1473);
impl NatIndex for Idx<1474> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1474> as NatIndex>::Out as Nat>::VAL == 1474);
impl NatIndex for Idx<1475> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1475> as NatIndex>::Out as Nat>::VAL == 1475);
impl NatIndex for Idx<1476> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1476> as NatIndex>::Out as Nat>::VAL == 1476);
impl NatIndex for Idx<1477> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1477> as NatIndex>::Out as Nat>::VAL == 1477);
impl NatIndex for Idx<1478> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1478> as NatIndex>::Out as Nat>::VAL == 1478);
impl NatIndex for Idx<1479> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1479> as NatIndex>::Out as Nat>::VAL == 1479);
impl NatIndex for Idx<1480> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1480> as NatIndex>::Out as Nat>::VAL == 1480);
impl NatIndex for Idx<1481> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1481> as NatIndex>::Out as Nat>::VAL == 1481);
impl NatIndex for Idx<1482> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1482> as NatIndex>::Out as Nat>::VAL == 1482);
impl NatIndex for Idx<1483> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1483> as NatIndex>::Out as Nat>::VAL == 1483);
impl NatIndex for Idx<1484> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1484> as NatIndex>::Out as Nat>::VAL == 1484);
impl NatIndex for Idx<1485> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1485> as NatIndex>::Out as Nat>::VAL == 1485);
impl NatIndex for Idx<1486> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1486> as NatIndex>::Out as Nat>::VAL == 1486);
impl NatIndex for Idx<1487> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1487> as NatIndex>::Out as Nat>::VAL == 1487);
impl NatIndex for Idx<1488> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1488> as NatIndex>::Out as Nat>::VAL == 1488);
impl NatIndex for Idx<1489> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1489> as NatIndex>::Out as Nat>::VAL == 1489);
impl NatIndex for Idx<1490> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1490> as NatIndex>::Out as Nat>::VAL == 1490);
impl NatIndex for Idx<1491> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1491> as NatIndex>::Out as Nat>::VAL == 1491);
impl NatIndex for Idx<1492> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1492> as NatIndex>::Out as Nat>::VAL == 1492);
impl NatIndex for Idx<1493> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1493> as NatIndex>::Out as Nat>::VAL == 1493);
impl NatIndex for Idx<1494> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1494> as NatIndex>::Out as Nat>::VAL == 1494);
impl NatIndex for Idx<1495> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1495> as NatIndex>::Out as Nat>::VAL == 1495);
impl NatIndex for Idx<1496> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1496> as NatIndex>::Out as Nat>::VAL == 1496);
impl NatIndex for Idx<1497> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1497> as NatIndex>::Out as Nat>::VAL == 1497);
impl NatIndex for Idx<1498> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1498> as NatIndex>::Out as Nat>::VAL == 1498);
impl NatIndex for Idx<1499> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1499> as NatIndex>::Out as Nat>::VAL == 1499);
impl NatIndex for Idx<1500> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1500> as NatIndex>::Out as Nat>::VAL == 1500);
impl NatIndex for Idx<1501> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1501> as NatIndex>::Out as Nat>::VAL == 1501);
impl NatIndex for Idx<1502> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1502> as NatIndex>::Out as Nat>::VAL == 1502);
impl NatIndex for Idx<1503> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1503> as NatIndex>::Out as Nat>::VAL == 1503);
impl NatIndex for Idx<1504> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1504> as NatIndex>::Out as Nat>::VAL == 1504);
impl NatIndex for Idx<1505> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1505> as NatIndex>::Out as Nat>::VAL == 1505);
impl NatIndex for Idx<1506> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1506> as NatIndex>::Out as Nat>::VAL == 1506);
impl NatIndex for Idx<1507> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1507> as NatIndex>::Out as Nat>::VAL == 1507);
impl NatIndex for Idx<1508> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1508> as NatIndex>::Out as Nat>::VAL == 1508);
impl NatIndex for Idx<1509> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1509> as NatIndex>::Out as Nat>::VAL == 1509);
impl NatIndex for Idx<1510> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1510> as NatIndex>::Out as Nat>::VAL == 1510);
impl NatIndex for Idx<1511> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1511> as NatIndex>::Out as Nat>::VAL == 1511);
impl NatIndex for Idx<1512> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1512> as NatIndex>::Out as Nat>::VAL == 1512);
impl NatIndex for Idx<1513> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1513> as NatIndex>::Out as Nat>::VAL == 1513);
impl NatIndex for Idx<1514> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1514> as NatIndex>::Out as Nat>::VAL == 1514);
impl NatIndex for Idx<1515> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1515> as NatIndex>::Out as Nat>::VAL == 1515);
impl NatIndex for Idx<1516> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1516> as NatIndex>::Out as Nat>::VAL == 1516);
impl NatIndex for Idx<1517> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1517> as NatIndex>::Out as Nat>::VAL == 1517);
impl NatIndex for Idx<1518> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1518> as NatIndex>::Out as Nat>::VAL == 1518);
impl NatIndex for Idx<1519> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1519> as NatIndex>::Out as Nat>::VAL == 1519);
impl NatIndex for Idx<1520> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1520> as NatIndex>::Out as Nat>::VAL == 1520);
impl NatIndex for Idx<1521> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1521> as NatIndex>::Out as Nat>::VAL == 1521);
impl NatIndex for Idx<1522> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1522> as NatIndex>::Out as Nat>::VAL == 1522);
impl NatIndex for Idx<1523> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1523> as NatIndex>::Out as Nat>::VAL == 1523);
impl NatIndex for Idx<1524> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1524> as NatIndex>::Out as Nat>::VAL == 1524);
impl NatIndex for Idx<1525> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1525> as NatIndex>::Out as Nat>::VAL == 1525);
impl NatIndex for Idx<1526> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1526> as NatIndex>::Out as Nat>::VAL == 1526);
impl NatIndex for Idx<1527> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1527> as NatIndex>::Out as Nat>::VAL == 1527);
impl NatIndex for Idx<1528> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1528> as NatIndex>::Out as Nat>::VAL == 1528);
impl NatIndex for Idx<1529> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1529> as NatIndex>::Out as Nat>::VAL == 1529);
impl NatIndex for Idx<1530> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1530> as NatIndex>::Out as Nat>::VAL == 1530);
impl NatIndex for Idx<1531> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1531> as NatIndex>::Out as Nat>::VAL == 1531);
impl NatIndex for Idx<1532> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1532> as NatIndex>::Out as Nat>::VAL == 1532);
impl NatIndex for Idx<1533> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1533> as NatIndex>::Out as Nat>::VAL == 1533);
impl NatIndex for Idx<1534> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1534> as NatIndex>::Out as Nat>::VAL == 1534);
impl NatIndex for Idx<1535> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1535> as NatIndex>::Out as Nat>::VAL == 1535);
impl NatIndex for Idx<1536> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1536> as NatIndex>::Out as Nat>::VAL == 1536);
impl NatIndex for Idx<1537> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1537> as NatIndex>::Out as Nat>::VAL == 1537);
impl NatIndex for Idx<1538> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1538> as NatIndex>::Out as Nat>::VAL == 1538);
impl NatIndex for Idx<1539> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1539> as NatIndex>::Out as Nat>::VAL == 1539);
impl NatIndex for Idx<1540> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1540> as NatIndex>::Out as Nat>::VAL == 1540);
impl NatIndex for Idx<1541> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1541> as NatIndex>::Out as Nat>::VAL == 1541);
impl NatIndex for Idx<1542> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1542> as NatIndex>::Out as Nat>::VAL == 1542);
impl NatIndex for Idx<1543> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1543> as NatIndex>::Out as Nat>::VAL == 1543);
impl NatIndex for Idx<1544> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1544> as NatIndex>::Out as Nat>::VAL == 1544);
impl NatIndex for Idx<1545> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1545> as NatIndex>::Out as Nat>::VAL == 1545);
impl NatIndex for Idx<1546> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1546> as NatIndex>::Out as Nat>::VAL == 1546);
impl NatIndex for Idx<1547> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1547> as NatIndex>::Out as Nat>::VAL == 1547);
impl NatIndex for Idx<1548> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1548> as NatIndex>::Out as Nat>::VAL == 1548);
impl NatIndex for Idx<1549> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1549> as NatIndex>::Out as Nat>::VAL == 1549);
impl NatIndex for Idx<1550> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1550> as NatIndex>::Out as Nat>::VAL == 1550);
impl NatIndex for Idx<1551> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1551> as NatIndex>::Out as Nat>::VAL == 1551);
impl NatIndex for Idx<1552> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1552> as NatIndex>::Out as Nat>::VAL == 1552);
impl NatIndex for Idx<1553> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1553> as NatIndex>::Out as Nat>::VAL == 1553);
impl NatIndex for Idx<1554> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1554> as NatIndex>::Out as Nat>::VAL == 1554);
impl NatIndex for Idx<1555> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1555> as NatIndex>::Out as Nat>::VAL == 1555);
impl NatIndex for Idx<1556> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1556> as NatIndex>::Out as Nat>::VAL == 1556);
impl NatIndex for Idx<1557> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1557> as NatIndex>::Out as Nat>::VAL == 1557);
impl NatIndex for Idx<1558> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1558> as NatIndex>::Out as Nat>::VAL == 1558);
impl NatIndex for Idx<1559> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1559> as NatIndex>::Out as Nat>::VAL == 1559);
impl NatIndex for Idx<1560> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1560> as NatIndex>::Out as Nat>::VAL == 1560);
impl NatIndex for Idx<1561> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1561> as NatIndex>::Out as Nat>::VAL == 1561);
impl NatIndex for Idx<1562> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1562> as NatIndex>::Out as Nat>::VAL == 1562);
impl NatIndex for Idx<1563> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1563> as NatIndex>::Out as Nat>::VAL == 1563);
impl NatIndex for Idx<1564> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1564> as NatIndex>::Out as Nat>::VAL == 1564);
impl NatIndex for Idx<1565> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1565> as NatIndex>::Out as Nat>::VAL == 1565);
impl NatIndex for Idx<1566> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1566> as NatIndex>::Out as Nat>::VAL == 1566);
impl NatIndex for Idx<1567> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1567> as NatIndex>::Out as Nat>::VAL == 1567);
impl NatIndex for Idx<1568> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1568> as NatIndex>::Out as Nat>::VAL == 1568);
impl NatIndex for Idx<1569> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1569> as NatIndex>::Out as Nat>::VAL == 1569);
impl NatIndex for Idx<1570> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1570> as NatIndex>::Out as Nat>::VAL == 1570);
impl NatIndex for Idx<1571> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1571> as NatIndex>::Out as Nat>::VAL == 1571);
impl NatIndex for Idx<1572> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1572> as NatIndex>::Out as Nat>::VAL == 1572);
impl NatIndex for Idx<1573> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1573> as NatIndex>::Out as Nat>::VAL == 1573);
impl NatIndex for Idx<1574> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1574> as NatIndex>::Out as Nat>::VAL == 1574);
impl NatIndex for Idx<1575> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1575> as NatIndex>::Out as Nat>::VAL == 1575);
impl NatIndex for Idx<1576> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1576> as NatIndex>::Out as Nat>::VAL == 1576);
impl NatIndex for Idx<1577> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1577> as NatIndex>::Out as Nat>::VAL == 1577);
impl NatIndex for Idx<1578> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1578> as NatIndex>::Out as Nat>::VAL == 1578);
impl NatIndex for Idx<1579> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1579> as NatIndex>::Out as Nat>::VAL == 1579);
impl NatIndex for Idx<1580> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1580> as NatIndex>::Out as Nat>::VAL == 1580);
impl NatIndex for Idx<1581> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1581> as NatIndex>::Out as Nat>::VAL == 1581);
impl NatIndex for Idx<1582> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1582> as NatIndex>::Out as Nat>::VAL == 1582);
impl NatIndex for Idx<1583> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1583> as NatIndex>::Out as Nat>::VAL == 1583);
impl NatIndex for Idx<1584> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1584> as NatIndex>::Out as Nat>::VAL == 1584);
impl NatIndex for Idx<1585> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1585> as NatIndex>::Out as Nat>::VAL == 1585);
impl NatIndex for Idx<1586> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1586> as NatIndex>::Out as Nat>::VAL == 1586);
impl NatIndex for Idx<1587> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1587> as NatIndex>::Out as Nat>::VAL == 1587);
impl NatIndex for Idx<1588> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1588> as NatIndex>::Out as Nat>::VAL == 1588);
impl NatIndex for Idx<1589> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1589> as NatIndex>::Out as Nat>::VAL == 1589);
impl NatIndex for Idx<1590> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1590> as NatIndex>::Out as Nat>::VAL == 1590);
impl NatIndex for Idx<1591> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1591> as NatIndex>::Out as Nat>::VAL == 1591);
impl NatIndex for Idx<1592> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1592> as NatIndex>::Out as Nat>::VAL == 1592);
impl NatIndex for Idx<1593> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1593> as NatIndex>::Out as Nat>::VAL == 1593);
impl NatIndex for Idx<1594> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1594> as NatIndex>::Out as Nat>::VAL == 1594);
impl NatIndex for Idx<1595> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1595> as NatIndex>::Out as Nat>::VAL == 1595);
impl NatIndex for Idx<1596> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1596> as NatIndex>::Out as Nat>::VAL == 1596);
impl NatIndex for Idx<1597> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1597> as NatIndex>::Out as Nat>::VAL == 1597);
impl NatIndex for Idx<1598> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1598> as NatIndex>::Out as Nat>::VAL == 1598);
impl NatIndex for Idx<1599> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1599> as NatIndex>::Out as Nat>::VAL == 1599);
impl NatIndex for Idx<1600> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1600> as NatIndex>::Out as Nat>::VAL == 1600);
impl NatIndex for Idx<1601> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1601> as NatIndex>::Out as Nat>::VAL == 1601);
impl NatIndex for Idx<1602> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1602> as NatIndex>::Out as Nat>::VAL == 1602);
impl NatIndex for Idx<1603> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1603> as NatIndex>::Out as Nat>::VAL == 1603);
impl NatIndex for Idx<1604> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1604> as NatIndex>::Out as Nat>::VAL == 1604);
impl NatIndex for Idx<1605> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1605> as NatIndex>::Out as Nat>::VAL == 1605);
impl NatIndex for Idx<1606> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1606> as NatIndex>::Out as Nat>::VAL == 1606);
impl NatIndex for Idx<1607> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1607> as NatIndex>::Out as Nat>::VAL == 1607);
impl NatIndex for Idx<1608> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1608> as NatIndex>::Out as Nat>::VAL == 1608);
impl NatIndex for Idx<1609> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1609> as NatIndex>::Out as Nat>::VAL == 1609);
impl NatIndex for Idx<1610> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1610> as NatIndex>::Out as Nat>::VAL == 1610);
impl NatIndex for Idx<1611> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1611> as NatIndex>::Out as Nat>::VAL == 1611);
impl NatIndex for Idx<1612> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1612> as NatIndex>::Out as Nat>::VAL == 1612);
impl NatIndex for Idx<1613> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1613> as NatIndex>::Out as Nat>::VAL == 1613);
impl NatIndex for Idx<1614> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1614> as NatIndex>::Out as Nat>::VAL == 1614);
impl NatIndex for Idx<1615> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1615> as NatIndex>::Out as Nat>::VAL == 1615);
impl NatIndex for Idx<1616> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1616> as NatIndex>::Out as Nat>::VAL == 1616);
impl NatIndex for Idx<1617> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1617> as NatIndex>::Out as Nat>::VAL == 1617);
impl NatIndex for Idx<1618> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1618> as NatIndex>::Out as Nat>::VAL == 1618);
impl NatIndex for Idx<1619> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1619> as NatIndex>::Out as Nat>::VAL == 1619);
impl NatIndex for Idx<1620> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1620> as NatIndex>::Out as Nat>::VAL == 1620);
impl NatIndex for Idx<1621> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1621> as NatIndex>::Out as Nat>::VAL == 1621);
impl NatIndex for Idx<1622> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1622> as NatIndex>::Out as Nat>::VAL == 1622);
impl NatIndex for Idx<1623> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1623> as NatIndex>::Out as Nat>::VAL == 1623);
impl NatIndex for Idx<1624> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1624> as NatIndex>::Out as Nat>::VAL == 1624);
impl NatIndex for Idx<1625> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1625> as NatIndex>::Out as Nat>::VAL == 1625);
impl NatIndex for Idx<1626> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1626> as NatIndex>::Out as Nat>::VAL == 1626);
impl NatIndex for Idx<1627> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1627> as NatIndex>::Out as Nat>::VAL == 1627);
impl NatIndex for Idx<1628> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1628> as NatIndex>::Out as Nat>::VAL == 1628);
impl NatIndex for Idx<1629> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1629> as NatIndex>::Out as Nat>::VAL == 1629);
impl NatIndex for Idx<1630> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1630> as NatIndex>::Out as Nat>::VAL == 1630);
impl NatIndex for Idx<1631> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1631> as NatIndex>::Out as Nat>::VAL == 1631);
impl NatIndex for Idx<1632> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1632> as NatIndex>::Out as Nat>::VAL == 1632);
impl NatIndex for Idx<1633> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1633> as NatIndex>::Out as Nat>::VAL == 1633);
impl NatIndex for Idx<1634> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1634> as NatIndex>::Out as Nat>::VAL == 1634);
impl NatIndex for Idx<1635> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1635> as NatIndex>::Out as Nat>::VAL == 1635);
impl NatIndex for Idx<1636> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1636> as NatIndex>::Out as Nat>::VAL == 1636);
impl NatIndex for Idx<1637> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1637> as NatIndex>::Out as Nat>::VAL == 1637);
impl NatIndex for Idx<1638> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1638> as NatIndex>::Out as Nat>::VAL == 1638);
impl NatIndex for Idx<1639> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1639> as NatIndex>::Out as Nat>::VAL == 1639);
impl NatIndex for Idx<1640> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1640> as NatIndex>::Out as Nat>::VAL == 1640);
impl NatIndex for Idx<1641> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1641> as NatIndex>::Out as Nat>::VAL == 1641);
impl NatIndex for Idx<1642> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1642> as NatIndex>::Out as Nat>::VAL == 1642);
impl NatIndex for Idx<1643> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1643> as NatIndex>::Out as Nat>::VAL == 1643);
impl NatIndex for Idx<1644> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1644> as NatIndex>::Out as Nat>::VAL == 1644);
impl NatIndex for Idx<1645> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1645> as NatIndex>::Out as Nat>::VAL == 1645);
impl NatIndex for Idx<1646> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1646> as NatIndex>::Out as Nat>::VAL == 1646);
impl NatIndex for Idx<1647> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1647> as NatIndex>::Out as Nat>::VAL == 1647);
impl NatIndex for Idx<1648> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1648> as NatIndex>::Out as Nat>::VAL == 1648);
impl NatIndex for Idx<1649> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1649> as NatIndex>::Out as Nat>::VAL == 1649);
impl NatIndex for Idx<1650> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1650> as NatIndex>::Out as Nat>::VAL == 1650);
impl NatIndex for Idx<1651> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1651> as NatIndex>::Out as Nat>::VAL == 1651);
impl NatIndex for Idx<1652> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1652> as NatIndex>::Out as Nat>::VAL == 1652);
impl NatIndex for Idx<1653> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1653> as NatIndex>::Out as Nat>::VAL == 1653);
impl NatIndex for Idx<1654> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1654> as NatIndex>::Out as Nat>::VAL == 1654);
impl NatIndex for Idx<1655> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1655> as NatIndex>::Out as Nat>::VAL == 1655);
impl NatIndex for Idx<1656> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1656> as NatIndex>::Out as Nat>::VAL == 1656);
impl NatIndex for Idx<1657> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1657> as NatIndex>::Out as Nat>::VAL == 1657);
impl NatIndex for Idx<1658> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1658> as NatIndex>::Out as Nat>::VAL == 1658);
impl NatIndex for Idx<1659> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1659> as NatIndex>::Out as Nat>::VAL == 1659);
impl NatIndex for Idx<1660> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1660> as NatIndex>::Out as Nat>::VAL == 1660);
impl NatIndex for Idx<1661> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1661> as NatIndex>::Out as Nat>::VAL == 1661);
impl NatIndex for Idx<1662> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1662> as NatIndex>::Out as Nat>::VAL == 1662);
impl NatIndex for Idx<1663> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1663> as NatIndex>::Out as Nat>::VAL == 1663);
impl NatIndex for Idx<1664> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1664> as NatIndex>::Out as Nat>::VAL == 1664);
impl NatIndex for Idx<1665> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1665> as NatIndex>::Out as Nat>::VAL == 1665);
impl NatIndex for Idx<1666> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1666> as NatIndex>::Out as Nat>::VAL == 1666);
impl NatIndex for Idx<1667> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1667> as NatIndex>::Out as Nat>::VAL == 1667);
impl NatIndex for Idx<1668> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1668> as NatIndex>::Out as Nat>::VAL == 1668);
impl NatIndex for Idx<1669> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1669> as NatIndex>::Out as Nat>::VAL == 1669);
impl NatIndex for Idx<1670> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1670> as NatIndex>::Out as Nat>::VAL == 1670);
impl NatIndex for Idx<1671> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1671> as NatIndex>::Out as Nat>::VAL == 1671);
impl NatIndex for Idx<1672> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1672> as NatIndex>::Out as Nat>::VAL == 1672);
impl NatIndex for Idx<1673> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1673> as NatIndex>::Out as Nat>::VAL == 1673);
impl NatIndex for Idx<1674> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1674> as NatIndex>::Out as Nat>::VAL == 1674);
impl NatIndex for Idx<1675> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1675> as NatIndex>::Out as Nat>::VAL == 1675);
impl NatIndex for Idx<1676> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1676> as NatIndex>::Out as Nat>::VAL == 1676);
impl NatIndex for Idx<1677> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1677> as NatIndex>::Out as Nat>::VAL == 1677);
impl NatIndex for Idx<1678> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1678> as NatIndex>::Out as Nat>::VAL == 1678);
impl NatIndex for Idx<1679> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1679> as NatIndex>::Out as Nat>::VAL == 1679);
impl NatIndex for Idx<1680> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1680> as NatIndex>::Out as Nat>::VAL == 1680);
impl NatIndex for Idx<1681> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1681> as NatIndex>::Out as Nat>::VAL == 1681);
impl NatIndex for Idx<1682> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1682> as NatIndex>::Out as Nat>::VAL == 1682);
impl NatIndex for Idx<1683> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1683> as NatIndex>::Out as Nat>::VAL == 1683);
impl NatIndex for Idx<1684> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1684> as NatIndex>::Out as Nat>::VAL == 1684);
impl NatIndex for Idx<1685> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1685> as NatIndex>::Out as Nat>::VAL == 1685);
impl NatIndex for Idx<1686> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1686> as NatIndex>::Out as Nat>::VAL == 1686);
impl NatIndex for Idx<1687> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1687> as NatIndex>::Out as Nat>::VAL == 1687);
impl NatIndex for Idx<1688> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1688> as NatIndex>::Out as Nat>::VAL == 1688);
impl NatIndex for Idx<1689> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1689> as NatIndex>::Out as Nat>::VAL == 1689);
impl NatIndex for Idx<1690> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1690> as NatIndex>::Out as Nat>::VAL == 1690);
impl NatIndex for Idx<1691> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1691> as NatIndex>::Out as Nat>::VAL == 1691);
impl NatIndex for Idx<1692> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1692> as NatIndex>::Out as Nat>::VAL == 1692);
impl NatIndex for Idx<1693> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1693> as NatIndex>::Out as Nat>::VAL == 1693);
impl NatIndex for Idx<1694> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1694> as NatIndex>::Out as Nat>::VAL == 1694);
impl NatIndex for Idx<1695> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1695> as NatIndex>::Out as Nat>::VAL == 1695);
impl NatIndex for Idx<1696> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1696> as NatIndex>::Out as Nat>::VAL == 1696);
impl NatIndex for Idx<1697> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1697> as NatIndex>::Out as Nat>::VAL == 1697);
impl NatIndex for Idx<1698> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1698> as NatIndex>::Out as Nat>::VAL == 1698);
impl NatIndex for Idx<1699> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1699> as NatIndex>::Out as Nat>::VAL == 1699);
impl NatIndex for Idx<1700> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1700> as NatIndex>::Out as Nat>::VAL == 1700);
impl NatIndex for Idx<1701> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1701> as NatIndex>::Out as Nat>::VAL == 1701);
impl NatIndex for Idx<1702> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1702> as NatIndex>::Out as Nat>::VAL == 1702);
impl NatIndex for Idx<1703> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1703> as NatIndex>::Out as Nat>::VAL == 1703);
impl NatIndex for Idx<1704> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1704> as NatIndex>::Out as Nat>::VAL == 1704);
impl NatIndex for Idx<1705> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1705> as NatIndex>::Out as Nat>::VAL == 1705);
impl NatIndex for Idx<1706> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1706> as NatIndex>::Out as Nat>::VAL == 1706);
impl NatIndex for Idx<1707> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1707> as NatIndex>::Out as Nat>::VAL == 1707);
impl NatIndex for Idx<1708> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1708> as NatIndex>::Out as Nat>::VAL == 1708);
impl NatIndex for Idx<1709> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1709> as NatIndex>::Out as Nat>::VAL == 1709);
impl NatIndex for Idx<1710> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1710> as NatIndex>::Out as Nat>::VAL == 1710);
impl NatIndex for Idx<1711> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1711> as NatIndex>::Out as Nat>::VAL == 1711);
impl NatIndex for Idx<1712> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1712> as NatIndex>::Out as Nat>::VAL == 1712);
impl NatIndex for Idx<1713> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1713> as NatIndex>::Out as Nat>::VAL == 1713);
impl NatIndex for Idx<1714> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1714> as NatIndex>::Out as Nat>::VAL == 1714);
impl NatIndex for Idx<1715> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1715> as NatIndex>::Out as Nat>::VAL == 1715);
impl NatIndex for Idx<1716> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1716> as NatIndex>::Out as Nat>::VAL == 1716);
impl NatIndex for Idx<1717> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1717> as NatIndex>::Out as Nat>::VAL == 1717);
impl NatIndex for Idx<1718> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1718> as NatIndex>::Out as Nat>::VAL == 1718);
impl NatIndex for Idx<1719> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1719> as NatIndex>::Out as Nat>::VAL == 1719);
impl NatIndex for Idx<1720> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1720> as NatIndex>::Out as Nat>::VAL == 1720);
impl NatIndex for Idx<1721> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1721> as NatIndex>::Out as Nat>::VAL == 1721);
impl NatIndex for Idx<1722> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1722> as NatIndex>::Out as Nat>::VAL == 1722);
impl NatIndex for Idx<1723> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1723> as NatIndex>::Out as Nat>::VAL == 1723);
impl NatIndex for Idx<1724> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1724> as NatIndex>::Out as Nat>::VAL == 1724);
impl NatIndex for Idx<1725> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1725> as NatIndex>::Out as Nat>::VAL == 1725);
impl NatIndex for Idx<1726> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1726> as NatIndex>::Out as Nat>::VAL == 1726);
impl NatIndex for Idx<1727> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1727> as NatIndex>::Out as Nat>::VAL == 1727);
impl NatIndex for Idx<1728> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1728> as NatIndex>::Out as Nat>::VAL == 1728);
impl NatIndex for Idx<1729> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1729> as NatIndex>::Out as Nat>::VAL == 1729);
impl NatIndex for Idx<1730> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1730> as NatIndex>::Out as Nat>::VAL == 1730);
impl NatIndex for Idx<1731> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1731> as NatIndex>::Out as Nat>::VAL == 1731);
impl NatIndex for Idx<1732> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1732> as NatIndex>::Out as Nat>::VAL == 1732);
impl NatIndex for Idx<1733> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1733> as NatIndex>::Out as Nat>::VAL == 1733);
impl NatIndex for Idx<1734> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1734> as NatIndex>::Out as Nat>::VAL == 1734);
impl NatIndex for Idx<1735> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1735> as NatIndex>::Out as Nat>::VAL == 1735);
impl NatIndex for Idx<1736> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1736> as NatIndex>::Out as Nat>::VAL == 1736);
impl NatIndex for Idx<1737> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1737> as NatIndex>::Out as Nat>::VAL == 1737);
impl NatIndex for Idx<1738> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1738> as NatIndex>::Out as Nat>::VAL == 1738);
impl NatIndex for Idx<1739> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1739> as NatIndex>::Out as Nat>::VAL == 1739);
impl NatIndex for Idx<1740> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1740> as NatIndex>::Out as Nat>::VAL == 1740);
impl NatIndex for Idx<1741> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1741> as NatIndex>::Out as Nat>::VAL == 1741);
impl NatIndex for Idx<1742> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1742> as NatIndex>::Out as Nat>::VAL == 1742);
impl NatIndex for Idx<1743> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1743> as NatIndex>::Out as Nat>::VAL == 1743);
impl NatIndex for Idx<1744> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1744> as NatIndex>::Out as Nat>::VAL == 1744);
impl NatIndex for Idx<1745> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1745> as NatIndex>::Out as Nat>::VAL == 1745);
impl NatIndex for Idx<1746> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1746> as NatIndex>::Out as Nat>::VAL == 1746);
impl NatIndex for Idx<1747> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1747> as NatIndex>::Out as Nat>::VAL == 1747);
impl NatIndex for Idx<1748> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1748> as NatIndex>::Out as Nat>::VAL == 1748);
impl NatIndex for Idx<1749> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1749> as NatIndex>::Out as Nat>::VAL == 1749);
impl NatIndex for Idx<1750> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1750> as NatIndex>::Out as Nat>::VAL == 1750);
impl NatIndex for Idx<1751> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1751> as NatIndex>::Out as Nat>::VAL == 1751);
impl NatIndex for Idx<1752> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1752> as NatIndex>::Out as Nat>::VAL == 1752);
impl NatIndex for Idx<1753> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1753> as NatIndex>::Out as Nat>::VAL == 1753);
impl NatIndex for Idx<1754> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1754> as NatIndex>::Out as Nat>::VAL == 1754);
impl NatIndex for Idx<1755> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1755> as NatIndex>::Out as Nat>::VAL == 1755);
impl NatIndex for Idx<1756> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1756> as NatIndex>::Out as Nat>::VAL == 1756);
impl NatIndex for Idx<1757> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1757> as NatIndex>::Out as Nat>::VAL == 1757);
impl NatIndex for Idx<1758> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1758> as NatIndex>::Out as Nat>::VAL == 1758);
impl NatIndex for Idx<1759> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1759> as NatIndex>::Out as Nat>::VAL == 1759);
impl NatIndex for Idx<1760> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1760> as NatIndex>::Out as Nat>::VAL == 1760);
impl NatIndex for Idx<1761> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1761> as NatIndex>::Out as Nat>::VAL == 1761);
impl NatIndex for Idx<1762> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1762> as NatIndex>::Out as Nat>::VAL == 1762);
impl NatIndex for Idx<1763> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1763> as NatIndex>::Out as Nat>::VAL == 1763);
impl NatIndex for Idx<1764> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1764> as NatIndex>::Out as Nat>::VAL == 1764);
impl NatIndex for Idx<1765> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1765> as NatIndex>::Out as Nat>::VAL == 1765);
impl NatIndex for Idx<1766> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1766> as NatIndex>::Out as Nat>::VAL == 1766);
impl NatIndex for Idx<1767> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1767> as NatIndex>::Out as Nat>::VAL == 1767);
impl NatIndex for Idx<1768> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1768> as NatIndex>::Out as Nat>::VAL == 1768);
impl NatIndex for Idx<1769> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1769> as NatIndex>::Out as Nat>::VAL == 1769);
impl NatIndex for Idx<1770> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1770> as NatIndex>::Out as Nat>::VAL == 1770);
impl NatIndex for Idx<1771> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1771> as NatIndex>::Out as Nat>::VAL == 1771);
impl NatIndex for Idx<1772> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1772> as NatIndex>::Out as Nat>::VAL == 1772);
impl NatIndex for Idx<1773> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1773> as NatIndex>::Out as Nat>::VAL == 1773);
impl NatIndex for Idx<1774> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1774> as NatIndex>::Out as Nat>::VAL == 1774);
impl NatIndex for Idx<1775> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1775> as NatIndex>::Out as Nat>::VAL == 1775);
impl NatIndex for Idx<1776> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1776> as NatIndex>::Out as Nat>::VAL == 1776);
impl NatIndex for Idx<1777> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1777> as NatIndex>::Out as Nat>::VAL == 1777);
impl NatIndex for Idx<1778> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1778> as NatIndex>::Out as Nat>::VAL == 1778);
impl NatIndex for Idx<1779> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1779> as NatIndex>::Out as Nat>::VAL == 1779);
impl NatIndex for Idx<1780> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1780> as NatIndex>::Out as Nat>::VAL == 1780);
impl NatIndex for Idx<1781> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1781> as NatIndex>::Out as Nat>::VAL == 1781);
impl NatIndex for Idx<1782> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1782> as NatIndex>::Out as Nat>::VAL == 1782);
impl NatIndex for Idx<1783> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1783> as NatIndex>::Out as Nat>::VAL == 1783);
impl NatIndex for Idx<1784> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1784> as NatIndex>::Out as Nat>::VAL == 1784);
impl NatIndex for Idx<1785> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1785> as NatIndex>::Out as Nat>::VAL == 1785);
impl NatIndex for Idx<1786> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1786> as NatIndex>::Out as Nat>::VAL == 1786);
impl NatIndex for Idx<1787> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1787> as NatIndex>::Out as Nat>::VAL == 1787);
impl NatIndex for Idx<1788> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1788> as NatIndex>::Out as Nat>::VAL == 1788);
impl NatIndex for Idx<1789> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1789> as NatIndex>::Out as Nat>::VAL == 1789);
impl NatIndex for Idx<1790> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1790> as NatIndex>::Out as Nat>::VAL == 1790);
impl NatIndex for Idx<1791> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1791> as NatIndex>::Out as Nat>::VAL == 1791);
impl NatIndex for Idx<1792> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1792> as NatIndex>::Out as Nat>::VAL == 1792);
impl NatIndex for Idx<1793> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1793> as NatIndex>::Out as Nat>::VAL == 1793);
impl NatIndex for Idx<1794> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1794> as NatIndex>::Out as Nat>::VAL == 1794);
impl NatIndex for Idx<1795> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1795> as NatIndex>::Out as Nat>::VAL == 1795);
impl NatIndex for Idx<1796> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1796> as NatIndex>::Out as Nat>::VAL == 1796);
impl NatIndex for Idx<1797> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1797> as NatIndex>::Out as Nat>::VAL == 1797);
impl NatIndex for Idx<1798> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1798> as NatIndex>::Out as Nat>::VAL == 1798);
impl NatIndex for Idx<1799> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1799> as NatIndex>::Out as Nat>::VAL == 1799);
impl NatIndex for Idx<1800> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1800> as NatIndex>::Out as Nat>::VAL == 1800);
impl NatIndex for Idx<1801> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1801> as NatIndex>::Out as Nat>::VAL == 1801);
impl NatIndex for Idx<1802> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1802> as NatIndex>::Out as Nat>::VAL == 1802);
impl NatIndex for Idx<1803> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1803> as NatIndex>::Out as Nat>::VAL == 1803);
impl NatIndex for Idx<1804> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1804> as NatIndex>::Out as Nat>::VAL == 1804);
impl NatIndex for Idx<1805> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1805> as NatIndex>::Out as Nat>::VAL == 1805);
impl NatIndex for Idx<1806> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1806> as NatIndex>::Out as Nat>::VAL == 1806);
impl NatIndex for Idx<1807> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1807> as NatIndex>::Out as Nat>::VAL == 1807);
impl NatIndex for Idx<1808> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1808> as NatIndex>::Out as Nat>::VAL == 1808);
impl NatIndex for Idx<1809> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1809> as NatIndex>::Out as Nat>::VAL == 1809);
impl NatIndex for Idx<1810> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1810> as NatIndex>::Out as Nat>::VAL == 1810);
impl NatIndex for Idx<1811> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1811> as NatIndex>::Out as Nat>::VAL == 1811);
impl NatIndex for Idx<1812> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1812> as NatIndex>::Out as Nat>::VAL == 1812);
impl NatIndex for Idx<1813> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1813> as NatIndex>::Out as Nat>::VAL == 1813);
impl NatIndex for Idx<1814> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1814> as NatIndex>::Out as Nat>::VAL == 1814);
impl NatIndex for Idx<1815> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1815> as NatIndex>::Out as Nat>::VAL == 1815);
impl NatIndex for Idx<1816> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1816> as NatIndex>::Out as Nat>::VAL == 1816);
impl NatIndex for Idx<1817> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1817> as NatIndex>::Out as Nat>::VAL == 1817);
impl NatIndex for Idx<1818> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1818> as NatIndex>::Out as Nat>::VAL == 1818);
impl NatIndex for Idx<1819> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1819> as NatIndex>::Out as Nat>::VAL == 1819);
impl NatIndex for Idx<1820> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1820> as NatIndex>::Out as Nat>::VAL == 1820);
impl NatIndex for Idx<1821> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1821> as NatIndex>::Out as Nat>::VAL == 1821);
impl NatIndex for Idx<1822> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1822> as NatIndex>::Out as Nat>::VAL == 1822);
impl NatIndex for Idx<1823> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1823> as NatIndex>::Out as Nat>::VAL == 1823);
impl NatIndex for Idx<1824> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1824> as NatIndex>::Out as Nat>::VAL == 1824);
impl NatIndex for Idx<1825> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1825> as NatIndex>::Out as Nat>::VAL == 1825);
impl NatIndex for Idx<1826> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1826> as NatIndex>::Out as Nat>::VAL == 1826);
impl NatIndex for Idx<1827> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1827> as NatIndex>::Out as Nat>::VAL == 1827);
impl NatIndex for Idx<1828> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1828> as NatIndex>::Out as Nat>::VAL == 1828);
impl NatIndex for Idx<1829> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1829> as NatIndex>::Out as Nat>::VAL == 1829);
impl NatIndex for Idx<1830> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1830> as NatIndex>::Out as Nat>::VAL == 1830);
impl NatIndex for Idx<1831> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1831> as NatIndex>::Out as Nat>::VAL == 1831);
impl NatIndex for Idx<1832> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1832> as NatIndex>::Out as Nat>::VAL == 1832);
impl NatIndex for Idx<1833> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1833> as NatIndex>::Out as Nat>::VAL == 1833);
impl NatIndex for Idx<1834> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1834> as NatIndex>::Out as Nat>::VAL == 1834);
impl NatIndex for Idx<1835> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1835> as NatIndex>::Out as Nat>::VAL == 1835);
impl NatIndex for Idx<1836> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1836> as NatIndex>::Out as Nat>::VAL == 1836);
impl NatIndex for Idx<1837> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1837> as NatIndex>::Out as Nat>::VAL == 1837);
impl NatIndex for Idx<1838> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1838> as NatIndex>::Out as Nat>::VAL == 1838);
impl NatIndex for Idx<1839> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1839> as NatIndex>::Out as Nat>::VAL == 1839);
impl NatIndex for Idx<1840> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1840> as NatIndex>::Out as Nat>::VAL == 1840);
impl NatIndex for Idx<1841> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1841> as NatIndex>::Out as Nat>::VAL == 1841);
impl NatIndex for Idx<1842> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1842> as NatIndex>::Out as Nat>::VAL == 1842);
impl NatIndex for Idx<1843> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1843> as NatIndex>::Out as Nat>::VAL == 1843);
impl NatIndex for Idx<1844> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1844> as NatIndex>::Out as Nat>::VAL == 1844);
impl NatIndex for Idx<1845> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1845> as NatIndex>::Out as Nat>::VAL == 1845);
impl NatIndex for Idx<1846> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1846> as NatIndex>::Out as Nat>::VAL == 1846);
impl NatIndex for Idx<1847> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1847> as NatIndex>::Out as Nat>::VAL == 1847);
impl NatIndex for Idx<1848> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1848> as NatIndex>::Out as Nat>::VAL == 1848);
impl NatIndex for Idx<1849> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1849> as NatIndex>::Out as Nat>::VAL == 1849);
impl NatIndex for Idx<1850> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1850> as NatIndex>::Out as Nat>::VAL == 1850);
impl NatIndex for Idx<1851> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1851> as NatIndex>::Out as Nat>::VAL == 1851);
impl NatIndex for Idx<1852> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1852> as NatIndex>::Out as Nat>::VAL == 1852);
impl NatIndex for Idx<1853> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1853> as NatIndex>::Out as Nat>::VAL == 1853);
impl NatIndex for Idx<1854> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1854> as NatIndex>::Out as Nat>::VAL == 1854);
impl NatIndex for Idx<1855> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1855> as NatIndex>::Out as Nat>::VAL == 1855);
impl NatIndex for Idx<1856> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1856> as NatIndex>::Out as Nat>::VAL == 1856);
impl NatIndex for Idx<1857> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1857> as NatIndex>::Out as Nat>::VAL == 1857);
impl NatIndex for Idx<1858> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1858> as NatIndex>::Out as Nat>::VAL == 1858);
impl NatIndex for Idx<1859> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1859> as NatIndex>::Out as Nat>::VAL == 1859);
impl NatIndex for Idx<1860> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1860> as NatIndex>::Out as Nat>::VAL == 1860);
impl NatIndex for Idx<1861> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1861> as NatIndex>::Out as Nat>::VAL == 1861);
impl NatIndex for Idx<1862> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1862> as NatIndex>::Out as Nat>::VAL == 1862);
impl NatIndex for Idx<1863> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1863> as NatIndex>::Out as Nat>::VAL == 1863);
impl NatIndex for Idx<1864> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1864> as NatIndex>::Out as Nat>::VAL == 1864);
impl NatIndex for Idx<1865> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1865> as NatIndex>::Out as Nat>::VAL == 1865);
impl NatIndex for Idx<1866> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1866> as NatIndex>::Out as Nat>::VAL == 1866);
impl NatIndex for Idx<1867> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1867> as NatIndex>::Out as Nat>::VAL == 1867);
impl NatIndex for Idx<1868> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1868> as NatIndex>::Out as Nat>::VAL == 1868);
impl NatIndex for Idx<1869> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1869> as NatIndex>::Out as Nat>::VAL == 1869);
impl NatIndex for Idx<1870> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1870> as NatIndex>::Out as Nat>::VAL == 1870);
impl NatIndex for Idx<1871> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1871> as NatIndex>::Out as Nat>::VAL == 1871);
impl NatIndex for Idx<1872> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1872> as NatIndex>::Out as Nat>::VAL == 1872);
impl NatIndex for Idx<1873> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1873> as NatIndex>::Out as Nat>::VAL == 1873);
impl NatIndex for Idx<1874> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1874> as NatIndex>::Out as Nat>::VAL == 1874);
impl NatIndex for Idx<1875> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1875> as NatIndex>::Out as Nat>::VAL == 1875);
impl NatIndex for Idx<1876> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1876> as NatIndex>::Out as Nat>::VAL == 1876);
impl NatIndex for Idx<1877> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1877> as NatIndex>::Out as Nat>::VAL == 1877);
impl NatIndex for Idx<1878> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1878> as NatIndex>::Out as Nat>::VAL == 1878);
impl NatIndex for Idx<1879> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1879> as NatIndex>::Out as Nat>::VAL == 1879);
impl NatIndex for Idx<1880> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1880> as NatIndex>::Out as Nat>::VAL == 1880);
impl NatIndex for Idx<1881> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1881> as NatIndex>::Out as Nat>::VAL == 1881);
impl NatIndex for Idx<1882> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1882> as NatIndex>::Out as Nat>::VAL == 1882);
impl NatIndex for Idx<1883> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1883> as NatIndex>::Out as Nat>::VAL == 1883);
impl NatIndex for Idx<1884> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1884> as NatIndex>::Out as Nat>::VAL == 1884);
impl NatIndex for Idx<1885> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1885> as NatIndex>::Out as Nat>::VAL == 1885);
impl NatIndex for Idx<1886> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1886> as NatIndex>::Out as Nat>::VAL == 1886);
impl NatIndex for Idx<1887> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1887> as NatIndex>::Out as Nat>::VAL == 1887);
impl NatIndex for Idx<1888> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1888> as NatIndex>::Out as Nat>::VAL == 1888);
impl NatIndex for Idx<1889> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1889> as NatIndex>::Out as Nat>::VAL == 1889);
impl NatIndex for Idx<1890> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1890> as NatIndex>::Out as Nat>::VAL == 1890);
impl NatIndex for Idx<1891> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1891> as NatIndex>::Out as Nat>::VAL == 1891);
impl NatIndex for Idx<1892> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1892> as NatIndex>::Out as Nat>::VAL == 1892);
impl NatIndex for Idx<1893> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1893> as NatIndex>::Out as Nat>::VAL == 1893);
impl NatIndex for Idx<1894> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1894> as NatIndex>::Out as Nat>::VAL == 1894);
impl NatIndex for Idx<1895> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1895> as NatIndex>::Out as Nat>::VAL == 1895);
impl NatIndex for Idx<1896> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1896> as NatIndex>::Out as Nat>::VAL == 1896);
impl NatIndex for Idx<1897> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1897> as NatIndex>::Out as Nat>::VAL == 1897);
impl NatIndex for Idx<1898> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1898> as NatIndex>::Out as Nat>::VAL == 1898);
impl NatIndex for Idx<1899> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1899> as NatIndex>::Out as Nat>::VAL == 1899);
impl NatIndex for Idx<1900> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1900> as NatIndex>::Out as Nat>::VAL == 1900);
impl NatIndex for Idx<1901> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1901> as NatIndex>::Out as Nat>::VAL == 1901);
impl NatIndex for Idx<1902> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1902> as NatIndex>::Out as Nat>::VAL == 1902);
impl NatIndex for Idx<1903> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1903> as NatIndex>::Out as Nat>::VAL == 1903);
impl NatIndex for Idx<1904> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1904> as NatIndex>::Out as Nat>::VAL == 1904);
impl NatIndex for Idx<1905> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1905> as NatIndex>::Out as Nat>::VAL == 1905);
impl NatIndex for Idx<1906> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1906> as NatIndex>::Out as Nat>::VAL == 1906);
impl NatIndex for Idx<1907> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1907> as NatIndex>::Out as Nat>::VAL == 1907);
impl NatIndex for Idx<1908> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1908> as NatIndex>::Out as Nat>::VAL == 1908);
impl NatIndex for Idx<1909> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1909> as NatIndex>::Out as Nat>::VAL == 1909);
impl NatIndex for Idx<1910> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1910> as NatIndex>::Out as Nat>::VAL == 1910);
impl NatIndex for Idx<1911> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1911> as NatIndex>::Out as Nat>::VAL == 1911);
impl NatIndex for Idx<1912> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1912> as NatIndex>::Out as Nat>::VAL == 1912);
impl NatIndex for Idx<1913> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1913> as NatIndex>::Out as Nat>::VAL == 1913);
impl NatIndex for Idx<1914> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1914> as NatIndex>::Out as Nat>::VAL == 1914);
impl NatIndex for Idx<1915> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1915> as NatIndex>::Out as Nat>::VAL == 1915);
impl NatIndex for Idx<1916> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1916> as NatIndex>::Out as Nat>::VAL == 1916);
impl NatIndex for Idx<1917> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1917> as NatIndex>::Out as Nat>::VAL == 1917);
impl NatIndex for Idx<1918> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1918> as NatIndex>::Out as Nat>::VAL == 1918);
impl NatIndex for Idx<1919> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1919> as NatIndex>::Out as Nat>::VAL == 1919);
impl NatIndex for Idx<1920> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1920> as NatIndex>::Out as Nat>::VAL == 1920);
impl NatIndex for Idx<1921> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1921> as NatIndex>::Out as Nat>::VAL == 1921);
impl NatIndex for Idx<1922> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1922> as NatIndex>::Out as Nat>::VAL == 1922);
impl NatIndex for Idx<1923> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1923> as NatIndex>::Out as Nat>::VAL == 1923);
impl NatIndex for Idx<1924> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1924> as NatIndex>::Out as Nat>::VAL == 1924);
impl NatIndex for Idx<1925> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1925> as NatIndex>::Out as Nat>::VAL == 1925);
impl NatIndex for Idx<1926> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1926> as NatIndex>::Out as Nat>::VAL == 1926);
impl NatIndex for Idx<1927> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1927> as NatIndex>::Out as Nat>::VAL == 1927);
impl NatIndex for Idx<1928> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1928> as NatIndex>::Out as Nat>::VAL == 1928);
impl NatIndex for Idx<1929> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1929> as NatIndex>::Out as Nat>::VAL == 1929);
impl NatIndex for Idx<1930> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1930> as NatIndex>::Out as Nat>::VAL == 1930);
impl NatIndex for Idx<1931> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1931> as NatIndex>::Out as Nat>::VAL == 1931);
impl NatIndex for Idx<1932> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1932> as NatIndex>::Out as Nat>::VAL == 1932);
impl NatIndex for Idx<1933> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1933> as NatIndex>::Out as Nat>::VAL == 1933);
impl NatIndex for Idx<1934> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1934> as NatIndex>::Out as Nat>::VAL == 1934);
impl NatIndex for Idx<1935> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1935> as NatIndex>::Out as Nat>::VAL == 1935);
impl NatIndex for Idx<1936> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1936> as NatIndex>::Out as Nat>::VAL == 1936);
impl NatIndex for Idx<1937> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1937> as NatIndex>::Out as Nat>::VAL == 1937);
impl NatIndex for Idx<1938> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1938> as NatIndex>::Out as Nat>::VAL == 1938);
impl NatIndex for Idx<1939> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1939> as NatIndex>::Out as Nat>::VAL == 1939);
impl NatIndex for Idx<1940> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1940> as NatIndex>::Out as Nat>::VAL == 1940);
impl NatIndex for Idx<1941> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1941> as NatIndex>::Out as Nat>::VAL == 1941);
impl NatIndex for Idx<1942> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1942> as NatIndex>::Out as Nat>::VAL == 1942);
impl NatIndex for Idx<1943> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1943> as NatIndex>::Out as Nat>::VAL == 1943);
impl NatIndex for Idx<1944> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1944> as NatIndex>::Out as Nat>::VAL == 1944);
impl NatIndex for Idx<1945> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1945> as NatIndex>::Out as Nat>::VAL == 1945);
impl NatIndex for Idx<1946> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1946> as NatIndex>::Out as Nat>::VAL == 1946);
impl NatIndex for Idx<1947> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1947> as NatIndex>::Out as Nat>::VAL == 1947);
impl NatIndex for Idx<1948> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1948> as NatIndex>::Out as Nat>::VAL == 1948);
impl NatIndex for Idx<1949> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1949> as NatIndex>::Out as Nat>::VAL == 1949);
impl NatIndex for Idx<1950> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1950> as NatIndex>::Out as Nat>::VAL == 1950);
impl NatIndex for Idx<1951> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1951> as NatIndex>::Out as Nat>::VAL == 1951);
impl NatIndex for Idx<1952> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1952> as NatIndex>::Out as Nat>::VAL == 1952);
impl NatIndex for Idx<1953> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1953> as NatIndex>::Out as Nat>::VAL == 1953);
impl NatIndex for Idx<1954> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1954> as NatIndex>::Out as Nat>::VAL == 1954);
impl NatIndex for Idx<1955> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1955> as NatIndex>::Out as Nat>::VAL == 1955);
impl NatIndex for Idx<1956> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1956> as NatIndex>::Out as Nat>::VAL == 1956);
impl NatIndex for Idx<1957> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1957> as NatIndex>::Out as Nat>::VAL == 1957);
impl NatIndex for Idx<1958> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1958> as NatIndex>::Out as Nat>::VAL == 1958);
impl NatIndex for Idx<1959> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1959> as NatIndex>::Out as Nat>::VAL == 1959);
impl NatIndex for Idx<1960> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1960> as NatIndex>::Out as Nat>::VAL == 1960);
impl NatIndex for Idx<1961> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1961> as NatIndex>::Out as Nat>::VAL == 1961);
impl NatIndex for Idx<1962> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1962> as NatIndex>::Out as Nat>::VAL == 1962);
impl NatIndex for Idx<1963> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1963> as NatIndex>::Out as Nat>::VAL == 1963);
impl NatIndex for Idx<1964> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1964> as NatIndex>::Out as Nat>::VAL == 1964);
impl NatIndex for Idx<1965> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1965> as NatIndex>::Out as Nat>::VAL == 1965);
impl NatIndex for Idx<1966> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1966> as NatIndex>::Out as Nat>::VAL == 1966);
impl NatIndex for Idx<1967> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1967> as NatIndex>::Out as Nat>::VAL == 1967);
impl NatIndex for Idx<1968> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1968> as NatIndex>::Out as Nat>::VAL == 1968);
impl NatIndex for Idx<1969> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1969> as NatIndex>::Out as Nat>::VAL == 1969);
impl NatIndex for Idx<1970> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1970> as NatIndex>::Out as Nat>::VAL == 1970);
impl NatIndex for Idx<1971> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1971> as NatIndex>::Out as Nat>::VAL == 1971);
impl NatIndex for Idx<1972> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1972> as NatIndex>::Out as Nat>::VAL == 1972);
impl NatIndex for Idx<1973> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1973> as NatIndex>::Out as Nat>::VAL == 1973);
impl NatIndex for Idx<1974> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1974> as NatIndex>::Out as Nat>::VAL == 1974);
impl NatIndex for Idx<1975> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1975> as NatIndex>::Out as Nat>::VAL == 1975);
impl NatIndex for Idx<1976> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1976> as NatIndex>::Out as Nat>::VAL == 1976);
impl NatIndex for Idx<1977> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1977> as NatIndex>::Out as Nat>::VAL == 1977);
impl NatIndex for Idx<1978> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1978> as NatIndex>::Out as Nat>::VAL == 1978);
impl NatIndex for Idx<1979> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1979> as NatIndex>::Out as Nat>::VAL == 1979);
impl NatIndex for Idx<1980> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1980> as NatIndex>::Out as Nat>::VAL == 1980);
impl NatIndex for Idx<1981> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1981> as NatIndex>::Out as Nat>::VAL == 1981);
impl NatIndex for Idx<1982> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1982> as NatIndex>::Out as Nat>::VAL == 1982);
impl NatIndex for Idx<1983> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1983> as NatIndex>::Out as Nat>::VAL == 1983);
impl NatIndex for Idx<1984> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1984> as NatIndex>::Out as Nat>::VAL == 1984);
impl NatIndex for Idx<1985> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1985> as NatIndex>::Out as Nat>::VAL == 1985);
impl NatIndex for Idx<1986> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1986> as NatIndex>::Out as Nat>::VAL == 1986);
impl NatIndex for Idx<1987> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1987> as NatIndex>::Out as Nat>::VAL == 1987);
impl NatIndex for Idx<1988> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1988> as NatIndex>::Out as Nat>::VAL == 1988);
impl NatIndex for Idx<1989> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1989> as NatIndex>::Out as Nat>::VAL == 1989);
impl NatIndex for Idx<1990> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1990> as NatIndex>::Out as Nat>::VAL == 1990);
impl NatIndex for Idx<1991> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1991> as NatIndex>::Out as Nat>::VAL == 1991);
impl NatIndex for Idx<1992> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1992> as NatIndex>::Out as Nat>::VAL == 1992);
impl NatIndex for Idx<1993> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1993> as NatIndex>::Out as Nat>::VAL == 1993);
impl NatIndex for Idx<1994> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1994> as NatIndex>::Out as Nat>::VAL == 1994);
impl NatIndex for Idx<1995> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1995> as NatIndex>::Out as Nat>::VAL == 1995);
impl NatIndex for Idx<1996> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1996> as NatIndex>::Out as Nat>::VAL == 1996);
impl NatIndex for Idx<1997> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1997> as NatIndex>::Out as Nat>::VAL == 1997);
impl NatIndex for Idx<1998> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1998> as NatIndex>::Out as Nat>::VAL == 1998);
impl NatIndex for Idx<1999> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<1999> as NatIndex>::Out as Nat>::VAL == 1999);
impl NatIndex for Idx<2000> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2000> as NatIndex>::Out as Nat>::VAL == 2000);
impl NatIndex for Idx<2001> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2001> as NatIndex>::Out as Nat>::VAL == 2001);
impl NatIndex for Idx<2002> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2002> as NatIndex>::Out as Nat>::VAL == 2002);
impl NatIndex for Idx<2003> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2003> as NatIndex>::Out as Nat>::VAL == 2003);
impl NatIndex for Idx<2004> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2004> as NatIndex>::Out as Nat>::VAL == 2004);
impl NatIndex for Idx<2005> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2005> as NatIndex>::Out as Nat>::VAL == 2005);
impl NatIndex for Idx<2006> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2006> as NatIndex>::Out as Nat>::VAL == 2006);
impl NatIndex for Idx<2007> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2007> as NatIndex>::Out as Nat>::VAL == 2007);
impl NatIndex for Idx<2008> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2008> as NatIndex>::Out as Nat>::VAL == 2008);
impl NatIndex for Idx<2009> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2009> as NatIndex>::Out as Nat>::VAL == 2009);
impl NatIndex for Idx<2010> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2010> as NatIndex>::Out as Nat>::VAL == 2010);
impl NatIndex for Idx<2011> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2011> as NatIndex>::Out as Nat>::VAL == 2011);
impl NatIndex for Idx<2012> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2012> as NatIndex>::Out as Nat>::VAL == 2012);
impl NatIndex for Idx<2013> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2013> as NatIndex>::Out as Nat>::VAL == 2013);
impl NatIndex for Idx<2014> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2014> as NatIndex>::Out as Nat>::VAL == 2014);
impl NatIndex for Idx<2015> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2015> as NatIndex>::Out as Nat>::VAL == 2015);
impl NatIndex for Idx<2016> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2016> as NatIndex>::Out as Nat>::VAL == 2016);
impl NatIndex for Idx<2017> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2017> as NatIndex>::Out as Nat>::VAL == 2017);
impl NatIndex for Idx<2018> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2018> as NatIndex>::Out as Nat>::VAL == 2018);
impl NatIndex for Idx<2019> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2019> as NatIndex>::Out as Nat>::VAL == 2019);
impl NatIndex for Idx<2020> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2020> as NatIndex>::Out as Nat>::VAL == 2020);
impl NatIndex for Idx<2021> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2021> as NatIndex>::Out as Nat>::VAL == 2021);
impl NatIndex for Idx<2022> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2022> as NatIndex>::Out as Nat>::VAL == 2022);
impl NatIndex for Idx<2023> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2023> as NatIndex>::Out as Nat>::VAL == 2023);
impl NatIndex for Idx<2024> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2024> as NatIndex>::Out as Nat>::VAL == 2024);
impl NatIndex for Idx<2025> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2025> as NatIndex>::Out as Nat>::VAL == 2025);
impl NatIndex for Idx<2026> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2026> as NatIndex>::Out as Nat>::VAL == 2026);
impl NatIndex for Idx<2027> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2027> as NatIndex>::Out as Nat>::VAL == 2027);
impl NatIndex for Idx<2028> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2028> as NatIndex>::Out as Nat>::VAL == 2028);
impl NatIndex for Idx<2029> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2029> as NatIndex>::Out as Nat>::VAL == 2029);
impl NatIndex for Idx<2030> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2030> as NatIndex>::Out as Nat>::VAL == 2030);
impl NatIndex for Idx<2031> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2031> as NatIndex>::Out as Nat>::VAL == 2031);
impl NatIndex for Idx<2032> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2032> as NatIndex>::Out as Nat>::VAL == 2032);
impl NatIndex for Idx<2033> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2033> as NatIndex>::Out as Nat>::VAL == 2033);
impl NatIndex for Idx<2034> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2034> as NatIndex>::Out as Nat>::VAL == 2034);
impl NatIndex for Idx<2035> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2035> as NatIndex>::Out as Nat>::VAL == 2035);
impl NatIndex for Idx<2036> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2036> as NatIndex>::Out as Nat>::VAL == 2036);
impl NatIndex for Idx<2037> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2037> as NatIndex>::Out as Nat>::VAL == 2037);
impl NatIndex for Idx<2038> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2038> as NatIndex>::Out as Nat>::VAL == 2038);
impl NatIndex for Idx<2039> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2039> as NatIndex>::Out as Nat>::VAL == 2039);
impl NatIndex for Idx<2040> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2040> as NatIndex>::Out as Nat>::VAL == 2040);
impl NatIndex for Idx<2041> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2041> as NatIndex>::Out as Nat>::VAL == 2041);
impl NatIndex for Idx<2042> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2042> as NatIndex>::Out as Nat>::VAL == 2042);
impl NatIndex for Idx<2043> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2043> as NatIndex>::Out as Nat>::VAL == 2043);
impl NatIndex for Idx<2044> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2044> as NatIndex>::Out as Nat>::VAL == 2044);
impl NatIndex for Idx<2045> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2045> as NatIndex>::Out as Nat>::VAL == 2045);
impl NatIndex for Idx<2046> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2046> as NatIndex>::Out as Nat>::VAL == 2046);
impl NatIndex for Idx<2047> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2047> as NatIndex>::Out as Nat>::VAL == 2047);
impl NatIndex for Idx<2048> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2048> as NatIndex>::Out as Nat>::VAL == 2048);
impl NatIndex for Idx<2049> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2049> as NatIndex>::Out as Nat>::VAL == 2049);
impl NatIndex for Idx<2050> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2050> as NatIndex>::Out as Nat>::VAL == 2050);
impl NatIndex for Idx<2051> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2051> as NatIndex>::Out as Nat>::VAL == 2051);
impl NatIndex for Idx<2052> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2052> as NatIndex>::Out as Nat>::VAL == 2052);
impl NatIndex for Idx<2053> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2053> as NatIndex>::Out as Nat>::VAL == 2053);
impl NatIndex for Idx<2054> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2054> as NatIndex>::Out as Nat>::VAL == 2054);
impl NatIndex for Idx<2055> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2055> as NatIndex>::Out as Nat>::VAL == 2055);
impl NatIndex for Idx<2056> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2056> as NatIndex>::Out as Nat>::VAL == 2056);
impl NatIndex for Idx<2057> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2057> as NatIndex>::Out as Nat>::VAL == 2057);
impl NatIndex for Idx<2058> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2058> as NatIndex>::Out as Nat>::VAL == 2058);
impl NatIndex for Idx<2059> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2059> as NatIndex>::Out as Nat>::VAL == 2059);
impl NatIndex for Idx<2060> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2060> as NatIndex>::Out as Nat>::VAL == 2060);
impl NatIndex for Idx<2061> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2061> as NatIndex>::Out as Nat>::VAL == 2061);
impl NatIndex for Idx<2062> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2062> as NatIndex>::Out as Nat>::VAL == 2062);
impl NatIndex for Idx<2063> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2063> as NatIndex>::Out as Nat>::VAL == 2063);
impl NatIndex for Idx<2064> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2064> as NatIndex>::Out as Nat>::VAL == 2064);
impl NatIndex for Idx<2065> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2065> as NatIndex>::Out as Nat>::VAL == 2065);
impl NatIndex for Idx<2066> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2066> as NatIndex>::Out as Nat>::VAL == 2066);
impl NatIndex for Idx<2067> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2067> as NatIndex>::Out as Nat>::VAL == 2067);
impl NatIndex for Idx<2068> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2068> as NatIndex>::Out as Nat>::VAL == 2068);
impl NatIndex for Idx<2069> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2069> as NatIndex>::Out as Nat>::VAL == 2069);
impl NatIndex for Idx<2070> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2070> as NatIndex>::Out as Nat>::VAL == 2070);
impl NatIndex for Idx<2071> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2071> as NatIndex>::Out as Nat>::VAL == 2071);
impl NatIndex for Idx<2072> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2072> as NatIndex>::Out as Nat>::VAL == 2072);
impl NatIndex for Idx<2073> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2073> as NatIndex>::Out as Nat>::VAL == 2073);
impl NatIndex for Idx<2074> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2074> as NatIndex>::Out as Nat>::VAL == 2074);
impl NatIndex for Idx<2075> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2075> as NatIndex>::Out as Nat>::VAL == 2075);
impl NatIndex for Idx<2076> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2076> as NatIndex>::Out as Nat>::VAL == 2076);
impl NatIndex for Idx<2077> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2077> as NatIndex>::Out as Nat>::VAL == 2077);
impl NatIndex for Idx<2078> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2078> as NatIndex>::Out as Nat>::VAL == 2078);
impl NatIndex for Idx<2079> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2079> as NatIndex>::Out as Nat>::VAL == 2079);
impl NatIndex for Idx<2080> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2080> as NatIndex>::Out as Nat>::VAL == 2080);
impl NatIndex for Idx<2081> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2081> as NatIndex>::Out as Nat>::VAL == 2081);
impl NatIndex for Idx<2082> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2082> as NatIndex>::Out as Nat>::VAL == 2082);
impl NatIndex for Idx<2083> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2083> as NatIndex>::Out as Nat>::VAL == 2083);
impl NatIndex for Idx<2084> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2084> as NatIndex>::Out as Nat>::VAL == 2084);
impl NatIndex for Idx<2085> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2085> as NatIndex>::Out as Nat>::VAL == 2085);
impl NatIndex for Idx<2086> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2086> as NatIndex>::Out as Nat>::VAL == 2086);
impl NatIndex for Idx<2087> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2087> as NatIndex>::Out as Nat>::VAL == 2087);
impl NatIndex for Idx<2088> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2088> as NatIndex>::Out as Nat>::VAL == 2088);
impl NatIndex for Idx<2089> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2089> as NatIndex>::Out as Nat>::VAL == 2089);
impl NatIndex for Idx<2090> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2090> as NatIndex>::Out as Nat>::VAL == 2090);
impl NatIndex for Idx<2091> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2091> as NatIndex>::Out as Nat>::VAL == 2091);
impl NatIndex for Idx<2092> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2092> as NatIndex>::Out as Nat>::VAL == 2092);
impl NatIndex for Idx<2093> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2093> as NatIndex>::Out as Nat>::VAL == 2093);
impl NatIndex for Idx<2094> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2094> as NatIndex>::Out as Nat>::VAL == 2094);
impl NatIndex for Idx<2095> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2095> as NatIndex>::Out as Nat>::VAL == 2095);
impl NatIndex for Idx<2096> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2096> as NatIndex>::Out as Nat>::VAL == 2096);
impl NatIndex for Idx<2097> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2097> as NatIndex>::Out as Nat>::VAL == 2097);
impl NatIndex for Idx<2098> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2098> as NatIndex>::Out as Nat>::VAL == 2098);
impl NatIndex for Idx<2099> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2099> as NatIndex>::Out as Nat>::VAL == 2099);
impl NatIndex for Idx<2100> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2100> as NatIndex>::Out as Nat>::VAL == 2100);
impl NatIndex for Idx<2101> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2101> as NatIndex>::Out as Nat>::VAL == 2101);
impl NatIndex for Idx<2102> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2102> as NatIndex>::Out as Nat>::VAL == 2102);
impl NatIndex for Idx<2103> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2103> as NatIndex>::Out as Nat>::VAL == 2103);
impl NatIndex for Idx<2104> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2104> as NatIndex>::Out as Nat>::VAL == 2104);
impl NatIndex for Idx<2105> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2105> as NatIndex>::Out as Nat>::VAL == 2105);
impl NatIndex for Idx<2106> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2106> as NatIndex>::Out as Nat>::VAL == 2106);
impl NatIndex for Idx<2107> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2107> as NatIndex>::Out as Nat>::VAL == 2107);
impl NatIndex for Idx<2108> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2108> as NatIndex>::Out as Nat>::VAL == 2108);
impl NatIndex for Idx<2109> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2109> as NatIndex>::Out as Nat>::VAL == 2109);
impl NatIndex for Idx<2110> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2110> as NatIndex>::Out as Nat>::VAL == 2110);
impl NatIndex for Idx<2111> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2111> as NatIndex>::Out as Nat>::VAL == 2111);
impl NatIndex for Idx<2112> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2112> as NatIndex>::Out as Nat>::VAL == 2112);
impl NatIndex for Idx<2113> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2113> as NatIndex>::Out as Nat>::VAL == 2113);
impl NatIndex for Idx<2114> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2114> as NatIndex>::Out as Nat>::VAL == 2114);
impl NatIndex for Idx<2115> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2115> as NatIndex>::Out as Nat>::VAL == 2115);
impl NatIndex for Idx<2116> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2116> as NatIndex>::Out as Nat>::VAL == 2116);
impl NatIndex for Idx<2117> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2117> as NatIndex>::Out as Nat>::VAL == 2117);
impl NatIndex for Idx<2118> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2118> as NatIndex>::Out as Nat>::VAL == 2118);
impl NatIndex for Idx<2119> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2119> as NatIndex>::Out as Nat>::VAL == 2119);
impl NatIndex for Idx<2120> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2120> as NatIndex>::Out as Nat>::VAL == 2120);
impl NatIndex for Idx<2121> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2121> as NatIndex>::Out as Nat>::VAL == 2121);
impl NatIndex for Idx<2122> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2122> as NatIndex>::Out as Nat>::VAL == 2122);
impl NatIndex for Idx<2123> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2123> as NatIndex>::Out as Nat>::VAL == 2123);
impl NatIndex for Idx<2124> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2124> as NatIndex>::Out as Nat>::VAL == 2124);
impl NatIndex for Idx<2125> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2125> as NatIndex>::Out as Nat>::VAL == 2125);
impl NatIndex for Idx<2126> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2126> as NatIndex>::Out as Nat>::VAL == 2126);
impl NatIndex for Idx<2127> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2127> as NatIndex>::Out as Nat>::VAL == 2127);
impl NatIndex for Idx<2128> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2128> as NatIndex>::Out as Nat>::VAL == 2128);
impl NatIndex for Idx<2129> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2129> as NatIndex>::Out as Nat>::VAL == 2129);
impl NatIndex for Idx<2130> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2130> as NatIndex>::Out as Nat>::VAL == 2130);
impl NatIndex for Idx<2131> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2131> as NatIndex>::Out as Nat>::VAL == 2131);
impl NatIndex for Idx<2132> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2132> as NatIndex>::Out as Nat>::VAL == 2132);
impl NatIndex for Idx<2133> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2133> as NatIndex>::Out as Nat>::VAL == 2133);
impl NatIndex for Idx<2134> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2134> as NatIndex>::Out as Nat>::VAL == 2134);
impl NatIndex for Idx<2135> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2135> as NatIndex>::Out as Nat>::VAL == 2135);
impl NatIndex for Idx<2136> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2136> as NatIndex>::Out as Nat>::VAL == 2136);
impl NatIndex for Idx<2137> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2137> as NatIndex>::Out as Nat>::VAL == 2137);
impl NatIndex for Idx<2138> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2138> as NatIndex>::Out as Nat>::VAL == 2138);
impl NatIndex for Idx<2139> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2139> as NatIndex>::Out as Nat>::VAL == 2139);
impl NatIndex for Idx<2140> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2140> as NatIndex>::Out as Nat>::VAL == 2140);
impl NatIndex for Idx<2141> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2141> as NatIndex>::Out as Nat>::VAL == 2141);
impl NatIndex for Idx<2142> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2142> as NatIndex>::Out as Nat>::VAL == 2142);
impl NatIndex for Idx<2143> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2143> as NatIndex>::Out as Nat>::VAL == 2143);
impl NatIndex for Idx<2144> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2144> as NatIndex>::Out as Nat>::VAL == 2144);
impl NatIndex for Idx<2145> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2145> as NatIndex>::Out as Nat>::VAL == 2145);
impl NatIndex for Idx<2146> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2146> as NatIndex>::Out as Nat>::VAL == 2146);
impl NatIndex for Idx<2147> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2147> as NatIndex>::Out as Nat>::VAL == 2147);
impl NatIndex for Idx<2148> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2148> as NatIndex>::Out as Nat>::VAL == 2148);
impl NatIndex for Idx<2149> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2149> as NatIndex>::Out as Nat>::VAL == 2149);
impl NatIndex for Idx<2150> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2150> as NatIndex>::Out as Nat>::VAL == 2150);
impl NatIndex for Idx<2151> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2151> as NatIndex>::Out as Nat>::VAL == 2151);
impl NatIndex for Idx<2152> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2152> as NatIndex>::Out as Nat>::VAL == 2152);
impl NatIndex for Idx<2153> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2153> as NatIndex>::Out as Nat>::VAL == 2153);
impl NatIndex for Idx<2154> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2154> as NatIndex>::Out as Nat>::VAL == 2154);
impl NatIndex for Idx<2155> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2155> as NatIndex>::Out as Nat>::VAL == 2155);
impl NatIndex for Idx<2156> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2156> as NatIndex>::Out as Nat>::VAL == 2156);
impl NatIndex for Idx<2157> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2157> as NatIndex>::Out as Nat>::VAL == 2157);
impl NatIndex for Idx<2158> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2158> as NatIndex>::Out as Nat>::VAL == 2158);
impl NatIndex for Idx<2159> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2159> as NatIndex>::Out as Nat>::VAL == 2159);
impl NatIndex for Idx<2160> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2160> as NatIndex>::Out as Nat>::VAL == 2160);
impl NatIndex for Idx<2161> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2161> as NatIndex>::Out as Nat>::VAL == 2161);
impl NatIndex for Idx<2162> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2162> as NatIndex>::Out as Nat>::VAL == 2162);
impl NatIndex for Idx<2163> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2163> as NatIndex>::Out as Nat>::VAL == 2163);
impl NatIndex for Idx<2164> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2164> as NatIndex>::Out as Nat>::VAL == 2164);
impl NatIndex for Idx<2165> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2165> as NatIndex>::Out as Nat>::VAL == 2165);
impl NatIndex for Idx<2166> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2166> as NatIndex>::Out as Nat>::VAL == 2166);
impl NatIndex for Idx<2167> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2167> as NatIndex>::Out as Nat>::VAL == 2167);
impl NatIndex for Idx<2168> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2168> as NatIndex>::Out as Nat>::VAL == 2168);
impl NatIndex for Idx<2169> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2169> as NatIndex>::Out as Nat>::VAL == 2169);
impl NatIndex for Idx<2170> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2170> as NatIndex>::Out as Nat>::VAL == 2170);
impl NatIndex for Idx<2171> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2171> as NatIndex>::Out as Nat>::VAL == 2171);
impl NatIndex for Idx<2172> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2172> as NatIndex>::Out as Nat>::VAL == 2172);
impl NatIndex for Idx<2173> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2173> as NatIndex>::Out as Nat>::VAL == 2173);
impl NatIndex for Idx<2174> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2174> as NatIndex>::Out as Nat>::VAL == 2174);
impl NatIndex for Idx<2175> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2175> as NatIndex>::Out as Nat>::VAL == 2175);
impl NatIndex for Idx<2176> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2176> as NatIndex>::Out as Nat>::VAL == 2176);
impl NatIndex for Idx<2177> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2177> as NatIndex>::Out as Nat>::VAL == 2177);
impl NatIndex for Idx<2178> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2178> as NatIndex>::Out as Nat>::VAL == 2178);
impl NatIndex for Idx<2179> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2179> as NatIndex>::Out as Nat>::VAL == 2179);
impl NatIndex for Idx<2180> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2180> as NatIndex>::Out as Nat>::VAL == 2180);
impl NatIndex for Idx<2181> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2181> as NatIndex>::Out as Nat>::VAL == 2181);
impl NatIndex for Idx<2182> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2182> as NatIndex>::Out as Nat>::VAL == 2182);
impl NatIndex for Idx<2183> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2183> as NatIndex>::Out as Nat>::VAL == 2183);
impl NatIndex for Idx<2184> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2184> as NatIndex>::Out as Nat>::VAL == 2184);
impl NatIndex for Idx<2185> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2185> as NatIndex>::Out as Nat>::VAL == 2185);
impl NatIndex for Idx<2186> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2186> as NatIndex>::Out as Nat>::VAL == 2186);
impl NatIndex for Idx<2187> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2187> as NatIndex>::Out as Nat>::VAL == 2187);
impl NatIndex for Idx<2188> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2188> as NatIndex>::Out as Nat>::VAL == 2188);
impl NatIndex for Idx<2189> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2189> as NatIndex>::Out as Nat>::VAL == 2189);
impl NatIndex for Idx<2190> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2190> as NatIndex>::Out as Nat>::VAL == 2190);
impl NatIndex for Idx<2191> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2191> as NatIndex>::Out as Nat>::VAL == 2191);
impl NatIndex for Idx<2192> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2192> as NatIndex>::Out as Nat>::VAL == 2192);
impl NatIndex for Idx<2193> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2193> as NatIndex>::Out as Nat>::VAL == 2193);
impl NatIndex for Idx<2194> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2194> as NatIndex>::Out as Nat>::VAL == 2194);
impl NatIndex for Idx<2195> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2195> as NatIndex>::Out as Nat>::VAL == 2195);
impl NatIndex for Idx<2196> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2196> as NatIndex>::Out as Nat>::VAL == 2196);
impl NatIndex for Idx<2197> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2197> as NatIndex>::Out as Nat>::VAL == 2197);
impl NatIndex for Idx<2198> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2198> as NatIndex>::Out as Nat>::VAL == 2198);
impl NatIndex for Idx<2199> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2199> as NatIndex>::Out as Nat>::VAL == 2199);
impl NatIndex for Idx<2200> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2200> as NatIndex>::Out as Nat>::VAL == 2200);
impl NatIndex for Idx<2201> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2201> as NatIndex>::Out as Nat>::VAL == 2201);
impl NatIndex for Idx<2202> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2202> as NatIndex>::Out as Nat>::VAL == 2202);
impl NatIndex for Idx<2203> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2203> as NatIndex>::Out as Nat>::VAL == 2203);
impl NatIndex for Idx<2204> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2204> as NatIndex>::Out as Nat>::VAL == 2204);
impl NatIndex for Idx<2205> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2205> as NatIndex>::Out as Nat>::VAL == 2205);
impl NatIndex for Idx<2206> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2206> as NatIndex>::Out as Nat>::VAL == 2206);
impl NatIndex for Idx<2207> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2207> as NatIndex>::Out as Nat>::VAL == 2207);
impl NatIndex for Idx<2208> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2208> as NatIndex>::Out as Nat>::VAL == 2208);
impl NatIndex for Idx<2209> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2209> as NatIndex>::Out as Nat>::VAL == 2209);
impl NatIndex for Idx<2210> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2210> as NatIndex>::Out as Nat>::VAL == 2210);
impl NatIndex for Idx<2211> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2211> as NatIndex>::Out as Nat>::VAL == 2211);
impl NatIndex for Idx<2212> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2212> as NatIndex>::Out as Nat>::VAL == 2212);
impl NatIndex for Idx<2213> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2213> as NatIndex>::Out as Nat>::VAL == 2213);
impl NatIndex for Idx<2214> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2214> as NatIndex>::Out as Nat>::VAL == 2214);
impl NatIndex for Idx<2215> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2215> as NatIndex>::Out as Nat>::VAL == 2215);
impl NatIndex for Idx<2216> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2216> as NatIndex>::Out as Nat>::VAL == 2216);
impl NatIndex for Idx<2217> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2217> as NatIndex>::Out as Nat>::VAL == 2217);
impl NatIndex for Idx<2218> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2218> as NatIndex>::Out as Nat>::VAL == 2218);
impl NatIndex for Idx<2219> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2219> as NatIndex>::Out as Nat>::VAL == 2219);
impl NatIndex for Idx<2220> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2220> as NatIndex>::Out as Nat>::VAL == 2220);
impl NatIndex for Idx<2221> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2221> as NatIndex>::Out as Nat>::VAL == 2221);
impl NatIndex for Idx<2222> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2222> as NatIndex>::Out as Nat>::VAL == 2222);
impl NatIndex for Idx<2223> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2223> as NatIndex>::Out as Nat>::VAL == 2223);
impl NatIndex for Idx<2224> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2224> as NatIndex>::Out as Nat>::VAL == 2224);
impl NatIndex for Idx<2225> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2225> as NatIndex>::Out as Nat>::VAL == 2225);
impl NatIndex for Idx<2226> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2226> as NatIndex>::Out as Nat>::VAL == 2226);
impl NatIndex for Idx<2227> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2227> as NatIndex>::Out as Nat>::VAL == 2227);
impl NatIndex for Idx<2228> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2228> as NatIndex>::Out as Nat>::VAL == 2228);
impl NatIndex for Idx<2229> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2229> as NatIndex>::Out as Nat>::VAL == 2229);
impl NatIndex for Idx<2230> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2230> as NatIndex>::Out as Nat>::VAL == 2230);
impl NatIndex for Idx<2231> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2231> as NatIndex>::Out as Nat>::VAL == 2231);
impl NatIndex for Idx<2232> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2232> as NatIndex>::Out as Nat>::VAL == 2232);
impl NatIndex for Idx<2233> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2233> as NatIndex>::Out as Nat>::VAL == 2233);
impl NatIndex for Idx<2234> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2234> as NatIndex>::Out as Nat>::VAL == 2234);
impl NatIndex for Idx<2235> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2235> as NatIndex>::Out as Nat>::VAL == 2235);
impl NatIndex for Idx<2236> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2236> as NatIndex>::Out as Nat>::VAL == 2236);
impl NatIndex for Idx<2237> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2237> as NatIndex>::Out as Nat>::VAL == 2237);
impl NatIndex for Idx<2238> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2238> as NatIndex>::Out as Nat>::VAL == 2238);
impl NatIndex for Idx<2239> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2239> as NatIndex>::Out as Nat>::VAL == 2239);
impl NatIndex for Idx<2240> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2240> as NatIndex>::Out as Nat>::VAL == 2240);
impl NatIndex for Idx<2241> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2241> as NatIndex>::Out as Nat>::VAL == 2241);
impl NatIndex for Idx<2242> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2242> as NatIndex>::Out as Nat>::VAL == 2242);
impl NatIndex for Idx<2243> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2243> as NatIndex>::Out as Nat>::VAL == 2243);
impl NatIndex for Idx<2244> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2244> as NatIndex>::Out as Nat>::VAL == 2244);
impl NatIndex for Idx<2245> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2245> as NatIndex>::Out as Nat>::VAL == 2245);
impl NatIndex for Idx<2246> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2246> as NatIndex>::Out as Nat>::VAL == 2246);
impl NatIndex for Idx<2247> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2247> as NatIndex>::Out as Nat>::VAL == 2247);
impl NatIndex for Idx<2248> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2248> as NatIndex>::Out as Nat>::VAL == 2248);
impl NatIndex for Idx<2249> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2249> as NatIndex>::Out as Nat>::VAL == 2249);
impl NatIndex for Idx<2250> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2250> as NatIndex>::Out as Nat>::VAL == 2250);
impl NatIndex for Idx<2251> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2251> as NatIndex>::Out as Nat>::VAL == 2251);
impl NatIndex for Idx<2252> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2252> as NatIndex>::Out as Nat>::VAL == 2252);
impl NatIndex for Idx<2253> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2253> as NatIndex>::Out as Nat>::VAL == 2253);
impl NatIndex for Idx<2254> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2254> as NatIndex>::Out as Nat>::VAL == 2254);
impl NatIndex for Idx<2255> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2255> as NatIndex>::Out as Nat>::VAL == 2255);
impl NatIndex for Idx<2256> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2256> as NatIndex>::Out as Nat>::VAL == 2256);
impl NatIndex for Idx<2257> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2257> as NatIndex>::Out as Nat>::VAL == 2257);
impl NatIndex for Idx<2258> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2258> as NatIndex>::Out as Nat>::VAL == 2258);
impl NatIndex for Idx<2259> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2259> as NatIndex>::Out as Nat>::VAL == 2259);
impl NatIndex for Idx<2260> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2260> as NatIndex>::Out as Nat>::VAL == 2260);
impl NatIndex for Idx<2261> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2261> as NatIndex>::Out as Nat>::VAL == 2261);
impl NatIndex for Idx<2262> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2262> as NatIndex>::Out as Nat>::VAL == 2262);
impl NatIndex for Idx<2263> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2263> as NatIndex>::Out as Nat>::VAL == 2263);
impl NatIndex for Idx<2264> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2264> as NatIndex>::Out as Nat>::VAL == 2264);
impl NatIndex for Idx<2265> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2265> as NatIndex>::Out as Nat>::VAL == 2265);
impl NatIndex for Idx<2266> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2266> as NatIndex>::Out as Nat>::VAL == 2266);
impl NatIndex for Idx<2267> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2267> as NatIndex>::Out as Nat>::VAL == 2267);
impl NatIndex for Idx<2268> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2268> as NatIndex>::Out as Nat>::VAL == 2268);
impl NatIndex for Idx<2269> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2269> as NatIndex>::Out as Nat>::VAL == 2269);
impl NatIndex for Idx<2270> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2270> as NatIndex>::Out as Nat>::VAL == 2270);
impl NatIndex for Idx<2271> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2271> as NatIndex>::Out as Nat>::VAL == 2271);
impl NatIndex for Idx<2272> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2272> as NatIndex>::Out as Nat>::VAL == 2272);
impl NatIndex for Idx<2273> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2273> as NatIndex>::Out as Nat>::VAL == 2273);
impl NatIndex for Idx<2274> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2274> as NatIndex>::Out as Nat>::VAL == 2274);
impl NatIndex for Idx<2275> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2275> as NatIndex>::Out as Nat>::VAL == 2275);
impl NatIndex for Idx<2276> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2276> as NatIndex>::Out as Nat>::VAL == 2276);
impl NatIndex for Idx<2277> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2277> as NatIndex>::Out as Nat>::VAL == 2277);
impl NatIndex for Idx<2278> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2278> as NatIndex>::Out as Nat>::VAL == 2278);
impl NatIndex for Idx<2279> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2279> as NatIndex>::Out as Nat>::VAL == 2279);
impl NatIndex for Idx<2280> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2280> as NatIndex>::Out as Nat>::VAL == 2280);
impl NatIndex for Idx<2281> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2281> as NatIndex>::Out as Nat>::VAL == 2281);
impl NatIndex for Idx<2282> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2282> as NatIndex>::Out as Nat>::VAL == 2282);
impl NatIndex for Idx<2283> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2283> as NatIndex>::Out as Nat>::VAL == 2283);
impl NatIndex for Idx<2284> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2284> as NatIndex>::Out as Nat>::VAL == 2284);
impl NatIndex for Idx<2285> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2285> as NatIndex>::Out as Nat>::VAL == 2285);
impl NatIndex for Idx<2286> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2286> as NatIndex>::Out as Nat>::VAL == 2286);
impl NatIndex for Idx<2287> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2287> as NatIndex>::Out as Nat>::VAL == 2287);
impl NatIndex for Idx<2288> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2288> as NatIndex>::Out as Nat>::VAL == 2288);
impl NatIndex for Idx<2289> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2289> as NatIndex>::Out as Nat>::VAL == 2289);
impl NatIndex for Idx<2290> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2290> as NatIndex>::Out as Nat>::VAL == 2290);
impl NatIndex for Idx<2291> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2291> as NatIndex>::Out as Nat>::VAL == 2291);
impl NatIndex for Idx<2292> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2292> as NatIndex>::Out as Nat>::VAL == 2292);
impl NatIndex for Idx<2293> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2293> as NatIndex>::Out as Nat>::VAL == 2293);
impl NatIndex for Idx<2294> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2294> as NatIndex>::Out as Nat>::VAL == 2294);
impl NatIndex for Idx<2295> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2295> as NatIndex>::Out as Nat>::VAL == 2295);
impl NatIndex for Idx<2296> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2296> as NatIndex>::Out as Nat>::VAL == 2296);
impl NatIndex for Idx<2297> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2297> as NatIndex>::Out as Nat>::VAL == 2297);
impl NatIndex for Idx<2298> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2298> as NatIndex>::Out as Nat>::VAL == 2298);
impl NatIndex for Idx<2299> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2299> as NatIndex>::Out as Nat>::VAL == 2299);
impl NatIndex for Idx<2300> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2300> as NatIndex>::Out as Nat>::VAL == 2300);
impl NatIndex for Idx<2301> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2301> as NatIndex>::Out as Nat>::VAL == 2301);
impl NatIndex for Idx<2302> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2302> as NatIndex>::Out as Nat>::VAL == 2302);
impl NatIndex for Idx<2303> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2303> as NatIndex>::Out as Nat>::VAL == 2303);
impl NatIndex for Idx<2304> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2304> as NatIndex>::Out as Nat>::VAL == 2304);
impl NatIndex for Idx<2305> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2305> as NatIndex>::Out as Nat>::VAL == 2305);
impl NatIndex for Idx<2306> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2306> as NatIndex>::Out as Nat>::VAL == 2306);
impl NatIndex for Idx<2307> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2307> as NatIndex>::Out as Nat>::VAL == 2307);
impl NatIndex for Idx<2308> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2308> as NatIndex>::Out as Nat>::VAL == 2308);
impl NatIndex for Idx<2309> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2309> as NatIndex>::Out as Nat>::VAL == 2309);
impl NatIndex for Idx<2310> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2310> as NatIndex>::Out as Nat>::VAL == 2310);
impl NatIndex for Idx<2311> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2311> as NatIndex>::Out as Nat>::VAL == 2311);
impl NatIndex for Idx<2312> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2312> as NatIndex>::Out as Nat>::VAL == 2312);
impl NatIndex for Idx<2313> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2313> as NatIndex>::Out as Nat>::VAL == 2313);
impl NatIndex for Idx<2314> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2314> as NatIndex>::Out as Nat>::VAL == 2314);
impl NatIndex for Idx<2315> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2315> as NatIndex>::Out as Nat>::VAL == 2315);
impl NatIndex for Idx<2316> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2316> as NatIndex>::Out as Nat>::VAL == 2316);
impl NatIndex for Idx<2317> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2317> as NatIndex>::Out as Nat>::VAL == 2317);
impl NatIndex for Idx<2318> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2318> as NatIndex>::Out as Nat>::VAL == 2318);
impl NatIndex for Idx<2319> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2319> as NatIndex>::Out as Nat>::VAL == 2319);
impl NatIndex for Idx<2320> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2320> as NatIndex>::Out as Nat>::VAL == 2320);
impl NatIndex for Idx<2321> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2321> as NatIndex>::Out as Nat>::VAL == 2321);
impl NatIndex for Idx<2322> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2322> as NatIndex>::Out as Nat>::VAL == 2322);
impl NatIndex for Idx<2323> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2323> as NatIndex>::Out as Nat>::VAL == 2323);
impl NatIndex for Idx<2324> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2324> as NatIndex>::Out as Nat>::VAL == 2324);
impl NatIndex for Idx<2325> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2325> as NatIndex>::Out as Nat>::VAL == 2325);
impl NatIndex for Idx<2326> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2326> as NatIndex>::Out as Nat>::VAL == 2326);
impl NatIndex for Idx<2327> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2327> as NatIndex>::Out as Nat>::VAL == 2327);
impl NatIndex for Idx<2328> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2328> as NatIndex>::Out as Nat>::VAL == 2328);
impl NatIndex for Idx<2329> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2329> as NatIndex>::Out as Nat>::VAL == 2329);
impl NatIndex for Idx<2330> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2330> as NatIndex>::Out as Nat>::VAL == 2330);
impl NatIndex for Idx<2331> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2331> as NatIndex>::Out as Nat>::VAL == 2331);
impl NatIndex for Idx<2332> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2332> as NatIndex>::Out as Nat>::VAL == 2332);
impl NatIndex for Idx<2333> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2333> as NatIndex>::Out as Nat>::VAL == 2333);
impl NatIndex for Idx<2334> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2334> as NatIndex>::Out as Nat>::VAL == 2334);
impl NatIndex for Idx<2335> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2335> as NatIndex>::Out as Nat>::VAL == 2335);
impl NatIndex for Idx<2336> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2336> as NatIndex>::Out as Nat>::VAL == 2336);
impl NatIndex for Idx<2337> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2337> as NatIndex>::Out as Nat>::VAL == 2337);
impl NatIndex for Idx<2338> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2338> as NatIndex>::Out as Nat>::VAL == 2338);
impl NatIndex for Idx<2339> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2339> as NatIndex>::Out as Nat>::VAL == 2339);
impl NatIndex for Idx<2340> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2340> as NatIndex>::Out as Nat>::VAL == 2340);
impl NatIndex for Idx<2341> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2341> as NatIndex>::Out as Nat>::VAL == 2341);
impl NatIndex for Idx<2342> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2342> as NatIndex>::Out as Nat>::VAL == 2342);
impl NatIndex for Idx<2343> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2343> as NatIndex>::Out as Nat>::VAL == 2343);
impl NatIndex for Idx<2344> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2344> as NatIndex>::Out as Nat>::VAL == 2344);
impl NatIndex for Idx<2345> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2345> as NatIndex>::Out as Nat>::VAL == 2345);
impl NatIndex for Idx<2346> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2346> as NatIndex>::Out as Nat>::VAL == 2346);
impl NatIndex for Idx<2347> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2347> as NatIndex>::Out as Nat>::VAL == 2347);
impl NatIndex for Idx<2348> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2348> as NatIndex>::Out as Nat>::VAL == 2348);
impl NatIndex for Idx<2349> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2349> as NatIndex>::Out as Nat>::VAL == 2349);
impl NatIndex for Idx<2350> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2350> as NatIndex>::Out as Nat>::VAL == 2350);
impl NatIndex for Idx<2351> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2351> as NatIndex>::Out as Nat>::VAL == 2351);
impl NatIndex for Idx<2352> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2352> as NatIndex>::Out as Nat>::VAL == 2352);
impl NatIndex for Idx<2353> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2353> as NatIndex>::Out as Nat>::VAL == 2353);
impl NatIndex for Idx<2354> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2354> as NatIndex>::Out as Nat>::VAL == 2354);
impl NatIndex for Idx<2355> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2355> as NatIndex>::Out as Nat>::VAL == 2355);
impl NatIndex for Idx<2356> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2356> as NatIndex>::Out as Nat>::VAL == 2356);
impl NatIndex for Idx<2357> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2357> as NatIndex>::Out as Nat>::VAL == 2357);
impl NatIndex for Idx<2358> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2358> as NatIndex>::Out as Nat>::VAL == 2358);
impl NatIndex for Idx<2359> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2359> as NatIndex>::Out as Nat>::VAL == 2359);
impl NatIndex for Idx<2360> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2360> as NatIndex>::Out as Nat>::VAL == 2360);
impl NatIndex for Idx<2361> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2361> as NatIndex>::Out as Nat>::VAL == 2361);
impl NatIndex for Idx<2362> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2362> as NatIndex>::Out as Nat>::VAL == 2362);
impl NatIndex for Idx<2363> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2363> as NatIndex>::Out as Nat>::VAL == 2363);
impl NatIndex for Idx<2364> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2364> as NatIndex>::Out as Nat>::VAL == 2364);
impl NatIndex for Idx<2365> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2365> as NatIndex>::Out as Nat>::VAL == 2365);
impl NatIndex for Idx<2366> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2366> as NatIndex>::Out as Nat>::VAL == 2366);
impl NatIndex for Idx<2367> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2367> as NatIndex>::Out as Nat>::VAL == 2367);
impl NatIndex for Idx<2368> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2368> as NatIndex>::Out as Nat>::VAL == 2368);
impl NatIndex for Idx<2369> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2369> as NatIndex>::Out as Nat>::VAL == 2369);
impl NatIndex for Idx<2370> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2370> as NatIndex>::Out as Nat>::VAL == 2370);
impl NatIndex for Idx<2371> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2371> as NatIndex>::Out as Nat>::VAL == 2371);
impl NatIndex for Idx<2372> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2372> as NatIndex>::Out as Nat>::VAL == 2372);
impl NatIndex for Idx<2373> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2373> as NatIndex>::Out as Nat>::VAL == 2373);
impl NatIndex for Idx<2374> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2374> as NatIndex>::Out as Nat>::VAL == 2374);
impl NatIndex for Idx<2375> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2375> as NatIndex>::Out as Nat>::VAL == 2375);
impl NatIndex for Idx<2376> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2376> as NatIndex>::Out as Nat>::VAL == 2376);
impl NatIndex for Idx<2377> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2377> as NatIndex>::Out as Nat>::VAL == 2377);
impl NatIndex for Idx<2378> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2378> as NatIndex>::Out as Nat>::VAL == 2378);
impl NatIndex for Idx<2379> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2379> as NatIndex>::Out as Nat>::VAL == 2379);
impl NatIndex for Idx<2380> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2380> as NatIndex>::Out as Nat>::VAL == 2380);
impl NatIndex for Idx<2381> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2381> as NatIndex>::Out as Nat>::VAL == 2381);
impl NatIndex for Idx<2382> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2382> as NatIndex>::Out as Nat>::VAL == 2382);
impl NatIndex for Idx<2383> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2383> as NatIndex>::Out as Nat>::VAL == 2383);
impl NatIndex for Idx<2384> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2384> as NatIndex>::Out as Nat>::VAL == 2384);
impl NatIndex for Idx<2385> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2385> as NatIndex>::Out as Nat>::VAL == 2385);
impl NatIndex for Idx<2386> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2386> as NatIndex>::Out as Nat>::VAL == 2386);
impl NatIndex for Idx<2387> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2387> as NatIndex>::Out as Nat>::VAL == 2387);
impl NatIndex for Idx<2388> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2388> as NatIndex>::Out as Nat>::VAL == 2388);
impl NatIndex for Idx<2389> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2389> as NatIndex>::Out as Nat>::VAL == 2389);
impl NatIndex for Idx<2390> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2390> as NatIndex>::Out as Nat>::VAL == 2390);
impl NatIndex for Idx<2391> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2391> as NatIndex>::Out as Nat>::VAL == 2391);
impl NatIndex for Idx<2392> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2392> as NatIndex>::Out as Nat>::VAL == 2392);
impl NatIndex for Idx<2393> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2393> as NatIndex>::Out as Nat>::VAL == 2393);
impl NatIndex for Idx<2394> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2394> as NatIndex>::Out as Nat>::VAL == 2394);
impl NatIndex for Idx<2395> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2395> as NatIndex>::Out as Nat>::VAL == 2395);
impl NatIndex for Idx<2396> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2396> as NatIndex>::Out as Nat>::VAL == 2396);
impl NatIndex for Idx<2397> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2397> as NatIndex>::Out as Nat>::VAL == 2397);
impl NatIndex for Idx<2398> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2398> as NatIndex>::Out as Nat>::VAL == 2398);
impl NatIndex for Idx<2399> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2399> as NatIndex>::Out as Nat>::VAL == 2399);
impl NatIndex for Idx<2400> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2400> as NatIndex>::Out as Nat>::VAL == 2400);
impl NatIndex for Idx<2401> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2401> as NatIndex>::Out as Nat>::VAL == 2401);
impl NatIndex for Idx<2402> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2402> as NatIndex>::Out as Nat>::VAL == 2402);
impl NatIndex for Idx<2403> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2403> as NatIndex>::Out as Nat>::VAL == 2403);
impl NatIndex for Idx<2404> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2404> as NatIndex>::Out as Nat>::VAL == 2404);
impl NatIndex for Idx<2405> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2405> as NatIndex>::Out as Nat>::VAL == 2405);
impl NatIndex for Idx<2406> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2406> as NatIndex>::Out as Nat>::VAL == 2406);
impl NatIndex for Idx<2407> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2407> as NatIndex>::Out as Nat>::VAL == 2407);
impl NatIndex for Idx<2408> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2408> as NatIndex>::Out as Nat>::VAL == 2408);
impl NatIndex for Idx<2409> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2409> as NatIndex>::Out as Nat>::VAL == 2409);
impl NatIndex for Idx<2410> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2410> as NatIndex>::Out as Nat>::VAL == 2410);
impl NatIndex for Idx<2411> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2411> as NatIndex>::Out as Nat>::VAL == 2411);
impl NatIndex for Idx<2412> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2412> as NatIndex>::Out as Nat>::VAL == 2412);
impl NatIndex for Idx<2413> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2413> as NatIndex>::Out as Nat>::VAL == 2413);
impl NatIndex for Idx<2414> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2414> as NatIndex>::Out as Nat>::VAL == 2414);
impl NatIndex for Idx<2415> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2415> as NatIndex>::Out as Nat>::VAL == 2415);
impl NatIndex for Idx<2416> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2416> as NatIndex>::Out as Nat>::VAL == 2416);
impl NatIndex for Idx<2417> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2417> as NatIndex>::Out as Nat>::VAL == 2417);
impl NatIndex for Idx<2418> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2418> as NatIndex>::Out as Nat>::VAL == 2418);
impl NatIndex for Idx<2419> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2419> as NatIndex>::Out as Nat>::VAL == 2419);
impl NatIndex for Idx<2420> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2420> as NatIndex>::Out as Nat>::VAL == 2420);
impl NatIndex for Idx<2421> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2421> as NatIndex>::Out as Nat>::VAL == 2421);
impl NatIndex for Idx<2422> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2422> as NatIndex>::Out as Nat>::VAL == 2422);
impl NatIndex for Idx<2423> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2423> as NatIndex>::Out as Nat>::VAL == 2423);
impl NatIndex for Idx<2424> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2424> as NatIndex>::Out as Nat>::VAL == 2424);
impl NatIndex for Idx<2425> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2425> as NatIndex>::Out as Nat>::VAL == 2425);
impl NatIndex for Idx<2426> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2426> as NatIndex>::Out as Nat>::VAL == 2426);
impl NatIndex for Idx<2427> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2427> as NatIndex>::Out as Nat>::VAL == 2427);
impl NatIndex for Idx<2428> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2428> as NatIndex>::Out as Nat>::VAL == 2428);
impl NatIndex for Idx<2429> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2429> as NatIndex>::Out as Nat>::VAL == 2429);
impl NatIndex for Idx<2430> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2430> as NatIndex>::Out as Nat>::VAL == 2430);
impl NatIndex for Idx<2431> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2431> as NatIndex>::Out as Nat>::VAL == 2431);
impl NatIndex for Idx<2432> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2432> as NatIndex>::Out as Nat>::VAL == 2432);
impl NatIndex for Idx<2433> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2433> as NatIndex>::Out as Nat>::VAL == 2433);
impl NatIndex for Idx<2434> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2434> as NatIndex>::Out as Nat>::VAL == 2434);
impl NatIndex for Idx<2435> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2435> as NatIndex>::Out as Nat>::VAL == 2435);
impl NatIndex for Idx<2436> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2436> as NatIndex>::Out as Nat>::VAL == 2436);
impl NatIndex for Idx<2437> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2437> as NatIndex>::Out as Nat>::VAL == 2437);
impl NatIndex for Idx<2438> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2438> as NatIndex>::Out as Nat>::VAL == 2438);
impl NatIndex for Idx<2439> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2439> as NatIndex>::Out as Nat>::VAL == 2439);
impl NatIndex for Idx<2440> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2440> as NatIndex>::Out as Nat>::VAL == 2440);
impl NatIndex for Idx<2441> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2441> as NatIndex>::Out as Nat>::VAL == 2441);
impl NatIndex for Idx<2442> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2442> as NatIndex>::Out as Nat>::VAL == 2442);
impl NatIndex for Idx<2443> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2443> as NatIndex>::Out as Nat>::VAL == 2443);
impl NatIndex for Idx<2444> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2444> as NatIndex>::Out as Nat>::VAL == 2444);
impl NatIndex for Idx<2445> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2445> as NatIndex>::Out as Nat>::VAL == 2445);
impl NatIndex for Idx<2446> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2446> as NatIndex>::Out as Nat>::VAL == 2446);
impl NatIndex for Idx<2447> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2447> as NatIndex>::Out as Nat>::VAL == 2447);
impl NatIndex for Idx<2448> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2448> as NatIndex>::Out as Nat>::VAL == 2448);
impl NatIndex for Idx<2449> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2449> as NatIndex>::Out as Nat>::VAL == 2449);
impl NatIndex for Idx<2450> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2450> as NatIndex>::Out as Nat>::VAL == 2450);
impl NatIndex for Idx<2451> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2451> as NatIndex>::Out as Nat>::VAL == 2451);
impl NatIndex for Idx<2452> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2452> as NatIndex>::Out as Nat>::VAL == 2452);
impl NatIndex for Idx<2453> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2453> as NatIndex>::Out as Nat>::VAL == 2453);
impl NatIndex for Idx<2454> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2454> as NatIndex>::Out as Nat>::VAL == 2454);
impl NatIndex for Idx<2455> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2455> as NatIndex>::Out as Nat>::VAL == 2455);
impl NatIndex for Idx<2456> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2456> as NatIndex>::Out as Nat>::VAL == 2456);
impl NatIndex for Idx<2457> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2457> as NatIndex>::Out as Nat>::VAL == 2457);
impl NatIndex for Idx<2458> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2458> as NatIndex>::Out as Nat>::VAL == 2458);
impl NatIndex for Idx<2459> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2459> as NatIndex>::Out as Nat>::VAL == 2459);
impl NatIndex for Idx<2460> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2460> as NatIndex>::Out as Nat>::VAL == 2460);
impl NatIndex for Idx<2461> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2461> as NatIndex>::Out as Nat>::VAL == 2461);
impl NatIndex for Idx<2462> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2462> as NatIndex>::Out as Nat>::VAL == 2462);
impl NatIndex for Idx<2463> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2463> as NatIndex>::Out as Nat>::VAL == 2463);
impl NatIndex for Idx<2464> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2464> as NatIndex>::Out as Nat>::VAL == 2464);
impl NatIndex for Idx<2465> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2465> as NatIndex>::Out as Nat>::VAL == 2465);
impl NatIndex for Idx<2466> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2466> as NatIndex>::Out as Nat>::VAL == 2466);
impl NatIndex for Idx<2467> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2467> as NatIndex>::Out as Nat>::VAL == 2467);
impl NatIndex for Idx<2468> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2468> as NatIndex>::Out as Nat>::VAL == 2468);
impl NatIndex for Idx<2469> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2469> as NatIndex>::Out as Nat>::VAL == 2469);
impl NatIndex for Idx<2470> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2470> as NatIndex>::Out as Nat>::VAL == 2470);
impl NatIndex for Idx<2471> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2471> as NatIndex>::Out as Nat>::VAL == 2471);
impl NatIndex for Idx<2472> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2472> as NatIndex>::Out as Nat>::VAL == 2472);
impl NatIndex for Idx<2473> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2473> as NatIndex>::Out as Nat>::VAL == 2473);
impl NatIndex for Idx<2474> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2474> as NatIndex>::Out as Nat>::VAL == 2474);
impl NatIndex for Idx<2475> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2475> as NatIndex>::Out as Nat>::VAL == 2475);
impl NatIndex for Idx<2476> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2476> as NatIndex>::Out as Nat>::VAL == 2476);
impl NatIndex for Idx<2477> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2477> as NatIndex>::Out as Nat>::VAL == 2477);
impl NatIndex for Idx<2478> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2478> as NatIndex>::Out as Nat>::VAL == 2478);
impl NatIndex for Idx<2479> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2479> as NatIndex>::Out as Nat>::VAL == 2479);
impl NatIndex for Idx<2480> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2480> as NatIndex>::Out as Nat>::VAL == 2480);
impl NatIndex for Idx<2481> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2481> as NatIndex>::Out as Nat>::VAL == 2481);
impl NatIndex for Idx<2482> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2482> as NatIndex>::Out as Nat>::VAL == 2482);
impl NatIndex for Idx<2483> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2483> as NatIndex>::Out as Nat>::VAL == 2483);
impl NatIndex for Idx<2484> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2484> as NatIndex>::Out as Nat>::VAL == 2484);
impl NatIndex for Idx<2485> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2485> as NatIndex>::Out as Nat>::VAL == 2485);
impl NatIndex for Idx<2486> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2486> as NatIndex>::Out as Nat>::VAL == 2486);
impl NatIndex for Idx<2487> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2487> as NatIndex>::Out as Nat>::VAL == 2487);
impl NatIndex for Idx<2488> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2488> as NatIndex>::Out as Nat>::VAL == 2488);
impl NatIndex for Idx<2489> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2489> as NatIndex>::Out as Nat>::VAL == 2489);
impl NatIndex for Idx<2490> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2490> as NatIndex>::Out as Nat>::VAL == 2490);
impl NatIndex for Idx<2491> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2491> as NatIndex>::Out as Nat>::VAL == 2491);
impl NatIndex for Idx<2492> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2492> as NatIndex>::Out as Nat>::VAL == 2492);
impl NatIndex for Idx<2493> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2493> as NatIndex>::Out as Nat>::VAL == 2493);
impl NatIndex for Idx<2494> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2494> as NatIndex>::Out as Nat>::VAL == 2494);
impl NatIndex for Idx<2495> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2495> as NatIndex>::Out as Nat>::VAL == 2495);
impl NatIndex for Idx<2496> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2496> as NatIndex>::Out as Nat>::VAL == 2496);
impl NatIndex for Idx<2497> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2497> as NatIndex>::Out as Nat>::VAL == 2497);
impl NatIndex for Idx<2498> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2498> as NatIndex>::Out as Nat>::VAL == 2498);
impl NatIndex for Idx<2499> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2499> as NatIndex>::Out as Nat>::VAL == 2499);
impl NatIndex for Idx<2500> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2500> as NatIndex>::Out as Nat>::VAL == 2500);
impl NatIndex for Idx<2501> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2501> as NatIndex>::Out as Nat>::VAL == 2501);
impl NatIndex for Idx<2502> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2502> as NatIndex>::Out as Nat>::VAL == 2502);
impl NatIndex for Idx<2503> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2503> as NatIndex>::Out as Nat>::VAL == 2503);
impl NatIndex for Idx<2504> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2504> as NatIndex>::Out as Nat>::VAL == 2504);
impl NatIndex for Idx<2505> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2505> as NatIndex>::Out as Nat>::VAL == 2505);
impl NatIndex for Idx<2506> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2506> as NatIndex>::Out as Nat>::VAL == 2506);
impl NatIndex for Idx<2507> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2507> as NatIndex>::Out as Nat>::VAL == 2507);
impl NatIndex for Idx<2508> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2508> as NatIndex>::Out as Nat>::VAL == 2508);
impl NatIndex for Idx<2509> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2509> as NatIndex>::Out as Nat>::VAL == 2509);
impl NatIndex for Idx<2510> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2510> as NatIndex>::Out as Nat>::VAL == 2510);
impl NatIndex for Idx<2511> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2511> as NatIndex>::Out as Nat>::VAL == 2511);
impl NatIndex for Idx<2512> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2512> as NatIndex>::Out as Nat>::VAL == 2512);
impl NatIndex for Idx<2513> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2513> as NatIndex>::Out as Nat>::VAL == 2513);
impl NatIndex for Idx<2514> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2514> as NatIndex>::Out as Nat>::VAL == 2514);
impl NatIndex for Idx<2515> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2515> as NatIndex>::Out as Nat>::VAL == 2515);
impl NatIndex for Idx<2516> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2516> as NatIndex>::Out as Nat>::VAL == 2516);
impl NatIndex for Idx<2517> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2517> as NatIndex>::Out as Nat>::VAL == 2517);
impl NatIndex for Idx<2518> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2518> as NatIndex>::Out as Nat>::VAL == 2518);
impl NatIndex for Idx<2519> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2519> as NatIndex>::Out as Nat>::VAL == 2519);
impl NatIndex for Idx<2520> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2520> as NatIndex>::Out as Nat>::VAL == 2520);
impl NatIndex for Idx<2521> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2521> as NatIndex>::Out as Nat>::VAL == 2521);
impl NatIndex for Idx<2522> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2522> as NatIndex>::Out as Nat>::VAL == 2522);
impl NatIndex for Idx<2523> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2523> as NatIndex>::Out as Nat>::VAL == 2523);
impl NatIndex for Idx<2524> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2524> as NatIndex>::Out as Nat>::VAL == 2524);
impl NatIndex for Idx<2525> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2525> as NatIndex>::Out as Nat>::VAL == 2525);
impl NatIndex for Idx<2526> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2526> as NatIndex>::Out as Nat>::VAL == 2526);
impl NatIndex for Idx<2527> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2527> as NatIndex>::Out as Nat>::VAL == 2527);
impl NatIndex for Idx<2528> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2528> as NatIndex>::Out as Nat>::VAL == 2528);
impl NatIndex for Idx<2529> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2529> as NatIndex>::Out as Nat>::VAL == 2529);
impl NatIndex for Idx<2530> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2530> as NatIndex>::Out as Nat>::VAL == 2530);
impl NatIndex for Idx<2531> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2531> as NatIndex>::Out as Nat>::VAL == 2531);
impl NatIndex for Idx<2532> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2532> as NatIndex>::Out as Nat>::VAL == 2532);
impl NatIndex for Idx<2533> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2533> as NatIndex>::Out as Nat>::VAL == 2533);
impl NatIndex for Idx<2534> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2534> as NatIndex>::Out as Nat>::VAL == 2534);
impl NatIndex for Idx<2535> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2535> as NatIndex>::Out as Nat>::VAL == 2535);
impl NatIndex for Idx<2536> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2536> as NatIndex>::Out as Nat>::VAL == 2536);
impl NatIndex for Idx<2537> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2537> as NatIndex>::Out as Nat>::VAL == 2537);
impl NatIndex for Idx<2538> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2538> as NatIndex>::Out as Nat>::VAL == 2538);
impl NatIndex for Idx<2539> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2539> as NatIndex>::Out as Nat>::VAL == 2539);
impl NatIndex for Idx<2540> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2540> as NatIndex>::Out as Nat>::VAL == 2540);
impl NatIndex for Idx<2541> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2541> as NatIndex>::Out as Nat>::VAL == 2541);
impl NatIndex for Idx<2542> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2542> as NatIndex>::Out as Nat>::VAL == 2542);
impl NatIndex for Idx<2543> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2543> as NatIndex>::Out as Nat>::VAL == 2543);
impl NatIndex for Idx<2544> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2544> as NatIndex>::Out as Nat>::VAL == 2544);
impl NatIndex for Idx<2545> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2545> as NatIndex>::Out as Nat>::VAL == 2545);
impl NatIndex for Idx<2546> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2546> as NatIndex>::Out as Nat>::VAL == 2546);
impl NatIndex for Idx<2547> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2547> as NatIndex>::Out as Nat>::VAL == 2547);
impl NatIndex for Idx<2548> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2548> as NatIndex>::Out as Nat>::VAL == 2548);
impl NatIndex for Idx<2549> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2549> as NatIndex>::Out as Nat>::VAL == 2549);
impl NatIndex for Idx<2550> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2550> as NatIndex>::Out as Nat>::VAL == 2550);
impl NatIndex for Idx<2551> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2551> as NatIndex>::Out as Nat>::VAL == 2551);
impl NatIndex for Idx<2552> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2552> as NatIndex>::Out as Nat>::VAL == 2552);
impl NatIndex for Idx<2553> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2553> as NatIndex>::Out as Nat>::VAL == 2553);
impl NatIndex for Idx<2554> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2554> as NatIndex>::Out as Nat>::VAL == 2554);
impl NatIndex for Idx<2555> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2555> as NatIndex>::Out as Nat>::VAL == 2555);
impl NatIndex for Idx<2556> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2556> as NatIndex>::Out as Nat>::VAL == 2556);
impl NatIndex for Idx<2557> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2557> as NatIndex>::Out as Nat>::VAL == 2557);
impl NatIndex for Idx<2558> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2558> as NatIndex>::Out as Nat>::VAL == 2558);
impl NatIndex for Idx<2559> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2559> as NatIndex>::Out as Nat>::VAL == 2559);
impl NatIndex for Idx<2560> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2560> as NatIndex>::Out as Nat>::VAL == 2560);
impl NatIndex for Idx<2561> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2561> as NatIndex>::Out as Nat>::VAL == 2561);
impl NatIndex for Idx<2562> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2562> as NatIndex>::Out as Nat>::VAL == 2562);
impl NatIndex for Idx<2563> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2563> as NatIndex>::Out as Nat>::VAL == 2563);
impl NatIndex for Idx<2564> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2564> as NatIndex>::Out as Nat>::VAL == 2564);
impl NatIndex for Idx<2565> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2565> as NatIndex>::Out as Nat>::VAL == 2565);
impl NatIndex for Idx<2566> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2566> as NatIndex>::Out as Nat>::VAL == 2566);
impl NatIndex for Idx<2567> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2567> as NatIndex>::Out as Nat>::VAL == 2567);
impl NatIndex for Idx<2568> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2568> as NatIndex>::Out as Nat>::VAL == 2568);
impl NatIndex for Idx<2569> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2569> as NatIndex>::Out as Nat>::VAL == 2569);
impl NatIndex for Idx<2570> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2570> as NatIndex>::Out as Nat>::VAL == 2570);
impl NatIndex for Idx<2571> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2571> as NatIndex>::Out as Nat>::VAL == 2571);
impl NatIndex for Idx<2572> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2572> as NatIndex>::Out as Nat>::VAL == 2572);
impl NatIndex for Idx<2573> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2573> as NatIndex>::Out as Nat>::VAL == 2573);
impl NatIndex for Idx<2574> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2574> as NatIndex>::Out as Nat>::VAL == 2574);
impl NatIndex for Idx<2575> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2575> as NatIndex>::Out as Nat>::VAL == 2575);
impl NatIndex for Idx<2576> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2576> as NatIndex>::Out as Nat>::VAL == 2576);
impl NatIndex for Idx<2577> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2577> as NatIndex>::Out as Nat>::VAL == 2577);
impl NatIndex for Idx<2578> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2578> as NatIndex>::Out as Nat>::VAL == 2578);
impl NatIndex for Idx<2579> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2579> as NatIndex>::Out as Nat>::VAL == 2579);
impl NatIndex for Idx<2580> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2580> as NatIndex>::Out as Nat>::VAL == 2580);
impl NatIndex for Idx<2581> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2581> as NatIndex>::Out as Nat>::VAL == 2581);
impl NatIndex for Idx<2582> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2582> as NatIndex>::Out as Nat>::VAL == 2582);
impl NatIndex for Idx<2583> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2583> as NatIndex>::Out as Nat>::VAL == 2583);
impl NatIndex for Idx<2584> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2584> as NatIndex>::Out as Nat>::VAL == 2584);
impl NatIndex for Idx<2585> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2585> as NatIndex>::Out as Nat>::VAL == 2585);
impl NatIndex for Idx<2586> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2586> as NatIndex>::Out as Nat>::VAL == 2586);
impl NatIndex for Idx<2587> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2587> as NatIndex>::Out as Nat>::VAL == 2587);
impl NatIndex for Idx<2588> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2588> as NatIndex>::Out as Nat>::VAL == 2588);
impl NatIndex for Idx<2589> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2589> as NatIndex>::Out as Nat>::VAL == 2589);
impl NatIndex for Idx<2590> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2590> as NatIndex>::Out as Nat>::VAL == 2590);
impl NatIndex for Idx<2591> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2591> as NatIndex>::Out as Nat>::VAL == 2591);
impl NatIndex for Idx<2592> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2592> as NatIndex>::Out as Nat>::VAL == 2592);
impl NatIndex for Idx<2593> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2593> as NatIndex>::Out as Nat>::VAL == 2593);
impl NatIndex for Idx<2594> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2594> as NatIndex>::Out as Nat>::VAL == 2594);
impl NatIndex for Idx<2595> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2595> as NatIndex>::Out as Nat>::VAL == 2595);
impl NatIndex for Idx<2596> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2596> as NatIndex>::Out as Nat>::VAL == 2596);
impl NatIndex for Idx<2597> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2597> as NatIndex>::Out as Nat>::VAL == 2597);
impl NatIndex for Idx<2598> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2598> as NatIndex>::Out as Nat>::VAL == 2598);
impl NatIndex for Idx<2599> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2599> as NatIndex>::Out as Nat>::VAL == 2599);
impl NatIndex for Idx<2600> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2600> as NatIndex>::Out as Nat>::VAL == 2600);
impl NatIndex for Idx<2601> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2601> as NatIndex>::Out as Nat>::VAL == 2601);
impl NatIndex for Idx<2602> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2602> as NatIndex>::Out as Nat>::VAL == 2602);
impl NatIndex for Idx<2603> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2603> as NatIndex>::Out as Nat>::VAL == 2603);
impl NatIndex for Idx<2604> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2604> as NatIndex>::Out as Nat>::VAL == 2604);
impl NatIndex for Idx<2605> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2605> as NatIndex>::Out as Nat>::VAL == 2605);
impl NatIndex for Idx<2606> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2606> as NatIndex>::Out as Nat>::VAL == 2606);
impl NatIndex for Idx<2607> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2607> as NatIndex>::Out as Nat>::VAL == 2607);
impl NatIndex for Idx<2608> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2608> as NatIndex>::Out as Nat>::VAL == 2608);
impl NatIndex for Idx<2609> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2609> as NatIndex>::Out as Nat>::VAL == 2609);
impl NatIndex for Idx<2610> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2610> as NatIndex>::Out as Nat>::VAL == 2610);
impl NatIndex for Idx<2611> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2611> as NatIndex>::Out as Nat>::VAL == 2611);
impl NatIndex for Idx<2612> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2612> as NatIndex>::Out as Nat>::VAL == 2612);
impl NatIndex for Idx<2613> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2613> as NatIndex>::Out as Nat>::VAL == 2613);
impl NatIndex for Idx<2614> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2614> as NatIndex>::Out as Nat>::VAL == 2614);
impl NatIndex for Idx<2615> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2615> as NatIndex>::Out as Nat>::VAL == 2615);
impl NatIndex for Idx<2616> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2616> as NatIndex>::Out as Nat>::VAL == 2616);
impl NatIndex for Idx<2617> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2617> as NatIndex>::Out as Nat>::VAL == 2617);
impl NatIndex for Idx<2618> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2618> as NatIndex>::Out as Nat>::VAL == 2618);
impl NatIndex for Idx<2619> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2619> as NatIndex>::Out as Nat>::VAL == 2619);
impl NatIndex for Idx<2620> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2620> as NatIndex>::Out as Nat>::VAL == 2620);
impl NatIndex for Idx<2621> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2621> as NatIndex>::Out as Nat>::VAL == 2621);
impl NatIndex for Idx<2622> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2622> as NatIndex>::Out as Nat>::VAL == 2622);
impl NatIndex for Idx<2623> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2623> as NatIndex>::Out as Nat>::VAL == 2623);
impl NatIndex for Idx<2624> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2624> as NatIndex>::Out as Nat>::VAL == 2624);
impl NatIndex for Idx<2625> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2625> as NatIndex>::Out as Nat>::VAL == 2625);
impl NatIndex for Idx<2626> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2626> as NatIndex>::Out as Nat>::VAL == 2626);
impl NatIndex for Idx<2627> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2627> as NatIndex>::Out as Nat>::VAL == 2627);
impl NatIndex for Idx<2628> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2628> as NatIndex>::Out as Nat>::VAL == 2628);
impl NatIndex for Idx<2629> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2629> as NatIndex>::Out as Nat>::VAL == 2629);
impl NatIndex for Idx<2630> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2630> as NatIndex>::Out as Nat>::VAL == 2630);
impl NatIndex for Idx<2631> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2631> as NatIndex>::Out as Nat>::VAL == 2631);
impl NatIndex for Idx<2632> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2632> as NatIndex>::Out as Nat>::VAL == 2632);
impl NatIndex for Idx<2633> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2633> as NatIndex>::Out as Nat>::VAL == 2633);
impl NatIndex for Idx<2634> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2634> as NatIndex>::Out as Nat>::VAL == 2634);
impl NatIndex for Idx<2635> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2635> as NatIndex>::Out as Nat>::VAL == 2635);
impl NatIndex for Idx<2636> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2636> as NatIndex>::Out as Nat>::VAL == 2636);
impl NatIndex for Idx<2637> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2637> as NatIndex>::Out as Nat>::VAL == 2637);
impl NatIndex for Idx<2638> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2638> as NatIndex>::Out as Nat>::VAL == 2638);
impl NatIndex for Idx<2639> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2639> as NatIndex>::Out as Nat>::VAL == 2639);
impl NatIndex for Idx<2640> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2640> as NatIndex>::Out as Nat>::VAL == 2640);
impl NatIndex for Idx<2641> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2641> as NatIndex>::Out as Nat>::VAL == 2641);
impl NatIndex for Idx<2642> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2642> as NatIndex>::Out as Nat>::VAL == 2642);
impl NatIndex for Idx<2643> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2643> as NatIndex>::Out as Nat>::VAL == 2643);
impl NatIndex for Idx<2644> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2644> as NatIndex>::Out as Nat>::VAL == 2644);
impl NatIndex for Idx<2645> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2645> as NatIndex>::Out as Nat>::VAL == 2645);
impl NatIndex for Idx<2646> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2646> as NatIndex>::Out as Nat>::VAL == 2646);
impl NatIndex for Idx<2647> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2647> as NatIndex>::Out as Nat>::VAL == 2647);
impl NatIndex for Idx<2648> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2648> as NatIndex>::Out as Nat>::VAL == 2648);
impl NatIndex for Idx<2649> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2649> as NatIndex>::Out as Nat>::VAL == 2649);
impl NatIndex for Idx<2650> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2650> as NatIndex>::Out as Nat>::VAL == 2650);
impl NatIndex for Idx<2651> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2651> as NatIndex>::Out as Nat>::VAL == 2651);
impl NatIndex for Idx<2652> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2652> as NatIndex>::Out as Nat>::VAL == 2652);
impl NatIndex for Idx<2653> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2653> as NatIndex>::Out as Nat>::VAL == 2653);
impl NatIndex for Idx<2654> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2654> as NatIndex>::Out as Nat>::VAL == 2654);
impl NatIndex for Idx<2655> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2655> as NatIndex>::Out as Nat>::VAL == 2655);
impl NatIndex for Idx<2656> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2656> as NatIndex>::Out as Nat>::VAL == 2656);
impl NatIndex for Idx<2657> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2657> as NatIndex>::Out as Nat>::VAL == 2657);
impl NatIndex for Idx<2658> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2658> as NatIndex>::Out as Nat>::VAL == 2658);
impl NatIndex for Idx<2659> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2659> as NatIndex>::Out as Nat>::VAL == 2659);
impl NatIndex for Idx<2660> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2660> as NatIndex>::Out as Nat>::VAL == 2660);
impl NatIndex for Idx<2661> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2661> as NatIndex>::Out as Nat>::VAL == 2661);
impl NatIndex for Idx<2662> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2662> as NatIndex>::Out as Nat>::VAL == 2662);
impl NatIndex for Idx<2663> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2663> as NatIndex>::Out as Nat>::VAL == 2663);
impl NatIndex for Idx<2664> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2664> as NatIndex>::Out as Nat>::VAL == 2664);
impl NatIndex for Idx<2665> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2665> as NatIndex>::Out as Nat>::VAL == 2665);
impl NatIndex for Idx<2666> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2666> as NatIndex>::Out as Nat>::VAL == 2666);
impl NatIndex for Idx<2667> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2667> as NatIndex>::Out as Nat>::VAL == 2667);
impl NatIndex for Idx<2668> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2668> as NatIndex>::Out as Nat>::VAL == 2668);
impl NatIndex for Idx<2669> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2669> as NatIndex>::Out as Nat>::VAL == 2669);
impl NatIndex for Idx<2670> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2670> as NatIndex>::Out as Nat>::VAL == 2670);
impl NatIndex for Idx<2671> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2671> as NatIndex>::Out as Nat>::VAL == 2671);
impl NatIndex for Idx<2672> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2672> as NatIndex>::Out as Nat>::VAL == 2672);
impl NatIndex for Idx<2673> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2673> as NatIndex>::Out as Nat>::VAL == 2673);
impl NatIndex for Idx<2674> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2674> as NatIndex>::Out as Nat>::VAL == 2674);
impl NatIndex for Idx<2675> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2675> as NatIndex>::Out as Nat>::VAL == 2675);
impl NatIndex for Idx<2676> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2676> as NatIndex>::Out as Nat>::VAL == 2676);
impl NatIndex for Idx<2677> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2677> as NatIndex>::Out as Nat>::VAL == 2677);
impl NatIndex for Idx<2678> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2678> as NatIndex>::Out as Nat>::VAL == 2678);
impl NatIndex for Idx<2679> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2679> as NatIndex>::Out as Nat>::VAL == 2679);
impl NatIndex for Idx<2680> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2680> as NatIndex>::Out as Nat>::VAL == 2680);
impl NatIndex for Idx<2681> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2681> as NatIndex>::Out as Nat>::VAL == 2681);
impl NatIndex for Idx<2682> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2682> as NatIndex>::Out as Nat>::VAL == 2682);
impl NatIndex for Idx<2683> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2683> as NatIndex>::Out as Nat>::VAL == 2683);
impl NatIndex for Idx<2684> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2684> as NatIndex>::Out as Nat>::VAL == 2684);
impl NatIndex for Idx<2685> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2685> as NatIndex>::Out as Nat>::VAL == 2685);
impl NatIndex for Idx<2686> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2686> as NatIndex>::Out as Nat>::VAL == 2686);
impl NatIndex for Idx<2687> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2687> as NatIndex>::Out as Nat>::VAL == 2687);
impl NatIndex for Idx<2688> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2688> as NatIndex>::Out as Nat>::VAL == 2688);
impl NatIndex for Idx<2689> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2689> as NatIndex>::Out as Nat>::VAL == 2689);
impl NatIndex for Idx<2690> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2690> as NatIndex>::Out as Nat>::VAL == 2690);
impl NatIndex for Idx<2691> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2691> as NatIndex>::Out as Nat>::VAL == 2691);
impl NatIndex for Idx<2692> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2692> as NatIndex>::Out as Nat>::VAL == 2692);
impl NatIndex for Idx<2693> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2693> as NatIndex>::Out as Nat>::VAL == 2693);
impl NatIndex for Idx<2694> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2694> as NatIndex>::Out as Nat>::VAL == 2694);
impl NatIndex for Idx<2695> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2695> as NatIndex>::Out as Nat>::VAL == 2695);
impl NatIndex for Idx<2696> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2696> as NatIndex>::Out as Nat>::VAL == 2696);
impl NatIndex for Idx<2697> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2697> as NatIndex>::Out as Nat>::VAL == 2697);
impl NatIndex for Idx<2698> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2698> as NatIndex>::Out as Nat>::VAL == 2698);
impl NatIndex for Idx<2699> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2699> as NatIndex>::Out as Nat>::VAL == 2699);
impl NatIndex for Idx<2700> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2700> as NatIndex>::Out as Nat>::VAL == 2700);
impl NatIndex for Idx<2701> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2701> as NatIndex>::Out as Nat>::VAL == 2701);
impl NatIndex for Idx<2702> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2702> as NatIndex>::Out as Nat>::VAL == 2702);
impl NatIndex for Idx<2703> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2703> as NatIndex>::Out as Nat>::VAL == 2703);
impl NatIndex for Idx<2704> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2704> as NatIndex>::Out as Nat>::VAL == 2704);
impl NatIndex for Idx<2705> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2705> as NatIndex>::Out as Nat>::VAL == 2705);
impl NatIndex for Idx<2706> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2706> as NatIndex>::Out as Nat>::VAL == 2706);
impl NatIndex for Idx<2707> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2707> as NatIndex>::Out as Nat>::VAL == 2707);
impl NatIndex for Idx<2708> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2708> as NatIndex>::Out as Nat>::VAL == 2708);
impl NatIndex for Idx<2709> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2709> as NatIndex>::Out as Nat>::VAL == 2709);
impl NatIndex for Idx<2710> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2710> as NatIndex>::Out as Nat>::VAL == 2710);
impl NatIndex for Idx<2711> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2711> as NatIndex>::Out as Nat>::VAL == 2711);
impl NatIndex for Idx<2712> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2712> as NatIndex>::Out as Nat>::VAL == 2712);
impl NatIndex for Idx<2713> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2713> as NatIndex>::Out as Nat>::VAL == 2713);
impl NatIndex for Idx<2714> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2714> as NatIndex>::Out as Nat>::VAL == 2714);
impl NatIndex for Idx<2715> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2715> as NatIndex>::Out as Nat>::VAL == 2715);
impl NatIndex for Idx<2716> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2716> as NatIndex>::Out as Nat>::VAL == 2716);
impl NatIndex for Idx<2717> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2717> as NatIndex>::Out as Nat>::VAL == 2717);
impl NatIndex for Idx<2718> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2718> as NatIndex>::Out as Nat>::VAL == 2718);
impl NatIndex for Idx<2719> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2719> as NatIndex>::Out as Nat>::VAL == 2719);
impl NatIndex for Idx<2720> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2720> as NatIndex>::Out as Nat>::VAL == 2720);
impl NatIndex for Idx<2721> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2721> as NatIndex>::Out as Nat>::VAL == 2721);
impl NatIndex for Idx<2722> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2722> as NatIndex>::Out as Nat>::VAL == 2722);
impl NatIndex for Idx<2723> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2723> as NatIndex>::Out as Nat>::VAL == 2723);
impl NatIndex for Idx<2724> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2724> as NatIndex>::Out as Nat>::VAL == 2724);
impl NatIndex for Idx<2725> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2725> as NatIndex>::Out as Nat>::VAL == 2725);
impl NatIndex for Idx<2726> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2726> as NatIndex>::Out as Nat>::VAL == 2726);
impl NatIndex for Idx<2727> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2727> as NatIndex>::Out as Nat>::VAL == 2727);
impl NatIndex for Idx<2728> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2728> as NatIndex>::Out as Nat>::VAL == 2728);
impl NatIndex for Idx<2729> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2729> as NatIndex>::Out as Nat>::VAL == 2729);
impl NatIndex for Idx<2730> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2730> as NatIndex>::Out as Nat>::VAL == 2730);
impl NatIndex for Idx<2731> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2731> as NatIndex>::Out as Nat>::VAL == 2731);
impl NatIndex for Idx<2732> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2732> as NatIndex>::Out as Nat>::VAL == 2732);
impl NatIndex for Idx<2733> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2733> as NatIndex>::Out as Nat>::VAL == 2733);
impl NatIndex for Idx<2734> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2734> as NatIndex>::Out as Nat>::VAL == 2734);
impl NatIndex for Idx<2735> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2735> as NatIndex>::Out as Nat>::VAL == 2735);
impl NatIndex for Idx<2736> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2736> as NatIndex>::Out as Nat>::VAL == 2736);
impl NatIndex for Idx<2737> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2737> as NatIndex>::Out as Nat>::VAL == 2737);
impl NatIndex for Idx<2738> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2738> as NatIndex>::Out as Nat>::VAL == 2738);
impl NatIndex for Idx<2739> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2739> as NatIndex>::Out as Nat>::VAL == 2739);
impl NatIndex for Idx<2740> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2740> as NatIndex>::Out as Nat>::VAL == 2740);
impl NatIndex for Idx<2741> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2741> as NatIndex>::Out as Nat>::VAL == 2741);
impl NatIndex for Idx<2742> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2742> as NatIndex>::Out as Nat>::VAL == 2742);
impl NatIndex for Idx<2743> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2743> as NatIndex>::Out as Nat>::VAL == 2743);
impl NatIndex for Idx<2744> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2744> as NatIndex>::Out as Nat>::VAL == 2744);
impl NatIndex for Idx<2745> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2745> as NatIndex>::Out as Nat>::VAL == 2745);
impl NatIndex for Idx<2746> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2746> as NatIndex>::Out as Nat>::VAL == 2746);
impl NatIndex for Idx<2747> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2747> as NatIndex>::Out as Nat>::VAL == 2747);
impl NatIndex for Idx<2748> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2748> as NatIndex>::Out as Nat>::VAL == 2748);
impl NatIndex for Idx<2749> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2749> as NatIndex>::Out as Nat>::VAL == 2749);
impl NatIndex for Idx<2750> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2750> as NatIndex>::Out as Nat>::VAL == 2750);
impl NatIndex for Idx<2751> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2751> as NatIndex>::Out as Nat>::VAL == 2751);
impl NatIndex for Idx<2752> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2752> as NatIndex>::Out as Nat>::VAL == 2752);
impl NatIndex for Idx<2753> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2753> as NatIndex>::Out as Nat>::VAL == 2753);
impl NatIndex for Idx<2754> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2754> as NatIndex>::Out as Nat>::VAL == 2754);
impl NatIndex for Idx<2755> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2755> as NatIndex>::Out as Nat>::VAL == 2755);
impl NatIndex for Idx<2756> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2756> as NatIndex>::Out as Nat>::VAL == 2756);
impl NatIndex for Idx<2757> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2757> as NatIndex>::Out as Nat>::VAL == 2757);
impl NatIndex for Idx<2758> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2758> as NatIndex>::Out as Nat>::VAL == 2758);
impl NatIndex for Idx<2759> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2759> as NatIndex>::Out as Nat>::VAL == 2759);
impl NatIndex for Idx<2760> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2760> as NatIndex>::Out as Nat>::VAL == 2760);
impl NatIndex for Idx<2761> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2761> as NatIndex>::Out as Nat>::VAL == 2761);
impl NatIndex for Idx<2762> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2762> as NatIndex>::Out as Nat>::VAL == 2762);
impl NatIndex for Idx<2763> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2763> as NatIndex>::Out as Nat>::VAL == 2763);
impl NatIndex for Idx<2764> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2764> as NatIndex>::Out as Nat>::VAL == 2764);
impl NatIndex for Idx<2765> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2765> as NatIndex>::Out as Nat>::VAL == 2765);
impl NatIndex for Idx<2766> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2766> as NatIndex>::Out as Nat>::VAL == 2766);
impl NatIndex for Idx<2767> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2767> as NatIndex>::Out as Nat>::VAL == 2767);
impl NatIndex for Idx<2768> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2768> as NatIndex>::Out as Nat>::VAL == 2768);
impl NatIndex for Idx<2769> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2769> as NatIndex>::Out as Nat>::VAL == 2769);
impl NatIndex for Idx<2770> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2770> as NatIndex>::Out as Nat>::VAL == 2770);
impl NatIndex for Idx<2771> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2771> as NatIndex>::Out as Nat>::VAL == 2771);
impl NatIndex for Idx<2772> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2772> as NatIndex>::Out as Nat>::VAL == 2772);
impl NatIndex for Idx<2773> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2773> as NatIndex>::Out as Nat>::VAL == 2773);
impl NatIndex for Idx<2774> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2774> as NatIndex>::Out as Nat>::VAL == 2774);
impl NatIndex for Idx<2775> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2775> as NatIndex>::Out as Nat>::VAL == 2775);
impl NatIndex for Idx<2776> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2776> as NatIndex>::Out as Nat>::VAL == 2776);
impl NatIndex for Idx<2777> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2777> as NatIndex>::Out as Nat>::VAL == 2777);
impl NatIndex for Idx<2778> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2778> as NatIndex>::Out as Nat>::VAL == 2778);
impl NatIndex for Idx<2779> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2779> as NatIndex>::Out as Nat>::VAL == 2779);
impl NatIndex for Idx<2780> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2780> as NatIndex>::Out as Nat>::VAL == 2780);
impl NatIndex for Idx<2781> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2781> as NatIndex>::Out as Nat>::VAL == 2781);
impl NatIndex for Idx<2782> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2782> as NatIndex>::Out as Nat>::VAL == 2782);
impl NatIndex for Idx<2783> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2783> as NatIndex>::Out as Nat>::VAL == 2783);
impl NatIndex for Idx<2784> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2784> as NatIndex>::Out as Nat>::VAL == 2784);
impl NatIndex for Idx<2785> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2785> as NatIndex>::Out as Nat>::VAL == 2785);
impl NatIndex for Idx<2786> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2786> as NatIndex>::Out as Nat>::VAL == 2786);
impl NatIndex for Idx<2787> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2787> as NatIndex>::Out as Nat>::VAL == 2787);
impl NatIndex for Idx<2788> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2788> as NatIndex>::Out as Nat>::VAL == 2788);
impl NatIndex for Idx<2789> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2789> as NatIndex>::Out as Nat>::VAL == 2789);
impl NatIndex for Idx<2790> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2790> as NatIndex>::Out as Nat>::VAL == 2790);
impl NatIndex for Idx<2791> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2791> as NatIndex>::Out as Nat>::VAL == 2791);
impl NatIndex for Idx<2792> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2792> as NatIndex>::Out as Nat>::VAL == 2792);
impl NatIndex for Idx<2793> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2793> as NatIndex>::Out as Nat>::VAL == 2793);
impl NatIndex for Idx<2794> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2794> as NatIndex>::Out as Nat>::VAL == 2794);
impl NatIndex for Idx<2795> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2795> as NatIndex>::Out as Nat>::VAL == 2795);
impl NatIndex for Idx<2796> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2796> as NatIndex>::Out as Nat>::VAL == 2796);
impl NatIndex for Idx<2797> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2797> as NatIndex>::Out as Nat>::VAL == 2797);
impl NatIndex for Idx<2798> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2798> as NatIndex>::Out as Nat>::VAL == 2798);
impl NatIndex for Idx<2799> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2799> as NatIndex>::Out as Nat>::VAL == 2799);
impl NatIndex for Idx<2800> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2800> as NatIndex>::Out as Nat>::VAL == 2800);
impl NatIndex for Idx<2801> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2801> as NatIndex>::Out as Nat>::VAL == 2801);
impl NatIndex for Idx<2802> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2802> as NatIndex>::Out as Nat>::VAL == 2802);
impl NatIndex for Idx<2803> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2803> as NatIndex>::Out as Nat>::VAL == 2803);
impl NatIndex for Idx<2804> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2804> as NatIndex>::Out as Nat>::VAL == 2804);
impl NatIndex for Idx<2805> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2805> as NatIndex>::Out as Nat>::VAL == 2805);
impl NatIndex for Idx<2806> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2806> as NatIndex>::Out as Nat>::VAL == 2806);
impl NatIndex for Idx<2807> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2807> as NatIndex>::Out as Nat>::VAL == 2807);
impl NatIndex for Idx<2808> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2808> as NatIndex>::Out as Nat>::VAL == 2808);
impl NatIndex for Idx<2809> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2809> as NatIndex>::Out as Nat>::VAL == 2809);
impl NatIndex for Idx<2810> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2810> as NatIndex>::Out as Nat>::VAL == 2810);
impl NatIndex for Idx<2811> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2811> as NatIndex>::Out as Nat>::VAL == 2811);
impl NatIndex for Idx<2812> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2812> as NatIndex>::Out as Nat>::VAL == 2812);
impl NatIndex for Idx<2813> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2813> as NatIndex>::Out as Nat>::VAL == 2813);
impl NatIndex for Idx<2814> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2814> as NatIndex>::Out as Nat>::VAL == 2814);
impl NatIndex for Idx<2815> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2815> as NatIndex>::Out as Nat>::VAL == 2815);
impl NatIndex for Idx<2816> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2816> as NatIndex>::Out as Nat>::VAL == 2816);
impl NatIndex for Idx<2817> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2817> as NatIndex>::Out as Nat>::VAL == 2817);
impl NatIndex for Idx<2818> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2818> as NatIndex>::Out as Nat>::VAL == 2818);
impl NatIndex for Idx<2819> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2819> as NatIndex>::Out as Nat>::VAL == 2819);
impl NatIndex for Idx<2820> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2820> as NatIndex>::Out as Nat>::VAL == 2820);
impl NatIndex for Idx<2821> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2821> as NatIndex>::Out as Nat>::VAL == 2821);
impl NatIndex for Idx<2822> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2822> as NatIndex>::Out as Nat>::VAL == 2822);
impl NatIndex for Idx<2823> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2823> as NatIndex>::Out as Nat>::VAL == 2823);
impl NatIndex for Idx<2824> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2824> as NatIndex>::Out as Nat>::VAL == 2824);
impl NatIndex for Idx<2825> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2825> as NatIndex>::Out as Nat>::VAL == 2825);
impl NatIndex for Idx<2826> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2826> as NatIndex>::Out as Nat>::VAL == 2826);
impl NatIndex for Idx<2827> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2827> as NatIndex>::Out as Nat>::VAL == 2827);
impl NatIndex for Idx<2828> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2828> as NatIndex>::Out as Nat>::VAL == 2828);
impl NatIndex for Idx<2829> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2829> as NatIndex>::Out as Nat>::VAL == 2829);
impl NatIndex for Idx<2830> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2830> as NatIndex>::Out as Nat>::VAL == 2830);
impl NatIndex for Idx<2831> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2831> as NatIndex>::Out as Nat>::VAL == 2831);
impl NatIndex for Idx<2832> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2832> as NatIndex>::Out as Nat>::VAL == 2832);
impl NatIndex for Idx<2833> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2833> as NatIndex>::Out as Nat>::VAL == 2833);
impl NatIndex for Idx<2834> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2834> as NatIndex>::Out as Nat>::VAL == 2834);
impl NatIndex for Idx<2835> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2835> as NatIndex>::Out as Nat>::VAL == 2835);
impl NatIndex for Idx<2836> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2836> as NatIndex>::Out as Nat>::VAL == 2836);
impl NatIndex for Idx<2837> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2837> as NatIndex>::Out as Nat>::VAL == 2837);
impl NatIndex for Idx<2838> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2838> as NatIndex>::Out as Nat>::VAL == 2838);
impl NatIndex for Idx<2839> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2839> as NatIndex>::Out as Nat>::VAL == 2839);
impl NatIndex for Idx<2840> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2840> as NatIndex>::Out as Nat>::VAL == 2840);
impl NatIndex for Idx<2841> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2841> as NatIndex>::Out as Nat>::VAL == 2841);
impl NatIndex for Idx<2842> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2842> as NatIndex>::Out as Nat>::VAL == 2842);
impl NatIndex for Idx<2843> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2843> as NatIndex>::Out as Nat>::VAL == 2843);
impl NatIndex for Idx<2844> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2844> as NatIndex>::Out as Nat>::VAL == 2844);
impl NatIndex for Idx<2845> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2845> as NatIndex>::Out as Nat>::VAL == 2845);
impl NatIndex for Idx<2846> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2846> as NatIndex>::Out as Nat>::VAL == 2846);
impl NatIndex for Idx<2847> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2847> as NatIndex>::Out as Nat>::VAL == 2847);
impl NatIndex for Idx<2848> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2848> as NatIndex>::Out as Nat>::VAL == 2848);
impl NatIndex for Idx<2849> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2849> as NatIndex>::Out as Nat>::VAL == 2849);
impl NatIndex for Idx<2850> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2850> as NatIndex>::Out as Nat>::VAL == 2850);
impl NatIndex for Idx<2851> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2851> as NatIndex>::Out as Nat>::VAL == 2851);
impl NatIndex for Idx<2852> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2852> as NatIndex>::Out as Nat>::VAL == 2852);
impl NatIndex for Idx<2853> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2853> as NatIndex>::Out as Nat>::VAL == 2853);
impl NatIndex for Idx<2854> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2854> as NatIndex>::Out as Nat>::VAL == 2854);
impl NatIndex for Idx<2855> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2855> as NatIndex>::Out as Nat>::VAL == 2855);
impl NatIndex for Idx<2856> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2856> as NatIndex>::Out as Nat>::VAL == 2856);
impl NatIndex for Idx<2857> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2857> as NatIndex>::Out as Nat>::VAL == 2857);
impl NatIndex for Idx<2858> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2858> as NatIndex>::Out as Nat>::VAL == 2858);
impl NatIndex for Idx<2859> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2859> as NatIndex>::Out as Nat>::VAL == 2859);
impl NatIndex for Idx<2860> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2860> as NatIndex>::Out as Nat>::VAL == 2860);
impl NatIndex for Idx<2861> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2861> as NatIndex>::Out as Nat>::VAL == 2861);
impl NatIndex for Idx<2862> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2862> as NatIndex>::Out as Nat>::VAL == 2862);
impl NatIndex for Idx<2863> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2863> as NatIndex>::Out as Nat>::VAL == 2863);
impl NatIndex for Idx<2864> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2864> as NatIndex>::Out as Nat>::VAL == 2864);
impl NatIndex for Idx<2865> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2865> as NatIndex>::Out as Nat>::VAL == 2865);
impl NatIndex for Idx<2866> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2866> as NatIndex>::Out as Nat>::VAL == 2866);
impl NatIndex for Idx<2867> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2867> as NatIndex>::Out as Nat>::VAL == 2867);
impl NatIndex for Idx<2868> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2868> as NatIndex>::Out as Nat>::VAL == 2868);
impl NatIndex for Idx<2869> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2869> as NatIndex>::Out as Nat>::VAL == 2869);
impl NatIndex for Idx<2870> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2870> as NatIndex>::Out as Nat>::VAL == 2870);
impl NatIndex for Idx<2871> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2871> as NatIndex>::Out as Nat>::VAL == 2871);
impl NatIndex for Idx<2872> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2872> as NatIndex>::Out as Nat>::VAL == 2872);
impl NatIndex for Idx<2873> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2873> as NatIndex>::Out as Nat>::VAL == 2873);
impl NatIndex for Idx<2874> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2874> as NatIndex>::Out as Nat>::VAL == 2874);
impl NatIndex for Idx<2875> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2875> as NatIndex>::Out as Nat>::VAL == 2875);
impl NatIndex for Idx<2876> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2876> as NatIndex>::Out as Nat>::VAL == 2876);
impl NatIndex for Idx<2877> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2877> as NatIndex>::Out as Nat>::VAL == 2877);
impl NatIndex for Idx<2878> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2878> as NatIndex>::Out as Nat>::VAL == 2878);
impl NatIndex for Idx<2879> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2879> as NatIndex>::Out as Nat>::VAL == 2879);
impl NatIndex for Idx<2880> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2880> as NatIndex>::Out as Nat>::VAL == 2880);
impl NatIndex for Idx<2881> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2881> as NatIndex>::Out as Nat>::VAL == 2881);
impl NatIndex for Idx<2882> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2882> as NatIndex>::Out as Nat>::VAL == 2882);
impl NatIndex for Idx<2883> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2883> as NatIndex>::Out as Nat>::VAL == 2883);
impl NatIndex for Idx<2884> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2884> as NatIndex>::Out as Nat>::VAL == 2884);
impl NatIndex for Idx<2885> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2885> as NatIndex>::Out as Nat>::VAL == 2885);
impl NatIndex for Idx<2886> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2886> as NatIndex>::Out as Nat>::VAL == 2886);
impl NatIndex for Idx<2887> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2887> as NatIndex>::Out as Nat>::VAL == 2887);
impl NatIndex for Idx<2888> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2888> as NatIndex>::Out as Nat>::VAL == 2888);
impl NatIndex for Idx<2889> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2889> as NatIndex>::Out as Nat>::VAL == 2889);
impl NatIndex for Idx<2890> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2890> as NatIndex>::Out as Nat>::VAL == 2890);
impl NatIndex for Idx<2891> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2891> as NatIndex>::Out as Nat>::VAL == 2891);
impl NatIndex for Idx<2892> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2892> as NatIndex>::Out as Nat>::VAL == 2892);
impl NatIndex for Idx<2893> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2893> as NatIndex>::Out as Nat>::VAL == 2893);
impl NatIndex for Idx<2894> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2894> as NatIndex>::Out as Nat>::VAL == 2894);
impl NatIndex for Idx<2895> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2895> as NatIndex>::Out as Nat>::VAL == 2895);
impl NatIndex for Idx<2896> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2896> as NatIndex>::Out as Nat>::VAL == 2896);
impl NatIndex for Idx<2897> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2897> as NatIndex>::Out as Nat>::VAL == 2897);
impl NatIndex for Idx<2898> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2898> as NatIndex>::Out as Nat>::VAL == 2898);
impl NatIndex for Idx<2899> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2899> as NatIndex>::Out as Nat>::VAL == 2899);
impl NatIndex for Idx<2900> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2900> as NatIndex>::Out as Nat>::VAL == 2900);
impl NatIndex for Idx<2901> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2901> as NatIndex>::Out as Nat>::VAL == 2901);
impl NatIndex for Idx<2902> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2902> as NatIndex>::Out as Nat>::VAL == 2902);
impl NatIndex for Idx<2903> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2903> as NatIndex>::Out as Nat>::VAL == 2903);
impl NatIndex for Idx<2904> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2904> as NatIndex>::Out as Nat>::VAL == 2904);
impl NatIndex for Idx<2905> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2905> as NatIndex>::Out as Nat>::VAL == 2905);
impl NatIndex for Idx<2906> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2906> as NatIndex>::Out as Nat>::VAL == 2906);
impl NatIndex for Idx<2907> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2907> as NatIndex>::Out as Nat>::VAL == 2907);
impl NatIndex for Idx<2908> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2908> as NatIndex>::Out as Nat>::VAL == 2908);
impl NatIndex for Idx<2909> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2909> as NatIndex>::Out as Nat>::VAL == 2909);
impl NatIndex for Idx<2910> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2910> as NatIndex>::Out as Nat>::VAL == 2910);
impl NatIndex for Idx<2911> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2911> as NatIndex>::Out as Nat>::VAL == 2911);
impl NatIndex for Idx<2912> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2912> as NatIndex>::Out as Nat>::VAL == 2912);
impl NatIndex for Idx<2913> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2913> as NatIndex>::Out as Nat>::VAL == 2913);
impl NatIndex for Idx<2914> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2914> as NatIndex>::Out as Nat>::VAL == 2914);
impl NatIndex for Idx<2915> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2915> as NatIndex>::Out as Nat>::VAL == 2915);
impl NatIndex for Idx<2916> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2916> as NatIndex>::Out as Nat>::VAL == 2916);
impl NatIndex for Idx<2917> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2917> as NatIndex>::Out as Nat>::VAL == 2917);
impl NatIndex for Idx<2918> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2918> as NatIndex>::Out as Nat>::VAL == 2918);
impl NatIndex for Idx<2919> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2919> as NatIndex>::Out as Nat>::VAL == 2919);
impl NatIndex for Idx<2920> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2920> as NatIndex>::Out as Nat>::VAL == 2920);
impl NatIndex for Idx<2921> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2921> as NatIndex>::Out as Nat>::VAL == 2921);
impl NatIndex for Idx<2922> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2922> as NatIndex>::Out as Nat>::VAL == 2922);
impl NatIndex for Idx<2923> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2923> as NatIndex>::Out as Nat>::VAL == 2923);
impl NatIndex for Idx<2924> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2924> as NatIndex>::Out as Nat>::VAL == 2924);
impl NatIndex for Idx<2925> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2925> as NatIndex>::Out as Nat>::VAL == 2925);
impl NatIndex for Idx<2926> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2926> as NatIndex>::Out as Nat>::VAL == 2926);
impl NatIndex for Idx<2927> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2927> as NatIndex>::Out as Nat>::VAL == 2927);
impl NatIndex for Idx<2928> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2928> as NatIndex>::Out as Nat>::VAL == 2928);
impl NatIndex for Idx<2929> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2929> as NatIndex>::Out as Nat>::VAL == 2929);
impl NatIndex for Idx<2930> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2930> as NatIndex>::Out as Nat>::VAL == 2930);
impl NatIndex for Idx<2931> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2931> as NatIndex>::Out as Nat>::VAL == 2931);
impl NatIndex for Idx<2932> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2932> as NatIndex>::Out as Nat>::VAL == 2932);
impl NatIndex for Idx<2933> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2933> as NatIndex>::Out as Nat>::VAL == 2933);
impl NatIndex for Idx<2934> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2934> as NatIndex>::Out as Nat>::VAL == 2934);
impl NatIndex for Idx<2935> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2935> as NatIndex>::Out as Nat>::VAL == 2935);
impl NatIndex for Idx<2936> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2936> as NatIndex>::Out as Nat>::VAL == 2936);
impl NatIndex for Idx<2937> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2937> as NatIndex>::Out as Nat>::VAL == 2937);
impl NatIndex for Idx<2938> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2938> as NatIndex>::Out as Nat>::VAL == 2938);
impl NatIndex for Idx<2939> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2939> as NatIndex>::Out as Nat>::VAL == 2939);
impl NatIndex for Idx<2940> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2940> as NatIndex>::Out as Nat>::VAL == 2940);
impl NatIndex for Idx<2941> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2941> as NatIndex>::Out as Nat>::VAL == 2941);
impl NatIndex for Idx<2942> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2942> as NatIndex>::Out as Nat>::VAL == 2942);
impl NatIndex for Idx<2943> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2943> as NatIndex>::Out as Nat>::VAL == 2943);
impl NatIndex for Idx<2944> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2944> as NatIndex>::Out as Nat>::VAL == 2944);
impl NatIndex for Idx<2945> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2945> as NatIndex>::Out as Nat>::VAL == 2945);
impl NatIndex for Idx<2946> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2946> as NatIndex>::Out as Nat>::VAL == 2946);
impl NatIndex for Idx<2947> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2947> as NatIndex>::Out as Nat>::VAL == 2947);
impl NatIndex for Idx<2948> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2948> as NatIndex>::Out as Nat>::VAL == 2948);
impl NatIndex for Idx<2949> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2949> as NatIndex>::Out as Nat>::VAL == 2949);
impl NatIndex for Idx<2950> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2950> as NatIndex>::Out as Nat>::VAL == 2950);
impl NatIndex for Idx<2951> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2951> as NatIndex>::Out as Nat>::VAL == 2951);
impl NatIndex for Idx<2952> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2952> as NatIndex>::Out as Nat>::VAL == 2952);
impl NatIndex for Idx<2953> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2953> as NatIndex>::Out as Nat>::VAL == 2953);
impl NatIndex for Idx<2954> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2954> as NatIndex>::Out as Nat>::VAL == 2954);
impl NatIndex for Idx<2955> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2955> as NatIndex>::Out as Nat>::VAL == 2955);
impl NatIndex for Idx<2956> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2956> as NatIndex>::Out as Nat>::VAL == 2956);
impl NatIndex for Idx<2957> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2957> as NatIndex>::Out as Nat>::VAL == 2957);
impl NatIndex for Idx<2958> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2958> as NatIndex>::Out as Nat>::VAL == 2958);
impl NatIndex for Idx<2959> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2959> as NatIndex>::Out as Nat>::VAL == 2959);
impl NatIndex for Idx<2960> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2960> as NatIndex>::Out as Nat>::VAL == 2960);
impl NatIndex for Idx<2961> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2961> as NatIndex>::Out as Nat>::VAL == 2961);
impl NatIndex for Idx<2962> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2962> as NatIndex>::Out as Nat>::VAL == 2962);
impl NatIndex for Idx<2963> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2963> as NatIndex>::Out as Nat>::VAL == 2963);
impl NatIndex for Idx<2964> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2964> as NatIndex>::Out as Nat>::VAL == 2964);
impl NatIndex for Idx<2965> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2965> as NatIndex>::Out as Nat>::VAL == 2965);
impl NatIndex for Idx<2966> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2966> as NatIndex>::Out as Nat>::VAL == 2966);
impl NatIndex for Idx<2967> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2967> as NatIndex>::Out as Nat>::VAL == 2967);
impl NatIndex for Idx<2968> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2968> as NatIndex>::Out as Nat>::VAL == 2968);
impl NatIndex for Idx<2969> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2969> as NatIndex>::Out as Nat>::VAL == 2969);
impl NatIndex for Idx<2970> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2970> as NatIndex>::Out as Nat>::VAL == 2970);
impl NatIndex for Idx<2971> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2971> as NatIndex>::Out as Nat>::VAL == 2971);
impl NatIndex for Idx<2972> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2972> as NatIndex>::Out as Nat>::VAL == 2972);
impl NatIndex for Idx<2973> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2973> as NatIndex>::Out as Nat>::VAL == 2973);
impl NatIndex for Idx<2974> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2974> as NatIndex>::Out as Nat>::VAL == 2974);
impl NatIndex for Idx<2975> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2975> as NatIndex>::Out as Nat>::VAL == 2975);
impl NatIndex for Idx<2976> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2976> as NatIndex>::Out as Nat>::VAL == 2976);
impl NatIndex for Idx<2977> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2977> as NatIndex>::Out as Nat>::VAL == 2977);
impl NatIndex for Idx<2978> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2978> as NatIndex>::Out as Nat>::VAL == 2978);
impl NatIndex for Idx<2979> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2979> as NatIndex>::Out as Nat>::VAL == 2979);
impl NatIndex for Idx<2980> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2980> as NatIndex>::Out as Nat>::VAL == 2980);
impl NatIndex for Idx<2981> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2981> as NatIndex>::Out as Nat>::VAL == 2981);
impl NatIndex for Idx<2982> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2982> as NatIndex>::Out as Nat>::VAL == 2982);
impl NatIndex for Idx<2983> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2983> as NatIndex>::Out as Nat>::VAL == 2983);
impl NatIndex for Idx<2984> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2984> as NatIndex>::Out as Nat>::VAL == 2984);
impl NatIndex for Idx<2985> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2985> as NatIndex>::Out as Nat>::VAL == 2985);
impl NatIndex for Idx<2986> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2986> as NatIndex>::Out as Nat>::VAL == 2986);
impl NatIndex for Idx<2987> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2987> as NatIndex>::Out as Nat>::VAL == 2987);
impl NatIndex for Idx<2988> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2988> as NatIndex>::Out as Nat>::VAL == 2988);
impl NatIndex for Idx<2989> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2989> as NatIndex>::Out as Nat>::VAL == 2989);
impl NatIndex for Idx<2990> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2990> as NatIndex>::Out as Nat>::VAL == 2990);
impl NatIndex for Idx<2991> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2991> as NatIndex>::Out as Nat>::VAL == 2991);
impl NatIndex for Idx<2992> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2992> as NatIndex>::Out as Nat>::VAL == 2992);
impl NatIndex for Idx<2993> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2993> as NatIndex>::Out as Nat>::VAL == 2993);
impl NatIndex for Idx<2994> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2994> as NatIndex>::Out as Nat>::VAL == 2994);
impl NatIndex for Idx<2995> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2995> as NatIndex>::Out as Nat>::VAL == 2995);
impl NatIndex for Idx<2996> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2996> as NatIndex>::Out as Nat>::VAL == 2996);
impl NatIndex for Idx<2997> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2997> as NatIndex>::Out as Nat>::VAL == 2997);
impl NatIndex for Idx<2998> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2998> as NatIndex>::Out as Nat>::VAL == 2998);
impl NatIndex for Idx<2999> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<2999> as NatIndex>::Out as Nat>::VAL == 2999);
impl NatIndex for Idx<3000> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3000> as NatIndex>::Out as Nat>::VAL == 3000);
impl NatIndex for Idx<3001> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3001> as NatIndex>::Out as Nat>::VAL == 3001);
impl NatIndex for Idx<3002> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3002> as NatIndex>::Out as Nat>::VAL == 3002);
impl NatIndex for Idx<3003> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3003> as NatIndex>::Out as Nat>::VAL == 3003);
impl NatIndex for Idx<3004> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3004> as NatIndex>::Out as Nat>::VAL == 3004);
impl NatIndex for Idx<3005> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3005> as NatIndex>::Out as Nat>::VAL == 3005);
impl NatIndex for Idx<3006> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3006> as NatIndex>::Out as Nat>::VAL == 3006);
impl NatIndex for Idx<3007> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3007> as NatIndex>::Out as Nat>::VAL == 3007);
impl NatIndex for Idx<3008> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3008> as NatIndex>::Out as Nat>::VAL == 3008);
impl NatIndex for Idx<3009> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3009> as NatIndex>::Out as Nat>::VAL == 3009);
impl NatIndex for Idx<3010> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3010> as NatIndex>::Out as Nat>::VAL == 3010);
impl NatIndex for Idx<3011> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3011> as NatIndex>::Out as Nat>::VAL == 3011);
impl NatIndex for Idx<3012> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3012> as NatIndex>::Out as Nat>::VAL == 3012);
impl NatIndex for Idx<3013> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3013> as NatIndex>::Out as Nat>::VAL == 3013);
impl NatIndex for Idx<3014> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3014> as NatIndex>::Out as Nat>::VAL == 3014);
impl NatIndex for Idx<3015> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3015> as NatIndex>::Out as Nat>::VAL == 3015);
impl NatIndex for Idx<3016> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3016> as NatIndex>::Out as Nat>::VAL == 3016);
impl NatIndex for Idx<3017> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3017> as NatIndex>::Out as Nat>::VAL == 3017);
impl NatIndex for Idx<3018> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3018> as NatIndex>::Out as Nat>::VAL == 3018);
impl NatIndex for Idx<3019> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3019> as NatIndex>::Out as Nat>::VAL == 3019);
impl NatIndex for Idx<3020> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3020> as NatIndex>::Out as Nat>::VAL == 3020);
impl NatIndex for Idx<3021> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3021> as NatIndex>::Out as Nat>::VAL == 3021);
impl NatIndex for Idx<3022> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3022> as NatIndex>::Out as Nat>::VAL == 3022);
impl NatIndex for Idx<3023> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3023> as NatIndex>::Out as Nat>::VAL == 3023);
impl NatIndex for Idx<3024> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3024> as NatIndex>::Out as Nat>::VAL == 3024);
impl NatIndex for Idx<3025> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3025> as NatIndex>::Out as Nat>::VAL == 3025);
impl NatIndex for Idx<3026> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3026> as NatIndex>::Out as Nat>::VAL == 3026);
impl NatIndex for Idx<3027> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3027> as NatIndex>::Out as Nat>::VAL == 3027);
impl NatIndex for Idx<3028> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3028> as NatIndex>::Out as Nat>::VAL == 3028);
impl NatIndex for Idx<3029> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3029> as NatIndex>::Out as Nat>::VAL == 3029);
impl NatIndex for Idx<3030> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3030> as NatIndex>::Out as Nat>::VAL == 3030);
impl NatIndex for Idx<3031> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3031> as NatIndex>::Out as Nat>::VAL == 3031);
impl NatIndex for Idx<3032> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3032> as NatIndex>::Out as Nat>::VAL == 3032);
impl NatIndex for Idx<3033> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3033> as NatIndex>::Out as Nat>::VAL == 3033);
impl NatIndex for Idx<3034> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3034> as NatIndex>::Out as Nat>::VAL == 3034);
impl NatIndex for Idx<3035> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3035> as NatIndex>::Out as Nat>::VAL == 3035);
impl NatIndex for Idx<3036> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3036> as NatIndex>::Out as Nat>::VAL == 3036);
impl NatIndex for Idx<3037> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3037> as NatIndex>::Out as Nat>::VAL == 3037);
impl NatIndex for Idx<3038> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3038> as NatIndex>::Out as Nat>::VAL == 3038);
impl NatIndex for Idx<3039> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3039> as NatIndex>::Out as Nat>::VAL == 3039);
impl NatIndex for Idx<3040> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3040> as NatIndex>::Out as Nat>::VAL == 3040);
impl NatIndex for Idx<3041> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3041> as NatIndex>::Out as Nat>::VAL == 3041);
impl NatIndex for Idx<3042> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3042> as NatIndex>::Out as Nat>::VAL == 3042);
impl NatIndex for Idx<3043> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3043> as NatIndex>::Out as Nat>::VAL == 3043);
impl NatIndex for Idx<3044> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3044> as NatIndex>::Out as Nat>::VAL == 3044);
impl NatIndex for Idx<3045> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3045> as NatIndex>::Out as Nat>::VAL == 3045);
impl NatIndex for Idx<3046> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3046> as NatIndex>::Out as Nat>::VAL == 3046);
impl NatIndex for Idx<3047> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3047> as NatIndex>::Out as Nat>::VAL == 3047);
impl NatIndex for Idx<3048> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3048> as NatIndex>::Out as Nat>::VAL == 3048);
impl NatIndex for Idx<3049> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3049> as NatIndex>::Out as Nat>::VAL == 3049);
impl NatIndex for Idx<3050> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3050> as NatIndex>::Out as Nat>::VAL == 3050);
impl NatIndex for Idx<3051> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3051> as NatIndex>::Out as Nat>::VAL == 3051);
impl NatIndex for Idx<3052> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3052> as NatIndex>::Out as Nat>::VAL == 3052);
impl NatIndex for Idx<3053> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3053> as NatIndex>::Out as Nat>::VAL == 3053);
impl NatIndex for Idx<3054> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3054> as NatIndex>::Out as Nat>::VAL == 3054);
impl NatIndex for Idx<3055> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3055> as NatIndex>::Out as Nat>::VAL == 3055);
impl NatIndex for Idx<3056> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3056> as NatIndex>::Out as Nat>::VAL == 3056);
impl NatIndex for Idx<3057> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3057> as NatIndex>::Out as Nat>::VAL == 3057);
impl NatIndex for Idx<3058> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3058> as NatIndex>::Out as Nat>::VAL == 3058);
impl NatIndex for Idx<3059> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3059> as NatIndex>::Out as Nat>::VAL == 3059);
impl NatIndex for Idx<3060> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3060> as NatIndex>::Out as Nat>::VAL == 3060);
impl NatIndex for Idx<3061> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3061> as NatIndex>::Out as Nat>::VAL == 3061);
impl NatIndex for Idx<3062> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3062> as NatIndex>::Out as Nat>::VAL == 3062);
impl NatIndex for Idx<3063> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3063> as NatIndex>::Out as Nat>::VAL == 3063);
impl NatIndex for Idx<3064> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3064> as NatIndex>::Out as Nat>::VAL == 3064);
impl NatIndex for Idx<3065> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3065> as NatIndex>::Out as Nat>::VAL == 3065);
impl NatIndex for Idx<3066> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3066> as NatIndex>::Out as Nat>::VAL == 3066);
impl NatIndex for Idx<3067> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3067> as NatIndex>::Out as Nat>::VAL == 3067);
impl NatIndex for Idx<3068> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3068> as NatIndex>::Out as Nat>::VAL == 3068);
impl NatIndex for Idx<3069> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3069> as NatIndex>::Out as Nat>::VAL == 3069);
impl NatIndex for Idx<3070> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3070> as NatIndex>::Out as Nat>::VAL == 3070);
impl NatIndex for Idx<3071> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3071> as NatIndex>::Out as Nat>::VAL == 3071);
impl NatIndex for Idx<3072> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3072> as NatIndex>::Out as Nat>::VAL == 3072);
impl NatIndex for Idx<3073> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3073> as NatIndex>::Out as Nat>::VAL == 3073);
impl NatIndex for Idx<3074> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3074> as NatIndex>::Out as Nat>::VAL == 3074);
impl NatIndex for Idx<3075> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3075> as NatIndex>::Out as Nat>::VAL == 3075);
impl NatIndex for Idx<3076> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3076> as NatIndex>::Out as Nat>::VAL == 3076);
impl NatIndex for Idx<3077> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3077> as NatIndex>::Out as Nat>::VAL == 3077);
impl NatIndex for Idx<3078> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3078> as NatIndex>::Out as Nat>::VAL == 3078);
impl NatIndex for Idx<3079> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3079> as NatIndex>::Out as Nat>::VAL == 3079);
impl NatIndex for Idx<3080> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3080> as NatIndex>::Out as Nat>::VAL == 3080);
impl NatIndex for Idx<3081> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3081> as NatIndex>::Out as Nat>::VAL == 3081);
impl NatIndex for Idx<3082> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3082> as NatIndex>::Out as Nat>::VAL == 3082);
impl NatIndex for Idx<3083> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3083> as NatIndex>::Out as Nat>::VAL == 3083);
impl NatIndex for Idx<3084> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3084> as NatIndex>::Out as Nat>::VAL == 3084);
impl NatIndex for Idx<3085> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3085> as NatIndex>::Out as Nat>::VAL == 3085);
impl NatIndex for Idx<3086> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3086> as NatIndex>::Out as Nat>::VAL == 3086);
impl NatIndex for Idx<3087> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3087> as NatIndex>::Out as Nat>::VAL == 3087);
impl NatIndex for Idx<3088> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3088> as NatIndex>::Out as Nat>::VAL == 3088);
impl NatIndex for Idx<3089> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3089> as NatIndex>::Out as Nat>::VAL == 3089);
impl NatIndex for Idx<3090> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3090> as NatIndex>::Out as Nat>::VAL == 3090);
impl NatIndex for Idx<3091> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3091> as NatIndex>::Out as Nat>::VAL == 3091);
impl NatIndex for Idx<3092> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3092> as NatIndex>::Out as Nat>::VAL == 3092);
impl NatIndex for Idx<3093> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3093> as NatIndex>::Out as Nat>::VAL == 3093);
impl NatIndex for Idx<3094> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3094> as NatIndex>::Out as Nat>::VAL == 3094);
impl NatIndex for Idx<3095> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3095> as NatIndex>::Out as Nat>::VAL == 3095);
impl NatIndex for Idx<3096> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3096> as NatIndex>::Out as Nat>::VAL == 3096);
impl NatIndex for Idx<3097> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3097> as NatIndex>::Out as Nat>::VAL == 3097);
impl NatIndex for Idx<3098> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3098> as NatIndex>::Out as Nat>::VAL == 3098);
impl NatIndex for Idx<3099> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3099> as NatIndex>::Out as Nat>::VAL == 3099);
impl NatIndex for Idx<3100> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3100> as NatIndex>::Out as Nat>::VAL == 3100);
impl NatIndex for Idx<3101> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3101> as NatIndex>::Out as Nat>::VAL == 3101);
impl NatIndex for Idx<3102> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3102> as NatIndex>::Out as Nat>::VAL == 3102);
impl NatIndex for Idx<3103> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3103> as NatIndex>::Out as Nat>::VAL == 3103);
impl NatIndex for Idx<3104> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3104> as NatIndex>::Out as Nat>::VAL == 3104);
impl NatIndex for Idx<3105> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3105> as NatIndex>::Out as Nat>::VAL == 3105);
impl NatIndex for Idx<3106> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3106> as NatIndex>::Out as Nat>::VAL == 3106);
impl NatIndex for Idx<3107> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3107> as NatIndex>::Out as Nat>::VAL == 3107);
impl NatIndex for Idx<3108> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3108> as NatIndex>::Out as Nat>::VAL == 3108);
impl NatIndex for Idx<3109> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3109> as NatIndex>::Out as Nat>::VAL == 3109);
impl NatIndex for Idx<3110> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3110> as NatIndex>::Out as Nat>::VAL == 3110);
impl NatIndex for Idx<3111> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3111> as NatIndex>::Out as Nat>::VAL == 3111);
impl NatIndex for Idx<3112> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3112> as NatIndex>::Out as Nat>::VAL == 3112);
impl NatIndex for Idx<3113> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3113> as NatIndex>::Out as Nat>::VAL == 3113);
impl NatIndex for Idx<3114> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3114> as NatIndex>::Out as Nat>::VAL == 3114);
impl NatIndex for Idx<3115> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3115> as NatIndex>::Out as Nat>::VAL == 3115);
impl NatIndex for Idx<3116> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3116> as NatIndex>::Out as Nat>::VAL == 3116);
impl NatIndex for Idx<3117> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3117> as NatIndex>::Out as Nat>::VAL == 3117);
impl NatIndex for Idx<3118> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3118> as NatIndex>::Out as Nat>::VAL == 3118);
impl NatIndex for Idx<3119> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3119> as NatIndex>::Out as Nat>::VAL == 3119);
impl NatIndex for Idx<3120> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3120> as NatIndex>::Out as Nat>::VAL == 3120);
impl NatIndex for Idx<3121> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3121> as NatIndex>::Out as Nat>::VAL == 3121);
impl NatIndex for Idx<3122> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3122> as NatIndex>::Out as Nat>::VAL == 3122);
impl NatIndex for Idx<3123> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3123> as NatIndex>::Out as Nat>::VAL == 3123);
impl NatIndex for Idx<3124> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3124> as NatIndex>::Out as Nat>::VAL == 3124);
impl NatIndex for Idx<3125> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3125> as NatIndex>::Out as Nat>::VAL == 3125);
impl NatIndex for Idx<3126> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3126> as NatIndex>::Out as Nat>::VAL == 3126);
impl NatIndex for Idx<3127> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3127> as NatIndex>::Out as Nat>::VAL == 3127);
impl NatIndex for Idx<3128> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3128> as NatIndex>::Out as Nat>::VAL == 3128);
impl NatIndex for Idx<3129> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3129> as NatIndex>::Out as Nat>::VAL == 3129);
impl NatIndex for Idx<3130> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3130> as NatIndex>::Out as Nat>::VAL == 3130);
impl NatIndex for Idx<3131> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3131> as NatIndex>::Out as Nat>::VAL == 3131);
impl NatIndex for Idx<3132> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3132> as NatIndex>::Out as Nat>::VAL == 3132);
impl NatIndex for Idx<3133> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3133> as NatIndex>::Out as Nat>::VAL == 3133);
impl NatIndex for Idx<3134> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3134> as NatIndex>::Out as Nat>::VAL == 3134);
impl NatIndex for Idx<3135> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3135> as NatIndex>::Out as Nat>::VAL == 3135);
impl NatIndex for Idx<3136> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3136> as NatIndex>::Out as Nat>::VAL == 3136);
impl NatIndex for Idx<3137> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3137> as NatIndex>::Out as Nat>::VAL == 3137);
impl NatIndex for Idx<3138> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3138> as NatIndex>::Out as Nat>::VAL == 3138);
impl NatIndex for Idx<3139> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3139> as NatIndex>::Out as Nat>::VAL == 3139);
impl NatIndex for Idx<3140> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3140> as NatIndex>::Out as Nat>::VAL == 3140);
impl NatIndex for Idx<3141> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3141> as NatIndex>::Out as Nat>::VAL == 3141);
impl NatIndex for Idx<3142> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3142> as NatIndex>::Out as Nat>::VAL == 3142);
impl NatIndex for Idx<3143> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3143> as NatIndex>::Out as Nat>::VAL == 3143);
impl NatIndex for Idx<3144> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3144> as NatIndex>::Out as Nat>::VAL == 3144);
impl NatIndex for Idx<3145> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3145> as NatIndex>::Out as Nat>::VAL == 3145);
impl NatIndex for Idx<3146> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3146> as NatIndex>::Out as Nat>::VAL == 3146);
impl NatIndex for Idx<3147> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3147> as NatIndex>::Out as Nat>::VAL == 3147);
impl NatIndex for Idx<3148> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3148> as NatIndex>::Out as Nat>::VAL == 3148);
impl NatIndex for Idx<3149> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3149> as NatIndex>::Out as Nat>::VAL == 3149);
impl NatIndex for Idx<3150> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3150> as NatIndex>::Out as Nat>::VAL == 3150);
impl NatIndex for Idx<3151> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3151> as NatIndex>::Out as Nat>::VAL == 3151);
impl NatIndex for Idx<3152> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3152> as NatIndex>::Out as Nat>::VAL == 3152);
impl NatIndex for Idx<3153> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3153> as NatIndex>::Out as Nat>::VAL == 3153);
impl NatIndex for Idx<3154> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3154> as NatIndex>::Out as Nat>::VAL == 3154);
impl NatIndex for Idx<3155> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3155> as NatIndex>::Out as Nat>::VAL == 3155);
impl NatIndex for Idx<3156> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3156> as NatIndex>::Out as Nat>::VAL == 3156);
impl NatIndex for Idx<3157> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3157> as NatIndex>::Out as Nat>::VAL == 3157);
impl NatIndex for Idx<3158> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3158> as NatIndex>::Out as Nat>::VAL == 3158);
impl NatIndex for Idx<3159> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3159> as NatIndex>::Out as Nat>::VAL == 3159);
impl NatIndex for Idx<3160> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3160> as NatIndex>::Out as Nat>::VAL == 3160);
impl NatIndex for Idx<3161> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3161> as NatIndex>::Out as Nat>::VAL == 3161);
impl NatIndex for Idx<3162> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3162> as NatIndex>::Out as Nat>::VAL == 3162);
impl NatIndex for Idx<3163> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3163> as NatIndex>::Out as Nat>::VAL == 3163);
impl NatIndex for Idx<3164> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3164> as NatIndex>::Out as Nat>::VAL == 3164);
impl NatIndex for Idx<3165> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3165> as NatIndex>::Out as Nat>::VAL == 3165);
impl NatIndex for Idx<3166> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3166> as NatIndex>::Out as Nat>::VAL == 3166);
impl NatIndex for Idx<3167> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3167> as NatIndex>::Out as Nat>::VAL == 3167);
impl NatIndex for Idx<3168> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3168> as NatIndex>::Out as Nat>::VAL == 3168);
impl NatIndex for Idx<3169> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3169> as NatIndex>::Out as Nat>::VAL == 3169);
impl NatIndex for Idx<3170> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3170> as NatIndex>::Out as Nat>::VAL == 3170);
impl NatIndex for Idx<3171> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3171> as NatIndex>::Out as Nat>::VAL == 3171);
impl NatIndex for Idx<3172> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3172> as NatIndex>::Out as Nat>::VAL == 3172);
impl NatIndex for Idx<3173> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3173> as NatIndex>::Out as Nat>::VAL == 3173);
impl NatIndex for Idx<3174> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3174> as NatIndex>::Out as Nat>::VAL == 3174);
impl NatIndex for Idx<3175> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3175> as NatIndex>::Out as Nat>::VAL == 3175);
impl NatIndex for Idx<3176> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3176> as NatIndex>::Out as Nat>::VAL == 3176);
impl NatIndex for Idx<3177> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3177> as NatIndex>::Out as Nat>::VAL == 3177);
impl NatIndex for Idx<3178> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3178> as NatIndex>::Out as Nat>::VAL == 3178);
impl NatIndex for Idx<3179> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3179> as NatIndex>::Out as Nat>::VAL == 3179);
impl NatIndex for Idx<3180> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3180> as NatIndex>::Out as Nat>::VAL == 3180);
impl NatIndex for Idx<3181> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3181> as NatIndex>::Out as Nat>::VAL == 3181);
impl NatIndex for Idx<3182> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3182> as NatIndex>::Out as Nat>::VAL == 3182);
impl NatIndex for Idx<3183> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3183> as NatIndex>::Out as Nat>::VAL == 3183);
impl NatIndex for Idx<3184> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3184> as NatIndex>::Out as Nat>::VAL == 3184);
impl NatIndex for Idx<3185> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3185> as NatIndex>::Out as Nat>::VAL == 3185);
impl NatIndex for Idx<3186> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3186> as NatIndex>::Out as Nat>::VAL == 3186);
impl NatIndex for Idx<3187> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3187> as NatIndex>::Out as Nat>::VAL == 3187);
impl NatIndex for Idx<3188> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3188> as NatIndex>::Out as Nat>::VAL == 3188);
impl NatIndex for Idx<3189> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3189> as NatIndex>::Out as Nat>::VAL == 3189);
impl NatIndex for Idx<3190> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3190> as NatIndex>::Out as Nat>::VAL == 3190);
impl NatIndex for Idx<3191> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3191> as NatIndex>::Out as Nat>::VAL == 3191);
impl NatIndex for Idx<3192> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3192> as NatIndex>::Out as Nat>::VAL == 3192);
impl NatIndex for Idx<3193> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3193> as NatIndex>::Out as Nat>::VAL == 3193);
impl NatIndex for Idx<3194> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3194> as NatIndex>::Out as Nat>::VAL == 3194);
impl NatIndex for Idx<3195> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3195> as NatIndex>::Out as Nat>::VAL == 3195);
impl NatIndex for Idx<3196> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3196> as NatIndex>::Out as Nat>::VAL == 3196);
impl NatIndex for Idx<3197> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3197> as NatIndex>::Out as Nat>::VAL == 3197);
impl NatIndex for Idx<3198> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3198> as NatIndex>::Out as Nat>::VAL == 3198);
impl NatIndex for Idx<3199> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3199> as NatIndex>::Out as Nat>::VAL == 3199);
impl NatIndex for Idx<3200> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3200> as NatIndex>::Out as Nat>::VAL == 3200);
impl NatIndex for Idx<3201> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3201> as NatIndex>::Out as Nat>::VAL == 3201);
impl NatIndex for Idx<3202> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3202> as NatIndex>::Out as Nat>::VAL == 3202);
impl NatIndex for Idx<3203> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3203> as NatIndex>::Out as Nat>::VAL == 3203);
impl NatIndex for Idx<3204> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3204> as NatIndex>::Out as Nat>::VAL == 3204);
impl NatIndex for Idx<3205> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3205> as NatIndex>::Out as Nat>::VAL == 3205);
impl NatIndex for Idx<3206> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3206> as NatIndex>::Out as Nat>::VAL == 3206);
impl NatIndex for Idx<3207> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3207> as NatIndex>::Out as Nat>::VAL == 3207);
impl NatIndex for Idx<3208> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3208> as NatIndex>::Out as Nat>::VAL == 3208);
impl NatIndex for Idx<3209> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3209> as NatIndex>::Out as Nat>::VAL == 3209);
impl NatIndex for Idx<3210> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3210> as NatIndex>::Out as Nat>::VAL == 3210);
impl NatIndex for Idx<3211> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3211> as NatIndex>::Out as Nat>::VAL == 3211);
impl NatIndex for Idx<3212> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3212> as NatIndex>::Out as Nat>::VAL == 3212);
impl NatIndex for Idx<3213> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3213> as NatIndex>::Out as Nat>::VAL == 3213);
impl NatIndex for Idx<3214> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3214> as NatIndex>::Out as Nat>::VAL == 3214);
impl NatIndex for Idx<3215> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3215> as NatIndex>::Out as Nat>::VAL == 3215);
impl NatIndex for Idx<3216> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3216> as NatIndex>::Out as Nat>::VAL == 3216);
impl NatIndex for Idx<3217> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3217> as NatIndex>::Out as Nat>::VAL == 3217);
impl NatIndex for Idx<3218> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3218> as NatIndex>::Out as Nat>::VAL == 3218);
impl NatIndex for Idx<3219> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3219> as NatIndex>::Out as Nat>::VAL == 3219);
impl NatIndex for Idx<3220> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3220> as NatIndex>::Out as Nat>::VAL == 3220);
impl NatIndex for Idx<3221> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3221> as NatIndex>::Out as Nat>::VAL == 3221);
impl NatIndex for Idx<3222> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3222> as NatIndex>::Out as Nat>::VAL == 3222);
impl NatIndex for Idx<3223> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3223> as NatIndex>::Out as Nat>::VAL == 3223);
impl NatIndex for Idx<3224> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3224> as NatIndex>::Out as Nat>::VAL == 3224);
impl NatIndex for Idx<3225> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3225> as NatIndex>::Out as Nat>::VAL == 3225);
impl NatIndex for Idx<3226> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3226> as NatIndex>::Out as Nat>::VAL == 3226);
impl NatIndex for Idx<3227> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3227> as NatIndex>::Out as Nat>::VAL == 3227);
impl NatIndex for Idx<3228> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3228> as NatIndex>::Out as Nat>::VAL == 3228);
impl NatIndex for Idx<3229> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3229> as NatIndex>::Out as Nat>::VAL == 3229);
impl NatIndex for Idx<3230> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3230> as NatIndex>::Out as Nat>::VAL == 3230);
impl NatIndex for Idx<3231> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3231> as NatIndex>::Out as Nat>::VAL == 3231);
impl NatIndex for Idx<3232> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3232> as NatIndex>::Out as Nat>::VAL == 3232);
impl NatIndex for Idx<3233> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3233> as NatIndex>::Out as Nat>::VAL == 3233);
impl NatIndex for Idx<3234> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3234> as NatIndex>::Out as Nat>::VAL == 3234);
impl NatIndex for Idx<3235> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3235> as NatIndex>::Out as Nat>::VAL == 3235);
impl NatIndex for Idx<3236> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3236> as NatIndex>::Out as Nat>::VAL == 3236);
impl NatIndex for Idx<3237> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3237> as NatIndex>::Out as Nat>::VAL == 3237);
impl NatIndex for Idx<3238> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3238> as NatIndex>::Out as Nat>::VAL == 3238);
impl NatIndex for Idx<3239> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3239> as NatIndex>::Out as Nat>::VAL == 3239);
impl NatIndex for Idx<3240> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3240> as NatIndex>::Out as Nat>::VAL == 3240);
impl NatIndex for Idx<3241> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3241> as NatIndex>::Out as Nat>::VAL == 3241);
impl NatIndex for Idx<3242> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3242> as NatIndex>::Out as Nat>::VAL == 3242);
impl NatIndex for Idx<3243> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3243> as NatIndex>::Out as Nat>::VAL == 3243);
impl NatIndex for Idx<3244> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3244> as NatIndex>::Out as Nat>::VAL == 3244);
impl NatIndex for Idx<3245> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3245> as NatIndex>::Out as Nat>::VAL == 3245);
impl NatIndex for Idx<3246> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3246> as NatIndex>::Out as Nat>::VAL == 3246);
impl NatIndex for Idx<3247> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3247> as NatIndex>::Out as Nat>::VAL == 3247);
impl NatIndex for Idx<3248> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3248> as NatIndex>::Out as Nat>::VAL == 3248);
impl NatIndex for Idx<3249> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3249> as NatIndex>::Out as Nat>::VAL == 3249);
impl NatIndex for Idx<3250> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3250> as NatIndex>::Out as Nat>::VAL == 3250);
impl NatIndex for Idx<3251> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3251> as NatIndex>::Out as Nat>::VAL == 3251);
impl NatIndex for Idx<3252> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3252> as NatIndex>::Out as Nat>::VAL == 3252);
impl NatIndex for Idx<3253> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3253> as NatIndex>::Out as Nat>::VAL == 3253);
impl NatIndex for Idx<3254> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3254> as NatIndex>::Out as Nat>::VAL == 3254);
impl NatIndex for Idx<3255> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3255> as NatIndex>::Out as Nat>::VAL == 3255);
impl NatIndex for Idx<3256> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3256> as NatIndex>::Out as Nat>::VAL == 3256);
impl NatIndex for Idx<3257> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3257> as NatIndex>::Out as Nat>::VAL == 3257);
impl NatIndex for Idx<3258> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3258> as NatIndex>::Out as Nat>::VAL == 3258);
impl NatIndex for Idx<3259> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3259> as NatIndex>::Out as Nat>::VAL == 3259);
impl NatIndex for Idx<3260> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3260> as NatIndex>::Out as Nat>::VAL == 3260);
impl NatIndex for Idx<3261> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3261> as NatIndex>::Out as Nat>::VAL == 3261);
impl NatIndex for Idx<3262> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3262> as NatIndex>::Out as Nat>::VAL == 3262);
impl NatIndex for Idx<3263> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3263> as NatIndex>::Out as Nat>::VAL == 3263);
impl NatIndex for Idx<3264> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3264> as NatIndex>::Out as Nat>::VAL == 3264);
impl NatIndex for Idx<3265> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3265> as NatIndex>::Out as Nat>::VAL == 3265);
impl NatIndex for Idx<3266> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3266> as NatIndex>::Out as Nat>::VAL == 3266);
impl NatIndex for Idx<3267> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3267> as NatIndex>::Out as Nat>::VAL == 3267);
impl NatIndex for Idx<3268> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3268> as NatIndex>::Out as Nat>::VAL == 3268);
impl NatIndex for Idx<3269> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3269> as NatIndex>::Out as Nat>::VAL == 3269);
impl NatIndex for Idx<3270> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3270> as NatIndex>::Out as Nat>::VAL == 3270);
impl NatIndex for Idx<3271> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3271> as NatIndex>::Out as Nat>::VAL == 3271);
impl NatIndex for Idx<3272> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3272> as NatIndex>::Out as Nat>::VAL == 3272);
impl NatIndex for Idx<3273> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3273> as NatIndex>::Out as Nat>::VAL == 3273);
impl NatIndex for Idx<3274> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3274> as NatIndex>::Out as Nat>::VAL == 3274);
impl NatIndex for Idx<3275> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3275> as NatIndex>::Out as Nat>::VAL == 3275);
impl NatIndex for Idx<3276> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3276> as NatIndex>::Out as Nat>::VAL == 3276);
impl NatIndex for Idx<3277> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3277> as NatIndex>::Out as Nat>::VAL == 3277);
impl NatIndex for Idx<3278> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3278> as NatIndex>::Out as Nat>::VAL == 3278);
impl NatIndex for Idx<3279> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3279> as NatIndex>::Out as Nat>::VAL == 3279);
impl NatIndex for Idx<3280> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3280> as NatIndex>::Out as Nat>::VAL == 3280);
impl NatIndex for Idx<3281> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3281> as NatIndex>::Out as Nat>::VAL == 3281);
impl NatIndex for Idx<3282> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3282> as NatIndex>::Out as Nat>::VAL == 3282);
impl NatIndex for Idx<3283> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3283> as NatIndex>::Out as Nat>::VAL == 3283);
impl NatIndex for Idx<3284> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3284> as NatIndex>::Out as Nat>::VAL == 3284);
impl NatIndex for Idx<3285> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3285> as NatIndex>::Out as Nat>::VAL == 3285);
impl NatIndex for Idx<3286> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3286> as NatIndex>::Out as Nat>::VAL == 3286);
impl NatIndex for Idx<3287> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3287> as NatIndex>::Out as Nat>::VAL == 3287);
impl NatIndex for Idx<3288> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3288> as NatIndex>::Out as Nat>::VAL == 3288);
impl NatIndex for Idx<3289> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3289> as NatIndex>::Out as Nat>::VAL == 3289);
impl NatIndex for Idx<3290> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3290> as NatIndex>::Out as Nat>::VAL == 3290);
impl NatIndex for Idx<3291> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3291> as NatIndex>::Out as Nat>::VAL == 3291);
impl NatIndex for Idx<3292> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3292> as NatIndex>::Out as Nat>::VAL == 3292);
impl NatIndex for Idx<3293> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3293> as NatIndex>::Out as Nat>::VAL == 3293);
impl NatIndex for Idx<3294> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3294> as NatIndex>::Out as Nat>::VAL == 3294);
impl NatIndex for Idx<3295> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3295> as NatIndex>::Out as Nat>::VAL == 3295);
impl NatIndex for Idx<3296> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3296> as NatIndex>::Out as Nat>::VAL == 3296);
impl NatIndex for Idx<3297> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3297> as NatIndex>::Out as Nat>::VAL == 3297);
impl NatIndex for Idx<3298> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3298> as NatIndex>::Out as Nat>::VAL == 3298);
impl NatIndex for Idx<3299> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3299> as NatIndex>::Out as Nat>::VAL == 3299);
impl NatIndex for Idx<3300> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3300> as NatIndex>::Out as Nat>::VAL == 3300);
impl NatIndex for Idx<3301> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3301> as NatIndex>::Out as Nat>::VAL == 3301);
impl NatIndex for Idx<3302> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3302> as NatIndex>::Out as Nat>::VAL == 3302);
impl NatIndex for Idx<3303> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3303> as NatIndex>::Out as Nat>::VAL == 3303);
impl NatIndex for Idx<3304> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3304> as NatIndex>::Out as Nat>::VAL == 3304);
impl NatIndex for Idx<3305> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3305> as NatIndex>::Out as Nat>::VAL == 3305);
impl NatIndex for Idx<3306> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3306> as NatIndex>::Out as Nat>::VAL == 3306);
impl NatIndex for Idx<3307> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3307> as NatIndex>::Out as Nat>::VAL == 3307);
impl NatIndex for Idx<3308> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3308> as NatIndex>::Out as Nat>::VAL == 3308);
impl NatIndex for Idx<3309> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3309> as NatIndex>::Out as Nat>::VAL == 3309);
impl NatIndex for Idx<3310> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3310> as NatIndex>::Out as Nat>::VAL == 3310);
impl NatIndex for Idx<3311> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3311> as NatIndex>::Out as Nat>::VAL == 3311);
impl NatIndex for Idx<3312> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3312> as NatIndex>::Out as Nat>::VAL == 3312);
impl NatIndex for Idx<3313> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3313> as NatIndex>::Out as Nat>::VAL == 3313);
impl NatIndex for Idx<3314> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3314> as NatIndex>::Out as Nat>::VAL == 3314);
impl NatIndex for Idx<3315> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3315> as NatIndex>::Out as Nat>::VAL == 3315);
impl NatIndex for Idx<3316> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3316> as NatIndex>::Out as Nat>::VAL == 3316);
impl NatIndex for Idx<3317> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3317> as NatIndex>::Out as Nat>::VAL == 3317);
impl NatIndex for Idx<3318> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3318> as NatIndex>::Out as Nat>::VAL == 3318);
impl NatIndex for Idx<3319> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3319> as NatIndex>::Out as Nat>::VAL == 3319);
impl NatIndex for Idx<3320> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3320> as NatIndex>::Out as Nat>::VAL == 3320);
impl NatIndex for Idx<3321> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3321> as NatIndex>::Out as Nat>::VAL == 3321);
impl NatIndex for Idx<3322> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3322> as NatIndex>::Out as Nat>::VAL == 3322);
impl NatIndex for Idx<3323> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3323> as NatIndex>::Out as Nat>::VAL == 3323);
impl NatIndex for Idx<3324> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3324> as NatIndex>::Out as Nat>::VAL == 3324);
impl NatIndex for Idx<3325> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3325> as NatIndex>::Out as Nat>::VAL == 3325);
impl NatIndex for Idx<3326> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3326> as NatIndex>::Out as Nat>::VAL == 3326);
impl NatIndex for Idx<3327> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3327> as NatIndex>::Out as Nat>::VAL == 3327);
impl NatIndex for Idx<3328> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3328> as NatIndex>::Out as Nat>::VAL == 3328);
impl NatIndex for Idx<3329> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3329> as NatIndex>::Out as Nat>::VAL == 3329);
impl NatIndex for Idx<3330> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3330> as NatIndex>::Out as Nat>::VAL == 3330);
impl NatIndex for Idx<3331> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3331> as NatIndex>::Out as Nat>::VAL == 3331);
impl NatIndex for Idx<3332> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3332> as NatIndex>::Out as Nat>::VAL == 3332);
impl NatIndex for Idx<3333> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3333> as NatIndex>::Out as Nat>::VAL == 3333);
impl NatIndex for Idx<3334> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3334> as NatIndex>::Out as Nat>::VAL == 3334);
impl NatIndex for Idx<3335> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3335> as NatIndex>::Out as Nat>::VAL == 3335);
impl NatIndex for Idx<3336> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3336> as NatIndex>::Out as Nat>::VAL == 3336);
impl NatIndex for Idx<3337> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3337> as NatIndex>::Out as Nat>::VAL == 3337);
impl NatIndex for Idx<3338> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3338> as NatIndex>::Out as Nat>::VAL == 3338);
impl NatIndex for Idx<3339> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3339> as NatIndex>::Out as Nat>::VAL == 3339);
impl NatIndex for Idx<3340> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3340> as NatIndex>::Out as Nat>::VAL == 3340);
impl NatIndex for Idx<3341> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3341> as NatIndex>::Out as Nat>::VAL == 3341);
impl NatIndex for Idx<3342> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3342> as NatIndex>::Out as Nat>::VAL == 3342);
impl NatIndex for Idx<3343> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3343> as NatIndex>::Out as Nat>::VAL == 3343);
impl NatIndex for Idx<3344> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3344> as NatIndex>::Out as Nat>::VAL == 3344);
impl NatIndex for Idx<3345> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3345> as NatIndex>::Out as Nat>::VAL == 3345);
impl NatIndex for Idx<3346> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3346> as NatIndex>::Out as Nat>::VAL == 3346);
impl NatIndex for Idx<3347> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3347> as NatIndex>::Out as Nat>::VAL == 3347);
impl NatIndex for Idx<3348> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3348> as NatIndex>::Out as Nat>::VAL == 3348);
impl NatIndex for Idx<3349> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3349> as NatIndex>::Out as Nat>::VAL == 3349);
impl NatIndex for Idx<3350> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3350> as NatIndex>::Out as Nat>::VAL == 3350);
impl NatIndex for Idx<3351> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3351> as NatIndex>::Out as Nat>::VAL == 3351);
impl NatIndex for Idx<3352> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3352> as NatIndex>::Out as Nat>::VAL == 3352);
impl NatIndex for Idx<3353> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3353> as NatIndex>::Out as Nat>::VAL == 3353);
impl NatIndex for Idx<3354> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3354> as NatIndex>::Out as Nat>::VAL == 3354);
impl NatIndex for Idx<3355> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3355> as NatIndex>::Out as Nat>::VAL == 3355);
impl NatIndex for Idx<3356> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3356> as NatIndex>::Out as Nat>::VAL == 3356);
impl NatIndex for Idx<3357> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3357> as NatIndex>::Out as Nat>::VAL == 3357);
impl NatIndex for Idx<3358> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3358> as NatIndex>::Out as Nat>::VAL == 3358);
impl NatIndex for Idx<3359> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3359> as NatIndex>::Out as Nat>::VAL == 3359);
impl NatIndex for Idx<3360> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3360> as NatIndex>::Out as Nat>::VAL == 3360);
impl NatIndex for Idx<3361> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3361> as NatIndex>::Out as Nat>::VAL == 3361);
impl NatIndex for Idx<3362> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3362> as NatIndex>::Out as Nat>::VAL == 3362);
impl NatIndex for Idx<3363> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3363> as NatIndex>::Out as Nat>::VAL == 3363);
impl NatIndex for Idx<3364> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3364> as NatIndex>::Out as Nat>::VAL == 3364);
impl NatIndex for Idx<3365> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3365> as NatIndex>::Out as Nat>::VAL == 3365);
impl NatIndex for Idx<3366> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3366> as NatIndex>::Out as Nat>::VAL == 3366);
impl NatIndex for Idx<3367> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3367> as NatIndex>::Out as Nat>::VAL == 3367);
impl NatIndex for Idx<3368> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3368> as NatIndex>::Out as Nat>::VAL == 3368);
impl NatIndex for Idx<3369> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3369> as NatIndex>::Out as Nat>::VAL == 3369);
impl NatIndex for Idx<3370> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3370> as NatIndex>::Out as Nat>::VAL == 3370);
impl NatIndex for Idx<3371> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3371> as NatIndex>::Out as Nat>::VAL == 3371);
impl NatIndex for Idx<3372> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3372> as NatIndex>::Out as Nat>::VAL == 3372);
impl NatIndex for Idx<3373> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3373> as NatIndex>::Out as Nat>::VAL == 3373);
impl NatIndex for Idx<3374> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3374> as NatIndex>::Out as Nat>::VAL == 3374);
impl NatIndex for Idx<3375> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3375> as NatIndex>::Out as Nat>::VAL == 3375);
impl NatIndex for Idx<3376> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3376> as NatIndex>::Out as Nat>::VAL == 3376);
impl NatIndex for Idx<3377> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3377> as NatIndex>::Out as Nat>::VAL == 3377);
impl NatIndex for Idx<3378> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3378> as NatIndex>::Out as Nat>::VAL == 3378);
impl NatIndex for Idx<3379> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3379> as NatIndex>::Out as Nat>::VAL == 3379);
impl NatIndex for Idx<3380> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3380> as NatIndex>::Out as Nat>::VAL == 3380);
impl NatIndex for Idx<3381> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3381> as NatIndex>::Out as Nat>::VAL == 3381);
impl NatIndex for Idx<3382> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3382> as NatIndex>::Out as Nat>::VAL == 3382);
impl NatIndex for Idx<3383> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3383> as NatIndex>::Out as Nat>::VAL == 3383);
impl NatIndex for Idx<3384> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3384> as NatIndex>::Out as Nat>::VAL == 3384);
impl NatIndex for Idx<3385> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3385> as NatIndex>::Out as Nat>::VAL == 3385);
impl NatIndex for Idx<3386> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3386> as NatIndex>::Out as Nat>::VAL == 3386);
impl NatIndex for Idx<3387> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3387> as NatIndex>::Out as Nat>::VAL == 3387);
impl NatIndex for Idx<3388> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3388> as NatIndex>::Out as Nat>::VAL == 3388);
impl NatIndex for Idx<3389> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3389> as NatIndex>::Out as Nat>::VAL == 3389);
impl NatIndex for Idx<3390> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3390> as NatIndex>::Out as Nat>::VAL == 3390);
impl NatIndex for Idx<3391> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3391> as NatIndex>::Out as Nat>::VAL == 3391);
impl NatIndex for Idx<3392> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3392> as NatIndex>::Out as Nat>::VAL == 3392);
impl NatIndex for Idx<3393> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3393> as NatIndex>::Out as Nat>::VAL == 3393);
impl NatIndex for Idx<3394> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3394> as NatIndex>::Out as Nat>::VAL == 3394);
impl NatIndex for Idx<3395> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3395> as NatIndex>::Out as Nat>::VAL == 3395);
impl NatIndex for Idx<3396> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3396> as NatIndex>::Out as Nat>::VAL == 3396);
impl NatIndex for Idx<3397> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3397> as NatIndex>::Out as Nat>::VAL == 3397);
impl NatIndex for Idx<3398> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3398> as NatIndex>::Out as Nat>::VAL == 3398);
impl NatIndex for Idx<3399> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3399> as NatIndex>::Out as Nat>::VAL == 3399);
impl NatIndex for Idx<3400> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3400> as NatIndex>::Out as Nat>::VAL == 3400);
impl NatIndex for Idx<3401> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3401> as NatIndex>::Out as Nat>::VAL == 3401);
impl NatIndex for Idx<3402> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3402> as NatIndex>::Out as Nat>::VAL == 3402);
impl NatIndex for Idx<3403> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3403> as NatIndex>::Out as Nat>::VAL == 3403);
impl NatIndex for Idx<3404> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3404> as NatIndex>::Out as Nat>::VAL == 3404);
impl NatIndex for Idx<3405> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3405> as NatIndex>::Out as Nat>::VAL == 3405);
impl NatIndex for Idx<3406> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3406> as NatIndex>::Out as Nat>::VAL == 3406);
impl NatIndex for Idx<3407> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3407> as NatIndex>::Out as Nat>::VAL == 3407);
impl NatIndex for Idx<3408> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3408> as NatIndex>::Out as Nat>::VAL == 3408);
impl NatIndex for Idx<3409> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3409> as NatIndex>::Out as Nat>::VAL == 3409);
impl NatIndex for Idx<3410> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3410> as NatIndex>::Out as Nat>::VAL == 3410);
impl NatIndex for Idx<3411> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3411> as NatIndex>::Out as Nat>::VAL == 3411);
impl NatIndex for Idx<3412> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3412> as NatIndex>::Out as Nat>::VAL == 3412);
impl NatIndex for Idx<3413> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3413> as NatIndex>::Out as Nat>::VAL == 3413);
impl NatIndex for Idx<3414> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3414> as NatIndex>::Out as Nat>::VAL == 3414);
impl NatIndex for Idx<3415> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3415> as NatIndex>::Out as Nat>::VAL == 3415);
impl NatIndex for Idx<3416> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3416> as NatIndex>::Out as Nat>::VAL == 3416);
impl NatIndex for Idx<3417> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3417> as NatIndex>::Out as Nat>::VAL == 3417);
impl NatIndex for Idx<3418> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3418> as NatIndex>::Out as Nat>::VAL == 3418);
impl NatIndex for Idx<3419> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3419> as NatIndex>::Out as Nat>::VAL == 3419);
impl NatIndex for Idx<3420> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3420> as NatIndex>::Out as Nat>::VAL == 3420);
impl NatIndex for Idx<3421> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3421> as NatIndex>::Out as Nat>::VAL == 3421);
impl NatIndex for Idx<3422> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3422> as NatIndex>::Out as Nat>::VAL == 3422);
impl NatIndex for Idx<3423> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3423> as NatIndex>::Out as Nat>::VAL == 3423);
impl NatIndex for Idx<3424> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3424> as NatIndex>::Out as Nat>::VAL == 3424);
impl NatIndex for Idx<3425> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3425> as NatIndex>::Out as Nat>::VAL == 3425);
impl NatIndex for Idx<3426> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3426> as NatIndex>::Out as Nat>::VAL == 3426);
impl NatIndex for Idx<3427> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3427> as NatIndex>::Out as Nat>::VAL == 3427);
impl NatIndex for Idx<3428> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3428> as NatIndex>::Out as Nat>::VAL == 3428);
impl NatIndex for Idx<3429> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3429> as NatIndex>::Out as Nat>::VAL == 3429);
impl NatIndex for Idx<3430> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3430> as NatIndex>::Out as Nat>::VAL == 3430);
impl NatIndex for Idx<3431> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3431> as NatIndex>::Out as Nat>::VAL == 3431);
impl NatIndex for Idx<3432> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3432> as NatIndex>::Out as Nat>::VAL == 3432);
impl NatIndex for Idx<3433> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3433> as NatIndex>::Out as Nat>::VAL == 3433);
impl NatIndex for Idx<3434> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3434> as NatIndex>::Out as Nat>::VAL == 3434);
impl NatIndex for Idx<3435> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3435> as NatIndex>::Out as Nat>::VAL == 3435);
impl NatIndex for Idx<3436> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3436> as NatIndex>::Out as Nat>::VAL == 3436);
impl NatIndex for Idx<3437> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3437> as NatIndex>::Out as Nat>::VAL == 3437);
impl NatIndex for Idx<3438> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3438> as NatIndex>::Out as Nat>::VAL == 3438);
impl NatIndex for Idx<3439> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3439> as NatIndex>::Out as Nat>::VAL == 3439);
impl NatIndex for Idx<3440> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3440> as NatIndex>::Out as Nat>::VAL == 3440);
impl NatIndex for Idx<3441> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3441> as NatIndex>::Out as Nat>::VAL == 3441);
impl NatIndex for Idx<3442> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3442> as NatIndex>::Out as Nat>::VAL == 3442);
impl NatIndex for Idx<3443> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3443> as NatIndex>::Out as Nat>::VAL == 3443);
impl NatIndex for Idx<3444> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3444> as NatIndex>::Out as Nat>::VAL == 3444);
impl NatIndex for Idx<3445> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3445> as NatIndex>::Out as Nat>::VAL == 3445);
impl NatIndex for Idx<3446> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3446> as NatIndex>::Out as Nat>::VAL == 3446);
impl NatIndex for Idx<3447> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3447> as NatIndex>::Out as Nat>::VAL == 3447);
impl NatIndex for Idx<3448> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3448> as NatIndex>::Out as Nat>::VAL == 3448);
impl NatIndex for Idx<3449> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3449> as NatIndex>::Out as Nat>::VAL == 3449);
impl NatIndex for Idx<3450> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3450> as NatIndex>::Out as Nat>::VAL == 3450);
impl NatIndex for Idx<3451> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3451> as NatIndex>::Out as Nat>::VAL == 3451);
impl NatIndex for Idx<3452> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3452> as NatIndex>::Out as Nat>::VAL == 3452);
impl NatIndex for Idx<3453> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3453> as NatIndex>::Out as Nat>::VAL == 3453);
impl NatIndex for Idx<3454> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3454> as NatIndex>::Out as Nat>::VAL == 3454);
impl NatIndex for Idx<3455> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3455> as NatIndex>::Out as Nat>::VAL == 3455);
impl NatIndex for Idx<3456> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3456> as NatIndex>::Out as Nat>::VAL == 3456);
impl NatIndex for Idx<3457> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3457> as NatIndex>::Out as Nat>::VAL == 3457);
impl NatIndex for Idx<3458> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3458> as NatIndex>::Out as Nat>::VAL == 3458);
impl NatIndex for Idx<3459> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3459> as NatIndex>::Out as Nat>::VAL == 3459);
impl NatIndex for Idx<3460> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3460> as NatIndex>::Out as Nat>::VAL == 3460);
impl NatIndex for Idx<3461> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3461> as NatIndex>::Out as Nat>::VAL == 3461);
impl NatIndex for Idx<3462> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3462> as NatIndex>::Out as Nat>::VAL == 3462);
impl NatIndex for Idx<3463> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3463> as NatIndex>::Out as Nat>::VAL == 3463);
impl NatIndex for Idx<3464> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3464> as NatIndex>::Out as Nat>::VAL == 3464);
impl NatIndex for Idx<3465> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3465> as NatIndex>::Out as Nat>::VAL == 3465);
impl NatIndex for Idx<3466> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3466> as NatIndex>::Out as Nat>::VAL == 3466);
impl NatIndex for Idx<3467> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3467> as NatIndex>::Out as Nat>::VAL == 3467);
impl NatIndex for Idx<3468> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3468> as NatIndex>::Out as Nat>::VAL == 3468);
impl NatIndex for Idx<3469> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3469> as NatIndex>::Out as Nat>::VAL == 3469);
impl NatIndex for Idx<3470> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3470> as NatIndex>::Out as Nat>::VAL == 3470);
impl NatIndex for Idx<3471> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3471> as NatIndex>::Out as Nat>::VAL == 3471);
impl NatIndex for Idx<3472> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3472> as NatIndex>::Out as Nat>::VAL == 3472);
impl NatIndex for Idx<3473> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3473> as NatIndex>::Out as Nat>::VAL == 3473);
impl NatIndex for Idx<3474> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3474> as NatIndex>::Out as Nat>::VAL == 3474);
impl NatIndex for Idx<3475> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3475> as NatIndex>::Out as Nat>::VAL == 3475);
impl NatIndex for Idx<3476> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3476> as NatIndex>::Out as Nat>::VAL == 3476);
impl NatIndex for Idx<3477> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3477> as NatIndex>::Out as Nat>::VAL == 3477);
impl NatIndex for Idx<3478> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3478> as NatIndex>::Out as Nat>::VAL == 3478);
impl NatIndex for Idx<3479> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3479> as NatIndex>::Out as Nat>::VAL == 3479);
impl NatIndex for Idx<3480> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3480> as NatIndex>::Out as Nat>::VAL == 3480);
impl NatIndex for Idx<3481> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3481> as NatIndex>::Out as Nat>::VAL == 3481);
impl NatIndex for Idx<3482> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3482> as NatIndex>::Out as Nat>::VAL == 3482);
impl NatIndex for Idx<3483> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3483> as NatIndex>::Out as Nat>::VAL == 3483);
impl NatIndex for Idx<3484> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3484> as NatIndex>::Out as Nat>::VAL == 3484);
impl NatIndex for Idx<3485> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3485> as NatIndex>::Out as Nat>::VAL == 3485);
impl NatIndex for Idx<3486> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3486> as NatIndex>::Out as Nat>::VAL == 3486);
impl NatIndex for Idx<3487> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3487> as NatIndex>::Out as Nat>::VAL == 3487);
impl NatIndex for Idx<3488> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3488> as NatIndex>::Out as Nat>::VAL == 3488);
impl NatIndex for Idx<3489> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3489> as NatIndex>::Out as Nat>::VAL == 3489);
impl NatIndex for Idx<3490> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3490> as NatIndex>::Out as Nat>::VAL == 3490);
impl NatIndex for Idx<3491> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3491> as NatIndex>::Out as Nat>::VAL == 3491);
impl NatIndex for Idx<3492> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3492> as NatIndex>::Out as Nat>::VAL == 3492);
impl NatIndex for Idx<3493> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3493> as NatIndex>::Out as Nat>::VAL == 3493);
impl NatIndex for Idx<3494> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3494> as NatIndex>::Out as Nat>::VAL == 3494);
impl NatIndex for Idx<3495> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3495> as NatIndex>::Out as Nat>::VAL == 3495);
impl NatIndex for Idx<3496> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3496> as NatIndex>::Out as Nat>::VAL == 3496);
impl NatIndex for Idx<3497> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3497> as NatIndex>::Out as Nat>::VAL == 3497);
impl NatIndex for Idx<3498> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3498> as NatIndex>::Out as Nat>::VAL == 3498);
impl NatIndex for Idx<3499> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3499> as NatIndex>::Out as Nat>::VAL == 3499);
impl NatIndex for Idx<3500> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3500> as NatIndex>::Out as Nat>::VAL == 3500);
impl NatIndex for Idx<3501> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3501> as NatIndex>::Out as Nat>::VAL == 3501);
impl NatIndex for Idx<3502> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3502> as NatIndex>::Out as Nat>::VAL == 3502);
impl NatIndex for Idx<3503> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3503> as NatIndex>::Out as Nat>::VAL == 3503);
impl NatIndex for Idx<3504> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3504> as NatIndex>::Out as Nat>::VAL == 3504);
impl NatIndex for Idx<3505> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3505> as NatIndex>::Out as Nat>::VAL == 3505);
impl NatIndex for Idx<3506> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3506> as NatIndex>::Out as Nat>::VAL == 3506);
impl NatIndex for Idx<3507> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3507> as NatIndex>::Out as Nat>::VAL == 3507);
impl NatIndex for Idx<3508> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3508> as NatIndex>::Out as Nat>::VAL == 3508);
impl NatIndex for Idx<3509> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3509> as NatIndex>::Out as Nat>::VAL == 3509);
impl NatIndex for Idx<3510> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3510> as NatIndex>::Out as Nat>::VAL == 3510);
impl NatIndex for Idx<3511> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3511> as NatIndex>::Out as Nat>::VAL == 3511);
impl NatIndex for Idx<3512> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3512> as NatIndex>::Out as Nat>::VAL == 3512);
impl NatIndex for Idx<3513> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3513> as NatIndex>::Out as Nat>::VAL == 3513);
impl NatIndex for Idx<3514> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3514> as NatIndex>::Out as Nat>::VAL == 3514);
impl NatIndex for Idx<3515> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3515> as NatIndex>::Out as Nat>::VAL == 3515);
impl NatIndex for Idx<3516> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3516> as NatIndex>::Out as Nat>::VAL == 3516);
impl NatIndex for Idx<3517> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3517> as NatIndex>::Out as Nat>::VAL == 3517);
impl NatIndex for Idx<3518> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3518> as NatIndex>::Out as Nat>::VAL == 3518);
impl NatIndex for Idx<3519> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3519> as NatIndex>::Out as Nat>::VAL == 3519);
impl NatIndex for Idx<3520> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3520> as NatIndex>::Out as Nat>::VAL == 3520);
impl NatIndex for Idx<3521> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3521> as NatIndex>::Out as Nat>::VAL == 3521);
impl NatIndex for Idx<3522> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3522> as NatIndex>::Out as Nat>::VAL == 3522);
impl NatIndex for Idx<3523> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3523> as NatIndex>::Out as Nat>::VAL == 3523);
impl NatIndex for Idx<3524> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3524> as NatIndex>::Out as Nat>::VAL == 3524);
impl NatIndex for Idx<3525> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3525> as NatIndex>::Out as Nat>::VAL == 3525);
impl NatIndex for Idx<3526> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3526> as NatIndex>::Out as Nat>::VAL == 3526);
impl NatIndex for Idx<3527> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3527> as NatIndex>::Out as Nat>::VAL == 3527);
impl NatIndex for Idx<3528> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3528> as NatIndex>::Out as Nat>::VAL == 3528);
impl NatIndex for Idx<3529> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3529> as NatIndex>::Out as Nat>::VAL == 3529);
impl NatIndex for Idx<3530> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3530> as NatIndex>::Out as Nat>::VAL == 3530);
impl NatIndex for Idx<3531> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3531> as NatIndex>::Out as Nat>::VAL == 3531);
impl NatIndex for Idx<3532> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3532> as NatIndex>::Out as Nat>::VAL == 3532);
impl NatIndex for Idx<3533> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3533> as NatIndex>::Out as Nat>::VAL == 3533);
impl NatIndex for Idx<3534> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3534> as NatIndex>::Out as Nat>::VAL == 3534);
impl NatIndex for Idx<3535> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3535> as NatIndex>::Out as Nat>::VAL == 3535);
impl NatIndex for Idx<3536> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3536> as NatIndex>::Out as Nat>::VAL == 3536);
impl NatIndex for Idx<3537> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3537> as NatIndex>::Out as Nat>::VAL == 3537);
impl NatIndex for Idx<3538> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3538> as NatIndex>::Out as Nat>::VAL == 3538);
impl NatIndex for Idx<3539> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3539> as NatIndex>::Out as Nat>::VAL == 3539);
impl NatIndex for Idx<3540> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3540> as NatIndex>::Out as Nat>::VAL == 3540);
impl NatIndex for Idx<3541> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3541> as NatIndex>::Out as Nat>::VAL == 3541);
impl NatIndex for Idx<3542> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3542> as NatIndex>::Out as Nat>::VAL == 3542);
impl NatIndex for Idx<3543> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3543> as NatIndex>::Out as Nat>::VAL == 3543);
impl NatIndex for Idx<3544> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3544> as NatIndex>::Out as Nat>::VAL == 3544);
impl NatIndex for Idx<3545> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3545> as NatIndex>::Out as Nat>::VAL == 3545);
impl NatIndex for Idx<3546> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3546> as NatIndex>::Out as Nat>::VAL == 3546);
impl NatIndex for Idx<3547> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3547> as NatIndex>::Out as Nat>::VAL == 3547);
impl NatIndex for Idx<3548> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3548> as NatIndex>::Out as Nat>::VAL == 3548);
impl NatIndex for Idx<3549> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3549> as NatIndex>::Out as Nat>::VAL == 3549);
impl NatIndex for Idx<3550> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3550> as NatIndex>::Out as Nat>::VAL == 3550);
impl NatIndex for Idx<3551> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3551> as NatIndex>::Out as Nat>::VAL == 3551);
impl NatIndex for Idx<3552> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3552> as NatIndex>::Out as Nat>::VAL == 3552);
impl NatIndex for Idx<3553> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3553> as NatIndex>::Out as Nat>::VAL == 3553);
impl NatIndex for Idx<3554> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3554> as NatIndex>::Out as Nat>::VAL == 3554);
impl NatIndex for Idx<3555> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3555> as NatIndex>::Out as Nat>::VAL == 3555);
impl NatIndex for Idx<3556> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3556> as NatIndex>::Out as Nat>::VAL == 3556);
impl NatIndex for Idx<3557> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3557> as NatIndex>::Out as Nat>::VAL == 3557);
impl NatIndex for Idx<3558> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3558> as NatIndex>::Out as Nat>::VAL == 3558);
impl NatIndex for Idx<3559> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3559> as NatIndex>::Out as Nat>::VAL == 3559);
impl NatIndex for Idx<3560> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3560> as NatIndex>::Out as Nat>::VAL == 3560);
impl NatIndex for Idx<3561> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3561> as NatIndex>::Out as Nat>::VAL == 3561);
impl NatIndex for Idx<3562> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3562> as NatIndex>::Out as Nat>::VAL == 3562);
impl NatIndex for Idx<3563> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3563> as NatIndex>::Out as Nat>::VAL == 3563);
impl NatIndex for Idx<3564> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3564> as NatIndex>::Out as Nat>::VAL == 3564);
impl NatIndex for Idx<3565> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3565> as NatIndex>::Out as Nat>::VAL == 3565);
impl NatIndex for Idx<3566> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3566> as NatIndex>::Out as Nat>::VAL == 3566);
impl NatIndex for Idx<3567> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3567> as NatIndex>::Out as Nat>::VAL == 3567);
impl NatIndex for Idx<3568> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3568> as NatIndex>::Out as Nat>::VAL == 3568);
impl NatIndex for Idx<3569> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3569> as NatIndex>::Out as Nat>::VAL == 3569);
impl NatIndex for Idx<3570> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3570> as NatIndex>::Out as Nat>::VAL == 3570);
impl NatIndex for Idx<3571> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3571> as NatIndex>::Out as Nat>::VAL == 3571);
impl NatIndex for Idx<3572> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3572> as NatIndex>::Out as Nat>::VAL == 3572);
impl NatIndex for Idx<3573> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3573> as NatIndex>::Out as Nat>::VAL == 3573);
impl NatIndex for Idx<3574> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3574> as NatIndex>::Out as Nat>::VAL == 3574);
impl NatIndex for Idx<3575> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3575> as NatIndex>::Out as Nat>::VAL == 3575);
impl NatIndex for Idx<3576> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3576> as NatIndex>::Out as Nat>::VAL == 3576);
impl NatIndex for Idx<3577> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3577> as NatIndex>::Out as Nat>::VAL == 3577);
impl NatIndex for Idx<3578> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3578> as NatIndex>::Out as Nat>::VAL == 3578);
impl NatIndex for Idx<3579> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3579> as NatIndex>::Out as Nat>::VAL == 3579);
impl NatIndex for Idx<3580> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3580> as NatIndex>::Out as Nat>::VAL == 3580);
impl NatIndex for Idx<3581> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3581> as NatIndex>::Out as Nat>::VAL == 3581);
impl NatIndex for Idx<3582> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3582> as NatIndex>::Out as Nat>::VAL == 3582);
impl NatIndex for Idx<3583> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3583> as NatIndex>::Out as Nat>::VAL == 3583);
impl NatIndex for Idx<3584> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3584> as NatIndex>::Out as Nat>::VAL == 3584);
impl NatIndex for Idx<3585> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3585> as NatIndex>::Out as Nat>::VAL == 3585);
impl NatIndex for Idx<3586> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3586> as NatIndex>::Out as Nat>::VAL == 3586);
impl NatIndex for Idx<3587> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3587> as NatIndex>::Out as Nat>::VAL == 3587);
impl NatIndex for Idx<3588> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3588> as NatIndex>::Out as Nat>::VAL == 3588);
impl NatIndex for Idx<3589> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3589> as NatIndex>::Out as Nat>::VAL == 3589);
impl NatIndex for Idx<3590> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3590> as NatIndex>::Out as Nat>::VAL == 3590);
impl NatIndex for Idx<3591> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3591> as NatIndex>::Out as Nat>::VAL == 3591);
impl NatIndex for Idx<3592> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3592> as NatIndex>::Out as Nat>::VAL == 3592);
impl NatIndex for Idx<3593> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3593> as NatIndex>::Out as Nat>::VAL == 3593);
impl NatIndex for Idx<3594> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3594> as NatIndex>::Out as Nat>::VAL == 3594);
impl NatIndex for Idx<3595> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3595> as NatIndex>::Out as Nat>::VAL == 3595);
impl NatIndex for Idx<3596> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3596> as NatIndex>::Out as Nat>::VAL == 3596);
impl NatIndex for Idx<3597> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3597> as NatIndex>::Out as Nat>::VAL == 3597);
impl NatIndex for Idx<3598> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3598> as NatIndex>::Out as Nat>::VAL == 3598);
impl NatIndex for Idx<3599> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3599> as NatIndex>::Out as Nat>::VAL == 3599);
impl NatIndex for Idx<3600> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3600> as NatIndex>::Out as Nat>::VAL == 3600);
impl NatIndex for Idx<3601> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3601> as NatIndex>::Out as Nat>::VAL == 3601);
impl NatIndex for Idx<3602> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3602> as NatIndex>::Out as Nat>::VAL == 3602);
impl NatIndex for Idx<3603> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3603> as NatIndex>::Out as Nat>::VAL == 3603);
impl NatIndex for Idx<3604> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3604> as NatIndex>::Out as Nat>::VAL == 3604);
impl NatIndex for Idx<3605> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3605> as NatIndex>::Out as Nat>::VAL == 3605);
impl NatIndex for Idx<3606> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3606> as NatIndex>::Out as Nat>::VAL == 3606);
impl NatIndex for Idx<3607> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3607> as NatIndex>::Out as Nat>::VAL == 3607);
impl NatIndex for Idx<3608> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3608> as NatIndex>::Out as Nat>::VAL == 3608);
impl NatIndex for Idx<3609> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3609> as NatIndex>::Out as Nat>::VAL == 3609);
impl NatIndex for Idx<3610> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3610> as NatIndex>::Out as Nat>::VAL == 3610);
impl NatIndex for Idx<3611> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3611> as NatIndex>::Out as Nat>::VAL == 3611);
impl NatIndex for Idx<3612> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3612> as NatIndex>::Out as Nat>::VAL == 3612);
impl NatIndex for Idx<3613> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3613> as NatIndex>::Out as Nat>::VAL == 3613);
impl NatIndex for Idx<3614> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3614> as NatIndex>::Out as Nat>::VAL == 3614);
impl NatIndex for Idx<3615> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3615> as NatIndex>::Out as Nat>::VAL == 3615);
impl NatIndex for Idx<3616> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3616> as NatIndex>::Out as Nat>::VAL == 3616);
impl NatIndex for Idx<3617> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3617> as NatIndex>::Out as Nat>::VAL == 3617);
impl NatIndex for Idx<3618> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3618> as NatIndex>::Out as Nat>::VAL == 3618);
impl NatIndex for Idx<3619> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3619> as NatIndex>::Out as Nat>::VAL == 3619);
impl NatIndex for Idx<3620> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3620> as NatIndex>::Out as Nat>::VAL == 3620);
impl NatIndex for Idx<3621> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3621> as NatIndex>::Out as Nat>::VAL == 3621);
impl NatIndex for Idx<3622> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3622> as NatIndex>::Out as Nat>::VAL == 3622);
impl NatIndex for Idx<3623> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3623> as NatIndex>::Out as Nat>::VAL == 3623);
impl NatIndex for Idx<3624> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3624> as NatIndex>::Out as Nat>::VAL == 3624);
impl NatIndex for Idx<3625> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3625> as NatIndex>::Out as Nat>::VAL == 3625);
impl NatIndex for Idx<3626> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3626> as NatIndex>::Out as Nat>::VAL == 3626);
impl NatIndex for Idx<3627> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3627> as NatIndex>::Out as Nat>::VAL == 3627);
impl NatIndex for Idx<3628> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3628> as NatIndex>::Out as Nat>::VAL == 3628);
impl NatIndex for Idx<3629> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3629> as NatIndex>::Out as Nat>::VAL == 3629);
impl NatIndex for Idx<3630> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3630> as NatIndex>::Out as Nat>::VAL == 3630);
impl NatIndex for Idx<3631> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3631> as NatIndex>::Out as Nat>::VAL == 3631);
impl NatIndex for Idx<3632> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3632> as NatIndex>::Out as Nat>::VAL == 3632);
impl NatIndex for Idx<3633> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3633> as NatIndex>::Out as Nat>::VAL == 3633);
impl NatIndex for Idx<3634> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3634> as NatIndex>::Out as Nat>::VAL == 3634);
impl NatIndex for Idx<3635> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3635> as NatIndex>::Out as Nat>::VAL == 3635);
impl NatIndex for Idx<3636> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3636> as NatIndex>::Out as Nat>::VAL == 3636);
impl NatIndex for Idx<3637> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3637> as NatIndex>::Out as Nat>::VAL == 3637);
impl NatIndex for Idx<3638> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3638> as NatIndex>::Out as Nat>::VAL == 3638);
impl NatIndex for Idx<3639> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3639> as NatIndex>::Out as Nat>::VAL == 3639);
impl NatIndex for Idx<3640> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3640> as NatIndex>::Out as Nat>::VAL == 3640);
impl NatIndex for Idx<3641> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3641> as NatIndex>::Out as Nat>::VAL == 3641);
impl NatIndex for Idx<3642> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3642> as NatIndex>::Out as Nat>::VAL == 3642);
impl NatIndex for Idx<3643> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3643> as NatIndex>::Out as Nat>::VAL == 3643);
impl NatIndex for Idx<3644> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3644> as NatIndex>::Out as Nat>::VAL == 3644);
impl NatIndex for Idx<3645> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3645> as NatIndex>::Out as Nat>::VAL == 3645);
impl NatIndex for Idx<3646> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3646> as NatIndex>::Out as Nat>::VAL == 3646);
impl NatIndex for Idx<3647> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3647> as NatIndex>::Out as Nat>::VAL == 3647);
impl NatIndex for Idx<3648> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3648> as NatIndex>::Out as Nat>::VAL == 3648);
impl NatIndex for Idx<3649> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3649> as NatIndex>::Out as Nat>::VAL == 3649);
impl NatIndex for Idx<3650> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3650> as NatIndex>::Out as Nat>::VAL == 3650);
impl NatIndex for Idx<3651> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3651> as NatIndex>::Out as Nat>::VAL == 3651);
impl NatIndex for Idx<3652> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3652> as NatIndex>::Out as Nat>::VAL == 3652);
impl NatIndex for Idx<3653> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3653> as NatIndex>::Out as Nat>::VAL == 3653);
impl NatIndex for Idx<3654> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3654> as NatIndex>::Out as Nat>::VAL == 3654);
impl NatIndex for Idx<3655> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3655> as NatIndex>::Out as Nat>::VAL == 3655);
impl NatIndex for Idx<3656> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3656> as NatIndex>::Out as Nat>::VAL == 3656);
impl NatIndex for Idx<3657> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3657> as NatIndex>::Out as Nat>::VAL == 3657);
impl NatIndex for Idx<3658> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3658> as NatIndex>::Out as Nat>::VAL == 3658);
impl NatIndex for Idx<3659> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3659> as NatIndex>::Out as Nat>::VAL == 3659);
impl NatIndex for Idx<3660> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3660> as NatIndex>::Out as Nat>::VAL == 3660);
impl NatIndex for Idx<3661> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3661> as NatIndex>::Out as Nat>::VAL == 3661);
impl NatIndex for Idx<3662> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3662> as NatIndex>::Out as Nat>::VAL == 3662);
impl NatIndex for Idx<3663> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3663> as NatIndex>::Out as Nat>::VAL == 3663);
impl NatIndex for Idx<3664> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3664> as NatIndex>::Out as Nat>::VAL == 3664);
impl NatIndex for Idx<3665> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3665> as NatIndex>::Out as Nat>::VAL == 3665);
impl NatIndex for Idx<3666> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3666> as NatIndex>::Out as Nat>::VAL == 3666);
impl NatIndex for Idx<3667> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3667> as NatIndex>::Out as Nat>::VAL == 3667);
impl NatIndex for Idx<3668> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3668> as NatIndex>::Out as Nat>::VAL == 3668);
impl NatIndex for Idx<3669> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3669> as NatIndex>::Out as Nat>::VAL == 3669);
impl NatIndex for Idx<3670> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3670> as NatIndex>::Out as Nat>::VAL == 3670);
impl NatIndex for Idx<3671> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3671> as NatIndex>::Out as Nat>::VAL == 3671);
impl NatIndex for Idx<3672> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3672> as NatIndex>::Out as Nat>::VAL == 3672);
impl NatIndex for Idx<3673> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3673> as NatIndex>::Out as Nat>::VAL == 3673);
impl NatIndex for Idx<3674> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3674> as NatIndex>::Out as Nat>::VAL == 3674);
impl NatIndex for Idx<3675> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3675> as NatIndex>::Out as Nat>::VAL == 3675);
impl NatIndex for Idx<3676> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3676> as NatIndex>::Out as Nat>::VAL == 3676);
impl NatIndex for Idx<3677> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3677> as NatIndex>::Out as Nat>::VAL == 3677);
impl NatIndex for Idx<3678> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3678> as NatIndex>::Out as Nat>::VAL == 3678);
impl NatIndex for Idx<3679> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3679> as NatIndex>::Out as Nat>::VAL == 3679);
impl NatIndex for Idx<3680> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3680> as NatIndex>::Out as Nat>::VAL == 3680);
impl NatIndex for Idx<3681> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3681> as NatIndex>::Out as Nat>::VAL == 3681);
impl NatIndex for Idx<3682> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3682> as NatIndex>::Out as Nat>::VAL == 3682);
impl NatIndex for Idx<3683> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3683> as NatIndex>::Out as Nat>::VAL == 3683);
impl NatIndex for Idx<3684> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3684> as NatIndex>::Out as Nat>::VAL == 3684);
impl NatIndex for Idx<3685> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3685> as NatIndex>::Out as Nat>::VAL == 3685);
impl NatIndex for Idx<3686> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3686> as NatIndex>::Out as Nat>::VAL == 3686);
impl NatIndex for Idx<3687> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3687> as NatIndex>::Out as Nat>::VAL == 3687);
impl NatIndex for Idx<3688> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3688> as NatIndex>::Out as Nat>::VAL == 3688);
impl NatIndex for Idx<3689> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3689> as NatIndex>::Out as Nat>::VAL == 3689);
impl NatIndex for Idx<3690> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3690> as NatIndex>::Out as Nat>::VAL == 3690);
impl NatIndex for Idx<3691> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3691> as NatIndex>::Out as Nat>::VAL == 3691);
impl NatIndex for Idx<3692> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3692> as NatIndex>::Out as Nat>::VAL == 3692);
impl NatIndex for Idx<3693> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3693> as NatIndex>::Out as Nat>::VAL == 3693);
impl NatIndex for Idx<3694> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3694> as NatIndex>::Out as Nat>::VAL == 3694);
impl NatIndex for Idx<3695> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3695> as NatIndex>::Out as Nat>::VAL == 3695);
impl NatIndex for Idx<3696> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3696> as NatIndex>::Out as Nat>::VAL == 3696);
impl NatIndex for Idx<3697> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3697> as NatIndex>::Out as Nat>::VAL == 3697);
impl NatIndex for Idx<3698> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3698> as NatIndex>::Out as Nat>::VAL == 3698);
impl NatIndex for Idx<3699> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3699> as NatIndex>::Out as Nat>::VAL == 3699);
impl NatIndex for Idx<3700> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3700> as NatIndex>::Out as Nat>::VAL == 3700);
impl NatIndex for Idx<3701> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3701> as NatIndex>::Out as Nat>::VAL == 3701);
impl NatIndex for Idx<3702> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3702> as NatIndex>::Out as Nat>::VAL == 3702);
impl NatIndex for Idx<3703> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3703> as NatIndex>::Out as Nat>::VAL == 3703);
impl NatIndex for Idx<3704> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3704> as NatIndex>::Out as Nat>::VAL == 3704);
impl NatIndex for Idx<3705> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3705> as NatIndex>::Out as Nat>::VAL == 3705);
impl NatIndex for Idx<3706> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3706> as NatIndex>::Out as Nat>::VAL == 3706);
impl NatIndex for Idx<3707> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3707> as NatIndex>::Out as Nat>::VAL == 3707);
impl NatIndex for Idx<3708> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3708> as NatIndex>::Out as Nat>::VAL == 3708);
impl NatIndex for Idx<3709> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3709> as NatIndex>::Out as Nat>::VAL == 3709);
impl NatIndex for Idx<3710> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3710> as NatIndex>::Out as Nat>::VAL == 3710);
impl NatIndex for Idx<3711> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3711> as NatIndex>::Out as Nat>::VAL == 3711);
impl NatIndex for Idx<3712> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3712> as NatIndex>::Out as Nat>::VAL == 3712);
impl NatIndex for Idx<3713> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3713> as NatIndex>::Out as Nat>::VAL == 3713);
impl NatIndex for Idx<3714> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3714> as NatIndex>::Out as Nat>::VAL == 3714);
impl NatIndex for Idx<3715> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3715> as NatIndex>::Out as Nat>::VAL == 3715);
impl NatIndex for Idx<3716> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3716> as NatIndex>::Out as Nat>::VAL == 3716);
impl NatIndex for Idx<3717> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3717> as NatIndex>::Out as Nat>::VAL == 3717);
impl NatIndex for Idx<3718> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3718> as NatIndex>::Out as Nat>::VAL == 3718);
impl NatIndex for Idx<3719> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3719> as NatIndex>::Out as Nat>::VAL == 3719);
impl NatIndex for Idx<3720> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3720> as NatIndex>::Out as Nat>::VAL == 3720);
impl NatIndex for Idx<3721> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3721> as NatIndex>::Out as Nat>::VAL == 3721);
impl NatIndex for Idx<3722> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3722> as NatIndex>::Out as Nat>::VAL == 3722);
impl NatIndex for Idx<3723> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3723> as NatIndex>::Out as Nat>::VAL == 3723);
impl NatIndex for Idx<3724> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3724> as NatIndex>::Out as Nat>::VAL == 3724);
impl NatIndex for Idx<3725> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3725> as NatIndex>::Out as Nat>::VAL == 3725);
impl NatIndex for Idx<3726> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3726> as NatIndex>::Out as Nat>::VAL == 3726);
impl NatIndex for Idx<3727> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3727> as NatIndex>::Out as Nat>::VAL == 3727);
impl NatIndex for Idx<3728> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3728> as NatIndex>::Out as Nat>::VAL == 3728);
impl NatIndex for Idx<3729> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3729> as NatIndex>::Out as Nat>::VAL == 3729);
impl NatIndex for Idx<3730> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3730> as NatIndex>::Out as Nat>::VAL == 3730);
impl NatIndex for Idx<3731> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3731> as NatIndex>::Out as Nat>::VAL == 3731);
impl NatIndex for Idx<3732> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3732> as NatIndex>::Out as Nat>::VAL == 3732);
impl NatIndex for Idx<3733> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3733> as NatIndex>::Out as Nat>::VAL == 3733);
impl NatIndex for Idx<3734> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3734> as NatIndex>::Out as Nat>::VAL == 3734);
impl NatIndex for Idx<3735> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3735> as NatIndex>::Out as Nat>::VAL == 3735);
impl NatIndex for Idx<3736> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3736> as NatIndex>::Out as Nat>::VAL == 3736);
impl NatIndex for Idx<3737> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3737> as NatIndex>::Out as Nat>::VAL == 3737);
impl NatIndex for Idx<3738> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3738> as NatIndex>::Out as Nat>::VAL == 3738);
impl NatIndex for Idx<3739> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3739> as NatIndex>::Out as Nat>::VAL == 3739);
impl NatIndex for Idx<3740> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3740> as NatIndex>::Out as Nat>::VAL == 3740);
impl NatIndex for Idx<3741> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3741> as NatIndex>::Out as Nat>::VAL == 3741);
impl NatIndex for Idx<3742> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3742> as NatIndex>::Out as Nat>::VAL == 3742);
impl NatIndex for Idx<3743> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3743> as NatIndex>::Out as Nat>::VAL == 3743);
impl NatIndex for Idx<3744> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3744> as NatIndex>::Out as Nat>::VAL == 3744);
impl NatIndex for Idx<3745> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3745> as NatIndex>::Out as Nat>::VAL == 3745);
impl NatIndex for Idx<3746> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3746> as NatIndex>::Out as Nat>::VAL == 3746);
impl NatIndex for Idx<3747> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3747> as NatIndex>::Out as Nat>::VAL == 3747);
impl NatIndex for Idx<3748> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3748> as NatIndex>::Out as Nat>::VAL == 3748);
impl NatIndex for Idx<3749> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3749> as NatIndex>::Out as Nat>::VAL == 3749);
impl NatIndex for Idx<3750> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3750> as NatIndex>::Out as Nat>::VAL == 3750);
impl NatIndex for Idx<3751> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3751> as NatIndex>::Out as Nat>::VAL == 3751);
impl NatIndex for Idx<3752> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3752> as NatIndex>::Out as Nat>::VAL == 3752);
impl NatIndex for Idx<3753> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3753> as NatIndex>::Out as Nat>::VAL == 3753);
impl NatIndex for Idx<3754> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3754> as NatIndex>::Out as Nat>::VAL == 3754);
impl NatIndex for Idx<3755> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3755> as NatIndex>::Out as Nat>::VAL == 3755);
impl NatIndex for Idx<3756> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3756> as NatIndex>::Out as Nat>::VAL == 3756);
impl NatIndex for Idx<3757> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3757> as NatIndex>::Out as Nat>::VAL == 3757);
impl NatIndex for Idx<3758> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3758> as NatIndex>::Out as Nat>::VAL == 3758);
impl NatIndex for Idx<3759> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3759> as NatIndex>::Out as Nat>::VAL == 3759);
impl NatIndex for Idx<3760> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3760> as NatIndex>::Out as Nat>::VAL == 3760);
impl NatIndex for Idx<3761> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3761> as NatIndex>::Out as Nat>::VAL == 3761);
impl NatIndex for Idx<3762> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3762> as NatIndex>::Out as Nat>::VAL == 3762);
impl NatIndex for Idx<3763> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3763> as NatIndex>::Out as Nat>::VAL == 3763);
impl NatIndex for Idx<3764> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3764> as NatIndex>::Out as Nat>::VAL == 3764);
impl NatIndex for Idx<3765> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3765> as NatIndex>::Out as Nat>::VAL == 3765);
impl NatIndex for Idx<3766> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3766> as NatIndex>::Out as Nat>::VAL == 3766);
impl NatIndex for Idx<3767> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3767> as NatIndex>::Out as Nat>::VAL == 3767);
impl NatIndex for Idx<3768> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3768> as NatIndex>::Out as Nat>::VAL == 3768);
impl NatIndex for Idx<3769> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3769> as NatIndex>::Out as Nat>::VAL == 3769);
impl NatIndex for Idx<3770> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3770> as NatIndex>::Out as Nat>::VAL == 3770);
impl NatIndex for Idx<3771> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3771> as NatIndex>::Out as Nat>::VAL == 3771);
impl NatIndex for Idx<3772> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3772> as NatIndex>::Out as Nat>::VAL == 3772);
impl NatIndex for Idx<3773> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3773> as NatIndex>::Out as Nat>::VAL == 3773);
impl NatIndex for Idx<3774> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3774> as NatIndex>::Out as Nat>::VAL == 3774);
impl NatIndex for Idx<3775> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3775> as NatIndex>::Out as Nat>::VAL == 3775);
impl NatIndex for Idx<3776> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3776> as NatIndex>::Out as Nat>::VAL == 3776);
impl NatIndex for Idx<3777> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3777> as NatIndex>::Out as Nat>::VAL == 3777);
impl NatIndex for Idx<3778> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3778> as NatIndex>::Out as Nat>::VAL == 3778);
impl NatIndex for Idx<3779> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3779> as NatIndex>::Out as Nat>::VAL == 3779);
impl NatIndex for Idx<3780> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3780> as NatIndex>::Out as Nat>::VAL == 3780);
impl NatIndex for Idx<3781> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3781> as NatIndex>::Out as Nat>::VAL == 3781);
impl NatIndex for Idx<3782> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3782> as NatIndex>::Out as Nat>::VAL == 3782);
impl NatIndex for Idx<3783> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3783> as NatIndex>::Out as Nat>::VAL == 3783);
impl NatIndex for Idx<3784> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3784> as NatIndex>::Out as Nat>::VAL == 3784);
impl NatIndex for Idx<3785> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3785> as NatIndex>::Out as Nat>::VAL == 3785);
impl NatIndex for Idx<3786> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3786> as NatIndex>::Out as Nat>::VAL == 3786);
impl NatIndex for Idx<3787> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3787> as NatIndex>::Out as Nat>::VAL == 3787);
impl NatIndex for Idx<3788> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3788> as NatIndex>::Out as Nat>::VAL == 3788);
impl NatIndex for Idx<3789> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3789> as NatIndex>::Out as Nat>::VAL == 3789);
impl NatIndex for Idx<3790> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3790> as NatIndex>::Out as Nat>::VAL == 3790);
impl NatIndex for Idx<3791> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3791> as NatIndex>::Out as Nat>::VAL == 3791);
impl NatIndex for Idx<3792> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3792> as NatIndex>::Out as Nat>::VAL == 3792);
impl NatIndex for Idx<3793> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3793> as NatIndex>::Out as Nat>::VAL == 3793);
impl NatIndex for Idx<3794> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3794> as NatIndex>::Out as Nat>::VAL == 3794);
impl NatIndex for Idx<3795> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3795> as NatIndex>::Out as Nat>::VAL == 3795);
impl NatIndex for Idx<3796> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3796> as NatIndex>::Out as Nat>::VAL == 3796);
impl NatIndex for Idx<3797> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3797> as NatIndex>::Out as Nat>::VAL == 3797);
impl NatIndex for Idx<3798> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3798> as NatIndex>::Out as Nat>::VAL == 3798);
impl NatIndex for Idx<3799> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3799> as NatIndex>::Out as Nat>::VAL == 3799);
impl NatIndex for Idx<3800> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3800> as NatIndex>::Out as Nat>::VAL == 3800);
impl NatIndex for Idx<3801> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3801> as NatIndex>::Out as Nat>::VAL == 3801);
impl NatIndex for Idx<3802> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3802> as NatIndex>::Out as Nat>::VAL == 3802);
impl NatIndex for Idx<3803> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3803> as NatIndex>::Out as Nat>::VAL == 3803);
impl NatIndex for Idx<3804> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3804> as NatIndex>::Out as Nat>::VAL == 3804);
impl NatIndex for Idx<3805> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3805> as NatIndex>::Out as Nat>::VAL == 3805);
impl NatIndex for Idx<3806> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3806> as NatIndex>::Out as Nat>::VAL == 3806);
impl NatIndex for Idx<3807> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3807> as NatIndex>::Out as Nat>::VAL == 3807);
impl NatIndex for Idx<3808> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3808> as NatIndex>::Out as Nat>::VAL == 3808);
impl NatIndex for Idx<3809> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3809> as NatIndex>::Out as Nat>::VAL == 3809);
impl NatIndex for Idx<3810> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3810> as NatIndex>::Out as Nat>::VAL == 3810);
impl NatIndex for Idx<3811> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3811> as NatIndex>::Out as Nat>::VAL == 3811);
impl NatIndex for Idx<3812> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3812> as NatIndex>::Out as Nat>::VAL == 3812);
impl NatIndex for Idx<3813> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3813> as NatIndex>::Out as Nat>::VAL == 3813);
impl NatIndex for Idx<3814> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3814> as NatIndex>::Out as Nat>::VAL == 3814);
impl NatIndex for Idx<3815> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3815> as NatIndex>::Out as Nat>::VAL == 3815);
impl NatIndex for Idx<3816> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3816> as NatIndex>::Out as Nat>::VAL == 3816);
impl NatIndex for Idx<3817> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3817> as NatIndex>::Out as Nat>::VAL == 3817);
impl NatIndex for Idx<3818> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3818> as NatIndex>::Out as Nat>::VAL == 3818);
impl NatIndex for Idx<3819> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3819> as NatIndex>::Out as Nat>::VAL == 3819);
impl NatIndex for Idx<3820> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3820> as NatIndex>::Out as Nat>::VAL == 3820);
impl NatIndex for Idx<3821> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3821> as NatIndex>::Out as Nat>::VAL == 3821);
impl NatIndex for Idx<3822> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3822> as NatIndex>::Out as Nat>::VAL == 3822);
impl NatIndex for Idx<3823> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3823> as NatIndex>::Out as Nat>::VAL == 3823);
impl NatIndex for Idx<3824> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3824> as NatIndex>::Out as Nat>::VAL == 3824);
impl NatIndex for Idx<3825> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3825> as NatIndex>::Out as Nat>::VAL == 3825);
impl NatIndex for Idx<3826> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3826> as NatIndex>::Out as Nat>::VAL == 3826);
impl NatIndex for Idx<3827> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3827> as NatIndex>::Out as Nat>::VAL == 3827);
impl NatIndex for Idx<3828> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3828> as NatIndex>::Out as Nat>::VAL == 3828);
impl NatIndex for Idx<3829> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3829> as NatIndex>::Out as Nat>::VAL == 3829);
impl NatIndex for Idx<3830> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3830> as NatIndex>::Out as Nat>::VAL == 3830);
impl NatIndex for Idx<3831> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3831> as NatIndex>::Out as Nat>::VAL == 3831);
impl NatIndex for Idx<3832> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3832> as NatIndex>::Out as Nat>::VAL == 3832);
impl NatIndex for Idx<3833> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3833> as NatIndex>::Out as Nat>::VAL == 3833);
impl NatIndex for Idx<3834> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3834> as NatIndex>::Out as Nat>::VAL == 3834);
impl NatIndex for Idx<3835> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3835> as NatIndex>::Out as Nat>::VAL == 3835);
impl NatIndex for Idx<3836> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3836> as NatIndex>::Out as Nat>::VAL == 3836);
impl NatIndex for Idx<3837> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3837> as NatIndex>::Out as Nat>::VAL == 3837);
impl NatIndex for Idx<3838> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3838> as NatIndex>::Out as Nat>::VAL == 3838);
impl NatIndex for Idx<3839> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3839> as NatIndex>::Out as Nat>::VAL == 3839);
impl NatIndex for Idx<3840> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3840> as NatIndex>::Out as Nat>::VAL == 3840);
impl NatIndex for Idx<3841> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3841> as NatIndex>::Out as Nat>::VAL == 3841);
impl NatIndex for Idx<3842> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3842> as NatIndex>::Out as Nat>::VAL == 3842);
impl NatIndex for Idx<3843> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3843> as NatIndex>::Out as Nat>::VAL == 3843);
impl NatIndex for Idx<3844> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3844> as NatIndex>::Out as Nat>::VAL == 3844);
impl NatIndex for Idx<3845> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3845> as NatIndex>::Out as Nat>::VAL == 3845);
impl NatIndex for Idx<3846> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3846> as NatIndex>::Out as Nat>::VAL == 3846);
impl NatIndex for Idx<3847> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3847> as NatIndex>::Out as Nat>::VAL == 3847);
impl NatIndex for Idx<3848> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3848> as NatIndex>::Out as Nat>::VAL == 3848);
impl NatIndex for Idx<3849> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3849> as NatIndex>::Out as Nat>::VAL == 3849);
impl NatIndex for Idx<3850> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3850> as NatIndex>::Out as Nat>::VAL == 3850);
impl NatIndex for Idx<3851> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3851> as NatIndex>::Out as Nat>::VAL == 3851);
impl NatIndex for Idx<3852> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3852> as NatIndex>::Out as Nat>::VAL == 3852);
impl NatIndex for Idx<3853> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3853> as NatIndex>::Out as Nat>::VAL == 3853);
impl NatIndex for Idx<3854> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3854> as NatIndex>::Out as Nat>::VAL == 3854);
impl NatIndex for Idx<3855> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3855> as NatIndex>::Out as Nat>::VAL == 3855);
impl NatIndex for Idx<3856> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3856> as NatIndex>::Out as Nat>::VAL == 3856);
impl NatIndex for Idx<3857> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3857> as NatIndex>::Out as Nat>::VAL == 3857);
impl NatIndex for Idx<3858> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3858> as NatIndex>::Out as Nat>::VAL == 3858);
impl NatIndex for Idx<3859> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3859> as NatIndex>::Out as Nat>::VAL == 3859);
impl NatIndex for Idx<3860> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3860> as NatIndex>::Out as Nat>::VAL == 3860);
impl NatIndex for Idx<3861> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3861> as NatIndex>::Out as Nat>::VAL == 3861);
impl NatIndex for Idx<3862> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3862> as NatIndex>::Out as Nat>::VAL == 3862);
impl NatIndex for Idx<3863> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3863> as NatIndex>::Out as Nat>::VAL == 3863);
impl NatIndex for Idx<3864> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3864> as NatIndex>::Out as Nat>::VAL == 3864);
impl NatIndex for Idx<3865> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3865> as NatIndex>::Out as Nat>::VAL == 3865);
impl NatIndex for Idx<3866> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3866> as NatIndex>::Out as Nat>::VAL == 3866);
impl NatIndex for Idx<3867> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3867> as NatIndex>::Out as Nat>::VAL == 3867);
impl NatIndex for Idx<3868> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3868> as NatIndex>::Out as Nat>::VAL == 3868);
impl NatIndex for Idx<3869> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3869> as NatIndex>::Out as Nat>::VAL == 3869);
impl NatIndex for Idx<3870> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3870> as NatIndex>::Out as Nat>::VAL == 3870);
impl NatIndex for Idx<3871> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3871> as NatIndex>::Out as Nat>::VAL == 3871);
impl NatIndex for Idx<3872> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3872> as NatIndex>::Out as Nat>::VAL == 3872);
impl NatIndex for Idx<3873> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3873> as NatIndex>::Out as Nat>::VAL == 3873);
impl NatIndex for Idx<3874> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3874> as NatIndex>::Out as Nat>::VAL == 3874);
impl NatIndex for Idx<3875> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3875> as NatIndex>::Out as Nat>::VAL == 3875);
impl NatIndex for Idx<3876> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3876> as NatIndex>::Out as Nat>::VAL == 3876);
impl NatIndex for Idx<3877> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3877> as NatIndex>::Out as Nat>::VAL == 3877);
impl NatIndex for Idx<3878> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3878> as NatIndex>::Out as Nat>::VAL == 3878);
impl NatIndex for Idx<3879> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3879> as NatIndex>::Out as Nat>::VAL == 3879);
impl NatIndex for Idx<3880> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3880> as NatIndex>::Out as Nat>::VAL == 3880);
impl NatIndex for Idx<3881> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3881> as NatIndex>::Out as Nat>::VAL == 3881);
impl NatIndex for Idx<3882> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3882> as NatIndex>::Out as Nat>::VAL == 3882);
impl NatIndex for Idx<3883> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3883> as NatIndex>::Out as Nat>::VAL == 3883);
impl NatIndex for Idx<3884> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3884> as NatIndex>::Out as Nat>::VAL == 3884);
impl NatIndex for Idx<3885> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3885> as NatIndex>::Out as Nat>::VAL == 3885);
impl NatIndex for Idx<3886> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3886> as NatIndex>::Out as Nat>::VAL == 3886);
impl NatIndex for Idx<3887> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3887> as NatIndex>::Out as Nat>::VAL == 3887);
impl NatIndex for Idx<3888> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3888> as NatIndex>::Out as Nat>::VAL == 3888);
impl NatIndex for Idx<3889> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3889> as NatIndex>::Out as Nat>::VAL == 3889);
impl NatIndex for Idx<3890> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3890> as NatIndex>::Out as Nat>::VAL == 3890);
impl NatIndex for Idx<3891> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3891> as NatIndex>::Out as Nat>::VAL == 3891);
impl NatIndex for Idx<3892> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3892> as NatIndex>::Out as Nat>::VAL == 3892);
impl NatIndex for Idx<3893> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3893> as NatIndex>::Out as Nat>::VAL == 3893);
impl NatIndex for Idx<3894> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3894> as NatIndex>::Out as Nat>::VAL == 3894);
impl NatIndex for Idx<3895> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3895> as NatIndex>::Out as Nat>::VAL == 3895);
impl NatIndex for Idx<3896> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3896> as NatIndex>::Out as Nat>::VAL == 3896);
impl NatIndex for Idx<3897> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3897> as NatIndex>::Out as Nat>::VAL == 3897);
impl NatIndex for Idx<3898> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3898> as NatIndex>::Out as Nat>::VAL == 3898);
impl NatIndex for Idx<3899> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3899> as NatIndex>::Out as Nat>::VAL == 3899);
impl NatIndex for Idx<3900> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3900> as NatIndex>::Out as Nat>::VAL == 3900);
impl NatIndex for Idx<3901> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3901> as NatIndex>::Out as Nat>::VAL == 3901);
impl NatIndex for Idx<3902> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3902> as NatIndex>::Out as Nat>::VAL == 3902);
impl NatIndex for Idx<3903> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3903> as NatIndex>::Out as Nat>::VAL == 3903);
impl NatIndex for Idx<3904> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3904> as NatIndex>::Out as Nat>::VAL == 3904);
impl NatIndex for Idx<3905> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3905> as NatIndex>::Out as Nat>::VAL == 3905);
impl NatIndex for Idx<3906> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3906> as NatIndex>::Out as Nat>::VAL == 3906);
impl NatIndex for Idx<3907> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3907> as NatIndex>::Out as Nat>::VAL == 3907);
impl NatIndex for Idx<3908> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3908> as NatIndex>::Out as Nat>::VAL == 3908);
impl NatIndex for Idx<3909> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3909> as NatIndex>::Out as Nat>::VAL == 3909);
impl NatIndex for Idx<3910> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3910> as NatIndex>::Out as Nat>::VAL == 3910);
impl NatIndex for Idx<3911> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3911> as NatIndex>::Out as Nat>::VAL == 3911);
impl NatIndex for Idx<3912> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3912> as NatIndex>::Out as Nat>::VAL == 3912);
impl NatIndex for Idx<3913> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3913> as NatIndex>::Out as Nat>::VAL == 3913);
impl NatIndex for Idx<3914> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3914> as NatIndex>::Out as Nat>::VAL == 3914);
impl NatIndex for Idx<3915> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3915> as NatIndex>::Out as Nat>::VAL == 3915);
impl NatIndex for Idx<3916> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3916> as NatIndex>::Out as Nat>::VAL == 3916);
impl NatIndex for Idx<3917> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3917> as NatIndex>::Out as Nat>::VAL == 3917);
impl NatIndex for Idx<3918> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3918> as NatIndex>::Out as Nat>::VAL == 3918);
impl NatIndex for Idx<3919> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3919> as NatIndex>::Out as Nat>::VAL == 3919);
impl NatIndex for Idx<3920> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3920> as NatIndex>::Out as Nat>::VAL == 3920);
impl NatIndex for Idx<3921> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3921> as NatIndex>::Out as Nat>::VAL == 3921);
impl NatIndex for Idx<3922> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3922> as NatIndex>::Out as Nat>::VAL == 3922);
impl NatIndex for Idx<3923> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3923> as NatIndex>::Out as Nat>::VAL == 3923);
impl NatIndex for Idx<3924> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3924> as NatIndex>::Out as Nat>::VAL == 3924);
impl NatIndex for Idx<3925> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3925> as NatIndex>::Out as Nat>::VAL == 3925);
impl NatIndex for Idx<3926> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3926> as NatIndex>::Out as Nat>::VAL == 3926);
impl NatIndex for Idx<3927> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3927> as NatIndex>::Out as Nat>::VAL == 3927);
impl NatIndex for Idx<3928> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3928> as NatIndex>::Out as Nat>::VAL == 3928);
impl NatIndex for Idx<3929> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3929> as NatIndex>::Out as Nat>::VAL == 3929);
impl NatIndex for Idx<3930> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3930> as NatIndex>::Out as Nat>::VAL == 3930);
impl NatIndex for Idx<3931> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3931> as NatIndex>::Out as Nat>::VAL == 3931);
impl NatIndex for Idx<3932> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3932> as NatIndex>::Out as Nat>::VAL == 3932);
impl NatIndex for Idx<3933> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3933> as NatIndex>::Out as Nat>::VAL == 3933);
impl NatIndex for Idx<3934> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3934> as NatIndex>::Out as Nat>::VAL == 3934);
impl NatIndex for Idx<3935> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3935> as NatIndex>::Out as Nat>::VAL == 3935);
impl NatIndex for Idx<3936> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3936> as NatIndex>::Out as Nat>::VAL == 3936);
impl NatIndex for Idx<3937> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3937> as NatIndex>::Out as Nat>::VAL == 3937);
impl NatIndex for Idx<3938> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3938> as NatIndex>::Out as Nat>::VAL == 3938);
impl NatIndex for Idx<3939> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3939> as NatIndex>::Out as Nat>::VAL == 3939);
impl NatIndex for Idx<3940> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3940> as NatIndex>::Out as Nat>::VAL == 3940);
impl NatIndex for Idx<3941> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3941> as NatIndex>::Out as Nat>::VAL == 3941);
impl NatIndex for Idx<3942> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3942> as NatIndex>::Out as Nat>::VAL == 3942);
impl NatIndex for Idx<3943> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3943> as NatIndex>::Out as Nat>::VAL == 3943);
impl NatIndex for Idx<3944> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3944> as NatIndex>::Out as Nat>::VAL == 3944);
impl NatIndex for Idx<3945> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3945> as NatIndex>::Out as Nat>::VAL == 3945);
impl NatIndex for Idx<3946> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3946> as NatIndex>::Out as Nat>::VAL == 3946);
impl NatIndex for Idx<3947> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3947> as NatIndex>::Out as Nat>::VAL == 3947);
impl NatIndex for Idx<3948> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3948> as NatIndex>::Out as Nat>::VAL == 3948);
impl NatIndex for Idx<3949> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3949> as NatIndex>::Out as Nat>::VAL == 3949);
impl NatIndex for Idx<3950> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3950> as NatIndex>::Out as Nat>::VAL == 3950);
impl NatIndex for Idx<3951> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3951> as NatIndex>::Out as Nat>::VAL == 3951);
impl NatIndex for Idx<3952> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3952> as NatIndex>::Out as Nat>::VAL == 3952);
impl NatIndex for Idx<3953> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3953> as NatIndex>::Out as Nat>::VAL == 3953);
impl NatIndex for Idx<3954> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3954> as NatIndex>::Out as Nat>::VAL == 3954);
impl NatIndex for Idx<3955> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3955> as NatIndex>::Out as Nat>::VAL == 3955);
impl NatIndex for Idx<3956> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3956> as NatIndex>::Out as Nat>::VAL == 3956);
impl NatIndex for Idx<3957> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3957> as NatIndex>::Out as Nat>::VAL == 3957);
impl NatIndex for Idx<3958> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3958> as NatIndex>::Out as Nat>::VAL == 3958);
impl NatIndex for Idx<3959> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3959> as NatIndex>::Out as Nat>::VAL == 3959);
impl NatIndex for Idx<3960> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3960> as NatIndex>::Out as Nat>::VAL == 3960);
impl NatIndex for Idx<3961> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3961> as NatIndex>::Out as Nat>::VAL == 3961);
impl NatIndex for Idx<3962> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3962> as NatIndex>::Out as Nat>::VAL == 3962);
impl NatIndex for Idx<3963> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3963> as NatIndex>::Out as Nat>::VAL == 3963);
impl NatIndex for Idx<3964> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3964> as NatIndex>::Out as Nat>::VAL == 3964);
impl NatIndex for Idx<3965> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3965> as NatIndex>::Out as Nat>::VAL == 3965);
impl NatIndex for Idx<3966> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3966> as NatIndex>::Out as Nat>::VAL == 3966);
impl NatIndex for Idx<3967> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3967> as NatIndex>::Out as Nat>::VAL == 3967);
impl NatIndex for Idx<3968> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3968> as NatIndex>::Out as Nat>::VAL == 3968);
impl NatIndex for Idx<3969> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3969> as NatIndex>::Out as Nat>::VAL == 3969);
impl NatIndex for Idx<3970> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3970> as NatIndex>::Out as Nat>::VAL == 3970);
impl NatIndex for Idx<3971> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3971> as NatIndex>::Out as Nat>::VAL == 3971);
impl NatIndex for Idx<3972> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3972> as NatIndex>::Out as Nat>::VAL == 3972);
impl NatIndex for Idx<3973> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3973> as NatIndex>::Out as Nat>::VAL == 3973);
impl NatIndex for Idx<3974> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3974> as NatIndex>::Out as Nat>::VAL == 3974);
impl NatIndex for Idx<3975> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3975> as NatIndex>::Out as Nat>::VAL == 3975);
impl NatIndex for Idx<3976> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3976> as NatIndex>::Out as Nat>::VAL == 3976);
impl NatIndex for Idx<3977> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3977> as NatIndex>::Out as Nat>::VAL == 3977);
impl NatIndex for Idx<3978> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3978> as NatIndex>::Out as Nat>::VAL == 3978);
impl NatIndex for Idx<3979> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3979> as NatIndex>::Out as Nat>::VAL == 3979);
impl NatIndex for Idx<3980> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3980> as NatIndex>::Out as Nat>::VAL == 3980);
impl NatIndex for Idx<3981> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3981> as NatIndex>::Out as Nat>::VAL == 3981);
impl NatIndex for Idx<3982> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3982> as NatIndex>::Out as Nat>::VAL == 3982);
impl NatIndex for Idx<3983> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3983> as NatIndex>::Out as Nat>::VAL == 3983);
impl NatIndex for Idx<3984> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3984> as NatIndex>::Out as Nat>::VAL == 3984);
impl NatIndex for Idx<3985> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3985> as NatIndex>::Out as Nat>::VAL == 3985);
impl NatIndex for Idx<3986> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3986> as NatIndex>::Out as Nat>::VAL == 3986);
impl NatIndex for Idx<3987> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3987> as NatIndex>::Out as Nat>::VAL == 3987);
impl NatIndex for Idx<3988> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3988> as NatIndex>::Out as Nat>::VAL == 3988);
impl NatIndex for Idx<3989> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3989> as NatIndex>::Out as Nat>::VAL == 3989);
impl NatIndex for Idx<3990> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3990> as NatIndex>::Out as Nat>::VAL == 3990);
impl NatIndex for Idx<3991> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3991> as NatIndex>::Out as Nat>::VAL == 3991);
impl NatIndex for Idx<3992> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3992> as NatIndex>::Out as Nat>::VAL == 3992);
impl NatIndex for Idx<3993> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3993> as NatIndex>::Out as Nat>::VAL == 3993);
impl NatIndex for Idx<3994> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3994> as NatIndex>::Out as Nat>::VAL == 3994);
impl NatIndex for Idx<3995> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3995> as NatIndex>::Out as Nat>::VAL == 3995);
impl NatIndex for Idx<3996> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3996> as NatIndex>::Out as Nat>::VAL == 3996);
impl NatIndex for Idx<3997> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3997> as NatIndex>::Out as Nat>::VAL == 3997);
impl NatIndex for Idx<3998> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3998> as NatIndex>::Out as Nat>::VAL == 3998);
impl NatIndex for Idx<3999> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<3999> as NatIndex>::Out as Nat>::VAL == 3999);
impl NatIndex for Idx<4000> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4000> as NatIndex>::Out as Nat>::VAL == 4000);
impl NatIndex for Idx<4001> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4001> as NatIndex>::Out as Nat>::VAL == 4001);
impl NatIndex for Idx<4002> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4002> as NatIndex>::Out as Nat>::VAL == 4002);
impl NatIndex for Idx<4003> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4003> as NatIndex>::Out as Nat>::VAL == 4003);
impl NatIndex for Idx<4004> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4004> as NatIndex>::Out as Nat>::VAL == 4004);
impl NatIndex for Idx<4005> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4005> as NatIndex>::Out as Nat>::VAL == 4005);
impl NatIndex for Idx<4006> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4006> as NatIndex>::Out as Nat>::VAL == 4006);
impl NatIndex for Idx<4007> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4007> as NatIndex>::Out as Nat>::VAL == 4007);
impl NatIndex for Idx<4008> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4008> as NatIndex>::Out as Nat>::VAL == 4008);
impl NatIndex for Idx<4009> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4009> as NatIndex>::Out as Nat>::VAL == 4009);
impl NatIndex for Idx<4010> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4010> as NatIndex>::Out as Nat>::VAL == 4010);
impl NatIndex for Idx<4011> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4011> as NatIndex>::Out as Nat>::VAL == 4011);
impl NatIndex for Idx<4012> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4012> as NatIndex>::Out as Nat>::VAL == 4012);
impl NatIndex for Idx<4013> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4013> as NatIndex>::Out as Nat>::VAL == 4013);
impl NatIndex for Idx<4014> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4014> as NatIndex>::Out as Nat>::VAL == 4014);
impl NatIndex for Idx<4015> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4015> as NatIndex>::Out as Nat>::VAL == 4015);
impl NatIndex for Idx<4016> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4016> as NatIndex>::Out as Nat>::VAL == 4016);
impl NatIndex for Idx<4017> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4017> as NatIndex>::Out as Nat>::VAL == 4017);
impl NatIndex for Idx<4018> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4018> as NatIndex>::Out as Nat>::VAL == 4018);
impl NatIndex for Idx<4019> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4019> as NatIndex>::Out as Nat>::VAL == 4019);
impl NatIndex for Idx<4020> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4020> as NatIndex>::Out as Nat>::VAL == 4020);
impl NatIndex for Idx<4021> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4021> as NatIndex>::Out as Nat>::VAL == 4021);
impl NatIndex for Idx<4022> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4022> as NatIndex>::Out as Nat>::VAL == 4022);
impl NatIndex for Idx<4023> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4023> as NatIndex>::Out as Nat>::VAL == 4023);
impl NatIndex for Idx<4024> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4024> as NatIndex>::Out as Nat>::VAL == 4024);
impl NatIndex for Idx<4025> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4025> as NatIndex>::Out as Nat>::VAL == 4025);
impl NatIndex for Idx<4026> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4026> as NatIndex>::Out as Nat>::VAL == 4026);
impl NatIndex for Idx<4027> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4027> as NatIndex>::Out as Nat>::VAL == 4027);
impl NatIndex for Idx<4028> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4028> as NatIndex>::Out as Nat>::VAL == 4028);
impl NatIndex for Idx<4029> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4029> as NatIndex>::Out as Nat>::VAL == 4029);
impl NatIndex for Idx<4030> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4030> as NatIndex>::Out as Nat>::VAL == 4030);
impl NatIndex for Idx<4031> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4031> as NatIndex>::Out as Nat>::VAL == 4031);
impl NatIndex for Idx<4032> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4032> as NatIndex>::Out as Nat>::VAL == 4032);
impl NatIndex for Idx<4033> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4033> as NatIndex>::Out as Nat>::VAL == 4033);
impl NatIndex for Idx<4034> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4034> as NatIndex>::Out as Nat>::VAL == 4034);
impl NatIndex for Idx<4035> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4035> as NatIndex>::Out as Nat>::VAL == 4035);
impl NatIndex for Idx<4036> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4036> as NatIndex>::Out as Nat>::VAL == 4036);
impl NatIndex for Idx<4037> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4037> as NatIndex>::Out as Nat>::VAL == 4037);
impl NatIndex for Idx<4038> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4038> as NatIndex>::Out as Nat>::VAL == 4038);
impl NatIndex for Idx<4039> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4039> as NatIndex>::Out as Nat>::VAL == 4039);
impl NatIndex for Idx<4040> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4040> as NatIndex>::Out as Nat>::VAL == 4040);
impl NatIndex for Idx<4041> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4041> as NatIndex>::Out as Nat>::VAL == 4041);
impl NatIndex for Idx<4042> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4042> as NatIndex>::Out as Nat>::VAL == 4042);
impl NatIndex for Idx<4043> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4043> as NatIndex>::Out as Nat>::VAL == 4043);
impl NatIndex for Idx<4044> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4044> as NatIndex>::Out as Nat>::VAL == 4044);
impl NatIndex for Idx<4045> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4045> as NatIndex>::Out as Nat>::VAL == 4045);
impl NatIndex for Idx<4046> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4046> as NatIndex>::Out as Nat>::VAL == 4046);
impl NatIndex for Idx<4047> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4047> as NatIndex>::Out as Nat>::VAL == 4047);
impl NatIndex for Idx<4048> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4048> as NatIndex>::Out as Nat>::VAL == 4048);
impl NatIndex for Idx<4049> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4049> as NatIndex>::Out as Nat>::VAL == 4049);
impl NatIndex for Idx<4050> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4050> as NatIndex>::Out as Nat>::VAL == 4050);
impl NatIndex for Idx<4051> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4051> as NatIndex>::Out as Nat>::VAL == 4051);
impl NatIndex for Idx<4052> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4052> as NatIndex>::Out as Nat>::VAL == 4052);
impl NatIndex for Idx<4053> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4053> as NatIndex>::Out as Nat>::VAL == 4053);
impl NatIndex for Idx<4054> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4054> as NatIndex>::Out as Nat>::VAL == 4054);
impl NatIndex for Idx<4055> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4055> as NatIndex>::Out as Nat>::VAL == 4055);
impl NatIndex for Idx<4056> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4056> as NatIndex>::Out as Nat>::VAL == 4056);
impl NatIndex for Idx<4057> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4057> as NatIndex>::Out as Nat>::VAL == 4057);
impl NatIndex for Idx<4058> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4058> as NatIndex>::Out as Nat>::VAL == 4058);
impl NatIndex for Idx<4059> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4059> as NatIndex>::Out as Nat>::VAL == 4059);
impl NatIndex for Idx<4060> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4060> as NatIndex>::Out as Nat>::VAL == 4060);
impl NatIndex for Idx<4061> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4061> as NatIndex>::Out as Nat>::VAL == 4061);
impl NatIndex for Idx<4062> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4062> as NatIndex>::Out as Nat>::VAL == 4062);
impl NatIndex for Idx<4063> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4063> as NatIndex>::Out as Nat>::VAL == 4063);
impl NatIndex for Idx<4064> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4064> as NatIndex>::Out as Nat>::VAL == 4064);
impl NatIndex for Idx<4065> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4065> as NatIndex>::Out as Nat>::VAL == 4065);
impl NatIndex for Idx<4066> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4066> as NatIndex>::Out as Nat>::VAL == 4066);
impl NatIndex for Idx<4067> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4067> as NatIndex>::Out as Nat>::VAL == 4067);
impl NatIndex for Idx<4068> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4068> as NatIndex>::Out as Nat>::VAL == 4068);
impl NatIndex for Idx<4069> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4069> as NatIndex>::Out as Nat>::VAL == 4069);
impl NatIndex for Idx<4070> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4070> as NatIndex>::Out as Nat>::VAL == 4070);
impl NatIndex for Idx<4071> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4071> as NatIndex>::Out as Nat>::VAL == 4071);
impl NatIndex for Idx<4072> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4072> as NatIndex>::Out as Nat>::VAL == 4072);
impl NatIndex for Idx<4073> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4073> as NatIndex>::Out as Nat>::VAL == 4073);
impl NatIndex for Idx<4074> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4074> as NatIndex>::Out as Nat>::VAL == 4074);
impl NatIndex for Idx<4075> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4075> as NatIndex>::Out as Nat>::VAL == 4075);
impl NatIndex for Idx<4076> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4076> as NatIndex>::Out as Nat>::VAL == 4076);
impl NatIndex for Idx<4077> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4077> as NatIndex>::Out as Nat>::VAL == 4077);
impl NatIndex for Idx<4078> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4078> as NatIndex>::Out as Nat>::VAL == 4078);
impl NatIndex for Idx<4079> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4079> as NatIndex>::Out as Nat>::VAL == 4079);
impl NatIndex for Idx<4080> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4080> as NatIndex>::Out as Nat>::VAL == 4080);
impl NatIndex for Idx<4081> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4081> as NatIndex>::Out as Nat>::VAL == 4081);
impl NatIndex for Idx<4082> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4082> as NatIndex>::Out as Nat>::VAL == 4082);
impl NatIndex for Idx<4083> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4083> as NatIndex>::Out as Nat>::VAL == 4083);
impl NatIndex for Idx<4084> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4084> as NatIndex>::Out as Nat>::VAL == 4084);
impl NatIndex for Idx<4085> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4085> as NatIndex>::Out as Nat>::VAL == 4085);
impl NatIndex for Idx<4086> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4086> as NatIndex>::Out as Nat>::VAL == 4086);
impl NatIndex for Idx<4087> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4087> as NatIndex>::Out as Nat>::VAL == 4087);
impl NatIndex for Idx<4088> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4088> as NatIndex>::Out as Nat>::VAL == 4088);
impl NatIndex for Idx<4089> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4089> as NatIndex>::Out as Nat>::VAL == 4089);
impl NatIndex for Idx<4090> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4090> as NatIndex>::Out as Nat>::VAL == 4090);
impl NatIndex for Idx<4091> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4091> as NatIndex>::Out as Nat>::VAL == 4091);
impl NatIndex for Idx<4092> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4092> as NatIndex>::Out as Nat>::VAL == 4092);
impl NatIndex for Idx<4093> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4093> as NatIndex>::Out as Nat>::VAL == 4093);
impl NatIndex for Idx<4094> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4094> as NatIndex>::Out as Nat>::VAL == 4094);
impl NatIndex for Idx<4095> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
const _: () = assert!(<<Idx<4095> as NatIndex>::Out as Nat>::VAL == 4095);

// the load-bearing claim
const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
