//! A model binary float format and a round-first quantiser over exact rationals.
//!
//! The point of this file is that NOTHING here knows about floats as a kind. A format is a
//! precision plus an exponent interval plus an underflow policy; the representable set is a
//! finite union of fixed-quantum grids; the quantiser rounds on the unbounded-above extension
//! of that union and classifies afterward. That is the design's own round-first quantiser with
//! one extra step in front of it: pick the grid from the operand's own magnitude.
//!
//! Included by the probes with `#[path = "model.rs"] mod model;`.

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underflow {
    /// Subnormals: the emin grid continues down to zero. IEEE 754 default.
    Gradual,
    /// Any nonzero result below 2^emin in magnitude becomes zero, sign preserved.
    /// This is the x86 SSE FTZ / ARM FPCR.FZ output behaviour.
    FlushToZero,
    /// The emin grid does not extend below 2^emin: the representable set has a hole
    /// between zero and the smallest normal. Underflow is a refusal, not a value.
    Abrupt,
}

#[derive(Clone, Copy, Debug)]
pub struct Fmt {
    /// significand digit count, radix 2
    pub p: u32,
    pub emin: i32,
    pub emax: i32,
    pub u: Underflow,
}

pub const BINARY32: Fmt = Fmt {
    p: 24,
    emin: -126,
    emax: 127,
    u: Underflow::Gradual,
};
pub const BINARY64: Fmt = Fmt {
    p: 53,
    emin: -1022,
    emax: 1023,
    u: Underflow::Gradual,
};
/// The model width the exhaustive checks run at.
pub const MODEL: Fmt = Fmt {
    p: 4,
    emin: -3,
    emax: 4,
    u: Underflow::Gradual,
};

impl Fmt {
    /// The quantum exponent of the binade at exponent e, floored by the underflow policy.
    /// This IS the "exponent function" the design's founding idea names: for an `Implicit`
    /// numeral it is a constant, for a `Ranged` numeral it is this.
    pub fn quantum_exp(&self, e: i32) -> i32 {
        let unfloored = e - (self.p as i32) + 1;
        let floor = self.emin - (self.p as i32) + 1;
        if unfloored < floor {
            floor
        } else {
            unfloored
        }
    }
    /// Largest finite magnitude, as (mag, scale): mag * 2^scale.
    pub fn max_finite(&self) -> (u128, i32) {
        ((1u128 << self.p) - 1, self.quantum_exp(self.emax))
    }
    /// Smallest positive normal, 2^emin.
    pub fn min_normal(&self) -> (u128, i32) {
        (1u128 << (self.p - 1), self.quantum_exp(self.emin))
    }
    /// Every finite value of the format, ascending, positives only (zero first).
    pub fn positives(&self) -> Vec<Dyadic> {
        let mut v = Vec::new();
        let subq = self.quantum_exp(self.emin);
        if self.u == Underflow::Gradual {
            for m in 1..(1u128 << (self.p - 1)) {
                v.push(Dyadic {
                    neg: false,
                    mag: m,
                    scale: subq,
                });
            }
        }
        for e in self.emin..=self.emax {
            let q = self.quantum_exp(e);
            for m in (1u128 << (self.p - 1))..(1u128 << self.p) {
                v.push(Dyadic {
                    neg: false,
                    mag: m,
                    scale: q,
                });
            }
        }
        v
    }
}

/// A finite value on some grid: sign * mag * 2^scale. mag == 0 means zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dyadic {
    pub neg: bool,
    pub mag: u128,
    pub scale: i32,
}

impl Dyadic {
    pub fn zero(neg: bool) -> Self {
        Dyadic {
            neg,
            mag: 0,
            scale: 0,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.mag == 0
    }
    /// Exact value as an f64, for reporting only. Not used in any decision.
    pub fn as_f64(&self) -> f64 {
        let m = self.mag as f64 * (2f64).powi(self.scale);
        if self.neg {
            -m
        } else {
            m
        }
    }
}

/// An exact nonzero rational: sign * (num/den) * 2^scale, den != 0.
#[derive(Clone, Copy, Debug)]
pub struct Rat {
    pub neg: bool,
    pub num: u128,
    pub den: u128,
    pub scale: i32,
}

impl Rat {
    pub fn from_dyadic(d: Dyadic) -> Self {
        Rat {
            neg: d.neg,
            num: d.mag,
            den: 1,
            scale: d.scale,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }
}

/// What a quantiser delivers. Separated from the value so the classification step is visible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Finite(Dyadic),
    Infinite {
        neg: bool,
    },
    /// The Abrupt underflow policy's answer, and the design's `Refuse` resolution.
    Refused(Cause),
}

/// IEEE 754-2019 clause 7's five exceptions, as the design's refusal causes and
/// quantisation events. `Inexact` and `Underflow` are events (the value is delivered);
/// `Invalid` and `DivideByZero` are causes with no quantiser origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cause {
    Invalid = 1,
    DivideByZero = 2,
    Overflow = 4,
    Underflow = 8,
    Inexact = 16,
}

/// The grade: a free commutative monoid over causes. With a five-element generator set and
/// no multiplicity it is a bitmask, which is exactly IEEE's sticky flag word.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Grade(pub u32);

impl Grade {
    pub const EMPTY: Grade = Grade(0);
    pub fn of(c: Cause) -> Grade {
        Grade(c as u32)
    }
    pub fn join(self, o: Grade) -> Grade {
        Grade(self.0 | o.0)
    }
    pub fn has(self, c: Cause) -> bool {
        self.0 & (c as u32) != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Graded {
    pub out: Outcome,
    pub grade: Grade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Nearest,
    TowardZero,
    TowardPositive,
    TowardNegative,
    NearestTiesAway,
}

/// floor(log2(num/den)) for a nonzero positive rational, plus the caller's scale.
/// Returned e satisfies 2^e <= value < 2^(e+1).
pub fn floor_log2(num: u128, den: u128, scale: i32) -> i32 {
    debug_assert!(num != 0 && den != 0);
    let bn = 127 - num.leading_zeros() as i32;
    let bd = 127 - den.leading_zeros() as i32;
    let mut e = bn - bd + scale;
    while !ge_pow2(num, den, scale, e) {
        e -= 1;
    }
    while ge_pow2(num, den, scale, e + 1) {
        e += 1;
    }
    e
}

/// num * 2^scale >= den * 2^e ?  Compared without overflow by shifting whichever side is small.
fn ge_pow2(num: u128, den: u128, scale: i32, e: i32) -> bool {
    let k = scale - e;
    if k >= 0 {
        if k >= 127 {
            return true;
        }
        match num.checked_shl(k as u32) {
            Some(v) if (v >> k) == num => v >= den,
            _ => true,
        }
    } else {
        let k = (-k) as u32;
        if k >= 127 {
            return false;
        }
        match den.checked_shl(k) {
            Some(v) if (v >> k) == den => num >= v,
            _ => false,
        }
    }
}

/// Round the exact rational to the grid of quantum 2^q, returning the integer significand
/// and whether the rounding was inexact. `e` is the value's own floor-log2.
///
/// Computed by long division rather than by shifting, because the shift form overflows
/// silently: the exact product of two binary32 subnormals has scale -298, and `den << 149`
/// on a u128 masks the shift amount and returns a plausible wrong answer. That bug delivered
/// a nonzero result for a product 130 binades below the smallest subnormal, and the hardware
/// disagreeing with it is what found it.
fn round_to_quantum(r: &Rat, q: i32, e: i32, dir: Dir) -> (u128, bool) {
    let from_nothing = match dir {
        Dir::TowardPositive => !r.neg as u128,
        Dir::TowardNegative => r.neg as u128,
        _ => 0,
    };
    // Strictly below half a quantum: no shift is safe and none is needed.
    if e < q - 1 {
        return (from_nothing, true);
    }
    let t = r.scale - q;
    let mut quo = r.num / r.den;
    let mut rem = r.num % r.den;
    if t >= 0 {
        // Feed t zero bits. `quo` stays below 2^p because q came from the value's own
        // exponent, so nothing intermediate overflows.
        for _ in 0..t {
            let bit = u128::from(rem * 2 >= r.den);
            quo = quo * 2 + bit;
            rem *= 2;
            if rem >= r.den {
                rem -= r.den;
            }
        }
        if rem == 0 {
            return (quo, false);
        }
        let twice = rem * 2;
        let up = match dir {
            Dir::Nearest => twice > r.den || (twice == r.den && quo % 2 == 1),
            Dir::NearestTiesAway => twice >= r.den,
            Dir::TowardZero => false,
            Dir::TowardPositive => !r.neg,
            Dir::TowardNegative => r.neg,
        };
        (quo + up as u128, true)
    } else {
        let s = (-t) as u32;
        if s >= 127 {
            return (from_nothing, true);
        }
        let mask = (1u128 << s) - 1;
        let lost = quo & mask;
        let out = quo >> s;
        let half = 1u128 << (s - 1);
        let inexact = lost != 0 || rem != 0;
        if !inexact {
            return (out, false);
        }
        let up = match dir {
            Dir::Nearest => lost > half || (lost == half && (rem != 0 || out % 2 == 1)),
            Dir::NearestTiesAway => lost >= half,
            Dir::TowardZero => false,
            Dir::TowardPositive => !r.neg,
            Dir::TowardNegative => r.neg,
        };
        (out + up as u128, true)
    }
}

/// The quantiser. Round first on the unbounded-above extension of the grid family,
/// then classify against the exponent range and resolve.
///
/// The one structural difference from the design's fixed-quantum quantiser is the second
/// line: the grid is selected from the value's own magnitude before rounding.
pub fn quantize(f: &Fmt, r: &Rat, dir: Dir) -> Graded {
    if r.is_zero() {
        return Graded {
            out: Outcome::Finite(Dyadic::zero(r.neg)),
            grade: Grade::EMPTY,
        };
    }
    let e = floor_log2(r.num, r.den, r.scale);
    let mut q = f.quantum_exp(e);
    let (mut m, inexact) = round_to_quantum(r, q, e, dir);
    // Rounding up out of the binade lands on the next, coarser grid. The value is a power of
    // two there, so no second rounding is needed; only the representation renormalises.
    if m >= (1u128 << f.p) {
        debug_assert_eq!(m, 1u128 << f.p);
        m >>= 1;
        q += 1;
    }
    let mut grade = if inexact {
        Grade::of(Cause::Inexact)
    } else {
        Grade::EMPTY
    };

    // classify: over-range
    let (maxm, maxq) = f.max_finite();
    if q > maxq || (q == maxq && m > maxm) {
        grade = grade
            .join(Grade::of(Cause::Overflow))
            .join(Grade::of(Cause::Inexact));
        let sat = Outcome::Finite(Dyadic {
            neg: r.neg,
            mag: maxm,
            scale: maxq,
        });
        let inf = Outcome::Infinite { neg: r.neg };
        let out = match dir {
            Dir::Nearest | Dir::NearestTiesAway => inf,
            Dir::TowardZero => sat,
            Dir::TowardPositive => {
                if r.neg {
                    sat
                } else {
                    inf
                }
            }
            Dir::TowardNegative => {
                if r.neg {
                    inf
                } else {
                    sat
                }
            }
        };
        return Graded { out, grade };
    }

    // classify: under-range. Tininess detected after rounding: the rounded result is
    // strictly below 2^emin in magnitude. The before-rounding fork is `e < f.emin`, which
    // differs from this one only on the flag, never on the delivered value.
    let (nm, _) = f.min_normal();
    let tiny_after = q == f.quantum_exp(f.emin) && m < nm;
    if tiny_after {
        match f.u {
            Underflow::Gradual => {
                if inexact {
                    grade = grade.join(Grade::of(Cause::Underflow));
                }
            }
            Underflow::FlushToZero => {
                grade = grade
                    .join(Grade::of(Cause::Underflow))
                    .join(Grade::of(Cause::Inexact));
                return Graded {
                    out: Outcome::Finite(Dyadic::zero(r.neg)),
                    grade,
                };
            }
            Underflow::Abrupt => {
                grade = grade.join(Grade::of(Cause::Underflow));
                return Graded {
                    out: Outcome::Refused(Cause::Underflow),
                    grade,
                };
            }
        }
    }
    Graded {
        out: Outcome::Finite(Dyadic {
            neg: r.neg,
            mag: m,
            scale: q,
        }),
        grade,
    }
}

/// Tininess detected BEFORE rounding: the exact value lies strictly inside (-2^emin, 2^emin).
/// Kept separate because the standard permits either and the two differ only on the flag.
pub fn tiny_before(f: &Fmt, r: &Rat) -> bool {
    if r.is_zero() {
        return false;
    }
    floor_log2(r.num, r.den, r.scale) < f.emin
}

// ---------------------------------------------------------------------------
// binary32 bridge, so the model can be cross-validated against the hardware.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F32Val {
    Fin(Dyadic),
    Inf(bool),
    Nan,
}

pub fn decode_f32(x: f32) -> F32Val {
    let b = x.to_bits();
    let neg = b >> 31 == 1;
    let be = ((b >> 23) & 0xff) as i32;
    let frac = (b & 0x7f_ffff) as u128;
    if be == 0xff {
        return if frac == 0 {
            F32Val::Inf(neg)
        } else {
            F32Val::Nan
        };
    }
    if be == 0 {
        return F32Val::Fin(Dyadic {
            neg,
            mag: frac,
            scale: -149,
        });
    }
    F32Val::Fin(Dyadic {
        neg,
        mag: frac + (1 << 23),
        scale: be - 127 - 23,
    })
}

/// Encode an Outcome back to f32 bits, for direct bit comparison with the hardware.
pub fn encode_f32(o: Outcome) -> u32 {
    match o {
        Outcome::Infinite { neg } => (neg as u32) << 31 | 0x7f80_0000,
        Outcome::Refused(_) => 0xdead_beef,
        Outcome::Finite(d) => {
            let s = (d.neg as u32) << 31;
            if d.mag == 0 {
                return s;
            }
            if d.scale == -149 && d.mag < (1 << 23) {
                return s | d.mag as u32;
            }
            let be = (d.scale + 23 + 127) as u32;
            s | (be << 23) | ((d.mag as u32) & 0x7f_ffff)
        }
    }
}

/// Exact sum of two decoded values. Panics if the exponent spread would overflow i128,
/// which the callers filter for and state as a bound.
pub fn exact_add(a: Dyadic, b: Dyadic) -> Rat {
    if a.is_zero() && b.is_zero() {
        return Rat {
            neg: a.neg && b.neg,
            num: 0,
            den: 1,
            scale: 0,
        };
    }
    if a.is_zero() {
        return Rat::from_dyadic(b);
    }
    if b.is_zero() {
        return Rat::from_dyadic(a);
    }
    let s = a.scale.min(b.scale);
    assert!(
        a.scale - s < 100 && b.scale - s < 100,
        "exponent spread out of model range"
    );
    let am = (a.mag as i128) << ((a.scale - s) as u32);
    let bm = (b.mag as i128) << ((b.scale - s) as u32);
    let sa = if a.neg { -am } else { am };
    let sb = if b.neg { -bm } else { bm };
    let t = sa + sb;
    if t == 0 {
        // Exact cancellation. IEEE 754-2019 6.3: the sign is + under every direction
        // except roundTowardNegative, where it is -. Callers that test directed rounding
        // handle that; roundTiesToEven is what the silicon check uses.
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

pub fn exact_mul(a: Dyadic, b: Dyadic) -> Rat {
    if a.is_zero() || b.is_zero() {
        return Rat {
            neg: a.neg != b.neg,
            num: 0,
            den: 1,
            scale: 0,
        };
    }
    Rat {
        neg: a.neg != b.neg,
        num: a.mag * b.mag,
        den: 1,
        scale: a.scale + b.scale,
    }
}

/// Exact quotient as a rational. Callers handle the zero-divisor cases.
pub fn exact_div(a: Dyadic, b: Dyadic) -> Rat {
    Rat {
        neg: a.neg != b.neg,
        num: a.mag,
        den: b.mag,
        scale: a.scale - b.scale,
    }
}
