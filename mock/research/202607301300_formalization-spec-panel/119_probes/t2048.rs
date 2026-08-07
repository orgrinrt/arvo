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
impl NatIndex for Idx<1> {
    type Out = Pz<H>;
}
impl NatIndex for Idx<2> {
    type Out = Pz<O<H>>;
}
impl NatIndex for Idx<3> {
    type Out = Pz<I<H>>;
}
impl NatIndex for Idx<4> {
    type Out = Pz<O<O<H>>>;
}
impl NatIndex for Idx<5> {
    type Out = Pz<I<O<H>>>;
}
impl NatIndex for Idx<6> {
    type Out = Pz<O<I<H>>>;
}
impl NatIndex for Idx<7> {
    type Out = Pz<I<I<H>>>;
}
impl NatIndex for Idx<8> {
    type Out = Pz<O<O<O<H>>>>;
}
impl NatIndex for Idx<9> {
    type Out = Pz<I<O<O<H>>>>;
}
impl NatIndex for Idx<10> {
    type Out = Pz<O<I<O<H>>>>;
}
impl NatIndex for Idx<11> {
    type Out = Pz<I<I<O<H>>>>;
}
impl NatIndex for Idx<12> {
    type Out = Pz<O<O<I<H>>>>;
}
impl NatIndex for Idx<13> {
    type Out = Pz<I<O<I<H>>>>;
}
impl NatIndex for Idx<14> {
    type Out = Pz<O<I<I<H>>>>;
}
impl NatIndex for Idx<15> {
    type Out = Pz<I<I<I<H>>>>;
}
impl NatIndex for Idx<16> {
    type Out = Pz<O<O<O<O<H>>>>>;
}
impl NatIndex for Idx<17> {
    type Out = Pz<I<O<O<O<H>>>>>;
}
impl NatIndex for Idx<18> {
    type Out = Pz<O<I<O<O<H>>>>>;
}
impl NatIndex for Idx<19> {
    type Out = Pz<I<I<O<O<H>>>>>;
}
impl NatIndex for Idx<20> {
    type Out = Pz<O<O<I<O<H>>>>>;
}
impl NatIndex for Idx<21> {
    type Out = Pz<I<O<I<O<H>>>>>;
}
impl NatIndex for Idx<22> {
    type Out = Pz<O<I<I<O<H>>>>>;
}
impl NatIndex for Idx<23> {
    type Out = Pz<I<I<I<O<H>>>>>;
}
impl NatIndex for Idx<24> {
    type Out = Pz<O<O<O<I<H>>>>>;
}
impl NatIndex for Idx<25> {
    type Out = Pz<I<O<O<I<H>>>>>;
}
impl NatIndex for Idx<26> {
    type Out = Pz<O<I<O<I<H>>>>>;
}
impl NatIndex for Idx<27> {
    type Out = Pz<I<I<O<I<H>>>>>;
}
impl NatIndex for Idx<28> {
    type Out = Pz<O<O<I<I<H>>>>>;
}
impl NatIndex for Idx<29> {
    type Out = Pz<I<O<I<I<H>>>>>;
}
impl NatIndex for Idx<30> {
    type Out = Pz<O<I<I<I<H>>>>>;
}
impl NatIndex for Idx<31> {
    type Out = Pz<I<I<I<I<H>>>>>;
}
impl NatIndex for Idx<32> {
    type Out = Pz<O<O<O<O<O<H>>>>>>;
}
impl NatIndex for Idx<33> {
    type Out = Pz<I<O<O<O<O<H>>>>>>;
}
impl NatIndex for Idx<34> {
    type Out = Pz<O<I<O<O<O<H>>>>>>;
}
impl NatIndex for Idx<35> {
    type Out = Pz<I<I<O<O<O<H>>>>>>;
}
impl NatIndex for Idx<36> {
    type Out = Pz<O<O<I<O<O<H>>>>>>;
}
impl NatIndex for Idx<37> {
    type Out = Pz<I<O<I<O<O<H>>>>>>;
}
impl NatIndex for Idx<38> {
    type Out = Pz<O<I<I<O<O<H>>>>>>;
}
impl NatIndex for Idx<39> {
    type Out = Pz<I<I<I<O<O<H>>>>>>;
}
impl NatIndex for Idx<40> {
    type Out = Pz<O<O<O<I<O<H>>>>>>;
}
impl NatIndex for Idx<41> {
    type Out = Pz<I<O<O<I<O<H>>>>>>;
}
impl NatIndex for Idx<42> {
    type Out = Pz<O<I<O<I<O<H>>>>>>;
}
impl NatIndex for Idx<43> {
    type Out = Pz<I<I<O<I<O<H>>>>>>;
}
impl NatIndex for Idx<44> {
    type Out = Pz<O<O<I<I<O<H>>>>>>;
}
impl NatIndex for Idx<45> {
    type Out = Pz<I<O<I<I<O<H>>>>>>;
}
impl NatIndex for Idx<46> {
    type Out = Pz<O<I<I<I<O<H>>>>>>;
}
impl NatIndex for Idx<47> {
    type Out = Pz<I<I<I<I<O<H>>>>>>;
}
impl NatIndex for Idx<48> {
    type Out = Pz<O<O<O<O<I<H>>>>>>;
}
impl NatIndex for Idx<49> {
    type Out = Pz<I<O<O<O<I<H>>>>>>;
}
impl NatIndex for Idx<50> {
    type Out = Pz<O<I<O<O<I<H>>>>>>;
}
impl NatIndex for Idx<51> {
    type Out = Pz<I<I<O<O<I<H>>>>>>;
}
impl NatIndex for Idx<52> {
    type Out = Pz<O<O<I<O<I<H>>>>>>;
}
impl NatIndex for Idx<53> {
    type Out = Pz<I<O<I<O<I<H>>>>>>;
}
impl NatIndex for Idx<54> {
    type Out = Pz<O<I<I<O<I<H>>>>>>;
}
impl NatIndex for Idx<55> {
    type Out = Pz<I<I<I<O<I<H>>>>>>;
}
impl NatIndex for Idx<56> {
    type Out = Pz<O<O<O<I<I<H>>>>>>;
}
impl NatIndex for Idx<57> {
    type Out = Pz<I<O<O<I<I<H>>>>>>;
}
impl NatIndex for Idx<58> {
    type Out = Pz<O<I<O<I<I<H>>>>>>;
}
impl NatIndex for Idx<59> {
    type Out = Pz<I<I<O<I<I<H>>>>>>;
}
impl NatIndex for Idx<60> {
    type Out = Pz<O<O<I<I<I<H>>>>>>;
}
impl NatIndex for Idx<61> {
    type Out = Pz<I<O<I<I<I<H>>>>>>;
}
impl NatIndex for Idx<62> {
    type Out = Pz<O<I<I<I<I<H>>>>>>;
}
impl NatIndex for Idx<63> {
    type Out = Pz<I<I<I<I<I<H>>>>>>;
}
impl NatIndex for Idx<64> {
    type Out = Pz<O<O<O<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<65> {
    type Out = Pz<I<O<O<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<66> {
    type Out = Pz<O<I<O<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<67> {
    type Out = Pz<I<I<O<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<68> {
    type Out = Pz<O<O<I<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<69> {
    type Out = Pz<I<O<I<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<70> {
    type Out = Pz<O<I<I<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<71> {
    type Out = Pz<I<I<I<O<O<O<H>>>>>>>;
}
impl NatIndex for Idx<72> {
    type Out = Pz<O<O<O<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<73> {
    type Out = Pz<I<O<O<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<74> {
    type Out = Pz<O<I<O<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<75> {
    type Out = Pz<I<I<O<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<76> {
    type Out = Pz<O<O<I<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<77> {
    type Out = Pz<I<O<I<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<78> {
    type Out = Pz<O<I<I<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<79> {
    type Out = Pz<I<I<I<I<O<O<H>>>>>>>;
}
impl NatIndex for Idx<80> {
    type Out = Pz<O<O<O<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<81> {
    type Out = Pz<I<O<O<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<82> {
    type Out = Pz<O<I<O<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<83> {
    type Out = Pz<I<I<O<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<84> {
    type Out = Pz<O<O<I<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<85> {
    type Out = Pz<I<O<I<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<86> {
    type Out = Pz<O<I<I<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<87> {
    type Out = Pz<I<I<I<O<I<O<H>>>>>>>;
}
impl NatIndex for Idx<88> {
    type Out = Pz<O<O<O<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<89> {
    type Out = Pz<I<O<O<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<90> {
    type Out = Pz<O<I<O<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<91> {
    type Out = Pz<I<I<O<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<92> {
    type Out = Pz<O<O<I<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<93> {
    type Out = Pz<I<O<I<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<94> {
    type Out = Pz<O<I<I<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<95> {
    type Out = Pz<I<I<I<I<I<O<H>>>>>>>;
}
impl NatIndex for Idx<96> {
    type Out = Pz<O<O<O<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<97> {
    type Out = Pz<I<O<O<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<98> {
    type Out = Pz<O<I<O<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<99> {
    type Out = Pz<I<I<O<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<100> {
    type Out = Pz<O<O<I<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<101> {
    type Out = Pz<I<O<I<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<102> {
    type Out = Pz<O<I<I<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<103> {
    type Out = Pz<I<I<I<O<O<I<H>>>>>>>;
}
impl NatIndex for Idx<104> {
    type Out = Pz<O<O<O<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<105> {
    type Out = Pz<I<O<O<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<106> {
    type Out = Pz<O<I<O<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<107> {
    type Out = Pz<I<I<O<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<108> {
    type Out = Pz<O<O<I<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<109> {
    type Out = Pz<I<O<I<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<110> {
    type Out = Pz<O<I<I<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<111> {
    type Out = Pz<I<I<I<I<O<I<H>>>>>>>;
}
impl NatIndex for Idx<112> {
    type Out = Pz<O<O<O<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<113> {
    type Out = Pz<I<O<O<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<114> {
    type Out = Pz<O<I<O<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<115> {
    type Out = Pz<I<I<O<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<116> {
    type Out = Pz<O<O<I<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<117> {
    type Out = Pz<I<O<I<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<118> {
    type Out = Pz<O<I<I<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<119> {
    type Out = Pz<I<I<I<O<I<I<H>>>>>>>;
}
impl NatIndex for Idx<120> {
    type Out = Pz<O<O<O<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<121> {
    type Out = Pz<I<O<O<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<122> {
    type Out = Pz<O<I<O<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<123> {
    type Out = Pz<I<I<O<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<124> {
    type Out = Pz<O<O<I<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<125> {
    type Out = Pz<I<O<I<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<126> {
    type Out = Pz<O<I<I<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<127> {
    type Out = Pz<I<I<I<I<I<I<H>>>>>>>;
}
impl NatIndex for Idx<128> {
    type Out = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<129> {
    type Out = Pz<I<O<O<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<130> {
    type Out = Pz<O<I<O<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<131> {
    type Out = Pz<I<I<O<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<132> {
    type Out = Pz<O<O<I<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<133> {
    type Out = Pz<I<O<I<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<134> {
    type Out = Pz<O<I<I<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<135> {
    type Out = Pz<I<I<I<O<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<136> {
    type Out = Pz<O<O<O<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<137> {
    type Out = Pz<I<O<O<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<138> {
    type Out = Pz<O<I<O<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<139> {
    type Out = Pz<I<I<O<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<140> {
    type Out = Pz<O<O<I<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<141> {
    type Out = Pz<I<O<I<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<142> {
    type Out = Pz<O<I<I<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<143> {
    type Out = Pz<I<I<I<I<O<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<144> {
    type Out = Pz<O<O<O<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<145> {
    type Out = Pz<I<O<O<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<146> {
    type Out = Pz<O<I<O<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<147> {
    type Out = Pz<I<I<O<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<148> {
    type Out = Pz<O<O<I<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<149> {
    type Out = Pz<I<O<I<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<150> {
    type Out = Pz<O<I<I<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<151> {
    type Out = Pz<I<I<I<O<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<152> {
    type Out = Pz<O<O<O<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<153> {
    type Out = Pz<I<O<O<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<154> {
    type Out = Pz<O<I<O<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<155> {
    type Out = Pz<I<I<O<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<156> {
    type Out = Pz<O<O<I<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<157> {
    type Out = Pz<I<O<I<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<158> {
    type Out = Pz<O<I<I<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<159> {
    type Out = Pz<I<I<I<I<I<O<O<H>>>>>>>>;
}
impl NatIndex for Idx<160> {
    type Out = Pz<O<O<O<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<161> {
    type Out = Pz<I<O<O<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<162> {
    type Out = Pz<O<I<O<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<163> {
    type Out = Pz<I<I<O<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<164> {
    type Out = Pz<O<O<I<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<165> {
    type Out = Pz<I<O<I<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<166> {
    type Out = Pz<O<I<I<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<167> {
    type Out = Pz<I<I<I<O<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<168> {
    type Out = Pz<O<O<O<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<169> {
    type Out = Pz<I<O<O<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<170> {
    type Out = Pz<O<I<O<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<171> {
    type Out = Pz<I<I<O<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<172> {
    type Out = Pz<O<O<I<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<173> {
    type Out = Pz<I<O<I<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<174> {
    type Out = Pz<O<I<I<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<175> {
    type Out = Pz<I<I<I<I<O<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<176> {
    type Out = Pz<O<O<O<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<177> {
    type Out = Pz<I<O<O<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<178> {
    type Out = Pz<O<I<O<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<179> {
    type Out = Pz<I<I<O<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<180> {
    type Out = Pz<O<O<I<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<181> {
    type Out = Pz<I<O<I<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<182> {
    type Out = Pz<O<I<I<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<183> {
    type Out = Pz<I<I<I<O<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<184> {
    type Out = Pz<O<O<O<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<185> {
    type Out = Pz<I<O<O<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<186> {
    type Out = Pz<O<I<O<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<187> {
    type Out = Pz<I<I<O<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<188> {
    type Out = Pz<O<O<I<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<189> {
    type Out = Pz<I<O<I<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<190> {
    type Out = Pz<O<I<I<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<191> {
    type Out = Pz<I<I<I<I<I<I<O<H>>>>>>>>;
}
impl NatIndex for Idx<192> {
    type Out = Pz<O<O<O<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<193> {
    type Out = Pz<I<O<O<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<194> {
    type Out = Pz<O<I<O<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<195> {
    type Out = Pz<I<I<O<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<196> {
    type Out = Pz<O<O<I<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<197> {
    type Out = Pz<I<O<I<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<198> {
    type Out = Pz<O<I<I<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<199> {
    type Out = Pz<I<I<I<O<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<200> {
    type Out = Pz<O<O<O<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<201> {
    type Out = Pz<I<O<O<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<202> {
    type Out = Pz<O<I<O<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<203> {
    type Out = Pz<I<I<O<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<204> {
    type Out = Pz<O<O<I<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<205> {
    type Out = Pz<I<O<I<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<206> {
    type Out = Pz<O<I<I<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<207> {
    type Out = Pz<I<I<I<I<O<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<208> {
    type Out = Pz<O<O<O<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<209> {
    type Out = Pz<I<O<O<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<210> {
    type Out = Pz<O<I<O<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<211> {
    type Out = Pz<I<I<O<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<212> {
    type Out = Pz<O<O<I<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<213> {
    type Out = Pz<I<O<I<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<214> {
    type Out = Pz<O<I<I<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<215> {
    type Out = Pz<I<I<I<O<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<216> {
    type Out = Pz<O<O<O<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<217> {
    type Out = Pz<I<O<O<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<218> {
    type Out = Pz<O<I<O<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<219> {
    type Out = Pz<I<I<O<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<220> {
    type Out = Pz<O<O<I<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<221> {
    type Out = Pz<I<O<I<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<222> {
    type Out = Pz<O<I<I<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<223> {
    type Out = Pz<I<I<I<I<I<O<I<H>>>>>>>>;
}
impl NatIndex for Idx<224> {
    type Out = Pz<O<O<O<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<225> {
    type Out = Pz<I<O<O<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<226> {
    type Out = Pz<O<I<O<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<227> {
    type Out = Pz<I<I<O<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<228> {
    type Out = Pz<O<O<I<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<229> {
    type Out = Pz<I<O<I<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<230> {
    type Out = Pz<O<I<I<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<231> {
    type Out = Pz<I<I<I<O<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<232> {
    type Out = Pz<O<O<O<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<233> {
    type Out = Pz<I<O<O<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<234> {
    type Out = Pz<O<I<O<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<235> {
    type Out = Pz<I<I<O<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<236> {
    type Out = Pz<O<O<I<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<237> {
    type Out = Pz<I<O<I<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<238> {
    type Out = Pz<O<I<I<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<239> {
    type Out = Pz<I<I<I<I<O<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<240> {
    type Out = Pz<O<O<O<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<241> {
    type Out = Pz<I<O<O<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<242> {
    type Out = Pz<O<I<O<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<243> {
    type Out = Pz<I<I<O<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<244> {
    type Out = Pz<O<O<I<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<245> {
    type Out = Pz<I<O<I<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<246> {
    type Out = Pz<O<I<I<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<247> {
    type Out = Pz<I<I<I<O<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<248> {
    type Out = Pz<O<O<O<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<249> {
    type Out = Pz<I<O<O<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<250> {
    type Out = Pz<O<I<O<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<251> {
    type Out = Pz<I<I<O<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<252> {
    type Out = Pz<O<O<I<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<253> {
    type Out = Pz<I<O<I<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<254> {
    type Out = Pz<O<I<I<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<255> {
    type Out = Pz<I<I<I<I<I<I<I<H>>>>>>>>;
}
impl NatIndex for Idx<256> {
    type Out = Pz<O<O<O<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<257> {
    type Out = Pz<I<O<O<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<258> {
    type Out = Pz<O<I<O<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<259> {
    type Out = Pz<I<I<O<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<260> {
    type Out = Pz<O<O<I<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<261> {
    type Out = Pz<I<O<I<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<262> {
    type Out = Pz<O<I<I<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<263> {
    type Out = Pz<I<I<I<O<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<264> {
    type Out = Pz<O<O<O<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<265> {
    type Out = Pz<I<O<O<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<266> {
    type Out = Pz<O<I<O<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<267> {
    type Out = Pz<I<I<O<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<268> {
    type Out = Pz<O<O<I<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<269> {
    type Out = Pz<I<O<I<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<270> {
    type Out = Pz<O<I<I<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<271> {
    type Out = Pz<I<I<I<I<O<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<272> {
    type Out = Pz<O<O<O<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<273> {
    type Out = Pz<I<O<O<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<274> {
    type Out = Pz<O<I<O<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<275> {
    type Out = Pz<I<I<O<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<276> {
    type Out = Pz<O<O<I<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<277> {
    type Out = Pz<I<O<I<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<278> {
    type Out = Pz<O<I<I<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<279> {
    type Out = Pz<I<I<I<O<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<280> {
    type Out = Pz<O<O<O<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<281> {
    type Out = Pz<I<O<O<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<282> {
    type Out = Pz<O<I<O<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<283> {
    type Out = Pz<I<I<O<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<284> {
    type Out = Pz<O<O<I<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<285> {
    type Out = Pz<I<O<I<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<286> {
    type Out = Pz<O<I<I<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<287> {
    type Out = Pz<I<I<I<I<I<O<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<288> {
    type Out = Pz<O<O<O<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<289> {
    type Out = Pz<I<O<O<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<290> {
    type Out = Pz<O<I<O<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<291> {
    type Out = Pz<I<I<O<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<292> {
    type Out = Pz<O<O<I<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<293> {
    type Out = Pz<I<O<I<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<294> {
    type Out = Pz<O<I<I<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<295> {
    type Out = Pz<I<I<I<O<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<296> {
    type Out = Pz<O<O<O<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<297> {
    type Out = Pz<I<O<O<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<298> {
    type Out = Pz<O<I<O<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<299> {
    type Out = Pz<I<I<O<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<300> {
    type Out = Pz<O<O<I<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<301> {
    type Out = Pz<I<O<I<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<302> {
    type Out = Pz<O<I<I<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<303> {
    type Out = Pz<I<I<I<I<O<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<304> {
    type Out = Pz<O<O<O<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<305> {
    type Out = Pz<I<O<O<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<306> {
    type Out = Pz<O<I<O<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<307> {
    type Out = Pz<I<I<O<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<308> {
    type Out = Pz<O<O<I<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<309> {
    type Out = Pz<I<O<I<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<310> {
    type Out = Pz<O<I<I<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<311> {
    type Out = Pz<I<I<I<O<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<312> {
    type Out = Pz<O<O<O<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<313> {
    type Out = Pz<I<O<O<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<314> {
    type Out = Pz<O<I<O<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<315> {
    type Out = Pz<I<I<O<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<316> {
    type Out = Pz<O<O<I<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<317> {
    type Out = Pz<I<O<I<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<318> {
    type Out = Pz<O<I<I<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<319> {
    type Out = Pz<I<I<I<I<I<I<O<O<H>>>>>>>>>;
}
impl NatIndex for Idx<320> {
    type Out = Pz<O<O<O<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<321> {
    type Out = Pz<I<O<O<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<322> {
    type Out = Pz<O<I<O<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<323> {
    type Out = Pz<I<I<O<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<324> {
    type Out = Pz<O<O<I<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<325> {
    type Out = Pz<I<O<I<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<326> {
    type Out = Pz<O<I<I<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<327> {
    type Out = Pz<I<I<I<O<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<328> {
    type Out = Pz<O<O<O<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<329> {
    type Out = Pz<I<O<O<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<330> {
    type Out = Pz<O<I<O<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<331> {
    type Out = Pz<I<I<O<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<332> {
    type Out = Pz<O<O<I<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<333> {
    type Out = Pz<I<O<I<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<334> {
    type Out = Pz<O<I<I<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<335> {
    type Out = Pz<I<I<I<I<O<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<336> {
    type Out = Pz<O<O<O<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<337> {
    type Out = Pz<I<O<O<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<338> {
    type Out = Pz<O<I<O<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<339> {
    type Out = Pz<I<I<O<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<340> {
    type Out = Pz<O<O<I<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<341> {
    type Out = Pz<I<O<I<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<342> {
    type Out = Pz<O<I<I<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<343> {
    type Out = Pz<I<I<I<O<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<344> {
    type Out = Pz<O<O<O<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<345> {
    type Out = Pz<I<O<O<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<346> {
    type Out = Pz<O<I<O<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<347> {
    type Out = Pz<I<I<O<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<348> {
    type Out = Pz<O<O<I<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<349> {
    type Out = Pz<I<O<I<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<350> {
    type Out = Pz<O<I<I<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<351> {
    type Out = Pz<I<I<I<I<I<O<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<352> {
    type Out = Pz<O<O<O<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<353> {
    type Out = Pz<I<O<O<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<354> {
    type Out = Pz<O<I<O<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<355> {
    type Out = Pz<I<I<O<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<356> {
    type Out = Pz<O<O<I<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<357> {
    type Out = Pz<I<O<I<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<358> {
    type Out = Pz<O<I<I<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<359> {
    type Out = Pz<I<I<I<O<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<360> {
    type Out = Pz<O<O<O<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<361> {
    type Out = Pz<I<O<O<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<362> {
    type Out = Pz<O<I<O<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<363> {
    type Out = Pz<I<I<O<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<364> {
    type Out = Pz<O<O<I<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<365> {
    type Out = Pz<I<O<I<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<366> {
    type Out = Pz<O<I<I<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<367> {
    type Out = Pz<I<I<I<I<O<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<368> {
    type Out = Pz<O<O<O<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<369> {
    type Out = Pz<I<O<O<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<370> {
    type Out = Pz<O<I<O<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<371> {
    type Out = Pz<I<I<O<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<372> {
    type Out = Pz<O<O<I<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<373> {
    type Out = Pz<I<O<I<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<374> {
    type Out = Pz<O<I<I<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<375> {
    type Out = Pz<I<I<I<O<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<376> {
    type Out = Pz<O<O<O<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<377> {
    type Out = Pz<I<O<O<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<378> {
    type Out = Pz<O<I<O<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<379> {
    type Out = Pz<I<I<O<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<380> {
    type Out = Pz<O<O<I<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<381> {
    type Out = Pz<I<O<I<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<382> {
    type Out = Pz<O<I<I<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<383> {
    type Out = Pz<I<I<I<I<I<I<I<O<H>>>>>>>>>;
}
impl NatIndex for Idx<384> {
    type Out = Pz<O<O<O<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<385> {
    type Out = Pz<I<O<O<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<386> {
    type Out = Pz<O<I<O<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<387> {
    type Out = Pz<I<I<O<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<388> {
    type Out = Pz<O<O<I<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<389> {
    type Out = Pz<I<O<I<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<390> {
    type Out = Pz<O<I<I<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<391> {
    type Out = Pz<I<I<I<O<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<392> {
    type Out = Pz<O<O<O<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<393> {
    type Out = Pz<I<O<O<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<394> {
    type Out = Pz<O<I<O<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<395> {
    type Out = Pz<I<I<O<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<396> {
    type Out = Pz<O<O<I<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<397> {
    type Out = Pz<I<O<I<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<398> {
    type Out = Pz<O<I<I<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<399> {
    type Out = Pz<I<I<I<I<O<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<400> {
    type Out = Pz<O<O<O<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<401> {
    type Out = Pz<I<O<O<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<402> {
    type Out = Pz<O<I<O<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<403> {
    type Out = Pz<I<I<O<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<404> {
    type Out = Pz<O<O<I<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<405> {
    type Out = Pz<I<O<I<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<406> {
    type Out = Pz<O<I<I<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<407> {
    type Out = Pz<I<I<I<O<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<408> {
    type Out = Pz<O<O<O<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<409> {
    type Out = Pz<I<O<O<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<410> {
    type Out = Pz<O<I<O<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<411> {
    type Out = Pz<I<I<O<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<412> {
    type Out = Pz<O<O<I<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<413> {
    type Out = Pz<I<O<I<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<414> {
    type Out = Pz<O<I<I<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<415> {
    type Out = Pz<I<I<I<I<I<O<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<416> {
    type Out = Pz<O<O<O<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<417> {
    type Out = Pz<I<O<O<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<418> {
    type Out = Pz<O<I<O<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<419> {
    type Out = Pz<I<I<O<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<420> {
    type Out = Pz<O<O<I<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<421> {
    type Out = Pz<I<O<I<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<422> {
    type Out = Pz<O<I<I<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<423> {
    type Out = Pz<I<I<I<O<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<424> {
    type Out = Pz<O<O<O<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<425> {
    type Out = Pz<I<O<O<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<426> {
    type Out = Pz<O<I<O<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<427> {
    type Out = Pz<I<I<O<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<428> {
    type Out = Pz<O<O<I<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<429> {
    type Out = Pz<I<O<I<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<430> {
    type Out = Pz<O<I<I<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<431> {
    type Out = Pz<I<I<I<I<O<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<432> {
    type Out = Pz<O<O<O<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<433> {
    type Out = Pz<I<O<O<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<434> {
    type Out = Pz<O<I<O<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<435> {
    type Out = Pz<I<I<O<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<436> {
    type Out = Pz<O<O<I<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<437> {
    type Out = Pz<I<O<I<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<438> {
    type Out = Pz<O<I<I<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<439> {
    type Out = Pz<I<I<I<O<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<440> {
    type Out = Pz<O<O<O<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<441> {
    type Out = Pz<I<O<O<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<442> {
    type Out = Pz<O<I<O<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<443> {
    type Out = Pz<I<I<O<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<444> {
    type Out = Pz<O<O<I<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<445> {
    type Out = Pz<I<O<I<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<446> {
    type Out = Pz<O<I<I<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<447> {
    type Out = Pz<I<I<I<I<I<I<O<I<H>>>>>>>>>;
}
impl NatIndex for Idx<448> {
    type Out = Pz<O<O<O<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<449> {
    type Out = Pz<I<O<O<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<450> {
    type Out = Pz<O<I<O<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<451> {
    type Out = Pz<I<I<O<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<452> {
    type Out = Pz<O<O<I<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<453> {
    type Out = Pz<I<O<I<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<454> {
    type Out = Pz<O<I<I<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<455> {
    type Out = Pz<I<I<I<O<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<456> {
    type Out = Pz<O<O<O<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<457> {
    type Out = Pz<I<O<O<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<458> {
    type Out = Pz<O<I<O<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<459> {
    type Out = Pz<I<I<O<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<460> {
    type Out = Pz<O<O<I<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<461> {
    type Out = Pz<I<O<I<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<462> {
    type Out = Pz<O<I<I<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<463> {
    type Out = Pz<I<I<I<I<O<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<464> {
    type Out = Pz<O<O<O<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<465> {
    type Out = Pz<I<O<O<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<466> {
    type Out = Pz<O<I<O<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<467> {
    type Out = Pz<I<I<O<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<468> {
    type Out = Pz<O<O<I<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<469> {
    type Out = Pz<I<O<I<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<470> {
    type Out = Pz<O<I<I<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<471> {
    type Out = Pz<I<I<I<O<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<472> {
    type Out = Pz<O<O<O<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<473> {
    type Out = Pz<I<O<O<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<474> {
    type Out = Pz<O<I<O<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<475> {
    type Out = Pz<I<I<O<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<476> {
    type Out = Pz<O<O<I<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<477> {
    type Out = Pz<I<O<I<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<478> {
    type Out = Pz<O<I<I<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<479> {
    type Out = Pz<I<I<I<I<I<O<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<480> {
    type Out = Pz<O<O<O<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<481> {
    type Out = Pz<I<O<O<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<482> {
    type Out = Pz<O<I<O<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<483> {
    type Out = Pz<I<I<O<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<484> {
    type Out = Pz<O<O<I<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<485> {
    type Out = Pz<I<O<I<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<486> {
    type Out = Pz<O<I<I<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<487> {
    type Out = Pz<I<I<I<O<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<488> {
    type Out = Pz<O<O<O<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<489> {
    type Out = Pz<I<O<O<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<490> {
    type Out = Pz<O<I<O<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<491> {
    type Out = Pz<I<I<O<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<492> {
    type Out = Pz<O<O<I<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<493> {
    type Out = Pz<I<O<I<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<494> {
    type Out = Pz<O<I<I<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<495> {
    type Out = Pz<I<I<I<I<O<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<496> {
    type Out = Pz<O<O<O<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<497> {
    type Out = Pz<I<O<O<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<498> {
    type Out = Pz<O<I<O<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<499> {
    type Out = Pz<I<I<O<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<500> {
    type Out = Pz<O<O<I<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<501> {
    type Out = Pz<I<O<I<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<502> {
    type Out = Pz<O<I<I<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<503> {
    type Out = Pz<I<I<I<O<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<504> {
    type Out = Pz<O<O<O<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<505> {
    type Out = Pz<I<O<O<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<506> {
    type Out = Pz<O<I<O<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<507> {
    type Out = Pz<I<I<O<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<508> {
    type Out = Pz<O<O<I<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<509> {
    type Out = Pz<I<O<I<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<510> {
    type Out = Pz<O<I<I<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<511> {
    type Out = Pz<I<I<I<I<I<I<I<I<H>>>>>>>>>;
}
impl NatIndex for Idx<512> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<513> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<514> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<515> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<516> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<517> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<518> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<519> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<520> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<521> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<522> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<523> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<524> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<525> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<526> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<527> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<528> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<529> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<530> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<531> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<532> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<533> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<534> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<535> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<536> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<537> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<538> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<539> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<540> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<541> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<542> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<543> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<544> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<545> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<546> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<547> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<548> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<549> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<550> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<551> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<552> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<553> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<554> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<555> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<556> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<557> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<558> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<559> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<560> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<561> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<562> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<563> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<564> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<565> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<566> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<567> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<568> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<569> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<570> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<571> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<572> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<573> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<574> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<575> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<576> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<577> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<578> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<579> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<580> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<581> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<582> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<583> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<584> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<585> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<586> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<587> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<588> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<589> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<590> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<591> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<592> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<593> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<594> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<595> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<596> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<597> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<598> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<599> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<600> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<601> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<602> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<603> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<604> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<605> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<606> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<607> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<608> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<609> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<610> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<611> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<612> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<613> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<614> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<615> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<616> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<617> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<618> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<619> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<620> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<621> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<622> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<623> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<624> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<625> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<626> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<627> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<628> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<629> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<630> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<631> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<632> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<633> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<634> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<635> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<636> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<637> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<638> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<639> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<640> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<641> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<642> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<643> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<644> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<645> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<646> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<647> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<648> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<649> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<650> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<651> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<652> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<653> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<654> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<655> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<656> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<657> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<658> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<659> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<660> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<661> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<662> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<663> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<664> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<665> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<666> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<667> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<668> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<669> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<670> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<671> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<672> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<673> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<674> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<675> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<676> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<677> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<678> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<679> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<680> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<681> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<682> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<683> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<684> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<685> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<686> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<687> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<688> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<689> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<690> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<691> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<692> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<693> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<694> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<695> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<696> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<697> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<698> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<699> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<700> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<701> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<702> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<703> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<704> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<705> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<706> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<707> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<708> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<709> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<710> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<711> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<712> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<713> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<714> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<715> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<716> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<717> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<718> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<719> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<720> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<721> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<722> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<723> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<724> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<725> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<726> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<727> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<728> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<729> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<730> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<731> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<732> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<733> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<734> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<735> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<736> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<737> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<738> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<739> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<740> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<741> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<742> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<743> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<744> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<745> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<746> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<747> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<748> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<749> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<750> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<751> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<752> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<753> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<754> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<755> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<756> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<757> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<758> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<759> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<760> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<761> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<762> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<763> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<764> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<765> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<766> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<767> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl NatIndex for Idx<768> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<769> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<770> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<771> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<772> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<773> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<774> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<775> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<776> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<777> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<778> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<779> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<780> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<781> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<782> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<783> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<784> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<785> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<786> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<787> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<788> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<789> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<790> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<791> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<792> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<793> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<794> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<795> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<796> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<797> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<798> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<799> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<800> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<801> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<802> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<803> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<804> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<805> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<806> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<807> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<808> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<809> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<810> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<811> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<812> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<813> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<814> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<815> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<816> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<817> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<818> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<819> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<820> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<821> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<822> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<823> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<824> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<825> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<826> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<827> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<828> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<829> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<830> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<831> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<832> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<833> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<834> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<835> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<836> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<837> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<838> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<839> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<840> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<841> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<842> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<843> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<844> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<845> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<846> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<847> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<848> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<849> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<850> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<851> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<852> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<853> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<854> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<855> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<856> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<857> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<858> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<859> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<860> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<861> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<862> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<863> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<864> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<865> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<866> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<867> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<868> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<869> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<870> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<871> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<872> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<873> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<874> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<875> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<876> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<877> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<878> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<879> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<880> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<881> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<882> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<883> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<884> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<885> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<886> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<887> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<888> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<889> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<890> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<891> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<892> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<893> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<894> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<895> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<896> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<897> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<898> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<899> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<900> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<901> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<902> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<903> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<904> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<905> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<906> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<907> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<908> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<909> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<910> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<911> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<912> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<913> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<914> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<915> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<916> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<917> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<918> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<919> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<920> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<921> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<922> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<923> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<924> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<925> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<926> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<927> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<928> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<929> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<930> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<931> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<932> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<933> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<934> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<935> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<936> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<937> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<938> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<939> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<940> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<941> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<942> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<943> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<944> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<945> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<946> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<947> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<948> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<949> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<950> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<951> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<952> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<953> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<954> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<955> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<956> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<957> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<958> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<959> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<960> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<961> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<962> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<963> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<964> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<965> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<966> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<967> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<968> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<969> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<970> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<971> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<972> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<973> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<974> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<975> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<976> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<977> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<978> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<979> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<980> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<981> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<982> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<983> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<984> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<985> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<986> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<987> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<988> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<989> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<990> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<991> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<992> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<993> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<994> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<995> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<996> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<997> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<998> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<999> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1000> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1001> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1002> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1003> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1004> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1005> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1006> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1007> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1008> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1009> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1010> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1011> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1012> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1013> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1014> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1015> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1016> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1017> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1018> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1019> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1020> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1021> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1022> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1023> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl NatIndex for Idx<1024> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1025> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1026> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1027> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1028> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1029> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1030> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1031> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1032> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1033> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1034> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1035> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1036> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1037> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1038> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1039> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1040> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1041> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1042> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1043> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1044> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1045> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1046> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1047> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1048> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1049> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1050> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1051> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1052> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1053> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1054> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1055> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1056> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1057> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1058> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1059> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1060> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1061> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1062> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1063> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1064> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1065> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1066> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1067> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1068> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1069> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1070> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1071> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1072> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1073> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1074> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1075> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1076> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1077> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1078> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1079> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1080> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1081> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1082> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1083> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1084> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1085> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1086> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1087> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1088> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1089> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1090> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1091> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1092> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1093> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1094> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1095> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1096> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1097> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1098> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1099> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1100> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1101> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1102> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1103> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1104> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1105> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1106> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1107> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1108> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1109> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1110> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1111> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1112> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1113> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1114> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1115> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1116> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1117> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1118> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1119> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1120> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1121> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1122> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1123> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1124> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1125> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1126> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1127> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1128> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1129> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1130> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1131> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1132> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1133> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1134> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1135> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1136> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1137> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1138> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1139> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1140> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1141> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1142> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1143> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1144> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1145> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1146> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1147> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1148> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1149> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1150> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1151> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1152> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1153> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1154> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1155> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1156> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1157> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1158> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1159> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1160> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1161> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1162> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1163> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1164> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1165> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1166> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1167> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1168> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1169> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1170> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1171> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1172> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1173> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1174> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1175> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1176> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1177> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1178> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1179> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1180> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1181> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1182> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1183> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1184> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1185> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1186> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1187> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1188> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1189> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1190> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1191> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1192> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1193> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1194> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1195> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1196> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1197> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1198> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1199> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1200> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1201> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1202> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1203> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1204> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1205> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1206> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1207> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1208> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1209> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1210> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1211> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1212> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1213> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1214> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1215> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1216> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1217> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1218> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1219> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1220> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1221> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1222> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1223> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1224> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1225> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1226> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1227> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1228> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1229> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1230> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1231> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1232> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1233> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1234> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1235> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1236> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1237> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1238> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1239> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1240> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1241> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1242> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1243> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1244> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1245> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1246> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1247> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1248> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1249> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1250> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1251> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1252> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1253> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1254> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1255> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1256> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1257> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1258> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1259> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1260> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1261> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1262> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1263> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1264> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1265> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1266> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1267> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1268> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1269> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1270> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1271> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1272> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1273> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1274> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1275> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1276> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1277> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1278> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1279> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1280> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1281> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1282> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1283> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1284> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1285> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1286> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1287> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1288> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1289> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1290> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1291> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1292> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1293> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1294> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1295> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1296> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1297> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1298> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1299> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1300> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1301> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1302> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1303> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1304> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1305> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1306> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1307> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1308> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1309> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1310> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1311> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1312> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1313> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1314> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1315> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1316> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1317> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1318> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1319> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1320> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1321> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1322> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1323> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1324> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1325> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1326> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1327> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1328> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1329> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1330> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1331> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1332> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1333> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1334> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1335> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1336> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1337> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1338> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1339> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1340> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1341> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1342> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1343> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1344> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1345> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1346> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1347> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1348> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1349> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1350> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1351> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1352> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1353> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1354> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1355> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1356> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1357> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1358> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1359> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1360> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1361> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1362> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1363> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1364> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1365> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1366> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1367> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1368> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1369> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1370> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1371> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1372> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1373> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1374> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1375> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1376> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1377> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1378> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1379> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1380> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1381> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1382> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1383> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1384> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1385> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1386> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1387> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1388> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1389> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1390> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1391> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1392> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1393> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1394> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1395> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1396> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1397> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1398> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1399> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1400> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1401> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1402> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1403> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1404> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1405> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1406> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1407> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1408> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1409> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1410> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1411> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1412> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1413> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1414> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1415> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1416> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1417> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1418> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1419> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1420> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1421> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1422> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1423> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1424> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1425> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1426> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1427> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1428> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1429> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1430> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1431> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1432> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1433> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1434> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1435> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1436> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1437> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1438> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1439> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1440> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1441> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1442> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1443> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1444> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1445> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1446> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1447> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1448> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1449> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1450> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1451> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1452> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1453> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1454> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1455> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1456> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1457> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1458> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1459> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1460> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1461> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1462> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1463> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1464> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1465> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1466> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1467> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1468> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1469> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1470> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1471> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1472> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1473> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1474> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1475> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1476> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1477> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1478> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1479> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1480> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1481> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1482> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1483> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1484> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1485> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1486> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1487> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1488> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1489> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1490> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1491> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1492> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1493> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1494> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1495> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1496> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1497> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1498> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1499> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1500> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1501> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1502> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1503> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1504> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1505> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1506> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1507> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1508> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1509> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1510> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1511> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1512> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1513> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1514> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1515> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1516> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1517> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1518> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1519> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1520> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1521> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1522> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1523> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1524> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1525> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1526> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1527> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1528> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1529> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1530> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1531> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1532> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1533> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1534> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1535> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1536> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1537> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1538> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1539> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1540> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1541> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1542> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1543> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1544> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1545> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1546> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1547> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1548> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1549> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1550> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1551> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1552> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1553> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1554> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1555> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1556> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1557> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1558> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1559> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1560> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1561> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1562> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1563> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1564> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1565> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1566> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1567> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1568> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1569> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1570> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1571> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1572> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1573> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1574> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1575> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1576> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1577> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1578> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1579> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1580> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1581> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1582> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1583> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1584> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1585> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1586> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1587> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1588> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1589> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1590> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1591> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1592> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1593> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1594> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1595> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1596> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1597> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1598> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1599> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1600> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1601> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1602> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1603> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1604> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1605> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1606> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1607> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1608> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1609> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1610> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1611> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1612> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1613> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1614> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1615> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1616> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1617> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1618> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1619> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1620> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1621> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1622> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1623> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1624> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1625> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1626> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1627> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1628> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1629> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1630> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1631> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1632> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1633> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1634> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1635> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1636> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1637> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1638> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1639> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1640> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1641> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1642> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1643> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1644> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1645> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1646> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1647> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1648> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1649> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1650> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1651> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1652> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1653> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1654> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1655> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1656> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1657> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1658> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1659> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1660> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1661> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1662> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1663> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1664> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1665> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1666> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1667> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1668> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1669> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1670> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1671> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1672> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1673> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1674> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1675> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1676> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1677> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1678> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1679> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1680> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1681> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1682> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1683> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1684> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1685> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1686> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1687> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1688> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1689> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1690> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1691> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1692> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1693> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1694> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1695> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1696> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1697> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1698> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1699> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1700> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1701> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1702> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1703> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1704> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1705> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1706> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1707> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1708> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1709> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1710> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1711> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1712> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1713> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1714> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1715> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1716> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1717> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1718> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1719> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1720> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1721> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1722> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1723> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1724> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1725> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1726> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1727> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1728> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1729> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1730> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1731> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1732> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1733> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1734> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1735> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1736> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1737> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1738> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1739> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1740> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1741> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1742> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1743> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1744> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1745> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1746> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1747> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1748> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1749> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1750> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1751> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1752> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1753> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1754> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1755> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1756> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1757> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1758> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1759> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1760> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1761> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1762> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1763> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1764> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1765> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1766> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1767> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1768> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1769> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1770> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1771> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1772> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1773> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1774> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1775> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1776> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1777> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1778> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1779> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1780> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1781> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1782> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1783> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1784> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1785> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1786> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1787> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1788> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1789> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1790> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1791> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1792> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1793> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1794> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1795> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1796> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1797> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1798> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1799> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1800> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1801> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1802> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1803> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1804> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1805> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1806> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1807> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1808> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1809> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1810> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1811> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1812> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1813> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1814> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1815> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1816> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1817> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1818> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1819> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1820> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1821> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1822> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1823> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1824> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1825> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1826> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1827> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1828> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1829> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1830> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1831> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1832> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1833> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1834> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1835> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1836> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1837> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1838> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1839> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1840> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1841> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1842> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1843> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1844> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1845> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1846> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1847> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1848> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1849> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1850> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1851> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1852> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1853> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1854> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1855> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1856> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1857> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1858> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1859> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1860> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1861> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1862> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1863> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1864> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1865> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1866> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1867> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1868> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1869> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1870> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1871> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1872> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1873> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1874> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1875> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1876> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1877> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1878> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1879> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1880> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1881> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1882> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1883> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1884> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1885> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1886> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1887> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1888> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1889> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1890> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1891> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1892> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1893> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1894> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1895> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1896> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1897> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1898> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1899> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1900> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1901> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1902> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1903> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1904> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1905> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1906> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1907> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1908> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1909> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1910> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1911> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1912> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1913> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1914> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1915> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1916> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1917> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1918> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1919> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1920> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1921> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1922> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1923> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1924> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1925> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1926> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1927> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1928> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1929> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1930> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1931> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1932> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1933> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1934> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1935> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1936> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1937> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1938> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1939> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1940> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1941> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1942> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1943> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1944> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1945> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1946> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1947> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1948> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1949> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1950> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1951> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1952> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1953> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1954> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1955> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1956> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1957> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1958> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1959> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1960> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1961> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1962> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1963> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1964> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1965> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1966> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1967> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1968> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1969> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1970> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1971> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1972> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1973> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1974> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1975> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1976> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1977> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1978> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1979> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1980> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1981> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1982> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1983> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1984> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1985> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1986> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1987> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1988> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1989> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1990> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1991> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1992> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1993> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1994> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1995> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1996> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1997> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1998> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<1999> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2000> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2001> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2002> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2003> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2004> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2005> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2006> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2007> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2008> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2009> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2010> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2011> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2012> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2013> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2014> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2015> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2016> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2017> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2018> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2019> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2020> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2021> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2022> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2023> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2024> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2025> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2026> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2027> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2028> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2029> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2030> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2031> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2032> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2033> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2034> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2035> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2036> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2037> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2038> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2039> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2040> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2041> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2042> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2043> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2044> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2045> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2046> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl NatIndex for Idx<2047> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}

// the load-bearing claim
const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
