#![no_std]
#![allow(dead_code)]

#[unsafe(no_mangle)] pub fn cw_wrap_1(x: u64) -> u64 { x & ((1u64 << 1) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_1(x: u64) -> u64 { let m: u64 = ((1u64 << 1) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_2(x: u64) -> u64 { x & ((1u64 << 2) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_2(x: u64) -> u64 { let m: u64 = ((1u64 << 2) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_3(x: u64) -> u64 { x & ((1u64 << 3) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_3(x: u64) -> u64 { let m: u64 = ((1u64 << 3) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_4(x: u64) -> u64 { x & ((1u64 << 4) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_4(x: u64) -> u64 { let m: u64 = ((1u64 << 4) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_5(x: u64) -> u64 { x & ((1u64 << 5) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_5(x: u64) -> u64 { let m: u64 = ((1u64 << 5) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_6(x: u64) -> u64 { x & ((1u64 << 6) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_6(x: u64) -> u64 { let m: u64 = ((1u64 << 6) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_7(x: u64) -> u64 { x & ((1u64 << 7) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_7(x: u64) -> u64 { let m: u64 = ((1u64 << 7) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_8(x: u64) -> u64 { x & ((1u64 << 8) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_8(x: u64) -> u64 { let m: u64 = ((1u64 << 8) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_9(x: u64) -> u64 { x & ((1u64 << 9) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_9(x: u64) -> u64 { let m: u64 = ((1u64 << 9) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_10(x: u64) -> u64 { x & ((1u64 << 10) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_10(x: u64) -> u64 { let m: u64 = ((1u64 << 10) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_11(x: u64) -> u64 { x & ((1u64 << 11) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_11(x: u64) -> u64 { let m: u64 = ((1u64 << 11) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_12(x: u64) -> u64 { x & ((1u64 << 12) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_12(x: u64) -> u64 { let m: u64 = ((1u64 << 12) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_13(x: u64) -> u64 { x & ((1u64 << 13) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_13(x: u64) -> u64 { let m: u64 = ((1u64 << 13) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_14(x: u64) -> u64 { x & ((1u64 << 14) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_14(x: u64) -> u64 { let m: u64 = ((1u64 << 14) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_15(x: u64) -> u64 { x & ((1u64 << 15) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_15(x: u64) -> u64 { let m: u64 = ((1u64 << 15) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_16(x: u64) -> u64 { x & ((1u64 << 16) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_16(x: u64) -> u64 { let m: u64 = ((1u64 << 16) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_17(x: u64) -> u64 { x & ((1u64 << 17) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_17(x: u64) -> u64 { let m: u64 = ((1u64 << 17) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_18(x: u64) -> u64 { x & ((1u64 << 18) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_18(x: u64) -> u64 { let m: u64 = ((1u64 << 18) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_19(x: u64) -> u64 { x & ((1u64 << 19) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_19(x: u64) -> u64 { let m: u64 = ((1u64 << 19) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_20(x: u64) -> u64 { x & ((1u64 << 20) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_20(x: u64) -> u64 { let m: u64 = ((1u64 << 20) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_21(x: u64) -> u64 { x & ((1u64 << 21) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_21(x: u64) -> u64 { let m: u64 = ((1u64 << 21) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_22(x: u64) -> u64 { x & ((1u64 << 22) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_22(x: u64) -> u64 { let m: u64 = ((1u64 << 22) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_23(x: u64) -> u64 { x & ((1u64 << 23) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_23(x: u64) -> u64 { let m: u64 = ((1u64 << 23) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_24(x: u64) -> u64 { x & ((1u64 << 24) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_24(x: u64) -> u64 { let m: u64 = ((1u64 << 24) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_25(x: u64) -> u64 { x & ((1u64 << 25) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_25(x: u64) -> u64 { let m: u64 = ((1u64 << 25) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_26(x: u64) -> u64 { x & ((1u64 << 26) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_26(x: u64) -> u64 { let m: u64 = ((1u64 << 26) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_27(x: u64) -> u64 { x & ((1u64 << 27) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_27(x: u64) -> u64 { let m: u64 = ((1u64 << 27) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_28(x: u64) -> u64 { x & ((1u64 << 28) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_28(x: u64) -> u64 { let m: u64 = ((1u64 << 28) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_29(x: u64) -> u64 { x & ((1u64 << 29) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_29(x: u64) -> u64 { let m: u64 = ((1u64 << 29) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_30(x: u64) -> u64 { x & ((1u64 << 30) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_30(x: u64) -> u64 { let m: u64 = ((1u64 << 30) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_31(x: u64) -> u64 { x & ((1u64 << 31) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_31(x: u64) -> u64 { let m: u64 = ((1u64 << 31) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_32(x: u64) -> u64 { x & ((1u64 << 32) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_32(x: u64) -> u64 { let m: u64 = ((1u64 << 32) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_33(x: u64) -> u64 { x & ((1u64 << 33) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_33(x: u64) -> u64 { let m: u64 = ((1u64 << 33) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_34(x: u64) -> u64 { x & ((1u64 << 34) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_34(x: u64) -> u64 { let m: u64 = ((1u64 << 34) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_35(x: u64) -> u64 { x & ((1u64 << 35) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_35(x: u64) -> u64 { let m: u64 = ((1u64 << 35) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_36(x: u64) -> u64 { x & ((1u64 << 36) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_36(x: u64) -> u64 { let m: u64 = ((1u64 << 36) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_37(x: u64) -> u64 { x & ((1u64 << 37) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_37(x: u64) -> u64 { let m: u64 = ((1u64 << 37) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_38(x: u64) -> u64 { x & ((1u64 << 38) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_38(x: u64) -> u64 { let m: u64 = ((1u64 << 38) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_39(x: u64) -> u64 { x & ((1u64 << 39) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_39(x: u64) -> u64 { let m: u64 = ((1u64 << 39) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_40(x: u64) -> u64 { x & ((1u64 << 40) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_40(x: u64) -> u64 { let m: u64 = ((1u64 << 40) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_41(x: u64) -> u64 { x & ((1u64 << 41) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_41(x: u64) -> u64 { let m: u64 = ((1u64 << 41) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_42(x: u64) -> u64 { x & ((1u64 << 42) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_42(x: u64) -> u64 { let m: u64 = ((1u64 << 42) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_43(x: u64) -> u64 { x & ((1u64 << 43) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_43(x: u64) -> u64 { let m: u64 = ((1u64 << 43) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_44(x: u64) -> u64 { x & ((1u64 << 44) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_44(x: u64) -> u64 { let m: u64 = ((1u64 << 44) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_45(x: u64) -> u64 { x & ((1u64 << 45) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_45(x: u64) -> u64 { let m: u64 = ((1u64 << 45) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_46(x: u64) -> u64 { x & ((1u64 << 46) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_46(x: u64) -> u64 { let m: u64 = ((1u64 << 46) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_47(x: u64) -> u64 { x & ((1u64 << 47) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_47(x: u64) -> u64 { let m: u64 = ((1u64 << 47) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_48(x: u64) -> u64 { x & ((1u64 << 48) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_48(x: u64) -> u64 { let m: u64 = ((1u64 << 48) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_49(x: u64) -> u64 { x & ((1u64 << 49) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_49(x: u64) -> u64 { let m: u64 = ((1u64 << 49) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_50(x: u64) -> u64 { x & ((1u64 << 50) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_50(x: u64) -> u64 { let m: u64 = ((1u64 << 50) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_51(x: u64) -> u64 { x & ((1u64 << 51) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_51(x: u64) -> u64 { let m: u64 = ((1u64 << 51) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_52(x: u64) -> u64 { x & ((1u64 << 52) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_52(x: u64) -> u64 { let m: u64 = ((1u64 << 52) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_53(x: u64) -> u64 { x & ((1u64 << 53) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_53(x: u64) -> u64 { let m: u64 = ((1u64 << 53) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_54(x: u64) -> u64 { x & ((1u64 << 54) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_54(x: u64) -> u64 { let m: u64 = ((1u64 << 54) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_55(x: u64) -> u64 { x & ((1u64 << 55) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_55(x: u64) -> u64 { let m: u64 = ((1u64 << 55) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_56(x: u64) -> u64 { x & ((1u64 << 56) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_56(x: u64) -> u64 { let m: u64 = ((1u64 << 56) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_57(x: u64) -> u64 { x & ((1u64 << 57) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_57(x: u64) -> u64 { let m: u64 = ((1u64 << 57) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_58(x: u64) -> u64 { x & ((1u64 << 58) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_58(x: u64) -> u64 { let m: u64 = ((1u64 << 58) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_59(x: u64) -> u64 { x & ((1u64 << 59) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_59(x: u64) -> u64 { let m: u64 = ((1u64 << 59) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_60(x: u64) -> u64 { x & ((1u64 << 60) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_60(x: u64) -> u64 { let m: u64 = ((1u64 << 60) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_61(x: u64) -> u64 { x & ((1u64 << 61) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_61(x: u64) -> u64 { let m: u64 = ((1u64 << 61) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_62(x: u64) -> u64 { x & ((1u64 << 62) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_62(x: u64) -> u64 { let m: u64 = ((1u64 << 62) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_63(x: u64) -> u64 { x & ((1u64 << 63) - 1) }
#[unsafe(no_mangle)] pub fn cw_clamp_63(x: u64) -> u64 { let m: u64 = ((1u64 << 63) - 1); if x > m { m } else { x } }
#[unsafe(no_mangle)] pub fn cw_wrap_64(x: u64) -> u64 { x & !0 }
#[unsafe(no_mangle)] pub fn cw_clamp_64(x: u64) -> u64 { let m: u64 = !0; if x > m { m } else { x } }
#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
