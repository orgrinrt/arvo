// Warm's wrapping semantics at three widths, under 131's rung-of-headroom rule
// and under the no-container-headroom rule. Instruction counts decide.
#![no_std]
#![allow(dead_code)]
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --- W = 64, exactly fills its rung. 131 gives Warm a u128; the alternative is u64.
#[unsafe(no_mangle)]
pub extern "C" fn w64_headroom(a: u128, b: u128) -> u128 {
    (a.wrapping_add(b)) & ((1u128 << 64) - 1) // logical wrap at 64 inside a u128
}
#[unsafe(no_mangle)]
pub extern "C" fn w64_native(a: u64, b: u64) -> u64 {
    a.wrapping_add(b) // the container's wrap IS the wrap at 64
}

// --- W = 13, below its rung. 131 gives Warm a u32; the alternative is u16.
#[unsafe(no_mangle)]
pub extern "C" fn w13_headroom(a: u32, b: u32) -> u32 {
    (a.wrapping_add(b)) & ((1u32 << 13) - 1)
}
#[unsafe(no_mangle)]
pub extern "C" fn w13_native(a: u16, b: u16) -> u16 {
    (a.wrapping_add(b)) & ((1u16 << 13) - 1) // spare bits: the add cannot lose a bit first
}

// --- the case that motivated the complaint: a slice of them, autovectorised.
#[unsafe(no_mangle)]
pub extern "C" fn w64_headroom_vec(x: &mut [u128; 64], y: &[u128; 64]) {
    for i in 0..64 {
        x[i] = x[i].wrapping_add(y[i]) & ((1u128 << 64) - 1);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn w64_native_vec(x: &mut [u64; 64], y: &[u64; 64]) {
    for i in 0..64 {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
