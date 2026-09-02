#![no_std]
#![feature(adt_const_params)]
use mach::*;
pub type N0 = Num<1, 0, OneYes, Hot>;
pub const W0: usize = 1;
pub fn one_ok_0() {
    <N0 as HasOne>::witness()
}
pub const F0_IS_ZERO: bool = true;
pub type N1 = Num<2, 0, OneYes, Warm>;
pub const W1: usize = 2;
pub fn one_ok_1() {
    <N1 as HasOne>::witness()
}
pub const F1_IS_ZERO: bool = true;
pub type N2 = Num<3, 0, OneYes, Cold>;
pub const W2: usize = 3;
pub fn one_ok_2() {
    <N2 as HasOne>::witness()
}
pub const F2_IS_ZERO: bool = true;
pub type N3 = Num<4, 0, OneYes, Hot>;
pub const W3: usize = 4;
pub fn one_ok_3() {
    <N3 as HasOne>::witness()
}
pub const F3_IS_ZERO: bool = true;
pub type N4 = Num<5, 0, OneYes, Warm>;
pub const W4: usize = 5;
pub fn one_ok_4() {
    <N4 as HasOne>::witness()
}
pub const F4_IS_ZERO: bool = true;
pub type N5 = Num<6, 0, OneYes, Cold>;
pub const W5: usize = 6;
pub fn one_ok_5() {
    <N5 as HasOne>::witness()
}
pub const F5_IS_ZERO: bool = true;
pub type N6 = Num<7, 0, OneYes, Hot>;
pub const W6: usize = 7;
pub fn one_ok_6() {
    <N6 as HasOne>::witness()
}
pub const F6_IS_ZERO: bool = true;
pub type N7 = Num<11, 0, OneYes, Warm>;
pub const W7: usize = 11;
pub fn one_ok_7() {
    <N7 as HasOne>::witness()
}
pub const F7_IS_ZERO: bool = true;
pub type N8 = Num<14, 0, OneYes, Cold>;
pub const W8: usize = 14;
pub fn one_ok_8() {
    <N8 as HasOne>::witness()
}
pub const F8_IS_ZERO: bool = true;
pub type N9 = Num<16, 0, OneYes, Hot>;
pub const W9: usize = 16;
pub fn one_ok_9() {
    <N9 as HasOne>::witness()
}
pub const F9_IS_ZERO: bool = true;
pub type N10 = Num<27, 0, OneYes, Warm>;
pub const W10: usize = 27;
pub fn one_ok_10() {
    <N10 as HasOne>::witness()
}
pub const F10_IS_ZERO: bool = true;
pub type N11 = Num<28, 0, OneYes, Cold>;
pub const W11: usize = 28;
pub fn one_ok_11() {
    <N11 as HasOne>::witness()
}
pub const F11_IS_ZERO: bool = true;
pub type N12 = Num<64, 0, OneYes, Hot>;
pub const W12: usize = 64;
pub fn one_ok_12() {
    <N12 as HasOne>::witness()
}
pub const F12_IS_ZERO: bool = true;
pub type N13 = Num<0, 16, OneNo, Warm>;
pub const W13: usize = 16;
pub const F13_IS_ZERO: bool = false;
pub type N14 = Num<8, 0, OneYes, Cold>;
pub const W14: usize = 8;
pub fn one_ok_14() {
    <N14 as HasOne>::witness()
}
pub const F14_IS_ZERO: bool = true;
pub type N15 = Num<9, 0, OneYes, Hot>;
pub const W15: usize = 9;
pub fn one_ok_15() {
    <N15 as HasOne>::witness()
}
pub const F15_IS_ZERO: bool = true;
pub type N16 = Num<10, 0, OneYes, Warm>;
pub const W16: usize = 10;
pub fn one_ok_16() {
    <N16 as HasOne>::witness()
}
pub const F16_IS_ZERO: bool = true;
pub type N17 = Num<12, 0, OneYes, Cold>;
pub const W17: usize = 12;
pub fn one_ok_17() {
    <N17 as HasOne>::witness()
}
pub const F17_IS_ZERO: bool = true;
pub type N18 = Num<13, 0, OneYes, Hot>;
pub const W18: usize = 13;
pub fn one_ok_18() {
    <N18 as HasOne>::witness()
}
pub const F18_IS_ZERO: bool = true;
pub type N19 = Num<18, 0, OneYes, Warm>;
pub const W19: usize = 18;
pub fn one_ok_19() {
    <N19 as HasOne>::witness()
}
pub const F19_IS_ZERO: bool = true;
pub type N20 = Num<21, 0, OneYes, Cold>;
pub const W20: usize = 21;
pub fn one_ok_20() {
    <N20 as HasOne>::witness()
}
pub const F20_IS_ZERO: bool = true;
pub type N21 = Num<23, 0, OneYes, Hot>;
pub const W21: usize = 23;
pub fn one_ok_21() {
    <N21 as HasOne>::witness()
}
pub const F21_IS_ZERO: bool = true;
pub type N22 = Num<34, 0, OneYes, Warm>;
pub const W22: usize = 34;
pub fn one_ok_22() {
    <N22 as HasOne>::witness()
}
pub const F22_IS_ZERO: bool = true;
pub type N23 = Num<35, 0, OneYes, Cold>;
pub const W23: usize = 35;
pub fn one_ok_23() {
    <N23 as HasOne>::witness()
}
pub const F23_IS_ZERO: bool = true;
pub type N24 = Num<71, 0, OneYes, Hot>;
pub const W24: usize = 71;
pub fn one_ok_24() {
    <N24 as HasOne>::witness()
}
pub const F24_IS_ZERO: bool = true;
pub type N25 = Num<0, 17, OneNo, Warm>;
pub const W25: usize = 17;
pub const F25_IS_ZERO: bool = false;
pub type N26 = Num<15, 0, OneYes, Cold>;
pub const W26: usize = 15;
pub fn one_ok_26() {
    <N26 as HasOne>::witness()
}
pub const F26_IS_ZERO: bool = true;
pub type N27 = Num<17, 0, OneYes, Hot>;
pub const W27: usize = 17;
pub fn one_ok_27() {
    <N27 as HasOne>::witness()
}
pub const F27_IS_ZERO: bool = true;
pub type N28 = Num<19, 0, OneYes, Warm>;
pub const W28: usize = 19;
pub fn one_ok_28() {
    <N28 as HasOne>::witness()
}
pub const F28_IS_ZERO: bool = true;
pub type N29 = Num<20, 0, OneYes, Cold>;
pub const W29: usize = 20;
pub fn one_ok_29() {
    <N29 as HasOne>::witness()
}
pub const F29_IS_ZERO: bool = true;
pub type N30 = Num<25, 0, OneYes, Hot>;
pub const W30: usize = 25;
pub fn one_ok_30() {
    <N30 as HasOne>::witness()
}
pub const F30_IS_ZERO: bool = true;
pub type N31 = Num<30, 0, OneYes, Warm>;
pub const W31: usize = 30;
pub fn one_ok_31() {
    <N31 as HasOne>::witness()
}
pub const F31_IS_ZERO: bool = true;
pub type N32 = Num<41, 0, OneYes, Cold>;
pub const W32: usize = 41;
pub fn one_ok_32() {
    <N32 as HasOne>::witness()
}
pub const F32_IS_ZERO: bool = true;
pub type N33 = Num<42, 0, OneYes, Hot>;
pub const W33: usize = 42;
pub fn one_ok_33() {
    <N33 as HasOne>::witness()
}
pub const F33_IS_ZERO: bool = true;
pub type N34 = Num<78, 0, OneYes, Warm>;
pub const W34: usize = 78;
pub fn one_ok_34() {
    <N34 as HasOne>::witness()
}
pub const F34_IS_ZERO: bool = true;
pub type N35 = Num<0, 18, OneNo, Cold>;
pub const W35: usize = 18;
pub const F35_IS_ZERO: bool = false;
pub type N36 = Num<22, 0, OneYes, Hot>;
pub const W36: usize = 22;
pub fn one_ok_36() {
    <N36 as HasOne>::witness()
}
pub const F36_IS_ZERO: bool = true;
pub type N37 = Num<24, 0, OneYes, Warm>;
pub const W37: usize = 24;
pub fn one_ok_37() {
    <N37 as HasOne>::witness()
}
pub const F37_IS_ZERO: bool = true;
pub type N38 = Num<26, 0, OneYes, Cold>;
pub const W38: usize = 26;
pub fn one_ok_38() {
    <N38 as HasOne>::witness()
}
pub const F38_IS_ZERO: bool = true;
pub type N39 = Num<32, 0, OneYes, Hot>;
pub const W39: usize = 32;
pub fn one_ok_39() {
    <N39 as HasOne>::witness()
}
pub const F39_IS_ZERO: bool = true;
pub type N40 = Num<37, 0, OneYes, Warm>;
pub const W40: usize = 37;
pub fn one_ok_40() {
    <N40 as HasOne>::witness()
}
pub const F40_IS_ZERO: bool = true;
pub type N41 = Num<48, 0, OneYes, Cold>;
pub const W41: usize = 48;
pub fn one_ok_41() {
    <N41 as HasOne>::witness()
}
pub const F41_IS_ZERO: bool = true;
pub type N42 = Num<49, 0, OneYes, Hot>;
pub const W42: usize = 49;
pub fn one_ok_42() {
    <N42 as HasOne>::witness()
}
pub const F42_IS_ZERO: bool = true;
pub type N43 = Num<85, 0, OneYes, Warm>;
pub const W43: usize = 85;
pub fn one_ok_43() {
    <N43 as HasOne>::witness()
}
pub const F43_IS_ZERO: bool = true;
pub type N44 = Num<0, 19, OneNo, Cold>;
pub const W44: usize = 19;
pub const F44_IS_ZERO: bool = false;
pub type N45 = Num<29, 0, OneYes, Hot>;
pub const W45: usize = 29;
pub fn one_ok_45() {
    <N45 as HasOne>::witness()
}
pub const F45_IS_ZERO: bool = true;
pub type N46 = Num<31, 0, OneYes, Warm>;
pub const W46: usize = 31;
pub fn one_ok_46() {
    <N46 as HasOne>::witness()
}
pub const F46_IS_ZERO: bool = true;
pub type N47 = Num<33, 0, OneYes, Cold>;
pub const W47: usize = 33;
pub fn one_ok_47() {
    <N47 as HasOne>::witness()
}
pub const F47_IS_ZERO: bool = true;
pub type N48 = Num<39, 0, OneYes, Hot>;
pub const W48: usize = 39;
pub fn one_ok_48() {
    <N48 as HasOne>::witness()
}
pub const F48_IS_ZERO: bool = true;
pub type N49 = Num<44, 0, OneYes, Warm>;
pub const W49: usize = 44;
pub fn one_ok_49() {
    <N49 as HasOne>::witness()
}
pub const F49_IS_ZERO: bool = true;
pub const SW0: usize = 5;
pub const SW1: usize = 7;
pub const SW2: usize = 9;
pub const SW3: usize = 11;
pub const SW4: usize = 16;
pub const SW5: usize = 20;
pub const SW6: usize = 23;
pub const SW7: usize = 38;
pub const SW8: usize = 42;
pub const SW9: usize = 80;
pub const SW10: usize = 43;
pub const SW11: usize = 36;
pub const SW12: usize = 73;
pub const SW13: usize = 26;
pub const SW14: usize = 20;
pub const SW15: usize = 22;
pub const SW16: usize = 28;
pub const SW17: usize = 33;
pub const SW18: usize = 36;
pub const SW19: usize = 52;
pub const SW20: usize = 56;
pub const SW21: usize = 94;
pub const SW22: usize = 51;
pub const SW23: usize = 50;
pub const SW24: usize = 88;
pub const SW25: usize = 36;
pub const SW26: usize = 35;
pub const SW27: usize = 42;
pub const SW28: usize = 49;
pub const SW29: usize = 61;
pub const SW30: usize = 67;
pub const SW31: usize = 108;
pub const SW32: usize = 59;
pub const SW33: usize = 64;
pub const SW34: usize = 102;
pub const SW35: usize = 44;
pub const SW36: usize = 54;
pub const SW37: usize = 61;
pub const SW38: usize = 74;
pub const SW39: usize = 81;
pub const SW40: usize = 122;
pub const SW41: usize = 67;
pub const SW42: usize = 78;
pub const SW43: usize = 116;
pub const SW44: usize = 52;
pub const SW45: usize = 68;
pub const SW46: usize = 75;
pub type C0 = Slot<1>;
pub fn build0() -> <C0 as Capacity>::Array<u32> {
    C0::build(0)
}
pub type C1 = Slot<3>;
pub fn build1() -> <C1 as Capacity>::Array<u32> {
    C1::build(0)
}
pub type C2 = Slot<4>;
pub fn build2() -> <C2 as Capacity>::Array<u32> {
    C2::build(0)
}
pub type C3 = Slot<7>;
pub fn build3() -> <C3 as Capacity>::Array<u32> {
    C3::build(0)
}
pub type C4 = Slot<8>;
pub fn build4() -> <C4 as Capacity>::Array<u32> {
    C4::build(0)
}
pub type C5 = Slot<13>;
pub fn build5() -> <C5 as Capacity>::Array<u32> {
    C5::build(0)
}
pub type C6 = Slot<16>;
pub fn build6() -> <C6 as Capacity>::Array<u32> {
    C6::build(0)
}
pub type C7 = Slot<28>;
pub fn build7() -> <C7 as Capacity>::Array<u32> {
    C7::build(0)
}
pub type C8 = Slot<32>;
pub fn build8() -> <C8 as Capacity>::Array<u32> {
    C8::build(0)
}
pub type C9 = Slot<64>;
pub fn build9() -> <C9 as Capacity>::Array<u32> {
    C9::build(0)
}
pub type C10 = Slot<12>;
pub fn build10() -> <C10 as Capacity>::Array<u32> {
    C10::build(0)
}
pub type C11 = Slot<14>;
pub fn build11() -> <C11 as Capacity>::Array<u32> {
    C11::build(0)
}
pub type C12 = Slot<15>;
pub fn build12() -> <C12 as Capacity>::Array<u32> {
    C12::build(0)
}
pub type C13 = Slot<18>;
pub fn build13() -> <C13 as Capacity>::Array<u32> {
    C13::build(0)
}
pub type C14 = Slot<19>;
pub fn build14() -> <C14 as Capacity>::Array<u32> {
    C14::build(0)
}
pub type C15 = Slot<24>;
pub fn build15() -> <C15 as Capacity>::Array<u32> {
    C15::build(0)
}
pub type C16 = Slot<27>;
pub fn build16() -> <C16 as Capacity>::Array<u32> {
    C16::build(0)
}
pub type C17 = Slot<39>;
pub fn build17() -> <C17 as Capacity>::Array<u32> {
    C17::build(0)
}
pub type C18 = Slot<43>;
pub fn build18() -> <C18 as Capacity>::Array<u32> {
    C18::build(0)
}
pub type C19 = Slot<75>;
pub fn build19() -> <C19 as Capacity>::Array<u32> {
    C19::build(0)
}
pub type C20 = Slot<23>;
pub fn build20() -> <C20 as Capacity>::Array<u32> {
    C20::build(0)
}
pub type C21 = Slot<25>;
pub fn build21() -> <C21 as Capacity>::Array<u32> {
    C21::build(0)
}
pub type C22 = Slot<26>;
pub fn build22() -> <C22 as Capacity>::Array<u32> {
    C22::build(0)
}
pub type C23 = Slot<29>;
pub fn build23() -> <C23 as Capacity>::Array<u32> {
    C23::build(0)
}
pub type C24 = Slot<30>;
pub fn build24() -> <C24 as Capacity>::Array<u32> {
    C24::build(0)
}
pub type C25 = Slot<35>;
pub fn build25() -> <C25 as Capacity>::Array<u32> {
    C25::build(0)
}
pub type C26 = Slot<38>;
pub fn build26() -> <C26 as Capacity>::Array<u32> {
    C26::build(0)
}
pub type C27 = Slot<50>;
pub fn build27() -> <C27 as Capacity>::Array<u32> {
    C27::build(0)
}
pub type C28 = Slot<54>;
pub fn build28() -> <C28 as Capacity>::Array<u32> {
    C28::build(0)
}
pub type C29 = Slot<86>;
pub fn build29() -> <C29 as Capacity>::Array<u32> {
    C29::build(0)
}
pub type C30 = Slot<34>;
pub fn build30() -> <C30 as Capacity>::Array<u32> {
    C30::build(0)
}
pub type C31 = Slot<36>;
pub fn build31() -> <C31 as Capacity>::Array<u32> {
    C31::build(0)
}
pub type C32 = Slot<37>;
pub fn build32() -> <C32 as Capacity>::Array<u32> {
    C32::build(0)
}
pub type C33 = Slot<40>;
pub fn build33() -> <C33 as Capacity>::Array<u32> {
    C33::build(0)
}
pub type C34 = Slot<41>;
pub fn build34() -> <C34 as Capacity>::Array<u32> {
    C34::build(0)
}
pub type C35 = Slot<46>;
pub fn build35() -> <C35 as Capacity>::Array<u32> {
    C35::build(0)
}
pub type C36 = Slot<49>;
pub fn build36() -> <C36 as Capacity>::Array<u32> {
    C36::build(0)
}
pub type C37 = Slot<61>;
pub fn build37() -> <C37 as Capacity>::Array<u32> {
    C37::build(0)
}
pub type C38 = Slot<65>;
pub fn build38() -> <C38 as Capacity>::Array<u32> {
    C38::build(0)
}
pub type C39 = Slot<97>;
pub fn build39() -> <C39 as Capacity>::Array<u32> {
    C39::build(0)
}
pub type C40 = Slot<45>;
pub fn build40() -> <C40 as Capacity>::Array<u32> {
    C40::build(0)
}
pub type C41 = Slot<47>;
pub fn build41() -> <C41 as Capacity>::Array<u32> {
    C41::build(0)
}
pub type C42 = Slot<48>;
pub fn build42() -> <C42 as Capacity>::Array<u32> {
    C42::build(0)
}
pub type C43 = Slot<51>;
pub fn build43() -> <C43 as Capacity>::Array<u32> {
    C43::build(0)
}
pub type C44 = Slot<52>;
pub fn build44() -> <C44 as Capacity>::Array<u32> {
    C44::build(0)
}
pub type C45 = Slot<57>;
pub fn build45() -> <C45 as Capacity>::Array<u32> {
    C45::build(0)
}
pub type C46 = Slot<60>;
pub fn build46() -> <C46 as Capacity>::Array<u32> {
    C46::build(0)
}
pub type C47 = Slot<72>;
pub fn build47() -> <C47 as Capacity>::Array<u32> {
    C47::build(0)
}
pub type C48 = Slot<76>;
pub fn build48() -> <C48 as Capacity>::Array<u32> {
    C48::build(0)
}
pub type C49 = Slot<108>;
pub fn build49() -> <C49 as Capacity>::Array<u32> {
    C49::build(0)
}

pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0;
    while i < s.len() {
        s[i] = s[i].wrapping_add(i as u32);
        i += 1;
    }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32;
    let mut j = 0;
    while j < r.len() {
        acc = acc.wrapping_add(r[j]);
        j += 1;
    }
    acc
}
pub fn scaled_fold<const I: u16, const F: u16, S: Strategy, C: Capacity>(seed: u32) -> u32
where
    UFixed<I, F, S>: Stored,
{
    fold_generic::<C>(seed).wrapping_mul(<UFixed<I, F, S> as Stored>::W as u32)
}

pub fn call0() -> u32 {
    scaled_fold::<1, 0, Hot, C0>(0)
}
pub fn call1() -> u32 {
    scaled_fold::<2, 0, Warm, C1>(1)
}
pub fn call2() -> u32 {
    scaled_fold::<3, 0, Cold, C2>(2)
}
pub fn call3() -> u32 {
    scaled_fold::<4, 0, Hot, C3>(3)
}
pub fn call4() -> u32 {
    scaled_fold::<5, 0, Warm, C4>(4)
}
pub fn call5() -> u32 {
    scaled_fold::<6, 0, Cold, C5>(5)
}
pub fn call6() -> u32 {
    scaled_fold::<7, 0, Hot, C6>(6)
}
pub fn call7() -> u32 {
    scaled_fold::<11, 0, Warm, C7>(7)
}
pub fn call8() -> u32 {
    scaled_fold::<14, 0, Cold, C8>(8)
}
pub fn call9() -> u32 {
    scaled_fold::<16, 0, Hot, C9>(9)
}
pub fn call10() -> u32 {
    scaled_fold::<27, 0, Warm, C10>(10)
}
pub fn call11() -> u32 {
    scaled_fold::<28, 0, Cold, C11>(11)
}
pub fn call12() -> u32 {
    scaled_fold::<64, 0, Hot, C12>(12)
}
pub fn call14() -> u32 {
    scaled_fold::<8, 0, Cold, C14>(14)
}
pub fn call15() -> u32 {
    scaled_fold::<9, 0, Hot, C15>(15)
}
pub fn call16() -> u32 {
    scaled_fold::<10, 0, Warm, C16>(16)
}
pub fn call17() -> u32 {
    scaled_fold::<12, 0, Cold, C17>(17)
}
pub fn call18() -> u32 {
    scaled_fold::<13, 0, Hot, C18>(18)
}
pub fn call19() -> u32 {
    scaled_fold::<18, 0, Warm, C19>(19)
}
pub fn call20() -> u32 {
    scaled_fold::<21, 0, Cold, C20>(20)
}
pub fn call21() -> u32 {
    scaled_fold::<23, 0, Hot, C21>(21)
}
pub fn call22() -> u32 {
    scaled_fold::<34, 0, Warm, C22>(22)
}
pub fn call23() -> u32 {
    scaled_fold::<35, 0, Cold, C23>(23)
}
pub fn call24() -> u32 {
    scaled_fold::<71, 0, Hot, C24>(24)
}
pub fn call26() -> u32 {
    scaled_fold::<15, 0, Cold, C26>(26)
}
pub fn call27() -> u32 {
    scaled_fold::<17, 0, Hot, C27>(27)
}
pub fn call28() -> u32 {
    scaled_fold::<19, 0, Warm, C28>(28)
}
pub fn call29() -> u32 {
    scaled_fold::<20, 0, Cold, C29>(29)
}
pub fn call30() -> u32 {
    scaled_fold::<25, 0, Hot, C30>(30)
}
pub fn call31() -> u32 {
    scaled_fold::<30, 0, Warm, C31>(31)
}
pub fn call32() -> u32 {
    scaled_fold::<41, 0, Cold, C32>(32)
}
pub fn call33() -> u32 {
    scaled_fold::<42, 0, Hot, C33>(33)
}
pub fn call34() -> u32 {
    scaled_fold::<78, 0, Warm, C34>(34)
}
pub fn call36() -> u32 {
    scaled_fold::<22, 0, Hot, C36>(36)
}
pub fn call37() -> u32 {
    scaled_fold::<24, 0, Warm, C37>(37)
}
pub fn call38() -> u32 {
    scaled_fold::<26, 0, Cold, C38>(38)
}
pub fn call39() -> u32 {
    scaled_fold::<32, 0, Hot, C39>(39)
}
pub fn call40() -> u32 {
    scaled_fold::<37, 0, Warm, C40>(40)
}
pub fn call41() -> u32 {
    scaled_fold::<48, 0, Cold, C41>(41)
}
pub fn call42() -> u32 {
    scaled_fold::<49, 0, Hot, C42>(42)
}
pub fn call43() -> u32 {
    scaled_fold::<85, 0, Warm, C43>(43)
}
pub fn call45() -> u32 {
    scaled_fold::<29, 0, Hot, C45>(45)
}
pub fn call46() -> u32 {
    scaled_fold::<31, 0, Warm, C46>(46)
}
pub fn call47() -> u32 {
    scaled_fold::<33, 0, Cold, C47>(47)
}
pub fn call48() -> u32 {
    scaled_fold::<39, 0, Hot, C48>(48)
}
pub fn call49() -> u32 {
    scaled_fold::<44, 0, Warm, C49>(49)
}
