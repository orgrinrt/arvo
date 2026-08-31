#![allow(dead_code)]

// Probe D. `stochastic` is not a function, so the readings are compared as
// distributions, computed exactly by enumerating the whole draw space rather
// than by sampling. No random number generator appears anywhere here, and the
// numbers are reproducible to the digit.
//
// Model: a value `k / 2^F` is rounded to an integer. The discarded field is the
// low F bits, so the draw space is the 2^F values a random word of that width
// can take, and enumerating it gives the exact distribution.
//
// Five readings, all called stochastic rounding somewhere in the literature or
// in hardware:
//
//   1. proportional          P(up) = the discarded fraction. Unbiased.
//   2. equal probability     P(up) = 1/2 whenever the value is off the grid.
//   3. add-then-bit-drop     add a uniform draw over the discarded field, then
//                            drop the low bits. The usual hardware realisation.
//   4. add-then-toward-zero  the same, with the other reading of the word the
//                            canon retired.
//   5. narrow draw           reading 3 with a draw one bit narrower than the
//                            discarded field.
//
// Build and run:
//   rustc -O d_stochastic_readings.rs -o /tmp/d && /tmp/d > d_output.txt

include!("modes.rs");

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Sr {
    Proportional,
    EqualProbability,
    AddThenBitDrop,
    AddThenTowardZero,
    NarrowDraw,
}

const ALL_SR: [Sr; 5] = [
    Sr::Proportional,
    Sr::EqualProbability,
    Sr::AddThenBitDrop,
    Sr::AddThenTowardZero,
    Sr::NarrowDraw,
];

fn sr_name(s: Sr) -> &'static str {
    match s {
        Sr::Proportional => "proportional",
        Sr::EqualProbability => "equal probability",
        Sr::AddThenBitDrop => "add-then-bit-drop",
        Sr::AddThenTowardZero => "add-then-toward-zero",
        Sr::NarrowDraw => "narrow draw (F-1 bits)",
    }
}

/// The exact distribution of outputs, as `(output, weight)` pairs summing to
/// `denominator`. Sorted by output so two distributions compare directly.
fn distribution(sr: Sr, k: i64, f: u32) -> (Vec<(i64, u64)>, u64) {
    let s: i64 = 1i64 << f;
    let q = fdiv(k, s);
    let r = frem(k, s);
    let mut out: Vec<(i64, u64)> = Vec::new();
    let denom: u64;
    match sr {
        Sr::Proportional => {
            denom = s as u64;
            if r != 0 {
                out.push((q, (s - r) as u64));
                out.push((q + 1, r as u64));
            } else {
                out.push((q, s as u64));
            }
        }
        Sr::EqualProbability => {
            denom = s as u64;
            if r != 0 {
                out.push((q, (s / 2) as u64));
                out.push((q + 1, (s / 2) as u64));
            } else {
                out.push((q, s as u64));
            }
        }
        Sr::AddThenBitDrop => {
            denom = s as u64;
            let mut acc: Vec<(i64, u64)> = Vec::new();
            for draw in 0..s {
                let v = round(Mode::BitDrop, k + draw, f);
                bump(&mut acc, v);
            }
            out = acc;
        }
        Sr::AddThenTowardZero => {
            denom = s as u64;
            let mut acc: Vec<(i64, u64)> = Vec::new();
            for draw in 0..s {
                let v = round(Mode::TowardZero, k + draw, f);
                bump(&mut acc, v);
            }
            out = acc;
        }
        Sr::NarrowDraw => {
            // A draw of F-1 bits, placed at the top of the discarded field, so
            // the granularity of the probability is halved.
            let bits = if f == 0 { 0 } else { f - 1 };
            let count = 1i64 << bits;
            denom = count as u64;
            let step = if f == 0 { 0 } else { 1i64 << (f - bits) };
            let mut acc: Vec<(i64, u64)> = Vec::new();
            for j in 0..count {
                let v = round(Mode::BitDrop, k + j * step, f);
                bump(&mut acc, v);
            }
            out = acc;
        }
    }
    out.sort();
    (out, denom)
}

fn bump(acc: &mut Vec<(i64, u64)>, v: i64) {
    for e in acc.iter_mut() {
        if e.0 == v {
            e.1 += 1;
            return;
        }
    }
    acc.push((v, 1));
}

/// `E[R(x)] - x`, as a numerator over `denominator * 2^f`. Zero means unbiased.
fn bias_numerator(sr: Sr, k: i64, f: u32) -> (i64, i64) {
    let s: i64 = 1i64 << f;
    let (dist, denom) = distribution(sr, k, f);
    let mut sum: i64 = 0;
    for (v, w) in &dist {
        sum += v * (*w as i64);
    }
    // E = sum / denom, x = k / s, so E - x = (sum * s - k * denom) / (denom * s)
    (sum * s - k * (denom as i64), (denom as i64) * s)
}

/// Does the reading return the input unchanged, with certainty, when the input
/// is already on the grid? This is law::rounding_retraction_is_the_identity.
fn retraction_failures(sr: Sr, w: u32, f: u32, signed: bool) -> u64 {
    let s: i64 = 1i64 << f;
    let mut n = 0u64;
    for k in domain(w, signed) {
        if frem(k, s) != 0 {
            continue;
        }
        let (dist, denom) = distribution(sr, k, f);
        let want = k / s;
        let ok = dist.len() == 1 && dist[0].0 == want && dist[0].1 == denom;
        if !ok {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("PROBE D: the readings of `stochastic`, as exact distributions");
    println!("no sampling and no rng. the whole draw space is enumerated.");
    println!();

    println!("== FIXTURE ==");
    let (ok, bad) = check_fixture();
    println!("  {} of {} fixture rows correct", ok, ok + bad);
    if bad != 0 {
        std::process::exit(1);
    }
    println!();

    println!("== CONTROL 1 (must agree) ==");
    println!("  add-then-bit-drop is the hardware realisation of the proportional");
    println!("  reading, so the two distributions must be identical on every value.");
    println!("  A difference means the model of one of them is wrong.");
    let mut c1 = 0u64;
    for w in [4u32, 6, 8] {
        for f in 1..w {
            for signed in [true, false] {
                for k in domain(w, signed) {
                    if distribution(Sr::Proportional, k, f) != distribution(Sr::AddThenBitDrop, k, f)
                    {
                        c1 += 1;
                    }
                }
            }
        }
    }
    println!("    differing values over W in {{4,6,8}}, F in 1..W, both: {}", c1);
    println!();

    println!("== CONTROL 2 (the case that must fail) ==");
    println!("  equal probability must differ from proportional somewhere, or the");
    println!("  instrument is comparing one reading with itself.");
    let mut c2 = 0u64;
    for k in domain(8, true) {
        if distribution(Sr::Proportional, k, 2) != distribution(Sr::EqualProbability, k, 2) {
            c2 += 1;
        }
    }
    println!("    differing values at W = 8, F = 2, signed: {}", c2);
    println!();

    println!("== BIAS: E[R(x)] - x, at W = 8, F = 2, signed ==");
    println!("  reported as the largest magnitude over the domain, as a rational.");
    for sr in ALL_SR {
        let mut worst = (0i64, 1i64);
        let mut worst_k = 0i64;
        for k in domain(8, true) {
            let (num, den) = bias_numerator(sr, k, 2);
            if (num.abs() as i128) * (worst.1 as i128) > (worst.0.abs() as i128) * (den as i128) {
                worst = (num, den);
                worst_k = k;
            }
        }
        println!(
            "  {:24} worst bias {:>5}/{:<5} at x = {}/4  {}",
            sr_name(sr),
            worst.0,
            worst.1,
            worst_k,
            if worst.0 == 0 { "UNBIASED" } else { "biased" }
        );
    }
    println!();

    println!("== RETRACTION: does an on-grid value come back unchanged? ==");
    println!("  law::rounding_retraction_is_the_identity. W = 8, signed and unsigned.");
    for sr in ALL_SR {
        let mut row = String::new();
        let mut tot = 0u64;
        for signed in [true, false] {
            for f in 1..=4u32 {
                let n = retraction_failures(sr, 8, f, signed);
                tot += n;
                row.push_str(&format!("{:>6}", n));
            }
        }
        println!(
            "  {:24} signed F=1..4 then unsigned F=1..4:{}   {}",
            sr_name(sr),
            row,
            if tot == 0 { "RETRACTS" } else { "DOES NOT RETRACT" }
        );
    }
    println!();

    println!("== WORKED VALUES at F = 2, so the draw space is four wide ==");
    println!("  x        reading                    distribution (output x weight)/denom");
    for k in [-3i64, -2, -1, 0, 1, 2, 3] {
        for sr in ALL_SR {
            let (dist, denom) = distribution(sr, k, 2);
            let mut s = String::new();
            for (v, w) in &dist {
                s.push_str(&format!("{}x{} ", v, w));
            }
            println!("  {:>3}/4    {:24}   {}/ {}", k, sr_name(sr), s, denom);
        }
        println!();
    }

    println!("== VERDICTS ==");
    println!("  control 1 (bit-drop SR is proportional): {}", if c1 == 0 { "PASS" } else { "FAIL" });
    println!("  control 2 (equal probability differs):   {}", if c2 > 0 { "PASS" } else { "FAIL" });
    let sound = c1 == 0 && c2 > 0;
    println!("  instrument: {}", if sound { "sound" } else { "INVALID" });
    if !sound {
        std::process::exit(1);
    }
}
