// p3: is 35's asymmetry (unsigned saturating addition associative at 33 of 33
// cells; signed saturating addition non-associative at 33 of 33) a fact about
// arvo's fixed-point widths, a fact about "one clamp versus two", or a fact
// about something else the first two candidates conflate?
//
// FIRST ATTEMPT, KEPT WITH ITS FAILURE, per RULES.md's discipline that a
// probe proving the wrong thing stays on disk with its output. The first
// version of this file clamped only a ceiling and left the floor entirely
// unconstrained (operands ranged over negative integers too), predicting
// zero associativity failures. It found 904 of 3375 at top=3. The prediction
// was wrong, and the reason is instructive: an unconstrained floor is not
// the same structure as unsigned addition's floor. Unsigned addition never
// goes negative because its OPERANDS are non-negative, not because a clamp
// is silently active there; there is no clamp EVENT at the bottom because
// the exact sum of two non-negative numbers is never negative. Letting a, b,
// c range over negative integers with no floor clamp reintroduces the two-
// sided interaction the single clamp was supposed to remove. That run and
// its output are kept below as `clamp_top_only_unconstrained_floor`, run
// first, so the correction is visible rather than quietly edited out.
//
// SECOND ATTEMPT, which isolates the real variable. Two candidate
// explanations remain and this separates them:
//
//   H1  "one active clamp associates; two do not"
//   H2  "operands (and therefore every intermediate exact sum) confined to
//        a half-line that already contains the result associates; letting
//        the exact sum range over both signs, with or without a clamp
//        there, does not"
//
// H1 and H2 agree on unsigned saturating add (one clamp, non-negative
// operands) and on signed saturating add (two clamps, operands of both
// signs) but disagree on two cases nobody has tested:
//
//   - non-negative operands, TWO clamps (an explicit floor at 0 that is
//     never actually reached, plus a ceiling): H1 predicts failure (two
//     clamps), H2 predicts success (operands never leave the half-line).
//   - operands of both signs, ONE clamp (only a ceiling, floor
//     unconstrained): this is the first attempt above. H1 predicted
//     success and was refuted. H2 predicts failure, correctly.
//
// So H1 is already refuted by the first attempt. The remaining question is
// whether H2 is right, or whether some third thing is going on. This probe
// tests the discriminating case directly: non-negative operands under an
// explicit two-sided clamp (floor 0, finite ceiling).
//
// Exhaustive over small integer boxes.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p3 p3_one_ended_clamp_versus_two.rs && ./p3

fn clamp_top_only(x: i64, top: i64) -> i64 {
    if x > top {
        top
    } else {
        x
    }
}

fn clamp_both(x: i64, lo: i64, hi: i64) -> i64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

fn assoc_fail_top_only(top: i64, lo_operand: i64, hi_operand: i64) -> (u64, u64) {
    let mut total = 0u64;
    let mut fail = 0u64;
    for a in lo_operand..=hi_operand {
        for b in lo_operand..=hi_operand {
            for c in lo_operand..=hi_operand {
                total += 1;
                let lhs = clamp_top_only(clamp_top_only(a + b, top) + c, top);
                let rhs = clamp_top_only(a + clamp_top_only(b + c, top), top);
                if lhs != rhs {
                    fail += 1;
                }
            }
        }
    }
    (total, fail)
}

fn assoc_fail_both(lo: i64, hi: i64, lo_operand: i64, hi_operand: i64) -> (u64, u64) {
    let mut total = 0u64;
    let mut fail = 0u64;
    for a in lo_operand..=hi_operand {
        for b in lo_operand..=hi_operand {
            for c in lo_operand..=hi_operand {
                total += 1;
                let lhs = clamp_both(clamp_both(a + b, lo, hi) + c, lo, hi);
                let rhs = clamp_both(a + clamp_both(b + c, lo, hi), lo, hi);
                if lhs != rhs {
                    fail += 1;
                }
            }
        }
    }
    (total, fail)
}

fn main() {
    println!("=== first attempt, kept with its refutation ===");
    println!("clamp_top_only_unconstrained_floor: one clamp (ceiling), operands of both signs");
    for top in [3i64, 8, 15] {
        let range = top + 4;
        let (total, fail) = assoc_fail_top_only(top, -range, range);
        println!(
            "  top={:<3} operands in [{:>4},{:<4}] triples={:>7}  assoc-failures={}   H1 predicted 0, got this",
            top, -range, range, total, fail
        );
    }

    println!();
    println!("=== second attempt, the discriminating case ===");
    println!("non-negative operands (0..=N), one clamp at the top, floor never reached:");
    for (top, n) in [(3i64, 12), (8, 16), (15, 24)] {
        let (total, fail) = assoc_fail_top_only(top, 0, n);
        println!(
            "  top={:<3} operands in [0,{:<3}] triples={:>7}  assoc-failures={}",
            top, n, total, fail
        );
    }

    println!();
    println!("non-negative operands (0..=N), TWO clamps (explicit floor 0, ceiling top),");
    println!("floor still structurally unreachable from non-negative operands:");
    for (top, n) in [(3i64, 12), (8, 16), (15, 24)] {
        let (total, fail) = assoc_fail_both(0, top, 0, n);
        println!(
            "  lo=0 hi={:<3} operands in [0,{:<3}] triples={:>7}  assoc-failures={}",
            top, n, total, fail
        );
    }

    println!();
    println!("non-negative operands (0..=N), TWO clamps where the floor IS reachable");
    println!("(a positive floor above 0, so a small enough partial sum clamps low):");
    for (lo, top, n) in [(2i64, 10, 14), (5, 20, 26)] {
        let (total, fail) = assoc_fail_both(lo, top, 0, n);
        println!(
            "  lo={:<3} hi={:<3} operands in [0,{:<3}] triples={:>7}  assoc-failures={}",
            lo, top, n, total, fail
        );
    }

    println!();
    println!("H1 (one clamp associates, two do not) predicts failures in the second and third");
    println!("blocks above and none in the first. H2 (confinement to a half-line that already");
    println!("contains the result associates, regardless of clamp count) predicts failures");
    println!("only in the fourth block, where the floor is actually reachable, and none in the");
    println!("second or third.");
}
