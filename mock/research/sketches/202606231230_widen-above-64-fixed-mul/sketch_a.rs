#![feature(const_unsigned_bigint_helpers)]
// Sub-problem A: 128-bit-container widening fixed-point multiply via 256-bit intermediate.
// 256-bit value held as two u128 limbs (lo, hi). Product of two u128 via carrying_mul.

// unsigned: full 256-bit product (lo, hi)
const fn umul256(a: u128, b: u128) -> (u128, u128) {
    a.carrying_mul(b, 0) // returns (low, high)
}

// 256-bit arithmetic-shift-right by FRAC, returning the low u128 (the narrowed container value).
// For the fixed-point multiply we only need the low 128 bits after the shift.
const fn shr256_lo(lo: u128, hi: u128, frac: u32) -> u128 {
    if frac == 0 {
        lo
    } else if frac < 128 {
        (lo >> frac) | (hi << (128 - frac))
    } else if frac == 128 {
        hi
    } else {
        hi >> (frac - 128)
    }
}

// Unsigned fixed-point multiply at 128-bit container.
const fn u_mul_fixed_128(a: u128, b: u128, frac: u32) -> u128 {
    let (lo, hi) = umul256(a, b);
    shr256_lo(lo, hi, frac)
}

// Signed: multiply magnitudes, arithmetic shift with floor-toward-minus-infinity correction.
// floor(x / 2^frac): if result is negative AND any shifted-out bit is nonzero, subtract 1.
const fn i_mul_fixed_128(a: i128, b: i128, frac: u32) -> i128 {
    let neg = (a < 0) != (b < 0);
    let ua = a.unsigned_abs();
    let ub = b.unsigned_abs();
    let (lo, hi) = umul256(ua, ub);
    let mag = shr256_lo(lo, hi, frac);
    if !neg {
        mag as i128
    } else {
        // floor: did we drop any low bits?
        let dropped = if frac == 0 {
            false
        } else if frac < 128 {
            (lo & ((1u128 << frac) - 1)) != 0
        } else if frac == 128 {
            lo != 0
        } else {
            // frac in 129..256: dropped if lo!=0 or low (frac-128) bits of hi nonzero
            lo != 0 || (hi & ((1u128 << (frac - 128)) - 1)) != 0
        };
        let m = mag as i128;
        if dropped { -m - 1 } else { -m }
    }
}

fn main() {
    // Catalogue target: Hot N=128 i128, raw 1<<64 squared at FRAC=30 -> 1<<98.
    const T1: i128 = i_mul_fixed_128(1<<64, 1<<64, 30);
    assert_eq!(T1, 1i128 << 98, "catalogue target 1<<98");

    // FRAC == 0 integer multiply
    const T2: i128 = i_mul_fixed_128(7, 6, 0);
    assert_eq!(T2, 42);

    // Negative floor rounding: -1 (raw) * 8 (raw) at FRAC=4 => -1/32 floors to raw -1.
    const T3: i128 = i_mul_fixed_128(-1, 8, 4);
    assert_eq!(T3, -1, "neg floor");

    // Negative product large: (-1<<64) * (1<<64) at FRAC=30 = -(1<<98)
    const T4: i128 = i_mul_fixed_128(-1<<64, 1<<64, 30);
    assert_eq!(T4, -(1i128<<98));

    // Negative with exact division (no dropped bits): -16 * 16 at FRAC=4 = -16
    const T5: i128 = i_mul_fixed_128(-16, 16, 4);
    assert_eq!(T5, -16, "neg exact");

    // Compare against the existing i128-native path for in-range values (no widen needed):
    // -1.5 * 1.0 raws at FRAC=4: -24 * 16 = -384 >> 4 = -24
    const T6: i128 = i_mul_fixed_128(-24, 16, 4);
    assert_eq!(T6, -24);

    // Unsigned target
    const U1: u128 = u_mul_fixed_128(1<<64, 1<<64, 30);
    assert_eq!(U1, 1u128 << 98);

    // FRAC == 128 corner
    const U2: u128 = u_mul_fixed_128(1<<100, 1<<100, 128);
    assert_eq!(U2, 1u128 << 72, "frac=128: (1<<200)>>128 = 1<<72");

    // FRAC > 128 corner: (1<<127)*(1<<127) = 1<<254, >>200 = 1<<54
    const U3: u128 = u_mul_fixed_128(1<<127, 1<<127, 200);
    assert_eq!(U3, 1u128 << 54, "frac>128");

    println!("ALL A PASS");
}
