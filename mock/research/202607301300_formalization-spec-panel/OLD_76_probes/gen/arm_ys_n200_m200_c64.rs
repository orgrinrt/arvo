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
pub type N100 = Num<81, 0, OneYes, Warm>;
pub const W100: usize = 81;
pub fn one_ok_100() {
    <N100 as HasOne>::witness()
}
pub const F100_IS_ZERO: bool = true;
pub type N101 = Num<86, 0, OneYes, Cold>;
pub const W101: usize = 86;
pub fn one_ok_101() {
    <N101 as HasOne>::witness()
}
pub const F101_IS_ZERO: bool = true;
pub type N102 = Num<97, 0, OneYes, Hot>;
pub const W102: usize = 97;
pub fn one_ok_102() {
    <N102 as HasOne>::witness()
}
pub const F102_IS_ZERO: bool = true;
pub type N103 = Num<98, 0, OneYes, Warm>;
pub const W103: usize = 98;
pub fn one_ok_103() {
    <N103 as HasOne>::witness()
}
pub const F103_IS_ZERO: bool = true;
pub type N104 = Num<134, 0, OneYes, Cold>;
pub const W104: usize = 134;
pub fn one_ok_104() {
    <N104 as HasOne>::witness()
}
pub const F104_IS_ZERO: bool = true;
pub type N105 = Num<0, 26, OneNo, Hot>;
pub const W105: usize = 26;
pub const F105_IS_ZERO: bool = false;
pub type N106 = Num<80, 0, OneYes, Warm>;
pub const W106: usize = 80;
pub fn one_ok_106() {
    <N106 as HasOne>::witness()
}
pub const F106_IS_ZERO: bool = true;
pub type N107 = Num<82, 0, OneYes, Cold>;
pub const W107: usize = 82;
pub fn one_ok_107() {
    <N107 as HasOne>::witness()
}
pub const F107_IS_ZERO: bool = true;
pub type N108 = Num<88, 0, OneYes, Hot>;
pub const W108: usize = 88;
pub fn one_ok_108() {
    <N108 as HasOne>::witness()
}
pub const F108_IS_ZERO: bool = true;
pub type N109 = Num<93, 0, OneYes, Warm>;
pub const W109: usize = 93;
pub fn one_ok_109() {
    <N109 as HasOne>::witness()
}
pub const F109_IS_ZERO: bool = true;
pub type N110 = Num<104, 0, OneYes, Cold>;
pub const W110: usize = 104;
pub fn one_ok_110() {
    <N110 as HasOne>::witness()
}
pub const F110_IS_ZERO: bool = true;
pub type N111 = Num<105, 0, OneYes, Hot>;
pub const W111: usize = 105;
pub fn one_ok_111() {
    <N111 as HasOne>::witness()
}
pub const F111_IS_ZERO: bool = true;
pub type N112 = Num<141, 0, OneYes, Warm>;
pub const W112: usize = 141;
pub fn one_ok_112() {
    <N112 as HasOne>::witness()
}
pub const F112_IS_ZERO: bool = true;
pub type N113 = Num<0, 27, OneNo, Cold>;
pub const W113: usize = 27;
pub const F113_IS_ZERO: bool = false;
pub type N114 = Num<87, 0, OneYes, Hot>;
pub const W114: usize = 87;
pub fn one_ok_114() {
    <N114 as HasOne>::witness()
}
pub const F114_IS_ZERO: bool = true;
pub type N115 = Num<89, 0, OneYes, Warm>;
pub const W115: usize = 89;
pub fn one_ok_115() {
    <N115 as HasOne>::witness()
}
pub const F115_IS_ZERO: bool = true;
pub type N116 = Num<95, 0, OneYes, Cold>;
pub const W116: usize = 95;
pub fn one_ok_116() {
    <N116 as HasOne>::witness()
}
pub const F116_IS_ZERO: bool = true;
pub type N117 = Num<100, 0, OneYes, Hot>;
pub const W117: usize = 100;
pub fn one_ok_117() {
    <N117 as HasOne>::witness()
}
pub const F117_IS_ZERO: bool = true;
pub type N118 = Num<111, 0, OneYes, Warm>;
pub const W118: usize = 111;
pub fn one_ok_118() {
    <N118 as HasOne>::witness()
}
pub const F118_IS_ZERO: bool = true;
pub type N119 = Num<112, 0, OneYes, Cold>;
pub const W119: usize = 112;
pub fn one_ok_119() {
    <N119 as HasOne>::witness()
}
pub const F119_IS_ZERO: bool = true;
pub type N120 = Num<148, 0, OneYes, Hot>;
pub const W120: usize = 148;
pub fn one_ok_120() {
    <N120 as HasOne>::witness()
}
pub const F120_IS_ZERO: bool = true;
pub type N121 = Num<0, 28, OneNo, Warm>;
pub const W121: usize = 28;
pub const F121_IS_ZERO: bool = false;
pub type N122 = Num<94, 0, OneYes, Cold>;
pub const W122: usize = 94;
pub fn one_ok_122() {
    <N122 as HasOne>::witness()
}
pub const F122_IS_ZERO: bool = true;
pub type N123 = Num<96, 0, OneYes, Hot>;
pub const W123: usize = 96;
pub fn one_ok_123() {
    <N123 as HasOne>::witness()
}
pub const F123_IS_ZERO: bool = true;
pub type N124 = Num<102, 0, OneYes, Warm>;
pub const W124: usize = 102;
pub fn one_ok_124() {
    <N124 as HasOne>::witness()
}
pub const F124_IS_ZERO: bool = true;
pub type N125 = Num<107, 0, OneYes, Cold>;
pub const W125: usize = 107;
pub fn one_ok_125() {
    <N125 as HasOne>::witness()
}
pub const F125_IS_ZERO: bool = true;
pub type N126 = Num<118, 0, OneYes, Hot>;
pub const W126: usize = 118;
pub fn one_ok_126() {
    <N126 as HasOne>::witness()
}
pub const F126_IS_ZERO: bool = true;
pub type N127 = Num<119, 0, OneYes, Warm>;
pub const W127: usize = 119;
pub fn one_ok_127() {
    <N127 as HasOne>::witness()
}
pub const F127_IS_ZERO: bool = true;
pub type N128 = Num<155, 0, OneYes, Cold>;
pub const W128: usize = 155;
pub fn one_ok_128() {
    <N128 as HasOne>::witness()
}
pub const F128_IS_ZERO: bool = true;
pub type N129 = Num<0, 29, OneNo, Hot>;
pub const W129: usize = 29;
pub const F129_IS_ZERO: bool = false;
pub type N130 = Num<101, 0, OneYes, Warm>;
pub const W130: usize = 101;
pub fn one_ok_130() {
    <N130 as HasOne>::witness()
}
pub const F130_IS_ZERO: bool = true;
pub type N131 = Num<103, 0, OneYes, Cold>;
pub const W131: usize = 103;
pub fn one_ok_131() {
    <N131 as HasOne>::witness()
}
pub const F131_IS_ZERO: bool = true;
pub type N132 = Num<109, 0, OneYes, Hot>;
pub const W132: usize = 109;
pub fn one_ok_132() {
    <N132 as HasOne>::witness()
}
pub const F132_IS_ZERO: bool = true;
pub type N133 = Num<114, 0, OneYes, Warm>;
pub const W133: usize = 114;
pub fn one_ok_133() {
    <N133 as HasOne>::witness()
}
pub const F133_IS_ZERO: bool = true;
pub type N134 = Num<125, 0, OneYes, Cold>;
pub const W134: usize = 125;
pub fn one_ok_134() {
    <N134 as HasOne>::witness()
}
pub const F134_IS_ZERO: bool = true;
pub type N135 = Num<126, 0, OneYes, Hot>;
pub const W135: usize = 126;
pub fn one_ok_135() {
    <N135 as HasOne>::witness()
}
pub const F135_IS_ZERO: bool = true;
pub type N136 = Num<162, 0, OneYes, Warm>;
pub const W136: usize = 162;
pub fn one_ok_136() {
    <N136 as HasOne>::witness()
}
pub const F136_IS_ZERO: bool = true;
pub type N137 = Num<0, 30, OneNo, Cold>;
pub const W137: usize = 30;
pub const F137_IS_ZERO: bool = false;
pub type N138 = Num<108, 0, OneYes, Hot>;
pub const W138: usize = 108;
pub fn one_ok_138() {
    <N138 as HasOne>::witness()
}
pub const F138_IS_ZERO: bool = true;
pub type N139 = Num<110, 0, OneYes, Warm>;
pub const W139: usize = 110;
pub fn one_ok_139() {
    <N139 as HasOne>::witness()
}
pub const F139_IS_ZERO: bool = true;
pub type N140 = Num<116, 0, OneYes, Cold>;
pub const W140: usize = 116;
pub fn one_ok_140() {
    <N140 as HasOne>::witness()
}
pub const F140_IS_ZERO: bool = true;
pub type N141 = Num<121, 0, OneYes, Hot>;
pub const W141: usize = 121;
pub fn one_ok_141() {
    <N141 as HasOne>::witness()
}
pub const F141_IS_ZERO: bool = true;
pub type N142 = Num<132, 0, OneYes, Warm>;
pub const W142: usize = 132;
pub fn one_ok_142() {
    <N142 as HasOne>::witness()
}
pub const F142_IS_ZERO: bool = true;
pub type N143 = Num<133, 0, OneYes, Cold>;
pub const W143: usize = 133;
pub fn one_ok_143() {
    <N143 as HasOne>::witness()
}
pub const F143_IS_ZERO: bool = true;
pub type N144 = Num<169, 0, OneYes, Hot>;
pub const W144: usize = 169;
pub fn one_ok_144() {
    <N144 as HasOne>::witness()
}
pub const F144_IS_ZERO: bool = true;
pub type N145 = Num<0, 31, OneNo, Warm>;
pub const W145: usize = 31;
pub const F145_IS_ZERO: bool = false;
pub type N146 = Num<115, 0, OneYes, Cold>;
pub const W146: usize = 115;
pub fn one_ok_146() {
    <N146 as HasOne>::witness()
}
pub const F146_IS_ZERO: bool = true;
pub type N147 = Num<117, 0, OneYes, Hot>;
pub const W147: usize = 117;
pub fn one_ok_147() {
    <N147 as HasOne>::witness()
}
pub const F147_IS_ZERO: bool = true;
pub type N148 = Num<123, 0, OneYes, Warm>;
pub const W148: usize = 123;
pub fn one_ok_148() {
    <N148 as HasOne>::witness()
}
pub const F148_IS_ZERO: bool = true;
pub type N149 = Num<128, 0, OneYes, Cold>;
pub const W149: usize = 128;
pub fn one_ok_149() {
    <N149 as HasOne>::witness()
}
pub const F149_IS_ZERO: bool = true;
pub type N150 = Num<139, 0, OneYes, Hot>;
pub const W150: usize = 139;
pub fn one_ok_150() {
    <N150 as HasOne>::witness()
}
pub const F150_IS_ZERO: bool = true;
pub type N151 = Num<140, 0, OneYes, Warm>;
pub const W151: usize = 140;
pub fn one_ok_151() {
    <N151 as HasOne>::witness()
}
pub const F151_IS_ZERO: bool = true;
pub type N152 = Num<176, 0, OneYes, Cold>;
pub const W152: usize = 176;
pub fn one_ok_152() {
    <N152 as HasOne>::witness()
}
pub const F152_IS_ZERO: bool = true;
pub type N153 = Num<0, 32, OneNo, Hot>;
pub const W153: usize = 32;
pub const F153_IS_ZERO: bool = false;
pub type N154 = Num<122, 0, OneYes, Warm>;
pub const W154: usize = 122;
pub fn one_ok_154() {
    <N154 as HasOne>::witness()
}
pub const F154_IS_ZERO: bool = true;
pub type N155 = Num<124, 0, OneYes, Cold>;
pub const W155: usize = 124;
pub fn one_ok_155() {
    <N155 as HasOne>::witness()
}
pub const F155_IS_ZERO: bool = true;
pub type N156 = Num<130, 0, OneYes, Hot>;
pub const W156: usize = 130;
pub fn one_ok_156() {
    <N156 as HasOne>::witness()
}
pub const F156_IS_ZERO: bool = true;
pub type N157 = Num<135, 0, OneYes, Warm>;
pub const W157: usize = 135;
pub fn one_ok_157() {
    <N157 as HasOne>::witness()
}
pub const F157_IS_ZERO: bool = true;
pub type N158 = Num<146, 0, OneYes, Cold>;
pub const W158: usize = 146;
pub fn one_ok_158() {
    <N158 as HasOne>::witness()
}
pub const F158_IS_ZERO: bool = true;
pub type N159 = Num<147, 0, OneYes, Hot>;
pub const W159: usize = 147;
pub fn one_ok_159() {
    <N159 as HasOne>::witness()
}
pub const F159_IS_ZERO: bool = true;
pub type N160 = Num<183, 0, OneYes, Warm>;
pub const W160: usize = 183;
pub fn one_ok_160() {
    <N160 as HasOne>::witness()
}
pub const F160_IS_ZERO: bool = true;
pub type N161 = Num<0, 33, OneNo, Cold>;
pub const W161: usize = 33;
pub const F161_IS_ZERO: bool = false;
pub type N162 = Num<129, 0, OneYes, Hot>;
pub const W162: usize = 129;
pub fn one_ok_162() {
    <N162 as HasOne>::witness()
}
pub const F162_IS_ZERO: bool = true;
pub type N163 = Num<131, 0, OneYes, Warm>;
pub const W163: usize = 131;
pub fn one_ok_163() {
    <N163 as HasOne>::witness()
}
pub const F163_IS_ZERO: bool = true;
pub type N164 = Num<137, 0, OneYes, Cold>;
pub const W164: usize = 137;
pub fn one_ok_164() {
    <N164 as HasOne>::witness()
}
pub const F164_IS_ZERO: bool = true;
pub type N165 = Num<142, 0, OneYes, Hot>;
pub const W165: usize = 142;
pub fn one_ok_165() {
    <N165 as HasOne>::witness()
}
pub const F165_IS_ZERO: bool = true;
pub type N166 = Num<153, 0, OneYes, Warm>;
pub const W166: usize = 153;
pub fn one_ok_166() {
    <N166 as HasOne>::witness()
}
pub const F166_IS_ZERO: bool = true;
pub type N167 = Num<154, 0, OneYes, Cold>;
pub const W167: usize = 154;
pub fn one_ok_167() {
    <N167 as HasOne>::witness()
}
pub const F167_IS_ZERO: bool = true;
pub type N168 = Num<190, 0, OneYes, Hot>;
pub const W168: usize = 190;
pub fn one_ok_168() {
    <N168 as HasOne>::witness()
}
pub const F168_IS_ZERO: bool = true;
pub type N169 = Num<0, 34, OneNo, Warm>;
pub const W169: usize = 34;
pub const F169_IS_ZERO: bool = false;
pub type N170 = Num<136, 0, OneYes, Cold>;
pub const W170: usize = 136;
pub fn one_ok_170() {
    <N170 as HasOne>::witness()
}
pub const F170_IS_ZERO: bool = true;
pub type N171 = Num<138, 0, OneYes, Hot>;
pub const W171: usize = 138;
pub fn one_ok_171() {
    <N171 as HasOne>::witness()
}
pub const F171_IS_ZERO: bool = true;
pub type N172 = Num<144, 0, OneYes, Warm>;
pub const W172: usize = 144;
pub fn one_ok_172() {
    <N172 as HasOne>::witness()
}
pub const F172_IS_ZERO: bool = true;
pub type N173 = Num<149, 0, OneYes, Cold>;
pub const W173: usize = 149;
pub fn one_ok_173() {
    <N173 as HasOne>::witness()
}
pub const F173_IS_ZERO: bool = true;
pub type N174 = Num<160, 0, OneYes, Hot>;
pub const W174: usize = 160;
pub fn one_ok_174() {
    <N174 as HasOne>::witness()
}
pub const F174_IS_ZERO: bool = true;
pub type N175 = Num<161, 0, OneYes, Warm>;
pub const W175: usize = 161;
pub fn one_ok_175() {
    <N175 as HasOne>::witness()
}
pub const F175_IS_ZERO: bool = true;
pub type N176 = Num<197, 0, OneYes, Cold>;
pub const W176: usize = 197;
pub fn one_ok_176() {
    <N176 as HasOne>::witness()
}
pub const F176_IS_ZERO: bool = true;
pub type N177 = Num<0, 35, OneNo, Hot>;
pub const W177: usize = 35;
pub const F177_IS_ZERO: bool = false;
pub type N178 = Num<143, 0, OneYes, Warm>;
pub const W178: usize = 143;
pub fn one_ok_178() {
    <N178 as HasOne>::witness()
}
pub const F178_IS_ZERO: bool = true;
pub type N179 = Num<145, 0, OneYes, Cold>;
pub const W179: usize = 145;
pub fn one_ok_179() {
    <N179 as HasOne>::witness()
}
pub const F179_IS_ZERO: bool = true;
pub type N180 = Num<151, 0, OneYes, Hot>;
pub const W180: usize = 151;
pub fn one_ok_180() {
    <N180 as HasOne>::witness()
}
pub const F180_IS_ZERO: bool = true;
pub type N181 = Num<156, 0, OneYes, Warm>;
pub const W181: usize = 156;
pub fn one_ok_181() {
    <N181 as HasOne>::witness()
}
pub const F181_IS_ZERO: bool = true;
pub type N182 = Num<167, 0, OneYes, Cold>;
pub const W182: usize = 167;
pub fn one_ok_182() {
    <N182 as HasOne>::witness()
}
pub const F182_IS_ZERO: bool = true;
pub type N183 = Num<168, 0, OneYes, Hot>;
pub const W183: usize = 168;
pub fn one_ok_183() {
    <N183 as HasOne>::witness()
}
pub const F183_IS_ZERO: bool = true;
pub type N184 = Num<204, 0, OneYes, Warm>;
pub const W184: usize = 204;
pub fn one_ok_184() {
    <N184 as HasOne>::witness()
}
pub const F184_IS_ZERO: bool = true;
pub type N185 = Num<0, 36, OneNo, Cold>;
pub const W185: usize = 36;
pub const F185_IS_ZERO: bool = false;
pub type N186 = Num<150, 0, OneYes, Hot>;
pub const W186: usize = 150;
pub fn one_ok_186() {
    <N186 as HasOne>::witness()
}
pub const F186_IS_ZERO: bool = true;
pub type N187 = Num<152, 0, OneYes, Warm>;
pub const W187: usize = 152;
pub fn one_ok_187() {
    <N187 as HasOne>::witness()
}
pub const F187_IS_ZERO: bool = true;
pub type N188 = Num<158, 0, OneYes, Cold>;
pub const W188: usize = 158;
pub fn one_ok_188() {
    <N188 as HasOne>::witness()
}
pub const F188_IS_ZERO: bool = true;
pub type N189 = Num<163, 0, OneYes, Hot>;
pub const W189: usize = 163;
pub fn one_ok_189() {
    <N189 as HasOne>::witness()
}
pub const F189_IS_ZERO: bool = true;
pub type N190 = Num<174, 0, OneYes, Warm>;
pub const W190: usize = 174;
pub fn one_ok_190() {
    <N190 as HasOne>::witness()
}
pub const F190_IS_ZERO: bool = true;
pub type N191 = Num<175, 0, OneYes, Cold>;
pub const W191: usize = 175;
pub fn one_ok_191() {
    <N191 as HasOne>::witness()
}
pub const F191_IS_ZERO: bool = true;
pub type N192 = Num<211, 0, OneYes, Hot>;
pub const W192: usize = 211;
pub fn one_ok_192() {
    <N192 as HasOne>::witness()
}
pub const F192_IS_ZERO: bool = true;
pub type N193 = Num<0, 37, OneNo, Warm>;
pub const W193: usize = 37;
pub const F193_IS_ZERO: bool = false;
pub type N194 = Num<157, 0, OneYes, Cold>;
pub const W194: usize = 157;
pub fn one_ok_194() {
    <N194 as HasOne>::witness()
}
pub const F194_IS_ZERO: bool = true;
pub type N195 = Num<159, 0, OneYes, Hot>;
pub const W195: usize = 159;
pub fn one_ok_195() {
    <N195 as HasOne>::witness()
}
pub const F195_IS_ZERO: bool = true;
pub type N196 = Num<165, 0, OneYes, Warm>;
pub const W196: usize = 165;
pub fn one_ok_196() {
    <N196 as HasOne>::witness()
}
pub const F196_IS_ZERO: bool = true;
pub type N197 = Num<170, 0, OneYes, Cold>;
pub const W197: usize = 170;
pub fn one_ok_197() {
    <N197 as HasOne>::witness()
}
pub const F197_IS_ZERO: bool = true;
pub type N198 = Num<181, 0, OneYes, Hot>;
pub const W198: usize = 181;
pub fn one_ok_198() {
    <N198 as HasOne>::witness()
}
pub const F198_IS_ZERO: bool = true;
pub type N199 = Num<182, 0, OneYes, Warm>;
pub const W199: usize = 182;
pub fn one_ok_199() {
    <N199 as HasOne>::witness()
}
pub const F199_IS_ZERO: bool = true;
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
pub const SW97: usize = 106;
pub const SW98: usize = 159;
pub const SW99: usize = 172;
pub const SW100: usize = 179;
pub const SW101: usize = 220;
pub const SW102: usize = 123;
pub const SW103: usize = 178;
pub const SW104: usize = 216;
pub const SW105: usize = 114;
pub const SW106: usize = 173;
pub const SW107: usize = 186;
pub const SW108: usize = 193;
pub const SW109: usize = 234;
pub const SW110: usize = 131;
pub const SW111: usize = 192;
pub const SW112: usize = 230;
pub const SW113: usize = 122;
pub const SW114: usize = 187;
pub const SW115: usize = 200;
pub const SW116: usize = 207;
pub const SW117: usize = 248;
pub const SW118: usize = 139;
pub const SW119: usize = 206;
pub const SW120: usize = 244;
pub const SW121: usize = 130;
pub const SW122: usize = 201;
pub const SW123: usize = 214;
pub const SW124: usize = 221;
pub const SW125: usize = 262;
pub const SW126: usize = 147;
pub const SW127: usize = 220;
pub const SW128: usize = 258;
pub const SW129: usize = 138;
pub const SW130: usize = 215;
pub const SW131: usize = 228;
pub const SW132: usize = 235;
pub const SW133: usize = 276;
pub const SW134: usize = 155;
pub const SW135: usize = 234;
pub const SW136: usize = 272;
pub const SW137: usize = 146;
pub const SW138: usize = 229;
pub const SW139: usize = 242;
pub const SW140: usize = 249;
pub const SW141: usize = 290;
pub const SW142: usize = 163;
pub const SW143: usize = 248;
pub const SW144: usize = 286;
pub const SW145: usize = 154;
pub const SW146: usize = 243;
pub const SW147: usize = 256;
pub const SW148: usize = 263;
pub const SW149: usize = 304;
pub const SW150: usize = 171;
pub const SW151: usize = 262;
pub const SW152: usize = 300;
pub const SW153: usize = 162;
pub const SW154: usize = 257;
pub const SW155: usize = 270;
pub const SW156: usize = 277;
pub const SW157: usize = 318;
pub const SW158: usize = 179;
pub const SW159: usize = 276;
pub const SW160: usize = 314;
pub const SW161: usize = 170;
pub const SW162: usize = 271;
pub const SW163: usize = 284;
pub const SW164: usize = 291;
pub const SW165: usize = 332;
pub const SW166: usize = 187;
pub const SW167: usize = 290;
pub const SW168: usize = 328;
pub const SW169: usize = 178;
pub const SW170: usize = 285;
pub const SW171: usize = 298;
pub const SW172: usize = 305;
pub const SW173: usize = 346;
pub const SW174: usize = 195;
pub const SW175: usize = 304;
pub const SW176: usize = 342;
pub const SW177: usize = 186;
pub const SW178: usize = 299;
pub const SW179: usize = 312;
pub const SW180: usize = 319;
pub const SW181: usize = 360;
pub const SW182: usize = 203;
pub const SW183: usize = 318;
pub const SW184: usize = 356;
pub const SW185: usize = 194;
pub const SW186: usize = 313;
pub const SW187: usize = 326;
pub const SW188: usize = 333;
pub const SW189: usize = 374;
pub const SW190: usize = 211;
pub const SW191: usize = 332;
pub const SW192: usize = 370;
pub const SW193: usize = 202;
pub const SW194: usize = 327;
pub const SW195: usize = 340;
pub const SW196: usize = 347;
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
pub type C100 = Slot<111>;
pub fn build100() -> <C100 as Capacity>::Array<u32> {
    C100::build(0)
}
pub type C101 = Slot<113>;
pub fn build101() -> <C101 as Capacity>::Array<u32> {
    C101::build(0)
}
pub type C102 = Slot<114>;
pub fn build102() -> <C102 as Capacity>::Array<u32> {
    C102::build(0)
}
pub type C103 = Slot<117>;
pub fn build103() -> <C103 as Capacity>::Array<u32> {
    C103::build(0)
}
pub type C104 = Slot<118>;
pub fn build104() -> <C104 as Capacity>::Array<u32> {
    C104::build(0)
}
pub type C105 = Slot<123>;
pub fn build105() -> <C105 as Capacity>::Array<u32> {
    C105::build(0)
}
pub type C106 = Slot<126>;
pub fn build106() -> <C106 as Capacity>::Array<u32> {
    C106::build(0)
}
pub type C107 = Slot<138>;
pub fn build107() -> <C107 as Capacity>::Array<u32> {
    C107::build(0)
}
pub type C108 = Slot<142>;
pub fn build108() -> <C108 as Capacity>::Array<u32> {
    C108::build(0)
}
pub type C109 = Slot<174>;
pub fn build109() -> <C109 as Capacity>::Array<u32> {
    C109::build(0)
}
pub type C110 = Slot<122>;
pub fn build110() -> <C110 as Capacity>::Array<u32> {
    C110::build(0)
}
pub type C111 = Slot<124>;
pub fn build111() -> <C111 as Capacity>::Array<u32> {
    C111::build(0)
}
pub type C112 = Slot<125>;
pub fn build112() -> <C112 as Capacity>::Array<u32> {
    C112::build(0)
}
pub type C113 = Slot<128>;
pub fn build113() -> <C113 as Capacity>::Array<u32> {
    C113::build(0)
}
pub type C114 = Slot<129>;
pub fn build114() -> <C114 as Capacity>::Array<u32> {
    C114::build(0)
}
pub type C115 = Slot<134>;
pub fn build115() -> <C115 as Capacity>::Array<u32> {
    C115::build(0)
}
pub type C116 = Slot<137>;
pub fn build116() -> <C116 as Capacity>::Array<u32> {
    C116::build(0)
}
pub type C117 = Slot<149>;
pub fn build117() -> <C117 as Capacity>::Array<u32> {
    C117::build(0)
}
pub type C118 = Slot<153>;
pub fn build118() -> <C118 as Capacity>::Array<u32> {
    C118::build(0)
}
pub type C119 = Slot<185>;
pub fn build119() -> <C119 as Capacity>::Array<u32> {
    C119::build(0)
}
pub type C120 = Slot<133>;
pub fn build120() -> <C120 as Capacity>::Array<u32> {
    C120::build(0)
}
pub type C121 = Slot<135>;
pub fn build121() -> <C121 as Capacity>::Array<u32> {
    C121::build(0)
}
pub type C122 = Slot<136>;
pub fn build122() -> <C122 as Capacity>::Array<u32> {
    C122::build(0)
}
pub type C123 = Slot<139>;
pub fn build123() -> <C123 as Capacity>::Array<u32> {
    C123::build(0)
}
pub type C124 = Slot<140>;
pub fn build124() -> <C124 as Capacity>::Array<u32> {
    C124::build(0)
}
pub type C125 = Slot<145>;
pub fn build125() -> <C125 as Capacity>::Array<u32> {
    C125::build(0)
}
pub type C126 = Slot<148>;
pub fn build126() -> <C126 as Capacity>::Array<u32> {
    C126::build(0)
}
pub type C127 = Slot<160>;
pub fn build127() -> <C127 as Capacity>::Array<u32> {
    C127::build(0)
}
pub type C128 = Slot<164>;
pub fn build128() -> <C128 as Capacity>::Array<u32> {
    C128::build(0)
}
pub type C129 = Slot<196>;
pub fn build129() -> <C129 as Capacity>::Array<u32> {
    C129::build(0)
}
pub type C130 = Slot<144>;
pub fn build130() -> <C130 as Capacity>::Array<u32> {
    C130::build(0)
}
pub type C131 = Slot<146>;
pub fn build131() -> <C131 as Capacity>::Array<u32> {
    C131::build(0)
}
pub type C132 = Slot<147>;
pub fn build132() -> <C132 as Capacity>::Array<u32> {
    C132::build(0)
}
pub type C133 = Slot<150>;
pub fn build133() -> <C133 as Capacity>::Array<u32> {
    C133::build(0)
}
pub type C134 = Slot<151>;
pub fn build134() -> <C134 as Capacity>::Array<u32> {
    C134::build(0)
}
pub type C135 = Slot<156>;
pub fn build135() -> <C135 as Capacity>::Array<u32> {
    C135::build(0)
}
pub type C136 = Slot<159>;
pub fn build136() -> <C136 as Capacity>::Array<u32> {
    C136::build(0)
}
pub type C137 = Slot<171>;
pub fn build137() -> <C137 as Capacity>::Array<u32> {
    C137::build(0)
}
pub type C138 = Slot<175>;
pub fn build138() -> <C138 as Capacity>::Array<u32> {
    C138::build(0)
}
pub type C139 = Slot<207>;
pub fn build139() -> <C139 as Capacity>::Array<u32> {
    C139::build(0)
}
pub type C140 = Slot<155>;
pub fn build140() -> <C140 as Capacity>::Array<u32> {
    C140::build(0)
}
pub type C141 = Slot<157>;
pub fn build141() -> <C141 as Capacity>::Array<u32> {
    C141::build(0)
}
pub type C142 = Slot<158>;
pub fn build142() -> <C142 as Capacity>::Array<u32> {
    C142::build(0)
}
pub type C143 = Slot<161>;
pub fn build143() -> <C143 as Capacity>::Array<u32> {
    C143::build(0)
}
pub type C144 = Slot<162>;
pub fn build144() -> <C144 as Capacity>::Array<u32> {
    C144::build(0)
}
pub type C145 = Slot<167>;
pub fn build145() -> <C145 as Capacity>::Array<u32> {
    C145::build(0)
}
pub type C146 = Slot<170>;
pub fn build146() -> <C146 as Capacity>::Array<u32> {
    C146::build(0)
}
pub type C147 = Slot<182>;
pub fn build147() -> <C147 as Capacity>::Array<u32> {
    C147::build(0)
}
pub type C148 = Slot<186>;
pub fn build148() -> <C148 as Capacity>::Array<u32> {
    C148::build(0)
}
pub type C149 = Slot<218>;
pub fn build149() -> <C149 as Capacity>::Array<u32> {
    C149::build(0)
}
pub type C150 = Slot<166>;
pub fn build150() -> <C150 as Capacity>::Array<u32> {
    C150::build(0)
}
pub type C151 = Slot<168>;
pub fn build151() -> <C151 as Capacity>::Array<u32> {
    C151::build(0)
}
pub type C152 = Slot<169>;
pub fn build152() -> <C152 as Capacity>::Array<u32> {
    C152::build(0)
}
pub type C153 = Slot<172>;
pub fn build153() -> <C153 as Capacity>::Array<u32> {
    C153::build(0)
}
pub type C154 = Slot<173>;
pub fn build154() -> <C154 as Capacity>::Array<u32> {
    C154::build(0)
}
pub type C155 = Slot<178>;
pub fn build155() -> <C155 as Capacity>::Array<u32> {
    C155::build(0)
}
pub type C156 = Slot<181>;
pub fn build156() -> <C156 as Capacity>::Array<u32> {
    C156::build(0)
}
pub type C157 = Slot<193>;
pub fn build157() -> <C157 as Capacity>::Array<u32> {
    C157::build(0)
}
pub type C158 = Slot<197>;
pub fn build158() -> <C158 as Capacity>::Array<u32> {
    C158::build(0)
}
pub type C159 = Slot<229>;
pub fn build159() -> <C159 as Capacity>::Array<u32> {
    C159::build(0)
}
pub type C160 = Slot<177>;
pub fn build160() -> <C160 as Capacity>::Array<u32> {
    C160::build(0)
}
pub type C161 = Slot<179>;
pub fn build161() -> <C161 as Capacity>::Array<u32> {
    C161::build(0)
}
pub type C162 = Slot<180>;
pub fn build162() -> <C162 as Capacity>::Array<u32> {
    C162::build(0)
}
pub type C163 = Slot<183>;
pub fn build163() -> <C163 as Capacity>::Array<u32> {
    C163::build(0)
}
pub type C164 = Slot<184>;
pub fn build164() -> <C164 as Capacity>::Array<u32> {
    C164::build(0)
}
pub type C165 = Slot<189>;
pub fn build165() -> <C165 as Capacity>::Array<u32> {
    C165::build(0)
}
pub type C166 = Slot<192>;
pub fn build166() -> <C166 as Capacity>::Array<u32> {
    C166::build(0)
}
pub type C167 = Slot<204>;
pub fn build167() -> <C167 as Capacity>::Array<u32> {
    C167::build(0)
}
pub type C168 = Slot<208>;
pub fn build168() -> <C168 as Capacity>::Array<u32> {
    C168::build(0)
}
pub type C169 = Slot<240>;
pub fn build169() -> <C169 as Capacity>::Array<u32> {
    C169::build(0)
}
pub type C170 = Slot<188>;
pub fn build170() -> <C170 as Capacity>::Array<u32> {
    C170::build(0)
}
pub type C171 = Slot<190>;
pub fn build171() -> <C171 as Capacity>::Array<u32> {
    C171::build(0)
}
pub type C172 = Slot<191>;
pub fn build172() -> <C172 as Capacity>::Array<u32> {
    C172::build(0)
}
pub type C173 = Slot<194>;
pub fn build173() -> <C173 as Capacity>::Array<u32> {
    C173::build(0)
}
pub type C174 = Slot<195>;
pub fn build174() -> <C174 as Capacity>::Array<u32> {
    C174::build(0)
}
pub type C175 = Slot<200>;
pub fn build175() -> <C175 as Capacity>::Array<u32> {
    C175::build(0)
}
pub type C176 = Slot<203>;
pub fn build176() -> <C176 as Capacity>::Array<u32> {
    C176::build(0)
}
pub type C177 = Slot<215>;
pub fn build177() -> <C177 as Capacity>::Array<u32> {
    C177::build(0)
}
pub type C178 = Slot<219>;
pub fn build178() -> <C178 as Capacity>::Array<u32> {
    C178::build(0)
}
pub type C179 = Slot<251>;
pub fn build179() -> <C179 as Capacity>::Array<u32> {
    C179::build(0)
}
pub type C180 = Slot<199>;
pub fn build180() -> <C180 as Capacity>::Array<u32> {
    C180::build(0)
}
pub type C181 = Slot<201>;
pub fn build181() -> <C181 as Capacity>::Array<u32> {
    C181::build(0)
}
pub type C182 = Slot<202>;
pub fn build182() -> <C182 as Capacity>::Array<u32> {
    C182::build(0)
}
pub type C183 = Slot<205>;
pub fn build183() -> <C183 as Capacity>::Array<u32> {
    C183::build(0)
}
pub type C184 = Slot<206>;
pub fn build184() -> <C184 as Capacity>::Array<u32> {
    C184::build(0)
}
pub type C185 = Slot<211>;
pub fn build185() -> <C185 as Capacity>::Array<u32> {
    C185::build(0)
}
pub type C186 = Slot<214>;
pub fn build186() -> <C186 as Capacity>::Array<u32> {
    C186::build(0)
}
pub type C187 = Slot<226>;
pub fn build187() -> <C187 as Capacity>::Array<u32> {
    C187::build(0)
}
pub type C188 = Slot<230>;
pub fn build188() -> <C188 as Capacity>::Array<u32> {
    C188::build(0)
}
pub type C189 = Slot<262>;
pub fn build189() -> <C189 as Capacity>::Array<u32> {
    C189::build(0)
}
pub type C190 = Slot<210>;
pub fn build190() -> <C190 as Capacity>::Array<u32> {
    C190::build(0)
}
pub type C191 = Slot<212>;
pub fn build191() -> <C191 as Capacity>::Array<u32> {
    C191::build(0)
}
pub type C192 = Slot<213>;
pub fn build192() -> <C192 as Capacity>::Array<u32> {
    C192::build(0)
}
pub type C193 = Slot<216>;
pub fn build193() -> <C193 as Capacity>::Array<u32> {
    C193::build(0)
}
pub type C194 = Slot<217>;
pub fn build194() -> <C194 as Capacity>::Array<u32> {
    C194::build(0)
}
pub type C195 = Slot<222>;
pub fn build195() -> <C195 as Capacity>::Array<u32> {
    C195::build(0)
}
pub type C196 = Slot<225>;
pub fn build196() -> <C196 as Capacity>::Array<u32> {
    C196::build(0)
}
pub type C197 = Slot<237>;
pub fn build197() -> <C197 as Capacity>::Array<u32> {
    C197::build(0)
}
pub type C198 = Slot<241>;
pub fn build198() -> <C198 as Capacity>::Array<u32> {
    C198::build(0)
}
pub type C199 = Slot<273>;
pub fn build199() -> <C199 as Capacity>::Array<u32> {
    C199::build(0)
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
pub fn call100() -> u32 {
    scaled_fold::<81, 0, Warm, C100>(100)
}
pub fn call101() -> u32 {
    scaled_fold::<86, 0, Cold, C101>(101)
}
pub fn call102() -> u32 {
    scaled_fold::<97, 0, Hot, C102>(102)
}
pub fn call103() -> u32 {
    scaled_fold::<98, 0, Warm, C103>(103)
}
pub fn call104() -> u32 {
    scaled_fold::<134, 0, Cold, C104>(104)
}
pub fn call106() -> u32 {
    scaled_fold::<80, 0, Warm, C106>(106)
}
pub fn call107() -> u32 {
    scaled_fold::<82, 0, Cold, C107>(107)
}
pub fn call108() -> u32 {
    scaled_fold::<88, 0, Hot, C108>(108)
}
pub fn call109() -> u32 {
    scaled_fold::<93, 0, Warm, C109>(109)
}
pub fn call110() -> u32 {
    scaled_fold::<104, 0, Cold, C110>(110)
}
pub fn call111() -> u32 {
    scaled_fold::<105, 0, Hot, C111>(111)
}
pub fn call112() -> u32 {
    scaled_fold::<141, 0, Warm, C112>(112)
}
pub fn call114() -> u32 {
    scaled_fold::<87, 0, Hot, C114>(114)
}
pub fn call115() -> u32 {
    scaled_fold::<89, 0, Warm, C115>(115)
}
pub fn call116() -> u32 {
    scaled_fold::<95, 0, Cold, C116>(116)
}
pub fn call117() -> u32 {
    scaled_fold::<100, 0, Hot, C117>(117)
}
pub fn call118() -> u32 {
    scaled_fold::<111, 0, Warm, C118>(118)
}
pub fn call119() -> u32 {
    scaled_fold::<112, 0, Cold, C119>(119)
}
pub fn call120() -> u32 {
    scaled_fold::<148, 0, Hot, C120>(120)
}
pub fn call122() -> u32 {
    scaled_fold::<94, 0, Cold, C122>(122)
}
pub fn call123() -> u32 {
    scaled_fold::<96, 0, Hot, C123>(123)
}
pub fn call124() -> u32 {
    scaled_fold::<102, 0, Warm, C124>(124)
}
pub fn call125() -> u32 {
    scaled_fold::<107, 0, Cold, C125>(125)
}
pub fn call126() -> u32 {
    scaled_fold::<118, 0, Hot, C126>(126)
}
pub fn call127() -> u32 {
    scaled_fold::<119, 0, Warm, C127>(127)
}
pub fn call128() -> u32 {
    scaled_fold::<155, 0, Cold, C128>(128)
}
pub fn call130() -> u32 {
    scaled_fold::<101, 0, Warm, C130>(130)
}
pub fn call131() -> u32 {
    scaled_fold::<103, 0, Cold, C131>(131)
}
pub fn call132() -> u32 {
    scaled_fold::<109, 0, Hot, C132>(132)
}
pub fn call133() -> u32 {
    scaled_fold::<114, 0, Warm, C133>(133)
}
pub fn call134() -> u32 {
    scaled_fold::<125, 0, Cold, C134>(134)
}
pub fn call135() -> u32 {
    scaled_fold::<126, 0, Hot, C135>(135)
}
pub fn call136() -> u32 {
    scaled_fold::<162, 0, Warm, C136>(136)
}
pub fn call138() -> u32 {
    scaled_fold::<108, 0, Hot, C138>(138)
}
pub fn call139() -> u32 {
    scaled_fold::<110, 0, Warm, C139>(139)
}
pub fn call140() -> u32 {
    scaled_fold::<116, 0, Cold, C140>(140)
}
pub fn call141() -> u32 {
    scaled_fold::<121, 0, Hot, C141>(141)
}
pub fn call142() -> u32 {
    scaled_fold::<132, 0, Warm, C142>(142)
}
pub fn call143() -> u32 {
    scaled_fold::<133, 0, Cold, C143>(143)
}
pub fn call144() -> u32 {
    scaled_fold::<169, 0, Hot, C144>(144)
}
pub fn call146() -> u32 {
    scaled_fold::<115, 0, Cold, C146>(146)
}
pub fn call147() -> u32 {
    scaled_fold::<117, 0, Hot, C147>(147)
}
pub fn call148() -> u32 {
    scaled_fold::<123, 0, Warm, C148>(148)
}
pub fn call149() -> u32 {
    scaled_fold::<128, 0, Cold, C149>(149)
}
pub fn call150() -> u32 {
    scaled_fold::<139, 0, Hot, C150>(150)
}
pub fn call151() -> u32 {
    scaled_fold::<140, 0, Warm, C151>(151)
}
pub fn call152() -> u32 {
    scaled_fold::<176, 0, Cold, C152>(152)
}
pub fn call154() -> u32 {
    scaled_fold::<122, 0, Warm, C154>(154)
}
pub fn call155() -> u32 {
    scaled_fold::<124, 0, Cold, C155>(155)
}
pub fn call156() -> u32 {
    scaled_fold::<130, 0, Hot, C156>(156)
}
pub fn call157() -> u32 {
    scaled_fold::<135, 0, Warm, C157>(157)
}
pub fn call158() -> u32 {
    scaled_fold::<146, 0, Cold, C158>(158)
}
pub fn call159() -> u32 {
    scaled_fold::<147, 0, Hot, C159>(159)
}
pub fn call160() -> u32 {
    scaled_fold::<183, 0, Warm, C160>(160)
}
pub fn call162() -> u32 {
    scaled_fold::<129, 0, Hot, C162>(162)
}
pub fn call163() -> u32 {
    scaled_fold::<131, 0, Warm, C163>(163)
}
pub fn call164() -> u32 {
    scaled_fold::<137, 0, Cold, C164>(164)
}
pub fn call165() -> u32 {
    scaled_fold::<142, 0, Hot, C165>(165)
}
pub fn call166() -> u32 {
    scaled_fold::<153, 0, Warm, C166>(166)
}
pub fn call167() -> u32 {
    scaled_fold::<154, 0, Cold, C167>(167)
}
pub fn call168() -> u32 {
    scaled_fold::<190, 0, Hot, C168>(168)
}
pub fn call170() -> u32 {
    scaled_fold::<136, 0, Cold, C170>(170)
}
pub fn call171() -> u32 {
    scaled_fold::<138, 0, Hot, C171>(171)
}
pub fn call172() -> u32 {
    scaled_fold::<144, 0, Warm, C172>(172)
}
pub fn call173() -> u32 {
    scaled_fold::<149, 0, Cold, C173>(173)
}
pub fn call174() -> u32 {
    scaled_fold::<160, 0, Hot, C174>(174)
}
pub fn call175() -> u32 {
    scaled_fold::<161, 0, Warm, C175>(175)
}
pub fn call176() -> u32 {
    scaled_fold::<197, 0, Cold, C176>(176)
}
pub fn call178() -> u32 {
    scaled_fold::<143, 0, Warm, C178>(178)
}
pub fn call179() -> u32 {
    scaled_fold::<145, 0, Cold, C179>(179)
}
pub fn call180() -> u32 {
    scaled_fold::<151, 0, Hot, C180>(180)
}
pub fn call181() -> u32 {
    scaled_fold::<156, 0, Warm, C181>(181)
}
pub fn call182() -> u32 {
    scaled_fold::<167, 0, Cold, C182>(182)
}
pub fn call183() -> u32 {
    scaled_fold::<168, 0, Hot, C183>(183)
}
pub fn call184() -> u32 {
    scaled_fold::<204, 0, Warm, C184>(184)
}
pub fn call186() -> u32 {
    scaled_fold::<150, 0, Hot, C186>(186)
}
pub fn call187() -> u32 {
    scaled_fold::<152, 0, Warm, C187>(187)
}
pub fn call188() -> u32 {
    scaled_fold::<158, 0, Cold, C188>(188)
}
pub fn call189() -> u32 {
    scaled_fold::<163, 0, Hot, C189>(189)
}
pub fn call190() -> u32 {
    scaled_fold::<174, 0, Warm, C190>(190)
}
pub fn call191() -> u32 {
    scaled_fold::<175, 0, Cold, C191>(191)
}
pub fn call192() -> u32 {
    scaled_fold::<211, 0, Hot, C192>(192)
}
pub fn call194() -> u32 {
    scaled_fold::<157, 0, Cold, C194>(194)
}
pub fn call195() -> u32 {
    scaled_fold::<159, 0, Hot, C195>(195)
}
pub fn call196() -> u32 {
    scaled_fold::<165, 0, Warm, C196>(196)
}
pub fn call197() -> u32 {
    scaled_fold::<170, 0, Cold, C197>(197)
}
pub fn call198() -> u32 {
    scaled_fold::<181, 0, Hot, C198>(198)
}
pub fn call199() -> u32 {
    scaled_fold::<182, 0, Warm, C199>(199)
}
