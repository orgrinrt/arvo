// y4: are the accumulator cell and the fusion cell the same question?
//
// `146` section 5.5 states three separate clauses with three separate predicate
// blocks: fusion is free under certain axes, fusion changes the answer at signed
// saturating, and the accumulator is free except at signed saturating. Each is an
// exhaustive enumeration over its own sweep, and the candidate carries no statement
// that the three are instances of one thing.
//
// My `141` replacements K and L said they are. Neither is carried, and `y3` confirms
// the word "congruence" appears in `146` only in the ledger and in the argument-kind
// list, never as a clause. So the claim is untested in the topic and I am obliged to
// test it before proposing it again rather than re-proposing a shape nobody checked.
//
// THE CLAIM. Both cells ask the same question: does the reduction commute with what
// comes after it? Relocating a reduction is free exactly where the reduction is a
// congruence for the following operation, on the domain the cell reaches.
//
//   wrapping, either signedness : congruence, by absorption mod 2^W
//   saturating, unsigned        : congruence, one-sided clamp of a monotone operation
//   saturating, signed          : NOT a congruence, two-sided clamp
//
// If that predicts both the fusion cell and the accumulator cell at four of four
// combinations, then the three clauses in section 5.5 are one clause with three
// instantiations, and the canon can say the reason rather than three tables.
//
// PREDICTIONS, before running:
//   W1. The congruence test is true at (wrap, unsigned), (wrap, signed) and
//       (sat, unsigned), and false at (sat, signed).
//   W2. The fusion difference, with the ROUNDING held fixed so only the reduction
//       moves, is zero at exactly the three congruence cells and nonzero at the
//       fourth.
//   W3. The accumulator visibility in a fold is zero at exactly the three congruence
//       cells and nonzero at the fourth.
//   W4. So one property predicts two independently measured cells at 4 of 4 each,
//       and the two cells coincide.
//
// CONTROLS:
//   C1 (mutation). A reduction that is deliberately not a congruence anywhere, here a
//      one's-complement fold-back, must fail the congruence test in every cell AND
//      show a nonzero fusion difference and a visible accumulator in every cell. If it
//      does not, the congruence test is not driving the prediction.
//   C2 (reach). The fold must actually saturate somewhere in each cell, or a zero is
//      vacuous. Counted and printed.
//   C3 (independence). The congruence test sweeps `x` outside the declared range and
//      the fusion and accumulator tests never do, so the three are not the same
//      computation wearing three names.
//
// Run: rustc -O -o /tmp/y4 y4_one_question_not_three.rs && /tmp/y4

#[derive(Clone, Copy, PartialEq, Eq)]
enum Red {
    Wrap,
    Sat,
    /// C1: fold an out-of-range value back by complementing. Monotone nowhere.
    BadFold,
}

fn bounds(signed: bool, w: u32) -> (i128, i128) {
    if signed {
        (-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
    } else {
        (0, (1i128 << w) - 1)
    }
}

fn reduce(v: i128, signed: bool, r: Red, w: u32) -> i128 {
    let (lo, hi) = bounds(signed, w);
    match r {
        Red::Sat => v.clamp(lo, hi),
        Red::Wrap => {
            let m = 1i128 << w;
            let k = v.rem_euclid(m);
            if signed && k >= (1i128 << (w - 1)) {
                k - m
            } else {
                k
            }
        }
        Red::BadFold => {
            if v >= lo && v <= hi {
                v
            } else {
                let m = 1i128 << w;
                let k = v.rem_euclid(m);
                let k = hi - (k % (hi - lo + 1));
                k.clamp(lo, hi)
            }
        }
    }
}

/// Is R a congruence for "add y", over the reachable domain?
/// R(R(x) + y) == R(x + y) for every x the cell can produce and every y in range.
fn is_congruence(signed: bool, r: Red, w: u32) -> (bool, u64) {
    let (lo, hi) = bounds(signed, w);
    // x ranges over everything a product at this width can produce.
    let xlo = if signed { -(1i128 << (2 * w)) } else { 0 };
    let xhi = 1i128 << (2 * w);
    let mut bad = 0u64;
    for x in xlo..=xhi {
        for y in lo..=hi {
            if reduce(reduce(x, signed, r, w) + y, signed, r, w) != reduce(x + y, signed, r, w) {
                bad += 1;
            }
        }
    }
    (bad == 0, bad)
}

/// Reduction relocation only: the rounding is identical in both arms.
fn fusion_diff(signed: bool, r: Red, w: u32, f: u32) -> (u64, u64, u64) {
    let (lo, hi) = bounds(signed, w);
    let mut n = 0u64;
    let mut diff = 0u64;
    let mut reached = 0u64;
    for a in lo..=hi {
        for b in lo..=hi {
            let p = if f == 0 { a * b } else { (a * b) >> f };
            let out = p < lo || p > hi;
            for c in lo..=hi {
                n += 1;
                if out {
                    reached += 1;
                }
                let stepwise = reduce(reduce(p, signed, r, w) + c, signed, r, w);
                let fused = reduce(p + c, signed, r, w);
                if stepwise != fused {
                    diff += 1;
                }
            }
        }
    }
    (diff, n, reached)
}

/// A fold of length 3, accumulated at width `acc` and narrowed to `w` once.
fn fold(seq: &[i128], signed: bool, r: Red, w: u32, acc: u32) -> i128 {
    let mut s = 0i128;
    for &v in seq {
        s = reduce(s + v, signed, r, acc);
    }
    reduce(s, signed, r, w)
}

fn accumulator_visible(signed: bool, r: Red, w: u32) -> (u64, u64) {
    let (lo, hi) = bounds(signed, w);
    let mut vis = 0u64;
    let mut saturated = 0u64;
    for a in lo..=hi {
        for b in lo..=hi {
            for c in lo..=hi {
                let seq = [a, b, c];
                let narrow = fold(&seq, signed, r, w, w);
                let wide = fold(&seq, signed, r, w, w + 2);
                if narrow != wide {
                    vis += 1;
                }
                // reach: did the running sum ever leave the declared range?
                let mut s = 0i128;
                let mut left = false;
                for &v in &seq {
                    s += v;
                    if s < lo || s > hi {
                        left = true;
                    }
                }
                if left {
                    saturated += 1;
                }
            }
        }
    }
    (vis, saturated)
}

fn main() {
    let w = 4u32;
    println!("y4: is one property behind the fusion cell and the accumulator cell?");
    println!("W = {w}, exhaustive per cell\n");

    for r in [Red::Wrap, Red::Sat, Red::BadFold] {
        let rn = match r {
            Red::Wrap => "wrapping",
            Red::Sat => "saturating",
            Red::BadFold => "BADFOLD (control)",
        };
        println!("=== reduction = {rn} ===");
        println!(
            "{:<10} {:>12} {:>10} {:>14} {:>12} {:>14} {:>10}",
            "signedness",
            "congruence",
            "failures",
            "fusion diff",
            "reach",
            "acc visible",
            "saturated"
        );
        for signed in [false, true] {
            let (cong, bad) = is_congruence(signed, r, w);
            let (fd, _fn_, reach) = fusion_diff(signed, r, w, 1);
            let (vis, sat) = accumulator_visible(signed, r, w);
            println!(
                "{:<10} {:>12} {:>10} {:>14} {:>12} {:>14} {:>10}",
                if signed { "signed" } else { "unsigned" },
                cong,
                bad,
                fd,
                reach,
                vis,
                sat
            );
        }
        println!();
    }

    println!("=== verdicts ===");
    let mut w2_ok = 0;
    let mut w3_ok = 0;
    for r in [Red::Wrap, Red::Sat] {
        for signed in [false, true] {
            let (cong, _) = is_congruence(signed, r, w);
            let (fd, _, _) = fusion_diff(signed, r, w, 1);
            let (vis, _) = accumulator_visible(signed, r, w);
            if cong == (fd == 0) {
                w2_ok += 1;
            }
            if cong == (vis == 0) {
                w3_ok += 1;
            }
        }
    }
    println!("W2: congruence predicts the fusion cell at {w2_ok} of 4");
    println!("W3: congruence predicts the accumulator cell at {w3_ok} of 4");
    println!("W4: the two cells coincide iff both are 4 of 4");
    println!();
    println!("C1: the BADFOLD control must show congruence=false, fusion diff > 0 and");
    println!("    accumulator visible > 0 in every row above, including the cells where");
    println!("    wrapping and saturating are all zero. If it does not, the congruence");
    println!("    test is not what is driving the prediction.");
}
