// The rounding operations, defined once, included by every probe in 229_probes/.
//
// A value is a scaled integer `k` denoting the rational `k / 2^F`. Rounding
// takes it to an integer `m`. Every operation below is total on i64 for the
// widths swept here (W <= 12), so no wrapping case arises inside the operation
// itself; the domain edges are the caller's business.
//
// Included with `include!`, not compiled as a crate, so a probe is one rustc
// invocation with no manifest.

/// Every operation the six names could denote, plus the two hardware spellings
/// the ratified ruling separated.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Greatest integer not above the value. Toward negative infinity.
    Floor,
    /// Least integer not below the value. Toward positive infinity.
    Ceil,
    /// Discard the fraction, keep the sign. C integer division.
    TowardZero,
    /// The conjugate of TowardZero under negation. Not one of the six.
    AwayFromZero,
    /// Drop the low F bits of a two's complement value. Arithmetic shift right.
    BitDrop,
    /// Nearest; a tie goes to the neighbour toward positive infinity.
    /// This is `floor(x + 1/2)`, and it is one reading of `half_up`.
    HalfUpTowardPosInf,
    /// Nearest; a tie goes to the neighbour of larger magnitude.
    /// This is IEEE 754 roundTiesToAway, and it is the other reading of `half_up`.
    HalfUpAwayFromZero,
    /// Nearest; a tie goes to the neighbour with an even multiplier.
    HalfEven,
}

pub const ALL_MODES: [Mode; 8] = [
    Mode::Floor,
    Mode::Ceil,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::BitDrop,
    Mode::HalfUpTowardPosInf,
    Mode::HalfUpAwayFromZero,
    Mode::HalfEven,
];

pub fn mode_name(m: Mode) -> &'static str {
    match m {
        Mode::Floor => "floor",
        Mode::Ceil => "ceil",
        Mode::TowardZero => "toward_zero",
        Mode::AwayFromZero => "away_from_zero",
        Mode::BitDrop => "bit_drop",
        Mode::HalfUpTowardPosInf => "half_up[toward +inf]",
        Mode::HalfUpAwayFromZero => "half_up[away from 0]",
        Mode::HalfEven => "half_even",
    }
}

/// Floor division, which Rust's `/` is not.
pub fn fdiv(k: i64, s: i64) -> i64 {
    k.div_euclid(s)
}

/// The non-negative remainder, so `k == fdiv(k, s) * s + frem(k, s)` and
/// `0 <= frem(k, s) < s`.
pub fn frem(k: i64, s: i64) -> i64 {
    k.rem_euclid(s)
}

/// Round `k / 2^f` to an integer under `m`. `f` is the fraction width of the
/// input; the output grid is the integers.
pub fn round(m: Mode, k: i64, f: u32) -> i64 {
    let s: i64 = 1i64 << f;
    let q = fdiv(k, s);
    let r = frem(k, s);
    match m {
        Mode::Floor => q,
        Mode::Ceil => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::TowardZero => k / s, // Rust `/` truncates toward zero
        Mode::AwayFromZero => {
            if r == 0 {
                q
            } else if k > 0 {
                k / s + 1
            } else {
                k / s - 1
            }
        }
        Mode::BitDrop => k >> f, // arithmetic shift on a two's complement i64
        Mode::HalfUpTowardPosInf => {
            // floor(x + 1/2). At f = 0 the half is 0 and this is the identity.
            if f == 0 {
                k
            } else {
                fdiv(k + (s >> 1), s)
            }
        }
        Mode::HalfUpAwayFromZero => {
            let twice = 2 * r;
            if twice > s {
                q + 1
            } else if twice < s {
                q
            } else if k > 0 {
                q + 1
            } else {
                q
            }
        }
        Mode::HalfEven => {
            let twice = 2 * r;
            if twice > s {
                q + 1
            } else if twice < s {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

/// The representable scaled integers of a W-bit format, signed or unsigned.
pub fn domain(w: u32, signed: bool) -> std::ops::RangeInclusive<i64> {
    if signed {
        -(1i64 << (w - 1))..=((1i64 << (w - 1)) - 1)
    } else {
        0..=((1i64 << w) - 1)
    }
}

/// The textbook values, as a fixture. Each entry is
/// `(mode, numerator at f = 1, expected integer)`, so the value is `k / 2`.
/// These are the results every reference states, and an implementation that
/// disagrees with one of them is wrong before any sweep runs.
pub const FIXTURE: &[(Mode, i64, i64)] = &[
    // x = 0.5
    (Mode::Floor, 1, 0),
    (Mode::Ceil, 1, 1),
    (Mode::TowardZero, 1, 0),
    (Mode::AwayFromZero, 1, 1),
    (Mode::BitDrop, 1, 0),
    (Mode::HalfUpTowardPosInf, 1, 1),
    (Mode::HalfUpAwayFromZero, 1, 1),
    (Mode::HalfEven, 1, 0),
    // x = -0.5, which is where the two readings of `half_up` part
    (Mode::Floor, -1, -1),
    (Mode::Ceil, -1, 0),
    (Mode::TowardZero, -1, 0),
    (Mode::AwayFromZero, -1, -1),
    (Mode::BitDrop, -1, -1),
    (Mode::HalfUpTowardPosInf, -1, 0),
    (Mode::HalfUpAwayFromZero, -1, -1),
    (Mode::HalfEven, -1, 0),
    // x = 1.5
    (Mode::HalfUpTowardPosInf, 3, 2),
    (Mode::HalfUpAwayFromZero, 3, 2),
    (Mode::HalfEven, 3, 2),
    // x = -1.5
    (Mode::HalfUpTowardPosInf, -3, -1),
    (Mode::HalfUpAwayFromZero, -3, -2),
    (Mode::HalfEven, -3, -2),
    // x = 2.5
    (Mode::HalfUpTowardPosInf, 5, 3),
    (Mode::HalfUpAwayFromZero, 5, 3),
    (Mode::HalfEven, 5, 2),
    // x = -2.5
    (Mode::HalfUpTowardPosInf, -5, -2),
    (Mode::HalfUpAwayFromZero, -5, -3),
    (Mode::HalfEven, -5, -2),
    // x = -0.25 and x = -0.75 need f = 2; kept out of this fixture, which is
    // f = 1 throughout, and covered by the sweeps instead.
];

pub fn check_fixture() -> (usize, usize) {
    let mut ok = 0usize;
    let mut bad = 0usize;
    for &(m, k, want) in FIXTURE {
        let got = round(m, k, 1);
        if got == want {
            ok += 1;
        } else {
            bad += 1;
            println!(
                "  FIXTURE FAILED: {} of {}/2 gave {}, expected {}",
                mode_name(m),
                k,
                got,
                want
            );
        }
    }
    (ok, bad)
}
