// PROBE p1. The threshold family that the model-band cross-check cannot see.
//
// The law family, in the wrapping fragment (wrapping mul, wrapping sub, integer
// constants, arity 1):
//
//   L_k(W):  for all x in Z/2^W:  (x)(x-1)(x-2)...(x-k+1)  ==  0   (mod 2^W)
//
// i.e. "every product of k consecutive integers vanishes at this width".
//
// CLAIM under test (the theory this file's argument rests on): L_k is true at
// width W exactly when W <= v2(k!) = k - s2(k), where v2 is the 2-adic
// valuation, s2 the binary digit sum, and the minimal counterexample at the
// first false width is x = k, whose product is k! itself.
//
// Why it matters: truth of L_k as a function of W is an initial segment
// [1 ..= k - s2(k)]. Choosing k places the threshold anywhere. A closed form
// that says "true" agrees with an exhaustive sweep at EVERY width below the
// threshold, so any model band ending below the threshold certifies agreement
// while the law is false at a shipped width above it. k = 16 defeats a band
// ending at 15; k = 64 is true through width 63 and false exactly at 64.
//
// This probe validates the theory against exhaustive sweeps, checks witness
// embedding (a counterexample at width W refutes at every wider width), checks
// that signed and unsigned wrapping are bit-identical (so the sign dimension
// collapses for this fragment), runs the INV3 constant-reduction law as the
// natural-law sibling, and runs a deliberate wrong-threshold control so the
// instrument is shown able to fail.
//
// Toolchain: pinned nightly-2026-05-28. Runtime probe; no feature gates.

fn mask_of(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

/// (x)(x-1)...(x-k+1) with wrapping arithmetic at width w.
fn falling_prod(x: u64, k: u32, w: u32) -> u64 {
    let m = mask_of(w);
    let mut acc: u64 = 1;
    let mut i: u64 = 0;
    while i < k as u64 {
        let f = x.wrapping_sub(i) & m;
        acc = acc.wrapping_mul(f) & m;
        i += 1;
    }
    acc
}

/// Exhaustive verdict of L_k at width w, with the minimal counterexample if false.
fn sweep(k: u32, w: u32) -> Option<u64> {
    let n: u64 = 1u64 << w;
    let mut x = 0u64;
    while x < n {
        if falling_prod(x, k, w) != 0 {
            return Some(x);
        }
        x += 1;
    }
    None
}

fn v2_factorial_legendre(k: u32) -> u32 {
    // sum_{i>=1} floor(k / 2^i)
    let mut s = 0u32;
    let mut p = 2u64;
    while p <= k as u64 {
        s += (k as u64 / p) as u32;
        p *= 2;
    }
    s
}

fn main() {
    let cap: u32 = 18; // exhaustive sweep cap per width (2^18 evaluations)
    let embed_cap: u32 = 22;

    println!("p1: the threshold family L_k, theory against exhaustive sweep\n");

    // Legendre's formula against k - s2(k), for every k a probe below uses.
    let mut bad_legendre = 0u32;
    for k in 2u32..=128 {
        if v2_factorial_legendre(k) != k - k.count_ones() {
            bad_legendre += 1;
        }
    }
    println!(
        "two spellings of v2(k!) (Legendre sum vs k - s2(k)), k = 2..=128: {} disagreements",
        bad_legendre
    );
    assert!(bad_legendre == 0);

    println!(
        "\n{:>4} {:>10} {:>12} {:>14} {:>10} {:>18}",
        "k", "W* = v2(k!)", "true widths", "first false W", "witness", "witness embeds to"
    );

    let ks = [2u32, 3, 4, 5, 6, 7, 8, 12, 16];
    let mut family_defects = 0u32;
    for &k in &ks {
        let wstar = k - k.count_ones();
        let mut first_false: Option<(u32, u64)> = None;
        let mut segment_ok = true;
        let mut last_true = 0u32;
        for w in 1..=cap {
            match sweep(k, w) {
                None => {
                    // true at w; must not come after a false width (initial segment)
                    if first_false.is_some() {
                        segment_ok = false;
                    }
                    last_true = w;
                }
                Some(x0) => {
                    if first_false.is_none() {
                        first_false = Some((w, x0));
                    }
                }
            }
        }
        let (fw, wit) = first_false.expect("every k here has a threshold below the cap");
        // theory checks
        if last_true != wstar || fw != wstar + 1 || wit != k as u64 {
            family_defects += 1;
        }
        if !segment_ok {
            family_defects += 1;
        }
        // witness embedding: the first-false witness refutes at every wider width
        let mut embeds_to = fw;
        for w in fw..=embed_cap {
            if falling_prod(wit, k, w) != 0 {
                embeds_to = w;
            } else {
                break;
            }
        }
        println!(
            "{:>4} {:>10} {:>12} {:>14} {:>10} {:>18}",
            k,
            wstar,
            format!("1..={}", last_true),
            fw,
            wit,
            format!("{} (cap)", embeds_to)
        );
        assert!(embeds_to == embed_cap, "witness failed to embed upward");
    }
    println!(
        "\ntheory defects (segment shape, threshold value, witness identity): {}",
        family_defects
    );
    assert!(family_defects == 0);

    // k = 64: threshold 63 is unsweepable; pin it by the residue instead.
    // 64! = 2^63 * m with m odd, so 64! mod 2^64 must be exactly 1 << 63.
    let p64 = falling_prod(64, 64, 64);
    println!(
        "\nk = 64: wrapping product at x = 64, width 64 = {:#x} (expected {:#x})",
        p64,
        1u64 << 63
    );
    assert!(p64 == 1u64 << 63);
    // and truth at every sweepable width (theory says true through 63):
    let mut k64_false_below = 0u32;
    for w in 1..=cap {
        if sweep(64, w).is_some() {
            k64_false_below += 1;
        }
    }
    println!(
        "k = 64: exhaustive sweep at widths 1..={}: {} false widths (theory: 0, true through 63)",
        cap, k64_false_below
    );
    assert!(k64_false_below == 0);

    // Signed/unsigned wrapping bit-identity at width 8, all pairs, add/sub/mul.
    let mut sign_diffs = 0u64;
    for a in 0u64..=255 {
        for b in 0u64..=255 {
            let (ua, ub) = (a as u8, b as u8);
            let (ia, ib) = (a as u8 as i8, b as u8 as i8);
            if ua.wrapping_add(ub) != ia.wrapping_add(ib) as u8 {
                sign_diffs += 1;
            }
            if ua.wrapping_sub(ub) != ia.wrapping_sub(ib) as u8 {
                sign_diffs += 1;
            }
            if ua.wrapping_mul(ub) != ia.wrapping_mul(ib) as u8 {
                sign_diffs += 1;
            }
        }
    }
    println!(
        "\nsigned vs unsigned wrapping add/sub/mul, width 8, all pairs: {} bit differences",
        sign_diffs
    );
    assert!(sign_diffs == 0);

    // The natural-law sibling: a hardcoded multiplicative inverse.
    // INV3 is the inverse of 3 mod 2^64. The law (x * 3) * INV3 == x is true at
    // every width <= 64 because congruence mod 2^64 implies congruence mod 2^W,
    // and false above 65 because 3 * INV3 = 2^65 + 1.
    const INV3: u64 = 0xAAAA_AAAA_AAAA_AAAB;
    let mut inv3_false = 0u32;
    for w in 1..=cap {
        let m = mask_of(w);
        let inv = INV3 & m;
        let mut x = 0u64;
        let n = 1u64 << w;
        let mut bad = false;
        while x < n {
            if x.wrapping_mul(3).wrapping_mul(inv) & m != x {
                bad = true;
                break;
            }
            x += 1;
        }
        if bad {
            inv3_false += 1;
        }
    }
    let three_inv3: u128 = 3u128 * INV3 as u128;
    println!(
        "\nINV3 law ((x*3)*INV3 == x): false widths in 1..={}: {} (theory: 0)",
        cap, inv3_false
    );
    println!(
        "3 * INV3 as an integer = {:#x} (= 2^65 + 1, so the law is FALSE mod 2^128: witness x = 1 gives {:#x})",
        three_inv3,
        (1u128.wrapping_mul(3).wrapping_mul(INV3 as u128))
    );
    assert!(inv3_false == 0);
    assert!(three_inv3 == (1u128 << 65) + 1);

    // Wrong-threshold control: for k = 6 the formula k - 1 = 5 is wrong (true
    // threshold is 4). The sweep must catch it, or this instrument cannot fail.
    let wrong_wstar = 6u32 - 1;
    let caught = sweep(6, wrong_wstar).is_some();
    println!(
        "\ncontrol: asserting the WRONG threshold k-1 for k = 6 (claims true at width {}): sweep {}",
        wrong_wstar,
        if caught {
            "REFUTES it (correct)"
        } else {
            "FAILS TO REFUTE (instrument broken)"
        }
    );
    assert!(caught);

    println!("\nall checks passed");
}
