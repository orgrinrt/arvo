// q3: the second read `141` asked for on its own only new claim.
//
// `141` F2 says the accumulator width is answer-visible exactly at
// `signedness = signed, overflow = saturating` and invisible everywhere else,
// and flags it as resting on one instrument, naming the specific risk: whether
// its `acc_width` is a shape any consumer would build, and whether the
// operations it routed through it are routed the way a real kernel would route
// them.
//
// So this is a deliberately different construction. `141` added an accumulator
// dimension to a set of operations and counted classes. This is a FOLD OVER A
// SEQUENCE, which is the shape a column kernel actually has: an accumulator
// carried across n steps, reduced at the accumulator's width on every step, and
// narrowed to the declared width once at the end. The sequence length is a
// dimension here and is not one there, and the final narrowing is a separate
// reduction rather than the same one.
//
// The sum fold isolates the question. An accumulator's width is about RANGE, so
// mixing a rounding step into the fold would let a rounding effect masquerade as
// an accumulator effect. The mac fold is included second, closer to a real
// kernel, so the isolation is a choice rather than a limitation.
//
// PREDICTIONS, recorded before the first run:
//   G1 signed saturating: the accumulator width is answer-visible. Widening the
//      accumulator lets a value that would have clamped survive and be pulled
//      back by a later step of opposite sign, which a narrow accumulator has
//      already thrown away.
//   G2 unsigned saturating: invisible, because the clamp is one-sided and a
//      one-sided clamp of a monotone accumulation is a congruence.
//   G3 wrapping, both signednesses: invisible, because reduction mod 2^k
//      followed by reduction mod 2^W is reduction mod 2^W whenever W <= k.
//   G4 at n = 1 no width is visible under any policy, because a fold of length
//      one has no intermediate to reduce.
//
// CONTROLS:
//   C1 a LOSSY accumulator, one bit narrower than the declared width, must
//      change answers. If the sweep cannot see that, it cannot see an
//      accumulator at all and every invisible verdict is free.
//   C2 the accumulator must actually be exercised: count how often the running
//      value leaves the declared range. A row where that is zero is vacuous and
//      is reported as such rather than counted.
//   C3 G4 is a structural control on the instrument itself. If n = 1 shows a
//      difference, the model is reducing somewhere it should not be.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ov {
    Wrap,
    Sat,
}

fn bounds(w: u32, signed: bool) -> (i128, i128) {
    let m = 1i128 << w;
    if signed {
        (-(m >> 1), (m >> 1) - 1)
    } else {
        (0, m - 1)
    }
}

fn reduce(v: i128, w: u32, signed: bool, ov: Ov) -> i128 {
    match ov {
        Ov::Sat => {
            let (lo, hi) = bounds(w, signed);
            v.clamp(lo, hi)
        }
        Ov::Wrap => {
            let m = 1i128 << w;
            let r = v.rem_euclid(m);
            if signed && r >= (m >> 1) {
                r - m
            } else {
                r
            }
        }
    }
}

fn rnd_floor(p: i128, sh: u32) -> i128 {
    if sh == 0 {
        p
    } else {
        p.div_euclid(1i128 << sh)
    }
}

/// The fold a column kernel performs: reduce at the accumulator's width on
/// every step, narrow to the declared width once at the end.
fn fold_sum(seq: &[i128], w: u32, accw: u32, signed: bool, ov: Ov) -> (i128, bool) {
    let mut acc = 0i128;
    let (dlo, dhi) = bounds(w, signed);
    let mut left_declared = false;
    for &v in seq {
        acc += v;
        if acc < dlo || acc > dhi {
            left_declared = true;
        }
        acc = reduce(acc, accw, signed, ov);
    }
    (reduce(acc, w, signed, ov), left_declared)
}

fn fold_mac(seq: &[i128], mult: &[i128], f: u32, w: u32, accw: u32, signed: bool, ov: Ov) -> (i128, bool) {
    let mut acc = 0i128;
    let (dlo, dhi) = bounds(w, signed);
    let mut left_declared = false;
    for (i, &v) in seq.iter().enumerate() {
        acc += rnd_floor(v * mult[i % mult.len()], f);
        if acc < dlo || acc > dhi {
            left_declared = true;
        }
        acc = reduce(acc, accw, signed, ov);
    }
    (reduce(acc, w, signed, ov), left_declared)
}

/// Every sequence of length n over the domain, as indices.
fn sequences(domain: &[i128], n: usize) -> Vec<Vec<i128>> {
    let mut out = vec![Vec::new()];
    for _ in 0..n {
        let mut next = Vec::new();
        for pre in &out {
            for &d in domain {
                let mut s = pre.clone();
                s.push(d);
                next.push(s);
            }
        }
        out = next;
    }
    out
}

fn main() {
    let mut failures = 0usize;
    let w = 4u32;

    println!("q3: an accumulator carried across a fold, W=4, exhaustive over every sequence");
    println!("accumulator widths swept: declared W, W+1, W+2, 2W. Control: W-1 (lossy).");
    println!();

    for signed in [false, true] {
        let (lo, hi) = bounds(w, signed);
        let domain: Vec<i128> = (lo..=hi).collect();
        for ov in [Ov::Wrap, Ov::Sat] {
            for n in [1usize, 2, 3, 4] {
                let seqs = sequences(&domain, n);
                // baseline: accumulator pinned at the declared width
                let mut visible = 0u64;
                let mut lossy_diff = 0u64;
                let mut exercised = 0u64;
                let mut nonzero = 0u64;
                let mut witness: Option<(Vec<i128>, i128, i128)> = None;

                for s in &seqs {
                    let (base, left) = fold_sum(s, w, w, signed, ov);
                    if left {
                        exercised += 1;
                    }
                    if base != 0 {
                        nonzero += 1;
                    }
                    for accw in [w + 1, w + 2, 2 * w] {
                        let (wide, _) = fold_sum(s, w, accw, signed, ov);
                        if wide != base {
                            visible += 1;
                            if witness.is_none() {
                                witness = Some((s.clone(), base, wide));
                            }
                        }
                    }
                    // C1: the lossy control
                    let (lossy, _) = fold_sum(s, w, w - 1, signed, ov);
                    if lossy != base {
                        lossy_diff += 1;
                    }
                }

                let vac = if exercised == 0 { "  [VACUOUS: the accumulator never left the declared range]" } else { "" };
                print!(
                    "  signed={:<5} {:<4} n={n}: seqs={:<6} accumulator visible at {:>6} comparisons | lossy control differs {:>6} | left declared range {:>6}{}",
                    signed,
                    if ov == Ov::Sat { "Sat" } else { "Wrap" },
                    seqs.len(),
                    visible,
                    lossy_diff,
                    exercised,
                    vac
                );
                match &witness {
                    Some((s, b, wi)) => println!("  witness seq={s:?} pinned={b} widened={wi}"),
                    None => println!(),
                }

                // C1 must fire wherever the accumulator is exercised at all.
                if exercised > 0 && lossy_diff == 0 {
                    println!("    C1 FAILED: a one-bit-narrower accumulator changed nothing");
                    failures += 1;
                }
                // C3: n = 1 must never show a difference.
                if n == 1 && visible != 0 {
                    println!("    C3 FAILED: a fold of length one saw the accumulator width");
                    failures += 1;
                }
                if nonzero == 0 {
                    println!("    non-vacuity FAILED: every result zero");
                    failures += 1;
                }
            }
        }
    }

    println!();
    println!("the same question through a multiply-accumulate fold, F=1.");
    println!("TWO multiplier ORDERS, because the first run showed zero and the mechanism");
    println!("said that was the arrangement rather than the answer: with [1,-1,2] the");
    println!("largest step is last, so a saturation is never followed by a step that could");
    println!("recover from it, and the effect cannot appear however hard the sweep looks.");
    println!("The reversed order puts the large step first. If the cell is real, the order");
    println!("must change the verdict, and if it does not, my sum-fold result is the one");
    println!("that needs explaining.");
    println!();
    let mult_signed = [1i128, -1, 2];
    let mult_unsigned = [1i128, 2, 3];
    for signed in [false, true] {
        let (lo, hi) = bounds(w, signed);
        let domain: Vec<i128> = (lo..=hi).collect();
        let mult_rev_signed = [2i128, -1, 1];
        let mult_rev_unsigned = [3i128, 2, 1];
        for (order, mult) in [
            ("large-last ", if signed { &mult_signed } else { &mult_unsigned }),
            ("large-first", if signed { &mult_rev_signed } else { &mult_rev_unsigned }),
        ] {
        for ov in [Ov::Wrap, Ov::Sat] {
            for n in [1usize, 3] {
                let seqs = sequences(&domain, n);
                let (mut visible, mut lossy_diff, mut exercised) = (0u64, 0u64, 0u64);
                for s in &seqs {
                    let (base, left) = fold_mac(s, mult, 1, w, w, signed, ov);
                    if left {
                        exercised += 1;
                    }
                    for accw in [w + 1, w + 2, 2 * w] {
                        let (wide, _) = fold_mac(s, mult, 1, w, accw, signed, ov);
                        if wide != base {
                            visible += 1;
                        }
                    }
                    let (lossy, _) = fold_mac(s, mult, 1, w, w - 1, signed, ov);
                    if lossy != base {
                        lossy_diff += 1;
                    }
                }
                println!(
                    "  signed={:<5} {:<4} {order} n={n}: seqs={:<6} accumulator visible at {:>6} | lossy control {:>6} | left range {:>6}",
                    signed,
                    if ov == Ov::Sat { "Sat" } else { "Wrap" },
                    seqs.len(),
                    visible,
                    lossy_diff,
                    exercised
                );
                if exercised > 0 && lossy_diff == 0 {
                    println!("    C1 FAILED");
                    failures += 1;
                }
                if n == 1 && visible != 0 {
                    println!("    C3 FAILED");
                    failures += 1;
                }
            }
        }
        }
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
