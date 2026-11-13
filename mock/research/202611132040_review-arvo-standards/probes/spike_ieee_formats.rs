mod ieee754;

use arvo_format::format::Format;
use arvo_format::overflow::Saturate;
use arvo_format::points::Floating;
use arvo_format::quantum::Quantum;
use arvo_format::rounding::{HalfUp, Mode, Rounding};
use arvo_format::slots::Slots;
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Binary16, Binary32, Binary64};
use ieee754::{Draws, Value, land, same};

// --- the host's answers, decoded from their bits ----------------------------------

/// A binary32 encoding as a value, or nothing for an infinity.
fn decode32(bits: u32) -> Option<Value> {
    let sign = if bits >> 31 == 1 { -1 } else { 1 };
    let field = ((bits >> 23) & 0xFF) as i32;
    let frac = (bits & 0x7F_FFFF) as i128;
    match field {
        0xFF => None,
        0 => {
            Some(Value {
                significand: sign * frac,
                exponent:    -149,
            })
        },
        _ => {
            Some(Value {
                significand: sign * (frac | 1 << 23),
                exponent:    field - 150,
            })
        },
    }
}

/// A binary64 encoding as a value, or nothing for an infinity.
fn decode64(bits: u64) -> Option<Value> {
    let sign = if bits >> 63 == 1 { -1 } else { 1 };
    let field = ((bits >> 52) & 0x7FF) as i32;
    let frac = (bits & 0xF_FFFF_FFFF_FFFF) as i128;
    match field {
        0x7FF => None,
        0 => {
            Some(Value {
                significand: sign * frac,
                exponent:    -1074,
            })
        },
        _ => {
            Some(Value {
                significand: sign * (frac | 1 << 52),
                exponent:    field - 1075,
            })
        },
    }
}

/// A binary16 encoding as a value, or nothing for an infinity.
fn decode16(bits: u16) -> Option<Value> {
    let sign = if bits >> 15 == 1 { -1 } else { 1 };
    let field = ((bits >> 10) & 0x1F) as i32;
    let frac = (bits & 0x3FF) as i128;
    match field {
        0x1F => None,
        0 => {
            Some(Value {
                significand: sign * frac,
                exponent:    -24,
            })
        },
        _ => {
            Some(Value {
                significand: sign * (frac | 1 << 10),
                exponent:    field - 25,
            })
        },
    }
}

/// `2^k` as an `f64`, exactly, for `k` in `[-1074, 1023]`.
fn pow2(k: i32) -> f64 {
    if k >= -1022 {
        f64::from_bits(((k + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (k + 1074))
    }
}

/// The neighbour of a binary32 one step toward positive or negative infinity.
fn step32(r: f32, up: bool) -> f32 {
    if r == 0.0 {
        let tiny = f32::from_bits(1);
        return if up { tiny } else { -tiny };
    }
    let away = (r > 0.0) == up;
    let bits = r.to_bits();
    f32::from_bits(if away { bits + 1 } else { bits - 1 })
}

/// The host's binary32 answer for `n * 2^k` under a mode, where `n * 2^k` is an
/// exact `f64`.
fn host32(mode: Mode, n: i64, k: i32) -> Option<Value> {
    let x = (n as f64) * pow2(k);
    let r = x as f32;
    let r = if r.is_infinite() {
        match mode {
            Mode::HalfEven => r,
            Mode::Ceil if r < 0.0 => -f32::MAX,
            Mode::Floor if r > 0.0 => f32::MAX,
            Mode::TowardZero => r.signum() * f32::MAX,
            _ => r,
        }
    } else {
        match mode {
            Mode::Floor if (r as f64) > x => step32(r, false),
            Mode::Ceil if (r as f64) < x => step32(r, true),
            Mode::TowardZero if (r as f64).abs() > x.abs() => step32(r, r < 0.0),
            _ => r,
        }
    };
    decode32(r.to_bits())
}

// --- the arm -------------------------------------------------------------------------

/// Whether the landed value is the expected one: the same number where the
/// standard's answer is finite, and no magnitude holding it where it is not.
///
/// Where the standard's answer is finite past the range, the largest member on
/// the side the direction rounds away from, the completion's saturation is what
/// meets it, so a value no magnitude held still agrees when it is that member.
fn agrees(landed: Option<ieee754::Landed>, want: Option<Value>) -> bool {
    match (landed, want) {
        (Some(l), Some(w)) => same(2, l.value, w),
        (Some(l), None) => !l.finite,
        (None, _) => false,
    }
}

/// The binary32 inputs: structured ties and carries, then a sweep across the
/// subnormals, the normals and past the largest finite member.
fn binary32_inputs() -> Vec<(i64, i32)> {
    let mut v = vec![
        ((1 << 24) + 1, 0),
        ((1 << 24) + 3, 0),
        ((1 << 25) - 1, 0),
        (-((1 << 24) + 1), 0),
        (-((1 << 24) + 3), 0),
        (3, -150),
        (5, -150),
        (1, -150),
        (-3, -150),
        ((1 << 25) - 1, -150),
        ((1 << 25) - 1, 103),
        ((1 << 24) - 1, 104),
        (-((1 << 25) - 1), 103),
    ];
    let mut d = Draws::seeded(0x1EEE_0754);
    for _ in 0 .. 4000 {
        let n = d.within(-(1 << 52), 1 << 52);
        let k = d.within(-230, 80) as i32;
        v.push((n, k));
        // A tie: a 24-bit significand and half a step.
        let s = d.within(1 << 23, (1 << 24) - 1);
        v.push((s * 2 + 1, d.within(-200, 102) as i32));
    }
    v
}

fn binary32_misses<R: Rounding>(mode: Mode) -> usize {
    binary32_inputs()
        .into_iter()
        .filter(|&(n, k)| {
            !agrees(
                land::<Attributed<Binary32, R, Saturate>>(n as i128, 1, k),
                host32(mode, n, k),
            )
        })
        .count()
}

fn misses_f<F>(mode: Mode) -> usize
where F: Format, Attributed<F, RoundTiesToEven, Saturate>: arvo_format::adapt::DeclaredSignature,
{
    let _ = mode;
    binary32_inputs().into_iter().filter(|&(n, k)| !agrees(land::<Attributed<F, RoundTiesToEven, Saturate>>(n as i128, 1, k), host32(Mode::HalfEven, n, k))).count()
}
use arvo_format::standards::ieee754::Binary;
#[test]
fn planted_formats() {
    let r = [
        ("Binary32", misses_f::<Binary32>(Mode::HalfEven)),
        ("p23", misses_f::<Binary<23, 127>>(Mode::HalfEven)),
        ("p25", misses_f::<Binary<25, 127>>(Mode::HalfEven)),
        ("emax126", misses_f::<Binary<24, 126>>(Mode::HalfEven)),
        ("emax128", misses_f::<Binary<24, 128>>(Mode::HalfEven)),
        ("Floating<25,-149,254>", misses_f::<Floating<25, -149, 254>>(Mode::HalfEven)),
    ];
    panic!("{:?}", r);
}

/// binary64 under directed modes: step from the host's RNE answer by bits, compare exactly.
fn cmp_exact(v: Value, n: i64, k: i32) -> core::cmp::Ordering {
    // v.significand * 2^v.exponent vs n * 2^k
    let (a, ea, b, eb) = (v.significand, v.exponent, n as i128, k);
    let e = ea.min(eb);
    let sa = (ea - e) as u32; let sb = (eb - e) as u32;
    if sa > 120 || sb > 120 { return if sa > sb { a.signum().cmp(&0).then(core::cmp::Ordering::Greater) } else { 0.cmp(&b.signum()) }; }
    (a << sa).cmp(&(b << sb))
}
fn host64(mode: Mode, n: i64, k: i32) -> Option<Value> {
    let r = (n as f64) * pow2(k);
    let step = |r: f64, up: bool| -> f64 {
        if r == 0.0 { let t = f64::from_bits(1); return if up { t } else { -t }; }
        let away = (r > 0.0) == up; let b = r.to_bits();
        f64::from_bits(if away { b + 1 } else { b - 1 })
    };
    let r = if r.is_infinite() {
        match mode { Mode::Ceil if r < 0.0 => -f64::MAX, Mode::Floor if r > 0.0 => f64::MAX, Mode::TowardZero => r.signum() * f64::MAX, _ => r }
    } else {
        let v = decode64(r.to_bits()).unwrap();
        let o = cmp_exact(v, n, k);
        use core::cmp::Ordering::*;
        match mode {
            Mode::Floor if o == Greater => step(r, false),
            Mode::Ceil if o == Less => step(r, true),
            Mode::TowardZero if (n > 0 && o == Greater) || (n < 0 && o == Less) => step(r, r < 0.0),
            _ => r,
        }
    };
    decode64(r.to_bits())
}
fn b64_directed<R: Rounding>(mode: Mode) -> usize {
    let mut d = Draws::seeded(0x6464_0755);
    let mut miss = 0;
    for _ in 0..3000 {
        for (n, k) in [(d.next() as i64, d.within(-1000, 900) as i32), (d.within(-(1 << 53) + 1, (1 << 53) - 1), d.within(960, 1023) as i32)] {
            if !agrees(land::<Attributed<Binary64, R, Saturate>>(n as i128, 1, k), host64(mode, n, k)) { miss += 1; }
        }
    }
    miss
}
#[test]
fn binary64_directed() {
    let r = [
        b64_directed::<RoundTiesToEven>(Mode::HalfEven),
        b64_directed::<RoundTowardPositive>(Mode::Ceil),
        b64_directed::<RoundTowardNegative>(Mode::Floor),
        b64_directed::<RoundTowardZero>(Mode::TowardZero),
        b64_directed::<HalfUp>(Mode::HalfEven),
    ];
    panic!("{:?}", r);
}
