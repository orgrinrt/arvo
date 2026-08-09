//! Probe 8 (generated). Probe 7's shape with a full per-width table to 256.
//!
//! The 2026-07-28 sketch's shape takes the width as a TYPE
//! (`BitsContainerFor<Wid<13>, Unsigned>`), which changes the trait's own
//! signature and so propagates to `Bits` and to the facade. This probe asks
//! whether the signature can stay exactly as shipped
//! (`BitsContainerFor<const N: Width, Sign>`) with `Wid<N>` used only INSIDE
//! the impl, where `N` is a standalone argument and needs no feature.
//!
//! If yes, the two gates are independent problems with independent prices,
//! and the cheap one is genuinely cheap.
#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Width(pub u16);

pub trait Signedness {}
pub struct Unsigned;
impl Signedness for Unsigned {}
pub struct Signed;
impl Signedness for Signed {}
pub trait Strategy {}
pub struct Hot;
impl Strategy for Hot {}
pub struct Warm;
impl Strategy for Warm {}

// -- the bucket vocabulary, as the sketch has it ---------------------------
pub struct B8;
pub struct B16;
pub struct B32;
pub struct B64;
pub struct B128;
pub struct BWide<const BYTES: usize>;
pub trait Bucket {}
impl Bucket for B8 {}
impl Bucket for B16 {}
impl Bucket for B32 {}
impl Bucket for B64 {}
impl Bucket for B128 {}
impl<const BYTES: usize> Bucket for BWide<BYTES> {}

pub trait Family {}
pub struct HotCold;
impl Family for HotCold {}
pub struct WarmPrecise;
impl Family for WarmPrecise {}

// Width-as-typestate, INTERNAL to the crate. Never appears in a signature.
pub struct Wid<const N: Width>;
pub trait WidthFor<F: Family> {
    type Bkt: Bucket;
}
impl WidthFor<HotCold> for Wid<{ Width(1) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(1) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(2) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(2) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(3) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(3) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(4) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(4) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(5) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(5) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(6) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(6) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(7) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(7) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(8) }> {
    type Bkt = B8;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(8) }> {
    type Bkt = B16;
}
impl WidthFor<HotCold> for Wid<{ Width(9) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(9) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(10) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(10) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(11) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(11) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(12) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(12) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(13) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(13) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(14) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(14) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(15) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(15) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(16) }> {
    type Bkt = B16;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(16) }> {
    type Bkt = B32;
}
impl WidthFor<HotCold> for Wid<{ Width(17) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(17) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(18) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(18) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(19) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(19) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(20) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(20) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(21) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(21) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(22) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(22) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(23) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(23) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(24) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(24) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(25) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(25) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(26) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(26) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(27) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(27) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(28) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(28) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(29) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(29) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(30) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(30) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(31) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(31) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(32) }> {
    type Bkt = B32;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(32) }> {
    type Bkt = B64;
}
impl WidthFor<HotCold> for Wid<{ Width(33) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(33) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(34) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(34) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(35) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(35) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(36) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(36) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(37) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(37) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(38) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(38) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(39) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(39) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(40) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(40) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(41) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(41) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(42) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(42) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(43) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(43) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(44) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(44) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(45) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(45) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(46) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(46) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(47) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(47) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(48) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(48) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(49) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(49) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(50) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(50) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(51) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(51) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(52) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(52) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(53) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(53) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(54) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(54) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(55) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(55) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(56) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(56) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(57) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(57) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(58) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(58) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(59) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(59) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(60) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(60) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(61) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(61) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(62) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(62) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(63) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(63) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(64) }> {
    type Bkt = B64;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(64) }> {
    type Bkt = B128;
}
impl WidthFor<HotCold> for Wid<{ Width(65) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(65) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(66) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(66) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(67) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(67) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(68) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(68) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(69) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(69) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(70) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(70) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(71) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(71) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(72) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(72) }> {
    type Bkt = BWide<9>;
}
impl WidthFor<HotCold> for Wid<{ Width(73) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(73) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(74) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(74) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(75) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(75) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(76) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(76) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(77) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(77) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(78) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(78) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(79) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(79) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(80) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(80) }> {
    type Bkt = BWide<10>;
}
impl WidthFor<HotCold> for Wid<{ Width(81) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(81) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(82) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(82) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(83) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(83) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(84) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(84) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(85) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(85) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(86) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(86) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(87) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(87) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(88) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(88) }> {
    type Bkt = BWide<11>;
}
impl WidthFor<HotCold> for Wid<{ Width(89) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(89) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(90) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(90) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(91) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(91) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(92) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(92) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(93) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(93) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(94) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(94) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(95) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(95) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(96) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(96) }> {
    type Bkt = BWide<12>;
}
impl WidthFor<HotCold> for Wid<{ Width(97) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(97) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(98) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(98) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(99) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(99) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(100) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(100) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(101) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(101) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(102) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(102) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(103) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(103) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(104) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(104) }> {
    type Bkt = BWide<13>;
}
impl WidthFor<HotCold> for Wid<{ Width(105) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(105) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(106) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(106) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(107) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(107) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(108) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(108) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(109) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(109) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(110) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(110) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(111) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(111) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(112) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(112) }> {
    type Bkt = BWide<14>;
}
impl WidthFor<HotCold> for Wid<{ Width(113) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(113) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(114) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(114) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(115) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(115) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(116) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(116) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(117) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(117) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(118) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(118) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(119) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(119) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(120) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(120) }> {
    type Bkt = BWide<15>;
}
impl WidthFor<HotCold> for Wid<{ Width(121) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(121) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(122) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(122) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(123) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(123) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(124) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(124) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(125) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(125) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(126) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(126) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(127) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(127) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(128) }> {
    type Bkt = B128;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(128) }> {
    type Bkt = BWide<16>;
}
impl WidthFor<HotCold> for Wid<{ Width(129) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(129) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(130) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(130) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(131) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(131) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(132) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(132) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(133) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(133) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(134) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(134) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(135) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(135) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(136) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(136) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<HotCold> for Wid<{ Width(137) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(137) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(138) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(138) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(139) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(139) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(140) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(140) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(141) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(141) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(142) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(142) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(143) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(143) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(144) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(144) }> {
    type Bkt = BWide<18>;
}
impl WidthFor<HotCold> for Wid<{ Width(145) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(145) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(146) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(146) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(147) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(147) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(148) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(148) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(149) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(149) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(150) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(150) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(151) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(151) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(152) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(152) }> {
    type Bkt = BWide<19>;
}
impl WidthFor<HotCold> for Wid<{ Width(153) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(153) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(154) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(154) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(155) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(155) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(156) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(156) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(157) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(157) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(158) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(158) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(159) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(159) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(160) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(160) }> {
    type Bkt = BWide<20>;
}
impl WidthFor<HotCold> for Wid<{ Width(161) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(161) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(162) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(162) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(163) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(163) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(164) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(164) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(165) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(165) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(166) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(166) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(167) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(167) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(168) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(168) }> {
    type Bkt = BWide<21>;
}
impl WidthFor<HotCold> for Wid<{ Width(169) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(169) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(170) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(170) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(171) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(171) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(172) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(172) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(173) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(173) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(174) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(174) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(175) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(175) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(176) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(176) }> {
    type Bkt = BWide<22>;
}
impl WidthFor<HotCold> for Wid<{ Width(177) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(177) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(178) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(178) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(179) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(179) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(180) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(180) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(181) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(181) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(182) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(182) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(183) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(183) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(184) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(184) }> {
    type Bkt = BWide<23>;
}
impl WidthFor<HotCold> for Wid<{ Width(185) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(185) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(186) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(186) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(187) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(187) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(188) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(188) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(189) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(189) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(190) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(190) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(191) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(191) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(192) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(192) }> {
    type Bkt = BWide<24>;
}
impl WidthFor<HotCold> for Wid<{ Width(193) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(193) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(194) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(194) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(195) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(195) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(196) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(196) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(197) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(197) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(198) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(198) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(199) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(199) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(200) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(200) }> {
    type Bkt = BWide<25>;
}
impl WidthFor<HotCold> for Wid<{ Width(201) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(201) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(202) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(202) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(203) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(203) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(204) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(204) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(205) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(205) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(206) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(206) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(207) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(207) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(208) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(208) }> {
    type Bkt = BWide<26>;
}
impl WidthFor<HotCold> for Wid<{ Width(209) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(209) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(210) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(210) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(211) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(211) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(212) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(212) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(213) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(213) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(214) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(214) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(215) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(215) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(216) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(216) }> {
    type Bkt = BWide<27>;
}
impl WidthFor<HotCold> for Wid<{ Width(217) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(217) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(218) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(218) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(219) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(219) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(220) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(220) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(221) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(221) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(222) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(222) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(223) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(223) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(224) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(224) }> {
    type Bkt = BWide<28>;
}
impl WidthFor<HotCold> for Wid<{ Width(225) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(225) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(226) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(226) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(227) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(227) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(228) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(228) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(229) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(229) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(230) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(230) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(231) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(231) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(232) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(232) }> {
    type Bkt = BWide<29>;
}
impl WidthFor<HotCold> for Wid<{ Width(233) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(233) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(234) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(234) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(235) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(235) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(236) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(236) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(237) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(237) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(238) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(238) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(239) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(239) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(240) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(240) }> {
    type Bkt = BWide<30>;
}
impl WidthFor<HotCold> for Wid<{ Width(241) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(241) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(242) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(242) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(243) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(243) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(244) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(244) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(245) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(245) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(246) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(246) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(247) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(247) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(248) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(248) }> {
    type Bkt = BWide<31>;
}
impl WidthFor<HotCold> for Wid<{ Width(249) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(249) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(250) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(250) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(251) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(251) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(252) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(252) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(253) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(253) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(254) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(254) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(255) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(255) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<HotCold> for Wid<{ Width(256) }> {
    type Bkt = BWide<32>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(256) }> {
    type Bkt = BWide<32>;
}

pub trait Project<B: Bucket, Sign: Signedness, S: Strategy> {
    type T: Copy;
}
pub struct Picker;
macro_rules! project {
    ($s:ty, $( $b:ty => ($u:ty, $i:ty) ),* $(,)?) => { $(
        impl Project<$b, Unsigned, $s> for Picker { type T = $u; }
        impl Project<$b, Signed,   $s> for Picker { type T = $i; }
    )* };
}
project!(Hot, B8 => (u8, i8), B16 => (u16, i16), B32 => (u32, i32),
              B64 => (u64, i64), B128 => (u128, i128));
project!(Warm, B16 => (u16, i16), B32 => (u32, i32),
               B64 => (u64, i64), B128 => (u128, i128));
pub struct WideStore<const BYTES: usize>([u8; BYTES]);
impl<const BYTES: usize> Clone for WideStore<BYTES> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const BYTES: usize> Copy for WideStore<BYTES> {}
impl<const BYTES: usize, Sign: Signedness> Project<BWide<BYTES>, Sign, Hot> for Picker {
    type T = WideStore<BYTES>;
}
impl<const BYTES: usize, Sign: Signedness> Project<BWide<BYTES>, Sign, Warm> for Picker {
    type T = WideStore<BYTES>;
}

// -- THE SHIPPED SIGNATURE, UNCHANGED --------------------------------------
// `arvo-strategy/src/container.rs:114`, except `u16` -> `Width` as the crate's
// own doc (`width.rs:6-8`) already says it should be.
pub trait BitsContainerFor<const N: Width, Sign: Signedness>: Strategy {
    type T: Copy;
}

impl<const N: Width, Sign: Signedness> BitsContainerFor<N, Sign> for Hot
where
    Wid<N>: WidthFor<HotCold>,
    Picker: Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Hot>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Hot>>::T;
}

impl<const N: Width, Sign: Signedness> BitsContainerFor<N, Sign> for Warm
where
    Wid<N>: WidthFor<WarmPrecise>,
    Picker: Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Warm>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Warm>>::T;
}

// -- `Bits`, signature unchanged from arvo-storage/src/bits.rs:57 ----------
#[repr(transparent)]
pub struct Bits<const N: Width, S: Strategy = Hot, Sign: Signedness = Unsigned>(
    <S as BitsContainerFor<N, Sign>>::T,
)
where
    S: BitsContainerFor<N, Sign>;

// -- resolution checks, not just parsing ----------------------------------
const _: () = {
    let _: <Hot as BitsContainerFor<{ Width(13) }, Unsigned>>::T = 0u16;
    let _: <Hot as BitsContainerFor<{ Width(64) }, Signed>>::T = 0i64;
    let _: <Warm as BitsContainerFor<{ Width(13) }, Unsigned>>::T = 0u32;
    let _: <Warm as BitsContainerFor<{ Width(32) }, Signed>>::T = 0i64;
    let _: <Hot as BitsContainerFor<{ Width(129) }, Unsigned>>::T = WideStore::<17>([0u8; 17]);
};
// the caller-threads-its-own-generic case
pub fn threaded<const N: Width, S: Strategy, Sign: Signedness>(_b: Bits<N, S, Sign>)
where
    S: BitsContainerFor<N, Sign>,
{
}
const _: () = {
    let _ = threaded::<{ Width(13) }, Hot, Unsigned>;
};
