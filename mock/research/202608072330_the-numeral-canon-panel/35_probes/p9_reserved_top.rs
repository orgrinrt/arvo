// p9: attacking my own proposal.
//
// Section 5's Q6 entry offered a shape nobody had written down: a numeral that
// WRAPS in its interior, so it keeps the additive inverse and behaves "like
// native primitives in regular old rust would", but reserves its top as an
// ABSORBING value, so min-plus algorithms get the infinity they need. That
// would serve both consumers at the cost of one value.
//
// I offered it without building it, which is the thing this panel's rules
// forbid, so this builds it. It runs the identical harness p5 used, with a
// third policy added, and it is written to refute the proposal rather than to
// confirm it: the in-range control is the same, the routines are the same, and
// the exact reference is the same.
//
// The prediction I want to test is my own, and I expect it to fail. Min-plus
// needs two things from the arithmetic, not one: an absorbing top, AND
// monotonicity (a <= b implies a+c <= b+c), because the relaxation's greedy
// argument rests on a path never getting shorter by being extended. A reserved
// top buys the first. Wrapping in the interior still destroys the second, so a
// finite distance plus a weight can wrap to something smaller and the routine
// accepts a path that does not exist.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p9 p9_reserved_top.rs && ./p9

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
    // Wrapping below a reserved absorbing top. The representable finite range
    // is [0, TOP-1]; TOP means infinity and absorbs.
    ReservedTop,
}

const N: usize = 4;
const E: usize = 6;
const EDGES: [(usize, usize); E] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

#[inline(always)]
fn add(a: u128, b: u128, w: u32, p: Policy) -> u128 {
    let m: u128 = 1u128 << w;
    let top = m - 1;
    match p {
        Policy::Wrap => (a + b) & (m - 1),
        Policy::Saturate => {
            let s = a + b;
            if s >= m {
                m - 1
            } else {
                s
            }
        }
        Policy::ReservedTop => {
            if a == top || b == top {
                top
            } else {
                // Wrap within the finite range [0, top-1], i.e. modulo top.
                (a + b) % top
            }
        }
    }
}

fn shortest_bounded(mask: u32, wts: &[u128; E], w: u32, p: Policy) -> [u128; N] {
    let top: u128 = (1u128 << w) - 1;
    let mut best: [u128; N] = [top; N];
    best[0] = 0;
    for v in 1..N {
        let mut acc = top;
        for (e, &(a, b)) in EDGES.iter().enumerate() {
            if b != v || (mask >> e) & 1 == 0 {
                continue;
            }
            let cand = add(best[a], wts[e], w, p);
            if cand < acc {
                acc = cand;
            }
        }
        best[v] = acc;
    }
    best
}

fn shortest_exact(mask: u32, wts: &[u128; E]) -> [Option<u128>; N] {
    let mut best: [Option<u128>; N] = [None; N];
    best[0] = Some(0);
    for v in 1..N {
        let mut acc: Option<u128> = None;
        for (e, &(a, b)) in EDGES.iter().enumerate() {
            if b != v || (mask >> e) & 1 == 0 {
                continue;
            }
            if let Some(bu) = best[a] {
                let cand = bu + wts[e];
                acc = Some(match acc {
                    None => cand,
                    Some(x) => {
                        if cand < x {
                            cand
                        } else {
                            x
                        }
                    }
                });
            }
        }
        best[v] = acc;
    }
    best
}

fn main() {
    println!("w,policy,routine,in_range_instances,wrong_answers,pct");
    println!("w,policy,law,failures,total,pct");

    for w in 3..=4u32 {
        let card: u128 = 1u128 << w;
        let top: u128 = card - 1;

        for (p, pn) in [
            (Policy::Wrap, "wrap"),
            (Policy::Saturate, "saturate"),
            (Policy::ReservedTop, "reserved_top"),
        ] {
            // --- the same end-to-end test p5 ran ---
            let mut sp_total = 0u64;
            let mut sp_wrong = 0u64;
            let mut witness: Option<(u32, [u128; E])> = None;

            for mask in 0..(1u32 << E) {
                let combos = card.pow(E as u32);
                for code in 0..combos {
                    let mut c = code;
                    let mut wts = [0u128; E];
                    for i in 0..E {
                        wts[i] = c % card;
                        c /= card;
                    }
                    // Under ReservedTop the top is not a finite weight, so an
                    // instance using it as a weight is not a fair input. Skip
                    // those for every policy alike, so the three are compared
                    // on identical instance sets.
                    if wts.iter().any(|&x| x == top) {
                        continue;
                    }

                    let exs = shortest_exact(mask, &wts);
                    let in_range = exs.iter().all(|o| o.map_or(true, |v| v < top));
                    if !in_range {
                        continue;
                    }
                    sp_total += 1;
                    let got = shortest_bounded(mask, &wts, w, p);
                    let agree = (0..N).all(|v| match exs[v] {
                        Some(x) => got[v] == x,
                        None => got[v] == top,
                    });
                    if !agree {
                        sp_wrong += 1;
                        if witness.is_none() {
                            witness = Some((mask, wts));
                        }
                    }
                }
            }
            println!(
                "{},{},shortest_min_plus,{},{},{:.4}",
                w,
                pn,
                sp_total,
                sp_wrong,
                100.0 * sp_wrong as f64 / sp_total.max(1) as f64
            );
            if let Some((m, ws)) = witness {
                eprintln!(
                    "w={} {} shortest wrong: mask {:06b}, weights {:?}",
                    w, pn, m, ws
                );
            }

            // --- the two laws min-plus actually needs, counted separately ---
            let mut absorb_fail = 0u64;
            for x in 0..card {
                if add(top, x, w, p) != top {
                    absorb_fail += 1;
                }
            }
            let mut mono_fail = 0u64;
            let mut mono_total = 0u64;
            for a in 0..card {
                for b in a..card {
                    for c in 0..card {
                        mono_total += 1;
                        if add(a, c, w, p) > add(b, c, w, p) {
                            mono_fail += 1;
                        }
                    }
                }
            }
            println!(
                "{},{},absorbing_top,{},{},{:.4}",
                w,
                pn,
                absorb_fail,
                card,
                100.0 * absorb_fail as f64 / card as f64
            );
            println!(
                "{},{},monotonicity_add,{},{},{:.4}",
                w,
                pn,
                mono_fail,
                mono_total,
                100.0 * mono_fail as f64 / mono_total as f64
            );
        }
    }
}
