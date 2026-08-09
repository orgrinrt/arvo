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

// the load-bearing claim
const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
