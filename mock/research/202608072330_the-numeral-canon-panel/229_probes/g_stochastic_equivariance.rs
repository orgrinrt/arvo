#![allow(dead_code)]

// Probe G. Probe F showed no deterministic nearest mode is both translation
// equivariant and unbiased. This asks whether the vocabulary's sixth name
// escapes that, by checking equivariance of the whole output distribution
// rather than of a single value.
//
// A randomised mode is equivariant when the distribution at x + t is the
// distribution at x shifted by t, for every integer t.
//
// Same exact enumeration of the draw space as probe D, no rng.
//
// Build and run:
//   rustc -O g_stochastic_equivariance.rs -o /tmp/g && /tmp/g > g_output.txt

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
                bump(&mut acc, round(Mode::BitDrop, k + draw, f));
            }
            out = acc;
        }
        Sr::AddThenTowardZero => {
            denom = s as u64;
            let mut acc: Vec<(i64, u64)> = Vec::new();
            for draw in 0..s {
                bump(&mut acc, round(Mode::TowardZero, k + draw, f));
            }
            out = acc;
        }
        Sr::NarrowDraw => {
            let bits = if f == 0 { 0 } else { f - 1 };
            let count = 1i64 << bits;
            denom = count as u64;
            let step = if f == 0 { 0 } else { 1i64 << (f - bits) };
            let mut acc: Vec<(i64, u64)> = Vec::new();
            for j in 0..count {
                bump(&mut acc, round(Mode::BitDrop, k + j * step, f));
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

/// Count (k, t) where the distribution at k + t*s is not the distribution at k
/// shifted by t.
fn equivariance_failures(sr: Sr, w: u32, f: u32, signed: bool) -> u64 {
    let s: i64 = 1i64 << f;
    let d = domain(w, signed);
    let (lo, hi) = (*d.start(), *d.end());
    let mut n = 0u64;
    for k in lo..=hi {
        let (base, bd) = distribution(sr, k, f);
        let mut t = 1i64;
        while k + t * s <= hi {
            let (got, gd) = distribution(sr, k + t * s, f);
            let want: Vec<(i64, u64)> = base.iter().map(|(v, c)| (v + t, *c)).collect();
            if got != want || gd != bd {
                n += 1;
            }
            t += 1;
        }
        let mut t = -1i64;
        while k + t * s >= lo {
            let (got, gd) = distribution(sr, k + t * s, f);
            let want: Vec<(i64, u64)> = base.iter().map(|(v, c)| (v + t, *c)).collect();
            if got != want || gd != bd {
                n += 1;
            }
            t -= 1;
        }
    }
    n
}

fn bias_sum(sr: Sr, w: u32, f: u32, signed: bool) -> i64 {
    let s: i64 = 1i64 << f;
    let d = domain(w, signed);
    let (mut lo, hi) = (*d.start(), *d.end());
    if signed {
        lo += 1;
    }
    let mut num: i64 = 0;
    for k in lo..=hi {
        let (dist, denom) = distribution(sr, k, f);
        let mut sum = 0i64;
        for (v, c) in &dist {
            sum += v * (*c as i64);
        }
        // (E - x) scaled by denom * s, accumulated.
        num += sum * s - k * (denom as i64);
    }
    num
}

fn main() {
    println!("PROBE G: is a randomised mode both equivariant and unbiased?");
    println!("probe F established that no deterministic nearest mode is.");
    println!();

    println!("== CONTROL 1 (must hold) ==");
    println!("  the proportional reading depends on the value only through its");
    println!("  discarded fraction, which an integer translation does not move,");
    println!("  so its distribution must shift with the argument.");
    let c1 = equivariance_failures(Sr::Proportional, 8, 3, true);
    println!("    failures at W = 8, F = 3, signed: {}", c1);
    println!();

    println!("== CONTROL 2 (the case that must fail) ==");
    println!("  the add-then-toward-zero reading is built on the operation whose");
    println!("  sign dependence probe B measured, so it must NOT be equivariant.");
    let c2 = equivariance_failures(Sr::AddThenTowardZero, 8, 3, true);
    println!("    failures at W = 8, F = 3, signed: {}", c2);
    println!();

    println!("== EQUIVARIANCE AND BIAS TOGETHER, W = 8, signed ==");
    println!("  bias is the summed E[R(x)] - x over the symmetric domain,");
    println!("  as a numerator over denom * 2^F.");
    for sr in ALL_SR {
        let mut eq = 0u64;
        for f in 1..=4u32 {
            eq += equivariance_failures(sr, 8, f, true);
        }
        let bias = bias_sum(sr, 8, 3, true);
        println!(
            "  {:24} equivariance failures F=1..4: {:>7}   bias {:>7}   {}",
            sr_name(sr),
            eq,
            bias,
            if eq == 0 && bias == 0 {
                "BOTH"
            } else if eq == 0 {
                "equivariant only"
            } else if bias == 0 {
                "unbiased only"
            } else {
                "neither"
            }
        );
    }
    println!();

    println!("== VERDICTS ==");
    println!(
        "  control 1 (proportional is equivariant): {}",
        if c1 == 0 { "PASS" } else { "FAIL" }
    );
    println!(
        "  control 2 (add-then-toward-zero is not): {}",
        if c2 > 0 { "PASS" } else { "FAIL" }
    );
    let sound = c1 == 0 && c2 > 0;
    println!("  instrument: {}", if sound { "sound" } else { "INVALID" });
    if !sound {
        std::process::exit(1);
    }
}
