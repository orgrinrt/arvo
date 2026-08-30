//! Probe 2: does the multiplicative fold have an additive-shaped grading, or a
//! structurally different one. Section 2 reports a bug in this probe's own
//! first hypothesis that turned into a second, independent finding; kept, per
//! the panel's discipline of leaving a refuted first run on the record.
//!
//! WHY. `57` section 3.7 measures, for ADDITION, that eager saturation needs no
//! extra accumulator width at all when the policy is coherent, and that an
//! incoherent (signed) policy needs exactly one bit less than the exact-sum
//! width, uniformly, at every fold length 2 through 8 (`57_probes/p6`). That is
//! a small, closed-form, fold-length-independent-in-shape answer: a few extra
//! INTEGER bits of headroom, chosen once, suffice forever.
//!
//! Multiplication has no integer-headroom knob to turn in the same way. Every
//! `UFixed<I, F, S>` multiply narrows a 2F-fractional-bit exact product back to
//! F bits, which is a FRACTIONAL narrowing, not a range clamp. Fixed-point DSP
//! practice (guard bits on a multiply-accumulate unit, the classic 16x16=32,
//! truncate-to-16 pattern) treats this as a per-step ROUNDING NOISE SOURCE, not
//! as a range problem, precisely because widening the integer part buys nothing
//! against it. Section 1 isolates that question with NO intermediate range
//! clamp at all (matching `exact_once`'s own shape: clamp once, at the end).
//!
//! WHAT SECTION 1 MEASURES. Format elements are raw integers r in [0, M]
//! denoting r / 2^F. For fold length n and guard width w, `LEFT(w)` and
//! `RIGHT(w)` fold left-to-right and right-to-left, keeping w guard bits of
//! fraction between steps and narrowing off F bits (never clamping the range)
//! after every pairwise multiply, then narrowing off the guard bits and
//! clamping to [0, M] exactly once, at the very end. w = (n-1)*F is full
//! precision; the question is the smallest w below that at which LEFT and
//! RIGHT still agree everywhere, and whether that grows like addition's small
//! constant or tracks the full-precision ceiling.
//!
//! SECTION 2 is the bug. The first version of this probe clamped the
//! accumulator to the format's range at every intermediate step, including at
//! full guard width, on the reasoning that a real accumulator has SOME finite
//! range. It reported LEFT != RIGHT at full precision (`min_w` exceeding
//! `full_w`), which is impossible for exact integer multiplication and is
//! therefore a bug rather than a finding about the fractional axis. The actual
//! cause is a SECOND, independent lossy step: an intermediate RANGE clamp,
//! reachable or not exactly per `57`'s job-one absorption mechanism, applied at
//! every step rather than once. Section 2 keeps that variant and measures it
//! deliberately, because it is real: it is what an accumulator with FINITE
//! integer headroom actually does, and it shows the two mechanisms (fractional
//! narrowing, range clamping) do not substitute for one another; an
//! implementation can pay full fractional precision and still not associate,
//! for a completely separate reason.
//!
//! INSTRUMENT VALIDATION. Section 1: w=0 must show divergence for n >= 3
//! (matches `57_probes/p4`'s coarsen-only finding); w = (n-1)*F must be clean
//! everywhere (exact integer multiplication is associative; no range clamp is
//! in the loop to break that). Section 2: the with-clamp variant must diverge
//! even at full fractional precision on some configuration (that is the bug
//! turned into the finding), and must diverge MORE than the no-clamp variant
//! at every w (the second mechanism adds failures, it does not remove any).
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p2 p2_multiplicative_grade_does_not_collapse.rs && ./p2

fn shift_reduce_trunc(x: i128, shift: u32) -> i128 {
    if shift == 0 {
        x
    } else {
        x >> shift
    }
}

/// section 1: no intermediate range clamp. narrows F bits after every
/// pairwise multiply (dropping only fraction, never truncating range), and
/// clamps to [0, M] exactly once, at the end, after narrowing off the w guard
/// bits. this isolates the fractional-coarsening axis from the range-clamp
/// axis entirely: nothing here can diverge for a reachability reason, only
/// for a rounding reason.
fn eager_no_clamp(ops: &[i64], f: u32, w: u32, m: i64, right_assoc: bool) -> i64 {
    let seq: Vec<i64> = if right_assoc {
        ops.iter().rev().copied().collect()
    } else {
        ops.to_vec()
    };
    let mut acc: i128 = (seq[0] as i128) << w;
    for &a in &seq[1..] {
        let raw = acc * (a as i128);
        acc = shift_reduce_trunc(raw, f); // fraction narrows; range is UNBOUNDED here
    }
    let final_reduced = shift_reduce_trunc(acc, w);
    final_reduced.clamp(0, m as i128) as i64
}

/// section 2: the bug kept as a variant. clamps the accumulator to the
/// format's range at EVERY intermediate step, at whatever guard width w is in
/// force, so a partial product that leaves [0, M<<w] is truncated back into
/// range immediately rather than carried exactly. this is what a real,
/// finite-range accumulator actually does.
fn eager_with_clamp(ops: &[i64], f: u32, w: u32, m: i64, right_assoc: bool) -> i64 {
    let seq: Vec<i64> = if right_assoc {
        ops.iter().rev().copied().collect()
    } else {
        ops.to_vec()
    };
    let m_wide: i128 = (m as i128) << w;
    let mut acc: i128 = (seq[0] as i128) << w;
    for &a in &seq[1..] {
        let raw = acc * (a as i128);
        let narrowed = shift_reduce_trunc(raw, f);
        acc = narrowed.clamp(0, m_wide);
    }
    let final_reduced = shift_reduce_trunc(acc, w);
    final_reduced.clamp(0, m as i128) as i64
}

/// exact product with NO rounding of any kind, NO intermediate range bound:
/// the true rational value, narrowed to scale F and range-clamped exactly
/// once, at the very end. this is what `eager_no_clamp` at w = (n-1)*F must
/// match everywhere, by construction: exact integer multiplication associates,
/// and there is no other lossy step in the way.
fn exact_once(ops: &[i64], f: u32, m: i64) -> i64 {
    let mut prod: i128 = 1;
    for &a in ops {
        prod *= a as i128;
    }
    let n = ops.len() as u32;
    let extra = (n - 1) * f;
    shift_reduce_trunc(prod, extra).clamp(0, m as i128) as i64
}

fn tuples_of(n: usize, m: i64) -> Vec<Vec<i64>> {
    let mut all: Vec<Vec<i64>> = vec![vec![]];
    for _ in 0..n {
        let mut next = Vec::with_capacity(all.len() * (m as usize + 1));
        for prefix in &all {
            for v in 0..=m {
                let mut p = prefix.clone();
                p.push(v);
                next.push(p);
            }
        }
        all = next;
    }
    all
}

fn divergence(
    fold: fn(&[i64], u32, u32, i64, bool) -> i64,
    ops: &[Vec<i64>],
    f: u32,
    w: u32,
    m: i64,
) -> u64 {
    let mut n = 0u64;
    for t in ops {
        let l = fold(t, f, w, m, false);
        let r = fold(t, f, w, m, true);
        if l != r {
            n += 1;
        }
    }
    n
}

fn min_sufficient_width(
    fold: fn(&[i64], u32, u32, i64, bool) -> i64,
    ops: &[Vec<i64>],
    f: u32,
    m: i64,
    full: u32,
) -> (u32, u64) {
    let div0 = divergence(fold, ops, f, 0, m);
    let mut w = 0u32;
    loop {
        let d = divergence(fold, ops, f, w, m);
        if d == 0 {
            return (w, div0);
        }
        if w >= full {
            return (full + 1, div0); // signals "not even full precision was clean"
        }
        w += 1;
    }
}

fn main() {
    let mut ok = true;
    let f = 3u32;
    let m = 15i64; // I = 1, F = 3 : Q1.3 unsigned, values denote r/8 in [0, 1.875]

    println!("=== section 1: pure fractional axis, no intermediate range clamp ===");
    println!();
    println!(
        "  format: M = {}, F = {} (I = 1). w = guard fractional bits kept",
        m, f
    );
    println!("  between steps; range is clamped exactly once, at the end, matching");
    println!("  exact_once's own shape. w=0 keeps no guard at all.");
    println!();
    println!(
        "  {:>2} {:>10} {:>12} {:>10} {:>12} {:>14}",
        "n", "tuples", "div at w=0", "full w", "min w clean", "min_w/full_w"
    );

    let mut zero_diverges_n3plus = true;
    let mut full_is_clean = true;
    let mut tracks_full = true;

    for n in 2usize..=4 {
        let full = ((n as u32) - 1) * f;
        let ops = tuples_of(n, m);
        let (minw, div0) = min_sufficient_width(eager_no_clamp, &ops, f, m, full);
        println!(
            "  {:>2} {:>10} {:>12} {:>10} {:>12} {:>13.2}%",
            n,
            ops.len(),
            div0,
            full,
            minw,
            if full > 0 {
                100.0 * minw as f64 / full as f64
            } else {
                0.0
            }
        );
        if n >= 3 && div0 == 0 {
            zero_diverges_n3plus = false;
        }
        if n >= 3 && minw > full {
            full_is_clean = false;
        }
        // measured pattern: min_w == full - F exactly (one narrowing's worth of
        // savings, constant in n), not a savings that itself grows with n the
        // way addition's does. either deviation from that exact pattern is
        // worth seeing, not just a loose "close to full" threshold.
        if n >= 3 && (full as i64) - (minw as i64) != f as i64 {
            tracks_full = false;
        }
        // cross-check against exact_once directly, at full precision
        let mut mismatch = 0u64;
        for t in &ops {
            let l = eager_no_clamp(t, f, full, m, false);
            let e = exact_once(t, f, m);
            if l != e {
                mismatch += 1;
            }
        }
        if mismatch != 0 {
            full_is_clean = false;
        }
    }

    println!();
    println!(
        "  n>=3 shows divergence at w=0 (matches p4's coarsen-only finding): {}",
        zero_diverges_n3plus
    );
    println!(
        "  full precision (w = (n-1)*F) is clean, LEFT == RIGHT == exact_once: {}",
        full_is_clean
    );
    println!(
        "  the savings below full precision is EXACTLY one rescale's worth (F bits),\n\
         constant in n, never more (min_w == full_w - F for every n >= 3 measured): {}",
        tracks_full
    );
    println!(
        "  so the ABSOLUTE guard still needed still grows linearly with n, unlike\n\
         addition's near-total, n-independent savings (57_probes/p6): one multiply\n\
         can be delayed past its narrowing point; the rest of the chain cannot."
    );
    ok &= zero_diverges_n3plus && full_is_clean && tracks_full;

    println!();
    println!("=== section 2: the bug, kept, and what it turned out to measure ===");
    println!();
    println!("  the first run of this probe clamped the accumulator to the format's");
    println!("  range at EVERY step, even at full guard width, and reported LEFT !=");
    println!("  RIGHT at w = (n-1)*F, which is impossible for exact integer");
    println!("  multiplication with no other lossy step present. it is a second,");
    println!("  independent source of non-associativity: an intermediate RANGE clamp,");
    println!("  exactly job one's absorption/reachability mechanism, now acting inside");
    println!("  a multiplicative fold rather than an additive one, and orthogonally to");
    println!("  the fractional axis this file exists to measure.");
    println!();
    println!(
        "  {:>2} {:>12} {:>14} {:>16}",
        "n", "full w", "no-clamp div", "with-clamp div"
    );
    let mut with_clamp_dirty_at_full = false;
    let mut with_clamp_always_worse_or_equal = true;
    for n in 3usize..=4 {
        let full = ((n as u32) - 1) * f;
        let ops = tuples_of(n, m);
        let no_clamp_full = divergence(eager_no_clamp, &ops, f, full, m);
        let with_clamp_full = divergence(eager_with_clamp, &ops, f, full, m);
        println!(
            "  {:>2} {:>12} {:>14} {:>16}",
            n, full, no_clamp_full, with_clamp_full
        );
        if with_clamp_full > 0 {
            with_clamp_dirty_at_full = true;
        }
        for w in 0..=full {
            let a = divergence(eager_no_clamp, &ops, f, w, m);
            let b = divergence(eager_with_clamp, &ops, f, w, m);
            if b < a {
                with_clamp_always_worse_or_equal = false;
            }
        }
    }
    println!();
    println!(
        "  the with-clamp variant still diverges at full fractional precision (the\n\
         bug's own symptom, reproduced and now understood): {}",
        with_clamp_dirty_at_full
    );
    println!(
        "  at every w measured, with-clamp divergence count >= no-clamp's (the second\n\
         mechanism adds failures, it substitutes for none): {}",
        with_clamp_always_worse_or_equal
    );
    ok &= with_clamp_dirty_at_full && with_clamp_always_worse_or_equal;

    println!();
    println!("=== contrast with 57_probes/p6's additive result ===");
    println!();
    println!("  addition needed exactly ONE bit less than the exact-sum width, at EVERY");
    println!("  fold length 2 through 8, uniformly: a constant offset from a bound that");
    println!("  itself grows only logarithmically in n (ceil(log2 n) extra integer bits).");
    println!("  the pure-fractional multiplicative guard measured in section 1 saves");
    println!("  EXACTLY one rescale's worth (F bits) below full precision, constant in n,");
    println!("  never more: only the LAST narrowing in the chain can be delayed. every");
    println!("  earlier one still needs its full F bits kept, so the ABSOLUTE guard still");
    println!("  grows LINEARLY in n. section 2 shows a second, additive-shaped mechanism");
    println!("  (range-clamp reachability) can ALSO fire on top of that, so the two are");
    println!("  not alternatives an implementation picks between; both apply, and paying");
    println!("  for one buys nothing against the other.");

    println!();
    println!("{}", if ok { "P2 WORKS" } else { "P2 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
