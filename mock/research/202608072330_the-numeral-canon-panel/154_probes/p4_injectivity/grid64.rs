#![no_std]
#![allow(dead_code)]

pub const fn mask(w: u32) -> u64 { if w >= 64 { u64::MAX } else { (1u64 << w) - 1 } }

#[unsafe(no_mangle)] pub fn g_wrap_1(x: u64) -> u64 { x & mask(1) }
#[unsafe(no_mangle)] pub fn g_clamp_1(x: u64) -> u64 { { let m = mask(1); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_2(x: u64) -> u64 { x & mask(2) }
#[unsafe(no_mangle)] pub fn g_clamp_2(x: u64) -> u64 { { let m = mask(2); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_3(x: u64) -> u64 { x & mask(3) }
#[unsafe(no_mangle)] pub fn g_clamp_3(x: u64) -> u64 { { let m = mask(3); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_4(x: u64) -> u64 { x & mask(4) }
#[unsafe(no_mangle)] pub fn g_clamp_4(x: u64) -> u64 { { let m = mask(4); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_5(x: u64) -> u64 { x & mask(5) }
#[unsafe(no_mangle)] pub fn g_clamp_5(x: u64) -> u64 { { let m = mask(5); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_6(x: u64) -> u64 { x & mask(6) }
#[unsafe(no_mangle)] pub fn g_clamp_6(x: u64) -> u64 { { let m = mask(6); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_7(x: u64) -> u64 { x & mask(7) }
#[unsafe(no_mangle)] pub fn g_clamp_7(x: u64) -> u64 { { let m = mask(7); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_8(x: u64) -> u64 { x & mask(8) }
#[unsafe(no_mangle)] pub fn g_clamp_8(x: u64) -> u64 { { let m = mask(8); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_9(x: u64) -> u64 { x & mask(9) }
#[unsafe(no_mangle)] pub fn g_clamp_9(x: u64) -> u64 { { let m = mask(9); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_10(x: u64) -> u64 { x & mask(10) }
#[unsafe(no_mangle)] pub fn g_clamp_10(x: u64) -> u64 { { let m = mask(10); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_11(x: u64) -> u64 { x & mask(11) }
#[unsafe(no_mangle)] pub fn g_clamp_11(x: u64) -> u64 { { let m = mask(11); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_12(x: u64) -> u64 { x & mask(12) }
#[unsafe(no_mangle)] pub fn g_clamp_12(x: u64) -> u64 { { let m = mask(12); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_13(x: u64) -> u64 { x & mask(13) }
#[unsafe(no_mangle)] pub fn g_clamp_13(x: u64) -> u64 { { let m = mask(13); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_14(x: u64) -> u64 { x & mask(14) }
#[unsafe(no_mangle)] pub fn g_clamp_14(x: u64) -> u64 { { let m = mask(14); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_15(x: u64) -> u64 { x & mask(15) }
#[unsafe(no_mangle)] pub fn g_clamp_15(x: u64) -> u64 { { let m = mask(15); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_16(x: u64) -> u64 { x & mask(16) }
#[unsafe(no_mangle)] pub fn g_clamp_16(x: u64) -> u64 { { let m = mask(16); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_17(x: u64) -> u64 { x & mask(17) }
#[unsafe(no_mangle)] pub fn g_clamp_17(x: u64) -> u64 { { let m = mask(17); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_18(x: u64) -> u64 { x & mask(18) }
#[unsafe(no_mangle)] pub fn g_clamp_18(x: u64) -> u64 { { let m = mask(18); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_19(x: u64) -> u64 { x & mask(19) }
#[unsafe(no_mangle)] pub fn g_clamp_19(x: u64) -> u64 { { let m = mask(19); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_20(x: u64) -> u64 { x & mask(20) }
#[unsafe(no_mangle)] pub fn g_clamp_20(x: u64) -> u64 { { let m = mask(20); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_21(x: u64) -> u64 { x & mask(21) }
#[unsafe(no_mangle)] pub fn g_clamp_21(x: u64) -> u64 { { let m = mask(21); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_22(x: u64) -> u64 { x & mask(22) }
#[unsafe(no_mangle)] pub fn g_clamp_22(x: u64) -> u64 { { let m = mask(22); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_23(x: u64) -> u64 { x & mask(23) }
#[unsafe(no_mangle)] pub fn g_clamp_23(x: u64) -> u64 { { let m = mask(23); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_24(x: u64) -> u64 { x & mask(24) }
#[unsafe(no_mangle)] pub fn g_clamp_24(x: u64) -> u64 { { let m = mask(24); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_25(x: u64) -> u64 { x & mask(25) }
#[unsafe(no_mangle)] pub fn g_clamp_25(x: u64) -> u64 { { let m = mask(25); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_26(x: u64) -> u64 { x & mask(26) }
#[unsafe(no_mangle)] pub fn g_clamp_26(x: u64) -> u64 { { let m = mask(26); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_27(x: u64) -> u64 { x & mask(27) }
#[unsafe(no_mangle)] pub fn g_clamp_27(x: u64) -> u64 { { let m = mask(27); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_28(x: u64) -> u64 { x & mask(28) }
#[unsafe(no_mangle)] pub fn g_clamp_28(x: u64) -> u64 { { let m = mask(28); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_29(x: u64) -> u64 { x & mask(29) }
#[unsafe(no_mangle)] pub fn g_clamp_29(x: u64) -> u64 { { let m = mask(29); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_30(x: u64) -> u64 { x & mask(30) }
#[unsafe(no_mangle)] pub fn g_clamp_30(x: u64) -> u64 { { let m = mask(30); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_31(x: u64) -> u64 { x & mask(31) }
#[unsafe(no_mangle)] pub fn g_clamp_31(x: u64) -> u64 { { let m = mask(31); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_32(x: u64) -> u64 { x & mask(32) }
#[unsafe(no_mangle)] pub fn g_clamp_32(x: u64) -> u64 { { let m = mask(32); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_33(x: u64) -> u64 { x & mask(33) }
#[unsafe(no_mangle)] pub fn g_clamp_33(x: u64) -> u64 { { let m = mask(33); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_34(x: u64) -> u64 { x & mask(34) }
#[unsafe(no_mangle)] pub fn g_clamp_34(x: u64) -> u64 { { let m = mask(34); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_35(x: u64) -> u64 { x & mask(35) }
#[unsafe(no_mangle)] pub fn g_clamp_35(x: u64) -> u64 { { let m = mask(35); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_36(x: u64) -> u64 { x & mask(36) }
#[unsafe(no_mangle)] pub fn g_clamp_36(x: u64) -> u64 { { let m = mask(36); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_37(x: u64) -> u64 { x & mask(37) }
#[unsafe(no_mangle)] pub fn g_clamp_37(x: u64) -> u64 { { let m = mask(37); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_38(x: u64) -> u64 { x & mask(38) }
#[unsafe(no_mangle)] pub fn g_clamp_38(x: u64) -> u64 { { let m = mask(38); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_39(x: u64) -> u64 { x & mask(39) }
#[unsafe(no_mangle)] pub fn g_clamp_39(x: u64) -> u64 { { let m = mask(39); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_40(x: u64) -> u64 { x & mask(40) }
#[unsafe(no_mangle)] pub fn g_clamp_40(x: u64) -> u64 { { let m = mask(40); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_41(x: u64) -> u64 { x & mask(41) }
#[unsafe(no_mangle)] pub fn g_clamp_41(x: u64) -> u64 { { let m = mask(41); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_42(x: u64) -> u64 { x & mask(42) }
#[unsafe(no_mangle)] pub fn g_clamp_42(x: u64) -> u64 { { let m = mask(42); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_43(x: u64) -> u64 { x & mask(43) }
#[unsafe(no_mangle)] pub fn g_clamp_43(x: u64) -> u64 { { let m = mask(43); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_44(x: u64) -> u64 { x & mask(44) }
#[unsafe(no_mangle)] pub fn g_clamp_44(x: u64) -> u64 { { let m = mask(44); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_45(x: u64) -> u64 { x & mask(45) }
#[unsafe(no_mangle)] pub fn g_clamp_45(x: u64) -> u64 { { let m = mask(45); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_46(x: u64) -> u64 { x & mask(46) }
#[unsafe(no_mangle)] pub fn g_clamp_46(x: u64) -> u64 { { let m = mask(46); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_47(x: u64) -> u64 { x & mask(47) }
#[unsafe(no_mangle)] pub fn g_clamp_47(x: u64) -> u64 { { let m = mask(47); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_48(x: u64) -> u64 { x & mask(48) }
#[unsafe(no_mangle)] pub fn g_clamp_48(x: u64) -> u64 { { let m = mask(48); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_49(x: u64) -> u64 { x & mask(49) }
#[unsafe(no_mangle)] pub fn g_clamp_49(x: u64) -> u64 { { let m = mask(49); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_50(x: u64) -> u64 { x & mask(50) }
#[unsafe(no_mangle)] pub fn g_clamp_50(x: u64) -> u64 { { let m = mask(50); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_51(x: u64) -> u64 { x & mask(51) }
#[unsafe(no_mangle)] pub fn g_clamp_51(x: u64) -> u64 { { let m = mask(51); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_52(x: u64) -> u64 { x & mask(52) }
#[unsafe(no_mangle)] pub fn g_clamp_52(x: u64) -> u64 { { let m = mask(52); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_53(x: u64) -> u64 { x & mask(53) }
#[unsafe(no_mangle)] pub fn g_clamp_53(x: u64) -> u64 { { let m = mask(53); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_54(x: u64) -> u64 { x & mask(54) }
#[unsafe(no_mangle)] pub fn g_clamp_54(x: u64) -> u64 { { let m = mask(54); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_55(x: u64) -> u64 { x & mask(55) }
#[unsafe(no_mangle)] pub fn g_clamp_55(x: u64) -> u64 { { let m = mask(55); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_56(x: u64) -> u64 { x & mask(56) }
#[unsafe(no_mangle)] pub fn g_clamp_56(x: u64) -> u64 { { let m = mask(56); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_57(x: u64) -> u64 { x & mask(57) }
#[unsafe(no_mangle)] pub fn g_clamp_57(x: u64) -> u64 { { let m = mask(57); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_58(x: u64) -> u64 { x & mask(58) }
#[unsafe(no_mangle)] pub fn g_clamp_58(x: u64) -> u64 { { let m = mask(58); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_59(x: u64) -> u64 { x & mask(59) }
#[unsafe(no_mangle)] pub fn g_clamp_59(x: u64) -> u64 { { let m = mask(59); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_60(x: u64) -> u64 { x & mask(60) }
#[unsafe(no_mangle)] pub fn g_clamp_60(x: u64) -> u64 { { let m = mask(60); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_61(x: u64) -> u64 { x & mask(61) }
#[unsafe(no_mangle)] pub fn g_clamp_61(x: u64) -> u64 { { let m = mask(61); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_62(x: u64) -> u64 { x & mask(62) }
#[unsafe(no_mangle)] pub fn g_clamp_62(x: u64) -> u64 { { let m = mask(62); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_63(x: u64) -> u64 { x & mask(63) }
#[unsafe(no_mangle)] pub fn g_clamp_63(x: u64) -> u64 { { let m = mask(63); if x > m { m } else { x } } }
#[unsafe(no_mangle)] pub fn g_wrap_64(x: u64) -> u64 { x & mask(64) }
#[unsafe(no_mangle)] pub fn g_clamp_64(x: u64) -> u64 { { let m = mask(64); if x > m { m } else { x } } }
#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
