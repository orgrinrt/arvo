#![no_std]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// hot_mul_direct: a truncating native multiply, no widening anywhere.
#[no_mangle]
pub fn hot_mul_direct(a: u64, b: u64) -> u64 {
    a.wrapping_mul(b)
}

// hot_mul_via_full_then_quantize: compute the exact 128-bit product
// (mul_full's shape: widen, multiply, name the full-width numeral),
// then truncate to the low 64 bits (quantize_wrap's shape).
#[no_mangle]
pub fn hot_mul_via_full_then_quantize(a: u64, b: u64) -> u64 {
    let full: u128 = (a as u128) * (b as u128);
    full as u64
}
