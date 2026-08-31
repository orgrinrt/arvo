// p3: the row `law::rounding_retraction_is_the_identity` re-run over the whole
// ratified vocabulary, where its instrument ran two modes and named neither.
//
// The instrument is `94_probes/c_retraction.rs` part 2, which is on disk. Its
// two modes are `("truncate", false)` and `("nearest", true)`, implemented as
// `x >> f` and `(x + half) >> f` over `u128` with a,b,c in 0..2^W. On a
// non-negative domain the first is floor, which is also toward_zero there, and
// the second is floor(x + 1/2), which is half_up under the corpus's reading.
//
// What it measures is NOT what the row's `statement` says. The statement is
// that rounding a value already on the grid returns it unchanged. That is true
// of every mode at every grid spacing, which p1 section 5 shows, so it cannot
// be the thing that fails at F >= 1. What the instrument compares is a staged
// quantisation against a deferred one over a two-multiply chain:
//
//     eager    = rnd(rnd(A*B, F) * C, F)
//     deferred = rnd(A*B*C, 2F)
//
// FAITHFULNESS. Checked against counts this file did not choose, taken from
// `94_probes/c_retraction.out.txt`: truncate at W=4 gives 0, 800, 1128, 910,
// 543 differing triples at F = 0 to 4, and nearest gives 0, 864, 1248, 880,
// 550. If those integers do not come back, this is not that instrument.
//
// Run: rustc --edition 2024 -O p3_retraction_over_the_whole_vocabulary.rs -o /tmp/p3 && /tmp/p3

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode { Floor, Ceil, TowardZero, HalfUp, HalfEven, AwayFromZero }
use Mode::*;

// The ratified six are toward_zero, floor, ceil, half_up, half_even and
// stochastic. Stochastic is not a function and is handled separately below.
// away_from_zero is carried as the seventh because the question is about it.
const MODES: [Mode; 6] = [Floor, Ceil, TowardZero, HalfUp, HalfEven, AwayFromZero];

fn name(m: Mode) -> &'static str {
    match m {
        Floor => "floor", Ceil => "ceil", TowardZero => "toward_zero",
        HalfUp => "half_up", HalfEven => "half_even", AwayFromZero => "away_from_zero",
    }
}
fn ratified(m: Mode) -> bool { m != AwayFromZero }

fn rnd(p: i128, s: u32, m: Mode) -> i128 {
    if s == 0 { return p; }
    let d = 1i128 << s;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Floor => q,
        Ceil => if r == 0 { q } else { q + 1 },
        TowardZero => if p >= 0 || r == 0 { q } else { q + 1 },
        AwayFromZero => if p >= 0 { if r == 0 { q } else { q + 1 } } else { q },
        HalfUp => if 2 * r >= d { q + 1 } else { q },
        HalfEven => {
            if 2 * r > d { q + 1 } else if 2 * r < d { q }
            else if q % 2 == 0 { q } else { q + 1 }
        }
    }
}

/// The instrument's comparison, generalised over the mode.
fn measure(w: u32, f: u32, m: Mode) -> (u64, u64) {
    let n: i128 = 1 << w;
    let mut differ = 0u64;
    let mut total = 0u64;
    for a in 0..n {
        for b in 0..n {
            let ab = a * b;
            let ab_q = rnd(ab, f, m);
            for c in 0..n {
                total += 1;
                let eager = rnd(ab_q * c, f, m);
                let deferred = rnd(ab * c, 2 * f, m);
                if eager != deferred { differ += 1; }
            }
        }
    }
    (differ, total)
}

/// How many triples reach a quantisation with a nonzero fractional part at the
/// eager step? Where that count is positive a stochastic mode is not a
/// function of the triple, so no deterministic agreement can hold.
fn undetermined_under_stochastic(w: u32, f: u32) -> u64 {
    if f == 0 { return 0; }
    let n: i128 = 1 << w;
    let d = 1i128 << f;
    let mut c_undet = 0u64;
    for a in 0..n {
        for b in 0..n {
            if (a * b).rem_euclid(d) != 0 { c_undet += 1; }
        }
    }
    c_undet
}

fn main() {
    println!("p3: staged against deferred quantisation over a two-multiply chain");
    println!("    exhaustive over a, b, c in 0..2^W, unsigned, as the instrument ran it");
    println!();

    // ---- faithfulness against 94's committed integers ----------------------
    println!("## 0. faithfulness against `94_probes/c_retraction.out.txt`, W = 4");
    println!();
    let pub_trunc: [u64; 5] = [0, 800, 1128, 910, 543];
    let pub_near: [u64; 5] = [0, 864, 1248, 880, 550];
    let mut faithful = true;
    println!("{:<12} {:>3} {:>10} {:>10}  {}", "94's name", "F", "published", "measured", "verdict");
    for f in 0..=4u32 {
        let (d, _) = measure(4, f, Floor);
        let want = pub_trunc[f as usize];
        if d != want { faithful = false; }
        println!("{:<12} {:>3} {:>10} {:>10}  {}", "truncate", f, want, d,
            if d == want { "matches" } else { "DIFFERS" });
    }
    for f in 0..=4u32 {
        let (d, _) = measure(4, f, HalfUp);
        let want = pub_near[f as usize];
        if d != want { faithful = false; }
        println!("{:<12} {:>3} {:>10} {:>10}  {}", "nearest", f, want, d,
            if d == want { "matches" } else { "DIFFERS" });
    }
    println!();
    println!("  So the instrument's two modes are floor and half_up, and this");
    println!("  implementation is that instrument: {}",
        if faithful { "ten of ten integers reproduced" } else { "MISMATCH, file is void" });
    println!();

    // ---- the whole vocabulary ----------------------------------------------
    for w in [4u32, 6, 8] {
        println!("## W = {w}");
        println!();
        print!("{:<16} {:<10}", "mode", "ratified");
        for f in 0..=w { print!("{:>10}", format!("F={f}")); }
        println!();
        for m in MODES {
            print!("{:<16} {:<10}", name(m), if ratified(m) { "yes" } else { "no" });
            for f in 0..=w {
                let (d, _) = measure(w, f, m);
                if d == 0 { print!("{:>10}", "RETRACTS"); } else { print!("{:>10}", d); }
            }
            println!();
        }
        println!();
    }

    // ---- the two regions ----------------------------------------------------
    println!("## the region on the rounding axis");
    println!();
    let mut holds_all = true;
    let mut fails_all = true;
    let mut checked = 0;
    for w in [4u32, 6, 8] {
        for m in MODES {
            let (d0, _) = measure(w, 0, m);
            if d0 != 0 { holds_all = false; }
            for f in 1..=w {
                let (d, _) = measure(w, f, m);
                checked += 1;
                if d == 0 { fails_all = false; }
            }
        }
    }
    println!("  F = 0:  every mode retracts at every swept W: {holds_all}");
    println!("  F >= 1: every mode fails at every swept (W, F), {checked} cells: {fails_all}");
    println!();
    println!("  stochastic, which is not a function and cannot be swept:");
    for w in [4u32, 6, 8] {
        for f in [0u32, 1, w] {
            let u = undetermined_under_stochastic(w, f);
            println!("    W={w} F={f}: {u} product(s) with a nonzero fraction at the eager step{}",
                if f == 0 { ", so the draw never happens and the mode is the identity" }
                else if u > 0 { ", so the eager result is not determined by the triple" }
                else { "" });
        }
    }
    println!();

    // ---- controls -----------------------------------------------------------
    println!("## controls");
    println!();
    let mut ok = true;

    // C1: at F = 0 every mode must give the IDENTICAL count, because the grid
    // is the whole value set and no mode can be told from another.
    let mut c1 = true;
    for w in [4u32, 6, 8] {
        let counts: Vec<u64> = MODES.iter().map(|&m| measure(w, 0, m).0).collect();
        println!("     W={w} F=0 counts across six modes: {counts:?}");
        if counts.iter().any(|&c| c != counts[0]) { c1 = false; }
    }
    if c1 { println!("  C1 EXPECTED-PASS ok: the rounding axis moves nothing at F = 0"); }
    else { println!("  C1 BROKEN"); ok = false; }

    // C1b: a rnd that is not the identity at F = 0 must break C1's invariant,
    // or C1 is asserting nothing.
    {
        let w = 4u32; let n: i128 = 1 << w;
        let mut differ = 0u64;
        for a in 0..n { for b in 0..n {
            let ab = a * b;
            let ab_q = rnd(ab, 0, Floor) + 1; // not the identity
            for c in 0..n {
                if rnd(ab_q * c, 0, Floor) != rnd(ab * c, 0, Floor) { differ += 1; }
            } } }
        if differ > 0 { println!("  C1b EXPECTED-FAIL ok: a non-identity map at F = 0 differs on {differ} triples"); }
        else { println!("  C1b BROKEN: the F = 0 arm cannot see a non-identity map"); ok = false; }
    }

    // C2: the row's own `statement` is true of every mode at every F, so it is
    // not the thing measured above. Checked directly on grid points.
    let mut c2_bad = 0;
    for w in [4u32, 6, 8] {
        for f in 0..=w {
            let d = 1i128 << f;
            for m in MODES {
                let mut k = 0i128;
                while k < (1i128 << w) {
                    if rnd(k * d, f, m) != k { c2_bad += 1; }
                    k += 1;
                }
            }
        }
    }
    if c2_bad == 0 {
        println!("  C2 EXPECTED-PASS ok: `rounding a value already on the grid returns it unchanged`");
        println!("      holds for every mode at every F, 0 counterexamples, so the row's");
        println!("      `statement` cannot be what its `fails` field is about");
    } else { println!("  C2 BROKEN: {c2_bad} grid points moved"); ok = false; }

    // C3: a map that is not a retraction must be caught by C2's check, or C2
    // is asserting nothing.
    {
        let mut bad = 0;
        let d = 1i128 << 2;
        let mut k = 0i128;
        while k < 16 { if rnd(k * d, 2, Floor) + 1 != k { bad += 1; } k += 1; }
        if bad > 0 { println!("  C3 EXPECTED-FAIL ok: a shifted map moves {bad} grid points"); }
        else { println!("  C3 BROKEN"); ok = false; }
    }

    println!();
    println!("controls: {}", if ok && faithful { "clean" } else { "BROKEN" });
}
