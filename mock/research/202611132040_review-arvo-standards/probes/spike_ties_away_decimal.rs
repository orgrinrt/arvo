mod ties_away;
mod ieee754;

use arvo_format::format::Format;
use arvo_format::overflow::Saturate;
use arvo_format::quantum::{Magnitude, Quantum};
use arvo_format::rounding::{HalfUp, Mode, Rounding};
use arvo_format::standards::ieee754::rounding_direction::{
    RoundTiesToEven,
    RoundTowardNegative,
    RoundTowardPositive,
    RoundTowardZero,
};
use arvo_format::standards::ieee754::{Attributed, Decimal32, Decimal64};
use ieee754::{Draws, Value, land, power, same};

/// The number of decimal digits of `|c|`, zero for zero.
fn digits(c: i128) -> u32 {
    let mut n = c.unsigned_abs();
    let mut d = 0;
    while n > 0 {
        n /= 10;
        d += 1;
    }
    d
}

/// `c` divided by `10^drop` under a direction, the standard's way: the kept digits
/// and the dropped part against half of what was dropped.
fn round_off(mode: Mode, c: i128, drop: u32) -> i128 {
    let Some(unit) = power(10, drop) else {
        // Everything is dropped and what is dropped is less than half a unit,
        // since `|c|` is below `10^38`.
        return match mode {
            Mode::Ceil if c > 0 => 1,
            Mode::Floor if c < 0 => -1,
            _ => 0,
        };
    };
    let kept = c / unit;
    let rest = c - kept * unit;
    if rest == 0 {
        return kept;
    }
    let toward_zero = kept;
    let away = kept + c.signum();
    match mode {
        Mode::TowardZero => toward_zero,
        Mode::Ceil => {
            if c > 0 {
                away
            } else {
                toward_zero
            }
        },
        Mode::Floor => {
            if c < 0 {
                away
            } else {
                toward_zero
            }
        },
        Mode::HalfEven => {
            match (rest.abs() * 2).cmp(&unit) {
                core::cmp::Ordering::Less => toward_zero,
                core::cmp::Ordering::Greater => away,
                core::cmp::Ordering::Equal => {
                    if kept % 2 == 0 {
                        toward_zero
                    } else {
                        away
                    }
                },
            }
        },
        _ => unreachable!(),
    }
}

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Remainder, adapt, panic_on_overflow};
use arvo_format::rounding::Floor;
use arvo_format::slots::Slot;
use arvo_format::format::step_exponent;

// ties-away digit rule: kept + away if 2*rest >= unit
fn round_away(c: i128, drop: u32) -> i128 {
    let Some(unit) = power(10, drop) else { return 0 };
    let kept = c / unit; let rest = c - kept * unit;
    if rest != 0 && rest.abs() * 2 >= unit { kept + c.signum() } else { kept }
}
fn rule_away(p: u32, qmin: i32, qmax: i32, c: i128, q: i32) -> Option<Value> {
    let too_many = digits(c).saturating_sub(p) as i32;
    let drop = too_many.max(qmin - q).max(0) as u32;
    let mut kept = round_away(c, drop);
    let mut at = q + drop as i32;
    if digits(kept) > p { kept /= 10; at += 1; }
    if at > qmax {
        match power(10, (at - qmax) as u32).and_then(|s| kept.checked_mul(s)) {
            Some(s) if digits(s) <= p => { kept = s; at = qmax; }
            _ => return None,
        }
    }
    Some(Value { significand: kept, exponent: at })
}
fn pos(num: i128, k: i32, s: i32) -> Option<Exact> {
    let shift = k - s;
    let (n, d) = if shift >= 0 { (num.checked_mul(power(10, shift as u32)?)?, 1i128) } else { (num, power(10, (-shift) as u32)?) };
    Some(Exact::at(Slot::at(n.div_euclid(d)), Remainder::of(n.rem_euclid(d) as u128, d as u128)))
}
fn land_away<F: Format>(c: i128, q: i32) -> Option<(Value, bool)> {
    let count = <F::Quantum as Quantum>::MAGNITUDES.count();
    let mut top = None;
    for m in 0..count {
        let s = step_exponent::<F>(Magnitude::at(m)).power();
        let Some(x) = pos(c, q, s) else { continue };
        let over = if x.is_tie().get() && x.slot().index() < 0 {
            panic_on_overflow::<Signature<F, Adapt<Floor, Saturate>>>(x, Dither::UNUSED).get()
        } else {
            panic_on_overflow::<Signature<F, Adapt<HalfUp, Saturate>>>(x, Dither::UNUSED).get()
        };
        let v = ties_away::read_off::<F, Saturate>(x);
        if !over { return Some((Value { significand: v, exponent: s }, true)); }
        top = Some((Value { significand: v, exponent: s }, false));
    }
    top
}
fn away_misses<F: Format>(p: u32, seed: u64) -> usize {
    let (qmin, qmax) = (<F::Quantum as Quantum>::BASE.power(), step_exponent::<F>(<F::Quantum as Quantum>::MAGNITUDES.largest()).power());
    let mut d = Draws::seeded(seed); let ten_p = power(10, p).unwrap(); let mut miss = 0;
    for _ in 0..3000 {
        let c = d.within(-999_999_999_999_999_999, 999_999_999_999_999_999) as i128;
        let q = d.within((qmin - 20) as i64, (qmax + 2) as i64) as i32;
        let s = d.within(power(10, p - 1).unwrap() as i64, (ten_p - 1) as i64) as i128;
        let sign = if d.next() & 1 == 0 { 1 } else { -1 };
        let tq = d.within((qmin - 1) as i64, (qmax - 1) as i64) as i32;
        for (c, q) in [(c, q), (sign * (s * 10 + 5), tq)] {
            let ok = match (land_away::<F>(c, q), rule_away(p, qmin, qmax, c, q)) {
                (Some((l, _)), Some(w)) => same(10, l, w),
                (Some((_, fin)), None) => !fin,
                _ => false,
            };
            if !ok { miss += 1; }
        }
    }
    miss
}
#[test]
fn decimal_ties_to_away() {
    panic!("d32 {} d64 {}", away_misses::<Decimal32>(7, 1), away_misses::<Decimal64>(16, 2));
}
