// p5: do the law failures actually produce wrong answers from a real
// algorithm, or are they violated identities nobody reaches?
//
// p2, p2b and p4 count law failures on pairs and triples. That is one kind of
// instrument. This is a different kind: it runs two of the algorithm layer's
// actual routines over the numeral and compares the answer against exact
// unbounded arithmetic. An algorithm can violate a law on many triples and
// still be right, because it may never visit the triples that break, so
// counting the law is not the same as counting the error.
//
// Two routines, both DAG dynamic programmes over a fixed topological order,
// which is the shape arvo-graph's longest_path already has:
//
//   longest   best[v] = max over incoming (u,v) of best[u] + w(u,v)   (max-plus)
//   shortest  best[v] = min over incoming (u,v) of best[u] + w(u,v)   (min-plus,
//             with the numeral's top standing in for infinity, which is the
//             only thing a bounded numeral has to stand it on)
//
// THE CONTROL THAT MAKES THIS FAIR. An instance is only counted when the exact
// answer, and every exact intermediate, fits inside the numeral's range. Then
// a disagreement cannot be blamed on the numeral being too narrow for the
// problem: the numeral could have held every value involved, and the arithmetic
// still gave a different answer. Without this control the measurement would be
// reporting range exhaustion and calling it a law failure.
//
// Exhaustive over every DAG on 4 nodes with edges respecting one topological
// order (2^6 = 64 shapes) and every weight assignment from the value set.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p5 p5_algorithm_end_to_end.rs && ./p5

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
}

const N: usize = 4;
const E: usize = 6; // pairs (i,j) with i<j over 4 nodes

const EDGES: [(usize, usize); E] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

#[inline(always)]
fn add(a: u128, b: u128, w: u32, p: Policy) -> u128 {
    let m: u128 = 1u128 << w;
    let s = a + b;
    match p {
        Policy::Wrap => s & (m - 1),
        Policy::Saturate => {
            if s >= m {
                m - 1
            } else {
                s
            }
        }
    }
}

// Longest path from node 0, in the numeral. `None` for unreachable.
fn longest(mask: u32, wts: &[u128; E], w: u32, p: Policy) -> [Option<u128>; N] {
    let mut best: [Option<u128>; N] = [None; N];
    best[0] = Some(0);
    for v in 1..N {
        let mut acc: Option<u128> = None;
        for (e, &(a, b)) in EDGES.iter().enumerate() {
            if b != v || (mask >> e) & 1 == 0 {
                continue;
            }
            if let Some(bu) = best[a] {
                let cand = add(bu, wts[e], w, p);
                acc = Some(match acc {
                    None => cand,
                    Some(x) => {
                        if cand > x {
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

// Shortest path from node 0, with the numeral's top standing for infinity.
// Written the way a bounded implementation has to write it: there is no
// infinity, so TOP is it, and TOP + w had better still be TOP.
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

// Exact versions, unbounded. `None` is genuinely unreachable.
fn longest_exact(mask: u32, wts: &[u128; E]) -> [Option<u128>; N] {
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
                        if cand > x {
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

    for w in 3..=4u32 {
        let card: u128 = 1u128 << w;
        let top: u128 = card - 1;

        for (p, pn) in [(Policy::Wrap, "wrap"), (Policy::Saturate, "saturate")] {
            let mut lg_total = 0u64;
            let mut lg_wrong = 0u64;
            let mut sp_total = 0u64;
            let mut sp_wrong = 0u64;
            let mut lg_witness: Option<(u32, [u128; E])> = None;
            let mut sp_witness: Option<(u32, [u128; E])> = None;

            for mask in 0..(1u32 << E) {
                let combos = card.pow(E as u32);
                for code in 0..combos {
                    let mut c = code;
                    let mut wts = [0u128; E];
                    for i in 0..E {
                        wts[i] = c % card;
                        c /= card;
                    }

                    // ---- longest ----
                    let ex = longest_exact(mask, &wts);
                    // In range when every reachable exact best fits strictly
                    // inside the representable set, top included.
                    let in_range = ex.iter().all(|o| o.map_or(true, |v| v <= top));
                    if in_range {
                        lg_total += 1;
                        let got = longest(mask, &wts, w, p);
                        if got != ex {
                            lg_wrong += 1;
                            if lg_witness.is_none() {
                                lg_witness = Some((mask, wts));
                            }
                        }
                    }

                    // ---- shortest ----
                    let exs = shortest_exact(mask, &wts);
                    // In range, and additionally every unreachable node must be
                    // distinguishable from a real distance of top, else the
                    // sentinel is ambiguous for reasons that are not the
                    // arithmetic's fault. Require every reachable exact best to
                    // be strictly below top.
                    let in_range_s = exs.iter().all(|o| o.map_or(true, |v| v < top));
                    if in_range_s {
                        sp_total += 1;
                        let got = shortest_bounded(mask, &wts, w, p);
                        let agree = (0..N).all(|v| match exs[v] {
                            Some(x) => got[v] == x,
                            None => got[v] == top, // unreachable stays at infinity
                        });
                        if !agree {
                            sp_wrong += 1;
                            if sp_witness.is_none() {
                                sp_witness = Some((mask, wts));
                            }
                        }
                    }
                }
            }

            println!(
                "{},{},longest_max_plus,{},{},{:.4}",
                w,
                pn,
                lg_total,
                lg_wrong,
                100.0 * lg_wrong as f64 / lg_total.max(1) as f64
            );
            println!(
                "{},{},shortest_min_plus,{},{},{:.4}",
                w,
                pn,
                sp_total,
                sp_wrong,
                100.0 * sp_wrong as f64 / sp_total.max(1) as f64
            );
            if let Some((m, ws)) = lg_witness {
                eprintln!(
                    "w={} {} longest wrong: edge mask {:06b}, weights {:?}",
                    w, pn, m, ws
                );
            }
            if let Some((m, ws)) = sp_witness {
                eprintln!(
                    "w={} {} shortest wrong: edge mask {:06b}, weights {:?}",
                    w, pn, m, ws
                );
            }
        }
    }
}
