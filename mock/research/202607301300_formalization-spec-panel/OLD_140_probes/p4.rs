// Boundary sweep: W one below a rung, exactly a rung, one above a rung,
// and the crossing into the wide region. 64 elements per loop.
#![no_std]
#![allow(dead_code)]
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
const N: usize = 64;
#[unsafe(no_mangle)]
pub extern "C" fn v_w1(x: &mut [u8; N], y: &[u8; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u8::MAX) >> (8 - 1));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w7(x: &mut [u8; N], y: &[u8; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u8::MAX) >> (8 - 7));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w8(x: &mut [u8; N], y: &[u8; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w9(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u16::MAX) >> (16 - 9));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w15(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u16::MAX) >> (16 - 15));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w16(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w17(x: &mut [u32; N], y: &[u32; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u32::MAX) >> (32 - 17));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w31(x: &mut [u32; N], y: &[u32; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u32::MAX) >> (32 - 31));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w32(x: &mut [u32; N], y: &[u32; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w33(x: &mut [u64; N], y: &[u64; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u64::MAX) >> (64 - 33));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w63(x: &mut [u64; N], y: &[u64; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u64::MAX) >> (64 - 63));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w64(x: &mut [u64; N], y: &[u64; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w65(x: &mut [u128; N], y: &[u128; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u128::MAX) >> (128 - 65));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w127(x: &mut [u128; N], y: &[u128; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((u128::MAX) >> (128 - 127));
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w128(x: &mut [u128; N], y: &[u128; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
