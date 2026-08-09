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
pub const W200: usize = 218;
pub const ONE_OK_200: bool = true;
pub const F200_IS_ZERO: bool = true;
pub const W201: usize = 38;
pub const ONE_OK_201: bool = false;
pub const F201_IS_ZERO: bool = false;
pub const W202: usize = 164;
pub const ONE_OK_202: bool = true;
pub const F202_IS_ZERO: bool = true;
pub const W203: usize = 166;
pub const ONE_OK_203: bool = true;
pub const F203_IS_ZERO: bool = true;
pub const W204: usize = 172;
pub const ONE_OK_204: bool = true;
pub const F204_IS_ZERO: bool = true;
pub const W205: usize = 177;
pub const ONE_OK_205: bool = true;
pub const F205_IS_ZERO: bool = true;
pub const W206: usize = 188;
pub const ONE_OK_206: bool = true;
pub const F206_IS_ZERO: bool = true;
pub const W207: usize = 189;
pub const ONE_OK_207: bool = true;
pub const F207_IS_ZERO: bool = true;
pub const W208: usize = 225;
pub const ONE_OK_208: bool = true;
pub const F208_IS_ZERO: bool = true;
pub const W209: usize = 39;
pub const ONE_OK_209: bool = false;
pub const F209_IS_ZERO: bool = false;
pub const W210: usize = 171;
pub const ONE_OK_210: bool = true;
pub const F210_IS_ZERO: bool = true;
pub const W211: usize = 173;
pub const ONE_OK_211: bool = true;
pub const F211_IS_ZERO: bool = true;
pub const W212: usize = 179;
pub const ONE_OK_212: bool = true;
pub const F212_IS_ZERO: bool = true;
pub const W213: usize = 184;
pub const ONE_OK_213: bool = true;
pub const F213_IS_ZERO: bool = true;
pub const W214: usize = 195;
pub const ONE_OK_214: bool = true;
pub const F214_IS_ZERO: bool = true;
pub const W215: usize = 196;
pub const ONE_OK_215: bool = true;
pub const F215_IS_ZERO: bool = true;
pub const W216: usize = 232;
pub const ONE_OK_216: bool = true;
pub const F216_IS_ZERO: bool = true;
pub const W217: usize = 40;
pub const ONE_OK_217: bool = false;
pub const F217_IS_ZERO: bool = false;
pub const W218: usize = 178;
pub const ONE_OK_218: bool = true;
pub const F218_IS_ZERO: bool = true;
pub const W219: usize = 180;
pub const ONE_OK_219: bool = true;
pub const F219_IS_ZERO: bool = true;
pub const W220: usize = 186;
pub const ONE_OK_220: bool = true;
pub const F220_IS_ZERO: bool = true;
pub const W221: usize = 191;
pub const ONE_OK_221: bool = true;
pub const F221_IS_ZERO: bool = true;
pub const W222: usize = 202;
pub const ONE_OK_222: bool = true;
pub const F222_IS_ZERO: bool = true;
pub const W223: usize = 203;
pub const ONE_OK_223: bool = true;
pub const F223_IS_ZERO: bool = true;
pub const W224: usize = 239;
pub const ONE_OK_224: bool = true;
pub const F224_IS_ZERO: bool = true;
pub const W225: usize = 41;
pub const ONE_OK_225: bool = false;
pub const F225_IS_ZERO: bool = false;
pub const W226: usize = 185;
pub const ONE_OK_226: bool = true;
pub const F226_IS_ZERO: bool = true;
pub const W227: usize = 187;
pub const ONE_OK_227: bool = true;
pub const F227_IS_ZERO: bool = true;
pub const W228: usize = 193;
pub const ONE_OK_228: bool = true;
pub const F228_IS_ZERO: bool = true;
pub const W229: usize = 198;
pub const ONE_OK_229: bool = true;
pub const F229_IS_ZERO: bool = true;
pub const W230: usize = 209;
pub const ONE_OK_230: bool = true;
pub const F230_IS_ZERO: bool = true;
pub const W231: usize = 210;
pub const ONE_OK_231: bool = true;
pub const F231_IS_ZERO: bool = true;
pub const W232: usize = 246;
pub const ONE_OK_232: bool = true;
pub const F232_IS_ZERO: bool = true;
pub const W233: usize = 42;
pub const ONE_OK_233: bool = false;
pub const F233_IS_ZERO: bool = false;
pub const W234: usize = 192;
pub const ONE_OK_234: bool = true;
pub const F234_IS_ZERO: bool = true;
pub const W235: usize = 194;
pub const ONE_OK_235: bool = true;
pub const F235_IS_ZERO: bool = true;
pub const W236: usize = 200;
pub const ONE_OK_236: bool = true;
pub const F236_IS_ZERO: bool = true;
pub const W237: usize = 205;
pub const ONE_OK_237: bool = true;
pub const F237_IS_ZERO: bool = true;
pub const W238: usize = 216;
pub const ONE_OK_238: bool = true;
pub const F238_IS_ZERO: bool = true;
pub const W239: usize = 217;
pub const ONE_OK_239: bool = true;
pub const F239_IS_ZERO: bool = true;
pub const W240: usize = 253;
pub const ONE_OK_240: bool = true;
pub const F240_IS_ZERO: bool = true;
pub const W241: usize = 43;
pub const ONE_OK_241: bool = false;
pub const F241_IS_ZERO: bool = false;
pub const W242: usize = 199;
pub const ONE_OK_242: bool = true;
pub const F242_IS_ZERO: bool = true;
pub const W243: usize = 201;
pub const ONE_OK_243: bool = true;
pub const F243_IS_ZERO: bool = true;
pub const W244: usize = 207;
pub const ONE_OK_244: bool = true;
pub const F244_IS_ZERO: bool = true;
pub const W245: usize = 212;
pub const ONE_OK_245: bool = true;
pub const F245_IS_ZERO: bool = true;
pub const W246: usize = 223;
pub const ONE_OK_246: bool = true;
pub const F246_IS_ZERO: bool = true;
pub const W247: usize = 224;
pub const ONE_OK_247: bool = true;
pub const F247_IS_ZERO: bool = true;
pub const W248: usize = 260;
pub const ONE_OK_248: bool = true;
pub const F248_IS_ZERO: bool = true;
pub const W249: usize = 44;
pub const ONE_OK_249: bool = false;
pub const F249_IS_ZERO: bool = false;
pub const W250: usize = 206;
pub const ONE_OK_250: bool = true;
pub const F250_IS_ZERO: bool = true;
pub const W251: usize = 208;
pub const ONE_OK_251: bool = true;
pub const F251_IS_ZERO: bool = true;
pub const W252: usize = 214;
pub const ONE_OK_252: bool = true;
pub const F252_IS_ZERO: bool = true;
pub const W253: usize = 219;
pub const ONE_OK_253: bool = true;
pub const F253_IS_ZERO: bool = true;
pub const W254: usize = 230;
pub const ONE_OK_254: bool = true;
pub const F254_IS_ZERO: bool = true;
pub const W255: usize = 231;
pub const ONE_OK_255: bool = true;
pub const F255_IS_ZERO: bool = true;
pub const W256: usize = 267;
pub const ONE_OK_256: bool = true;
pub const F256_IS_ZERO: bool = true;
pub const W257: usize = 45;
pub const ONE_OK_257: bool = false;
pub const F257_IS_ZERO: bool = false;
pub const W258: usize = 213;
pub const ONE_OK_258: bool = true;
pub const F258_IS_ZERO: bool = true;
pub const W259: usize = 215;
pub const ONE_OK_259: bool = true;
pub const F259_IS_ZERO: bool = true;
pub const W260: usize = 221;
pub const ONE_OK_260: bool = true;
pub const F260_IS_ZERO: bool = true;
pub const W261: usize = 226;
pub const ONE_OK_261: bool = true;
pub const F261_IS_ZERO: bool = true;
pub const W262: usize = 237;
pub const ONE_OK_262: bool = true;
pub const F262_IS_ZERO: bool = true;
pub const W263: usize = 238;
pub const ONE_OK_263: bool = true;
pub const F263_IS_ZERO: bool = true;
pub const W264: usize = 274;
pub const ONE_OK_264: bool = true;
pub const F264_IS_ZERO: bool = true;
pub const W265: usize = 46;
pub const ONE_OK_265: bool = false;
pub const F265_IS_ZERO: bool = false;
pub const W266: usize = 220;
pub const ONE_OK_266: bool = true;
pub const F266_IS_ZERO: bool = true;
pub const W267: usize = 222;
pub const ONE_OK_267: bool = true;
pub const F267_IS_ZERO: bool = true;
pub const W268: usize = 228;
pub const ONE_OK_268: bool = true;
pub const F268_IS_ZERO: bool = true;
pub const W269: usize = 233;
pub const ONE_OK_269: bool = true;
pub const F269_IS_ZERO: bool = true;
pub const W270: usize = 244;
pub const ONE_OK_270: bool = true;
pub const F270_IS_ZERO: bool = true;
pub const W271: usize = 245;
pub const ONE_OK_271: bool = true;
pub const F271_IS_ZERO: bool = true;
pub const W272: usize = 281;
pub const ONE_OK_272: bool = true;
pub const F272_IS_ZERO: bool = true;
pub const W273: usize = 47;
pub const ONE_OK_273: bool = false;
pub const F273_IS_ZERO: bool = false;
pub const W274: usize = 227;
pub const ONE_OK_274: bool = true;
pub const F274_IS_ZERO: bool = true;
pub const W275: usize = 229;
pub const ONE_OK_275: bool = true;
pub const F275_IS_ZERO: bool = true;
pub const W276: usize = 235;
pub const ONE_OK_276: bool = true;
pub const F276_IS_ZERO: bool = true;
pub const W277: usize = 240;
pub const ONE_OK_277: bool = true;
pub const F277_IS_ZERO: bool = true;
pub const W278: usize = 251;
pub const ONE_OK_278: bool = true;
pub const F278_IS_ZERO: bool = true;
pub const W279: usize = 252;
pub const ONE_OK_279: bool = true;
pub const F279_IS_ZERO: bool = true;
pub const W280: usize = 288;
pub const ONE_OK_280: bool = true;
pub const F280_IS_ZERO: bool = true;
pub const W281: usize = 48;
pub const ONE_OK_281: bool = false;
pub const F281_IS_ZERO: bool = false;
pub const W282: usize = 234;
pub const ONE_OK_282: bool = true;
pub const F282_IS_ZERO: bool = true;
pub const W283: usize = 236;
pub const ONE_OK_283: bool = true;
pub const F283_IS_ZERO: bool = true;
pub const W284: usize = 242;
pub const ONE_OK_284: bool = true;
pub const F284_IS_ZERO: bool = true;
pub const W285: usize = 247;
pub const ONE_OK_285: bool = true;
pub const F285_IS_ZERO: bool = true;
pub const W286: usize = 258;
pub const ONE_OK_286: bool = true;
pub const F286_IS_ZERO: bool = true;
pub const W287: usize = 259;
pub const ONE_OK_287: bool = true;
pub const F287_IS_ZERO: bool = true;
pub const W288: usize = 295;
pub const ONE_OK_288: bool = true;
pub const F288_IS_ZERO: bool = true;
pub const W289: usize = 49;
pub const ONE_OK_289: bool = false;
pub const F289_IS_ZERO: bool = false;
pub const W290: usize = 241;
pub const ONE_OK_290: bool = true;
pub const F290_IS_ZERO: bool = true;
pub const W291: usize = 243;
pub const ONE_OK_291: bool = true;
pub const F291_IS_ZERO: bool = true;
pub const W292: usize = 249;
pub const ONE_OK_292: bool = true;
pub const F292_IS_ZERO: bool = true;
pub const W293: usize = 254;
pub const ONE_OK_293: bool = true;
pub const F293_IS_ZERO: bool = true;
pub const W294: usize = 265;
pub const ONE_OK_294: bool = true;
pub const F294_IS_ZERO: bool = true;
pub const W295: usize = 266;
pub const ONE_OK_295: bool = true;
pub const F295_IS_ZERO: bool = true;
pub const W296: usize = 302;
pub const ONE_OK_296: bool = true;
pub const F296_IS_ZERO: bool = true;
pub const W297: usize = 50;
pub const ONE_OK_297: bool = false;
pub const F297_IS_ZERO: bool = false;
pub const W298: usize = 248;
pub const ONE_OK_298: bool = true;
pub const F298_IS_ZERO: bool = true;
pub const W299: usize = 250;
pub const ONE_OK_299: bool = true;
pub const F299_IS_ZERO: bool = true;
pub const W300: usize = 256;
pub const ONE_OK_300: bool = true;
pub const F300_IS_ZERO: bool = true;
pub const W301: usize = 261;
pub const ONE_OK_301: bool = true;
pub const F301_IS_ZERO: bool = true;
pub const W302: usize = 272;
pub const ONE_OK_302: bool = true;
pub const F302_IS_ZERO: bool = true;
pub const W303: usize = 273;
pub const ONE_OK_303: bool = true;
pub const F303_IS_ZERO: bool = true;
pub const W304: usize = 309;
pub const ONE_OK_304: bool = true;
pub const F304_IS_ZERO: bool = true;
pub const W305: usize = 51;
pub const ONE_OK_305: bool = false;
pub const F305_IS_ZERO: bool = false;
pub const W306: usize = 255;
pub const ONE_OK_306: bool = true;
pub const F306_IS_ZERO: bool = true;
pub const W307: usize = 257;
pub const ONE_OK_307: bool = true;
pub const F307_IS_ZERO: bool = true;
pub const W308: usize = 263;
pub const ONE_OK_308: bool = true;
pub const F308_IS_ZERO: bool = true;
pub const W309: usize = 268;
pub const ONE_OK_309: bool = true;
pub const F309_IS_ZERO: bool = true;
pub const W310: usize = 279;
pub const ONE_OK_310: bool = true;
pub const F310_IS_ZERO: bool = true;
pub const W311: usize = 280;
pub const ONE_OK_311: bool = true;
pub const F311_IS_ZERO: bool = true;
pub const W312: usize = 316;
pub const ONE_OK_312: bool = true;
pub const F312_IS_ZERO: bool = true;
pub const W313: usize = 52;
pub const ONE_OK_313: bool = false;
pub const F313_IS_ZERO: bool = false;
pub const W314: usize = 262;
pub const ONE_OK_314: bool = true;
pub const F314_IS_ZERO: bool = true;
pub const W315: usize = 264;
pub const ONE_OK_315: bool = true;
pub const F315_IS_ZERO: bool = true;
pub const W316: usize = 270;
pub const ONE_OK_316: bool = true;
pub const F316_IS_ZERO: bool = true;
pub const W317: usize = 275;
pub const ONE_OK_317: bool = true;
pub const F317_IS_ZERO: bool = true;
pub const W318: usize = 286;
pub const ONE_OK_318: bool = true;
pub const F318_IS_ZERO: bool = true;
pub const W319: usize = 287;
pub const ONE_OK_319: bool = true;
pub const F319_IS_ZERO: bool = true;
pub const W320: usize = 323;
pub const ONE_OK_320: bool = true;
pub const F320_IS_ZERO: bool = true;
pub const W321: usize = 53;
pub const ONE_OK_321: bool = false;
pub const F321_IS_ZERO: bool = false;
pub const W322: usize = 269;
pub const ONE_OK_322: bool = true;
pub const F322_IS_ZERO: bool = true;
pub const W323: usize = 271;
pub const ONE_OK_323: bool = true;
pub const F323_IS_ZERO: bool = true;
pub const W324: usize = 277;
pub const ONE_OK_324: bool = true;
pub const F324_IS_ZERO: bool = true;
pub const W325: usize = 282;
pub const ONE_OK_325: bool = true;
pub const F325_IS_ZERO: bool = true;
pub const W326: usize = 293;
pub const ONE_OK_326: bool = true;
pub const F326_IS_ZERO: bool = true;
pub const W327: usize = 294;
pub const ONE_OK_327: bool = true;
pub const F327_IS_ZERO: bool = true;
pub const W328: usize = 330;
pub const ONE_OK_328: bool = true;
pub const F328_IS_ZERO: bool = true;
pub const W329: usize = 54;
pub const ONE_OK_329: bool = false;
pub const F329_IS_ZERO: bool = false;
pub const W330: usize = 276;
pub const ONE_OK_330: bool = true;
pub const F330_IS_ZERO: bool = true;
pub const W331: usize = 278;
pub const ONE_OK_331: bool = true;
pub const F331_IS_ZERO: bool = true;
pub const W332: usize = 284;
pub const ONE_OK_332: bool = true;
pub const F332_IS_ZERO: bool = true;
pub const W333: usize = 289;
pub const ONE_OK_333: bool = true;
pub const F333_IS_ZERO: bool = true;
pub const W334: usize = 300;
pub const ONE_OK_334: bool = true;
pub const F334_IS_ZERO: bool = true;
pub const W335: usize = 301;
pub const ONE_OK_335: bool = true;
pub const F335_IS_ZERO: bool = true;
pub const W336: usize = 337;
pub const ONE_OK_336: bool = true;
pub const F336_IS_ZERO: bool = true;
pub const W337: usize = 55;
pub const ONE_OK_337: bool = false;
pub const F337_IS_ZERO: bool = false;
pub const W338: usize = 283;
pub const ONE_OK_338: bool = true;
pub const F338_IS_ZERO: bool = true;
pub const W339: usize = 285;
pub const ONE_OK_339: bool = true;
pub const F339_IS_ZERO: bool = true;
pub const W340: usize = 291;
pub const ONE_OK_340: bool = true;
pub const F340_IS_ZERO: bool = true;
pub const W341: usize = 296;
pub const ONE_OK_341: bool = true;
pub const F341_IS_ZERO: bool = true;
pub const W342: usize = 307;
pub const ONE_OK_342: bool = true;
pub const F342_IS_ZERO: bool = true;
pub const W343: usize = 308;
pub const ONE_OK_343: bool = true;
pub const F343_IS_ZERO: bool = true;
pub const W344: usize = 344;
pub const ONE_OK_344: bool = true;
pub const F344_IS_ZERO: bool = true;
pub const W345: usize = 56;
pub const ONE_OK_345: bool = false;
pub const F345_IS_ZERO: bool = false;
pub const W346: usize = 290;
pub const ONE_OK_346: bool = true;
pub const F346_IS_ZERO: bool = true;
pub const W347: usize = 292;
pub const ONE_OK_347: bool = true;
pub const F347_IS_ZERO: bool = true;
pub const W348: usize = 298;
pub const ONE_OK_348: bool = true;
pub const F348_IS_ZERO: bool = true;
pub const W349: usize = 303;
pub const ONE_OK_349: bool = true;
pub const F349_IS_ZERO: bool = true;
pub const W350: usize = 314;
pub const ONE_OK_350: bool = true;
pub const F350_IS_ZERO: bool = true;
pub const W351: usize = 315;
pub const ONE_OK_351: bool = true;
pub const F351_IS_ZERO: bool = true;
pub const W352: usize = 351;
pub const ONE_OK_352: bool = true;
pub const F352_IS_ZERO: bool = true;
pub const W353: usize = 57;
pub const ONE_OK_353: bool = false;
pub const F353_IS_ZERO: bool = false;
pub const W354: usize = 297;
pub const ONE_OK_354: bool = true;
pub const F354_IS_ZERO: bool = true;
pub const W355: usize = 299;
pub const ONE_OK_355: bool = true;
pub const F355_IS_ZERO: bool = true;
pub const W356: usize = 305;
pub const ONE_OK_356: bool = true;
pub const F356_IS_ZERO: bool = true;
pub const W357: usize = 310;
pub const ONE_OK_357: bool = true;
pub const F357_IS_ZERO: bool = true;
pub const W358: usize = 321;
pub const ONE_OK_358: bool = true;
pub const F358_IS_ZERO: bool = true;
pub const W359: usize = 322;
pub const ONE_OK_359: bool = true;
pub const F359_IS_ZERO: bool = true;
pub const W360: usize = 358;
pub const ONE_OK_360: bool = true;
pub const F360_IS_ZERO: bool = true;
pub const W361: usize = 58;
pub const ONE_OK_361: bool = false;
pub const F361_IS_ZERO: bool = false;
pub const W362: usize = 304;
pub const ONE_OK_362: bool = true;
pub const F362_IS_ZERO: bool = true;
pub const W363: usize = 306;
pub const ONE_OK_363: bool = true;
pub const F363_IS_ZERO: bool = true;
pub const W364: usize = 312;
pub const ONE_OK_364: bool = true;
pub const F364_IS_ZERO: bool = true;
pub const W365: usize = 317;
pub const ONE_OK_365: bool = true;
pub const F365_IS_ZERO: bool = true;
pub const W366: usize = 328;
pub const ONE_OK_366: bool = true;
pub const F366_IS_ZERO: bool = true;
pub const W367: usize = 329;
pub const ONE_OK_367: bool = true;
pub const F367_IS_ZERO: bool = true;
pub const W368: usize = 365;
pub const ONE_OK_368: bool = true;
pub const F368_IS_ZERO: bool = true;
pub const W369: usize = 59;
pub const ONE_OK_369: bool = false;
pub const F369_IS_ZERO: bool = false;
pub const W370: usize = 311;
pub const ONE_OK_370: bool = true;
pub const F370_IS_ZERO: bool = true;
pub const W371: usize = 313;
pub const ONE_OK_371: bool = true;
pub const F371_IS_ZERO: bool = true;
pub const W372: usize = 319;
pub const ONE_OK_372: bool = true;
pub const F372_IS_ZERO: bool = true;
pub const W373: usize = 324;
pub const ONE_OK_373: bool = true;
pub const F373_IS_ZERO: bool = true;
pub const W374: usize = 335;
pub const ONE_OK_374: bool = true;
pub const F374_IS_ZERO: bool = true;
pub const W375: usize = 336;
pub const ONE_OK_375: bool = true;
pub const F375_IS_ZERO: bool = true;
pub const W376: usize = 372;
pub const ONE_OK_376: bool = true;
pub const F376_IS_ZERO: bool = true;
pub const W377: usize = 60;
pub const ONE_OK_377: bool = false;
pub const F377_IS_ZERO: bool = false;
pub const W378: usize = 318;
pub const ONE_OK_378: bool = true;
pub const F378_IS_ZERO: bool = true;
pub const W379: usize = 320;
pub const ONE_OK_379: bool = true;
pub const F379_IS_ZERO: bool = true;
pub const W380: usize = 326;
pub const ONE_OK_380: bool = true;
pub const F380_IS_ZERO: bool = true;
pub const W381: usize = 331;
pub const ONE_OK_381: bool = true;
pub const F381_IS_ZERO: bool = true;
pub const W382: usize = 342;
pub const ONE_OK_382: bool = true;
pub const F382_IS_ZERO: bool = true;
pub const W383: usize = 343;
pub const ONE_OK_383: bool = true;
pub const F383_IS_ZERO: bool = true;
pub const W384: usize = 379;
pub const ONE_OK_384: bool = true;
pub const F384_IS_ZERO: bool = true;
pub const W385: usize = 61;
pub const ONE_OK_385: bool = false;
pub const F385_IS_ZERO: bool = false;
pub const W386: usize = 325;
pub const ONE_OK_386: bool = true;
pub const F386_IS_ZERO: bool = true;
pub const W387: usize = 327;
pub const ONE_OK_387: bool = true;
pub const F387_IS_ZERO: bool = true;
pub const W388: usize = 333;
pub const ONE_OK_388: bool = true;
pub const F388_IS_ZERO: bool = true;
pub const W389: usize = 338;
pub const ONE_OK_389: bool = true;
pub const F389_IS_ZERO: bool = true;
pub const W390: usize = 349;
pub const ONE_OK_390: bool = true;
pub const F390_IS_ZERO: bool = true;
pub const W391: usize = 350;
pub const ONE_OK_391: bool = true;
pub const F391_IS_ZERO: bool = true;
pub const W392: usize = 386;
pub const ONE_OK_392: bool = true;
pub const F392_IS_ZERO: bool = true;
pub const W393: usize = 62;
pub const ONE_OK_393: bool = false;
pub const F393_IS_ZERO: bool = false;
pub const W394: usize = 332;
pub const ONE_OK_394: bool = true;
pub const F394_IS_ZERO: bool = true;
pub const W395: usize = 334;
pub const ONE_OK_395: bool = true;
pub const F395_IS_ZERO: bool = true;
pub const W396: usize = 340;
pub const ONE_OK_396: bool = true;
pub const F396_IS_ZERO: bool = true;
pub const W397: usize = 345;
pub const ONE_OK_397: bool = true;
pub const F397_IS_ZERO: bool = true;
pub const W398: usize = 356;
pub const ONE_OK_398: bool = true;
pub const F398_IS_ZERO: bool = true;
pub const W399: usize = 357;
pub const ONE_OK_399: bool = true;
pub const F399_IS_ZERO: bool = true;
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
pub const SW197: usize = 388;
pub const SW198: usize = 219;
pub const SW199: usize = 346;
pub const SW200: usize = 384;
pub const SW201: usize = 210;
pub const SW202: usize = 341;
pub const SW203: usize = 354;
pub const SW204: usize = 361;
pub const SW205: usize = 402;
pub const SW206: usize = 227;
pub const SW207: usize = 360;
pub const SW208: usize = 398;
pub const SW209: usize = 218;
pub const SW210: usize = 355;
pub const SW211: usize = 368;
pub const SW212: usize = 375;
pub const SW213: usize = 416;
pub const SW214: usize = 235;
pub const SW215: usize = 374;
pub const SW216: usize = 412;
pub const SW217: usize = 226;
pub const SW218: usize = 369;
pub const SW219: usize = 382;
pub const SW220: usize = 389;
pub const SW221: usize = 430;
pub const SW222: usize = 243;
pub const SW223: usize = 388;
pub const SW224: usize = 426;
pub const SW225: usize = 234;
pub const SW226: usize = 383;
pub const SW227: usize = 396;
pub const SW228: usize = 403;
pub const SW229: usize = 444;
pub const SW230: usize = 251;
pub const SW231: usize = 402;
pub const SW232: usize = 440;
pub const SW233: usize = 242;
pub const SW234: usize = 397;
pub const SW235: usize = 410;
pub const SW236: usize = 417;
pub const SW237: usize = 458;
pub const SW238: usize = 259;
pub const SW239: usize = 416;
pub const SW240: usize = 454;
pub const SW241: usize = 250;
pub const SW242: usize = 411;
pub const SW243: usize = 424;
pub const SW244: usize = 431;
pub const SW245: usize = 472;
pub const SW246: usize = 267;
pub const SW247: usize = 430;
pub const SW248: usize = 468;
pub const SW249: usize = 258;
pub const SW250: usize = 425;
pub const SW251: usize = 438;
pub const SW252: usize = 445;
pub const SW253: usize = 486;
pub const SW254: usize = 275;
pub const SW255: usize = 444;
pub const SW256: usize = 482;
pub const SW257: usize = 266;
pub const SW258: usize = 439;
pub const SW259: usize = 452;
pub const SW260: usize = 459;
pub const SW261: usize = 500;
pub const SW262: usize = 283;
pub const SW263: usize = 458;
pub const SW264: usize = 496;
pub const SW265: usize = 274;
pub const SW266: usize = 453;
pub const SW267: usize = 466;
pub const SW268: usize = 473;
pub const SW269: usize = 514;
pub const SW270: usize = 291;
pub const SW271: usize = 472;
pub const SW272: usize = 510;
pub const SW273: usize = 282;
pub const SW274: usize = 467;
pub const SW275: usize = 480;
pub const SW276: usize = 487;
pub const SW277: usize = 528;
pub const SW278: usize = 299;
pub const SW279: usize = 486;
pub const SW280: usize = 524;
pub const SW281: usize = 290;
pub const SW282: usize = 481;
pub const SW283: usize = 494;
pub const SW284: usize = 501;
pub const SW285: usize = 542;
pub const SW286: usize = 307;
pub const SW287: usize = 500;
pub const SW288: usize = 538;
pub const SW289: usize = 298;
pub const SW290: usize = 495;
pub const SW291: usize = 508;
pub const SW292: usize = 515;
pub const SW293: usize = 556;
pub const SW294: usize = 315;
pub const SW295: usize = 514;
pub const SW296: usize = 552;
pub const SW297: usize = 306;
pub const SW298: usize = 509;
pub const SW299: usize = 522;
pub const SW300: usize = 529;
pub const SW301: usize = 570;
pub const SW302: usize = 323;
pub const SW303: usize = 528;
pub const SW304: usize = 566;
pub const SW305: usize = 314;
pub const SW306: usize = 523;
pub const SW307: usize = 536;
pub const SW308: usize = 543;
pub const SW309: usize = 584;
pub const SW310: usize = 331;
pub const SW311: usize = 542;
pub const SW312: usize = 580;
pub const SW313: usize = 322;
pub const SW314: usize = 537;
pub const SW315: usize = 550;
pub const SW316: usize = 557;
pub const SW317: usize = 598;
pub const SW318: usize = 339;
pub const SW319: usize = 556;
pub const SW320: usize = 594;
pub const SW321: usize = 330;
pub const SW322: usize = 551;
pub const SW323: usize = 564;
pub const SW324: usize = 571;
pub const SW325: usize = 612;
pub const SW326: usize = 347;
pub const SW327: usize = 570;
pub const SW328: usize = 608;
pub const SW329: usize = 338;
pub const SW330: usize = 565;
pub const SW331: usize = 578;
pub const SW332: usize = 585;
pub const SW333: usize = 626;
pub const SW334: usize = 355;
pub const SW335: usize = 584;
pub const SW336: usize = 622;
pub const SW337: usize = 346;
pub const SW338: usize = 579;
pub const SW339: usize = 592;
pub const SW340: usize = 599;
pub const SW341: usize = 640;
pub const SW342: usize = 363;
pub const SW343: usize = 598;
pub const SW344: usize = 636;
pub const SW345: usize = 354;
pub const SW346: usize = 593;
pub const SW347: usize = 606;
pub const SW348: usize = 613;
pub const SW349: usize = 654;
pub const SW350: usize = 371;
pub const SW351: usize = 612;
pub const SW352: usize = 650;
pub const SW353: usize = 362;
pub const SW354: usize = 607;
pub const SW355: usize = 620;
pub const SW356: usize = 627;
pub const SW357: usize = 668;
pub const SW358: usize = 379;
pub const SW359: usize = 626;
pub const SW360: usize = 664;
pub const SW361: usize = 370;
pub const SW362: usize = 621;
pub const SW363: usize = 634;
pub const SW364: usize = 641;
pub const SW365: usize = 682;
pub const SW366: usize = 387;
pub const SW367: usize = 640;
pub const SW368: usize = 678;
pub const SW369: usize = 378;
pub const SW370: usize = 635;
pub const SW371: usize = 648;
pub const SW372: usize = 655;
pub const SW373: usize = 696;
pub const SW374: usize = 395;
pub const SW375: usize = 654;
pub const SW376: usize = 692;
pub const SW377: usize = 386;
pub const SW378: usize = 649;
pub const SW379: usize = 662;
pub const SW380: usize = 669;
pub const SW381: usize = 710;
pub const SW382: usize = 403;
pub const SW383: usize = 668;
pub const SW384: usize = 706;
pub const SW385: usize = 394;
pub const SW386: usize = 663;
pub const SW387: usize = 676;
pub const SW388: usize = 683;
pub const SW389: usize = 724;
pub const SW390: usize = 411;
pub const SW391: usize = 682;
pub const SW392: usize = 720;
pub const SW393: usize = 402;
pub const SW394: usize = 677;
pub const SW395: usize = 690;
pub const SW396: usize = 697;
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
pub type C200 = Slot<221>;
pub fn build200() -> [u32; 221] {
    [0; 221]
}
pub type C201 = Slot<223>;
pub fn build201() -> [u32; 223] {
    [0; 223]
}
pub type C202 = Slot<224>;
pub fn build202() -> [u32; 224] {
    [0; 224]
}
pub type C203 = Slot<227>;
pub fn build203() -> [u32; 227] {
    [0; 227]
}
pub type C204 = Slot<228>;
pub fn build204() -> [u32; 228] {
    [0; 228]
}
pub type C205 = Slot<233>;
pub fn build205() -> [u32; 233] {
    [0; 233]
}
pub type C206 = Slot<236>;
pub fn build206() -> [u32; 236] {
    [0; 236]
}
pub type C207 = Slot<248>;
pub fn build207() -> [u32; 248] {
    [0; 248]
}
pub type C208 = Slot<252>;
pub fn build208() -> [u32; 252] {
    [0; 252]
}
pub type C209 = Slot<284>;
pub fn build209() -> [u32; 284] {
    [0; 284]
}
pub type C210 = Slot<232>;
pub fn build210() -> [u32; 232] {
    [0; 232]
}
pub type C211 = Slot<234>;
pub fn build211() -> [u32; 234] {
    [0; 234]
}
pub type C212 = Slot<235>;
pub fn build212() -> [u32; 235] {
    [0; 235]
}
pub type C213 = Slot<238>;
pub fn build213() -> [u32; 238] {
    [0; 238]
}
pub type C214 = Slot<239>;
pub fn build214() -> [u32; 239] {
    [0; 239]
}
pub type C215 = Slot<244>;
pub fn build215() -> [u32; 244] {
    [0; 244]
}
pub type C216 = Slot<247>;
pub fn build216() -> [u32; 247] {
    [0; 247]
}
pub type C217 = Slot<259>;
pub fn build217() -> [u32; 259] {
    [0; 259]
}
pub type C218 = Slot<263>;
pub fn build218() -> [u32; 263] {
    [0; 263]
}
pub type C219 = Slot<295>;
pub fn build219() -> [u32; 295] {
    [0; 295]
}
pub type C220 = Slot<243>;
pub fn build220() -> [u32; 243] {
    [0; 243]
}
pub type C221 = Slot<245>;
pub fn build221() -> [u32; 245] {
    [0; 245]
}
pub type C222 = Slot<246>;
pub fn build222() -> [u32; 246] {
    [0; 246]
}
pub type C223 = Slot<249>;
pub fn build223() -> [u32; 249] {
    [0; 249]
}
pub type C224 = Slot<250>;
pub fn build224() -> [u32; 250] {
    [0; 250]
}
pub type C225 = Slot<255>;
pub fn build225() -> [u32; 255] {
    [0; 255]
}
pub type C226 = Slot<258>;
pub fn build226() -> [u32; 258] {
    [0; 258]
}
pub type C227 = Slot<270>;
pub fn build227() -> [u32; 270] {
    [0; 270]
}
pub type C228 = Slot<274>;
pub fn build228() -> [u32; 274] {
    [0; 274]
}
pub type C229 = Slot<306>;
pub fn build229() -> [u32; 306] {
    [0; 306]
}
pub type C230 = Slot<254>;
pub fn build230() -> [u32; 254] {
    [0; 254]
}
pub type C231 = Slot<256>;
pub fn build231() -> [u32; 256] {
    [0; 256]
}
pub type C232 = Slot<257>;
pub fn build232() -> [u32; 257] {
    [0; 257]
}
pub type C233 = Slot<260>;
pub fn build233() -> [u32; 260] {
    [0; 260]
}
pub type C234 = Slot<261>;
pub fn build234() -> [u32; 261] {
    [0; 261]
}
pub type C235 = Slot<266>;
pub fn build235() -> [u32; 266] {
    [0; 266]
}
pub type C236 = Slot<269>;
pub fn build236() -> [u32; 269] {
    [0; 269]
}
pub type C237 = Slot<281>;
pub fn build237() -> [u32; 281] {
    [0; 281]
}
pub type C238 = Slot<285>;
pub fn build238() -> [u32; 285] {
    [0; 285]
}
pub type C239 = Slot<317>;
pub fn build239() -> [u32; 317] {
    [0; 317]
}
pub type C240 = Slot<265>;
pub fn build240() -> [u32; 265] {
    [0; 265]
}
pub type C241 = Slot<267>;
pub fn build241() -> [u32; 267] {
    [0; 267]
}
pub type C242 = Slot<268>;
pub fn build242() -> [u32; 268] {
    [0; 268]
}
pub type C243 = Slot<271>;
pub fn build243() -> [u32; 271] {
    [0; 271]
}
pub type C244 = Slot<272>;
pub fn build244() -> [u32; 272] {
    [0; 272]
}
pub type C245 = Slot<277>;
pub fn build245() -> [u32; 277] {
    [0; 277]
}
pub type C246 = Slot<280>;
pub fn build246() -> [u32; 280] {
    [0; 280]
}
pub type C247 = Slot<292>;
pub fn build247() -> [u32; 292] {
    [0; 292]
}
pub type C248 = Slot<296>;
pub fn build248() -> [u32; 296] {
    [0; 296]
}
pub type C249 = Slot<328>;
pub fn build249() -> [u32; 328] {
    [0; 328]
}
pub type C250 = Slot<276>;
pub fn build250() -> [u32; 276] {
    [0; 276]
}
pub type C251 = Slot<278>;
pub fn build251() -> [u32; 278] {
    [0; 278]
}
pub type C252 = Slot<279>;
pub fn build252() -> [u32; 279] {
    [0; 279]
}
pub type C253 = Slot<282>;
pub fn build253() -> [u32; 282] {
    [0; 282]
}
pub type C254 = Slot<283>;
pub fn build254() -> [u32; 283] {
    [0; 283]
}
pub type C255 = Slot<288>;
pub fn build255() -> [u32; 288] {
    [0; 288]
}
pub type C256 = Slot<291>;
pub fn build256() -> [u32; 291] {
    [0; 291]
}
pub type C257 = Slot<303>;
pub fn build257() -> [u32; 303] {
    [0; 303]
}
pub type C258 = Slot<307>;
pub fn build258() -> [u32; 307] {
    [0; 307]
}
pub type C259 = Slot<339>;
pub fn build259() -> [u32; 339] {
    [0; 339]
}
pub type C260 = Slot<287>;
pub fn build260() -> [u32; 287] {
    [0; 287]
}
pub type C261 = Slot<289>;
pub fn build261() -> [u32; 289] {
    [0; 289]
}
pub type C262 = Slot<290>;
pub fn build262() -> [u32; 290] {
    [0; 290]
}
pub type C263 = Slot<293>;
pub fn build263() -> [u32; 293] {
    [0; 293]
}
pub type C264 = Slot<294>;
pub fn build264() -> [u32; 294] {
    [0; 294]
}
pub type C265 = Slot<299>;
pub fn build265() -> [u32; 299] {
    [0; 299]
}
pub type C266 = Slot<302>;
pub fn build266() -> [u32; 302] {
    [0; 302]
}
pub type C267 = Slot<314>;
pub fn build267() -> [u32; 314] {
    [0; 314]
}
pub type C268 = Slot<318>;
pub fn build268() -> [u32; 318] {
    [0; 318]
}
pub type C269 = Slot<350>;
pub fn build269() -> [u32; 350] {
    [0; 350]
}
pub type C270 = Slot<298>;
pub fn build270() -> [u32; 298] {
    [0; 298]
}
pub type C271 = Slot<300>;
pub fn build271() -> [u32; 300] {
    [0; 300]
}
pub type C272 = Slot<301>;
pub fn build272() -> [u32; 301] {
    [0; 301]
}
pub type C273 = Slot<304>;
pub fn build273() -> [u32; 304] {
    [0; 304]
}
pub type C274 = Slot<305>;
pub fn build274() -> [u32; 305] {
    [0; 305]
}
pub type C275 = Slot<310>;
pub fn build275() -> [u32; 310] {
    [0; 310]
}
pub type C276 = Slot<313>;
pub fn build276() -> [u32; 313] {
    [0; 313]
}
pub type C277 = Slot<325>;
pub fn build277() -> [u32; 325] {
    [0; 325]
}
pub type C278 = Slot<329>;
pub fn build278() -> [u32; 329] {
    [0; 329]
}
pub type C279 = Slot<361>;
pub fn build279() -> [u32; 361] {
    [0; 361]
}
pub type C280 = Slot<309>;
pub fn build280() -> [u32; 309] {
    [0; 309]
}
pub type C281 = Slot<311>;
pub fn build281() -> [u32; 311] {
    [0; 311]
}
pub type C282 = Slot<312>;
pub fn build282() -> [u32; 312] {
    [0; 312]
}
pub type C283 = Slot<315>;
pub fn build283() -> [u32; 315] {
    [0; 315]
}
pub type C284 = Slot<316>;
pub fn build284() -> [u32; 316] {
    [0; 316]
}
pub type C285 = Slot<321>;
pub fn build285() -> [u32; 321] {
    [0; 321]
}
pub type C286 = Slot<324>;
pub fn build286() -> [u32; 324] {
    [0; 324]
}
pub type C287 = Slot<336>;
pub fn build287() -> [u32; 336] {
    [0; 336]
}
pub type C288 = Slot<340>;
pub fn build288() -> [u32; 340] {
    [0; 340]
}
pub type C289 = Slot<372>;
pub fn build289() -> [u32; 372] {
    [0; 372]
}
pub type C290 = Slot<320>;
pub fn build290() -> [u32; 320] {
    [0; 320]
}
pub type C291 = Slot<322>;
pub fn build291() -> [u32; 322] {
    [0; 322]
}
pub type C292 = Slot<323>;
pub fn build292() -> [u32; 323] {
    [0; 323]
}
pub type C293 = Slot<326>;
pub fn build293() -> [u32; 326] {
    [0; 326]
}
pub type C294 = Slot<327>;
pub fn build294() -> [u32; 327] {
    [0; 327]
}
pub type C295 = Slot<332>;
pub fn build295() -> [u32; 332] {
    [0; 332]
}
pub type C296 = Slot<335>;
pub fn build296() -> [u32; 335] {
    [0; 335]
}
pub type C297 = Slot<347>;
pub fn build297() -> [u32; 347] {
    [0; 347]
}
pub type C298 = Slot<351>;
pub fn build298() -> [u32; 351] {
    [0; 351]
}
pub type C299 = Slot<383>;
pub fn build299() -> [u32; 383] {
    [0; 383]
}
pub type C300 = Slot<331>;
pub fn build300() -> [u32; 331] {
    [0; 331]
}
pub type C301 = Slot<333>;
pub fn build301() -> [u32; 333] {
    [0; 333]
}
pub type C302 = Slot<334>;
pub fn build302() -> [u32; 334] {
    [0; 334]
}
pub type C303 = Slot<337>;
pub fn build303() -> [u32; 337] {
    [0; 337]
}
pub type C304 = Slot<338>;
pub fn build304() -> [u32; 338] {
    [0; 338]
}
pub type C305 = Slot<343>;
pub fn build305() -> [u32; 343] {
    [0; 343]
}
pub type C306 = Slot<346>;
pub fn build306() -> [u32; 346] {
    [0; 346]
}
pub type C307 = Slot<358>;
pub fn build307() -> [u32; 358] {
    [0; 358]
}
pub type C308 = Slot<362>;
pub fn build308() -> [u32; 362] {
    [0; 362]
}
pub type C309 = Slot<394>;
pub fn build309() -> [u32; 394] {
    [0; 394]
}
pub type C310 = Slot<342>;
pub fn build310() -> [u32; 342] {
    [0; 342]
}
pub type C311 = Slot<344>;
pub fn build311() -> [u32; 344] {
    [0; 344]
}
pub type C312 = Slot<345>;
pub fn build312() -> [u32; 345] {
    [0; 345]
}
pub type C313 = Slot<348>;
pub fn build313() -> [u32; 348] {
    [0; 348]
}
pub type C314 = Slot<349>;
pub fn build314() -> [u32; 349] {
    [0; 349]
}
pub type C315 = Slot<354>;
pub fn build315() -> [u32; 354] {
    [0; 354]
}
pub type C316 = Slot<357>;
pub fn build316() -> [u32; 357] {
    [0; 357]
}
pub type C317 = Slot<369>;
pub fn build317() -> [u32; 369] {
    [0; 369]
}
pub type C318 = Slot<373>;
pub fn build318() -> [u32; 373] {
    [0; 373]
}
pub type C319 = Slot<405>;
pub fn build319() -> [u32; 405] {
    [0; 405]
}
pub type C320 = Slot<353>;
pub fn build320() -> [u32; 353] {
    [0; 353]
}
pub type C321 = Slot<355>;
pub fn build321() -> [u32; 355] {
    [0; 355]
}
pub type C322 = Slot<356>;
pub fn build322() -> [u32; 356] {
    [0; 356]
}
pub type C323 = Slot<359>;
pub fn build323() -> [u32; 359] {
    [0; 359]
}
pub type C324 = Slot<360>;
pub fn build324() -> [u32; 360] {
    [0; 360]
}
pub type C325 = Slot<365>;
pub fn build325() -> [u32; 365] {
    [0; 365]
}
pub type C326 = Slot<368>;
pub fn build326() -> [u32; 368] {
    [0; 368]
}
pub type C327 = Slot<380>;
pub fn build327() -> [u32; 380] {
    [0; 380]
}
pub type C328 = Slot<384>;
pub fn build328() -> [u32; 384] {
    [0; 384]
}
pub type C329 = Slot<416>;
pub fn build329() -> [u32; 416] {
    [0; 416]
}
pub type C330 = Slot<364>;
pub fn build330() -> [u32; 364] {
    [0; 364]
}
pub type C331 = Slot<366>;
pub fn build331() -> [u32; 366] {
    [0; 366]
}
pub type C332 = Slot<367>;
pub fn build332() -> [u32; 367] {
    [0; 367]
}
pub type C333 = Slot<370>;
pub fn build333() -> [u32; 370] {
    [0; 370]
}
pub type C334 = Slot<371>;
pub fn build334() -> [u32; 371] {
    [0; 371]
}
pub type C335 = Slot<376>;
pub fn build335() -> [u32; 376] {
    [0; 376]
}
pub type C336 = Slot<379>;
pub fn build336() -> [u32; 379] {
    [0; 379]
}
pub type C337 = Slot<391>;
pub fn build337() -> [u32; 391] {
    [0; 391]
}
pub type C338 = Slot<395>;
pub fn build338() -> [u32; 395] {
    [0; 395]
}
pub type C339 = Slot<427>;
pub fn build339() -> [u32; 427] {
    [0; 427]
}
pub type C340 = Slot<375>;
pub fn build340() -> [u32; 375] {
    [0; 375]
}
pub type C341 = Slot<377>;
pub fn build341() -> [u32; 377] {
    [0; 377]
}
pub type C342 = Slot<378>;
pub fn build342() -> [u32; 378] {
    [0; 378]
}
pub type C343 = Slot<381>;
pub fn build343() -> [u32; 381] {
    [0; 381]
}
pub type C344 = Slot<382>;
pub fn build344() -> [u32; 382] {
    [0; 382]
}
pub type C345 = Slot<387>;
pub fn build345() -> [u32; 387] {
    [0; 387]
}
pub type C346 = Slot<390>;
pub fn build346() -> [u32; 390] {
    [0; 390]
}
pub type C347 = Slot<402>;
pub fn build347() -> [u32; 402] {
    [0; 402]
}
pub type C348 = Slot<406>;
pub fn build348() -> [u32; 406] {
    [0; 406]
}
pub type C349 = Slot<438>;
pub fn build349() -> [u32; 438] {
    [0; 438]
}
pub type C350 = Slot<386>;
pub fn build350() -> [u32; 386] {
    [0; 386]
}
pub type C351 = Slot<388>;
pub fn build351() -> [u32; 388] {
    [0; 388]
}
pub type C352 = Slot<389>;
pub fn build352() -> [u32; 389] {
    [0; 389]
}
pub type C353 = Slot<392>;
pub fn build353() -> [u32; 392] {
    [0; 392]
}
pub type C354 = Slot<393>;
pub fn build354() -> [u32; 393] {
    [0; 393]
}
pub type C355 = Slot<398>;
pub fn build355() -> [u32; 398] {
    [0; 398]
}
pub type C356 = Slot<401>;
pub fn build356() -> [u32; 401] {
    [0; 401]
}
pub type C357 = Slot<413>;
pub fn build357() -> [u32; 413] {
    [0; 413]
}
pub type C358 = Slot<417>;
pub fn build358() -> [u32; 417] {
    [0; 417]
}
pub type C359 = Slot<449>;
pub fn build359() -> [u32; 449] {
    [0; 449]
}
pub type C360 = Slot<397>;
pub fn build360() -> [u32; 397] {
    [0; 397]
}
pub type C361 = Slot<399>;
pub fn build361() -> [u32; 399] {
    [0; 399]
}
pub type C362 = Slot<400>;
pub fn build362() -> [u32; 400] {
    [0; 400]
}
pub type C363 = Slot<403>;
pub fn build363() -> [u32; 403] {
    [0; 403]
}
pub type C364 = Slot<404>;
pub fn build364() -> [u32; 404] {
    [0; 404]
}
pub type C365 = Slot<409>;
pub fn build365() -> [u32; 409] {
    [0; 409]
}
pub type C366 = Slot<412>;
pub fn build366() -> [u32; 412] {
    [0; 412]
}
pub type C367 = Slot<424>;
pub fn build367() -> [u32; 424] {
    [0; 424]
}
pub type C368 = Slot<428>;
pub fn build368() -> [u32; 428] {
    [0; 428]
}
pub type C369 = Slot<460>;
pub fn build369() -> [u32; 460] {
    [0; 460]
}
pub type C370 = Slot<408>;
pub fn build370() -> [u32; 408] {
    [0; 408]
}
pub type C371 = Slot<410>;
pub fn build371() -> [u32; 410] {
    [0; 410]
}
pub type C372 = Slot<411>;
pub fn build372() -> [u32; 411] {
    [0; 411]
}
pub type C373 = Slot<414>;
pub fn build373() -> [u32; 414] {
    [0; 414]
}
pub type C374 = Slot<415>;
pub fn build374() -> [u32; 415] {
    [0; 415]
}
pub type C375 = Slot<420>;
pub fn build375() -> [u32; 420] {
    [0; 420]
}
pub type C376 = Slot<423>;
pub fn build376() -> [u32; 423] {
    [0; 423]
}
pub type C377 = Slot<435>;
pub fn build377() -> [u32; 435] {
    [0; 435]
}
pub type C378 = Slot<439>;
pub fn build378() -> [u32; 439] {
    [0; 439]
}
pub type C379 = Slot<471>;
pub fn build379() -> [u32; 471] {
    [0; 471]
}
pub type C380 = Slot<419>;
pub fn build380() -> [u32; 419] {
    [0; 419]
}
pub type C381 = Slot<421>;
pub fn build381() -> [u32; 421] {
    [0; 421]
}
pub type C382 = Slot<422>;
pub fn build382() -> [u32; 422] {
    [0; 422]
}
pub type C383 = Slot<425>;
pub fn build383() -> [u32; 425] {
    [0; 425]
}
pub type C384 = Slot<426>;
pub fn build384() -> [u32; 426] {
    [0; 426]
}
pub type C385 = Slot<431>;
pub fn build385() -> [u32; 431] {
    [0; 431]
}
pub type C386 = Slot<434>;
pub fn build386() -> [u32; 434] {
    [0; 434]
}
pub type C387 = Slot<446>;
pub fn build387() -> [u32; 446] {
    [0; 446]
}
pub type C388 = Slot<450>;
pub fn build388() -> [u32; 450] {
    [0; 450]
}
pub type C389 = Slot<482>;
pub fn build389() -> [u32; 482] {
    [0; 482]
}
pub type C390 = Slot<430>;
pub fn build390() -> [u32; 430] {
    [0; 430]
}
pub type C391 = Slot<432>;
pub fn build391() -> [u32; 432] {
    [0; 432]
}
pub type C392 = Slot<433>;
pub fn build392() -> [u32; 433] {
    [0; 433]
}
pub type C393 = Slot<436>;
pub fn build393() -> [u32; 436] {
    [0; 436]
}
pub type C394 = Slot<437>;
pub fn build394() -> [u32; 437] {
    [0; 437]
}
pub type C395 = Slot<442>;
pub fn build395() -> [u32; 442] {
    [0; 442]
}
pub type C396 = Slot<445>;
pub fn build396() -> [u32; 445] {
    [0; 445]
}
pub type C397 = Slot<457>;
pub fn build397() -> [u32; 457] {
    [0; 457]
}
pub type C398 = Slot<461>;
pub fn build398() -> [u32; 461] {
    [0; 461]
}
pub type C399 = Slot<493>;
pub fn build399() -> [u32; 493] {
    [0; 493]
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
pub fn call200() -> u32 {
    scaled_fold::<C200>(200, W200)
}
pub fn call202() -> u32 {
    scaled_fold::<C202>(202, W202)
}
pub fn call203() -> u32 {
    scaled_fold::<C203>(203, W203)
}
pub fn call204() -> u32 {
    scaled_fold::<C204>(204, W204)
}
pub fn call205() -> u32 {
    scaled_fold::<C205>(205, W205)
}
pub fn call206() -> u32 {
    scaled_fold::<C206>(206, W206)
}
pub fn call207() -> u32 {
    scaled_fold::<C207>(207, W207)
}
pub fn call208() -> u32 {
    scaled_fold::<C208>(208, W208)
}
pub fn call210() -> u32 {
    scaled_fold::<C210>(210, W210)
}
pub fn call211() -> u32 {
    scaled_fold::<C211>(211, W211)
}
pub fn call212() -> u32 {
    scaled_fold::<C212>(212, W212)
}
pub fn call213() -> u32 {
    scaled_fold::<C213>(213, W213)
}
pub fn call214() -> u32 {
    scaled_fold::<C214>(214, W214)
}
pub fn call215() -> u32 {
    scaled_fold::<C215>(215, W215)
}
pub fn call216() -> u32 {
    scaled_fold::<C216>(216, W216)
}
pub fn call218() -> u32 {
    scaled_fold::<C218>(218, W218)
}
pub fn call219() -> u32 {
    scaled_fold::<C219>(219, W219)
}
pub fn call220() -> u32 {
    scaled_fold::<C220>(220, W220)
}
pub fn call221() -> u32 {
    scaled_fold::<C221>(221, W221)
}
pub fn call222() -> u32 {
    scaled_fold::<C222>(222, W222)
}
pub fn call223() -> u32 {
    scaled_fold::<C223>(223, W223)
}
pub fn call224() -> u32 {
    scaled_fold::<C224>(224, W224)
}
pub fn call226() -> u32 {
    scaled_fold::<C226>(226, W226)
}
pub fn call227() -> u32 {
    scaled_fold::<C227>(227, W227)
}
pub fn call228() -> u32 {
    scaled_fold::<C228>(228, W228)
}
pub fn call229() -> u32 {
    scaled_fold::<C229>(229, W229)
}
pub fn call230() -> u32 {
    scaled_fold::<C230>(230, W230)
}
pub fn call231() -> u32 {
    scaled_fold::<C231>(231, W231)
}
pub fn call232() -> u32 {
    scaled_fold::<C232>(232, W232)
}
pub fn call234() -> u32 {
    scaled_fold::<C234>(234, W234)
}
pub fn call235() -> u32 {
    scaled_fold::<C235>(235, W235)
}
pub fn call236() -> u32 {
    scaled_fold::<C236>(236, W236)
}
pub fn call237() -> u32 {
    scaled_fold::<C237>(237, W237)
}
pub fn call238() -> u32 {
    scaled_fold::<C238>(238, W238)
}
pub fn call239() -> u32 {
    scaled_fold::<C239>(239, W239)
}
pub fn call240() -> u32 {
    scaled_fold::<C240>(240, W240)
}
pub fn call242() -> u32 {
    scaled_fold::<C242>(242, W242)
}
pub fn call243() -> u32 {
    scaled_fold::<C243>(243, W243)
}
pub fn call244() -> u32 {
    scaled_fold::<C244>(244, W244)
}
pub fn call245() -> u32 {
    scaled_fold::<C245>(245, W245)
}
pub fn call246() -> u32 {
    scaled_fold::<C246>(246, W246)
}
pub fn call247() -> u32 {
    scaled_fold::<C247>(247, W247)
}
pub fn call248() -> u32 {
    scaled_fold::<C248>(248, W248)
}
pub fn call250() -> u32 {
    scaled_fold::<C250>(250, W250)
}
pub fn call251() -> u32 {
    scaled_fold::<C251>(251, W251)
}
pub fn call252() -> u32 {
    scaled_fold::<C252>(252, W252)
}
pub fn call253() -> u32 {
    scaled_fold::<C253>(253, W253)
}
pub fn call254() -> u32 {
    scaled_fold::<C254>(254, W254)
}
pub fn call255() -> u32 {
    scaled_fold::<C255>(255, W255)
}
pub fn call256() -> u32 {
    scaled_fold::<C256>(256, W256)
}
pub fn call258() -> u32 {
    scaled_fold::<C258>(258, W258)
}
pub fn call259() -> u32 {
    scaled_fold::<C259>(259, W259)
}
pub fn call260() -> u32 {
    scaled_fold::<C260>(260, W260)
}
pub fn call261() -> u32 {
    scaled_fold::<C261>(261, W261)
}
pub fn call262() -> u32 {
    scaled_fold::<C262>(262, W262)
}
pub fn call263() -> u32 {
    scaled_fold::<C263>(263, W263)
}
pub fn call264() -> u32 {
    scaled_fold::<C264>(264, W264)
}
pub fn call266() -> u32 {
    scaled_fold::<C266>(266, W266)
}
pub fn call267() -> u32 {
    scaled_fold::<C267>(267, W267)
}
pub fn call268() -> u32 {
    scaled_fold::<C268>(268, W268)
}
pub fn call269() -> u32 {
    scaled_fold::<C269>(269, W269)
}
pub fn call270() -> u32 {
    scaled_fold::<C270>(270, W270)
}
pub fn call271() -> u32 {
    scaled_fold::<C271>(271, W271)
}
pub fn call272() -> u32 {
    scaled_fold::<C272>(272, W272)
}
pub fn call274() -> u32 {
    scaled_fold::<C274>(274, W274)
}
pub fn call275() -> u32 {
    scaled_fold::<C275>(275, W275)
}
pub fn call276() -> u32 {
    scaled_fold::<C276>(276, W276)
}
pub fn call277() -> u32 {
    scaled_fold::<C277>(277, W277)
}
pub fn call278() -> u32 {
    scaled_fold::<C278>(278, W278)
}
pub fn call279() -> u32 {
    scaled_fold::<C279>(279, W279)
}
pub fn call280() -> u32 {
    scaled_fold::<C280>(280, W280)
}
pub fn call282() -> u32 {
    scaled_fold::<C282>(282, W282)
}
pub fn call283() -> u32 {
    scaled_fold::<C283>(283, W283)
}
pub fn call284() -> u32 {
    scaled_fold::<C284>(284, W284)
}
pub fn call285() -> u32 {
    scaled_fold::<C285>(285, W285)
}
pub fn call286() -> u32 {
    scaled_fold::<C286>(286, W286)
}
pub fn call287() -> u32 {
    scaled_fold::<C287>(287, W287)
}
pub fn call288() -> u32 {
    scaled_fold::<C288>(288, W288)
}
pub fn call290() -> u32 {
    scaled_fold::<C290>(290, W290)
}
pub fn call291() -> u32 {
    scaled_fold::<C291>(291, W291)
}
pub fn call292() -> u32 {
    scaled_fold::<C292>(292, W292)
}
pub fn call293() -> u32 {
    scaled_fold::<C293>(293, W293)
}
pub fn call294() -> u32 {
    scaled_fold::<C294>(294, W294)
}
pub fn call295() -> u32 {
    scaled_fold::<C295>(295, W295)
}
pub fn call296() -> u32 {
    scaled_fold::<C296>(296, W296)
}
pub fn call298() -> u32 {
    scaled_fold::<C298>(298, W298)
}
pub fn call299() -> u32 {
    scaled_fold::<C299>(299, W299)
}
pub fn call300() -> u32 {
    scaled_fold::<C300>(300, W300)
}
pub fn call301() -> u32 {
    scaled_fold::<C301>(301, W301)
}
pub fn call302() -> u32 {
    scaled_fold::<C302>(302, W302)
}
pub fn call303() -> u32 {
    scaled_fold::<C303>(303, W303)
}
pub fn call304() -> u32 {
    scaled_fold::<C304>(304, W304)
}
pub fn call306() -> u32 {
    scaled_fold::<C306>(306, W306)
}
pub fn call307() -> u32 {
    scaled_fold::<C307>(307, W307)
}
pub fn call308() -> u32 {
    scaled_fold::<C308>(308, W308)
}
pub fn call309() -> u32 {
    scaled_fold::<C309>(309, W309)
}
pub fn call310() -> u32 {
    scaled_fold::<C310>(310, W310)
}
pub fn call311() -> u32 {
    scaled_fold::<C311>(311, W311)
}
pub fn call312() -> u32 {
    scaled_fold::<C312>(312, W312)
}
pub fn call314() -> u32 {
    scaled_fold::<C314>(314, W314)
}
pub fn call315() -> u32 {
    scaled_fold::<C315>(315, W315)
}
pub fn call316() -> u32 {
    scaled_fold::<C316>(316, W316)
}
pub fn call317() -> u32 {
    scaled_fold::<C317>(317, W317)
}
pub fn call318() -> u32 {
    scaled_fold::<C318>(318, W318)
}
pub fn call319() -> u32 {
    scaled_fold::<C319>(319, W319)
}
pub fn call320() -> u32 {
    scaled_fold::<C320>(320, W320)
}
pub fn call322() -> u32 {
    scaled_fold::<C322>(322, W322)
}
pub fn call323() -> u32 {
    scaled_fold::<C323>(323, W323)
}
pub fn call324() -> u32 {
    scaled_fold::<C324>(324, W324)
}
pub fn call325() -> u32 {
    scaled_fold::<C325>(325, W325)
}
pub fn call326() -> u32 {
    scaled_fold::<C326>(326, W326)
}
pub fn call327() -> u32 {
    scaled_fold::<C327>(327, W327)
}
pub fn call328() -> u32 {
    scaled_fold::<C328>(328, W328)
}
pub fn call330() -> u32 {
    scaled_fold::<C330>(330, W330)
}
pub fn call331() -> u32 {
    scaled_fold::<C331>(331, W331)
}
pub fn call332() -> u32 {
    scaled_fold::<C332>(332, W332)
}
pub fn call333() -> u32 {
    scaled_fold::<C333>(333, W333)
}
pub fn call334() -> u32 {
    scaled_fold::<C334>(334, W334)
}
pub fn call335() -> u32 {
    scaled_fold::<C335>(335, W335)
}
pub fn call336() -> u32 {
    scaled_fold::<C336>(336, W336)
}
pub fn call338() -> u32 {
    scaled_fold::<C338>(338, W338)
}
pub fn call339() -> u32 {
    scaled_fold::<C339>(339, W339)
}
pub fn call340() -> u32 {
    scaled_fold::<C340>(340, W340)
}
pub fn call341() -> u32 {
    scaled_fold::<C341>(341, W341)
}
pub fn call342() -> u32 {
    scaled_fold::<C342>(342, W342)
}
pub fn call343() -> u32 {
    scaled_fold::<C343>(343, W343)
}
pub fn call344() -> u32 {
    scaled_fold::<C344>(344, W344)
}
pub fn call346() -> u32 {
    scaled_fold::<C346>(346, W346)
}
pub fn call347() -> u32 {
    scaled_fold::<C347>(347, W347)
}
pub fn call348() -> u32 {
    scaled_fold::<C348>(348, W348)
}
pub fn call349() -> u32 {
    scaled_fold::<C349>(349, W349)
}
pub fn call350() -> u32 {
    scaled_fold::<C350>(350, W350)
}
pub fn call351() -> u32 {
    scaled_fold::<C351>(351, W351)
}
pub fn call352() -> u32 {
    scaled_fold::<C352>(352, W352)
}
pub fn call354() -> u32 {
    scaled_fold::<C354>(354, W354)
}
pub fn call355() -> u32 {
    scaled_fold::<C355>(355, W355)
}
pub fn call356() -> u32 {
    scaled_fold::<C356>(356, W356)
}
pub fn call357() -> u32 {
    scaled_fold::<C357>(357, W357)
}
pub fn call358() -> u32 {
    scaled_fold::<C358>(358, W358)
}
pub fn call359() -> u32 {
    scaled_fold::<C359>(359, W359)
}
pub fn call360() -> u32 {
    scaled_fold::<C360>(360, W360)
}
pub fn call362() -> u32 {
    scaled_fold::<C362>(362, W362)
}
pub fn call363() -> u32 {
    scaled_fold::<C363>(363, W363)
}
pub fn call364() -> u32 {
    scaled_fold::<C364>(364, W364)
}
pub fn call365() -> u32 {
    scaled_fold::<C365>(365, W365)
}
pub fn call366() -> u32 {
    scaled_fold::<C366>(366, W366)
}
pub fn call367() -> u32 {
    scaled_fold::<C367>(367, W367)
}
pub fn call368() -> u32 {
    scaled_fold::<C368>(368, W368)
}
pub fn call370() -> u32 {
    scaled_fold::<C370>(370, W370)
}
pub fn call371() -> u32 {
    scaled_fold::<C371>(371, W371)
}
pub fn call372() -> u32 {
    scaled_fold::<C372>(372, W372)
}
pub fn call373() -> u32 {
    scaled_fold::<C373>(373, W373)
}
pub fn call374() -> u32 {
    scaled_fold::<C374>(374, W374)
}
pub fn call375() -> u32 {
    scaled_fold::<C375>(375, W375)
}
pub fn call376() -> u32 {
    scaled_fold::<C376>(376, W376)
}
pub fn call378() -> u32 {
    scaled_fold::<C378>(378, W378)
}
pub fn call379() -> u32 {
    scaled_fold::<C379>(379, W379)
}
pub fn call380() -> u32 {
    scaled_fold::<C380>(380, W380)
}
pub fn call381() -> u32 {
    scaled_fold::<C381>(381, W381)
}
pub fn call382() -> u32 {
    scaled_fold::<C382>(382, W382)
}
pub fn call383() -> u32 {
    scaled_fold::<C383>(383, W383)
}
pub fn call384() -> u32 {
    scaled_fold::<C384>(384, W384)
}
pub fn call386() -> u32 {
    scaled_fold::<C386>(386, W386)
}
pub fn call387() -> u32 {
    scaled_fold::<C387>(387, W387)
}
pub fn call388() -> u32 {
    scaled_fold::<C388>(388, W388)
}
pub fn call389() -> u32 {
    scaled_fold::<C389>(389, W389)
}
pub fn call390() -> u32 {
    scaled_fold::<C390>(390, W390)
}
pub fn call391() -> u32 {
    scaled_fold::<C391>(391, W391)
}
pub fn call392() -> u32 {
    scaled_fold::<C392>(392, W392)
}
pub fn call394() -> u32 {
    scaled_fold::<C394>(394, W394)
}
pub fn call395() -> u32 {
    scaled_fold::<C395>(395, W395)
}
pub fn call396() -> u32 {
    scaled_fold::<C396>(396, W396)
}
pub fn call397() -> u32 {
    scaled_fold::<C397>(397, W397)
}
pub fn call398() -> u32 {
    scaled_fold::<C398>(398, W398)
}
pub fn call399() -> u32 {
    scaled_fold::<C399>(399, W399)
}
