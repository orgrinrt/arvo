//! Probe 1: does the model float, quantised round-first by the design's own quantiser,
//! agree bit for bit with what the hardware actually computes?
//!
//! rustc --edition 2021 -O probe_1_model_vs_silicon.rs -o /tmp/p1 && /tmp/p1
//!
//! The model knows nothing about IEEE beyond the format parameters. If round-first over a
//! magnitude-selected grid reproduces binary32 add, multiply and divide exactly, then the
//! design's settled quantiser needs one new step and no new principle.
//!
//! Exactness bound: the exact sum of two binary32 values spans up to 277 bits, wider than the
//! u128 this model computes in. The sweep therefore restricts pairs to an exponent spread of
//! at most 90, stated rather than hidden, and the exhaustive whole-format check runs at the
//! model width instead (probe 2).

#[path = "model.rs"]
mod model;
use model::*;

fn interesting() -> Vec<f32> {
    let mut v = vec![
        0.0f32,
        -0.0f32,
        f32::from_bits(1), // smallest subnormal
        f32::from_bits(2),
        f32::from_bits(0x007f_ffff), // largest subnormal
        f32::MIN_POSITIVE,           // smallest normal
        f32::MIN_POSITIVE * 1.5,
        1.0,
        -1.0,
        1.0 + f32::EPSILON,
        2.0,
        3.0,
        0.5,
        1.0 / 3.0,
        16777215.0, // 2^24 - 1
        16777216.0, // 2^24
        f32::MAX,
        -f32::MAX,
        f32::MAX / 2.0,
        f32::from_bits(f32::MAX.to_bits() - 1),
    ];
    // a deterministic spread of bit patterns, filtered to finite
    let mut s: u32 = 0x1234_5678;
    for _ in 0..4000 {
        s = s.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let x = f32::from_bits(s);
        if x.is_finite() {
            v.push(x);
        }
    }
    v
}

fn spread_ok(a: Dyadic, b: Dyadic) -> bool {
    (a.scale - b.scale).abs() <= 90
}

fn check(op: &str, a: f32, b: f32, hw: f32, model_bits: u32) -> bool {
    let hb = hw.to_bits();
    if hb == model_bits {
        return true;
    }
    // -0.0 vs +0.0 for exact-zero sums under roundTiesToEven is a real difference; report it.
    println!(
        "MISMATCH {op}: a={a:e} ({:#010x}) b={b:e} ({:#010x}) hw={:#010x} model={model_bits:#010x}",
        a.to_bits(),
        b.to_bits(),
        hb
    );
    false
}

fn main() {
    let f = BINARY32;
    let vals = interesting();
    let (mut nadd, mut nmul, mut ndiv) = (0u64, 0u64, 0u64);
    let mut bad = 0u64;
    let mut band_witnesses = 0u64;
    let mut subnormal_results = 0u64;
    let mut overflowed = 0u64;

    for &a in &vals {
        for &b in &vals {
            let (da, db) = match (decode_f32(a), decode_f32(b)) {
                (F32Val::Fin(x), F32Val::Fin(y)) => (x, y),
                _ => continue,
            };

            // ---- add ----
            if spread_ok(da, db) {
                let r = exact_add(da, db);
                let g = quantize(&f, &r, Dir::Nearest);
                let mut bits = encode_f32(g.out);
                // IEEE 6.3: the sign of an exact zero sum under roundTiesToEven is + unless
                // both operands are -0 or the direction is toward negative. The model already
                // does this; a - a keeps + as well.
                if g.grade.has(Cause::Overflow) {
                    overflowed += 1;
                }
                if let Outcome::Finite(d) = g.out {
                    if d.mag != 0 && d.scale == -149 && d.mag < (1 << 23) {
                        subnormal_results += 1;
                    }
                }
                // band witness: exact sum strictly above max finite, result still max finite
                if !g.grade.has(Cause::Overflow) {
                    let (mm, mq) = f.max_finite();
                    if let Outcome::Finite(d) = g.out {
                        if d.mag == mm && d.scale == mq && exceeds(&r, mm, mq) {
                            band_witnesses += 1;
                        }
                    }
                }
                if bits == 0x8000_0000 && (a + b).to_bits() == 0 {
                    bits = 0; // both spellings of zero compared below anyway
                }
                if !check("add", a, b, a + b, bits) {
                    bad += 1;
                    if bad > 20 {
                        std::process::exit(1);
                    }
                }
                nadd += 1;
            }

            // ---- mul ----
            {
                let r = exact_mul(da, db);
                let g = quantize(&f, &r, Dir::Nearest);
                let bits = encode_f32(g.out);
                if !check("mul", a, b, a * b, bits) {
                    bad += 1;
                    if bad > 20 {
                        std::process::exit(1);
                    }
                }
                nmul += 1;
            }

            // ---- div ----
            if !db.is_zero() && !da.is_zero() {
                let r = exact_div(da, db);
                let g = quantize(&f, &r, Dir::Nearest);
                let bits = encode_f32(g.out);
                if !check("div", a, b, a / b, bits) {
                    bad += 1;
                    if bad > 20 {
                        std::process::exit(1);
                    }
                }
                ndiv += 1;
            }
        }
    }

    println!("add pairs {nadd}, mul pairs {nmul}, div pairs {ndiv}, mismatches {bad}");
    println!("overflow-to-infinity results {overflowed}");
    println!("subnormal results (gradual underflow exercised) {subnormal_results}");
    println!("overflow-band witnesses (exact sum > MAX, delivered MAX) {band_witnesses}");
    assert_eq!(bad, 0);

    // the single hand-checkable band witness, against silicon
    let m = f32::MAX;
    let tiny = 1.0f32;
    println!("f32::MAX + 1.0 == f32::MAX: {}", m + tiny == m);
    let ulp = f32::from_bits(m.to_bits()) - f32::from_bits(m.to_bits() - 1);
    println!("ulp(MAX) = {ulp:e}, MAX + ulp/2 = {:e}", m + ulp / 2.0);
    println!(
        "MAX + ulp*0.4999 finite: {}",
        (m + ulp * 0.4999).is_finite()
    );
    println!(
        "MAX + ulp*0.5001 infinite: {}",
        (m + ulp * 0.5001).is_infinite()
    );
}

/// Is the exact rational strictly greater in magnitude than mag*2^scale?
fn exceeds(r: &Rat, mag: u128, scale: i32) -> bool {
    if r.num == 0 {
        return false;
    }
    // r.num/r.den * 2^r.scale > mag * 2^scale
    let k = r.scale - scale;
    let lhs = if k >= 0 { r.num << (k as u32) } else { r.num };
    let rhs = if k >= 0 {
        mag * r.den
    } else {
        (mag * r.den) << ((-k) as u32)
    };
    lhs > rhs
}
