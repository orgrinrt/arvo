#![no_std]
#![feature(adt_const_params)]
use core::marker::PhantomData;
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub trait Strategy {}
impl Strategy for Hot {}
impl Strategy for Warm {}
impl Strategy for Cold {}
pub struct Slot<const K: usize>;
pub trait Capacity {
    const VAL: usize;
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    fn build<T: Copy>(v: T) -> Self::Array<T>;
}
impl<const K: usize> Capacity for Slot<K> {
    const VAL: usize = K;
    type Array<T> = [T; K];
    fn build<T: Copy>(v: T) -> [T; K] {
        [v; K]
    }
}

pub struct UFixed<const I: u16, const F: u16, S>(PhantomData<S>);
pub trait Stored {
    const W: usize;
}
impl<const I: u16, const F: u16, S: Strategy> Stored for UFixed<I, F, S> {
    const W: usize = (I as usize) + (F as usize);
}
pub trait IsZeroW<const F: u16> {}
pub struct FracFlag<const F: u16>;
impl IsZeroW<0> for FracFlag<0> {}
pub trait NonZeroW<const F: u16> {}

// Obligation 2, staged: the predicate is decided at expansion time and carried
// as a sealed type-level witness. One impl, no table. The agreement between the
// witness and the widths is checked at the one door, so a hand-written lie does
// not survive the build (see y_attack.rs).
mod wseal {
    pub trait Sealed {}
}
pub struct OneYes;
pub struct OneNo;
impl wseal::Sealed for OneYes {}
impl wseal::Sealed for OneNo {}
pub trait OneWitness: wseal::Sealed {
    const YES: bool;
}
impl OneWitness for OneYes {
    const YES: bool = true;
}
impl OneWitness for OneNo {
    const YES: bool = false;
}
pub struct Num<const I: u16, const F: u16, W, S>(PhantomData<(W, S)>);
pub trait HasOne {
    fn witness();
}
impl<const I: u16, const F: u16, S: Strategy> HasOne for Num<I, F, OneYes, S> {
    fn witness() {
        const { assert!(I > 0, "one-witness disagrees with the widths") };
    }
}

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
pub type N50 = Num<55, 0, OneYes, Cold>;
pub const W50: usize = 55;
pub fn one_ok_50() {
    <N50 as HasOne>::witness()
}
pub const F50_IS_ZERO: bool = true;
pub type N51 = Num<56, 0, OneYes, Hot>;
pub const W51: usize = 56;
pub fn one_ok_51() {
    <N51 as HasOne>::witness()
}
pub const F51_IS_ZERO: bool = true;
pub type N52 = Num<92, 0, OneYes, Warm>;
pub const W52: usize = 92;
pub fn one_ok_52() {
    <N52 as HasOne>::witness()
}
pub const F52_IS_ZERO: bool = true;
pub type N53 = Num<0, 20, OneNo, Cold>;
pub const W53: usize = 20;
pub const F53_IS_ZERO: bool = false;
pub type N54 = Num<36, 0, OneYes, Hot>;
pub const W54: usize = 36;
pub fn one_ok_54() {
    <N54 as HasOne>::witness()
}
pub const F54_IS_ZERO: bool = true;
pub type N55 = Num<38, 0, OneYes, Warm>;
pub const W55: usize = 38;
pub fn one_ok_55() {
    <N55 as HasOne>::witness()
}
pub const F55_IS_ZERO: bool = true;
pub type N56 = Num<40, 0, OneYes, Cold>;
pub const W56: usize = 40;
pub fn one_ok_56() {
    <N56 as HasOne>::witness()
}
pub const F56_IS_ZERO: bool = true;
pub type N57 = Num<46, 0, OneYes, Hot>;
pub const W57: usize = 46;
pub fn one_ok_57() {
    <N57 as HasOne>::witness()
}
pub const F57_IS_ZERO: bool = true;
pub type N58 = Num<51, 0, OneYes, Warm>;
pub const W58: usize = 51;
pub fn one_ok_58() {
    <N58 as HasOne>::witness()
}
pub const F58_IS_ZERO: bool = true;
pub type N59 = Num<62, 0, OneYes, Cold>;
pub const W59: usize = 62;
pub fn one_ok_59() {
    <N59 as HasOne>::witness()
}
pub const F59_IS_ZERO: bool = true;
pub type N60 = Num<63, 0, OneYes, Hot>;
pub const W60: usize = 63;
pub fn one_ok_60() {
    <N60 as HasOne>::witness()
}
pub const F60_IS_ZERO: bool = true;
pub type N61 = Num<99, 0, OneYes, Warm>;
pub const W61: usize = 99;
pub fn one_ok_61() {
    <N61 as HasOne>::witness()
}
pub const F61_IS_ZERO: bool = true;
pub type N62 = Num<0, 21, OneNo, Cold>;
pub const W62: usize = 21;
pub const F62_IS_ZERO: bool = false;
pub type N63 = Num<43, 0, OneYes, Hot>;
pub const W63: usize = 43;
pub fn one_ok_63() {
    <N63 as HasOne>::witness()
}
pub const F63_IS_ZERO: bool = true;
pub type N64 = Num<45, 0, OneYes, Warm>;
pub const W64: usize = 45;
pub fn one_ok_64() {
    <N64 as HasOne>::witness()
}
pub const F64_IS_ZERO: bool = true;
pub type N65 = Num<47, 0, OneYes, Cold>;
pub const W65: usize = 47;
pub fn one_ok_65() {
    <N65 as HasOne>::witness()
}
pub const F65_IS_ZERO: bool = true;
pub type N66 = Num<53, 0, OneYes, Hot>;
pub const W66: usize = 53;
pub fn one_ok_66() {
    <N66 as HasOne>::witness()
}
pub const F66_IS_ZERO: bool = true;
pub type N67 = Num<58, 0, OneYes, Warm>;
pub const W67: usize = 58;
pub fn one_ok_67() {
    <N67 as HasOne>::witness()
}
pub const F67_IS_ZERO: bool = true;
pub type N68 = Num<69, 0, OneYes, Cold>;
pub const W68: usize = 69;
pub fn one_ok_68() {
    <N68 as HasOne>::witness()
}
pub const F68_IS_ZERO: bool = true;
pub type N69 = Num<70, 0, OneYes, Hot>;
pub const W69: usize = 70;
pub fn one_ok_69() {
    <N69 as HasOne>::witness()
}
pub const F69_IS_ZERO: bool = true;
pub type N70 = Num<106, 0, OneYes, Warm>;
pub const W70: usize = 106;
pub fn one_ok_70() {
    <N70 as HasOne>::witness()
}
pub const F70_IS_ZERO: bool = true;
pub type N71 = Num<0, 22, OneNo, Cold>;
pub const W71: usize = 22;
pub const F71_IS_ZERO: bool = false;
pub type N72 = Num<50, 0, OneYes, Hot>;
pub const W72: usize = 50;
pub fn one_ok_72() {
    <N72 as HasOne>::witness()
}
pub const F72_IS_ZERO: bool = true;
pub type N73 = Num<52, 0, OneYes, Warm>;
pub const W73: usize = 52;
pub fn one_ok_73() {
    <N73 as HasOne>::witness()
}
pub const F73_IS_ZERO: bool = true;
pub type N74 = Num<54, 0, OneYes, Cold>;
pub const W74: usize = 54;
pub fn one_ok_74() {
    <N74 as HasOne>::witness()
}
pub const F74_IS_ZERO: bool = true;
pub type N75 = Num<60, 0, OneYes, Hot>;
pub const W75: usize = 60;
pub fn one_ok_75() {
    <N75 as HasOne>::witness()
}
pub const F75_IS_ZERO: bool = true;
pub type N76 = Num<65, 0, OneYes, Warm>;
pub const W76: usize = 65;
pub fn one_ok_76() {
    <N76 as HasOne>::witness()
}
pub const F76_IS_ZERO: bool = true;
pub type N77 = Num<76, 0, OneYes, Cold>;
pub const W77: usize = 76;
pub fn one_ok_77() {
    <N77 as HasOne>::witness()
}
pub const F77_IS_ZERO: bool = true;
pub type N78 = Num<77, 0, OneYes, Hot>;
pub const W78: usize = 77;
pub fn one_ok_78() {
    <N78 as HasOne>::witness()
}
pub const F78_IS_ZERO: bool = true;
pub type N79 = Num<113, 0, OneYes, Warm>;
pub const W79: usize = 113;
pub fn one_ok_79() {
    <N79 as HasOne>::witness()
}
pub const F79_IS_ZERO: bool = true;
pub type N80 = Num<0, 23, OneNo, Cold>;
pub const W80: usize = 23;
pub const F80_IS_ZERO: bool = false;
pub type N81 = Num<57, 0, OneYes, Hot>;
pub const W81: usize = 57;
pub fn one_ok_81() {
    <N81 as HasOne>::witness()
}
pub const F81_IS_ZERO: bool = true;
pub type N82 = Num<59, 0, OneYes, Warm>;
pub const W82: usize = 59;
pub fn one_ok_82() {
    <N82 as HasOne>::witness()
}
pub const F82_IS_ZERO: bool = true;
pub type N83 = Num<61, 0, OneYes, Cold>;
pub const W83: usize = 61;
pub fn one_ok_83() {
    <N83 as HasOne>::witness()
}
pub const F83_IS_ZERO: bool = true;
pub type N84 = Num<67, 0, OneYes, Hot>;
pub const W84: usize = 67;
pub fn one_ok_84() {
    <N84 as HasOne>::witness()
}
pub const F84_IS_ZERO: bool = true;
pub type N85 = Num<72, 0, OneYes, Warm>;
pub const W85: usize = 72;
pub fn one_ok_85() {
    <N85 as HasOne>::witness()
}
pub const F85_IS_ZERO: bool = true;
pub type N86 = Num<83, 0, OneYes, Cold>;
pub const W86: usize = 83;
pub fn one_ok_86() {
    <N86 as HasOne>::witness()
}
pub const F86_IS_ZERO: bool = true;
pub type N87 = Num<84, 0, OneYes, Hot>;
pub const W87: usize = 84;
pub fn one_ok_87() {
    <N87 as HasOne>::witness()
}
pub const F87_IS_ZERO: bool = true;
pub type N88 = Num<120, 0, OneYes, Warm>;
pub const W88: usize = 120;
pub fn one_ok_88() {
    <N88 as HasOne>::witness()
}
pub const F88_IS_ZERO: bool = true;
pub type N89 = Num<0, 24, OneNo, Cold>;
pub const W89: usize = 24;
pub const F89_IS_ZERO: bool = false;
pub type N90 = Num<66, 0, OneYes, Hot>;
pub const W90: usize = 66;
pub fn one_ok_90() {
    <N90 as HasOne>::witness()
}
pub const F90_IS_ZERO: bool = true;
pub type N91 = Num<68, 0, OneYes, Warm>;
pub const W91: usize = 68;
pub fn one_ok_91() {
    <N91 as HasOne>::witness()
}
pub const F91_IS_ZERO: bool = true;
pub type N92 = Num<74, 0, OneYes, Cold>;
pub const W92: usize = 74;
pub fn one_ok_92() {
    <N92 as HasOne>::witness()
}
pub const F92_IS_ZERO: bool = true;
pub type N93 = Num<79, 0, OneYes, Hot>;
pub const W93: usize = 79;
pub fn one_ok_93() {
    <N93 as HasOne>::witness()
}
pub const F93_IS_ZERO: bool = true;
pub type N94 = Num<90, 0, OneYes, Warm>;
pub const W94: usize = 90;
pub fn one_ok_94() {
    <N94 as HasOne>::witness()
}
pub const F94_IS_ZERO: bool = true;
pub type N95 = Num<91, 0, OneYes, Cold>;
pub const W95: usize = 91;
pub fn one_ok_95() {
    <N95 as HasOne>::witness()
}
pub const F95_IS_ZERO: bool = true;
pub type N96 = Num<127, 0, OneYes, Hot>;
pub const W96: usize = 127;
pub fn one_ok_96() {
    <N96 as HasOne>::witness()
}
pub const F96_IS_ZERO: bool = true;
pub type N97 = Num<0, 25, OneNo, Warm>;
pub const W97: usize = 25;
pub const F97_IS_ZERO: bool = false;
pub type N98 = Num<73, 0, OneYes, Cold>;
pub const W98: usize = 73;
pub fn one_ok_98() {
    <N98 as HasOne>::witness()
}
pub const F98_IS_ZERO: bool = true;
pub type N99 = Num<75, 0, OneYes, Hot>;
pub const W99: usize = 75;
pub fn one_ok_99() {
    <N99 as HasOne>::witness()
}
pub const F99_IS_ZERO: bool = true;
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
pub const SW47: usize = 88;
pub const SW48: usize = 95;
pub const SW49: usize = 136;
pub const SW50: usize = 75;
pub const SW51: usize = 92;
pub const SW52: usize = 130;
pub const SW53: usize = 60;
pub const SW54: usize = 82;
pub const SW55: usize = 89;
pub const SW56: usize = 102;
pub const SW57: usize = 109;
pub const SW58: usize = 150;
pub const SW59: usize = 83;
pub const SW60: usize = 106;
pub const SW61: usize = 144;
pub const SW62: usize = 68;
pub const SW63: usize = 96;
pub const SW64: usize = 103;
pub const SW65: usize = 116;
pub const SW66: usize = 123;
pub const SW67: usize = 164;
pub const SW68: usize = 91;
pub const SW69: usize = 120;
pub const SW70: usize = 158;
pub const SW71: usize = 76;
pub const SW72: usize = 110;
pub const SW73: usize = 117;
pub const SW74: usize = 130;
pub const SW75: usize = 137;
pub const SW76: usize = 178;
pub const SW77: usize = 99;
pub const SW78: usize = 134;
pub const SW79: usize = 172;
pub const SW80: usize = 84;
pub const SW81: usize = 124;
pub const SW82: usize = 131;
pub const SW83: usize = 144;
pub const SW84: usize = 151;
pub const SW85: usize = 192;
pub const SW86: usize = 107;
pub const SW87: usize = 150;
pub const SW88: usize = 188;
pub const SW89: usize = 98;
pub const SW90: usize = 145;
pub const SW91: usize = 158;
pub const SW92: usize = 165;
pub const SW93: usize = 206;
pub const SW94: usize = 115;
pub const SW95: usize = 164;
pub const SW96: usize = 202;
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
pub type C50 = Slot<56>;
pub fn build50() -> <C50 as Capacity>::Array<u32> {
    C50::build(0)
}
pub type C51 = Slot<58>;
pub fn build51() -> <C51 as Capacity>::Array<u32> {
    C51::build(0)
}
pub type C52 = Slot<59>;
pub fn build52() -> <C52 as Capacity>::Array<u32> {
    C52::build(0)
}
pub type C53 = Slot<62>;
pub fn build53() -> <C53 as Capacity>::Array<u32> {
    C53::build(0)
}
pub type C54 = Slot<63>;
pub fn build54() -> <C54 as Capacity>::Array<u32> {
    C54::build(0)
}
pub type C55 = Slot<68>;
pub fn build55() -> <C55 as Capacity>::Array<u32> {
    C55::build(0)
}
pub type C56 = Slot<71>;
pub fn build56() -> <C56 as Capacity>::Array<u32> {
    C56::build(0)
}
pub type C57 = Slot<83>;
pub fn build57() -> <C57 as Capacity>::Array<u32> {
    C57::build(0)
}
pub type C58 = Slot<87>;
pub fn build58() -> <C58 as Capacity>::Array<u32> {
    C58::build(0)
}
pub type C59 = Slot<119>;
pub fn build59() -> <C59 as Capacity>::Array<u32> {
    C59::build(0)
}
pub type C60 = Slot<67>;
pub fn build60() -> <C60 as Capacity>::Array<u32> {
    C60::build(0)
}
pub type C61 = Slot<69>;
pub fn build61() -> <C61 as Capacity>::Array<u32> {
    C61::build(0)
}
pub type C62 = Slot<70>;
pub fn build62() -> <C62 as Capacity>::Array<u32> {
    C62::build(0)
}
pub type C63 = Slot<73>;
pub fn build63() -> <C63 as Capacity>::Array<u32> {
    C63::build(0)
}
pub type C64 = Slot<74>;
pub fn build64() -> <C64 as Capacity>::Array<u32> {
    C64::build(0)
}
pub type C65 = Slot<79>;
pub fn build65() -> <C65 as Capacity>::Array<u32> {
    C65::build(0)
}
pub type C66 = Slot<82>;
pub fn build66() -> <C66 as Capacity>::Array<u32> {
    C66::build(0)
}
pub type C67 = Slot<94>;
pub fn build67() -> <C67 as Capacity>::Array<u32> {
    C67::build(0)
}
pub type C68 = Slot<98>;
pub fn build68() -> <C68 as Capacity>::Array<u32> {
    C68::build(0)
}
pub type C69 = Slot<130>;
pub fn build69() -> <C69 as Capacity>::Array<u32> {
    C69::build(0)
}
pub type C70 = Slot<78>;
pub fn build70() -> <C70 as Capacity>::Array<u32> {
    C70::build(0)
}
pub type C71 = Slot<80>;
pub fn build71() -> <C71 as Capacity>::Array<u32> {
    C71::build(0)
}
pub type C72 = Slot<81>;
pub fn build72() -> <C72 as Capacity>::Array<u32> {
    C72::build(0)
}
pub type C73 = Slot<84>;
pub fn build73() -> <C73 as Capacity>::Array<u32> {
    C73::build(0)
}
pub type C74 = Slot<85>;
pub fn build74() -> <C74 as Capacity>::Array<u32> {
    C74::build(0)
}
pub type C75 = Slot<90>;
pub fn build75() -> <C75 as Capacity>::Array<u32> {
    C75::build(0)
}
pub type C76 = Slot<93>;
pub fn build76() -> <C76 as Capacity>::Array<u32> {
    C76::build(0)
}
pub type C77 = Slot<105>;
pub fn build77() -> <C77 as Capacity>::Array<u32> {
    C77::build(0)
}
pub type C78 = Slot<109>;
pub fn build78() -> <C78 as Capacity>::Array<u32> {
    C78::build(0)
}
pub type C79 = Slot<141>;
pub fn build79() -> <C79 as Capacity>::Array<u32> {
    C79::build(0)
}
pub type C80 = Slot<89>;
pub fn build80() -> <C80 as Capacity>::Array<u32> {
    C80::build(0)
}
pub type C81 = Slot<91>;
pub fn build81() -> <C81 as Capacity>::Array<u32> {
    C81::build(0)
}
pub type C82 = Slot<92>;
pub fn build82() -> <C82 as Capacity>::Array<u32> {
    C82::build(0)
}
pub type C83 = Slot<95>;
pub fn build83() -> <C83 as Capacity>::Array<u32> {
    C83::build(0)
}
pub type C84 = Slot<96>;
pub fn build84() -> <C84 as Capacity>::Array<u32> {
    C84::build(0)
}
pub type C85 = Slot<101>;
pub fn build85() -> <C85 as Capacity>::Array<u32> {
    C85::build(0)
}
pub type C86 = Slot<104>;
pub fn build86() -> <C86 as Capacity>::Array<u32> {
    C86::build(0)
}
pub type C87 = Slot<116>;
pub fn build87() -> <C87 as Capacity>::Array<u32> {
    C87::build(0)
}
pub type C88 = Slot<120>;
pub fn build88() -> <C88 as Capacity>::Array<u32> {
    C88::build(0)
}
pub type C89 = Slot<152>;
pub fn build89() -> <C89 as Capacity>::Array<u32> {
    C89::build(0)
}
pub type C90 = Slot<100>;
pub fn build90() -> <C90 as Capacity>::Array<u32> {
    C90::build(0)
}
pub type C91 = Slot<102>;
pub fn build91() -> <C91 as Capacity>::Array<u32> {
    C91::build(0)
}
pub type C92 = Slot<103>;
pub fn build92() -> <C92 as Capacity>::Array<u32> {
    C92::build(0)
}
pub type C93 = Slot<106>;
pub fn build93() -> <C93 as Capacity>::Array<u32> {
    C93::build(0)
}
pub type C94 = Slot<107>;
pub fn build94() -> <C94 as Capacity>::Array<u32> {
    C94::build(0)
}
pub type C95 = Slot<112>;
pub fn build95() -> <C95 as Capacity>::Array<u32> {
    C95::build(0)
}
pub type C96 = Slot<115>;
pub fn build96() -> <C96 as Capacity>::Array<u32> {
    C96::build(0)
}
pub type C97 = Slot<127>;
pub fn build97() -> <C97 as Capacity>::Array<u32> {
    C97::build(0)
}
pub type C98 = Slot<131>;
pub fn build98() -> <C98 as Capacity>::Array<u32> {
    C98::build(0)
}
pub type C99 = Slot<163>;
pub fn build99() -> <C99 as Capacity>::Array<u32> {
    C99::build(0)
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
pub fn call50() -> u32 {
    scaled_fold::<55, 0, Cold, C50>(50)
}
pub fn call51() -> u32 {
    scaled_fold::<56, 0, Hot, C51>(51)
}
pub fn call52() -> u32 {
    scaled_fold::<92, 0, Warm, C52>(52)
}
pub fn call54() -> u32 {
    scaled_fold::<36, 0, Hot, C54>(54)
}
pub fn call55() -> u32 {
    scaled_fold::<38, 0, Warm, C55>(55)
}
pub fn call56() -> u32 {
    scaled_fold::<40, 0, Cold, C56>(56)
}
pub fn call57() -> u32 {
    scaled_fold::<46, 0, Hot, C57>(57)
}
pub fn call58() -> u32 {
    scaled_fold::<51, 0, Warm, C58>(58)
}
pub fn call59() -> u32 {
    scaled_fold::<62, 0, Cold, C59>(59)
}
pub fn call60() -> u32 {
    scaled_fold::<63, 0, Hot, C60>(60)
}
pub fn call61() -> u32 {
    scaled_fold::<99, 0, Warm, C61>(61)
}
pub fn call63() -> u32 {
    scaled_fold::<43, 0, Hot, C63>(63)
}
pub fn call64() -> u32 {
    scaled_fold::<45, 0, Warm, C64>(64)
}
pub fn call65() -> u32 {
    scaled_fold::<47, 0, Cold, C65>(65)
}
pub fn call66() -> u32 {
    scaled_fold::<53, 0, Hot, C66>(66)
}
pub fn call67() -> u32 {
    scaled_fold::<58, 0, Warm, C67>(67)
}
pub fn call68() -> u32 {
    scaled_fold::<69, 0, Cold, C68>(68)
}
pub fn call69() -> u32 {
    scaled_fold::<70, 0, Hot, C69>(69)
}
pub fn call70() -> u32 {
    scaled_fold::<106, 0, Warm, C70>(70)
}
pub fn call72() -> u32 {
    scaled_fold::<50, 0, Hot, C72>(72)
}
pub fn call73() -> u32 {
    scaled_fold::<52, 0, Warm, C73>(73)
}
pub fn call74() -> u32 {
    scaled_fold::<54, 0, Cold, C74>(74)
}
pub fn call75() -> u32 {
    scaled_fold::<60, 0, Hot, C75>(75)
}
pub fn call76() -> u32 {
    scaled_fold::<65, 0, Warm, C76>(76)
}
pub fn call77() -> u32 {
    scaled_fold::<76, 0, Cold, C77>(77)
}
pub fn call78() -> u32 {
    scaled_fold::<77, 0, Hot, C78>(78)
}
pub fn call79() -> u32 {
    scaled_fold::<113, 0, Warm, C79>(79)
}
pub fn call81() -> u32 {
    scaled_fold::<57, 0, Hot, C81>(81)
}
pub fn call82() -> u32 {
    scaled_fold::<59, 0, Warm, C82>(82)
}
pub fn call83() -> u32 {
    scaled_fold::<61, 0, Cold, C83>(83)
}
pub fn call84() -> u32 {
    scaled_fold::<67, 0, Hot, C84>(84)
}
pub fn call85() -> u32 {
    scaled_fold::<72, 0, Warm, C85>(85)
}
pub fn call86() -> u32 {
    scaled_fold::<83, 0, Cold, C86>(86)
}
pub fn call87() -> u32 {
    scaled_fold::<84, 0, Hot, C87>(87)
}
pub fn call88() -> u32 {
    scaled_fold::<120, 0, Warm, C88>(88)
}
pub fn call90() -> u32 {
    scaled_fold::<66, 0, Hot, C90>(90)
}
pub fn call91() -> u32 {
    scaled_fold::<68, 0, Warm, C91>(91)
}
pub fn call92() -> u32 {
    scaled_fold::<74, 0, Cold, C92>(92)
}
pub fn call93() -> u32 {
    scaled_fold::<79, 0, Hot, C93>(93)
}
pub fn call94() -> u32 {
    scaled_fold::<90, 0, Warm, C94>(94)
}
pub fn call95() -> u32 {
    scaled_fold::<91, 0, Cold, C95>(95)
}
pub fn call96() -> u32 {
    scaled_fold::<127, 0, Hot, C96>(96)
}
pub fn call98() -> u32 {
    scaled_fold::<73, 0, Cold, C98>(98)
}
pub fn call99() -> u32 {
    scaled_fold::<75, 0, Hot, C99>(99)
}
