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
