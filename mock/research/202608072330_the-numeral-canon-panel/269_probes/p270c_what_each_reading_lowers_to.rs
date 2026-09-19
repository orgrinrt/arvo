// p270c: what each reading of `half_up` lowers to, as emitted assembly.
//
// An ad-hoc quick spike, not a bench. It reads instructions off one compiler's
// output for one target and prices nothing. It can show a shape: whether a
// reading needs a branch, and roughly how many operations its cheapest form is.
//
// Each function rounds k / 2^F to an integer, F = 8, over i32, with the
// intermediate widened to i64 so the addition of the half step is exact, which
// is the one-bit headroom p270a measures. The `_narrow` pair does the addition
// in i32 wrapping arithmetic, the form a datapath with no headroom would use,
// and is wrong at the top 2^(F-1) values (p270a, N3).
//
// Run: rustc --edition 2021 -C opt-level=3 --crate-type=lib --emit=asm \
//        p270c_what_each_reading_lowers_to.rs -o p270c_what_each_reading_lowers_to.s

const F: u32 = 8;
const HALF: i64 = 1 << (F - 1);

/// Nearest, ties toward positive infinity: floor(x + 1/2).
#[no_mangle]
pub fn ties_pos_inf(k: i32) -> i32 {
    ((k as i64 + HALF) >> F) as i32
}

/// Nearest, ties away from zero, branchless: floor(x + 1/2 - [x < 0] * ulp).
#[no_mangle]
pub fn ties_away(k: i32) -> i32 {
    let neg = (k < 0) as i64;
    ((k as i64 + HALF - neg) >> F) as i32
}

/// Nearest, ties away from zero, through toward_zero: trunc(x + sign(x) / 2).
#[no_mangle]
pub fn ties_away_via_toward_zero(k: i32) -> i32 {
    let signed_half = if k < 0 { -HALF } else { HALF };
    ((k as i64 + signed_half) / (1i64 << F)) as i32
}

/// Nearest, ties to even, for comparison with the mode the vocabulary already
/// carries on the zero-mean side.
#[no_mangle]
pub fn ties_even(k: i32) -> i32 {
    let k = k as i64;
    let q = k >> F;
    let r = k & ((1i64 << F) - 1);
    let up = (r > HALF) | ((r == HALF) & (q & 1 == 1));
    (q + up as i64) as i32
}

/// The two readings with the addition in i32, wrapping, no headroom.
#[no_mangle]
pub fn ties_pos_inf_narrow(k: i32) -> i32 {
    k.wrapping_add(HALF as i32) >> F
}

#[no_mangle]
pub fn ties_away_narrow(k: i32) -> i32 {
    k.wrapping_add(HALF as i32).wrapping_sub((k < 0) as i32) >> F
}
