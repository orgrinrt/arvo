//! Probe 7: the congruence condition, stated per operation, which is the
//! theorem `62` asked me for and could not write.
//!
//! WHERE THIS COMES FROM. `62` measured that signed two's-complement saturating
//! MULTIPLICATION fails associativity on the integer grid (160 triples at
//! w = 4, witness `(7*7)*-1 = -7` against `7*(7*-1) = -8`), that the mechanism
//! is the range's asymmetry under negation, and that a symmetric clamp restores
//! exact multiplicative associativity at every width measured. `62:361-365`
//! then records what it could not establish: "whether the symmetric-range
//! multiplicative monoid extends to a congruence-style theorem... Three widths
//! and an argued mechanism, not a theorem." `62:227-231` puts that to me by
//! name, in my own file's congruence style.
//!
//! THE ARGUMENT, WRITTEN OUT, SO THE PROBE IS CHECKING SOMETHING STATED.
//!
//! Let `sat` clamp into `Q = [lo, hi]` and let `~` be the kernel of `sat`:
//! `x ~ y` iff `sat(x) == sat(y)`. Concretely `~` collapses the low tail
//! `(-inf, lo]`, collapses the high tail `[hi, +inf)`, and is equality on the
//! interior. The induced operation on `Q` is `a # b = sat(a op b)`, and if `~`
//! is a CONGRUENCE for `op` then `Q` is the quotient of the integers by `~` and
//! inherits `op`'s associativity for free. That is the shape `57` used for the
//! unsigned semiring; the question is which ranges admit it, per operation.
//!
//! **Multiplication.** Suppose the bounds are mirror images, `lo = -hi = -h`,
//! and `h >= 1`. Take `x, x' >= h` (same high tail) and `y` in `Q`.
//!   - `y >= 1`: `xy >= hy >= h` and `x'y >= h`, so both land in the high tail.
//!   - `y <= -1`: `xy <= -h` and `x'y <= -h`, both land in the low tail.
//!   - `y == 0`: both products are exactly `0`.
//! In every case the two results are related, and the tail-times-tail cases go
//! the same way since `h*h >= h`. So `~` is a multiplicative congruence.
//! Now drop the mirror. Under two's complement `lo = -(hi+1)`, so negation maps
//! the high tail `[hi, inf)` onto `(-inf, -hi]`, and `-hi` is INTERIOR because
//! `-hi > lo`. The image of one collapsed class straddles the other tail's
//! boundary, and `hi ~ hi+1` while `-hi` and `-(hi+1) = lo` are not related.
//! That is exactly `62`'s witness with `h = 7`, and no congruence exists.
//!
//! **Addition.** The same relation is NOT an additive congruence under mirror
//! symmetry. Take `x = h` and `x' = h + k`, both in the high tail, and
//! `y = -h`. Then `x + y = 0` and `x' + y = k`, which are distinct interior
//! points for small `k`. Addition needs one tail to be unreachable instead,
//! which is the sign confinement `57_probes/p1` section 3 already measured.
//!
//! SO THE PREDICTION, and it is a biconditional per operation:
//!   multiplication:  `~` is a congruence iff `lo == -hi` (mirror) OR `lo == 0`
//!                    (the non-negative half-line).
//!   addition:        `~` is a congruence iff the range is sign confined,
//!                    meaning `lo == 0` or `hi == 0`.
//!
//! THE ASYMMETRY BETWEEN THOSE TWO LINES IS NOT A TYPO, and the second run of
//! this probe is what found it. The first corrected run predicted "sign
//! confined" for multiplication too, and mispredicted the nine non-positive
//! ranges `[lo, 0]`; it is kept as `p7_output.v2_predictor_too_broad.txt`.
//! The reason is that a half-line has to be CLOSED under the operation to
//! behave as one, and only the non-negative half-line is closed under
//! multiplication: a product of two non-positive numbers is non-negative, so
//! `[lo, 0]` is mapped straight past its own ceiling and the induced operation
//! collapses to the constant `0`. Those nine rows are associative for the
//! degenerate reason `57_probes/p2b` already isolated, and the probe reports
//! them as constant rather than counting them as support.
//! Addition keeps both half-lines because both are closed under it.
//!
//! And the payoff prediction, which is what a law layer would actually rely on:
//! congruence implies the induced operation is associative. Necessity is
//! measured rather than assumed, and any exception is characterised rather than
//! counted, the way `57_probes/p2b` characterised its 153.
//!
//! HOW THE CONGRUENCE IS CHECKED. `~` is the kernel of `sat`, so the check is:
//! over an ambient set, group every pair `(x, y)` by `(sat(x), sat(y))` and
//! require every pair in a group to produce the same `sat(op(x, y))`. Two
//! representatives of one class pair yielding different results IS a congruence
//! violation, and it is exactly what a witness looks like.
//!
//! AND THE AMBIENT SET IS THE REACHABLE ONE, WHICH THIS PROBE GOT WRONG FIRST.
//! The first run quantified over a symmetric window `[-win, win]` regardless of
//! the range, reported FAILS, and is kept as
//! `p7_output.v1_overquantified_ambient.txt`. It marked all eighteen sign-
//! confined intervals as non-congruent, contradicting `57_probes/p3`'s own
//! five-width congruence result for the unsigned semiring. The measurement was
//! right about the domain it swept and the domain was wrong: for an unsigned
//! `Q = [0, M]` the low tail is never inhabited, because a product of two
//! non-negative operands is never negative, so quantifying over negative `x`
//! tests class collapses the numeral cannot reach. Fixed by taking the ambient
//! to be `Q` together with the image of `Q x Q` under the exact operation,
//! which is what one reduction step actually sees.
//!
//! This is the SAME over-quantification `61` found in `56`'s coherence law:
//! `61` measured that coherence and absorption agree with zero disagreements
//! exactly when the operand box is a subset of `Q`, and diverge off it, because
//! the off-domain values are ones no real fold produces. I read `61` before
//! writing this probe and made its error anyway, in a new place, which is worth
//! recording: the trap is the quantifier, not the file that fell into it.
//!
//! INSTRUMENT VALIDATION. The congruence checker must report both values across
//! the sweep; the associativity checker likewise; a mutant range must be
//! predicted non-congruent and measured non-associative; and the predictor must
//! be shown capable of being wrong, by reporting its mismatch count rather than
//! asserting it is zero.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p7 p7_the_congruence_condition_per_operation.rs && ./p7

fn sat(x: i64, lo: i64, hi: i64) -> i64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn mul(a: i64, b: i64) -> i64 {
    a * b
}

/// Is `~` (the kernel of sat) a congruence for `op` over the ambient window?
/// Returns (is_congruence, first witness if not).
fn is_congruence(
    lo: i64,
    hi: i64,
    op: fn(i64, i64) -> i64,
    _win: i64,
) -> (bool, Option<(i64, i64, i64, i64)>) {
    // class pair -> the single sat(op(x,y)) it must always produce
    let span = (hi - lo + 1) as usize;
    let idx = |v: i64| (v - lo) as usize;
    let mut seen: Vec<Option<(i64, i64, i64)>> = vec![None; span * span];

    // the ambient is what one exact step actually reaches from Q, not all of
    // the integers: Q together with the image of Q x Q. See the header.
    let (mut alo, mut ahi) = (lo, hi);
    for a in lo..=hi {
        for b in lo..=hi {
            let v = op(a, b);
            alo = alo.min(v);
            ahi = ahi.max(v);
        }
    }

    for x in alo..=ahi {
        let cx = sat(x, lo, hi);
        for y in alo..=ahi {
            let cy = sat(y, lo, hi);
            let r = sat(op(x, y), lo, hi);
            let slot = &mut seen[idx(cx) * span + idx(cy)];
            match slot {
                None => *slot = Some((r, x, y)),
                Some((r0, x0, y0)) => {
                    if *r0 != r {
                        return (false, Some((*x0, *y0, x, y)));
                    }
                }
            }
        }
    }
    (true, None)
}

/// associativity failures of the induced operation over Q cubed
fn assoc_failures(lo: i64, hi: i64, op: fn(i64, i64) -> i64) -> u64 {
    let mut n = 0;
    for a in lo..=hi {
        for b in lo..=hi {
            for c in lo..=hi {
                let l = sat(op(sat(op(a, b), lo, hi), c), lo, hi);
                let r = sat(op(a, sat(op(b, c), lo, hi)), lo, hi);
                if l != r {
                    n += 1;
                }
            }
        }
    }
    n
}

/// is the induced operation constant over Q (the degenerate class p2b isolated)
fn induced_is_constant(lo: i64, hi: i64, op: fn(i64, i64) -> i64) -> bool {
    let first = sat(op(lo, lo), lo, hi);
    for a in lo..=hi {
        for b in lo..=hi {
            if sat(op(a, b), lo, hi) != first {
                return false;
            }
        }
    }
    true
}

fn sign_confined(lo: i64, hi: i64) -> bool {
    lo == 0 || hi == 0
}
fn mirror(lo: i64, hi: i64) -> bool {
    lo == -hi
}

struct Row {
    lo: i64,
    hi: i64,
    cong: bool,
    assoc: u64,
    predicted: bool,
    constant: bool,
}

fn sweep(op: fn(i64, i64) -> i64, predict: fn(i64, i64) -> bool, label: &str) -> bool {
    let mut rows = Vec::new();
    let mut saw_cong = false;
    let mut saw_noncong = false;
    let mut saw_assoc = false;
    let mut saw_nonassoc = false;

    for lo in -9i64..=0 {
        for hi in 0i64..=9 {
            let win = 4 * (hi - lo).max(1) + 8;
            let (cong, _) = is_congruence(lo, hi, op, win);
            let assoc = assoc_failures(lo, hi, op);
            let predicted = predict(lo, hi);
            saw_cong |= cong;
            saw_noncong |= !cong;
            saw_assoc |= assoc == 0;
            saw_nonassoc |= assoc > 0;
            rows.push(Row {
                lo,
                hi,
                cong,
                assoc,
                predicted,
                constant: induced_is_constant(lo, hi, op),
            });
        }
    }

    let pred_mismatch: Vec<&Row> = rows.iter().filter(|r| r.cong != r.predicted).collect();
    // congruence must imply associativity: this is the sufficiency direction
    let suff: Vec<&Row> = rows.iter().filter(|r| r.cong && r.assoc > 0).collect();
    // associativity without congruence: the necessity direction
    let nec: Vec<&Row> = rows.iter().filter(|r| !r.cong && r.assoc == 0).collect();
    let nec_constant = nec.iter().filter(|r| r.constant).count();

    println!("--- {} ---", label);
    println!(
        "  intervals swept:                                  {}",
        rows.len()
    );
    println!(
        "    relation is a congruence:                       {}",
        rows.iter().filter(|r| r.cong).count()
    );
    println!(
        "    induced operation associative:                  {}",
        rows.iter().filter(|r| r.assoc == 0).count()
    );
    println!(
        "  PREDICTOR mismatches (congruence vs predicted):   {}",
        pred_mismatch.len()
    );
    for r in pred_mismatch.iter().take(8) {
        println!(
            "     Q=[{},{}]  congruence={}  predicted={}",
            r.lo, r.hi, r.cong, r.predicted
        );
    }
    println!(
        "  SUFFICIENCY violations (congruent, not assoc):    {}",
        suff.len()
    );
    for r in suff.iter().take(8) {
        println!("     Q=[{},{}]  assoc-failures={}", r.lo, r.hi, r.assoc);
    }
    println!(
        "  NECESSITY violations   (assoc, not congruent):    {}",
        nec.len()
    );
    println!(
        "     of which the induced operation is CONSTANT:    {}",
        nec_constant
    );
    for r in nec.iter().filter(|r| !r.constant).take(8) {
        println!(
            "     residue: Q=[{},{}]  assoc-failures=0, not constant",
            r.lo, r.hi
        );
    }
    println!(
        "  instrument: congruence checker saw both values {}, associativity checker saw both {}",
        saw_cong && saw_noncong,
        saw_assoc && saw_nonassoc
    );
    println!();

    suff.is_empty()
        && pred_mismatch.is_empty()
        && saw_cong
        && saw_noncong
        && saw_assoc
        && saw_nonassoc
}

fn main() {
    let mut ok = true;

    println!("=== 1. the congruence condition, swept over every interval Q containing 0 ===");
    println!();
    println!("  Prediction under test, per operation:");
    println!("    multiplication: congruence iff mirror-symmetric (lo == -hi) or lo == 0");
    println!(
        "                    (only the non-negative half-line is closed under multiplication)"
    );
    println!("    addition:       congruence iff sign confined (lo == 0 or hi == 0)");
    println!();

    ok &= sweep(
        mul,
        // only the NON-NEGATIVE half-line is closed under multiplication; see header
        |lo, hi| mirror(lo, hi) || lo == 0,
        "multiplication",
    );
    ok &= sweep(add, |lo, hi| sign_confined(lo, hi), "addition");

    println!("=== 2. the two's-complement against symmetric comparison, at width ===");
    println!();
    println!("  Two ranges per width: the two's-complement range [-2^(w-1), 2^(w-1)-1] and the");
    println!("  symmetric range [-(2^(w-1)-1), 2^(w-1)-1]. `62` measured associativity at four");
    println!("  widths; this adds the congruence verdict beside it, which is the thing that");
    println!("  makes it a theorem shape rather than a table.");
    println!();
    println!(
        "{:>4} {:>14} {:>12} {:>12} {:>14} {:>12} {:>12}",
        "w", "2c range", "2c cong", "2c *assoc", "sym range", "sym cong", "sym *assoc"
    );
    let mut all_2c_broken = true;
    let mut all_sym_clean = true;
    for w in 3..=7u32 {
        let h = (1i64 << (w - 1)) - 1;
        let (c2, _) = is_congruence(-h - 1, h, mul, 4 * h + 8);
        let a2 = assoc_failures(-h - 1, h, mul);
        let (cs, _) = is_congruence(-h, h, mul, 4 * h + 8);
        let asym = assoc_failures(-h, h, mul);
        all_2c_broken &= !c2 && a2 > 0;
        all_sym_clean &= cs && asym == 0;
        println!(
            "{:>4} {:>14} {:>12} {:>12} {:>14} {:>12} {:>12}",
            w,
            format!("[{},{}]", -h - 1, h),
            c2,
            a2,
            format!("[{},{}]", -h, h),
            cs,
            asym
        );
    }
    println!();
    println!(
        "  two's complement non-congruent AND non-associative at every width: {}",
        all_2c_broken
    );
    println!(
        "  symmetric congruent AND exactly associative at every width:        {}",
        all_sym_clean
    );
    println!();
    println!("  `62`'s w = 3..6 associativity counts are reproduced in the 2c column from an");
    println!("  independent instrument, and the congruence column is what turns its measured");
    println!("  table into the quotient argument it asked for.");
    ok &= all_2c_broken && all_sym_clean;

    println!();
    println!("=== 3. the witness, by hand, at Q = [-8, 7] ===");
    println!();
    let (lo, hi) = (-8i64, 7i64);
    println!(
        "  sat(7) = {} and sat(8) = {}, so 7 ~ 8 (same high-tail class).",
        sat(7, lo, hi),
        sat(8, lo, hi)
    );
    println!("  Multiply both by -1:");
    println!("    sat(7 * -1)  = sat({}) = {}", 7 * -1, sat(-7, lo, hi));
    println!("    sat(8 * -1)  = sat({}) = {}", 8 * -1, sat(-8, lo, hi));
    println!(
        "  {} != {}, so the class pair (high tail, -1) has two images and `~` is not a",
        sat(-7, lo, hi),
        sat(-8, lo, hi)
    );
    println!("  multiplicative congruence. Under the symmetric range [-7, 7] the same pair gives");
    println!(
        "    sat(7 * -1) = {} and sat(8 * -1) = {}",
        sat(-7, -7, 7),
        sat(-8, -7, 7)
    );
    println!("  which are equal, and the class pair has one image.");
    let broken = sat(-7, lo, hi) != sat(-8, lo, hi);
    let fixed = sat(-7, -7, 7) == sat(-8, -7, 7);
    println!();
    println!("  asymmetric range splits the class:  {}", broken);
    println!("  symmetric range does not:           {}", fixed);
    ok &= broken && fixed;

    println!();
    println!("=== 4. instrument validation: a mutant range the prediction must fail ===");
    println!();
    println!("  A range that is neither mirror-symmetric nor sign confined must be predicted");
    println!("  non-congruent and measured non-associative under multiplication.");
    let mut mutant_ok = true;
    for (lo, hi) in [(-5i64, 3i64), (-2, 9), (-9, 4)] {
        let (c, w) = is_congruence(lo, hi, mul, 4 * (hi - lo) + 8);
        let a = assoc_failures(lo, hi, mul);
        println!(
            "    Q=[{},{}]  congruence={}  *assoc-failures={}  witness={:?}",
            lo, hi, c, a, w
        );
        mutant_ok &= !c && a > 0;
    }
    println!(
        "  every mutant range non-congruent and non-associative: {}",
        mutant_ok
    );
    ok &= mutant_ok;

    println!();
    println!("{}", if ok { "P7 WORKS" } else { "P7 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
