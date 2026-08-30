#![no_std]
use mach::*;
pub const W0: usize = 1;
pub const ONE_OK_0: bool = true;
pub const F0_IS_ZERO: bool = true;
pub const W1: usize = 2;
pub const ONE_OK_1: bool = true;
pub const F1_IS_ZERO: bool = true;
pub const W2: usize = 3;
pub const ONE_OK_2: bool = true;
pub const F2_IS_ZERO: bool = true;
pub const W3: usize = 4;
pub const ONE_OK_3: bool = true;
pub const F3_IS_ZERO: bool = true;
pub const W4: usize = 5;
pub const ONE_OK_4: bool = true;
pub const F4_IS_ZERO: bool = true;
pub const W5: usize = 6;
pub const ONE_OK_5: bool = true;
pub const F5_IS_ZERO: bool = true;
pub const W6: usize = 7;
pub const ONE_OK_6: bool = true;
pub const F6_IS_ZERO: bool = true;
pub const W7: usize = 11;
pub const ONE_OK_7: bool = true;
pub const F7_IS_ZERO: bool = true;
pub const W8: usize = 14;
pub const ONE_OK_8: bool = true;
pub const F8_IS_ZERO: bool = true;
pub const W9: usize = 16;
pub const ONE_OK_9: bool = true;
pub const F9_IS_ZERO: bool = true;
pub const W10: usize = 27;
pub const ONE_OK_10: bool = true;
pub const F10_IS_ZERO: bool = true;
pub const W11: usize = 28;
pub const ONE_OK_11: bool = true;
pub const F11_IS_ZERO: bool = true;
pub const W12: usize = 64;
pub const ONE_OK_12: bool = true;
pub const F12_IS_ZERO: bool = true;
pub const W13: usize = 16;
pub const ONE_OK_13: bool = false;
pub const F13_IS_ZERO: bool = false;
pub const W14: usize = 8;
pub const ONE_OK_14: bool = true;
pub const F14_IS_ZERO: bool = true;
pub const W15: usize = 9;
pub const ONE_OK_15: bool = true;
pub const F15_IS_ZERO: bool = true;
pub const W16: usize = 10;
pub const ONE_OK_16: bool = true;
pub const F16_IS_ZERO: bool = true;
pub const W17: usize = 12;
pub const ONE_OK_17: bool = true;
pub const F17_IS_ZERO: bool = true;
pub const W18: usize = 13;
pub const ONE_OK_18: bool = true;
pub const F18_IS_ZERO: bool = true;
pub const W19: usize = 18;
pub const ONE_OK_19: bool = true;
pub const F19_IS_ZERO: bool = true;
pub const W20: usize = 21;
pub const ONE_OK_20: bool = true;
pub const F20_IS_ZERO: bool = true;
pub const W21: usize = 23;
pub const ONE_OK_21: bool = true;
pub const F21_IS_ZERO: bool = true;
pub const W22: usize = 34;
pub const ONE_OK_22: bool = true;
pub const F22_IS_ZERO: bool = true;
pub const W23: usize = 35;
pub const ONE_OK_23: bool = true;
pub const F23_IS_ZERO: bool = true;
pub const W24: usize = 71;
pub const ONE_OK_24: bool = true;
pub const F24_IS_ZERO: bool = true;
pub const W25: usize = 17;
pub const ONE_OK_25: bool = false;
pub const F25_IS_ZERO: bool = false;
pub const W26: usize = 15;
pub const ONE_OK_26: bool = true;
pub const F26_IS_ZERO: bool = true;
pub const W27: usize = 17;
pub const ONE_OK_27: bool = true;
pub const F27_IS_ZERO: bool = true;
pub const W28: usize = 19;
pub const ONE_OK_28: bool = true;
pub const F28_IS_ZERO: bool = true;
pub const W29: usize = 20;
pub const ONE_OK_29: bool = true;
pub const F29_IS_ZERO: bool = true;
pub const W30: usize = 25;
pub const ONE_OK_30: bool = true;
pub const F30_IS_ZERO: bool = true;
pub const W31: usize = 30;
pub const ONE_OK_31: bool = true;
pub const F31_IS_ZERO: bool = true;
pub const W32: usize = 41;
pub const ONE_OK_32: bool = true;
pub const F32_IS_ZERO: bool = true;
pub const W33: usize = 42;
pub const ONE_OK_33: bool = true;
pub const F33_IS_ZERO: bool = true;
pub const W34: usize = 78;
pub const ONE_OK_34: bool = true;
pub const F34_IS_ZERO: bool = true;
pub const W35: usize = 18;
pub const ONE_OK_35: bool = false;
pub const F35_IS_ZERO: bool = false;
pub const W36: usize = 22;
pub const ONE_OK_36: bool = true;
pub const F36_IS_ZERO: bool = true;
pub const W37: usize = 24;
pub const ONE_OK_37: bool = true;
pub const F37_IS_ZERO: bool = true;
pub const W38: usize = 26;
pub const ONE_OK_38: bool = true;
pub const F38_IS_ZERO: bool = true;
pub const W39: usize = 32;
pub const ONE_OK_39: bool = true;
pub const F39_IS_ZERO: bool = true;
pub const W40: usize = 37;
pub const ONE_OK_40: bool = true;
pub const F40_IS_ZERO: bool = true;
pub const W41: usize = 48;
pub const ONE_OK_41: bool = true;
pub const F41_IS_ZERO: bool = true;
pub const W42: usize = 49;
pub const ONE_OK_42: bool = true;
pub const F42_IS_ZERO: bool = true;
pub const W43: usize = 85;
pub const ONE_OK_43: bool = true;
pub const F43_IS_ZERO: bool = true;
pub const W44: usize = 19;
pub const ONE_OK_44: bool = false;
pub const F44_IS_ZERO: bool = false;
pub const W45: usize = 29;
pub const ONE_OK_45: bool = true;
pub const F45_IS_ZERO: bool = true;
pub const W46: usize = 31;
pub const ONE_OK_46: bool = true;
pub const F46_IS_ZERO: bool = true;
pub const W47: usize = 33;
pub const ONE_OK_47: bool = true;
pub const F47_IS_ZERO: bool = true;
pub const W48: usize = 39;
pub const ONE_OK_48: bool = true;
pub const F48_IS_ZERO: bool = true;
pub const W49: usize = 44;
pub const ONE_OK_49: bool = true;
pub const F49_IS_ZERO: bool = true;
pub const W50: usize = 55;
pub const ONE_OK_50: bool = true;
pub const F50_IS_ZERO: bool = true;
pub const W51: usize = 56;
pub const ONE_OK_51: bool = true;
pub const F51_IS_ZERO: bool = true;
pub const W52: usize = 92;
pub const ONE_OK_52: bool = true;
pub const F52_IS_ZERO: bool = true;
pub const W53: usize = 20;
pub const ONE_OK_53: bool = false;
pub const F53_IS_ZERO: bool = false;
pub const W54: usize = 36;
pub const ONE_OK_54: bool = true;
pub const F54_IS_ZERO: bool = true;
pub const W55: usize = 38;
pub const ONE_OK_55: bool = true;
pub const F55_IS_ZERO: bool = true;
pub const W56: usize = 40;
pub const ONE_OK_56: bool = true;
pub const F56_IS_ZERO: bool = true;
pub const W57: usize = 46;
pub const ONE_OK_57: bool = true;
pub const F57_IS_ZERO: bool = true;
pub const W58: usize = 51;
pub const ONE_OK_58: bool = true;
pub const F58_IS_ZERO: bool = true;
pub const W59: usize = 62;
pub const ONE_OK_59: bool = true;
pub const F59_IS_ZERO: bool = true;
pub const W60: usize = 63;
pub const ONE_OK_60: bool = true;
pub const F60_IS_ZERO: bool = true;
pub const W61: usize = 99;
pub const ONE_OK_61: bool = true;
pub const F61_IS_ZERO: bool = true;
pub const W62: usize = 21;
pub const ONE_OK_62: bool = false;
pub const F62_IS_ZERO: bool = false;
pub const W63: usize = 43;
pub const ONE_OK_63: bool = true;
pub const F63_IS_ZERO: bool = true;
pub const W64: usize = 45;
pub const ONE_OK_64: bool = true;
pub const F64_IS_ZERO: bool = true;
pub const W65: usize = 47;
pub const ONE_OK_65: bool = true;
pub const F65_IS_ZERO: bool = true;
pub const W66: usize = 53;
pub const ONE_OK_66: bool = true;
pub const F66_IS_ZERO: bool = true;
pub const W67: usize = 58;
pub const ONE_OK_67: bool = true;
pub const F67_IS_ZERO: bool = true;
pub const W68: usize = 69;
pub const ONE_OK_68: bool = true;
pub const F68_IS_ZERO: bool = true;
pub const W69: usize = 70;
pub const ONE_OK_69: bool = true;
pub const F69_IS_ZERO: bool = true;
pub const W70: usize = 106;
pub const ONE_OK_70: bool = true;
pub const F70_IS_ZERO: bool = true;
pub const W71: usize = 22;
pub const ONE_OK_71: bool = false;
pub const F71_IS_ZERO: bool = false;
pub const W72: usize = 50;
pub const ONE_OK_72: bool = true;
pub const F72_IS_ZERO: bool = true;
pub const W73: usize = 52;
pub const ONE_OK_73: bool = true;
pub const F73_IS_ZERO: bool = true;
pub const W74: usize = 54;
pub const ONE_OK_74: bool = true;
pub const F74_IS_ZERO: bool = true;
pub const W75: usize = 60;
pub const ONE_OK_75: bool = true;
pub const F75_IS_ZERO: bool = true;
pub const W76: usize = 65;
pub const ONE_OK_76: bool = true;
pub const F76_IS_ZERO: bool = true;
pub const W77: usize = 76;
pub const ONE_OK_77: bool = true;
pub const F77_IS_ZERO: bool = true;
pub const W78: usize = 77;
pub const ONE_OK_78: bool = true;
pub const F78_IS_ZERO: bool = true;
pub const W79: usize = 113;
pub const ONE_OK_79: bool = true;
pub const F79_IS_ZERO: bool = true;
pub const W80: usize = 23;
pub const ONE_OK_80: bool = false;
pub const F80_IS_ZERO: bool = false;
pub const W81: usize = 57;
pub const ONE_OK_81: bool = true;
pub const F81_IS_ZERO: bool = true;
pub const W82: usize = 59;
pub const ONE_OK_82: bool = true;
pub const F82_IS_ZERO: bool = true;
pub const W83: usize = 61;
pub const ONE_OK_83: bool = true;
pub const F83_IS_ZERO: bool = true;
pub const W84: usize = 67;
pub const ONE_OK_84: bool = true;
pub const F84_IS_ZERO: bool = true;
pub const W85: usize = 72;
pub const ONE_OK_85: bool = true;
pub const F85_IS_ZERO: bool = true;
pub const W86: usize = 83;
pub const ONE_OK_86: bool = true;
pub const F86_IS_ZERO: bool = true;
pub const W87: usize = 84;
pub const ONE_OK_87: bool = true;
pub const F87_IS_ZERO: bool = true;
pub const W88: usize = 120;
pub const ONE_OK_88: bool = true;
pub const F88_IS_ZERO: bool = true;
pub const W89: usize = 24;
pub const ONE_OK_89: bool = false;
pub const F89_IS_ZERO: bool = false;
pub const W90: usize = 66;
pub const ONE_OK_90: bool = true;
pub const F90_IS_ZERO: bool = true;
pub const W91: usize = 68;
pub const ONE_OK_91: bool = true;
pub const F91_IS_ZERO: bool = true;
pub const W92: usize = 74;
pub const ONE_OK_92: bool = true;
pub const F92_IS_ZERO: bool = true;
pub const W93: usize = 79;
pub const ONE_OK_93: bool = true;
pub const F93_IS_ZERO: bool = true;
pub const W94: usize = 90;
pub const ONE_OK_94: bool = true;
pub const F94_IS_ZERO: bool = true;
pub const W95: usize = 91;
pub const ONE_OK_95: bool = true;
pub const F95_IS_ZERO: bool = true;
pub const W96: usize = 127;
pub const ONE_OK_96: bool = true;
pub const F96_IS_ZERO: bool = true;
pub const W97: usize = 25;
pub const ONE_OK_97: bool = false;
pub const F97_IS_ZERO: bool = false;
pub const W98: usize = 73;
pub const ONE_OK_98: bool = true;
pub const F98_IS_ZERO: bool = true;
pub const W99: usize = 75;
pub const ONE_OK_99: bool = true;
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
pub fn build0() -> [u32; 1] {
    [0; 1]
}
pub type C1 = Slot<3>;
pub fn build1() -> [u32; 3] {
    [0; 3]
}
pub type C2 = Slot<4>;
pub fn build2() -> [u32; 4] {
    [0; 4]
}
pub type C3 = Slot<7>;
pub fn build3() -> [u32; 7] {
    [0; 7]
}
pub type C4 = Slot<8>;
pub fn build4() -> [u32; 8] {
    [0; 8]
}
pub type C5 = Slot<13>;
pub fn build5() -> [u32; 13] {
    [0; 13]
}
pub type C6 = Slot<16>;
pub fn build6() -> [u32; 16] {
    [0; 16]
}
pub type C7 = Slot<28>;
pub fn build7() -> [u32; 28] {
    [0; 28]
}
pub type C8 = Slot<32>;
pub fn build8() -> [u32; 32] {
    [0; 32]
}
pub type C9 = Slot<64>;
pub fn build9() -> [u32; 64] {
    [0; 64]
}
pub type C10 = Slot<12>;
pub fn build10() -> [u32; 12] {
    [0; 12]
}
pub type C11 = Slot<14>;
pub fn build11() -> [u32; 14] {
    [0; 14]
}
pub type C12 = Slot<15>;
pub fn build12() -> [u32; 15] {
    [0; 15]
}
pub type C13 = Slot<18>;
pub fn build13() -> [u32; 18] {
    [0; 18]
}
pub type C14 = Slot<19>;
pub fn build14() -> [u32; 19] {
    [0; 19]
}
pub type C15 = Slot<24>;
pub fn build15() -> [u32; 24] {
    [0; 24]
}
pub type C16 = Slot<27>;
pub fn build16() -> [u32; 27] {
    [0; 27]
}
pub type C17 = Slot<39>;
pub fn build17() -> [u32; 39] {
    [0; 39]
}
pub type C18 = Slot<43>;
pub fn build18() -> [u32; 43] {
    [0; 43]
}
pub type C19 = Slot<75>;
pub fn build19() -> [u32; 75] {
    [0; 75]
}
pub type C20 = Slot<23>;
pub fn build20() -> [u32; 23] {
    [0; 23]
}
pub type C21 = Slot<25>;
pub fn build21() -> [u32; 25] {
    [0; 25]
}
pub type C22 = Slot<26>;
pub fn build22() -> [u32; 26] {
    [0; 26]
}
pub type C23 = Slot<29>;
pub fn build23() -> [u32; 29] {
    [0; 29]
}
pub type C24 = Slot<30>;
pub fn build24() -> [u32; 30] {
    [0; 30]
}
pub type C25 = Slot<35>;
pub fn build25() -> [u32; 35] {
    [0; 35]
}
pub type C26 = Slot<38>;
pub fn build26() -> [u32; 38] {
    [0; 38]
}
pub type C27 = Slot<50>;
pub fn build27() -> [u32; 50] {
    [0; 50]
}
pub type C28 = Slot<54>;
pub fn build28() -> [u32; 54] {
    [0; 54]
}
pub type C29 = Slot<86>;
pub fn build29() -> [u32; 86] {
    [0; 86]
}
pub type C30 = Slot<34>;
pub fn build30() -> [u32; 34] {
    [0; 34]
}
pub type C31 = Slot<36>;
pub fn build31() -> [u32; 36] {
    [0; 36]
}
pub type C32 = Slot<37>;
pub fn build32() -> [u32; 37] {
    [0; 37]
}
pub type C33 = Slot<40>;
pub fn build33() -> [u32; 40] {
    [0; 40]
}
pub type C34 = Slot<41>;
pub fn build34() -> [u32; 41] {
    [0; 41]
}
pub type C35 = Slot<46>;
pub fn build35() -> [u32; 46] {
    [0; 46]
}
pub type C36 = Slot<49>;
pub fn build36() -> [u32; 49] {
    [0; 49]
}
pub type C37 = Slot<61>;
pub fn build37() -> [u32; 61] {
    [0; 61]
}
pub type C38 = Slot<65>;
pub fn build38() -> [u32; 65] {
    [0; 65]
}
pub type C39 = Slot<97>;
pub fn build39() -> [u32; 97] {
    [0; 97]
}
pub type C40 = Slot<45>;
pub fn build40() -> [u32; 45] {
    [0; 45]
}
pub type C41 = Slot<47>;
pub fn build41() -> [u32; 47] {
    [0; 47]
}
pub type C42 = Slot<48>;
pub fn build42() -> [u32; 48] {
    [0; 48]
}
pub type C43 = Slot<51>;
pub fn build43() -> [u32; 51] {
    [0; 51]
}
pub type C44 = Slot<52>;
pub fn build44() -> [u32; 52] {
    [0; 52]
}
pub type C45 = Slot<57>;
pub fn build45() -> [u32; 57] {
    [0; 57]
}
pub type C46 = Slot<60>;
pub fn build46() -> [u32; 60] {
    [0; 60]
}
pub type C47 = Slot<72>;
pub fn build47() -> [u32; 72] {
    [0; 72]
}
pub type C48 = Slot<76>;
pub fn build48() -> [u32; 76] {
    [0; 76]
}
pub type C49 = Slot<108>;
pub fn build49() -> [u32; 108] {
    [0; 108]
}
pub type C50 = Slot<56>;
pub fn build50() -> [u32; 56] {
    [0; 56]
}
pub type C51 = Slot<58>;
pub fn build51() -> [u32; 58] {
    [0; 58]
}
pub type C52 = Slot<59>;
pub fn build52() -> [u32; 59] {
    [0; 59]
}
pub type C53 = Slot<62>;
pub fn build53() -> [u32; 62] {
    [0; 62]
}
pub type C54 = Slot<63>;
pub fn build54() -> [u32; 63] {
    [0; 63]
}
pub type C55 = Slot<68>;
pub fn build55() -> [u32; 68] {
    [0; 68]
}
pub type C56 = Slot<71>;
pub fn build56() -> [u32; 71] {
    [0; 71]
}
pub type C57 = Slot<83>;
pub fn build57() -> [u32; 83] {
    [0; 83]
}
pub type C58 = Slot<87>;
pub fn build58() -> [u32; 87] {
    [0; 87]
}
pub type C59 = Slot<119>;
pub fn build59() -> [u32; 119] {
    [0; 119]
}
pub type C60 = Slot<67>;
pub fn build60() -> [u32; 67] {
    [0; 67]
}
pub type C61 = Slot<69>;
pub fn build61() -> [u32; 69] {
    [0; 69]
}
pub type C62 = Slot<70>;
pub fn build62() -> [u32; 70] {
    [0; 70]
}
pub type C63 = Slot<73>;
pub fn build63() -> [u32; 73] {
    [0; 73]
}
pub type C64 = Slot<74>;
pub fn build64() -> [u32; 74] {
    [0; 74]
}
pub type C65 = Slot<79>;
pub fn build65() -> [u32; 79] {
    [0; 79]
}
pub type C66 = Slot<82>;
pub fn build66() -> [u32; 82] {
    [0; 82]
}
pub type C67 = Slot<94>;
pub fn build67() -> [u32; 94] {
    [0; 94]
}
pub type C68 = Slot<98>;
pub fn build68() -> [u32; 98] {
    [0; 98]
}
pub type C69 = Slot<130>;
pub fn build69() -> [u32; 130] {
    [0; 130]
}
pub type C70 = Slot<78>;
pub fn build70() -> [u32; 78] {
    [0; 78]
}
pub type C71 = Slot<80>;
pub fn build71() -> [u32; 80] {
    [0; 80]
}
pub type C72 = Slot<81>;
pub fn build72() -> [u32; 81] {
    [0; 81]
}
pub type C73 = Slot<84>;
pub fn build73() -> [u32; 84] {
    [0; 84]
}
pub type C74 = Slot<85>;
pub fn build74() -> [u32; 85] {
    [0; 85]
}
pub type C75 = Slot<90>;
pub fn build75() -> [u32; 90] {
    [0; 90]
}
pub type C76 = Slot<93>;
pub fn build76() -> [u32; 93] {
    [0; 93]
}
pub type C77 = Slot<105>;
pub fn build77() -> [u32; 105] {
    [0; 105]
}
pub type C78 = Slot<109>;
pub fn build78() -> [u32; 109] {
    [0; 109]
}
pub type C79 = Slot<141>;
pub fn build79() -> [u32; 141] {
    [0; 141]
}
pub type C80 = Slot<89>;
pub fn build80() -> [u32; 89] {
    [0; 89]
}
pub type C81 = Slot<91>;
pub fn build81() -> [u32; 91] {
    [0; 91]
}
pub type C82 = Slot<92>;
pub fn build82() -> [u32; 92] {
    [0; 92]
}
pub type C83 = Slot<95>;
pub fn build83() -> [u32; 95] {
    [0; 95]
}
pub type C84 = Slot<96>;
pub fn build84() -> [u32; 96] {
    [0; 96]
}
pub type C85 = Slot<101>;
pub fn build85() -> [u32; 101] {
    [0; 101]
}
pub type C86 = Slot<104>;
pub fn build86() -> [u32; 104] {
    [0; 104]
}
pub type C87 = Slot<116>;
pub fn build87() -> [u32; 116] {
    [0; 116]
}
pub type C88 = Slot<120>;
pub fn build88() -> [u32; 120] {
    [0; 120]
}
pub type C89 = Slot<152>;
pub fn build89() -> [u32; 152] {
    [0; 152]
}
pub type C90 = Slot<100>;
pub fn build90() -> [u32; 100] {
    [0; 100]
}
pub type C91 = Slot<102>;
pub fn build91() -> [u32; 102] {
    [0; 102]
}
pub type C92 = Slot<103>;
pub fn build92() -> [u32; 103] {
    [0; 103]
}
pub type C93 = Slot<106>;
pub fn build93() -> [u32; 106] {
    [0; 106]
}
pub type C94 = Slot<107>;
pub fn build94() -> [u32; 107] {
    [0; 107]
}
pub type C95 = Slot<112>;
pub fn build95() -> [u32; 112] {
    [0; 112]
}
pub type C96 = Slot<115>;
pub fn build96() -> [u32; 115] {
    [0; 115]
}
pub type C97 = Slot<127>;
pub fn build97() -> [u32; 127] {
    [0; 127]
}
pub type C98 = Slot<131>;
pub fn build98() -> [u32; 131] {
    [0; 131]
}
pub type C99 = Slot<163>;
pub fn build99() -> [u32; 163] {
    [0; 163]
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
pub fn scaled_fold<C: Capacity>(seed: u32, w: usize) -> u32 {
    fold_generic::<C>(seed).wrapping_mul(w as u32)
}

pub fn call0() -> u32 {
    scaled_fold::<C0>(0, W0)
}
pub fn call1() -> u32 {
    scaled_fold::<C1>(1, W1)
}
pub fn call2() -> u32 {
    scaled_fold::<C2>(2, W2)
}
pub fn call3() -> u32 {
    scaled_fold::<C3>(3, W3)
}
pub fn call4() -> u32 {
    scaled_fold::<C4>(4, W4)
}
pub fn call5() -> u32 {
    scaled_fold::<C5>(5, W5)
}
pub fn call6() -> u32 {
    scaled_fold::<C6>(6, W6)
}
pub fn call7() -> u32 {
    scaled_fold::<C7>(7, W7)
}
pub fn call8() -> u32 {
    scaled_fold::<C8>(8, W8)
}
pub fn call9() -> u32 {
    scaled_fold::<C9>(9, W9)
}
pub fn call10() -> u32 {
    scaled_fold::<C10>(10, W10)
}
pub fn call11() -> u32 {
    scaled_fold::<C11>(11, W11)
}
pub fn call12() -> u32 {
    scaled_fold::<C12>(12, W12)
}
pub fn call14() -> u32 {
    scaled_fold::<C14>(14, W14)
}
pub fn call15() -> u32 {
    scaled_fold::<C15>(15, W15)
}
pub fn call16() -> u32 {
    scaled_fold::<C16>(16, W16)
}
pub fn call17() -> u32 {
    scaled_fold::<C17>(17, W17)
}
pub fn call18() -> u32 {
    scaled_fold::<C18>(18, W18)
}
pub fn call19() -> u32 {
    scaled_fold::<C19>(19, W19)
}
pub fn call20() -> u32 {
    scaled_fold::<C20>(20, W20)
}
pub fn call21() -> u32 {
    scaled_fold::<C21>(21, W21)
}
pub fn call22() -> u32 {
    scaled_fold::<C22>(22, W22)
}
pub fn call23() -> u32 {
    scaled_fold::<C23>(23, W23)
}
pub fn call24() -> u32 {
    scaled_fold::<C24>(24, W24)
}
pub fn call26() -> u32 {
    scaled_fold::<C26>(26, W26)
}
pub fn call27() -> u32 {
    scaled_fold::<C27>(27, W27)
}
pub fn call28() -> u32 {
    scaled_fold::<C28>(28, W28)
}
pub fn call29() -> u32 {
    scaled_fold::<C29>(29, W29)
}
pub fn call30() -> u32 {
    scaled_fold::<C30>(30, W30)
}
pub fn call31() -> u32 {
    scaled_fold::<C31>(31, W31)
}
pub fn call32() -> u32 {
    scaled_fold::<C32>(32, W32)
}
pub fn call33() -> u32 {
    scaled_fold::<C33>(33, W33)
}
pub fn call34() -> u32 {
    scaled_fold::<C34>(34, W34)
}
pub fn call36() -> u32 {
    scaled_fold::<C36>(36, W36)
}
pub fn call37() -> u32 {
    scaled_fold::<C37>(37, W37)
}
pub fn call38() -> u32 {
    scaled_fold::<C38>(38, W38)
}
pub fn call39() -> u32 {
    scaled_fold::<C39>(39, W39)
}
pub fn call40() -> u32 {
    scaled_fold::<C40>(40, W40)
}
pub fn call41() -> u32 {
    scaled_fold::<C41>(41, W41)
}
pub fn call42() -> u32 {
    scaled_fold::<C42>(42, W42)
}
pub fn call43() -> u32 {
    scaled_fold::<C43>(43, W43)
}
pub fn call45() -> u32 {
    scaled_fold::<C45>(45, W45)
}
pub fn call46() -> u32 {
    scaled_fold::<C46>(46, W46)
}
pub fn call47() -> u32 {
    scaled_fold::<C47>(47, W47)
}
pub fn call48() -> u32 {
    scaled_fold::<C48>(48, W48)
}
pub fn call49() -> u32 {
    scaled_fold::<C49>(49, W49)
}
pub fn call50() -> u32 {
    scaled_fold::<C50>(50, W50)
}
pub fn call51() -> u32 {
    scaled_fold::<C51>(51, W51)
}
pub fn call52() -> u32 {
    scaled_fold::<C52>(52, W52)
}
pub fn call54() -> u32 {
    scaled_fold::<C54>(54, W54)
}
pub fn call55() -> u32 {
    scaled_fold::<C55>(55, W55)
}
pub fn call56() -> u32 {
    scaled_fold::<C56>(56, W56)
}
pub fn call57() -> u32 {
    scaled_fold::<C57>(57, W57)
}
pub fn call58() -> u32 {
    scaled_fold::<C58>(58, W58)
}
pub fn call59() -> u32 {
    scaled_fold::<C59>(59, W59)
}
pub fn call60() -> u32 {
    scaled_fold::<C60>(60, W60)
}
pub fn call61() -> u32 {
    scaled_fold::<C61>(61, W61)
}
pub fn call63() -> u32 {
    scaled_fold::<C63>(63, W63)
}
pub fn call64() -> u32 {
    scaled_fold::<C64>(64, W64)
}
pub fn call65() -> u32 {
    scaled_fold::<C65>(65, W65)
}
pub fn call66() -> u32 {
    scaled_fold::<C66>(66, W66)
}
pub fn call67() -> u32 {
    scaled_fold::<C67>(67, W67)
}
pub fn call68() -> u32 {
    scaled_fold::<C68>(68, W68)
}
pub fn call69() -> u32 {
    scaled_fold::<C69>(69, W69)
}
pub fn call70() -> u32 {
    scaled_fold::<C70>(70, W70)
}
pub fn call72() -> u32 {
    scaled_fold::<C72>(72, W72)
}
pub fn call73() -> u32 {
    scaled_fold::<C73>(73, W73)
}
pub fn call74() -> u32 {
    scaled_fold::<C74>(74, W74)
}
pub fn call75() -> u32 {
    scaled_fold::<C75>(75, W75)
}
pub fn call76() -> u32 {
    scaled_fold::<C76>(76, W76)
}
pub fn call77() -> u32 {
    scaled_fold::<C77>(77, W77)
}
pub fn call78() -> u32 {
    scaled_fold::<C78>(78, W78)
}
pub fn call79() -> u32 {
    scaled_fold::<C79>(79, W79)
}
pub fn call81() -> u32 {
    scaled_fold::<C81>(81, W81)
}
pub fn call82() -> u32 {
    scaled_fold::<C82>(82, W82)
}
pub fn call83() -> u32 {
    scaled_fold::<C83>(83, W83)
}
pub fn call84() -> u32 {
    scaled_fold::<C84>(84, W84)
}
pub fn call85() -> u32 {
    scaled_fold::<C85>(85, W85)
}
pub fn call86() -> u32 {
    scaled_fold::<C86>(86, W86)
}
pub fn call87() -> u32 {
    scaled_fold::<C87>(87, W87)
}
pub fn call88() -> u32 {
    scaled_fold::<C88>(88, W88)
}
pub fn call90() -> u32 {
    scaled_fold::<C90>(90, W90)
}
pub fn call91() -> u32 {
    scaled_fold::<C91>(91, W91)
}
pub fn call92() -> u32 {
    scaled_fold::<C92>(92, W92)
}
pub fn call93() -> u32 {
    scaled_fold::<C93>(93, W93)
}
pub fn call94() -> u32 {
    scaled_fold::<C94>(94, W94)
}
pub fn call95() -> u32 {
    scaled_fold::<C95>(95, W95)
}
pub fn call96() -> u32 {
    scaled_fold::<C96>(96, W96)
}
pub fn call98() -> u32 {
    scaled_fold::<C98>(98, W98)
}
pub fn call99() -> u32 {
    scaled_fold::<C99>(99, W99)
}
