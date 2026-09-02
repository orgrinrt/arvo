// Premise attack: the mask exists because the numeral sits at the BOTTOM of the
// container. If it sits at the TOP, the container's own wrap is the wrap at W and
// no mask is needed for + or -. What does that cost elsewhere?
#![no_std]
#![allow(dead_code)]
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
const N: usize = 64;
const S: u32 = 3; // W = 13 in a u16: shift = 16 - 13
const M13: u16 = (1u16 << 13) - 1;

// low-aligned (the design's current representation) with the mask
#[unsafe(no_mangle)]
pub extern "C" fn lo_add(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]) & M13;
    }
}
// high-aligned: no mask at all
#[unsafe(no_mangle)]
pub extern "C" fn hi_add(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
// high-aligned multiply: one extra shift to re-normalise
#[unsafe(no_mangle)]
pub extern "C" fn hi_mul(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_mul(y[i] >> S);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn lo_mul(x: &mut [u16; N], y: &[u16; N]) {
    for i in 0..N {
        x[i] = x[i].wrapping_mul(y[i]) & M13;
    }
}
// high-aligned observation: every read out of the type pays a shift
#[unsafe(no_mangle)]
pub extern "C" fn hi_read(x: &[u16; N], o: &mut [u16; N]) {
    for i in 0..N {
        o[i] = x[i] >> S;
    }
}
// high-aligned right shift by k: needs no correction (bits fall off the bottom)
// high-aligned unsigned compare: direct
#[unsafe(no_mangle)]
pub extern "C" fn hi_cmp(x: &[u16; N], y: &[u16; N], o: &mut [u16; N]) {
    for i in 0..N {
        o[i] = if x[i] < y[i] { 1 } else { 0 };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn lo_cmp(x: &[u16; N], y: &[u16; N], o: &mut [u16; N]) {
    for i in 0..N {
        o[i] = if x[i] < y[i] { 1 } else { 0 };
    }
}
