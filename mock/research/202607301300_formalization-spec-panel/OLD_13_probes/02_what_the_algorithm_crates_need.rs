//! Probe 02: which law does `arvo-graph` actually depend on?
//!
//! File 12 (section 5a) says the ladder's first clients are arvo's algorithm
//! crates, and that if they bound on it "a signed `Warm` edge weight in
//! `upward_rank` would be refused, because clamping is not translation-stable."
//! That premise treats `upward_rank` as a fold over `+`. Read the source:
//!
//!   arvo-graph/src/rank.rs:84    rank[node] = if any { w + best } else { w }
//!   arvo-graph/src/path.rs:81    this_best = if any_pred { w + top } else { w }
//!
//! where `best` / `top` is a MAX over already-computed neighbour values
//! (rank.rs:70-82, path.rs:65-79). The reduction is over `max`. Addition is
//! applied exactly once per node, with its grouping pinned by the graph.
//! Nothing is ever regrouped, so associativity of `+` is not what makes the
//! answer come out.
//!
//! These are max-plus (tropical) recurrences. What they need is
//!
//!     w + max(a, b) == max(w + a, w + b)                      (D)
//!
//! which for a total order is monotonicity of `+` in its second argument. This
//! probe checks (D) and the algorithm's own claim ("the maximum path weight")
//! for each of the four preset arithmetics, exhaustively over every DAG on N
//! nodes and every weight assignment from a small set.
//!
//! Run: rustc -O 02_what_the_algorithm_crates_need.rs -o /tmp/p02 && /tmp/p02

const MIN: i32 = -4;
const MAX: i32 = 3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Arith {
    /// `Hot`: reduce modulo. The one preset the draft says folds when signed.
    Wrap,
    /// `Warm` / `Cold`: clamp. Non-associative when signed (probe 01).
    Saturate,
    /// A quantiser that substitutes zero out of range.
    SubZero,
    /// The exact answer in a wider carrier, as the oracle.
    Exact,
}

fn q(a: Arith, x: i32) -> i32 {
    match a {
        Arith::Wrap => {
            let n = MAX - MIN + 1;
            ((x - MIN).rem_euclid(n)) + MIN
        }
        Arith::Saturate => x.clamp(MIN, MAX),
        Arith::SubZero => {
            if x < MIN || x > MAX {
                0
            } else {
                x
            }
        }
        Arith::Exact => x,
    }
}

fn add(a: Arith, x: i32, y: i32) -> i32 {
    q(a, x + y)
}

fn reps() -> impl Iterator<Item = i32> {
    MIN..=MAX
}

/// (D): `w + max(a, b) == max(w + a, w + b)`, over every representable triple.
fn distributes(a: Arith) -> Option<(i32, i32, i32)> {
    for w in reps() {
        for x in reps() {
            for y in reps() {
                let l = add(a, w, x.max(y));
                let r = add(a, w, x).max(add(a, w, y));
                if l != r {
                    return Some((w, x, y));
                }
            }
        }
    }
    None
}

/// Associativity of `+` alone, for contrast. Probe 01 established this at a
/// different width; repeated here so both columns come from one run.
fn associates(a: Arith) -> Option<(i32, i32, i32)> {
    for x in reps() {
        for y in reps() {
            for z in reps() {
                if add(a, add(a, x, y), z) != add(a, x, add(a, y, z)) {
                    return Some((x, y, z));
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// The recurrence itself, and the specification it claims to meet.
// ---------------------------------------------------------------------------

const N: usize = 4;

/// `upward_rank` / `longest_path` transcribed to a dense model. Nodes are in
/// topological order (edges go low -> high), so a single forward sweep gives
/// every predecessor before it is read, which is what `path.rs:49-96` does.
fn dp(a: Arith, edges: u32, w: &[i32; N]) -> i32 {
    let mut best = [0i32; N];
    let mut overall = i32::MIN;
    for v in 0..N {
        let mut top = 0i32;
        let mut any = false;
        for p in 0..v {
            if edges >> (p * N + v) & 1 == 1 {
                if !any || best[p] > top {
                    top = best[p];
                    any = true;
                }
            }
        }
        best[v] = if any { add(a, w[v], top) } else { w[v] };
        overall = overall.max(best[v]);
    }
    overall
}

/// The specification the doc comments state: "the maximum path weight ending at
/// any node" (path.rs:23). Enumerate every path, sum its weights, take the max.
/// `right_nested` picks the grouping: the DP builds `w_last + (.. + w_first)`,
/// so a spec summed the other way is a different function whenever `+` is not
/// associative.
fn spec(a: Arith, edges: u32, w: &[i32; N], right_nested: bool) -> i32 {
    fn paths(edges: u32, v: usize, acc: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        acc.push(v);
        out.push(acc.clone());
        for s in (v + 1)..N {
            if edges >> (v * N + s) & 1 == 1 {
                paths(edges, s, acc, out);
            }
        }
        acc.pop();
    }
    let mut all = Vec::new();
    for r in 0..N {
        let has_pred = (0..r).any(|p| edges >> (p * N + r) & 1 == 1);
        if !has_pred {
            paths(edges, r, &mut Vec::new(), &mut all);
        }
    }
    let mut m = i32::MIN;
    for p in &all {
        let val = if right_nested {
            // fold from the far end back toward the start: w0 + (w1 + (w2 + ..))
            p.iter().rev().fold(None::<i32>, |acc, &i| match acc {
                None => Some(w[i]),
                Some(s) => Some(add(a, w[i], s)),
            })
        } else {
            p.iter().fold(None::<i32>, |acc, &i| match acc {
                None => Some(w[i]),
                Some(s) => Some(add(a, s, w[i])),
            })
        };
        m = m.max(val.unwrap());
    }
    m
}

fn main() {
    println!("model: representable [{}, {}], {} nodes\n", MIN, MAX, N);

    println!(
        "{:<10} {:<28} {:<28}",
        "arith", "+ associative", "+ distributes over max (D)"
    );
    for a in [Arith::Wrap, Arith::Saturate, Arith::SubZero, Arith::Exact] {
        let asc = associates(a);
        let dis = distributes(a);
        println!(
            "{:<10} {:<28} {:<28}",
            format!("{:?}", a),
            match asc {
                None => "yes".to_string(),
                Some(t) => format!("NO at {:?}", t),
            },
            match dis {
                None => "yes".to_string(),
                Some(t) => format!("NO at {:?}", t),
            }
        );
    }

    // Exhaustive: every upper-triangular edge set on N nodes, every weight
    // vector over a coarse grid of the representable set.
    let grid: Vec<i32> = vec![MIN, -1, 0, 1, MAX];
    let mut edge_sets = Vec::new();
    for e in 0u32..(1u32 << (N * N)) {
        let mut ok = true;
        for i in 0..N {
            for j in 0..N {
                if j <= i && e >> (i * N + j) & 1 == 1 {
                    ok = false;
                }
            }
        }
        if ok {
            edge_sets.push(e);
        }
    }

    println!(
        "\ndp result vs the stated specification, over {} DAGs x {} weight vectors",
        edge_sets.len(),
        grid.len().pow(N as u32)
    );
    println!(
        "{:<10} {:<26} {:<26}",
        "arith", "dp == spec(right-nested)", "dp == spec(left-nested)"
    );
    for a in [Arith::Wrap, Arith::Saturate, Arith::SubZero, Arith::Exact] {
        let mut bad_r: Option<(u32, [i32; N], i32, i32)> = None;
        let mut bad_l: Option<(u32, [i32; N], i32, i32)> = None;
        for &e in &edge_sets {
            for i0 in &grid {
                for i1 in &grid {
                    for i2 in &grid {
                        for i3 in &grid {
                            let w = [*i0, *i1, *i2, *i3];
                            let d = dp(a, e, &w);
                            if bad_r.is_none() {
                                let s = spec(a, e, &w, true);
                                if d != s {
                                    bad_r = Some((e, w, d, s));
                                }
                            }
                            if bad_l.is_none() {
                                let s = spec(a, e, &w, false);
                                if d != s {
                                    bad_l = Some((e, w, d, s));
                                }
                            }
                        }
                    }
                }
            }
        }
        let fmt = |b: Option<(u32, [i32; N], i32, i32)>| match b {
            None => "yes".to_string(),
            Some((e, w, d, s)) => format!("NO e={:#x} w={:?} dp={} spec={}", e, w, d, s),
        };
        println!(
            "{:<10} {:<26} {:<26}",
            format!("{:?}", a),
            fmt(bad_r),
            fmt(bad_l)
        );
    }
}
