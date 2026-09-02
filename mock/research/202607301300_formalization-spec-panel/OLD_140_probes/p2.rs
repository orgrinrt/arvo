// Vector shapes. 64 elements each, so counts are total cost for a fixed workload.
#![no_std]
#![allow(dead_code)]
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
const N: usize = 64;

// === W = 64. exact rung. no mask possible or needed.
#[unsafe(no_mangle)]
pub extern "C" fn v_w64_min(x: &mut [u64; N], y: &[u64; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w64_headroom(x: &mut [u128; N], y: &[u128; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((1u128 << 64) - 1);
    }
}

// === W = 13. below its rung. min container u16, 131 gives u32.
const M13U16: u16 = (1u16 << 13) - 1;
const M13U32: u32 = (1u32 << 13) - 1;
#[unsafe(no_mangle)]
pub extern "C" fn v_w13_min(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & M13U16;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w13_headroom(x: &mut [u32; N], y: &[u32; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & M13U32;
    }
}
// control: same container, no logical mask (W=16 exactly)
#[unsafe(no_mangle)]
pub extern "C" fn v_w16_min(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}

// === W = 60. below its rung. min container u64, 131 gives u128.
const M60: u64 = (1u64 << 60) - 1;
#[unsafe(no_mangle)]
pub extern "C" fn v_w60_min(x: &mut [u64; N], y: &[u64; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & M60;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w60_headroom(x: &mut [u128; N], y: &[u128; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & ((1u128 << 60) - 1);
    }
}

// === can the mask be hoisted out of a reduction? sum then mask once.
#[unsafe(no_mangle)]
pub extern "C" fn v_w13_reduce_eager(y: &[u16; N]) -> u16 {
    let mut a: u16 = 0;
    for i in 0..N {
        a = a.wrapping_add(y[i]) & M13U16;
    }
    a
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w13_reduce_lazy(y: &[u16; N]) -> u16 {
    let mut a: u16 = 0;
    for i in 0..N {
        a = a.wrapping_add(y[i]);
    }
    a & M13U16
}

// === three ops per element, mask after each vs mask once
#[unsafe(no_mangle)]
pub extern "C" fn v_w13_three_eager(x: &mut [u16; N], y: &[u16; N], z: &[u16; N]) {
    for i in 0..N {
        let a = x[i].wrapping_add(y[i]) & M13U16;
        let b = a.wrapping_mul(z[i]) & M13U16;
        x[i] = b.wrapping_sub(y[i]) & M13U16;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn v_w13_three_lazy(x: &mut [u16; N], y: &[u16; N], z: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i]
            .wrapping_add(y[i])
            .wrapping_mul(z[i])
            .wrapping_sub(y[i])
            & M13U16;
    }
}
