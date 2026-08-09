// What wrapping at logical width W requires on the container it lands in.
#![no_std]
#![allow(dead_code)]
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// ---- W exactly a rung: the container's own wrap IS the wrap at W.
#[unsafe(no_mangle)]
pub extern "C" fn exact64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub extern "C" fn exact16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub extern "C" fn exact8(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

// ---- W below a rung: mask after the op.
const M13: u16 = (1u16 << 13) - 1;
#[unsafe(no_mangle)]
pub extern "C" fn sub13(a: u16, b: u16) -> u16 {
    a.wrapping_add(b) & M13
}
const M60: u64 = (1u64 << 60) - 1;
#[unsafe(no_mangle)]
pub extern "C" fn sub60(a: u64, b: u64) -> u64 {
    a.wrapping_add(b) & M60
}

// ---- 131's rule for comparison: W=13 in a u32, W=60 in a u128.
#[unsafe(no_mangle)]
pub extern "C" fn hr13(a: u32, b: u32) -> u32 {
    a.wrapping_add(b) & ((1u32 << 13) - 1)
}
#[unsafe(no_mangle)]
pub extern "C" fn hr60(a: u128, b: u128) -> u128 {
    a.wrapping_add(b) & ((1u128 << 60) - 1)
}

// ---- is the mask a homomorphism? chain of four adds, masked every step
#[unsafe(no_mangle)]
pub extern "C" fn chain13_eager(a: u16, b: u16, c: u16, d: u16) -> u16 {
    let x = a.wrapping_add(b) & M13;
    let y = x.wrapping_add(c) & M13;
    let z = y.wrapping_add(d) & M13;
    z
}
// ---- the same chain masked once at the end
#[unsafe(no_mangle)]
pub extern "C" fn chain13_lazy(a: u16, b: u16, c: u16, d: u16) -> u16 {
    a.wrapping_add(b).wrapping_add(c).wrapping_add(d) & M13
}
// ---- mixed add/sub/mul chain, eager vs lazy
#[unsafe(no_mangle)]
pub extern "C" fn mix13_eager(a: u16, b: u16, c: u16) -> u16 {
    let x = a.wrapping_add(b) & M13;
    let y = x.wrapping_mul(c) & M13;
    let z = y.wrapping_sub(a) & M13;
    z
}
#[unsafe(no_mangle)]
pub extern "C" fn mix13_lazy(a: u16, b: u16, c: u16) -> u16 {
    a.wrapping_add(b).wrapping_mul(c).wrapping_sub(a) & M13
}
// ---- an observation that needs the canonical form: comparison
#[unsafe(no_mangle)]
pub extern "C" fn cmp13(a: u16, b: u16, c: u16) -> bool {
    (a.wrapping_add(b) & M13) < c
}
// ---- and one where the mask is unobservable: low-bit extract
#[unsafe(no_mangle)]
pub extern "C" fn low13(a: u16, b: u16) -> u16 {
    (a.wrapping_add(b) & M13) & 0x7
}
