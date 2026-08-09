#![no_std]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// hot_128_direct: native truncating 128-bit multiply.
#[no_mangle]
pub fn hot_128_direct(a: u128, b: u128) -> u128 {
    a.wrapping_mul(b)
}

// warm_mul_via_full_then_quantize_128: schoolbook limb multiplication
// computing the genuine 256-bit product (mul_full's shape at 2-limb
// width), truncated back to 128 bits (quantize_wrap's shape). Four
// limb products (lo*lo, lo*hi, hi*lo, hi*hi) named explicitly so the
// optimiser has to prove hi_hi and its carries are dead, not merely
// see through a single hardware op.
#[no_mangle]
pub fn warm_mul_via_full_then_quantize_128(a: u128, b: u128) -> u128 {
    let a_lo = a as u64 as u128;
    let a_hi = (a >> 64) as u64 as u128;
    let b_lo = b as u64 as u128;
    let b_hi = (b >> 64) as u64 as u128;

    let lo_lo = a_lo * b_lo;
    let lo_hi = a_lo * b_hi;
    let hi_lo = a_hi * b_lo;
    let hi_hi = a_hi * b_hi;

    // full 256-bit product, four limbs: (lo_lo, mid, hi_hi-carry-adjusted, ...)
    let mid = (lo_lo >> 64) + (lo_hi & 0xFFFF_FFFF_FFFF_FFFF) + (hi_lo & 0xFFFF_FFFF_FFFF_FFFF);
    let low128 = (lo_lo & 0xFFFF_FFFF_FFFF_FFFF) | (mid << 64);
    let _upper_would_be = hi_hi + (lo_hi >> 64) + (hi_lo >> 64) + (mid >> 64);
    // truncate: the design's quantizer keeps only the low 128 bits, so
    // _upper_would_be is computed (to be a real mul_full call) and thrown away.
    low128
}
