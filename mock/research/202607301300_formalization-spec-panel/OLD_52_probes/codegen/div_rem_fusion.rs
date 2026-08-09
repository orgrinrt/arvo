#![no_std]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// div_floor and rem, called together on the same operands, the shape a
// consumer computing modular indexing (floor quotient + residue) writes.
// If the optimiser fuses them into one hardware divide, only one
// division-family instruction (sdiv, or udiv/msub pair) should appear
// rather than two independent division sequences.
#[no_mangle]
pub fn div_floor(a: i64, b: i64) -> i64 {
    let q = a.wrapping_div(b);
    let r = a.wrapping_rem(b);
    if (r != 0) && ((r < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

#[no_mangle]
pub fn rem_euclid(a: i64, b: i64) -> i64 {
    let r = a.wrapping_rem(b);
    if r < 0 {
        r + b.wrapping_abs()
    } else {
        r
    }
}

#[no_mangle]
pub fn div_floor_and_rem(a: i64, b: i64) -> (i64, i64) {
    (div_floor(a, b), rem_euclid(a, b))
}
