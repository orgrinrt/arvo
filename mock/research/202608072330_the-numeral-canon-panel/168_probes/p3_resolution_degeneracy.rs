//! p3. How much a chain actually leaves for the design to decide, counted.
//!
//! A chain of `D` steps has `2^(D-1)` places an interior resolution may be
//! written or omitted (the resolution at the chain boundary is not optional).
//! Each placement is a candidate realisation. The question this probe answers
//! is how many DISTINCT functions those placements compute, exhaustively over
//! the declared domain. Call that the chain's resolution degeneracy.
//!
//!   degeneracy = 1  ->  every placement computes the same function, so the
//!                       design has nothing to decide and may pick the cheapest.
//!   degeneracy > 1  ->  the placements disagree, so the design must NAME which
//!                       one is the answer before any of them can be called an
//!                       optimisation.
//!
//! Then, where the degeneracy is above one, it reports which placement is
//! closest to the exact composite computed without any resolution at all, both
//! in aggregate absolute error and in worst case.
//!
//! THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
//!   - wrapping over ring-affine steps must report degeneracy exactly 1. If it
//!     did not, the homomorphism claim p1 rests on would be false.
//!   - rounding must report degeneracy above 1 and rising with depth. If it did
//!     not, this probe would be counting nothing.
//!   - there must exist at least one input on which the FULLY EAGER placement
//!     is closer to exact than the fully deferred one, because error can cancel.
//!     If no such input existed, "defer everything" would be pointwise optimal
//!     and the aggregate framing would be unnecessary.
//!
//! Run: rustc -O p3_resolution_degeneracy.rs -o /tmp/p3 && /tmp/p3

const W: u32 = 8;
const DOMAIN: u128 = 1 << W;
const LIMIT: u128 = (1 << W) - 1;

#[derive(Clone, Copy, Debug)]
enum Step { AddK(u128), WSubK(u128), MulK(u128), SatSubK(u128), ShrK(u32), XorK(u128) }

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pi { Wrap, Clamp, RoundTo(u32), TruncTo(u32) }

fn apply(s: Step, v: u128) -> u128 {
    match s {
        Step::AddK(k) => v + k,
        Step::WSubK(k) => v.wrapping_sub(k) & ((1u128 << 120) - 1),
        Step::MulK(k) => v * k,
        Step::SatSubK(k) => v.saturating_sub(k),
        Step::ShrK(g) => v >> g,
        Step::XorK(k) => v ^ k,
    }
}

fn resolve(p: Pi, v: u128) -> u128 {
    match p {
        Pi::Wrap => v & LIMIT,
        Pi::Clamp => if v > LIMIT { LIMIT } else { v },
        Pi::RoundTo(g) => {
            if g == 0 { return v; }
            let step = 1u128 << g;
            ((v + (step >> 1)) / step) * step
        }
        Pi::TruncTo(g) => {
            if g == 0 { return v; }
            let step = 1u128 << g;
            (v / step) * step
        }
    }
}

/// Run the chain with interior resolutions at the positions named by the bit
/// set `mask`; the boundary resolution always fires.
fn run(x: u128, steps: &[Step], p: Pi, mask: u32) -> u128 {
    let mut v = x;
    for (i, &s) in steps.iter().enumerate() {
        v = apply(s, v);
        if i + 1 < steps.len() && (mask >> i) & 1 == 1 {
            v = resolve(p, v);
        }
    }
    resolve(p, v)
}

/// The exact composite: no resolution anywhere, not even at the boundary.
fn exact(x: u128, steps: &[Step]) -> u128 {
    let mut v = x;
    for &s in steps { v = apply(s, v); }
    v
}

fn degeneracy(steps: &[Step], p: Pi) -> (usize, Vec<(u32, Vec<u128>)>) {
    let interior = steps.len().saturating_sub(1);
    let mut seen: Vec<(u32, Vec<u128>)> = Vec::new();
    for mask in 0..(1u32 << interior) {
        let f: Vec<u128> = (0..DOMAIN).map(|x| run(x, steps, p, mask)).collect();
        if !seen.iter().any(|(_, g)| *g == f) { seen.push((mask, f)); }
    }
    (seen.len(), seen)
}

fn errors(steps: &[Step], p: Pi, mask: u32) -> (u128, u128) {
    let mut total = 0u128;
    let mut worst = 0u128;
    for x in 0..DOMAIN {
        let got = run(x, steps, p, mask);
        let want = exact(x, steps);
        let e = if got > want { got - want } else { want - got };
        total += e;
        if e > worst { worst = e; }
    }
    (total, worst)
}

fn full_mask(d: usize) -> u32 { (1u32 << d.saturating_sub(1)) - 1 }

fn line(label: &str, steps: &[Step], p: Pi) -> usize {
    let (n, _) = degeneracy(steps, p);
    println!("  {label:34} depth {}  degeneracy {n}", steps.len());
    n
}

fn main() {
    let k = 97u128;
    println!("W = {W}, domain 0..{DOMAIN} exhaustive, limit {LIMIT}");
    println!();

    let affine3 = [Step::AddK(k), Step::MulK(3), Step::WSubK(k)];
    let affine5 = [Step::AddK(k), Step::MulK(3), Step::WSubK(k), Step::MulK(5), Step::AddK(k)];
    let clampy  = [Step::AddK(k), Step::AddK(k), Step::SatSubK(LIMIT / 2)];
    let nonneg  = [Step::AddK(k), Step::AddK(k), Step::AddK(k)];

    println!("DEGENERACY 1 IS THE CASE WITH NOTHING TO DECIDE");
    let n = line("wrap, affine, D=3", &affine3, Pi::Wrap);
    assert_eq!(n, 1, "wrapping over affine steps must be placement-invariant");
    let n = line("wrap, affine, D=5", &affine5, Pi::Wrap);
    assert_eq!(n, 1, "wrapping over affine steps must be placement-invariant at any depth");
    let n = line("clamp, non-negative additions, D=3", &nonneg, Pi::Clamp);
    assert_eq!(n, 1, "the retraction lemma must make clamping placement-invariant here");
    println!();

    println!("DEGENERACY ABOVE 1 IS THE CASE THE DESIGN MUST DECIDE");
    let n = line("clamp, mixed, D=3", &clampy, Pi::Clamp);
    assert!(n > 1, "CONTROL FAILED: mixed clamping was placement-invariant");
    let r3 = line("round to 2^3, affine, D=3", &affine3, Pi::RoundTo(3));
    assert!(r3 > 1, "CONTROL FAILED: rounding was placement-invariant, so nothing is counted");
    let r5 = line("round to 2^3, affine, D=5", &affine5, Pi::RoundTo(3));
    assert!(
        r5 > r3,
        "CONTROL FAILED: the degeneracy did not rise with depth, so depth is not \
         a dimension of the decision after all"
    );
    println!("  -> rounding's decision space grows with depth: {r3} at D=3, {r5} at D=5.");
    println!();

    println!("WHICH PLACEMENT IS CLOSEST TO THE EXACT COMPOSITE (rounding, D=5)");
    let fm = full_mask(affine5.len());
    let (te, we) = errors(&affine5, Pi::RoundTo(3), fm);
    let (td, wd) = errors(&affine5, Pi::RoundTo(3), 0);
    println!("  fully eager  (mask {fm:#06b}): total |err| {te}, worst |err| {we}");
    println!("  fully defer  (mask {:#06b}): total |err| {td}, worst |err| {wd}", 0);
    assert!(td < te, "deferring did not reduce aggregate error, which the whole argument assumes");
    assert!(wd <= we, "deferring did not reduce worst-case error");

    // And is the fully deferred placement the best of ALL placements?
    let mut best_mask = 0u32;
    let mut best_total = u128::MAX;
    for mask in 0..(1u32 << (affine5.len() - 1)) {
        let (t, _) = errors(&affine5, Pi::RoundTo(3), mask);
        if t < best_total { best_total = t; best_mask = mask; }
    }
    println!("  best of all {} placements: mask {best_mask:#06b}, total |err| {best_total}",
             1u32 << (affine5.len() - 1));
    assert_eq!(
        best_mask, 0,
        "the fully deferred placement is NOT the aggregate optimum, which would \
         mean 'defer everything' is the wrong reading of an accuracy-first intent"
    );
    println!();

    println!("IS DEFERRAL POINTWISE OPTIMAL, OR ONLY IN AGGREGATE?");
    println!("  The reason to expect it is: every placement ends with the SAME boundary");
    println!("  resolution, so every output is a representable point; and where that");
    println!("  resolution is a NEAREST-point projection, the deferred form outputs the");
    println!("  nearest representable point to the exact value by definition. Nothing");
    println!("  else can be strictly closer. Below, that is checked rather than argued,");
    println!("  and then broken on a resolution that is not nearest-point.");
    println!();

    let eager_wins = |steps: &[Step], p: Pi| -> (usize, Option<(u128, u128, u128)>) {
        let fm = full_mask(steps.len());
        let mut wins = 0usize;
        let mut ex = None;
        for x in 0..DOMAIN {
            let want = exact(x, steps);
            let g1 = run(x, steps, p, fm);
            let g2 = run(x, steps, p, 0);
            let de = if g1 > want { g1 - want } else { want - g1 };
            let dd = if g2 > want { g2 - want } else { want - g2 };
            if de < dd { wins += 1; if ex.is_none() { ex = Some((x, de, dd)); } }
        }
        (wins, ex)
    };

    // A search rather than one hand-picked chain, over an alphabet that includes
    // a contracting step and a non-monotone one, so a counterexample has room
    // to exist.
    let alphabet = [
        Step::AddK(k), Step::MulK(3), Step::ShrK(2), Step::XorK(0b1011_0110),
        Step::SatSubK(LIMIT / 2), Step::MulK(5), Step::ShrK(1), Step::AddK(13),
    ];
    let mut rng: u64 = 0xA5A5_1234_DEAD_BEEF;
    let mut next = || { rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17; rng };
    let mut chains: Vec<Vec<Step>> = Vec::new();
    for _ in 0..3000 {
        let d = 2 + (next() % 4) as usize;
        chains.push((0..d).map(|_| alphabet[(next() % 8) as usize]).collect());
    }

    for (name, p) in [("nearest (round to 2^3)", Pi::RoundTo(3)), ("nearest (clamp)", Pi::Clamp)] {
        let mut total_wins = 0usize;
        let mut chains_with_a_win = 0usize;
        for c in chains.iter() {
            let (w, _) = eager_wins(c, p);
            if w > 0 { chains_with_a_win += 1; total_wins += w; }
        }
        println!("  {name:24} {} chains searched, {chains_with_a_win} with any eager win, {total_wins} winning inputs",
                 chains.len());
        assert_eq!(
            total_wins, 0,
            "a nearest-point boundary resolution let an eager placement land strictly \
             closer to exact, which contradicts the projection argument and means the \
             argument, not the measurement, is wrong"
        );
    }

    // The control. Truncation is a projection onto the representable set but is
    // NOT the nearest one, so the argument above does not cover it and eager
    // must be able to win somewhere. If it cannot, this probe has not actually
    // tested the nearest-point hypothesis, only restated it.
    let mut total_wins = 0usize;
    let mut chains_with_a_win = 0usize;
    let mut example: Option<(Vec<Step>, u128, u128, u128)> = None;
    for c in chains.iter() {
        let (w, ex) = eager_wins(c, Pi::TruncTo(3));
        if w > 0 {
            chains_with_a_win += 1;
            total_wins += w;
            if example.is_none() { if let Some((x, de, dd)) = ex { example = Some((c.clone(), x, de, dd)); } }
        }
    }
    println!("  {:24} {} chains searched, {chains_with_a_win} with any eager win, {total_wins} winning inputs",
             "NOT nearest (truncate)", chains.len());
    if let Some((c, x, de, dd)) = &example {
        println!("  e.g. {c:?} at x={x}: eager |err| {de} < deferred |err| {dd}");
    }
    assert!(
        total_wins > 0,
        "CONTROL FAILED: even a non-nearest projection never let eager win, so this \
         search cannot distinguish the nearest-point hypothesis from a search that \
         finds nothing"
    );
    println!();
    println!("RESULT: degeneracy separates the chains that pose a decision from those that");
    println!("do not. Where it is above one, deferring every interior resolution is the");
    println!("aggregate and worst-case optimum, and it is also the POINTWISE optimum for");
    println!("every chain searched whose boundary resolution is a nearest-point projection.");
    println!("That last part fails as soon as the boundary resolution is not nearest-point,");
    println!("which is what the truncation row shows, so it is a property of the resolution");
    println!("rather than of chains in general.");
}
