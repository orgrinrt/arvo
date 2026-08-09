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
