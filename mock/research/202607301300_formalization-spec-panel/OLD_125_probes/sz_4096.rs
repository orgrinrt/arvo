#![no_std]
#![allow(dead_code)]
extern crate tower;
pub use tower::*;
pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Out: Nat;
}
impl AdmittedWidth for Idx<0> {
    type Out = Z;
}
impl AdmittedWidth for Idx<1> {
    type Out = Pz<H>;
}
impl AdmittedWidth for Idx<2> {
    type Out = Pz<O<H>>;
}
impl AdmittedWidth for Idx<3> {
    type Out = Pz<I<H>>;
}
impl AdmittedWidth for Idx<4> {
    type Out = Pz<O<O<H>>>;
}
impl AdmittedWidth for Idx<5> {
    type Out = Pz<I<O<H>>>;
}
impl AdmittedWidth for Idx<6> {
    type Out = Pz<O<I<H>>>;
}
impl AdmittedWidth for Idx<7> {
    type Out = Pz<I<I<H>>>;
}
impl AdmittedWidth for Idx<8> {
    type Out = Pz<O<O<O<H>>>>;
}
impl AdmittedWidth for Idx<9> {
    type Out = Pz<I<O<O<H>>>>;
}
impl AdmittedWidth for Idx<10> {
    type Out = Pz<O<I<O<H>>>>;
}
impl AdmittedWidth for Idx<11> {
    type Out = Pz<I<I<O<H>>>>;
}
impl AdmittedWidth for Idx<12> {
    type Out = Pz<O<O<I<H>>>>;
}
impl AdmittedWidth for Idx<13> {
    type Out = Pz<I<O<I<H>>>>;
}
impl AdmittedWidth for Idx<14> {
    type Out = Pz<O<I<I<H>>>>;
}
impl AdmittedWidth for Idx<15> {
    type Out = Pz<I<I<I<H>>>>;
}
impl AdmittedWidth for Idx<16> {
    type Out = Pz<O<O<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<17> {
    type Out = Pz<I<O<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<18> {
    type Out = Pz<O<I<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<19> {
    type Out = Pz<I<I<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<20> {
    type Out = Pz<O<O<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<21> {
    type Out = Pz<I<O<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<22> {
    type Out = Pz<O<I<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<23> {
    type Out = Pz<I<I<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<24> {
    type Out = Pz<O<O<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<25> {
    type Out = Pz<I<O<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<26> {
    type Out = Pz<O<I<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<27> {
    type Out = Pz<I<I<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<28> {
    type Out = Pz<O<O<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<29> {
    type Out = Pz<I<O<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<30> {
    type Out = Pz<O<I<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<31> {
    type Out = Pz<I<I<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<32> {
    type Out = Pz<O<O<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<33> {
    type Out = Pz<I<O<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<34> {
    type Out = Pz<O<I<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<35> {
    type Out = Pz<I<I<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<36> {
    type Out = Pz<O<O<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<37> {
    type Out = Pz<I<O<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<38> {
    type Out = Pz<O<I<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<39> {
    type Out = Pz<I<I<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<40> {
    type Out = Pz<O<O<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<41> {
    type Out = Pz<I<O<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<42> {
    type Out = Pz<O<I<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<43> {
    type Out = Pz<I<I<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<44> {
    type Out = Pz<O<O<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<45> {
    type Out = Pz<I<O<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<46> {
    type Out = Pz<O<I<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<47> {
    type Out = Pz<I<I<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<48> {
    type Out = Pz<O<O<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<49> {
    type Out = Pz<I<O<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<50> {
    type Out = Pz<O<I<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<51> {
    type Out = Pz<I<I<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<52> {
    type Out = Pz<O<O<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<53> {
    type Out = Pz<I<O<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<54> {
    type Out = Pz<O<I<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<55> {
    type Out = Pz<I<I<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<56> {
    type Out = Pz<O<O<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<57> {
    type Out = Pz<I<O<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<58> {
    type Out = Pz<O<I<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<59> {
    type Out = Pz<I<I<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<60> {
    type Out = Pz<O<O<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<61> {
    type Out = Pz<I<O<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<62> {
    type Out = Pz<O<I<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<63> {
    type Out = Pz<I<I<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<64> {
    type Out = Pz<O<O<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<65> {
    type Out = Pz<I<O<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<66> {
    type Out = Pz<O<I<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<67> {
    type Out = Pz<I<I<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<68> {
    type Out = Pz<O<O<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<69> {
    type Out = Pz<I<O<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<70> {
    type Out = Pz<O<I<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<71> {
    type Out = Pz<I<I<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<72> {
    type Out = Pz<O<O<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<73> {
    type Out = Pz<I<O<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<74> {
    type Out = Pz<O<I<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<75> {
    type Out = Pz<I<I<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<76> {
    type Out = Pz<O<O<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<77> {
    type Out = Pz<I<O<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<78> {
    type Out = Pz<O<I<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<79> {
    type Out = Pz<I<I<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<80> {
    type Out = Pz<O<O<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<81> {
    type Out = Pz<I<O<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<82> {
    type Out = Pz<O<I<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<83> {
    type Out = Pz<I<I<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<84> {
    type Out = Pz<O<O<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<85> {
    type Out = Pz<I<O<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<86> {
    type Out = Pz<O<I<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<87> {
    type Out = Pz<I<I<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<88> {
    type Out = Pz<O<O<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<89> {
    type Out = Pz<I<O<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<90> {
    type Out = Pz<O<I<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<91> {
    type Out = Pz<I<I<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<92> {
    type Out = Pz<O<O<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<93> {
    type Out = Pz<I<O<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<94> {
    type Out = Pz<O<I<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<95> {
    type Out = Pz<I<I<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<96> {
    type Out = Pz<O<O<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<97> {
    type Out = Pz<I<O<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<98> {
    type Out = Pz<O<I<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<99> {
    type Out = Pz<I<I<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<100> {
    type Out = Pz<O<O<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<101> {
    type Out = Pz<I<O<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<102> {
    type Out = Pz<O<I<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<103> {
    type Out = Pz<I<I<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<104> {
    type Out = Pz<O<O<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<105> {
    type Out = Pz<I<O<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<106> {
    type Out = Pz<O<I<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<107> {
    type Out = Pz<I<I<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<108> {
    type Out = Pz<O<O<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<109> {
    type Out = Pz<I<O<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<110> {
    type Out = Pz<O<I<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<111> {
    type Out = Pz<I<I<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<112> {
    type Out = Pz<O<O<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<113> {
    type Out = Pz<I<O<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<114> {
    type Out = Pz<O<I<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<115> {
    type Out = Pz<I<I<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<116> {
    type Out = Pz<O<O<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<117> {
    type Out = Pz<I<O<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<118> {
    type Out = Pz<O<I<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<119> {
    type Out = Pz<I<I<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<120> {
    type Out = Pz<O<O<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<121> {
    type Out = Pz<I<O<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<122> {
    type Out = Pz<O<I<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<123> {
    type Out = Pz<I<I<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<124> {
    type Out = Pz<O<O<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<125> {
    type Out = Pz<I<O<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<126> {
    type Out = Pz<O<I<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<127> {
    type Out = Pz<I<I<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<128> {
    type Out = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<129> {
    type Out = Pz<I<O<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<130> {
    type Out = Pz<O<I<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<131> {
    type Out = Pz<I<I<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<132> {
    type Out = Pz<O<O<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<133> {
    type Out = Pz<I<O<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<134> {
    type Out = Pz<O<I<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<135> {
    type Out = Pz<I<I<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<136> {
    type Out = Pz<O<O<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<137> {
    type Out = Pz<I<O<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<138> {
    type Out = Pz<O<I<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<139> {
    type Out = Pz<I<I<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<140> {
    type Out = Pz<O<O<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<141> {
    type Out = Pz<I<O<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<142> {
    type Out = Pz<O<I<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<143> {
    type Out = Pz<I<I<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<144> {
    type Out = Pz<O<O<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<145> {
    type Out = Pz<I<O<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<146> {
    type Out = Pz<O<I<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<147> {
    type Out = Pz<I<I<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<148> {
    type Out = Pz<O<O<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<149> {
    type Out = Pz<I<O<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<150> {
    type Out = Pz<O<I<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<151> {
    type Out = Pz<I<I<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<152> {
    type Out = Pz<O<O<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<153> {
    type Out = Pz<I<O<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<154> {
    type Out = Pz<O<I<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<155> {
    type Out = Pz<I<I<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<156> {
    type Out = Pz<O<O<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<157> {
    type Out = Pz<I<O<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<158> {
    type Out = Pz<O<I<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<159> {
    type Out = Pz<I<I<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<160> {
    type Out = Pz<O<O<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<161> {
    type Out = Pz<I<O<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<162> {
    type Out = Pz<O<I<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<163> {
    type Out = Pz<I<I<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<164> {
    type Out = Pz<O<O<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<165> {
    type Out = Pz<I<O<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<166> {
    type Out = Pz<O<I<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<167> {
    type Out = Pz<I<I<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<168> {
    type Out = Pz<O<O<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<169> {
    type Out = Pz<I<O<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<170> {
    type Out = Pz<O<I<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<171> {
    type Out = Pz<I<I<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<172> {
    type Out = Pz<O<O<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<173> {
    type Out = Pz<I<O<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<174> {
    type Out = Pz<O<I<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<175> {
    type Out = Pz<I<I<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<176> {
    type Out = Pz<O<O<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<177> {
    type Out = Pz<I<O<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<178> {
    type Out = Pz<O<I<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<179> {
    type Out = Pz<I<I<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<180> {
    type Out = Pz<O<O<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<181> {
    type Out = Pz<I<O<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<182> {
    type Out = Pz<O<I<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<183> {
    type Out = Pz<I<I<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<184> {
    type Out = Pz<O<O<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<185> {
    type Out = Pz<I<O<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<186> {
    type Out = Pz<O<I<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<187> {
    type Out = Pz<I<I<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<188> {
    type Out = Pz<O<O<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<189> {
    type Out = Pz<I<O<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<190> {
    type Out = Pz<O<I<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<191> {
    type Out = Pz<I<I<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<192> {
    type Out = Pz<O<O<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<193> {
    type Out = Pz<I<O<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<194> {
    type Out = Pz<O<I<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<195> {
    type Out = Pz<I<I<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<196> {
    type Out = Pz<O<O<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<197> {
    type Out = Pz<I<O<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<198> {
    type Out = Pz<O<I<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<199> {
    type Out = Pz<I<I<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<200> {
    type Out = Pz<O<O<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<201> {
    type Out = Pz<I<O<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<202> {
    type Out = Pz<O<I<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<203> {
    type Out = Pz<I<I<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<204> {
    type Out = Pz<O<O<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<205> {
    type Out = Pz<I<O<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<206> {
    type Out = Pz<O<I<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<207> {
    type Out = Pz<I<I<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<208> {
    type Out = Pz<O<O<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<209> {
    type Out = Pz<I<O<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<210> {
    type Out = Pz<O<I<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<211> {
    type Out = Pz<I<I<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<212> {
    type Out = Pz<O<O<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<213> {
    type Out = Pz<I<O<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<214> {
    type Out = Pz<O<I<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<215> {
    type Out = Pz<I<I<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<216> {
    type Out = Pz<O<O<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<217> {
    type Out = Pz<I<O<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<218> {
    type Out = Pz<O<I<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<219> {
    type Out = Pz<I<I<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<220> {
    type Out = Pz<O<O<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<221> {
    type Out = Pz<I<O<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<222> {
    type Out = Pz<O<I<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<223> {
    type Out = Pz<I<I<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<224> {
    type Out = Pz<O<O<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<225> {
    type Out = Pz<I<O<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<226> {
    type Out = Pz<O<I<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<227> {
    type Out = Pz<I<I<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<228> {
    type Out = Pz<O<O<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<229> {
    type Out = Pz<I<O<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<230> {
    type Out = Pz<O<I<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<231> {
    type Out = Pz<I<I<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<232> {
    type Out = Pz<O<O<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<233> {
    type Out = Pz<I<O<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<234> {
    type Out = Pz<O<I<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<235> {
    type Out = Pz<I<I<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<236> {
    type Out = Pz<O<O<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<237> {
    type Out = Pz<I<O<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<238> {
    type Out = Pz<O<I<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<239> {
    type Out = Pz<I<I<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<240> {
    type Out = Pz<O<O<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<241> {
    type Out = Pz<I<O<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<242> {
    type Out = Pz<O<I<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<243> {
    type Out = Pz<I<I<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<244> {
    type Out = Pz<O<O<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<245> {
    type Out = Pz<I<O<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<246> {
    type Out = Pz<O<I<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<247> {
    type Out = Pz<I<I<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<248> {
    type Out = Pz<O<O<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<249> {
    type Out = Pz<I<O<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<250> {
    type Out = Pz<O<I<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<251> {
    type Out = Pz<I<I<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<252> {
    type Out = Pz<O<O<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<253> {
    type Out = Pz<I<O<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<254> {
    type Out = Pz<O<I<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<255> {
    type Out = Pz<I<I<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<256> {
    type Out = Pz<O<O<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<257> {
    type Out = Pz<I<O<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<258> {
    type Out = Pz<O<I<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<259> {
    type Out = Pz<I<I<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<260> {
    type Out = Pz<O<O<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<261> {
    type Out = Pz<I<O<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<262> {
    type Out = Pz<O<I<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<263> {
    type Out = Pz<I<I<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<264> {
    type Out = Pz<O<O<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<265> {
    type Out = Pz<I<O<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<266> {
    type Out = Pz<O<I<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<267> {
    type Out = Pz<I<I<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<268> {
    type Out = Pz<O<O<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<269> {
    type Out = Pz<I<O<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<270> {
    type Out = Pz<O<I<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<271> {
    type Out = Pz<I<I<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<272> {
    type Out = Pz<O<O<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<273> {
    type Out = Pz<I<O<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<274> {
    type Out = Pz<O<I<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<275> {
    type Out = Pz<I<I<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<276> {
    type Out = Pz<O<O<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<277> {
    type Out = Pz<I<O<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<278> {
    type Out = Pz<O<I<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<279> {
    type Out = Pz<I<I<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<280> {
    type Out = Pz<O<O<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<281> {
    type Out = Pz<I<O<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<282> {
    type Out = Pz<O<I<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<283> {
    type Out = Pz<I<I<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<284> {
    type Out = Pz<O<O<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<285> {
    type Out = Pz<I<O<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<286> {
    type Out = Pz<O<I<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<287> {
    type Out = Pz<I<I<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<288> {
    type Out = Pz<O<O<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<289> {
    type Out = Pz<I<O<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<290> {
    type Out = Pz<O<I<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<291> {
    type Out = Pz<I<I<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<292> {
    type Out = Pz<O<O<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<293> {
    type Out = Pz<I<O<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<294> {
    type Out = Pz<O<I<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<295> {
    type Out = Pz<I<I<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<296> {
    type Out = Pz<O<O<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<297> {
    type Out = Pz<I<O<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<298> {
    type Out = Pz<O<I<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<299> {
    type Out = Pz<I<I<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<300> {
    type Out = Pz<O<O<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<301> {
    type Out = Pz<I<O<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<302> {
    type Out = Pz<O<I<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<303> {
    type Out = Pz<I<I<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<304> {
    type Out = Pz<O<O<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<305> {
    type Out = Pz<I<O<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<306> {
    type Out = Pz<O<I<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<307> {
    type Out = Pz<I<I<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<308> {
    type Out = Pz<O<O<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<309> {
    type Out = Pz<I<O<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<310> {
    type Out = Pz<O<I<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<311> {
    type Out = Pz<I<I<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<312> {
    type Out = Pz<O<O<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<313> {
    type Out = Pz<I<O<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<314> {
    type Out = Pz<O<I<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<315> {
    type Out = Pz<I<I<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<316> {
    type Out = Pz<O<O<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<317> {
    type Out = Pz<I<O<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<318> {
    type Out = Pz<O<I<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<319> {
    type Out = Pz<I<I<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<320> {
    type Out = Pz<O<O<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<321> {
    type Out = Pz<I<O<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<322> {
    type Out = Pz<O<I<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<323> {
    type Out = Pz<I<I<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<324> {
    type Out = Pz<O<O<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<325> {
    type Out = Pz<I<O<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<326> {
    type Out = Pz<O<I<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<327> {
    type Out = Pz<I<I<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<328> {
    type Out = Pz<O<O<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<329> {
    type Out = Pz<I<O<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<330> {
    type Out = Pz<O<I<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<331> {
    type Out = Pz<I<I<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<332> {
    type Out = Pz<O<O<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<333> {
    type Out = Pz<I<O<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<334> {
    type Out = Pz<O<I<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<335> {
    type Out = Pz<I<I<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<336> {
    type Out = Pz<O<O<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<337> {
    type Out = Pz<I<O<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<338> {
    type Out = Pz<O<I<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<339> {
    type Out = Pz<I<I<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<340> {
    type Out = Pz<O<O<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<341> {
    type Out = Pz<I<O<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<342> {
    type Out = Pz<O<I<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<343> {
    type Out = Pz<I<I<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<344> {
    type Out = Pz<O<O<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<345> {
    type Out = Pz<I<O<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<346> {
    type Out = Pz<O<I<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<347> {
    type Out = Pz<I<I<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<348> {
    type Out = Pz<O<O<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<349> {
    type Out = Pz<I<O<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<350> {
    type Out = Pz<O<I<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<351> {
    type Out = Pz<I<I<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<352> {
    type Out = Pz<O<O<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<353> {
    type Out = Pz<I<O<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<354> {
    type Out = Pz<O<I<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<355> {
    type Out = Pz<I<I<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<356> {
    type Out = Pz<O<O<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<357> {
    type Out = Pz<I<O<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<358> {
    type Out = Pz<O<I<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<359> {
    type Out = Pz<I<I<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<360> {
    type Out = Pz<O<O<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<361> {
    type Out = Pz<I<O<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<362> {
    type Out = Pz<O<I<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<363> {
    type Out = Pz<I<I<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<364> {
    type Out = Pz<O<O<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<365> {
    type Out = Pz<I<O<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<366> {
    type Out = Pz<O<I<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<367> {
    type Out = Pz<I<I<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<368> {
    type Out = Pz<O<O<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<369> {
    type Out = Pz<I<O<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<370> {
    type Out = Pz<O<I<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<371> {
    type Out = Pz<I<I<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<372> {
    type Out = Pz<O<O<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<373> {
    type Out = Pz<I<O<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<374> {
    type Out = Pz<O<I<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<375> {
    type Out = Pz<I<I<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<376> {
    type Out = Pz<O<O<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<377> {
    type Out = Pz<I<O<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<378> {
    type Out = Pz<O<I<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<379> {
    type Out = Pz<I<I<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<380> {
    type Out = Pz<O<O<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<381> {
    type Out = Pz<I<O<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<382> {
    type Out = Pz<O<I<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<383> {
    type Out = Pz<I<I<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<384> {
    type Out = Pz<O<O<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<385> {
    type Out = Pz<I<O<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<386> {
    type Out = Pz<O<I<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<387> {
    type Out = Pz<I<I<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<388> {
    type Out = Pz<O<O<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<389> {
    type Out = Pz<I<O<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<390> {
    type Out = Pz<O<I<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<391> {
    type Out = Pz<I<I<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<392> {
    type Out = Pz<O<O<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<393> {
    type Out = Pz<I<O<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<394> {
    type Out = Pz<O<I<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<395> {
    type Out = Pz<I<I<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<396> {
    type Out = Pz<O<O<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<397> {
    type Out = Pz<I<O<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<398> {
    type Out = Pz<O<I<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<399> {
    type Out = Pz<I<I<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<400> {
    type Out = Pz<O<O<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<401> {
    type Out = Pz<I<O<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<402> {
    type Out = Pz<O<I<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<403> {
    type Out = Pz<I<I<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<404> {
    type Out = Pz<O<O<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<405> {
    type Out = Pz<I<O<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<406> {
    type Out = Pz<O<I<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<407> {
    type Out = Pz<I<I<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<408> {
    type Out = Pz<O<O<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<409> {
    type Out = Pz<I<O<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<410> {
    type Out = Pz<O<I<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<411> {
    type Out = Pz<I<I<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<412> {
    type Out = Pz<O<O<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<413> {
    type Out = Pz<I<O<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<414> {
    type Out = Pz<O<I<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<415> {
    type Out = Pz<I<I<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<416> {
    type Out = Pz<O<O<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<417> {
    type Out = Pz<I<O<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<418> {
    type Out = Pz<O<I<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<419> {
    type Out = Pz<I<I<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<420> {
    type Out = Pz<O<O<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<421> {
    type Out = Pz<I<O<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<422> {
    type Out = Pz<O<I<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<423> {
    type Out = Pz<I<I<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<424> {
    type Out = Pz<O<O<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<425> {
    type Out = Pz<I<O<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<426> {
    type Out = Pz<O<I<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<427> {
    type Out = Pz<I<I<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<428> {
    type Out = Pz<O<O<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<429> {
    type Out = Pz<I<O<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<430> {
    type Out = Pz<O<I<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<431> {
    type Out = Pz<I<I<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<432> {
    type Out = Pz<O<O<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<433> {
    type Out = Pz<I<O<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<434> {
    type Out = Pz<O<I<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<435> {
    type Out = Pz<I<I<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<436> {
    type Out = Pz<O<O<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<437> {
    type Out = Pz<I<O<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<438> {
    type Out = Pz<O<I<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<439> {
    type Out = Pz<I<I<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<440> {
    type Out = Pz<O<O<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<441> {
    type Out = Pz<I<O<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<442> {
    type Out = Pz<O<I<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<443> {
    type Out = Pz<I<I<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<444> {
    type Out = Pz<O<O<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<445> {
    type Out = Pz<I<O<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<446> {
    type Out = Pz<O<I<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<447> {
    type Out = Pz<I<I<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<448> {
    type Out = Pz<O<O<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<449> {
    type Out = Pz<I<O<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<450> {
    type Out = Pz<O<I<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<451> {
    type Out = Pz<I<I<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<452> {
    type Out = Pz<O<O<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<453> {
    type Out = Pz<I<O<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<454> {
    type Out = Pz<O<I<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<455> {
    type Out = Pz<I<I<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<456> {
    type Out = Pz<O<O<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<457> {
    type Out = Pz<I<O<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<458> {
    type Out = Pz<O<I<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<459> {
    type Out = Pz<I<I<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<460> {
    type Out = Pz<O<O<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<461> {
    type Out = Pz<I<O<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<462> {
    type Out = Pz<O<I<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<463> {
    type Out = Pz<I<I<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<464> {
    type Out = Pz<O<O<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<465> {
    type Out = Pz<I<O<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<466> {
    type Out = Pz<O<I<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<467> {
    type Out = Pz<I<I<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<468> {
    type Out = Pz<O<O<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<469> {
    type Out = Pz<I<O<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<470> {
    type Out = Pz<O<I<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<471> {
    type Out = Pz<I<I<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<472> {
    type Out = Pz<O<O<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<473> {
    type Out = Pz<I<O<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<474> {
    type Out = Pz<O<I<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<475> {
    type Out = Pz<I<I<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<476> {
    type Out = Pz<O<O<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<477> {
    type Out = Pz<I<O<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<478> {
    type Out = Pz<O<I<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<479> {
    type Out = Pz<I<I<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<480> {
    type Out = Pz<O<O<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<481> {
    type Out = Pz<I<O<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<482> {
    type Out = Pz<O<I<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<483> {
    type Out = Pz<I<I<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<484> {
    type Out = Pz<O<O<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<485> {
    type Out = Pz<I<O<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<486> {
    type Out = Pz<O<I<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<487> {
    type Out = Pz<I<I<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<488> {
    type Out = Pz<O<O<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<489> {
    type Out = Pz<I<O<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<490> {
    type Out = Pz<O<I<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<491> {
    type Out = Pz<I<I<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<492> {
    type Out = Pz<O<O<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<493> {
    type Out = Pz<I<O<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<494> {
    type Out = Pz<O<I<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<495> {
    type Out = Pz<I<I<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<496> {
    type Out = Pz<O<O<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<497> {
    type Out = Pz<I<O<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<498> {
    type Out = Pz<O<I<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<499> {
    type Out = Pz<I<I<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<500> {
    type Out = Pz<O<O<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<501> {
    type Out = Pz<I<O<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<502> {
    type Out = Pz<O<I<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<503> {
    type Out = Pz<I<I<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<504> {
    type Out = Pz<O<O<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<505> {
    type Out = Pz<I<O<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<506> {
    type Out = Pz<O<I<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<507> {
    type Out = Pz<I<I<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<508> {
    type Out = Pz<O<O<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<509> {
    type Out = Pz<I<O<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<510> {
    type Out = Pz<O<I<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<511> {
    type Out = Pz<I<I<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<512> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<513> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<514> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<515> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<516> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<517> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<518> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<519> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<520> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<521> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<522> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<523> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<524> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<525> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<526> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<527> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<528> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<529> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<530> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<531> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<532> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<533> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<534> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<535> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<536> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<537> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<538> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<539> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<540> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<541> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<542> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<543> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<544> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<545> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<546> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<547> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<548> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<549> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<550> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<551> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<552> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<553> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<554> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<555> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<556> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<557> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<558> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<559> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<560> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<561> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<562> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<563> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<564> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<565> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<566> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<567> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<568> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<569> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<570> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<571> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<572> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<573> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<574> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<575> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<576> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<577> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<578> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<579> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<580> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<581> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<582> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<583> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<584> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<585> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<586> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<587> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<588> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<589> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<590> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<591> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<592> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<593> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<594> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<595> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<596> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<597> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<598> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<599> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<600> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<601> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<602> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<603> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<604> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<605> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<606> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<607> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<608> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<609> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<610> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<611> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<612> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<613> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<614> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<615> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<616> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<617> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<618> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<619> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<620> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<621> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<622> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<623> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<624> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<625> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<626> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<627> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<628> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<629> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<630> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<631> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<632> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<633> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<634> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<635> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<636> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<637> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<638> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<639> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<640> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<641> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<642> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<643> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<644> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<645> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<646> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<647> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<648> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<649> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<650> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<651> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<652> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<653> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<654> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<655> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<656> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<657> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<658> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<659> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<660> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<661> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<662> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<663> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<664> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<665> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<666> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<667> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<668> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<669> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<670> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<671> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<672> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<673> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<674> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<675> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<676> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<677> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<678> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<679> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<680> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<681> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<682> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<683> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<684> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<685> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<686> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<687> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<688> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<689> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<690> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<691> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<692> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<693> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<694> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<695> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<696> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<697> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<698> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<699> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<700> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<701> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<702> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<703> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<704> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<705> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<706> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<707> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<708> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<709> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<710> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<711> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<712> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<713> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<714> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<715> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<716> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<717> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<718> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<719> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<720> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<721> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<722> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<723> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<724> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<725> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<726> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<727> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<728> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<729> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<730> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<731> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<732> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<733> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<734> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<735> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<736> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<737> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<738> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<739> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<740> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<741> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<742> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<743> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<744> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<745> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<746> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<747> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<748> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<749> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<750> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<751> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<752> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<753> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<754> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<755> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<756> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<757> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<758> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<759> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<760> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<761> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<762> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<763> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<764> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<765> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<766> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<767> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<768> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<769> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<770> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<771> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<772> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<773> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<774> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<775> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<776> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<777> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<778> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<779> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<780> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<781> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<782> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<783> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<784> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<785> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<786> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<787> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<788> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<789> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<790> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<791> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<792> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<793> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<794> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<795> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<796> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<797> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<798> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<799> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<800> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<801> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<802> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<803> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<804> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<805> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<806> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<807> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<808> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<809> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<810> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<811> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<812> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<813> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<814> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<815> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<816> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<817> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<818> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<819> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<820> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<821> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<822> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<823> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<824> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<825> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<826> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<827> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<828> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<829> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<830> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<831> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<832> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<833> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<834> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<835> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<836> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<837> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<838> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<839> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<840> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<841> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<842> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<843> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<844> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<845> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<846> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<847> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<848> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<849> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<850> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<851> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<852> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<853> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<854> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<855> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<856> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<857> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<858> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<859> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<860> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<861> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<862> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<863> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<864> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<865> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<866> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<867> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<868> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<869> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<870> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<871> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<872> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<873> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<874> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<875> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<876> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<877> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<878> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<879> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<880> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<881> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<882> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<883> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<884> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<885> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<886> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<887> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<888> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<889> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<890> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<891> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<892> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<893> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<894> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<895> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<896> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<897> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<898> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<899> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<900> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<901> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<902> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<903> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<904> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<905> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<906> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<907> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<908> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<909> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<910> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<911> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<912> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<913> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<914> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<915> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<916> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<917> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<918> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<919> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<920> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<921> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<922> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<923> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<924> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<925> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<926> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<927> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<928> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<929> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<930> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<931> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<932> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<933> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<934> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<935> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<936> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<937> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<938> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<939> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<940> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<941> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<942> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<943> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<944> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<945> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<946> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<947> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<948> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<949> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<950> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<951> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<952> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<953> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<954> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<955> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<956> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<957> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<958> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<959> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<960> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<961> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<962> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<963> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<964> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<965> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<966> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<967> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<968> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<969> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<970> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<971> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<972> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<973> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<974> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<975> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<976> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<977> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<978> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<979> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<980> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<981> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<982> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<983> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<984> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<985> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<986> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<987> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<988> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<989> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<990> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<991> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<992> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<993> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<994> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<995> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<996> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<997> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<998> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<999> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1000> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1001> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1002> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1003> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1004> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1005> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1006> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1007> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1008> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1009> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1010> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1011> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1012> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1013> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1014> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1015> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1016> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1017> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1018> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1019> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1020> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1021> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1022> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1023> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1024> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1025> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1026> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1027> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1028> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1029> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1030> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1031> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1032> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1033> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1034> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1035> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1036> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1037> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1038> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1039> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1040> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1041> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1042> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1043> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1044> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1045> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1046> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1047> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1048> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1049> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1050> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1051> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1052> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1053> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1054> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1055> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1056> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1057> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1058> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1059> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1060> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1061> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1062> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1063> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1064> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1065> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1066> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1067> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1068> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1069> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1070> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1071> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1072> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1073> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1074> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1075> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1076> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1077> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1078> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1079> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1080> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1081> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1082> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1083> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1084> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1085> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1086> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1087> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1088> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1089> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1090> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1091> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1092> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1093> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1094> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1095> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1096> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1097> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1098> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1099> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1100> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1101> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1102> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1103> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1104> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1105> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1106> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1107> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1108> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1109> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1110> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1111> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1112> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1113> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1114> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1115> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1116> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1117> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1118> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1119> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1120> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1121> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1122> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1123> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1124> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1125> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1126> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1127> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1128> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1129> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1130> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1131> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1132> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1133> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1134> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1135> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1136> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1137> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1138> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1139> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1140> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1141> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1142> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1143> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1144> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1145> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1146> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1147> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1148> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1149> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1150> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1151> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1152> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1153> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1154> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1155> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1156> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1157> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1158> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1159> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1160> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1161> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1162> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1163> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1164> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1165> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1166> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1167> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1168> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1169> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1170> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1171> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1172> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1173> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1174> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1175> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1176> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1177> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1178> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1179> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1180> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1181> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1182> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1183> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1184> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1185> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1186> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1187> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1188> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1189> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1190> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1191> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1192> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1193> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1194> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1195> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1196> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1197> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1198> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1199> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1200> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1201> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1202> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1203> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1204> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1205> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1206> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1207> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1208> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1209> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1210> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1211> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1212> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1213> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1214> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1215> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1216> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1217> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1218> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1219> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1220> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1221> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1222> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1223> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1224> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1225> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1226> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1227> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1228> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1229> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1230> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1231> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1232> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1233> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1234> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1235> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1236> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1237> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1238> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1239> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1240> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1241> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1242> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1243> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1244> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1245> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1246> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1247> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1248> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1249> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1250> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1251> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1252> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1253> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1254> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1255> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1256> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1257> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1258> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1259> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1260> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1261> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1262> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1263> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1264> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1265> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1266> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1267> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1268> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1269> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1270> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1271> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1272> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1273> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1274> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1275> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1276> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1277> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1278> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1279> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1280> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1281> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1282> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1283> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1284> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1285> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1286> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1287> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1288> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1289> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1290> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1291> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1292> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1293> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1294> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1295> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1296> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1297> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1298> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1299> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1300> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1301> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1302> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1303> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1304> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1305> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1306> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1307> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1308> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1309> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1310> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1311> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1312> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1313> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1314> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1315> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1316> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1317> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1318> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1319> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1320> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1321> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1322> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1323> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1324> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1325> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1326> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1327> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1328> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1329> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1330> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1331> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1332> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1333> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1334> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1335> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1336> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1337> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1338> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1339> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1340> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1341> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1342> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1343> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1344> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1345> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1346> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1347> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1348> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1349> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1350> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1351> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1352> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1353> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1354> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1355> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1356> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1357> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1358> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1359> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1360> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1361> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1362> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1363> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1364> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1365> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1366> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1367> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1368> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1369> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1370> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1371> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1372> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1373> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1374> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1375> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1376> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1377> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1378> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1379> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1380> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1381> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1382> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1383> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1384> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1385> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1386> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1387> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1388> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1389> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1390> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1391> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1392> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1393> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1394> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1395> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1396> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1397> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1398> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1399> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1400> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1401> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1402> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1403> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1404> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1405> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1406> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1407> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1408> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1409> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1410> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1411> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1412> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1413> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1414> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1415> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1416> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1417> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1418> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1419> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1420> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1421> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1422> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1423> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1424> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1425> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1426> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1427> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1428> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1429> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1430> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1431> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1432> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1433> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1434> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1435> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1436> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1437> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1438> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1439> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1440> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1441> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1442> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1443> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1444> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1445> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1446> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1447> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1448> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1449> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1450> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1451> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1452> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1453> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1454> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1455> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1456> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1457> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1458> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1459> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1460> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1461> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1462> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1463> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1464> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1465> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1466> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1467> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1468> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1469> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1470> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1471> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1472> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1473> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1474> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1475> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1476> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1477> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1478> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1479> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1480> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1481> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1482> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1483> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1484> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1485> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1486> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1487> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1488> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1489> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1490> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1491> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1492> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1493> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1494> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1495> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1496> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1497> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1498> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1499> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1500> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1501> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1502> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1503> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1504> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1505> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1506> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1507> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1508> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1509> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1510> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1511> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1512> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1513> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1514> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1515> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1516> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1517> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1518> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1519> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1520> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1521> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1522> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1523> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1524> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1525> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1526> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1527> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1528> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1529> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1530> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1531> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1532> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1533> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1534> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1535> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1536> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1537> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1538> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1539> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1540> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1541> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1542> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1543> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1544> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1545> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1546> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1547> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1548> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1549> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1550> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1551> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1552> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1553> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1554> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1555> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1556> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1557> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1558> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1559> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1560> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1561> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1562> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1563> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1564> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1565> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1566> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1567> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1568> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1569> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1570> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1571> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1572> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1573> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1574> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1575> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1576> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1577> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1578> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1579> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1580> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1581> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1582> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1583> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1584> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1585> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1586> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1587> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1588> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1589> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1590> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1591> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1592> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1593> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1594> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1595> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1596> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1597> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1598> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1599> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1600> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1601> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1602> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1603> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1604> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1605> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1606> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1607> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1608> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1609> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1610> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1611> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1612> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1613> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1614> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1615> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1616> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1617> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1618> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1619> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1620> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1621> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1622> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1623> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1624> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1625> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1626> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1627> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1628> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1629> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1630> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1631> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1632> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1633> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1634> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1635> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1636> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1637> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1638> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1639> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1640> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1641> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1642> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1643> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1644> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1645> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1646> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1647> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1648> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1649> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1650> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1651> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1652> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1653> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1654> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1655> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1656> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1657> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1658> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1659> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1660> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1661> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1662> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1663> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1664> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1665> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1666> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1667> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1668> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1669> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1670> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1671> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1672> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1673> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1674> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1675> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1676> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1677> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1678> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1679> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1680> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1681> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1682> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1683> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1684> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1685> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1686> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1687> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1688> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1689> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1690> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1691> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1692> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1693> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1694> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1695> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1696> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1697> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1698> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1699> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1700> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1701> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1702> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1703> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1704> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1705> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1706> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1707> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1708> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1709> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1710> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1711> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1712> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1713> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1714> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1715> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1716> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1717> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1718> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1719> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1720> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1721> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1722> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1723> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1724> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1725> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1726> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1727> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1728> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1729> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1730> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1731> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1732> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1733> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1734> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1735> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1736> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1737> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1738> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1739> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1740> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1741> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1742> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1743> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1744> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1745> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1746> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1747> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1748> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1749> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1750> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1751> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1752> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1753> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1754> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1755> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1756> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1757> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1758> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1759> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1760> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1761> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1762> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1763> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1764> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1765> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1766> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1767> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1768> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1769> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1770> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1771> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1772> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1773> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1774> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1775> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1776> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1777> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1778> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1779> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1780> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1781> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1782> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1783> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1784> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1785> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1786> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1787> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1788> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1789> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1790> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1791> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1792> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1793> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1794> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1795> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1796> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1797> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1798> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1799> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1800> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1801> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1802> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1803> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1804> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1805> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1806> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1807> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1808> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1809> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1810> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1811> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1812> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1813> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1814> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1815> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1816> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1817> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1818> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1819> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1820> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1821> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1822> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1823> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1824> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1825> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1826> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1827> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1828> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1829> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1830> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1831> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1832> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1833> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1834> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1835> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1836> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1837> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1838> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1839> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1840> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1841> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1842> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1843> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1844> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1845> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1846> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1847> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1848> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1849> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1850> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1851> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1852> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1853> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1854> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1855> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1856> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1857> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1858> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1859> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1860> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1861> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1862> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1863> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1864> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1865> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1866> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1867> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1868> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1869> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1870> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1871> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1872> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1873> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1874> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1875> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1876> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1877> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1878> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1879> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1880> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1881> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1882> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1883> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1884> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1885> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1886> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1887> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1888> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1889> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1890> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1891> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1892> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1893> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1894> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1895> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1896> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1897> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1898> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1899> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1900> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1901> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1902> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1903> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1904> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1905> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1906> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1907> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1908> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1909> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1910> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1911> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1912> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1913> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1914> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1915> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1916> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1917> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1918> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1919> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1920> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1921> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1922> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1923> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1924> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1925> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1926> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1927> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1928> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1929> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1930> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1931> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1932> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1933> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1934> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1935> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1936> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1937> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1938> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1939> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1940> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1941> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1942> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1943> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1944> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1945> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1946> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1947> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1948> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1949> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1950> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1951> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1952> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1953> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1954> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1955> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1956> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1957> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1958> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1959> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1960> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1961> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1962> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1963> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1964> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1965> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1966> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1967> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1968> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1969> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1970> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1971> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1972> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1973> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1974> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1975> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1976> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1977> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1978> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1979> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1980> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1981> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1982> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1983> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1984> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1985> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1986> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1987> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1988> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1989> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1990> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1991> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1992> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1993> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1994> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1995> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1996> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1997> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1998> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<1999> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2000> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2001> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2002> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2003> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2004> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2005> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2006> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2007> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2008> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2009> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2010> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2011> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2012> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2013> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2014> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2015> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2016> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2017> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2018> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2019> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2020> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2021> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2022> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2023> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2024> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2025> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2026> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2027> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2028> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2029> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2030> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2031> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2032> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2033> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2034> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2035> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2036> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2037> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2038> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2039> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2040> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2041> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2042> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2043> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2044> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2045> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2046> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2047> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2048> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2049> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2050> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2051> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2052> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2053> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2054> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2055> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2056> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2057> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2058> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2059> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2060> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2061> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2062> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2063> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2064> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2065> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2066> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2067> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2068> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2069> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2070> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2071> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2072> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2073> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2074> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2075> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2076> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2077> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2078> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2079> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2080> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2081> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2082> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2083> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2084> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2085> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2086> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2087> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2088> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2089> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2090> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2091> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2092> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2093> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2094> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2095> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2096> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2097> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2098> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2099> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2100> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2101> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2102> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2103> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2104> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2105> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2106> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2107> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2108> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2109> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2110> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2111> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2112> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2113> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2114> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2115> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2116> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2117> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2118> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2119> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2120> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2121> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2122> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2123> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2124> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2125> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2126> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2127> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2128> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2129> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2130> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2131> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2132> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2133> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2134> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2135> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2136> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2137> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2138> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2139> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2140> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2141> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2142> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2143> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2144> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2145> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2146> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2147> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2148> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2149> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2150> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2151> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2152> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2153> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2154> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2155> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2156> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2157> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2158> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2159> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2160> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2161> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2162> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2163> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2164> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2165> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2166> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2167> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2168> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2169> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2170> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2171> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2172> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2173> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2174> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2175> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2176> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2177> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2178> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2179> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2180> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2181> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2182> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2183> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2184> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2185> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2186> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2187> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2188> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2189> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2190> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2191> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2192> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2193> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2194> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2195> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2196> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2197> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2198> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2199> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2200> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2201> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2202> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2203> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2204> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2205> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2206> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2207> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2208> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2209> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2210> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2211> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2212> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2213> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2214> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2215> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2216> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2217> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2218> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2219> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2220> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2221> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2222> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2223> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2224> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2225> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2226> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2227> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2228> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2229> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2230> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2231> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2232> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2233> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2234> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2235> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2236> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2237> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2238> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2239> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2240> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2241> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2242> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2243> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2244> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2245> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2246> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2247> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2248> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2249> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2250> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2251> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2252> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2253> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2254> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2255> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2256> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2257> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2258> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2259> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2260> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2261> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2262> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2263> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2264> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2265> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2266> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2267> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2268> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2269> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2270> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2271> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2272> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2273> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2274> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2275> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2276> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2277> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2278> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2279> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2280> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2281> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2282> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2283> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2284> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2285> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2286> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2287> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2288> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2289> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2290> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2291> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2292> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2293> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2294> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2295> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2296> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2297> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2298> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2299> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2300> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2301> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2302> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2303> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2304> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2305> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2306> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2307> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2308> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2309> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2310> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2311> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2312> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2313> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2314> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2315> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2316> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2317> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2318> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2319> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2320> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2321> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2322> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2323> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2324> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2325> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2326> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2327> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2328> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2329> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2330> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2331> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2332> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2333> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2334> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2335> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2336> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2337> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2338> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2339> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2340> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2341> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2342> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2343> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2344> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2345> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2346> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2347> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2348> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2349> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2350> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2351> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2352> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2353> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2354> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2355> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2356> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2357> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2358> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2359> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2360> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2361> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2362> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2363> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2364> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2365> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2366> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2367> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2368> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2369> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2370> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2371> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2372> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2373> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2374> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2375> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2376> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2377> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2378> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2379> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2380> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2381> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2382> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2383> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2384> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2385> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2386> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2387> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2388> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2389> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2390> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2391> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2392> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2393> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2394> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2395> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2396> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2397> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2398> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2399> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2400> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2401> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2402> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2403> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2404> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2405> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2406> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2407> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2408> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2409> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2410> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2411> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2412> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2413> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2414> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2415> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2416> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2417> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2418> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2419> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2420> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2421> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2422> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2423> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2424> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2425> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2426> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2427> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2428> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2429> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2430> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2431> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2432> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2433> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2434> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2435> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2436> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2437> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2438> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2439> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2440> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2441> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2442> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2443> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2444> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2445> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2446> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2447> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2448> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2449> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2450> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2451> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2452> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2453> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2454> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2455> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2456> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2457> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2458> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2459> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2460> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2461> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2462> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2463> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2464> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2465> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2466> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2467> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2468> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2469> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2470> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2471> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2472> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2473> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2474> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2475> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2476> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2477> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2478> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2479> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2480> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2481> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2482> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2483> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2484> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2485> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2486> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2487> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2488> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2489> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2490> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2491> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2492> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2493> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2494> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2495> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2496> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2497> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2498> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2499> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2500> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2501> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2502> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2503> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2504> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2505> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2506> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2507> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2508> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2509> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2510> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2511> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2512> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2513> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2514> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2515> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2516> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2517> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2518> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2519> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2520> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2521> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2522> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2523> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2524> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2525> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2526> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2527> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2528> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2529> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2530> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2531> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2532> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2533> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2534> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2535> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2536> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2537> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2538> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2539> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2540> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2541> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2542> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2543> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2544> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2545> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2546> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2547> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2548> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2549> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2550> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2551> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2552> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2553> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2554> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2555> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2556> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2557> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2558> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2559> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2560> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2561> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2562> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2563> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2564> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2565> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2566> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2567> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2568> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2569> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2570> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2571> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2572> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2573> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2574> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2575> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2576> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2577> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2578> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2579> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2580> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2581> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2582> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2583> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2584> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2585> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2586> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2587> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2588> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2589> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2590> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2591> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2592> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2593> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2594> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2595> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2596> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2597> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2598> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2599> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2600> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2601> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2602> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2603> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2604> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2605> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2606> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2607> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2608> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2609> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2610> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2611> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2612> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2613> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2614> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2615> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2616> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2617> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2618> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2619> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2620> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2621> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2622> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2623> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2624> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2625> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2626> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2627> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2628> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2629> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2630> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2631> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2632> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2633> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2634> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2635> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2636> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2637> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2638> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2639> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2640> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2641> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2642> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2643> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2644> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2645> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2646> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2647> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2648> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2649> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2650> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2651> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2652> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2653> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2654> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2655> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2656> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2657> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2658> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2659> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2660> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2661> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2662> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2663> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2664> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2665> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2666> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2667> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2668> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2669> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2670> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2671> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2672> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2673> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2674> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2675> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2676> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2677> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2678> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2679> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2680> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2681> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2682> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2683> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2684> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2685> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2686> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2687> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2688> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2689> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2690> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2691> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2692> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2693> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2694> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2695> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2696> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2697> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2698> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2699> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2700> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2701> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2702> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2703> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2704> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2705> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2706> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2707> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2708> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2709> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2710> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2711> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2712> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2713> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2714> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2715> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2716> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2717> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2718> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2719> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2720> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2721> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2722> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2723> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2724> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2725> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2726> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2727> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2728> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2729> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2730> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2731> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2732> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2733> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2734> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2735> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2736> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2737> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2738> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2739> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2740> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2741> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2742> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2743> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2744> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2745> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2746> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2747> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2748> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2749> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2750> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2751> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2752> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2753> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2754> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2755> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2756> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2757> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2758> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2759> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2760> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2761> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2762> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2763> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2764> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2765> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2766> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2767> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2768> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2769> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2770> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2771> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2772> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2773> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2774> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2775> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2776> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2777> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2778> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2779> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2780> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2781> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2782> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2783> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2784> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2785> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2786> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2787> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2788> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2789> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2790> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2791> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2792> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2793> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2794> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2795> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2796> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2797> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2798> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2799> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2800> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2801> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2802> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2803> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2804> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2805> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2806> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2807> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2808> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2809> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2810> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2811> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2812> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2813> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2814> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2815> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2816> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2817> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2818> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2819> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2820> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2821> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2822> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2823> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2824> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2825> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2826> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2827> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2828> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2829> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2830> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2831> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2832> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2833> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2834> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2835> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2836> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2837> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2838> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2839> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2840> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2841> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2842> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2843> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2844> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2845> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2846> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2847> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2848> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2849> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2850> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2851> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2852> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2853> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2854> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2855> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2856> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2857> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2858> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2859> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2860> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2861> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2862> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2863> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2864> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2865> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2866> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2867> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2868> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2869> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2870> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2871> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2872> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2873> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2874> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2875> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2876> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2877> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2878> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2879> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2880> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2881> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2882> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2883> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2884> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2885> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2886> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2887> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2888> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2889> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2890> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2891> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2892> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2893> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2894> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2895> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2896> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2897> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2898> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2899> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2900> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2901> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2902> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2903> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2904> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2905> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2906> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2907> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2908> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2909> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2910> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2911> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2912> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2913> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2914> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2915> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2916> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2917> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2918> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2919> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2920> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2921> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2922> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2923> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2924> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2925> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2926> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2927> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2928> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2929> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2930> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2931> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2932> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2933> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2934> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2935> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2936> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2937> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2938> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2939> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2940> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2941> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2942> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2943> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2944> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2945> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2946> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2947> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2948> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2949> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2950> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2951> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2952> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2953> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2954> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2955> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2956> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2957> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2958> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2959> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2960> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2961> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2962> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2963> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2964> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2965> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2966> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2967> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2968> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2969> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2970> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2971> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2972> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2973> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2974> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2975> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2976> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2977> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2978> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2979> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2980> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2981> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2982> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2983> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2984> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2985> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2986> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2987> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2988> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2989> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2990> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2991> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2992> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2993> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2994> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2995> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2996> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2997> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2998> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<2999> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3000> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3001> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3002> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3003> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3004> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3005> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3006> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3007> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3008> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3009> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3010> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3011> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3012> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3013> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3014> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3015> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3016> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3017> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3018> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3019> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3020> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3021> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3022> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3023> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3024> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3025> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3026> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3027> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3028> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3029> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3030> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3031> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3032> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3033> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3034> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3035> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3036> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3037> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3038> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3039> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3040> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3041> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3042> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3043> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3044> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3045> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3046> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3047> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3048> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3049> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3050> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3051> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3052> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3053> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3054> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3055> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3056> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3057> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3058> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3059> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3060> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3061> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3062> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3063> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3064> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3065> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3066> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3067> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3068> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3069> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3070> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3071> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3072> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3073> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3074> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3075> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3076> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3077> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3078> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3079> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3080> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3081> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3082> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3083> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3084> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3085> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3086> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3087> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3088> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3089> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3090> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3091> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3092> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3093> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3094> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3095> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3096> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3097> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3098> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3099> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3100> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3101> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3102> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3103> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3104> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3105> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3106> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3107> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3108> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3109> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3110> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3111> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3112> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3113> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3114> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3115> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3116> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3117> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3118> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3119> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3120> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3121> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3122> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3123> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3124> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3125> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3126> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3127> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3128> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3129> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3130> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3131> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3132> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3133> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3134> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3135> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3136> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3137> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3138> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3139> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3140> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3141> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3142> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3143> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3144> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3145> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3146> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3147> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3148> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3149> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3150> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3151> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3152> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3153> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3154> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3155> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3156> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3157> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3158> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3159> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3160> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3161> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3162> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3163> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3164> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3165> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3166> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3167> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3168> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3169> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3170> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3171> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3172> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3173> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3174> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3175> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3176> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3177> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3178> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3179> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3180> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3181> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3182> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3183> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3184> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3185> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3186> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3187> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3188> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3189> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3190> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3191> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3192> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3193> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3194> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3195> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3196> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3197> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3198> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3199> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3200> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3201> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3202> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3203> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3204> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3205> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3206> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3207> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3208> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3209> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3210> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3211> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3212> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3213> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3214> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3215> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3216> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3217> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3218> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3219> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3220> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3221> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3222> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3223> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3224> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3225> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3226> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3227> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3228> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3229> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3230> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3231> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3232> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3233> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3234> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3235> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3236> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3237> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3238> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3239> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3240> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3241> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3242> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3243> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3244> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3245> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3246> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3247> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3248> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3249> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3250> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3251> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3252> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3253> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3254> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3255> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3256> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3257> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3258> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3259> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3260> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3261> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3262> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3263> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3264> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3265> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3266> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3267> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3268> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3269> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3270> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3271> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3272> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3273> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3274> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3275> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3276> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3277> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3278> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3279> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3280> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3281> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3282> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3283> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3284> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3285> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3286> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3287> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3288> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3289> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3290> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3291> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3292> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3293> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3294> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3295> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3296> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3297> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3298> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3299> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3300> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3301> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3302> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3303> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3304> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3305> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3306> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3307> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3308> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3309> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3310> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3311> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3312> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3313> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3314> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3315> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3316> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3317> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3318> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3319> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3320> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3321> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3322> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3323> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3324> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3325> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3326> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3327> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3328> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3329> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3330> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3331> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3332> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3333> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3334> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3335> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3336> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3337> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3338> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3339> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3340> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3341> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3342> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3343> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3344> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3345> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3346> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3347> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3348> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3349> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3350> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3351> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3352> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3353> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3354> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3355> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3356> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3357> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3358> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3359> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3360> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3361> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3362> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3363> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3364> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3365> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3366> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3367> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3368> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3369> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3370> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3371> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3372> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3373> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3374> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3375> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3376> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3377> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3378> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3379> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3380> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3381> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3382> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3383> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3384> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3385> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3386> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3387> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3388> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3389> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3390> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3391> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3392> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3393> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3394> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3395> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3396> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3397> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3398> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3399> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3400> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3401> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3402> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3403> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3404> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3405> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3406> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3407> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3408> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3409> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3410> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3411> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3412> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3413> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3414> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3415> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3416> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3417> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3418> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3419> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3420> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3421> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3422> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3423> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3424> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3425> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3426> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3427> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3428> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3429> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3430> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3431> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3432> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3433> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3434> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3435> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3436> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3437> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3438> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3439> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3440> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3441> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3442> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3443> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3444> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3445> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3446> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3447> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3448> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3449> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3450> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3451> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3452> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3453> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3454> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3455> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3456> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3457> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3458> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3459> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3460> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3461> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3462> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3463> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3464> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3465> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3466> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3467> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3468> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3469> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3470> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3471> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3472> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3473> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3474> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3475> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3476> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3477> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3478> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3479> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3480> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3481> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3482> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3483> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3484> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3485> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3486> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3487> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3488> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3489> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3490> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3491> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3492> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3493> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3494> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3495> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3496> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3497> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3498> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3499> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3500> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3501> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3502> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3503> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3504> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3505> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3506> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3507> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3508> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3509> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3510> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3511> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3512> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3513> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3514> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3515> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3516> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3517> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3518> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3519> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3520> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3521> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3522> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3523> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3524> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3525> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3526> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3527> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3528> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3529> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3530> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3531> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3532> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3533> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3534> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3535> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3536> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3537> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3538> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3539> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3540> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3541> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3542> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3543> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3544> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3545> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3546> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3547> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3548> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3549> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3550> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3551> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3552> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3553> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3554> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3555> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3556> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3557> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3558> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3559> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3560> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3561> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3562> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3563> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3564> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3565> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3566> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3567> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3568> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3569> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3570> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3571> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3572> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3573> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3574> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3575> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3576> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3577> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3578> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3579> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3580> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3581> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3582> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3583> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3584> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3585> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3586> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3587> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3588> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3589> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3590> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3591> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3592> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3593> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3594> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3595> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3596> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3597> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3598> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3599> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3600> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3601> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3602> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3603> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3604> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3605> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3606> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3607> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3608> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3609> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3610> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3611> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3612> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3613> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3614> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3615> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3616> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3617> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3618> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3619> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3620> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3621> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3622> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3623> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3624> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3625> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3626> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3627> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3628> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3629> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3630> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3631> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3632> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3633> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3634> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3635> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3636> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3637> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3638> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3639> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3640> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3641> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3642> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3643> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3644> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3645> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3646> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3647> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3648> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3649> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3650> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3651> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3652> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3653> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3654> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3655> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3656> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3657> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3658> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3659> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3660> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3661> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3662> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3663> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3664> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3665> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3666> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3667> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3668> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3669> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3670> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3671> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3672> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3673> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3674> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3675> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3676> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3677> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3678> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3679> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3680> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3681> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3682> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3683> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3684> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3685> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3686> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3687> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3688> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3689> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3690> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3691> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3692> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3693> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3694> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3695> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3696> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3697> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3698> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3699> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3700> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3701> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3702> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3703> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3704> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3705> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3706> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3707> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3708> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3709> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3710> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3711> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3712> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3713> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3714> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3715> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3716> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3717> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3718> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3719> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3720> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3721> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3722> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3723> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3724> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3725> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3726> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3727> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3728> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3729> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3730> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3731> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3732> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3733> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3734> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3735> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3736> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3737> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3738> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3739> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3740> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3741> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3742> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3743> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3744> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3745> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3746> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3747> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3748> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3749> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3750> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3751> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3752> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3753> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3754> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3755> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3756> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3757> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3758> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3759> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3760> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3761> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3762> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3763> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3764> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3765> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3766> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3767> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3768> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3769> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3770> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3771> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3772> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3773> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3774> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3775> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3776> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3777> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3778> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3779> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3780> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3781> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3782> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3783> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3784> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3785> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3786> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3787> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3788> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3789> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3790> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3791> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3792> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3793> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3794> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3795> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3796> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3797> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3798> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3799> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3800> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3801> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3802> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3803> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3804> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3805> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3806> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3807> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3808> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3809> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3810> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3811> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3812> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3813> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3814> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3815> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3816> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3817> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3818> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3819> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3820> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3821> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3822> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3823> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3824> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3825> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3826> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3827> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3828> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3829> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3830> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3831> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3832> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3833> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3834> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3835> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3836> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3837> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3838> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3839> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3840> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3841> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3842> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3843> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3844> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3845> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3846> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3847> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3848> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3849> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3850> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3851> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3852> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3853> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3854> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3855> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3856> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3857> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3858> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3859> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3860> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3861> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3862> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3863> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3864> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3865> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3866> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3867> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3868> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3869> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3870> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3871> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3872> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3873> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3874> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3875> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3876> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3877> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3878> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3879> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3880> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3881> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3882> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3883> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3884> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3885> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3886> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3887> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3888> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3889> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3890> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3891> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3892> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3893> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3894> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3895> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3896> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3897> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3898> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3899> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3900> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3901> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3902> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3903> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3904> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3905> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3906> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3907> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3908> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3909> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3910> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3911> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3912> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3913> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3914> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3915> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3916> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3917> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3918> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3919> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3920> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3921> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3922> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3923> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3924> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3925> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3926> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3927> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3928> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3929> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3930> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3931> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3932> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3933> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3934> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3935> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3936> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3937> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3938> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3939> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3940> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3941> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3942> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3943> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3944> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3945> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3946> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3947> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3948> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3949> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3950> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3951> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3952> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3953> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3954> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3955> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3956> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3957> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3958> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3959> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3960> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3961> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3962> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3963> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3964> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3965> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3966> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3967> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3968> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3969> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3970> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3971> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3972> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3973> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3974> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3975> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3976> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3977> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3978> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3979> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3980> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3981> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3982> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3983> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3984> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3985> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3986> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3987> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3988> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3989> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3990> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3991> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3992> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3993> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3994> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3995> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3996> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3997> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3998> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<3999> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4000> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4001> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4002> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4003> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4004> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4005> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4006> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4007> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4008> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4009> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4010> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4011> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4012> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4013> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4014> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4015> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4016> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4017> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4018> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4019> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4020> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4021> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4022> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4023> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4024> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4025> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4026> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4027> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4028> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4029> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4030> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4031> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4032> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4033> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4034> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4035> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4036> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4037> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4038> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4039> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4040> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4041> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4042> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4043> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4044> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4045> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4046> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4047> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4048> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4049> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4050> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4051> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4052> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4053> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4054> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4055> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4056> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4057> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4058> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4059> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4060> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4061> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4062> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4063> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4064> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4065> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4066> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4067> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4068> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4069> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4070> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4071> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4072> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4073> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4074> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4075> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4076> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4077> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4078> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4079> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4080> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4081> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4082> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4083> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4084> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4085> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4086> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4087> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4088> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4089> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4090> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4091> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4092> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4093> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4094> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<4095> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
