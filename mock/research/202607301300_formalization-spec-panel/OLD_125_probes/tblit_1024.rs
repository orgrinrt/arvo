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
pub type NatOf<const W: u16> = <Idx<W> as AdmittedWidth>::Out;
