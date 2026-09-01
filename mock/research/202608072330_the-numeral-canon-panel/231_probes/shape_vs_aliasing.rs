// Are association shape and leaf aliasing one coordinate or two?
//
// `229` names `term shapes` as its strongest result, on `82_probes/p2`
// section 3: a length-8 fold, left-nested against balanced tree, diverging on
// 63.6% of samples at a straddling declared window and on 0% at a one-sided
// one. `230` names `leaf_aliasing` and says the axis is not term shape, on
// `111`'s condition that "every leaf occurs at most once".
//
// Neither instrument varied the other's coordinate. `82_probes/p2` draws eight
// independent operands, so every leaf is distinct in all four million samples
// and aliasing never moves. `111`'s condition is stated across both discharge
// checks, so association never moves there either. Two instruments that both
// leave a dimension unvaried agree about it perfectly without measuring it.
//
// This varies both, exhaustively, in one 2x2.
//
//   A, association: left-nested against balanced, over the same operator
//      sequence, so the two are re-associations of one expression.
//   B, aliasing: four distinct leaves against a term in which one variable
//      occupies two leaf positions.
//
// Two questions are asked at every cell.
//
//   Q1 does the VALUE depend on the association? This is `82`'s question,
//      re-asked at a width small enough to enumerate rather than sample.
//   Q2 is the corner rule EXACT, that is, does evaluating the term at the
//      corners of the declared box give the true range? This is `111`'s
//      question, and it is the one the "every leaf occurs at most once"
//      condition guards: the corner rule treats each leaf occurrence as free,
//      so a variable appearing twice is bounded as if its two occurrences
//      were independent.
//
// If Q1 moves with A at both values of B, and Q2 moves with B at both values
// of A, then neither coordinate subsumes the other and both are axes. If Q2
// also moves with A, they are entangled and one row may serve.
//
// Predictions, written before the run:
//   P-A  Q1 diverges at the straddling window under both aliasings.
//   P-B  Q2 is exact for the distinct term and inexact for the aliased term,
//        under both associations.
//   P-C  at a window narrow enough that no clamp is reachable, Q1 is zero
//        everywhere. This is the negative control for Q1: an arm that reports
//        divergence there is measuring the arithmetic and not the association.
//
// Controls, outcomes written before the run:
//   K1  the distinct term must have as many free variables as leaf positions,
//       and the aliased term one fewer. Asserted, not assumed.
//   K2  P-C above: a narrow window must give zero divergence in all four cells.
//   K3  the exact-arithmetic evaluation of the two associations must agree in
//       every cell. If it does not, the two expressions are not
//       re-associations of one term and Q1 measures the wrong thing.
//   K4  the corner rule must be exact for the distinct term in every window
//       tried. That is the textbook result for a monotone term and a failure
//       means the corner computation is wrong rather than that the finding is
//       interesting.

const W: i32 = 4;
const LO: i32 = -(1 << (W - 1)); // -8
const HI: i32 = (1 << (W - 1)) - 1; // 7

fn sat(v: i32) -> i32 {
    if v < LO {
        LO
    } else if v > HI {
        HI
    } else {
        v
    }
}

/// `l1 - l2 + l3 - l4`, associated left to right, every step clamped.
fn left(l: [i32; 4]) -> i32 {
    sat(sat(sat(l[0] - l[1]) + l[2]) - l[3])
}

/// The same operator sequence, associated as a balanced tree.
fn tree(l: [i32; 4]) -> i32 {
    sat(sat(l[0] - l[1]) + sat(l[2] - l[3]))
}

/// The same expression with no clamping anywhere, used only by K3.
fn exact_left(l: [i32; 4]) -> i32 {
    ((l[0] - l[1]) + l[2]) - l[3]
}
fn exact_tree(l: [i32; 4]) -> i32 {
    (l[0] - l[1]) + (l[2] - l[3])
}

/// How the free variables are placed into the four leaf positions.
#[derive(Clone, Copy, PartialEq)]
enum Aliasing {
    /// Four free variables, one per leaf.
    Distinct,
    /// Three free variables; `v1` occupies leaf positions 2 and 3.
    Aliased,
}

impl Aliasing {
    fn free(self) -> usize {
        match self {
            Aliasing::Distinct => 4,
            Aliasing::Aliased => 3,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Aliasing::Distinct => "distinct",
            Aliasing::Aliased => "aliased",
        }
    }
    /// Place free variables into leaf positions.
    fn leaves(self, v: &[i32]) -> [i32; 4] {
        match self {
            Aliasing::Distinct => [v[0], v[1], v[2], v[3]],
            // `v[1]` twice: once subtracted, once added.
            Aliasing::Aliased => [v[0], v[1], v[1], v[2]],
        }
    }
}

/// Every assignment of `n` free variables over `[lo, hi]`.
fn assignments(n: usize, lo: i32, hi: i32) -> Vec<Vec<i32>> {
    let span: Vec<i32> = (lo..=hi).collect();
    let mut out = vec![vec![]];
    for _ in 0..n {
        let mut next = Vec::new();
        for prefix in &out {
            for &x in &span {
                let mut p = prefix.clone();
                p.push(x);
                next.push(p);
            }
        }
        out = next;
    }
    out
}

/// Q1: over how many assignments do the two associations disagree.
fn q1(a: Aliasing, lo: i32, hi: i32) -> (usize, usize, Option<([i32; 4], i32, i32)>) {
    let mut n = 0usize;
    let mut total = 0usize;
    let mut witness = None;
    for v in assignments(a.free(), lo, hi) {
        let l = a.leaves(&v);
        total += 1;
        let (x, y) = (left(l), tree(l));
        if x != y {
            n += 1;
            if witness.is_none() {
                witness = Some((l, x, y));
            }
        }
    }
    (n, total, witness)
}

/// Q2: is the corner rule exact for this association and aliasing over the box.
///
/// The corner rule evaluates the term with each LEAF POSITION independently at
/// `lo` or `hi`, which is what an interval bound over the term's syntax does.
/// The true range enumerates assignments to the FREE VARIABLES, so a variable
/// occupying two positions takes one value in both.
fn q2(a: Aliasing, f: fn([i32; 4]) -> i32, lo: i32, hi: i32) -> (bool, (i32, i32), (i32, i32)) {
    let mut cmin = i32::MAX;
    let mut cmax = i32::MIN;
    for m in 0..16u32 {
        let l = [
            if m & 1 == 0 { lo } else { hi },
            if m & 2 == 0 { lo } else { hi },
            if m & 4 == 0 { lo } else { hi },
            if m & 8 == 0 { lo } else { hi },
        ];
        let r = f(l);
        cmin = cmin.min(r);
        cmax = cmax.max(r);
    }
    let mut tmin = i32::MAX;
    let mut tmax = i32::MIN;
    for v in assignments(a.free(), lo, hi) {
        let r = f(a.leaves(&v));
        tmin = tmin.min(r);
        tmax = tmax.max(r);
    }
    ((cmin, cmax) == (tmin, tmax), (cmin, cmax), (tmin, tmax))
}

fn main() {
    println!("width {W} signed, representable [{LO}, {HI}], saturating at every step");
    println!("term: l1 - l2 + l3 - l4, left-nested against balanced\n");

    // K1
    let k1 = Aliasing::Distinct.free() == 4 && Aliasing::Aliased.free() == 3;
    println!("### K1, the aliased term has one fewer free variable than leaf positions");
    println!("  {}", if k1 { "PASS, 4 and 3" } else { "FAIL" });

    // K3
    let mut k3 = true;
    for a in [Aliasing::Distinct, Aliasing::Aliased] {
        for v in assignments(a.free(), LO, HI) {
            let l = a.leaves(&v);
            if exact_left(l) != exact_tree(l) {
                k3 = false;
            }
        }
    }
    println!("### K3, without clamping the two associations agree everywhere");
    println!(
        "  {}",
        if k3 {
            "PASS, they are re-associations of one expression"
        } else {
            "FAIL, they are different expressions and Q1 means nothing"
        }
    );

    let windows: [(&str, i32, i32); 4] = [
        ("full range, straddles", LO, HI),
        ("non-negative", 0, HI),
        ("non-positive", LO, 0),
        ("narrow [-1, 1]", -1, 1),
    ];

    println!("\n### Q1: does the VALUE depend on the association?");
    println!(
        "{:<24} {:<10} {:>12} {:>12}  {}",
        "declared window", "aliasing", "divergent", "of", "first witness"
    );
    for (wn, lo, hi) in windows {
        for a in [Aliasing::Distinct, Aliasing::Aliased] {
            let (n, t, w) = q1(a, lo, hi);
            let wd = match w {
                Some((l, x, y)) => format!("{l:?} left={x} tree={y}"),
                None => String::new(),
            };
            println!("{:<24} {:<10} {:>12} {:>12}  {}", wn, a.name(), n, t, wd);
        }
    }

    println!("\n### Q2: is the corner rule exact?");
    println!(
        "{:<24} {:<10} {:<8} {:<8} {:<14} {}",
        "declared window", "aliasing", "assoc", "exact?", "corner", "true"
    );
    let mut k4 = true;
    for (wn, lo, hi) in windows {
        for a in [Aliasing::Distinct, Aliasing::Aliased] {
            for (an, f) in [
                ("left", left as fn([i32; 4]) -> i32),
                ("tree", tree as fn([i32; 4]) -> i32),
            ] {
                let (ok, c, t) = q2(a, f, lo, hi);
                if a == Aliasing::Distinct && !ok {
                    k4 = false;
                }
                println!(
                    "{:<24} {:<10} {:<8} {:<8} {:<14} {}",
                    wn,
                    a.name(),
                    an,
                    if ok { "exact" } else { "WIDE" },
                    format!("[{}, {}]", c.0, c.1),
                    format!("[{}, {}]", t.0, t.1)
                );
            }
        }
    }
    println!("### K4, the corner rule must be exact for the distinct term everywhere");
    println!(
        "  {}",
        if k4 {
            "PASS"
        } else {
            "FAIL, the corner computation is wrong rather than the finding interesting"
        }
    );

    // K2 / P-C
    let mut k2 = true;
    for a in [Aliasing::Distinct, Aliasing::Aliased] {
        let (n, _, _) = q1(a, -1, 1);
        if n != 0 {
            k2 = false;
        }
    }
    println!("### K2, a window narrow enough that no clamp is reachable gives no divergence");
    println!(
        "  {}",
        if k2 {
            "PASS, zero in both aliasings at [-1, 1]"
        } else {
            "FAIL, the arm is measuring the arithmetic rather than the association"
        }
    );
}
