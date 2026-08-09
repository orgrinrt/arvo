//! Probe 2: the correctly-rounded sqrt in const position, and what each position emits.
//!
//! The fourth design rule (the pricing pillar) demands a const-callable form, and its
//! standing clause says the guarantee exists in const position and nowhere else. This
//! probe builds the correctly-rounded same-grid sqrt from probe 1's residue rule as a
//! `const fn`, evaluates it in const position at several widths (compilation is the
//! outcome), and exposes both a const-position and a value-position consumer so the
//! emitted code can be inspected under -O (see OUTCOMES.md for the asm findings, with
//! flags and target stated).

/// Correctly-rounded nearest sqrt on the dyadic index grid: operand index k with
/// quantum 2^-F, result index on the same grid. Total over the unsigned domain;
/// range classification is the caller's (the quantiser's) job, per the ratified
/// round-first order.
pub const fn sqrt_rn(k: u64, f: u32) -> u64 {
    let x = k << f;
    let m = isqrt(x);
    let r = x - m * m;
    // round up iff r > m; ties are parity-impossible (probe 1, claim B).
    if r > m {
        m + 1
    } else {
        m
    }
}

const fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = x.div_ceil(2);
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

// const position: guaranteed const evaluation, the fourth rule's required form.
pub const SQRT_HALF_F8: u64 = sqrt_rn(128, 8); // sqrt(0.5) on the 1/256 grid
pub const SQRT_2_F8: u64 = sqrt_rn(512, 8); // sqrt(2)   on the 1/256 grid
pub const SQRT_MAX_F8: u64 = sqrt_rn(255, 8); // sqrt(255/256), the near-far case
pub const SQRT_F32_STYLE: u64 = sqrt_rn(3, 24); // a deep-fraction width, F = 24

const _: () = {
    // sqrt(0.5) = 0.70710678..; * 256 = 181.02  -> 181
    assert!(SQRT_HALF_F8 == 181);
    // sqrt(2) = 1.41421356..; * 256 = 362.04    -> 362
    assert!(SQRT_2_F8 == 362);
    // sqrt(255/256) = 0.998043..; * 256 = 255.499968 -> 255: inside the far point,
    // by less than 2^-14 of an index. the emptiness criterion cuts exactly this fine.
    assert!(SQRT_MAX_F8 == 255);
    // sqrt(3/2^24) * 2^24 = sqrt(3 * 2^24) = 7094.48 -> 7094
    assert!(SQRT_F32_STYLE == 7094);
};

/// Value-position consumer: the optimiser MAY fold this, and the pricing pillar
/// forbids relying on it. Exposed so the asm shows what a per-element loop pays
/// when the operand is genuinely runtime data: an isqrt iteration, not a constant.
#[unsafe(no_mangle)]
pub fn sqrt_rn_runtime(k: u64) -> u64 {
    sqrt_rn(k, 8)
}

/// Const-position consumer: lowers to an immediate, guaranteed.
#[unsafe(no_mangle)]
pub fn sqrt_rn_const_position() -> u64 {
    const R: u64 = sqrt_rn(512, 8);
    R
}
