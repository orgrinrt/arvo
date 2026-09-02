//! A radix-general round-first quantiser.
//!
//! `50_probes/model.rs` (copied into `../quantiser-fadd-shared/src/model.rs`) is
//! the validated reference implementation of the design's ratified quantiser,
//! and it is radix-two shaped in exactly three places, none of which is the
//! design:
//!
//!   - `floor_log2` / `ge_pow2`: the grid is selected by the value's binade,
//!     found with `leading_zeros`.
//!   - `round_to_quantum`: alignment is `<<` and `>>`, and the tie threshold is
//!     `1 << (s - 1)`.
//!   - the carry-out renormalisation `m >>= 1`.
//!
//! Each generalises to `R` by replacing a shift with a multiply or divide by
//! `R^k`. The design's own statement of the quantiser (58:205-224) never
//! mentions two: it says "the grids with quantum `radix^(e - p + 1)`", and this
//! file is that sentence with `radix` left as a parameter.
//!
//! Two facts fall out of the generalisation that the radix-two model could not
//! have shown, and both are stated in `59_fog_the_lowering_door.md` section 3:
//!
//!   1. A TIE IS ONLY REACHABLE AT AN EVEN RADIX. A tie is `2 * lost == R^s`.
//!      For odd `R`, `R^s` is odd and the left side is even, so no exact tie
//!      exists at any `s`, and every tie-breaking rule the `Quantisation` axis
//!      offers is vacuous there. Radix ten is even, so decimal keeps ties.
//!   2. THE CARRY-OUT RENORMALISATION IS STILL EXACT. `m == R^p` exactly when
//!      it fires, so `m / R` loses nothing at any radix.
//!
//! `R` is a const generic so each bench variant monomorphises to its own radix
//! and neither pays a runtime branch on it.

#![allow(dead_code)]

/// A finite value on some grid: sign * mag * R^scale. mag == 0 means zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scaled {
    pub neg: bool,
    pub mag: u128,
    pub scale: i32,
}

/// An exact nonzero rational: sign * (num/den) * R^scale.
#[derive(Clone, Copy, Debug)]
pub struct Rat {
    pub neg: bool,
    pub num: u128,
    pub den: u128,
    pub scale: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    Finite(Scaled),
    Infinite { neg: bool },
    Refused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Graded {
    pub out: Out,
    /// The IEEE-754 clause-7 flag word the design's grade is (58:396-409).
    /// bit 2 overflow, bit 3 underflow, bit 4 inexact.
    pub grade: u32,
}

pub const INEXACT: u32 = 16;
pub const UNDERFLOW: u32 = 8;
pub const OVERFLOW: u32 = 4;

/// A format: `P` significand digits in radix `R`, exponent in `[EMIN, EMAX]`,
/// gradual underflow. Everything is a const parameter so a variant's whole
/// format is one monomorphisation.
pub struct Fmt<const R: u128, const P: u32, const EMIN: i32, const EMAX: i32>;

impl<const R: u128, const P: u32, const EMIN: i32, const EMAX: i32> Fmt<R, P, EMIN, EMAX> {
    /// The exponent function: the quantum exponent of the grid at exponent `e`,
    /// floored by gradual underflow. Radix-independent in form; this IS the
    /// design's founding "a format is an exponent function" sentence.
    #[inline]
    pub fn quantum_exp(e: i32) -> i32 {
        let unfloored = e - (P as i32) + 1;
        let floor = EMIN - (P as i32) + 1;
        if unfloored < floor {
            floor
        } else {
            unfloored
        }
    }

    /// `R^P`, the exclusive upper bound on a significand.
    #[inline]
    pub fn pow_p() -> u128 {
        ipow::<R>(P)
    }

    /// `R^(P-1)`, the smallest normalised significand.
    #[inline]
    pub fn pow_p_minus_1() -> u128 {
        ipow::<R>(P - 1)
    }
}

/// `R^k`, saturating at `u128::MAX` so an out-of-range `k` is detectable
/// rather than wrapping. Callers guard before using the result as a divisor.
///
/// Each radix gets its best available form, per the always-optimal-internals
/// rule: at `R == 2` a power is a shift, and at every other radix it is
/// exponentiation by squaring, `O(log k)`. The naive `O(k)` loop this replaces
/// made the FORMAT WIDTH the dominant term instead of the radix (binary32's
/// twenty-four digits cost twenty-four iterations where decimal32's seven cost
/// seven), so the bench's first run measured this helper rather than either
/// quantiser. Stated because a reader comparing the numbers below against a
/// naive reimplementation will not reproduce them.
#[inline]
pub fn ipow<const R: u128>(k: u32) -> u128 {
    if R == 2 {
        return if k >= 128 { u128::MAX } else { 1u128 << k };
    }
    let mut acc: u128 = 1;
    let mut base: u128 = R;
    let mut e = k;
    while e > 0 {
        if e & 1 == 1 {
            match acc.checked_mul(base) {
                Some(v) => acc = v,
                None => return u128::MAX,
            }
        }
        e >>= 1;
        if e > 0 {
            match base.checked_mul(base) {
                Some(v) => base = v,
                None => return u128::MAX,
            }
        }
    }
    acc
}

/// `floor(log_R(v))` for `v != 0`. At `R == 2` this is `ilog2`, one `clz`; at
/// every other radix it is `ilog(R)`, which is a loop or a table. That
/// asymmetry is a real cost of a non-binary radix and is deliberately left in
/// rather than engineered around: finding a value's decade is genuinely more
/// work than finding its binade.
#[inline]
fn ilog_r<const R: u128>(v: u128) -> u32 {
    if R == 2 {
        v.ilog2()
    } else {
        v.ilog(R)
    }
}

/// `num * R^scale >= den * R^e`, compared without overflow by scaling whichever
/// side is small. The radix-two model's `ge_pow2`, with the shift replaced.
#[inline]
fn ge_pow_r<const R: u128>(num: u128, den: u128, scale: i32, e: i32) -> bool {
    let k = scale - e;
    if k >= 0 {
        let p = ipow::<R>(k as u32);
        if p == u128::MAX {
            return true;
        }
        match num.checked_mul(p) {
            Some(v) => v >= den,
            None => true,
        }
    } else {
        let p = ipow::<R>((-k) as u32);
        if p == u128::MAX {
            return false;
        }
        match den.checked_mul(p) {
            Some(v) => num >= v,
            None => false,
        }
    }
}

/// `e` with `R^e <= value < R^(e+1)`, for a nonzero positive rational.
#[inline]
pub fn floor_log_r<const R: u128>(num: u128, den: u128, scale: i32) -> i32 {
    debug_assert!(num != 0 && den != 0);
    let mut e = ilog_r::<R>(num) as i32 - ilog_r::<R>(den) as i32 + scale;
    while !ge_pow_r::<R>(num, den, scale, e) {
        e -= 1;
    }
    while ge_pow_r::<R>(num, den, scale, e + 1) {
        e += 1;
    }
    e
}

/// Round the exact rational to the grid of quantum `R^q`, nearest-ties-even.
/// Returns `(significand, inexact)`.
#[inline]
fn round_to_quantum<const R: u128>(r: &Rat, q: i32, e: i32) -> (u128, bool) {
    // Strictly below half a quantum. No scaling is safe and none is needed.
    if e < q - 1 {
        return (0, true);
    }
    let t = r.scale - q;
    let mut quo = r.num / r.den;
    let mut rem = r.num % r.den;
    if t >= 0 {
        // Feed `t` radix-R digits. `quo` stays below `R^P` because `q` came
        // from the value's own exponent, so nothing intermediate overflows.
        for _ in 0..t {
            let scaled = rem * R;
            quo = quo * R + scaled / r.den;
            rem = scaled % r.den;
        }
        if rem == 0 {
            return (quo, false);
        }
        let twice = rem * 2;
        // Ties-to-even. `twice == r.den` is the tie; see the module header on
        // why the odd-radix case can never reach it through the `t < 0` arm.
        let up = twice > r.den || (twice == r.den && quo % 2 == 1);
        (quo + up as u128, true)
    } else {
        let s = (-t) as u32;
        let p = ipow::<R>(s);
        if p == u128::MAX {
            return (0, true);
        }
        let lost = quo % p;
        let out = quo / p;
        let inexact = lost != 0 || rem != 0;
        if !inexact {
            return (out, false);
        }
        // `2 * lost` against `p` rather than `lost` against `p / 2`, so an odd
        // `R^s` is compared exactly instead of through a floored half.
        let twice = lost * 2;
        let up = twice > p || (twice == p && (rem != 0 || out % 2 == 1));
        (out + up as u128, true)
    }
}

/// The quantiser. Select the grid from the value's own magnitude, round on the
/// unbounded-above extension of it, classify afterward.
#[inline]
pub fn quantize<const R: u128, const P: u32, const EMIN: i32, const EMAX: i32>(r: &Rat) -> Graded {
    if r.num == 0 {
        return Graded {
            out: Out::Finite(Scaled {
                neg: r.neg,
                mag: 0,
                scale: 0,
            }),
            grade: 0,
        };
    }
    let e = floor_log_r::<R>(r.num, r.den, r.scale);
    let mut q = Fmt::<R, P, EMIN, EMAX>::quantum_exp(e);
    let (mut m, inexact) = round_to_quantum::<R>(r, q, e);
    let pow_p = Fmt::<R, P, EMIN, EMAX>::pow_p();
    if m >= pow_p {
        // Rounding up out of the binade lands on the next, coarser grid, at a
        // power of the radix, so the division is exact and no second rounding
        // is needed. Exactly the radix-two `m >>= 1`, generalised.
        debug_assert_eq!(m, pow_p);
        m /= R;
        q += 1;
    }
    let mut grade = if inexact { INEXACT } else { 0 };

    let maxq = Fmt::<R, P, EMIN, EMAX>::quantum_exp(EMAX);
    let maxm = pow_p - 1;
    if q > maxq || (q == maxq && m > maxm) {
        return Graded {
            out: Out::Infinite { neg: r.neg },
            grade: grade | OVERFLOW | INEXACT,
        };
    }

    let subq = Fmt::<R, P, EMIN, EMAX>::quantum_exp(EMIN);
    let nm = Fmt::<R, P, EMIN, EMAX>::pow_p_minus_1();
    if q == subq && m < nm && inexact {
        grade |= UNDERFLOW;
    }
    Graded {
        out: Out::Finite(Scaled {
            neg: r.neg,
            mag: m,
            scale: q,
        }),
        grade,
    }
}

/// Exact sum of two grid values at radix `R`. Aligns to the lower scale by
/// multiplying by `R^k`, where the radix-two model shifted.
#[inline]
pub fn exact_add<const R: u128>(a: Scaled, b: Scaled) -> Rat {
    if a.mag == 0 && b.mag == 0 {
        return Rat {
            neg: a.neg && b.neg,
            num: 0,
            den: 1,
            scale: 0,
        };
    }
    if a.mag == 0 {
        return Rat {
            neg: b.neg,
            num: b.mag,
            den: 1,
            scale: b.scale,
        };
    }
    if b.mag == 0 {
        return Rat {
            neg: a.neg,
            num: a.mag,
            den: 1,
            scale: a.scale,
        };
    }
    let s = a.scale.min(b.scale);
    let am = (a.mag * ipow::<R>((a.scale - s) as u32)) as i128;
    let bm = (b.mag * ipow::<R>((b.scale - s) as u32)) as i128;
    let sa = if a.neg { -am } else { am };
    let sb = if b.neg { -bm } else { bm };
    let t = sa + sb;
    if t == 0 {
        return Rat {
            neg: false,
            num: 0,
            den: 1,
            scale: 0,
        };
    }
    Rat {
        neg: t < 0,
        num: t.unsigned_abs(),
        den: 1,
        scale: s,
    }
}
