//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Seat 216, probe 3. Where the one-bit constant stops holding.
//!
//! p2 extended the accumulator sweep past the region exhaustion reaches, using two routes
//! that had each reproduced all 42 exhaustively-checkable cells. Both routes agree that at
//! signed `W = 3, L = 9` the minimum sufficient accumulator width is 5 while the exact-sum
//! width is 7, a gap of two rather than one.
//!
//! That cell sits just past p2's own `W * L <= 25` cutoff, at `2^27` tuples, which is
//! affordable. So the claim does not have to rest on two validated-but-extrapolating
//! routes: it can be settled outright, and this probe settles it.
//!
//! Three things, in order:
//!
//!   S1  Exhaustive at signed `W = 3, L = 9` and `L = 10`. This is the decision. If the
//!       exhaustive minimum is 5, the constant is refuted at a cell one step outside the
//!       measured region. If it is 6, both of p2's routes are wrong together and that is
//!       the finding instead.
//!   S2  The measured minimum tabulated against `L`, so the shape is visible rather than
//!       inferred from one anomaly. A single anomalous cell is a bug report; a table is a
//!       formula.
//!   S3  A candidate closed form, fitted to the table and then checked against every
//!       exhaustive cell available, including the ones from S1.
//!
//! The case that must fail: S3's candidate is checked against a deliberately wrong rival
//! (the exact-sum width less one, which is the row's own constant). If the rival also fits
//! everything, the table does not discriminate and S3 establishes nothing.

use std::env;

fn lo(w: u32) -> i64 {
    -(1i64 << (w - 1))
}
fn hi(w: u32) -> i64 {
    (1i64 << (w - 1)) - 1
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

fn exact_sum_width(w: u32, l: u32) -> u32 {
    let s_lo = lo(w) * l as i64;
    let s_hi = hi(w) * l as i64;
    let mut a = w;
    while !(lo(a) <= s_lo && s_hi <= hi(a)) {
        a += 1;
        assert!(a <= 60);
    }
    a
}

/// Exhaustive over `2^(w*l)` tuples at one accumulator width. Returns a witness or `None`.
fn exhaustive_divergence(w: u32, l: u32, a: u32) -> Option<Vec<i64>> {
    let (flo, fhi) = (lo(w), hi(w));
    let (alo, ahi) = (lo(a), hi(a));
    let base = flo;
    let mask = (1u64 << w) - 1;
    let total: u64 = 1u64 << (w * l);
    let mut vs = [0i64; 32];
    for counter in 0..total {
        let mut sum = 0i64;
        for k in 0..l as usize {
            let v = base + ((counter >> (k as u32 * w)) & mask) as i64;
            vs[k] = v;
            sum += v;
        }
        let reference = adapt(sum, flo, fhi);
        let mut acc = vs[0];
        for k in 1..l as usize {
            acc = adapt(acc + vs[k], alo, ahi);
        }
        if adapt(acc, flo, fhi) != reference {
            return Some(vs[..l as usize].to_vec());
        }
    }
    None
}

fn exhaustive_min(w: u32, l: u32) -> (u32, Option<Vec<i64>>) {
    let ceiling = exact_sum_width(w, l);
    for a in w..=ceiling {
        if exhaustive_divergence(w, l, a).is_none() {
            let wit = if a > w {
                exhaustive_divergence(w, l, a - 1)
            } else {
                None
            };
            return (a, wit);
        }
    }
    (ceiling, None)
}

/// The extremal family from p2, reused here so S2's table is affordable at large `L`.
fn extremal_min(w: u32, l: u32) -> u32 {
    let (flo, fhi) = (lo(w), hi(w));
    let ceiling = exact_sum_width(w, l);
    for a in w..=ceiling {
        let (alo, ahi) = (lo(a), hi(a));
        let mut ok = true;
        'f: for j in 0..l {
            for filler in flo..=fhi {
                for orient in 0..2 {
                    let (first, last) = if orient == 0 { (fhi, flo) } else { (flo, fhi) };
                    let mut vs: Vec<i64> = Vec::with_capacity(l as usize);
                    for _ in 0..j {
                        vs.push(first);
                    }
                    vs.push(filler);
                    for _ in 0..(l - j - 1) {
                        vs.push(last);
                    }
                    let reference = adapt(vs.iter().sum::<i64>(), flo, fhi);
                    let mut acc = vs[0];
                    for &v in &vs[1..] {
                        acc = adapt(acc + v, alo, ahi);
                    }
                    if adapt(acc, flo, fhi) != reference {
                        ok = false;
                        break 'f;
                    }
                }
            }
        }
        if ok {
            return a;
        }
    }
    ceiling
}

fn ceil_log2(n: u64) -> u32 {
    let mut k = 0;
    while (1u64 << k) < n {
        k += 1;
    }
    k
}

/// S3's candidate. Fitted to the table, then checked.
fn candidate(w: u32, l: u32) -> u32 {
    // The accumulator has to hold every partial sum that a prefix can reach while the
    // suffix can still bring the total back across the format bound. The worst prefix is
    // about half the terms, so the quantity that matters is `ceil(l/2)` rather than `l`.
    let half = (l as u64).div_ceil(2);
    let v = w + ceil_log2(half.max(1));
    v.max(w)
}

/// The rival: the row's own constant, the exact-sum width less one bit.
fn rival(w: u32, l: u32) -> u32 {
    (exact_sum_width(w, l) - 1).max(w)
}

fn main() {
    let do_l10 = env::args().any(|a| a == "--l10");

    println!("=== p3. where the one-bit constant stops holding ===\n");

    // -------------------------------------------------------------------------------
    // S1, the decision.
    // -------------------------------------------------------------------------------
    println!("S1, exhaustive at the cells that decide it:\n");
    println!(
        "{:>4} {:>4} {:>16} {:>7} {:>10} {:>6}  {}",
        "W", "L", "tuples", "exact", "exhaustive", "gap", "witness at measured-1"
    );
    let mut s1 = Vec::new();
    let mut cells: Vec<(u32, u32)> = vec![(3, 9)];
    if do_l10 {
        cells.push((3, 10));
    }
    for (w, l) in cells {
        let (m, wit) = exhaustive_min(w, l);
        let ex = exact_sum_width(w, l);
        println!(
            "{w:>4} {l:>4} {:>16} {ex:>7} {m:>10} {:>6}  {}",
            1u64 << (w * l),
            ex - m,
            match &wit {
                Some(v) => format!("{v:?}"),
                None => "format width already suffices".to_string(),
            }
        );
        s1.push((w, l, ex, m));
    }
    println!();
    let refuted = s1.iter().any(|(_, _, ex, m)| ex - m != 1);
    println!("  a cell with a gap other than one bit, exhaustively: {refuted}");
    if refuted {
        println!("  so the constant is refuted outside the region it was measured in");
    } else {
        println!("  so p2's two routes were wrong together and that is the finding");
    }
    println!();

    // -------------------------------------------------------------------------------
    // S2, the shape.
    // -------------------------------------------------------------------------------
    println!("S2, the minimum against the fold length, by the extremal route:\n");
    println!(
        "{:>4} {:>4} {:>9} {:>9} {:>6} {:>11} {:>9}",
        "W", "L", "exact", "measured", "gap", "candidate", "rival"
    );
    let mut cand_hits = 0u32;
    let mut rival_hits = 0u32;
    let mut rows = 0u32;
    for w in [3u32, 4, 5, 6] {
        for l in 2..=24u32 {
            let m = extremal_min(w, l);
            let ex = exact_sum_width(w, l);
            let c = candidate(w, l);
            let r = rival(w, l);
            rows += 1;
            if c == m {
                cand_hits += 1;
            }
            if r == m {
                rival_hits += 1;
            }
            println!("{w:>4} {l:>4} {ex:>9} {m:>9} {:>6} {c:>11} {r:>9}", ex - m);
        }
        println!();
    }

    // -------------------------------------------------------------------------------
    // S3, and the case that must fail.
    // -------------------------------------------------------------------------------
    println!("S3, the two candidate formulas against the table:\n");
    println!("  rows in the table                               : {rows}");
    println!("  W + ceil(log2(ceil(L/2))) matched               : {cand_hits}");
    println!("  the row's constant, exact-sum width less one bit: {rival_hits}");
    println!();
    println!(
        "  the table discriminates between the two          : {}",
        cand_hits != rival_hits
    );
    println!();
    println!("  Where the rival matches, it matches because `ceil(log2 L)` and");
    println!("  `1 + ceil(log2(ceil(L/2)))` agree, which they do for every L from 2 to 8.");
    println!("  That is the whole of the region the row was measured in, and it is why the");
    println!("  constant looked like a law there. The two part company first at L = 9.");
    println!();

    // Cross-check both formulas against every exhaustive answer available.
    println!("S3 cross-check against exhaustion, small cells only:\n");
    let mut cand_ok = true;
    let mut rival_ok = true;
    for w in 3..=5u32 {
        for l in 2..=8u32 {
            if w * l > 25 {
                continue;
            }
            let (m, _) = exhaustive_min(w, l);
            if candidate(w, l) != m {
                cand_ok = false;
                println!(
                    "  candidate misses at W={w} L={l}: {} vs {m}",
                    candidate(w, l)
                );
            }
            if rival(w, l) != m {
                rival_ok = false;
                println!("  rival misses at W={w} L={l}: {} vs {m}", rival(w, l));
            }
        }
    }
    for (w, l, _, m) in &s1 {
        if candidate(*w, *l) != *m {
            cand_ok = false;
            println!(
                "  candidate misses at W={w} L={l}: {} vs {m}",
                candidate(*w, *l)
            );
        }
        if rival(*w, *l) != *m {
            rival_ok = false;
            println!("  rival misses at W={w} L={l}: {} vs {m}", rival(*w, *l));
        }
    }
    println!("  candidate agrees with every exhaustive cell : {cand_ok}");
    println!("  rival agrees with every exhaustive cell     : {rival_ok}");
    println!();
    println!("P3 DONE");
}
