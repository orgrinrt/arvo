//! Is precision sign-free? Two's-complement product widths, checked.
#![no_std]
#![crate_type = "lib"]

/// Stored width of a numeral with I integer digits, F fraction digits, sign bit S.
pub const fn stored(sign: u32, i: u32, f: u32) -> u32 {
    sign + i + f
}
/// Significand digits (precision), which does NOT include the sign bit.
pub const fn precision(i: u32, f: u32) -> u32 {
    i + f
}

// Unsigned Q13.3 times Q13.3.
const _: () = assert!(precision(13, 3) == 16);
const _: () = assert!(stored(0, 13, 3) == 16);
const _: () = assert!(precision(26, 6) == 32); // coordinates add
const _: () = assert!(stored(0, 26, 6) == 32); // product needs 32 bits

// Signed Q12.3 times Q12.3. Each is 15 significand digits in 16 stored bits.
const _: () = assert!(precision(12, 3) == 15);
const _: () = assert!(stored(1, 12, 3) == 16);
const _: () = assert!(precision(24, 6) == 30); // coordinates add, same law
const _: () = assert!(stored(1, 24, 6) == 31); // ONE sign bit, not two

// The width-keyed law R == P + Q, applied to the STORED widths, demands 32.
const _: () = assert!(16 + 16 == 32);
// The true stored width of the signed product is 31. The law is off by one,
// and one is the rung boundary at 64: 1 + 31 + 32 = 64 fits u64, 32 + 33 = 65 does not.
pub const fn rung(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else if w <= 64 {
        64
    } else {
        128
    }
}
const _: () = assert!(rung(1 + 31 + 32) == 64);
const _: () = assert!(rung(32 + 33) == 128);
