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
