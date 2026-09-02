#![no_std]
#![allow(dead_code)]

#[unsafe(no_mangle)]
pub fn cm_wrap_1(x: u8) -> u8 {
    x & ((1u8 << 1) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_1(x: u8) -> u8 {
    let m: u8 = ((1u8 << 1) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_2(x: u8) -> u8 {
    x & ((1u8 << 2) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_2(x: u8) -> u8 {
    let m: u8 = ((1u8 << 2) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_3(x: u8) -> u8 {
    x & ((1u8 << 3) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_3(x: u8) -> u8 {
    let m: u8 = ((1u8 << 3) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_4(x: u8) -> u8 {
    x & ((1u8 << 4) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_4(x: u8) -> u8 {
    let m: u8 = ((1u8 << 4) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_5(x: u8) -> u8 {
    x & ((1u8 << 5) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_5(x: u8) -> u8 {
    let m: u8 = ((1u8 << 5) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_6(x: u8) -> u8 {
    x & ((1u8 << 6) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_6(x: u8) -> u8 {
    let m: u8 = ((1u8 << 6) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_7(x: u8) -> u8 {
    x & ((1u8 << 7) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_7(x: u8) -> u8 {
    let m: u8 = ((1u8 << 7) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_8(x: u8) -> u8 {
    x & !0
}
#[unsafe(no_mangle)]
pub fn cm_clamp_8(x: u8) -> u8 {
    let m: u8 = !0;
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_9(x: u16) -> u16 {
    x & ((1u16 << 9) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_9(x: u16) -> u16 {
    let m: u16 = ((1u16 << 9) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_10(x: u16) -> u16 {
    x & ((1u16 << 10) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_10(x: u16) -> u16 {
    let m: u16 = ((1u16 << 10) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_11(x: u16) -> u16 {
    x & ((1u16 << 11) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_11(x: u16) -> u16 {
    let m: u16 = ((1u16 << 11) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_12(x: u16) -> u16 {
    x & ((1u16 << 12) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_12(x: u16) -> u16 {
    let m: u16 = ((1u16 << 12) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_13(x: u16) -> u16 {
    x & ((1u16 << 13) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_13(x: u16) -> u16 {
    let m: u16 = ((1u16 << 13) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_14(x: u16) -> u16 {
    x & ((1u16 << 14) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_14(x: u16) -> u16 {
    let m: u16 = ((1u16 << 14) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_15(x: u16) -> u16 {
    x & ((1u16 << 15) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_15(x: u16) -> u16 {
    let m: u16 = ((1u16 << 15) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_16(x: u16) -> u16 {
    x & !0
}
#[unsafe(no_mangle)]
pub fn cm_clamp_16(x: u16) -> u16 {
    let m: u16 = !0;
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_17(x: u32) -> u32 {
    x & ((1u32 << 17) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_17(x: u32) -> u32 {
    let m: u32 = ((1u32 << 17) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_18(x: u32) -> u32 {
    x & ((1u32 << 18) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_18(x: u32) -> u32 {
    let m: u32 = ((1u32 << 18) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_19(x: u32) -> u32 {
    x & ((1u32 << 19) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_19(x: u32) -> u32 {
    let m: u32 = ((1u32 << 19) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_20(x: u32) -> u32 {
    x & ((1u32 << 20) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_20(x: u32) -> u32 {
    let m: u32 = ((1u32 << 20) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_21(x: u32) -> u32 {
    x & ((1u32 << 21) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_21(x: u32) -> u32 {
    let m: u32 = ((1u32 << 21) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_22(x: u32) -> u32 {
    x & ((1u32 << 22) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_22(x: u32) -> u32 {
    let m: u32 = ((1u32 << 22) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_23(x: u32) -> u32 {
    x & ((1u32 << 23) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_23(x: u32) -> u32 {
    let m: u32 = ((1u32 << 23) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_24(x: u32) -> u32 {
    x & ((1u32 << 24) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_24(x: u32) -> u32 {
    let m: u32 = ((1u32 << 24) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_25(x: u32) -> u32 {
    x & ((1u32 << 25) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_25(x: u32) -> u32 {
    let m: u32 = ((1u32 << 25) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_26(x: u32) -> u32 {
    x & ((1u32 << 26) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_26(x: u32) -> u32 {
    let m: u32 = ((1u32 << 26) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_27(x: u32) -> u32 {
    x & ((1u32 << 27) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_27(x: u32) -> u32 {
    let m: u32 = ((1u32 << 27) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_28(x: u32) -> u32 {
    x & ((1u32 << 28) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_28(x: u32) -> u32 {
    let m: u32 = ((1u32 << 28) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_29(x: u32) -> u32 {
    x & ((1u32 << 29) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_29(x: u32) -> u32 {
    let m: u32 = ((1u32 << 29) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_30(x: u32) -> u32 {
    x & ((1u32 << 30) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_30(x: u32) -> u32 {
    let m: u32 = ((1u32 << 30) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_31(x: u32) -> u32 {
    x & ((1u32 << 31) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_31(x: u32) -> u32 {
    let m: u32 = ((1u32 << 31) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_32(x: u32) -> u32 {
    x & !0
}
#[unsafe(no_mangle)]
pub fn cm_clamp_32(x: u32) -> u32 {
    let m: u32 = !0;
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_33(x: u64) -> u64 {
    x & ((1u64 << 33) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_33(x: u64) -> u64 {
    let m: u64 = ((1u64 << 33) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_34(x: u64) -> u64 {
    x & ((1u64 << 34) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_34(x: u64) -> u64 {
    let m: u64 = ((1u64 << 34) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_35(x: u64) -> u64 {
    x & ((1u64 << 35) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_35(x: u64) -> u64 {
    let m: u64 = ((1u64 << 35) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_36(x: u64) -> u64 {
    x & ((1u64 << 36) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_36(x: u64) -> u64 {
    let m: u64 = ((1u64 << 36) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_37(x: u64) -> u64 {
    x & ((1u64 << 37) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_37(x: u64) -> u64 {
    let m: u64 = ((1u64 << 37) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_38(x: u64) -> u64 {
    x & ((1u64 << 38) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_38(x: u64) -> u64 {
    let m: u64 = ((1u64 << 38) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_39(x: u64) -> u64 {
    x & ((1u64 << 39) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_39(x: u64) -> u64 {
    let m: u64 = ((1u64 << 39) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_40(x: u64) -> u64 {
    x & ((1u64 << 40) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_40(x: u64) -> u64 {
    let m: u64 = ((1u64 << 40) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_41(x: u64) -> u64 {
    x & ((1u64 << 41) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_41(x: u64) -> u64 {
    let m: u64 = ((1u64 << 41) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_42(x: u64) -> u64 {
    x & ((1u64 << 42) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_42(x: u64) -> u64 {
    let m: u64 = ((1u64 << 42) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_43(x: u64) -> u64 {
    x & ((1u64 << 43) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_43(x: u64) -> u64 {
    let m: u64 = ((1u64 << 43) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_44(x: u64) -> u64 {
    x & ((1u64 << 44) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_44(x: u64) -> u64 {
    let m: u64 = ((1u64 << 44) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_45(x: u64) -> u64 {
    x & ((1u64 << 45) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_45(x: u64) -> u64 {
    let m: u64 = ((1u64 << 45) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_46(x: u64) -> u64 {
    x & ((1u64 << 46) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_46(x: u64) -> u64 {
    let m: u64 = ((1u64 << 46) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_47(x: u64) -> u64 {
    x & ((1u64 << 47) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_47(x: u64) -> u64 {
    let m: u64 = ((1u64 << 47) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_48(x: u64) -> u64 {
    x & ((1u64 << 48) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_48(x: u64) -> u64 {
    let m: u64 = ((1u64 << 48) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_49(x: u64) -> u64 {
    x & ((1u64 << 49) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_49(x: u64) -> u64 {
    let m: u64 = ((1u64 << 49) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_50(x: u64) -> u64 {
    x & ((1u64 << 50) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_50(x: u64) -> u64 {
    let m: u64 = ((1u64 << 50) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_51(x: u64) -> u64 {
    x & ((1u64 << 51) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_51(x: u64) -> u64 {
    let m: u64 = ((1u64 << 51) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_52(x: u64) -> u64 {
    x & ((1u64 << 52) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_52(x: u64) -> u64 {
    let m: u64 = ((1u64 << 52) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_53(x: u64) -> u64 {
    x & ((1u64 << 53) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_53(x: u64) -> u64 {
    let m: u64 = ((1u64 << 53) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_54(x: u64) -> u64 {
    x & ((1u64 << 54) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_54(x: u64) -> u64 {
    let m: u64 = ((1u64 << 54) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_55(x: u64) -> u64 {
    x & ((1u64 << 55) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_55(x: u64) -> u64 {
    let m: u64 = ((1u64 << 55) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_56(x: u64) -> u64 {
    x & ((1u64 << 56) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_56(x: u64) -> u64 {
    let m: u64 = ((1u64 << 56) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_57(x: u64) -> u64 {
    x & ((1u64 << 57) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_57(x: u64) -> u64 {
    let m: u64 = ((1u64 << 57) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_58(x: u64) -> u64 {
    x & ((1u64 << 58) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_58(x: u64) -> u64 {
    let m: u64 = ((1u64 << 58) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_59(x: u64) -> u64 {
    x & ((1u64 << 59) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_59(x: u64) -> u64 {
    let m: u64 = ((1u64 << 59) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_60(x: u64) -> u64 {
    x & ((1u64 << 60) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_60(x: u64) -> u64 {
    let m: u64 = ((1u64 << 60) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_61(x: u64) -> u64 {
    x & ((1u64 << 61) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_61(x: u64) -> u64 {
    let m: u64 = ((1u64 << 61) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_62(x: u64) -> u64 {
    x & ((1u64 << 62) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_62(x: u64) -> u64 {
    let m: u64 = ((1u64 << 62) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_63(x: u64) -> u64 {
    x & ((1u64 << 63) - 1)
}
#[unsafe(no_mangle)]
pub fn cm_clamp_63(x: u64) -> u64 {
    let m: u64 = ((1u64 << 63) - 1);
    if x > m {
        m
    } else {
        x
    }
}
#[unsafe(no_mangle)]
pub fn cm_wrap_64(x: u64) -> u64 {
    x & !0
}
#[unsafe(no_mangle)]
pub fn cm_clamp_64(x: u64) -> u64 {
    let m: u64 = !0;
    if x > m {
        m
    } else {
        x
    }
}
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
