// Precise saturates. Saturation at W in a container of width C is a different
// obligation from wrapping at W. Does it need the headroom rung?
#![no_std]
#![allow(dead_code)]
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
const N: usize = 64;

// --- unsigned, W exactly a rung: hardware saturating add.
#[unsafe(no_mangle)]
pub extern "C" fn s_u64_exact(a: u64, b: u64) -> u64 {
    a.saturating_add(b)
}
// --- unsigned, W below rung, MIN container: spare bits make a+b exact, then clamp.
const M13: u16 = (1u16 << 13) - 1;
#[unsafe(no_mangle)]
pub extern "C" fn s_u13_min(a: u16, b: u16) -> u16 {
    let s = a.wrapping_add(b);
    if s > M13 {
        M13
    } else {
        s
    }
}
// --- unsigned, W below rung, 131's headroom container.
const M13W: u32 = (1u32 << 13) - 1;
#[unsafe(no_mangle)]
pub extern "C" fn s_u13_headroom(a: u32, b: u32) -> u32 {
    let s = a.wrapping_add(b);
    if s > M13W {
        M13W
    } else {
        s
    }
}
// --- unsigned W=64 under 131's rule: u128 container, clamp at 2^64-1.
#[unsafe(no_mangle)]
pub extern "C" fn s_u64_headroom(a: u128, b: u128) -> u128 {
    let s = a.wrapping_add(b);
    let m = (1u128 << 64) - 1;
    if s > m {
        m
    } else {
        s
    }
}

// --- SIGNED. wrapping at W in a wider container needs sign extension, not masking.
#[unsafe(no_mangle)]
pub extern "C" fn w_i13_min(a: i16, b: i16) -> i16 {
    let s = a.wrapping_add(b);
    (s << 3) >> 3
} // sign-extend from bit 12
#[unsafe(no_mangle)]
pub extern "C" fn w_i16_exact(a: i16, b: i16) -> i16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub extern "C" fn w_i13_headroom(a: i32, b: i32) -> i32 {
    let s = a.wrapping_add(b);
    (s << 19) >> 19
}
// --- signed saturation at W, min container.
const IMAX13: i16 = (1i16 << 12) - 1;
const IMIN13: i16 = -(1i16 << 12);
#[unsafe(no_mangle)]
pub extern "C" fn s_i13_min(a: i16, b: i16) -> i16 {
    let s = a.wrapping_add(b);
    if s > IMAX13 {
        IMAX13
    } else if s < IMIN13 {
        IMIN13
    } else {
        s
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn s_i64_exact(a: i64, b: i64) -> i64 {
    a.saturating_add(b)
}

// --- vector forms, 64 elements. does saturation vectorise at each container?
#[unsafe(no_mangle)]
pub extern "C" fn vs_u13_min(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        let s = x[i].wrapping_add(y[i]);
        x[i] = if s > M13 { M13 } else { s };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vs_u13_headroom(x: &mut [u32; N], y: &[u32; N]) {
    for i in 0..N {
        let s = x[i].wrapping_add(y[i]);
        x[i] = if s > M13W { M13W } else { s };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vs_u64_exact(x: &mut [u64; N], y: &[u64; N]) {
    for i in 0..N {
        x[i] = x[i].saturating_add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vs_u64_headroom(x: &mut [u128; N], y: &[u128; N]) {
    for i in 0..N {
        let s = x[i].wrapping_add(y[i]);
        let m = (1u128 << 64) - 1;
        x[i] = if s > m { m } else { s };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vw_i13_min(x: &mut [i16; N], y: &[i16; N]) {
    for i in 0..N {
        let s = x[i].wrapping_add(y[i]);
        x[i] = (s << 3) >> 3;
    }
}
