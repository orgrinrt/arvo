// Probe: at MULTI-LIMB width (beyond one native register, the case that
// matters because a genuine cost asymmetry exists in principle: schoolbook
// multiplication of two 2-limb numbers needs 4 limb-products for the full
// 4-limb result but only 3 for a truncated 2-limb result, so "always
// compute the exact wide product via mul_full, then narrow" is NOT
// trivially free the way it was at native width (probe 1). Does the
// optimiser recover the cheaper, truncated-only computation once the
// composition is inlinable (shipping-shaped), the way it recovered the
// vectorised add in file 32/34's own finding?
//
// Also tried, not committed: marking `mul_full_256` #[inline(never)]
// (the check-build-shaped, axis-legible version file 34's section 1
// names). That variant genuinely does NOT fold; it pays a real call, a
// stack frame, and a return-value spill (24 lines against 7). This is
// exactly file 34's own lesson restated for multiplication: axis
// legibility and codegen quality are different questions needing
// different build shapes, and asking the codegen-quality question of a
// deliberately-uninlined function gets the wrong answer for the same
// reason asking it of a `-C lto=fat` unlinked build did there.
#![allow(dead_code)]

// direct 128x128 -> 128 truncating multiply (what "Widening::None" wants
// at a width beyond native register size: one truncated wide multiply,
// no software-visible 256-bit intermediate).
#[no_mangle]
pub extern "C" fn hot_128_direct(a: u128, b: u128) -> u128 {
    a.wrapping_mul(b)
}

// mul_full-shaped: genuinely compute the FULL 256-bit product via
// schoolbook (4 limb products over u64 halves, matching the widths-add
// discipline: I1+I2 grows every time), materialise it as a real (hi, lo)
// pair, THEN narrow (take the low 128 bits) exactly as quantize would.
fn mul_full_256(a: u128, b: u128) -> (u128, u128) {
    let (a_hi, a_lo) = ((a >> 64) as u64, a as u64);
    let (b_hi, b_lo) = ((b >> 64) as u64, b as u64);
    let lo_lo = (a_lo as u128) * (b_lo as u128);
    let lo_hi = (a_lo as u128) * (b_hi as u128);
    let hi_lo = (a_hi as u128) * (b_lo as u128);
    let hi_hi = (a_hi as u128) * (b_hi as u128);
    // schoolbook assembly into a genuine 256-bit (hi128, lo128) result
    let mid = (lo_lo >> 64) + (lo_hi & 0xFFFF_FFFF_FFFF_FFFF) + (hi_lo & 0xFFFF_FFFF_FFFF_FFFF);
    let lo = (lo_lo & 0xFFFF_FFFF_FFFF_FFFF) | (mid << 64);
    let carry = mid >> 64;
    let hi = hi_hi + (lo_hi >> 64) + (hi_lo >> 64) + carry;
    (hi, lo)
}
#[no_mangle]
pub extern "C" fn precise_mul_widens_128(a: u128, b: u128) -> (u128, u128) {
    mul_full_256(a, b)
}
#[no_mangle]
pub extern "C" fn warm_mul_via_full_then_quantize_128(a: u128, b: u128) -> u128 {
    // composite: mul_full (kept as its own function so the optimiser sees
    // exactly what a real crossing-crate call would see), then quantize
    // discards the high limb.
    mul_full_256(a, b).1
}
