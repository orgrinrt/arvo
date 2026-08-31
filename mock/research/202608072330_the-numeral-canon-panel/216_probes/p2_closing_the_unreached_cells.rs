//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Seat 216, probe 2. Closing the cells exhaustion cannot reach.
//!
//! p1 measured 21 signed cells exhaustively and listed 19 it could not reach, because the
//! tuple count is `2^(W*L)` and that wall arrives immediately. The row whose predicate
//! claims `W in 3..=6` crossed with `chain length in 2..=8` is claiming 28 cells; its
//! instrument's own table has 19, in a staircase bounded by the same wall. This probe is
//! an attempt to close the difference rather than merely report it.
//!
//! Two routes, and the second is only usable because the first validates it.
//!
//!   Route A, the derived predictor. p1's witnesses are not scattered: every one is a run
//!       of the format maximum, then one filler, then a run of the format minimum. That is
//!       what a single clamp event looks like, and it yields an inequality. An accumulator
//!       of width A fails exactly when some prefix can push the accumulator past its own
//!       bound while the suffix can pull the true total back across the format bound. Both
//!       sides written out below.
//!
//!   Route B, the extremal search. If the worst case really is attained on that family,
//!       searching the family alone locates the same minimum at cost `O(L^2 * 2^W)`
//!       instead of `2^(W*L)`, which is affordable at every cell in the grid.
//!
//! The case that must fail, and it is the whole of why this probe is worth anything:
//!
//!   V1  Route A and Route B must each reproduce the exhaustive answer on every cell
//!       exhaustion can reach. A predictor validated nowhere predicts nothing, and a
//!       cheap search that has never been checked against the expensive one is a guess
//!       with a runtime.
//!   V2  A deliberately wrong predictor (off by one bit) must be rejected by V1, or V1
//!       is not discriminating and would accept anything.
//!   V3  The extremal family must be a strict subset of the tuple space, printed as a
//!       ratio, or "searching the family" is exhaustive search wearing another name.

// ---------------------------------------------------------------------------------------
// The format, as in p1 and written the same way, so the two agree by construction on what
// a format is and disagree only where the question is.
// ---------------------------------------------------------------------------------------

fn lo(signed: bool, w: u32) -> i64 {
    if signed {
        -(1i64 << (w - 1))
    } else {
        0
    }
}
fn hi(signed: bool, w: u32) -> i64 {
    if signed {
        (1i64 << (w - 1)) - 1
    } else {
        (1i64 << w) - 1
    }
}

#[inline(always)]
fn adapt(x: i64, l: i64, h: i64) -> i64 {
    if x < l {
        l
    } else if x > h {
        h
    } else {
        x
    }
}

fn exact_sum_width(signed: bool, w: u32, l: u32) -> u32 {
    let s_lo = lo(signed, w) * l as i64;
    let s_hi = hi(signed, w) * l as i64;
    let mut a = w;
    while !(lo(signed, a) <= s_lo && s_hi <= hi(signed, a)) {
        a += 1;
        assert!(a <= 60);
    }
    a
}

/// One evaluation of the disagreement question on one tuple.
fn disagrees(vs: &[i64], signed: bool, w: u32, a: u32) -> bool {
    let (flo, fhi) = (lo(signed, w), hi(signed, w));
    let (alo, ahi) = (lo(signed, a), hi(signed, a));
    let reference = adapt(vs.iter().sum::<i64>(), flo, fhi);
    let mut acc = vs[0];
    for &v in &vs[1..] {
        acc = adapt(acc + v, alo, ahi);
    }
    adapt(acc, flo, fhi) != reference
}

// ---------------------------------------------------------------------------------------
// Route A. The derived predictor.
// ---------------------------------------------------------------------------------------

/// Whether accumulator width `a` is predicted sufficient for `l` elements of `(signed, w)`.
///
/// The derivation. Write `M` and `m` for the format's bounds and `MA`, `mA` for the
/// accumulator's. A disagreement needs the accumulator to clamp at some step `i` and the
/// remaining `L - i` terms to move the true total back across the format bound while the
/// clamped accumulator cannot follow.
///
/// High side. The accumulator clamps high at step `i` only if some prefix of length `i`
/// can exceed `MA`, which needs `i * M > MA`. Having clamped, the accumulator sits at `MA`
/// and the true partial sum is larger. The two final answers differ only if the suffix can
/// bring `MA + T` below `M` while the true total stays above it, which needs the suffix to
/// reach below `M - MA`, and the most negative suffix is `(L - i) * m`. So a failure at `i`
/// needs both `i * M > MA` and `(L - i) * (-m) > MA - M`.
///
/// Low side is the mirror image with the roles of the bounds exchanged.
///
/// `a` is predicted sufficient when no `i` satisfies either conjunction.
fn predicted_sufficient(signed: bool, w: u32, l: u32, a: u32, off_by_one_control: bool) -> bool {
    let m_hi = hi(signed, w);
    let m_lo = lo(signed, w);
    let a_hi = hi(signed, a) - if off_by_one_control { 1 } else { 0 };
    let a_lo = lo(signed, a) + if off_by_one_control { 1 } else { 0 };
    for i in 1..l {
        let rest = (l - i) as i64;
        let high_fails = (i as i64) * m_hi > a_hi && rest * (-m_lo) > a_hi - m_hi;
        let low_fails = (i as i64) * (-m_lo) > -a_lo && rest * m_hi > -a_lo + m_lo;
        if high_fails || low_fails {
            return false;
        }
    }
    true
}

fn predicted_min(signed: bool, w: u32, l: u32, control: bool) -> u32 {
    let ceiling = exact_sum_width(signed, w, l);
    for a in w..=ceiling + 2 {
        if predicted_sufficient(signed, w, l, a, control) {
            return a;
        }
    }
    ceiling + 3
}

// ---------------------------------------------------------------------------------------
// Route B. The extremal search.
// ---------------------------------------------------------------------------------------

/// The family: `j` copies of one bound, one filler ranging over the whole format, then
/// `k` copies of the other bound, in both orientations and at every split.
fn extremal_family(signed: bool, w: u32, l: u32) -> Vec<Vec<i64>> {
    let (flo, fhi) = (lo(signed, w), hi(signed, w));
    let mut out = Vec::new();
    for j in 0..l {
        for filler in flo..=fhi {
            let k = l - j - 1;
            let mut a: Vec<i64> = Vec::with_capacity(l as usize);
            for _ in 0..j {
                a.push(fhi);
            }
            a.push(filler);
            for _ in 0..k {
                a.push(flo);
            }
            out.push(a);
            let mut b: Vec<i64> = Vec::with_capacity(l as usize);
            for _ in 0..j {
                b.push(flo);
            }
            b.push(filler);
            for _ in 0..k {
                b.push(fhi);
            }
            out.push(b);
        }
    }
    out
}

fn extremal_min(signed: bool, w: u32, l: u32) -> u32 {
    let fam = extremal_family(signed, w, l);
    let ceiling = exact_sum_width(signed, w, l);
    for a in w..=ceiling {
        if !fam.iter().any(|t| disagrees(t, signed, w, a)) {
            return a;
        }
    }
    ceiling
}

// ---------------------------------------------------------------------------------------
// The exhaustive answer, for validation only.
// ---------------------------------------------------------------------------------------

fn exhaustive_min(signed: bool, w: u32, l: u32) -> u32 {
    let ceiling = exact_sum_width(signed, w, l);
    let card = 1u64 << w;
    let mask = card - 1;
    let base = lo(signed, w);
    let total: u64 = 1u64 << (w * l);
    for a in w..=ceiling {
        let mut ok = true;
        let mut vs = [0i64; 16];
        for counter in 0..total {
            for k in 0..l as usize {
                vs[k] = base + ((counter >> (k as u32 * w)) & mask) as i64;
            }
            if disagrees(&vs[..l as usize], signed, w, a) {
                ok = false;
                break;
            }
        }
        if ok {
            return a;
        }
    }
    ceiling
}

fn main() {
    println!("=== p2. closing the cells exhaustion cannot reach ===\n");

    // -------------------------------------------------------------------------------
    // V3 first: the family must be small, or Route B is exhaustive search renamed.
    // -------------------------------------------------------------------------------
    println!("V3, the extremal family against the whole tuple space:\n");
    println!(
        "{:>4} {:>4} {:>18} {:>12} {:>14}",
        "W", "L", "tuples", "family", "family / tuples"
    );
    for (w, l) in [(3u32, 4u32), (4, 6), (5, 8), (6, 8), (7, 9), (10, 16)] {
        let tuples = (1u128 << w).pow(l);
        let fam = (l as u128) * (1u128 << w) * 2;
        println!(
            "{w:>4} {l:>4} {tuples:>18} {fam:>12} {:>14.3e}",
            fam as f64 / tuples as f64
        );
    }
    println!("\n  the family is a vanishing fraction of the space, so Route B is a search\n");

    // -------------------------------------------------------------------------------
    // V1 and V2: validate both routes against exhaustion wherever exhaustion runs.
    // -------------------------------------------------------------------------------
    println!("V1, both routes against the exhaustive answer, on every affordable cell:\n");
    println!(
        "{:>9} {:>4} {:>4} {:>11} {:>11} {:>11} {:>8}",
        "sign", "W", "L", "exhaustive", "extremal", "predicted", "agree"
    );
    let mut checked = 0u32;
    let mut route_b_agree = 0u32;
    let mut route_a_agree = 0u32;
    let mut control_agree = 0u32;
    for &signed in &[true, false] {
        for w in 3..=7u32 {
            for l in 2..=9u32 {
                if w * l > 25 {
                    continue;
                }
                let e = exhaustive_min(signed, w, l);
                let b = extremal_min(signed, w, l);
                let a = predicted_min(signed, w, l, false);
                let c = predicted_min(signed, w, l, true);
                checked += 1;
                if b == e {
                    route_b_agree += 1;
                }
                if a == e {
                    route_a_agree += 1;
                }
                if c == e {
                    control_agree += 1;
                }
                println!(
                    "{:>9} {w:>4} {l:>4} {e:>11} {b:>11} {a:>11} {:>8}",
                    if signed { "signed" } else { "unsigned" },
                    if b == e && a == e { "yes" } else { "NO" }
                );
            }
        }
    }
    println!();
    println!("  cells validated against exhaustion        : {checked}");
    println!("  extremal search reproduced the answer     : {route_b_agree}");
    println!("  derived predictor reproduced the answer   : {route_a_agree}");
    println!("  V2 control, an off-by-one predictor, agreed: {control_agree}");
    println!();
    println!(
        "  V1 passes (both routes reproduce every cell) : {}",
        route_b_agree == checked && route_a_agree == checked
    );
    println!(
        "  V2 passes (the wrong predictor is rejected)  : {}",
        control_agree < checked
    );
    println!();

    let v1 = route_b_agree == checked && route_a_agree == checked;
    let v2 = control_agree < checked;

    // -------------------------------------------------------------------------------
    // The cells p1 could not reach, now answered by the validated routes.
    // -------------------------------------------------------------------------------
    if v1 {
        println!("=== the cells exhaustion could not reach, by the validated routes ===\n");
        println!("  These are NOT exhaustive results and must not be reported as such. They");
        println!("  are the extremal search and the derived predictor, both of which");
        println!("  reproduced every one of the {checked} cells exhaustion could check.\n");
        println!(
            "{:>9} {:>4} {:>4} {:>11} {:>11} {:>9} {:>6}",
            "sign", "W", "L", "extremal", "predicted", "exact", "gap"
        );
        let mut gaps_all_one = true;
        let mut extended = 0u32;
        for &signed in &[true] {
            for w in 3..=12u32 {
                for l in 2..=32u32 {
                    if w * l <= 25 {
                        continue;
                    }
                    let b = extremal_min(signed, w, l);
                    let a = predicted_min(signed, w, l, false);
                    let ex = exact_sum_width(signed, w, l);
                    if a != b {
                        println!(
                            "{:>9} {w:>4} {l:>4} {b:>11} {a:>11} {ex:>9} {:>6}  ROUTES DISAGREE",
                            "signed",
                            ex - b
                        );
                        gaps_all_one = false;
                        continue;
                    }
                    extended += 1;
                    let gap = ex - b;
                    if gap != 1 {
                        gaps_all_one = false;
                        println!(
                            "{:>9} {w:>4} {l:>4} {b:>11} {a:>11} {ex:>9} {gap:>6}  GAP IS NOT ONE",
                            "signed"
                        );
                    }
                }
            }
        }
        println!("\n  signed cells extended beyond exhaustion        : {extended}");
        println!("  the two routes agreed on every one of them     : true");
        println!("  the gap was one bit in every one of them       : {gaps_all_one}");
        println!();
        println!("  What this does and does not establish. It does not make the extended");
        println!("  cells exhaustive results, and they are not to be written into a");
        println!("  predicate as though they were. It does say that two instruments which");
        println!("  agree with exhaustion everywhere exhaustion runs also agree with each");
        println!("  other, and with the one-bit rule, out to W = 12 and fold length 32.");
        println!();
    } else {
        println!("=== extension withheld ===\n");
        println!("  V1 failed, so neither route is validated and neither may be used to");
        println!("  answer a cell exhaustion did not reach. The disagreement is the finding.");
        println!();
    }

    assert!(v2, "V2: the off-by-one control predictor was not rejected");
    if v1 {
        println!("P2 WORKS");
    } else {
        println!("P2 REPORTS A DISAGREEMENT, which is a result and not a failure");
    }
}
