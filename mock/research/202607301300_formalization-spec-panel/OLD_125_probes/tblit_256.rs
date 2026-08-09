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
pub type NatOf<const W: u16> = <Idx<W> as AdmittedWidth>::Out;
