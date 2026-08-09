#![no_std]
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
pub const W100: usize = 81;
pub const ONE_OK_100: bool = true;
pub const F100_IS_ZERO: bool = true;
pub const W101: usize = 86;
pub const ONE_OK_101: bool = true;
pub const F101_IS_ZERO: bool = true;
pub const W102: usize = 97;
pub const ONE_OK_102: bool = true;
pub const F102_IS_ZERO: bool = true;
pub const W103: usize = 98;
pub const ONE_OK_103: bool = true;
pub const F103_IS_ZERO: bool = true;
pub const W104: usize = 134;
pub const ONE_OK_104: bool = true;
pub const F104_IS_ZERO: bool = true;
pub const W105: usize = 26;
pub const ONE_OK_105: bool = false;
pub const F105_IS_ZERO: bool = false;
pub const W106: usize = 80;
pub const ONE_OK_106: bool = true;
pub const F106_IS_ZERO: bool = true;
pub const W107: usize = 82;
pub const ONE_OK_107: bool = true;
pub const F107_IS_ZERO: bool = true;
pub const W108: usize = 88;
pub const ONE_OK_108: bool = true;
pub const F108_IS_ZERO: bool = true;
pub const W109: usize = 93;
pub const ONE_OK_109: bool = true;
pub const F109_IS_ZERO: bool = true;
pub const W110: usize = 104;
pub const ONE_OK_110: bool = true;
pub const F110_IS_ZERO: bool = true;
pub const W111: usize = 105;
pub const ONE_OK_111: bool = true;
pub const F111_IS_ZERO: bool = true;
pub const W112: usize = 141;
pub const ONE_OK_112: bool = true;
pub const F112_IS_ZERO: bool = true;
pub const W113: usize = 27;
pub const ONE_OK_113: bool = false;
pub const F113_IS_ZERO: bool = false;
pub const W114: usize = 87;
pub const ONE_OK_114: bool = true;
pub const F114_IS_ZERO: bool = true;
pub const W115: usize = 89;
pub const ONE_OK_115: bool = true;
pub const F115_IS_ZERO: bool = true;
pub const W116: usize = 95;
pub const ONE_OK_116: bool = true;
pub const F116_IS_ZERO: bool = true;
pub const W117: usize = 100;
pub const ONE_OK_117: bool = true;
pub const F117_IS_ZERO: bool = true;
pub const W118: usize = 111;
pub const ONE_OK_118: bool = true;
pub const F118_IS_ZERO: bool = true;
pub const W119: usize = 112;
pub const ONE_OK_119: bool = true;
pub const F119_IS_ZERO: bool = true;
pub const W120: usize = 148;
pub const ONE_OK_120: bool = true;
pub const F120_IS_ZERO: bool = true;
pub const W121: usize = 28;
pub const ONE_OK_121: bool = false;
pub const F121_IS_ZERO: bool = false;
pub const W122: usize = 94;
pub const ONE_OK_122: bool = true;
pub const F122_IS_ZERO: bool = true;
pub const W123: usize = 96;
pub const ONE_OK_123: bool = true;
pub const F123_IS_ZERO: bool = true;
pub const W124: usize = 102;
pub const ONE_OK_124: bool = true;
pub const F124_IS_ZERO: bool = true;
pub const W125: usize = 107;
pub const ONE_OK_125: bool = true;
pub const F125_IS_ZERO: bool = true;
pub const W126: usize = 118;
pub const ONE_OK_126: bool = true;
pub const F126_IS_ZERO: bool = true;
pub const W127: usize = 119;
pub const ONE_OK_127: bool = true;
pub const F127_IS_ZERO: bool = true;
pub const W128: usize = 155;
pub const ONE_OK_128: bool = true;
pub const F128_IS_ZERO: bool = true;
pub const W129: usize = 29;
pub const ONE_OK_129: bool = false;
pub const F129_IS_ZERO: bool = false;
pub const W130: usize = 101;
pub const ONE_OK_130: bool = true;
pub const F130_IS_ZERO: bool = true;
pub const W131: usize = 103;
pub const ONE_OK_131: bool = true;
pub const F131_IS_ZERO: bool = true;
pub const W132: usize = 109;
pub const ONE_OK_132: bool = true;
pub const F132_IS_ZERO: bool = true;
pub const W133: usize = 114;
pub const ONE_OK_133: bool = true;
pub const F133_IS_ZERO: bool = true;
pub const W134: usize = 125;
pub const ONE_OK_134: bool = true;
pub const F134_IS_ZERO: bool = true;
pub const W135: usize = 126;
pub const ONE_OK_135: bool = true;
pub const F135_IS_ZERO: bool = true;
pub const W136: usize = 162;
pub const ONE_OK_136: bool = true;
pub const F136_IS_ZERO: bool = true;
pub const W137: usize = 30;
pub const ONE_OK_137: bool = false;
pub const F137_IS_ZERO: bool = false;
pub const W138: usize = 108;
pub const ONE_OK_138: bool = true;
pub const F138_IS_ZERO: bool = true;
pub const W139: usize = 110;
pub const ONE_OK_139: bool = true;
pub const F139_IS_ZERO: bool = true;
pub const W140: usize = 116;
pub const ONE_OK_140: bool = true;
pub const F140_IS_ZERO: bool = true;
pub const W141: usize = 121;
pub const ONE_OK_141: bool = true;
pub const F141_IS_ZERO: bool = true;
pub const W142: usize = 132;
pub const ONE_OK_142: bool = true;
pub const F142_IS_ZERO: bool = true;
pub const W143: usize = 133;
pub const ONE_OK_143: bool = true;
pub const F143_IS_ZERO: bool = true;
pub const W144: usize = 169;
pub const ONE_OK_144: bool = true;
pub const F144_IS_ZERO: bool = true;
pub const W145: usize = 31;
pub const ONE_OK_145: bool = false;
pub const F145_IS_ZERO: bool = false;
pub const W146: usize = 115;
pub const ONE_OK_146: bool = true;
pub const F146_IS_ZERO: bool = true;
pub const W147: usize = 117;
pub const ONE_OK_147: bool = true;
pub const F147_IS_ZERO: bool = true;
pub const W148: usize = 123;
pub const ONE_OK_148: bool = true;
pub const F148_IS_ZERO: bool = true;
pub const W149: usize = 128;
pub const ONE_OK_149: bool = true;
pub const F149_IS_ZERO: bool = true;
pub const W150: usize = 139;
pub const ONE_OK_150: bool = true;
pub const F150_IS_ZERO: bool = true;
pub const W151: usize = 140;
pub const ONE_OK_151: bool = true;
pub const F151_IS_ZERO: bool = true;
pub const W152: usize = 176;
pub const ONE_OK_152: bool = true;
pub const F152_IS_ZERO: bool = true;
pub const W153: usize = 32;
pub const ONE_OK_153: bool = false;
pub const F153_IS_ZERO: bool = false;
pub const W154: usize = 122;
pub const ONE_OK_154: bool = true;
pub const F154_IS_ZERO: bool = true;
pub const W155: usize = 124;
pub const ONE_OK_155: bool = true;
pub const F155_IS_ZERO: bool = true;
pub const W156: usize = 130;
pub const ONE_OK_156: bool = true;
pub const F156_IS_ZERO: bool = true;
pub const W157: usize = 135;
pub const ONE_OK_157: bool = true;
pub const F157_IS_ZERO: bool = true;
pub const W158: usize = 146;
pub const ONE_OK_158: bool = true;
pub const F158_IS_ZERO: bool = true;
pub const W159: usize = 147;
pub const ONE_OK_159: bool = true;
pub const F159_IS_ZERO: bool = true;
pub const W160: usize = 183;
pub const ONE_OK_160: bool = true;
pub const F160_IS_ZERO: bool = true;
pub const W161: usize = 33;
pub const ONE_OK_161: bool = false;
pub const F161_IS_ZERO: bool = false;
pub const W162: usize = 129;
pub const ONE_OK_162: bool = true;
pub const F162_IS_ZERO: bool = true;
pub const W163: usize = 131;
pub const ONE_OK_163: bool = true;
pub const F163_IS_ZERO: bool = true;
pub const W164: usize = 137;
pub const ONE_OK_164: bool = true;
pub const F164_IS_ZERO: bool = true;
pub const W165: usize = 142;
pub const ONE_OK_165: bool = true;
pub const F165_IS_ZERO: bool = true;
pub const W166: usize = 153;
pub const ONE_OK_166: bool = true;
pub const F166_IS_ZERO: bool = true;
pub const W167: usize = 154;
pub const ONE_OK_167: bool = true;
pub const F167_IS_ZERO: bool = true;
pub const W168: usize = 190;
pub const ONE_OK_168: bool = true;
pub const F168_IS_ZERO: bool = true;
pub const W169: usize = 34;
pub const ONE_OK_169: bool = false;
pub const F169_IS_ZERO: bool = false;
pub const W170: usize = 136;
pub const ONE_OK_170: bool = true;
pub const F170_IS_ZERO: bool = true;
pub const W171: usize = 138;
pub const ONE_OK_171: bool = true;
pub const F171_IS_ZERO: bool = true;
pub const W172: usize = 144;
pub const ONE_OK_172: bool = true;
pub const F172_IS_ZERO: bool = true;
pub const W173: usize = 149;
pub const ONE_OK_173: bool = true;
pub const F173_IS_ZERO: bool = true;
pub const W174: usize = 160;
pub const ONE_OK_174: bool = true;
pub const F174_IS_ZERO: bool = true;
pub const W175: usize = 161;
pub const ONE_OK_175: bool = true;
pub const F175_IS_ZERO: bool = true;
pub const W176: usize = 197;
pub const ONE_OK_176: bool = true;
pub const F176_IS_ZERO: bool = true;
pub const W177: usize = 35;
pub const ONE_OK_177: bool = false;
pub const F177_IS_ZERO: bool = false;
pub const W178: usize = 143;
pub const ONE_OK_178: bool = true;
pub const F178_IS_ZERO: bool = true;
pub const W179: usize = 145;
pub const ONE_OK_179: bool = true;
pub const F179_IS_ZERO: bool = true;
pub const W180: usize = 151;
pub const ONE_OK_180: bool = true;
pub const F180_IS_ZERO: bool = true;
pub const W181: usize = 156;
pub const ONE_OK_181: bool = true;
pub const F181_IS_ZERO: bool = true;
pub const W182: usize = 167;
pub const ONE_OK_182: bool = true;
pub const F182_IS_ZERO: bool = true;
pub const W183: usize = 168;
pub const ONE_OK_183: bool = true;
pub const F183_IS_ZERO: bool = true;
pub const W184: usize = 204;
pub const ONE_OK_184: bool = true;
pub const F184_IS_ZERO: bool = true;
pub const W185: usize = 36;
pub const ONE_OK_185: bool = false;
pub const F185_IS_ZERO: bool = false;
pub const W186: usize = 150;
pub const ONE_OK_186: bool = true;
pub const F186_IS_ZERO: bool = true;
pub const W187: usize = 152;
pub const ONE_OK_187: bool = true;
pub const F187_IS_ZERO: bool = true;
pub const W188: usize = 158;
pub const ONE_OK_188: bool = true;
pub const F188_IS_ZERO: bool = true;
pub const W189: usize = 163;
pub const ONE_OK_189: bool = true;
pub const F189_IS_ZERO: bool = true;
pub const W190: usize = 174;
pub const ONE_OK_190: bool = true;
pub const F190_IS_ZERO: bool = true;
pub const W191: usize = 175;
pub const ONE_OK_191: bool = true;
pub const F191_IS_ZERO: bool = true;
pub const W192: usize = 211;
pub const ONE_OK_192: bool = true;
pub const F192_IS_ZERO: bool = true;
pub const W193: usize = 37;
pub const ONE_OK_193: bool = false;
pub const F193_IS_ZERO: bool = false;
pub const W194: usize = 157;
pub const ONE_OK_194: bool = true;
pub const F194_IS_ZERO: bool = true;
pub const W195: usize = 159;
pub const ONE_OK_195: bool = true;
pub const F195_IS_ZERO: bool = true;
pub const W196: usize = 165;
pub const ONE_OK_196: bool = true;
pub const F196_IS_ZERO: bool = true;
pub const W197: usize = 170;
pub const ONE_OK_197: bool = true;
pub const F197_IS_ZERO: bool = true;
pub const W198: usize = 181;
pub const ONE_OK_198: bool = true;
pub const F198_IS_ZERO: bool = true;
pub const W199: usize = 182;
pub const ONE_OK_199: bool = true;
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
pub type C100 = Slot<111>;
pub fn build100() -> [u32; 111] {
    [0; 111]
}
pub type C101 = Slot<113>;
pub fn build101() -> [u32; 113] {
    [0; 113]
}
pub type C102 = Slot<114>;
pub fn build102() -> [u32; 114] {
    [0; 114]
}
pub type C103 = Slot<117>;
pub fn build103() -> [u32; 117] {
    [0; 117]
}
pub type C104 = Slot<118>;
pub fn build104() -> [u32; 118] {
    [0; 118]
}
pub type C105 = Slot<123>;
pub fn build105() -> [u32; 123] {
    [0; 123]
}
pub type C106 = Slot<126>;
pub fn build106() -> [u32; 126] {
    [0; 126]
}
pub type C107 = Slot<138>;
pub fn build107() -> [u32; 138] {
    [0; 138]
}
pub type C108 = Slot<142>;
pub fn build108() -> [u32; 142] {
    [0; 142]
}
pub type C109 = Slot<174>;
pub fn build109() -> [u32; 174] {
    [0; 174]
}
pub type C110 = Slot<122>;
pub fn build110() -> [u32; 122] {
    [0; 122]
}
pub type C111 = Slot<124>;
pub fn build111() -> [u32; 124] {
    [0; 124]
}
pub type C112 = Slot<125>;
pub fn build112() -> [u32; 125] {
    [0; 125]
}
pub type C113 = Slot<128>;
pub fn build113() -> [u32; 128] {
    [0; 128]
}
pub type C114 = Slot<129>;
pub fn build114() -> [u32; 129] {
    [0; 129]
}
pub type C115 = Slot<134>;
pub fn build115() -> [u32; 134] {
    [0; 134]
}
pub type C116 = Slot<137>;
pub fn build116() -> [u32; 137] {
    [0; 137]
}
pub type C117 = Slot<149>;
pub fn build117() -> [u32; 149] {
    [0; 149]
}
pub type C118 = Slot<153>;
pub fn build118() -> [u32; 153] {
    [0; 153]
}
pub type C119 = Slot<185>;
pub fn build119() -> [u32; 185] {
    [0; 185]
}
pub type C120 = Slot<133>;
pub fn build120() -> [u32; 133] {
    [0; 133]
}
pub type C121 = Slot<135>;
pub fn build121() -> [u32; 135] {
    [0; 135]
}
pub type C122 = Slot<136>;
pub fn build122() -> [u32; 136] {
    [0; 136]
}
pub type C123 = Slot<139>;
pub fn build123() -> [u32; 139] {
    [0; 139]
}
pub type C124 = Slot<140>;
pub fn build124() -> [u32; 140] {
    [0; 140]
}
pub type C125 = Slot<145>;
pub fn build125() -> [u32; 145] {
    [0; 145]
}
pub type C126 = Slot<148>;
pub fn build126() -> [u32; 148] {
    [0; 148]
}
pub type C127 = Slot<160>;
pub fn build127() -> [u32; 160] {
    [0; 160]
}
pub type C128 = Slot<164>;
pub fn build128() -> [u32; 164] {
    [0; 164]
}
pub type C129 = Slot<196>;
pub fn build129() -> [u32; 196] {
    [0; 196]
}
pub type C130 = Slot<144>;
pub fn build130() -> [u32; 144] {
    [0; 144]
}
pub type C131 = Slot<146>;
pub fn build131() -> [u32; 146] {
    [0; 146]
}
pub type C132 = Slot<147>;
pub fn build132() -> [u32; 147] {
    [0; 147]
}
pub type C133 = Slot<150>;
pub fn build133() -> [u32; 150] {
    [0; 150]
}
pub type C134 = Slot<151>;
pub fn build134() -> [u32; 151] {
    [0; 151]
}
pub type C135 = Slot<156>;
pub fn build135() -> [u32; 156] {
    [0; 156]
}
pub type C136 = Slot<159>;
pub fn build136() -> [u32; 159] {
    [0; 159]
}
pub type C137 = Slot<171>;
pub fn build137() -> [u32; 171] {
    [0; 171]
}
pub type C138 = Slot<175>;
pub fn build138() -> [u32; 175] {
    [0; 175]
}
pub type C139 = Slot<207>;
pub fn build139() -> [u32; 207] {
    [0; 207]
}
pub type C140 = Slot<155>;
pub fn build140() -> [u32; 155] {
    [0; 155]
}
pub type C141 = Slot<157>;
pub fn build141() -> [u32; 157] {
    [0; 157]
}
pub type C142 = Slot<158>;
pub fn build142() -> [u32; 158] {
    [0; 158]
}
pub type C143 = Slot<161>;
pub fn build143() -> [u32; 161] {
    [0; 161]
}
pub type C144 = Slot<162>;
pub fn build144() -> [u32; 162] {
    [0; 162]
}
pub type C145 = Slot<167>;
pub fn build145() -> [u32; 167] {
    [0; 167]
}
pub type C146 = Slot<170>;
pub fn build146() -> [u32; 170] {
    [0; 170]
}
pub type C147 = Slot<182>;
pub fn build147() -> [u32; 182] {
    [0; 182]
}
pub type C148 = Slot<186>;
pub fn build148() -> [u32; 186] {
    [0; 186]
}
pub type C149 = Slot<218>;
pub fn build149() -> [u32; 218] {
    [0; 218]
}
pub type C150 = Slot<166>;
pub fn build150() -> [u32; 166] {
    [0; 166]
}
pub type C151 = Slot<168>;
pub fn build151() -> [u32; 168] {
    [0; 168]
}
pub type C152 = Slot<169>;
pub fn build152() -> [u32; 169] {
    [0; 169]
}
pub type C153 = Slot<172>;
pub fn build153() -> [u32; 172] {
    [0; 172]
}
pub type C154 = Slot<173>;
pub fn build154() -> [u32; 173] {
    [0; 173]
}
pub type C155 = Slot<178>;
pub fn build155() -> [u32; 178] {
    [0; 178]
}
pub type C156 = Slot<181>;
pub fn build156() -> [u32; 181] {
    [0; 181]
}
pub type C157 = Slot<193>;
pub fn build157() -> [u32; 193] {
    [0; 193]
}
pub type C158 = Slot<197>;
pub fn build158() -> [u32; 197] {
    [0; 197]
}
pub type C159 = Slot<229>;
pub fn build159() -> [u32; 229] {
    [0; 229]
}
pub type C160 = Slot<177>;
pub fn build160() -> [u32; 177] {
    [0; 177]
}
pub type C161 = Slot<179>;
pub fn build161() -> [u32; 179] {
    [0; 179]
}
pub type C162 = Slot<180>;
pub fn build162() -> [u32; 180] {
    [0; 180]
}
pub type C163 = Slot<183>;
pub fn build163() -> [u32; 183] {
    [0; 183]
}
pub type C164 = Slot<184>;
pub fn build164() -> [u32; 184] {
    [0; 184]
}
pub type C165 = Slot<189>;
pub fn build165() -> [u32; 189] {
    [0; 189]
}
pub type C166 = Slot<192>;
pub fn build166() -> [u32; 192] {
    [0; 192]
}
pub type C167 = Slot<204>;
pub fn build167() -> [u32; 204] {
    [0; 204]
}
pub type C168 = Slot<208>;
pub fn build168() -> [u32; 208] {
    [0; 208]
}
pub type C169 = Slot<240>;
pub fn build169() -> [u32; 240] {
    [0; 240]
}
pub type C170 = Slot<188>;
pub fn build170() -> [u32; 188] {
    [0; 188]
}
pub type C171 = Slot<190>;
pub fn build171() -> [u32; 190] {
    [0; 190]
}
pub type C172 = Slot<191>;
pub fn build172() -> [u32; 191] {
    [0; 191]
}
pub type C173 = Slot<194>;
pub fn build173() -> [u32; 194] {
    [0; 194]
}
pub type C174 = Slot<195>;
pub fn build174() -> [u32; 195] {
    [0; 195]
}
pub type C175 = Slot<200>;
pub fn build175() -> [u32; 200] {
    [0; 200]
}
pub type C176 = Slot<203>;
pub fn build176() -> [u32; 203] {
    [0; 203]
}
pub type C177 = Slot<215>;
pub fn build177() -> [u32; 215] {
    [0; 215]
}
pub type C178 = Slot<219>;
pub fn build178() -> [u32; 219] {
    [0; 219]
}
pub type C179 = Slot<251>;
pub fn build179() -> [u32; 251] {
    [0; 251]
}
pub type C180 = Slot<199>;
pub fn build180() -> [u32; 199] {
    [0; 199]
}
pub type C181 = Slot<201>;
pub fn build181() -> [u32; 201] {
    [0; 201]
}
pub type C182 = Slot<202>;
pub fn build182() -> [u32; 202] {
    [0; 202]
}
pub type C183 = Slot<205>;
pub fn build183() -> [u32; 205] {
    [0; 205]
}
pub type C184 = Slot<206>;
pub fn build184() -> [u32; 206] {
    [0; 206]
}
pub type C185 = Slot<211>;
pub fn build185() -> [u32; 211] {
    [0; 211]
}
pub type C186 = Slot<214>;
pub fn build186() -> [u32; 214] {
    [0; 214]
}
pub type C187 = Slot<226>;
pub fn build187() -> [u32; 226] {
    [0; 226]
}
pub type C188 = Slot<230>;
pub fn build188() -> [u32; 230] {
    [0; 230]
}
pub type C189 = Slot<262>;
pub fn build189() -> [u32; 262] {
    [0; 262]
}
pub type C190 = Slot<210>;
pub fn build190() -> [u32; 210] {
    [0; 210]
}
pub type C191 = Slot<212>;
pub fn build191() -> [u32; 212] {
    [0; 212]
}
pub type C192 = Slot<213>;
pub fn build192() -> [u32; 213] {
    [0; 213]
}
pub type C193 = Slot<216>;
pub fn build193() -> [u32; 216] {
    [0; 216]
}
pub type C194 = Slot<217>;
pub fn build194() -> [u32; 217] {
    [0; 217]
}
pub type C195 = Slot<222>;
pub fn build195() -> [u32; 222] {
    [0; 222]
}
pub type C196 = Slot<225>;
pub fn build196() -> [u32; 225] {
    [0; 225]
}
pub type C197 = Slot<237>;
pub fn build197() -> [u32; 237] {
    [0; 237]
}
pub type C198 = Slot<241>;
pub fn build198() -> [u32; 241] {
    [0; 241]
}
pub type C199 = Slot<273>;
pub fn build199() -> [u32; 273] {
    [0; 273]
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
pub fn call100() -> u32 {
    scaled_fold::<C100>(100, W100)
}
pub fn call101() -> u32 {
    scaled_fold::<C101>(101, W101)
}
pub fn call102() -> u32 {
    scaled_fold::<C102>(102, W102)
}
pub fn call103() -> u32 {
    scaled_fold::<C103>(103, W103)
}
pub fn call104() -> u32 {
    scaled_fold::<C104>(104, W104)
}
pub fn call106() -> u32 {
    scaled_fold::<C106>(106, W106)
}
pub fn call107() -> u32 {
    scaled_fold::<C107>(107, W107)
}
pub fn call108() -> u32 {
    scaled_fold::<C108>(108, W108)
}
pub fn call109() -> u32 {
    scaled_fold::<C109>(109, W109)
}
pub fn call110() -> u32 {
    scaled_fold::<C110>(110, W110)
}
pub fn call111() -> u32 {
    scaled_fold::<C111>(111, W111)
}
pub fn call112() -> u32 {
    scaled_fold::<C112>(112, W112)
}
pub fn call114() -> u32 {
    scaled_fold::<C114>(114, W114)
}
pub fn call115() -> u32 {
    scaled_fold::<C115>(115, W115)
}
pub fn call116() -> u32 {
    scaled_fold::<C116>(116, W116)
}
pub fn call117() -> u32 {
    scaled_fold::<C117>(117, W117)
}
pub fn call118() -> u32 {
    scaled_fold::<C118>(118, W118)
}
pub fn call119() -> u32 {
    scaled_fold::<C119>(119, W119)
}
pub fn call120() -> u32 {
    scaled_fold::<C120>(120, W120)
}
pub fn call122() -> u32 {
    scaled_fold::<C122>(122, W122)
}
pub fn call123() -> u32 {
    scaled_fold::<C123>(123, W123)
}
pub fn call124() -> u32 {
    scaled_fold::<C124>(124, W124)
}
pub fn call125() -> u32 {
    scaled_fold::<C125>(125, W125)
}
pub fn call126() -> u32 {
    scaled_fold::<C126>(126, W126)
}
pub fn call127() -> u32 {
    scaled_fold::<C127>(127, W127)
}
pub fn call128() -> u32 {
    scaled_fold::<C128>(128, W128)
}
pub fn call130() -> u32 {
    scaled_fold::<C130>(130, W130)
}
pub fn call131() -> u32 {
    scaled_fold::<C131>(131, W131)
}
pub fn call132() -> u32 {
    scaled_fold::<C132>(132, W132)
}
pub fn call133() -> u32 {
    scaled_fold::<C133>(133, W133)
}
pub fn call134() -> u32 {
    scaled_fold::<C134>(134, W134)
}
pub fn call135() -> u32 {
    scaled_fold::<C135>(135, W135)
}
pub fn call136() -> u32 {
    scaled_fold::<C136>(136, W136)
}
pub fn call138() -> u32 {
    scaled_fold::<C138>(138, W138)
}
pub fn call139() -> u32 {
    scaled_fold::<C139>(139, W139)
}
pub fn call140() -> u32 {
    scaled_fold::<C140>(140, W140)
}
pub fn call141() -> u32 {
    scaled_fold::<C141>(141, W141)
}
pub fn call142() -> u32 {
    scaled_fold::<C142>(142, W142)
}
pub fn call143() -> u32 {
    scaled_fold::<C143>(143, W143)
}
pub fn call144() -> u32 {
    scaled_fold::<C144>(144, W144)
}
pub fn call146() -> u32 {
    scaled_fold::<C146>(146, W146)
}
pub fn call147() -> u32 {
    scaled_fold::<C147>(147, W147)
}
pub fn call148() -> u32 {
    scaled_fold::<C148>(148, W148)
}
pub fn call149() -> u32 {
    scaled_fold::<C149>(149, W149)
}
pub fn call150() -> u32 {
    scaled_fold::<C150>(150, W150)
}
pub fn call151() -> u32 {
    scaled_fold::<C151>(151, W151)
}
pub fn call152() -> u32 {
    scaled_fold::<C152>(152, W152)
}
pub fn call154() -> u32 {
    scaled_fold::<C154>(154, W154)
}
pub fn call155() -> u32 {
    scaled_fold::<C155>(155, W155)
}
pub fn call156() -> u32 {
    scaled_fold::<C156>(156, W156)
}
pub fn call157() -> u32 {
    scaled_fold::<C157>(157, W157)
}
pub fn call158() -> u32 {
    scaled_fold::<C158>(158, W158)
}
pub fn call159() -> u32 {
    scaled_fold::<C159>(159, W159)
}
pub fn call160() -> u32 {
    scaled_fold::<C160>(160, W160)
}
pub fn call162() -> u32 {
    scaled_fold::<C162>(162, W162)
}
pub fn call163() -> u32 {
    scaled_fold::<C163>(163, W163)
}
pub fn call164() -> u32 {
    scaled_fold::<C164>(164, W164)
}
pub fn call165() -> u32 {
    scaled_fold::<C165>(165, W165)
}
pub fn call166() -> u32 {
    scaled_fold::<C166>(166, W166)
}
pub fn call167() -> u32 {
    scaled_fold::<C167>(167, W167)
}
pub fn call168() -> u32 {
    scaled_fold::<C168>(168, W168)
}
pub fn call170() -> u32 {
    scaled_fold::<C170>(170, W170)
}
pub fn call171() -> u32 {
    scaled_fold::<C171>(171, W171)
}
pub fn call172() -> u32 {
    scaled_fold::<C172>(172, W172)
}
pub fn call173() -> u32 {
    scaled_fold::<C173>(173, W173)
}
pub fn call174() -> u32 {
    scaled_fold::<C174>(174, W174)
}
pub fn call175() -> u32 {
    scaled_fold::<C175>(175, W175)
}
pub fn call176() -> u32 {
    scaled_fold::<C176>(176, W176)
}
pub fn call178() -> u32 {
    scaled_fold::<C178>(178, W178)
}
pub fn call179() -> u32 {
    scaled_fold::<C179>(179, W179)
}
pub fn call180() -> u32 {
    scaled_fold::<C180>(180, W180)
}
pub fn call181() -> u32 {
    scaled_fold::<C181>(181, W181)
}
pub fn call182() -> u32 {
    scaled_fold::<C182>(182, W182)
}
pub fn call183() -> u32 {
    scaled_fold::<C183>(183, W183)
}
pub fn call184() -> u32 {
    scaled_fold::<C184>(184, W184)
}
pub fn call186() -> u32 {
    scaled_fold::<C186>(186, W186)
}
pub fn call187() -> u32 {
    scaled_fold::<C187>(187, W187)
}
pub fn call188() -> u32 {
    scaled_fold::<C188>(188, W188)
}
pub fn call189() -> u32 {
    scaled_fold::<C189>(189, W189)
}
pub fn call190() -> u32 {
    scaled_fold::<C190>(190, W190)
}
pub fn call191() -> u32 {
    scaled_fold::<C191>(191, W191)
}
pub fn call192() -> u32 {
    scaled_fold::<C192>(192, W192)
}
pub fn call194() -> u32 {
    scaled_fold::<C194>(194, W194)
}
pub fn call195() -> u32 {
    scaled_fold::<C195>(195, W195)
}
pub fn call196() -> u32 {
    scaled_fold::<C196>(196, W196)
}
pub fn call197() -> u32 {
    scaled_fold::<C197>(197, W197)
}
pub fn call198() -> u32 {
    scaled_fold::<C198>(198, W198)
}
pub fn call199() -> u32 {
    scaled_fold::<C199>(199, W199)
}
