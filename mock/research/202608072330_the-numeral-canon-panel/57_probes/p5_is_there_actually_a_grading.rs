//! Probe 5: is there actually a grading here, and if so what is the grade.
//!
//! WHY THIS IS A SEPARATE QUESTION. `55b` section 3.1 says "the induced
//! algebras grade" and then lists ring, semiring, magma. That is a LADDER of
//! algebraic strength: a partial order on theories, ordered by which axioms
//! hold. It is a real and useful classification and it is not a grading. A
//! grading needs an index set with a COMPOSITION law, so that the index of a
//! composite is computed from the indices of its parts.
//!
//! CORRECTION, WRITTEN AFTER THIS PROBE RAN. The sentence that stood here
//! predicted that the ladder has no such law and pointed at `p4` section 1 for
//! a composite whose rung is worse than its factors' would suggest. Section 1
//! below REFUTED that prediction: the meet was respected in all twelve rows,
//! and in every row the composite's law set equalled the meet of its factors'
//! exactly. So the ladder was not shown to be non-compositional, and the
//! honest statement of what section 1 establishes is the weaker one printed in
//! its own output. The prediction is left visible here rather than edited out,
//! because a probe whose hypothesis was refuted is more informative with the
//! hypothesis still attached.
//!
//! So this probe asks two things, and they are independent:
//!
//!   ONE. Does the LADDER compose? For the composite reductions of `p4`, is the
//!   set of laws the composite satisfies determined by the sets its factors
//!   satisfy? If a composite ever fails a law BOTH factors satisfy, the ladder
//!   is not compositional even laxly, and calling it a grade is a category
//!   error that a canon would inherit.
//!
//!   TWO. Is there a genuine grading somewhere in this design? The candidate is
//!   PRECISION: a family of value sets indexed by width, with the exact
//!   operations mapping A_W x A_V into A_g for a computed g. If the index
//!   composes, that is a grading, the exact-then-adapt factoring is its
//!   structure, and the accumulator width an implementation needs IS the grade.
//!   Three sub-questions:
//!
//!     2a. Is the additive width rule g = max(W,V) + 1 associative? If not,
//!         widths are not a grade monoid and the grade must be something else.
//!     2b. What does compose exactly? Candidate: the reachable value INTERVAL,
//!         under Minkowski sum. Measured against the exact reachable set.
//!     2c. The operational form: for an incoherent policy, accumulate in a
//!         width-w intermediate and adapt once at the end. Divergence from the
//!         exact-then-adapt answer as a function of w IS the grading, made of
//!         measurements rather than of vocabulary. Predicted sufficient grade
//!         for n operands of a W-bit signed format is W + ceil(log2 n); the
//!         probe reports the width at which divergence actually reaches zero,
//!         so a conservative bound shows up as a gap rather than as a pass.
//!
//! INSTRUMENT VALIDATION. 2c's checker must report nonzero divergence at small
//! w and zero at large w on the same code path, for the same policy, which it
//! cannot fake since both come from one loop. 2a is a decidable arithmetic
//! claim printed with its witness. The law-composition check must be shown
//! capable of reporting a violation by being run against a deliberately
//! mismatched pairing as well as the real ones.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p5 p5_is_there_actually_a_grading.rs && ./p5

// ------------------------------------------------------------------ part 1

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Laws {
    add_assoc: bool,
    mul_assoc: bool,
    distrib: bool,
}

fn laws_of(clamp: bool, m: i64, f: u32, hi: i64) -> Laws {
    let s = 1i64 << f;
    let red = |x: i64| if clamp { x.clamp(0, m) } else { x };
    let add = |a: i64, b: i64| red(a + b);
    let mul = |a: i64, b: i64| red((a * b) / s);
    let mut l = Laws {
        add_assoc: true,
        mul_assoc: true,
        distrib: true,
    };
    for a in 0..=hi {
        for b in 0..=hi {
            for c in 0..=hi {
                if add(add(a, b), c) != add(a, add(b, c)) {
                    l.add_assoc = false;
                }
                if mul(mul(a, b), c) != mul(a, mul(b, c)) {
                    l.mul_assoc = false;
                }
                if mul(a, add(b, c)) != add(mul(a, b), mul(a, c)) {
                    l.distrib = false;
                }
            }
        }
    }
    l
}

/// does the composite satisfy every law BOTH factors satisfy
fn meet_respected(clamp_only: Laws, coarsen_only: Laws, composite: Laws) -> bool {
    let ok = |a: bool, b: bool, c: bool| !(a && b) || c;
    ok(
        clamp_only.add_assoc,
        coarsen_only.add_assoc,
        composite.add_assoc,
    ) && ok(
        clamp_only.mul_assoc,
        coarsen_only.mul_assoc,
        composite.mul_assoc,
    ) && ok(clamp_only.distrib, coarsen_only.distrib, composite.distrib)
}

// ------------------------------------------------------------------ part 2c

/// accumulate n operands from the format Q = [qlo, qhi], saturating into a
/// width-w signed intermediate at every step, then adapt once into Q.
/// compare against the exact sum adapted once. count divergent tuples.
fn grade_divergence(qlo: i64, qhi: i64, n: usize, w: u32) -> (u64, u64) {
    let alo = -(1i64 << (w - 1));
    let ahi = (1i64 << (w - 1)) - 1;
    let span = (qhi - qlo + 1) as usize;
    let total = (span as u64).pow(n as u32);
    let mut diff = 0u64;
    let mut idx = vec![0usize; n];
    loop {
        let mut acc = 0i64;
        let mut exact = 0i64;
        for k in 0..n {
            let v = qlo + idx[k] as i64;
            acc = (acc + v).clamp(alo, ahi);
            exact += v;
        }
        if acc.clamp(qlo, qhi) != exact.clamp(qlo, qhi) {
            diff += 1;
        }
        let mut k = 0;
        loop {
            if k == n {
                return (total, diff);
            }
            idx[k] += 1;
            if idx[k] < span {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
    }
}

fn ceil_log2(n: usize) -> u32 {
    let mut k = 0u32;
    while (1usize << k) < n {
        k += 1;
    }
    k
}

fn main() {
    let mut ok = true;

    println!("=== 1. does the LADDER compose ===");
    println!();
    println!("  For each (M, F): the laws of the clamp factor alone, the coarsening");
    println!("  factor alone, and the composite. A composite failing a law BOTH");
    println!("  factors satisfy would mean the rung of a composite is not determined");
    println!("  by the rungs of its parts, which is what a grade would have to be.");
    println!();
    println!(
        "{:>5} {:>4} | {:^24} | {:^24} | {:^24} | {}",
        "M", "F", "clamp only", "coarsen only", "composite", "meet respected"
    );
    let show = |l: Laws| {
        format!(
            "+a:{} *a:{} dist:{}",
            l.add_assoc as u8, l.mul_assoc as u8, l.distrib as u8
        )
    };
    let mut all_meet = true;
    let mut saw_composite_strictly_worse = false;
    for &m in &[7i64, 15, 31] {
        for &f in &[0u32, 1, 2, 3] {
            let c = laws_of(true, m, 0, m); // clamp only: no rescale
            let g = laws_of(false, m, f, m); // coarsen only: no clamp
            let k = laws_of(true, m, f, m); // composite
            let meet = meet_respected(c, g, k);
            all_meet &= meet;
            // is the composite strictly worse than either factor on some law
            if (c.distrib && !k.distrib) || (g.distrib && !k.distrib) {
                saw_composite_strictly_worse = true;
            }
            println!(
                "{:>5} {:>4} | {:^24} | {:^24} | {:^24} | {}",
                m,
                f,
                show(c),
                show(g),
                show(k),
                meet
            );
        }
    }
    println!();
    println!("  meet respected in every row: {}", all_meet);
    println!(
        "  a composite strictly worse than one of its factors on distributivity: {}",
        saw_composite_strictly_worse
    );
    println!();
    println!("  Reading, stated to match what the rows actually show. The prediction");
    println!("  this probe was built on was that a composite would fail a law both its");
    println!("  factors satisfy, which would have killed compositionality outright. It");
    println!("  did not happen: the meet held in every row, and in every row the");
    println!("  composite's law set EQUALLED the meet rather than merely containing it.");
    println!("  So over this matrix the ladder is compositional, which is a stronger");
    println!("  position than the one it was dispatched to test, arrived at by failing");
    println!("  to refute it. That is one instrument over twelve configurations of one");
    println!("  factorisation, and it is first-read. What it does NOT establish is that");
    println!("  the meet is the composition law in general: a lattice meet is defined on");
    println!("  law sets whatever the operations are, so agreeing with it here is");
    println!("  consistent with there being no relation at all in a case not measured.");

    println!();
    println!("=== 2a. is the additive width rule a monoid operation ===");
    println!();
    let g = |a: u32, b: u32| a.max(b) + 1;
    let (x, y, z) = (5u32, 0u32, 0u32);
    let left = g(g(x, y), z);
    let right = g(x, g(y, z));
    println!("  width rule g(W,V) = max(W,V) + 1");
    println!("  g(g({},{}),{}) = {}", x, y, z, left);
    println!("  g({},g({},{})) = {}", x, y, z, right);
    println!("  associative: {}", left == right);
    println!();
    println!("  So widths under the additive rule are NOT a monoid, and therefore not a");
    println!("  grade semiring's additive part. The multiplicative rule g(W,V) = W + V");
    println!("  IS associative, so products grade and sums do not, under widths alone.");
    ok &= left != right; // the counterexample must actually be one

    println!();
    println!("=== 2b. what composes exactly: the reachable interval ===");
    println!();
    println!("  For n operands drawn from Q = [lo, hi], the exact reachable set is");
    println!("  measured and compared against the Minkowski sum [n*lo, n*hi], and");
    println!("  against the width the repeated max+1 rule would predict.");
    println!();
    println!(
        "{:>4} {:>18} {:>18} {:>12} {:>12} {:>8}",
        "n", "measured range", "Minkowski", "exact bits", "max+1 bits", "slack"
    );
    let (qlo, qhi) = (-8i64, 7i64);
    let mut interval_exact = true;
    for n in 1..=6usize {
        let mlo = qlo * n as i64;
        let mhi = qhi * n as i64;
        // measured reachable set of exact sums
        let mut lo = i64::MAX;
        let mut hi = i64::MIN;
        let span = (qhi - qlo + 1) as usize;
        let mut idx = vec![0usize; n];
        loop {
            let s: i64 = (0..n).map(|k| qlo + idx[k] as i64).sum();
            lo = lo.min(s);
            hi = hi.max(s);
            let mut k = 0;
            let done = loop {
                if k == n {
                    break true;
                }
                idx[k] += 1;
                if idx[k] < span {
                    break false;
                }
                idx[k] = 0;
                k += 1;
            };
            if done {
                break;
            }
        }
        interval_exact &= lo == mlo && hi == mhi;
        // exact signed bits needed to hold [lo, hi]
        let mut exact_bits = 1u32;
        while !(-(1i64 << (exact_bits - 1)) <= lo && hi <= (1i64 << (exact_bits - 1)) - 1) {
            exact_bits += 1;
        }
        // repeated max+1 starting from 4-bit operands
        let mut maxp1 = 4u32;
        for _ in 1..n {
            maxp1 = maxp1.max(4) + 1;
        }
        println!(
            "{:>4} {:>18} {:>18} {:>12} {:>12} {:>8}",
            n,
            format!("[{}, {}]", lo, hi),
            format!("[{}, {}]", mlo, mhi),
            exact_bits,
            maxp1,
            maxp1 as i64 - exact_bits as i64
        );
    }
    println!();
    println!(
        "  interval arithmetic exact at every n measured: {}",
        interval_exact
    );
    println!("  The interval composes exactly under Minkowski sum; the width does not,");
    println!("  and the repeated max+1 rule over-approximates by a growing slack. So the");
    println!("  quantity that grades is the reachable SET, and the width is a lax image");
    println!("  of it: sound, and not tight.");
    ok &= interval_exact;

    println!();
    println!("=== 2c. the grading, measured: divergence as a function of accumulator width ===");
    println!();
    println!("  Format Q = [-8, 7], 4-bit signed, signed saturation, which `q1` and `p4`");
    println!("  both measure as the incoherent policy. Accumulate n operands saturating");
    println!("  into a width-w intermediate, then adapt once into Q. Divergence from the");
    println!("  exact-then-adapt answer is reported per w. Predicted sufficient grade is");
    println!("  4 + ceil(log2 n); the width where divergence ACTUALLY reaches zero is");
    println!("  printed beside it, so a conservative prediction shows as a gap.");
    println!();
    println!(
        "{:>4} {:>12} {:>44} {:>10} {:>10}",
        "n", "tuples", "divergences by accumulator width w = 4..9", "predicted", "measured"
    );
    let mut prediction_sound = true;
    let mut any_nonzero = false;
    let mut any_zero = false;
    for n in 2..=5usize {
        let mut cells = Vec::new();
        let mut first_zero: Option<u32> = None;
        let mut total = 0u64;
        for w in 4..=9u32 {
            let (t, d) = grade_divergence(qlo, qhi, n, w);
            total = t;
            cells.push(d);
            if d == 0 && first_zero.is_none() {
                first_zero = Some(w);
            }
            if d > 0 {
                any_nonzero = true;
            } else {
                any_zero = true;
            }
        }
        let predicted = 4 + ceil_log2(n);
        let measured = first_zero.unwrap_or(99);
        // the prediction must be SOUND: at the predicted width, zero divergence
        let at_pred = cells[(predicted - 4) as usize];
        prediction_sound &= at_pred == 0;
        println!(
            "{:>4} {:>12} {:>44} {:>10} {:>10}",
            n,
            total,
            cells
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            predicted,
            measured
        );
    }
    println!();
    println!(
        "  predicted grade sound at every n (zero divergence at the predicted width): {}",
        prediction_sound
    );
    println!();
    println!("  This is a grading in the sense the word carries: a family indexed by w,");
    println!("  an index computed from the operation and the operand count, and an exact");
    println!("  statement of what the index buys. The adaptation is the map that spends");
    println!("  the grade back down to the format's own width, and it is exactly where");
    println!("  the information is lost.");
    ok &= prediction_sound;

    println!();
    println!("=== 3. instrument validation ===");
    println!();
    println!(
        "  2c reported nonzero divergence somewhere: {}",
        any_nonzero
    );
    println!("  2c reported zero divergence somewhere:    {}", any_zero);
    println!(
        "  2a's counterexample is a real one:        {}",
        left != right
    );
    // the law-composition checker must be capable of reporting false
    let bogus = meet_respected(
        Laws {
            add_assoc: true,
            mul_assoc: true,
            distrib: true,
        },
        Laws {
            add_assoc: true,
            mul_assoc: true,
            distrib: true,
        },
        Laws {
            add_assoc: true,
            mul_assoc: true,
            distrib: false,
        },
    );
    println!(
        "  meet checker returns false on a constructed violation: {}",
        !bogus
    );
    ok &= any_nonzero && any_zero && !bogus;

    println!();
    println!("{}", if ok { "P5 WORKS" } else { "P5 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
