#![no_std]
#![allow(dead_code)]

#[repr(transparent)] pub struct P<const W: u32, S>(pub u64, pub core::marker::PhantomData<S>);
pub struct Wrap; pub struct Clamp;
pub const fn mask(w: u32) -> u64 { if w >= 64 { u64::MAX } else { (1u64 << w) - 1 } }

// --- family d: distinct by construction, one per (width, policy) ---
#[unsafe(no_mangle)] pub fn d_wrap_3(x: u64) -> u64 { x & mask(3) }
#[unsafe(no_mangle)] pub fn d_clamp_3(x: u64) -> u64 { { let m = mask(3); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_7(x: u64) -> u64 { x & mask(7) }
#[unsafe(no_mangle)] pub fn d_clamp_7(x: u64) -> u64 { { let m = mask(7); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_11(x: u64) -> u64 { x & mask(11) }
#[unsafe(no_mangle)] pub fn d_clamp_11(x: u64) -> u64 { { let m = mask(11); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_13(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn d_clamp_13(x: u64) -> u64 { { let m = mask(13); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_14(x: u64) -> u64 { x & mask(14) }
#[unsafe(no_mangle)] pub fn d_clamp_14(x: u64) -> u64 { { let m = mask(14); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_23(x: u64) -> u64 { x & mask(23) }
#[unsafe(no_mangle)] pub fn d_clamp_23(x: u64) -> u64 { { let m = mask(23); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_27(x: u64) -> u64 { x & mask(27) }
#[unsafe(no_mangle)] pub fn d_clamp_27(x: u64) -> u64 { { let m = mask(27); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_31(x: u64) -> u64 { x & mask(31) }
#[unsafe(no_mangle)] pub fn d_clamp_31(x: u64) -> u64 { { let m = mask(31); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_47(x: u64) -> u64 { x & mask(47) }
#[unsafe(no_mangle)] pub fn d_clamp_47(x: u64) -> u64 { { let m = mask(47); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn d_wrap_61(x: u64) -> u64 { x & mask(61) }
#[unsafe(no_mangle)] pub fn d_clamp_61(x: u64) -> u64 { { let m = mask(61); if x > m { m } else { x } } }

// --- family s: identical by construction, distinct names only ---
#[unsafe(no_mangle)] pub fn s_alias_0(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_1(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_2(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_3(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_4(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_5(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_6(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_7(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_8(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_9(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_10(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_11(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_12(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_13(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_14(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_15(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_16(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_17(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_18(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn s_alias_19(x: u64) -> u64 { x & mask(13) }

#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
