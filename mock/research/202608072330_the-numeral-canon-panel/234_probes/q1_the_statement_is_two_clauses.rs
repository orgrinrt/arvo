// q1. `law::rounding_retraction_is_the_identity`'s `statement` is two clauses
// joined by "so", and they do not have the same truth value.
//
//   "Rounding a value already on the representable grid returns it unchanged,
//    so the reduction retracts."
//
//   clause A  forall grid points x. rnd(x) == x
//   clause B  forall a b c. rnd_F(rnd_F(a*b) * c) == rnd_2F(a*b*c)
//
// Clause B is the property `94_probes/c_retraction.rs` measures, and that file
// defines the word in its own header:
//
//   retracts(q, op1, op2)  :=  forall a b c. q(q(a op1 b) op2 c) == q(a op1 b op2 c)
//
// So the row's second clause IS what its fields are about, and only its first
// clause is not. Seat 228 section 3.3 asserts the sentence as a whole is "true
// of every mode at every fraction width" and checks clause A alone. This probe
// checks both, over the same mode set 228 used plus the two values of the axis
// 228 left out, and prints them side by side so the split is visible.
//
// The two values 228 left out, both of which bear on its proposed repair
// `fails: rounding any`:
//
//   `exact`       `dimension::rounding`'s grammar declares it: "`rounding =
//                 exact` names the case where nothing is discarded, which is a
//                 value of the axis rather than its absence." Nothing is
//                 discarded, so no staging can disagree with a deferral, so
//                 clause B HOLDS at `rounding = exact` at every fraction width.
//                 A `fails` region spanning `rounding any` therefore claims a
//                 failure at a declared value of the axis where the law holds.
//
//   `half_up`     under its second reading, ties away from zero, which
//   (ties-away)   `229` measured to be a shipped operation and a different
//                 function on a signed domain. Carried here so that the six
//                 columns are not silently one reading of a two-reading name.
//
// THE CASES THAT MUST FAIL, stated before the run:
//
//   C1  a quantiser that is not the identity on grid points must be caught by
//       part A. `Shifted` adds one grid step to every result. Expect nonzero.
//   C2  part D's zero for `exact` must be earned rather than structural. The
//       same comparison, with the "discard nothing" arm replaced by an arm that
//       discards exactly one bit, must report nonzero at every F >= 1. If it
//       does not, part D is measuring its own harness.
//   C3  part B's zero at F = 0 must be earned. `Shifted` at F = 0 must report
//       nonzero, or a column of zeros is indistinguishable from a dead loop.
//
// Build and run:
//   rustc --edition 2024 -O -o q1 q1_the_statement_is_two_clauses.rs && ./q1

use Mode::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUp,     // floor(x + 1/2), the corpus's reading
    HalfUpAway, // ties away from zero, the standard-name reading
    HalfEven,
    Exact,   // declared value of the axis: nothing is discarded
    Shifted, // planted, not a rounding mode: floor then add one grid step
    OneBit,  // planted: discards exactly one bit whatever F says
}

const CLAUSE_A_MODES: [Mode; 7] = [
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUp,
    HalfUpAway,
    HalfEven,
];

fn name(m: Mode) -> &'static str {
    match m {
        Floor => "floor",
        Ceil => "ceil",
        TowardZero => "toward_zero",
        AwayFromZero => "away_from_zero",
        HalfUp => "half_up(+inf)",
        HalfUpAway => "half_up(away)",
        HalfEven => "half_even",
        Exact => "exact",
        Shifted => "PLANTED shifted",
        OneBit => "PLANTED one-bit",
    }
}

/// Drop `s` bits from `p` under mode `m`, returning the value at the coarser
/// scale. `Exact` drops nothing, so it returns `p` at the finer scale; every
/// caller that uses it tracks the scale rather than assuming it moved.
fn rnd(p: i128, s: u32, m: Mode) -> i128 {
    if m == Exact {
        return p;
    }
    if m == OneBit {
        // discards one bit regardless of s, so it can never be a no-op
        return p.div_euclid(2);
    }
    if s == 0 {
        return if m == Shifted { p + 1 } else { p };
    }
    let d = 1i128 << s;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Floor => q,
        Ceil => q + if r == 0 { 0 } else { 1 },
        TowardZero => {
            if p >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        AwayFromZero => {
            if p >= 0 {
                q + if r == 0 { 0 } else { 1 }
            } else {
                q
            }
        }
        HalfUp => {
            if 2 * r >= d {
                q + 1
            } else {
                q
            }
        }
        HalfUpAway => {
            // nearest, ties away from zero. Off a tie it agrees with HalfUp.
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if p >= 0 {
                q + 1
            } else {
                q
            }
        }
        HalfEven => {
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
        Shifted => q + 1,
        Exact | OneBit => unreachable!(),
    }
}

/// Domain of the declared W-bit format at the given signedness, as raw scaled
/// integers.
fn domain(w: u32, signed: bool) -> (i128, i128) {
    if signed {
        (-(1i128 << (w - 1)), 1i128 << (w - 1))
    } else {
        (0, 1i128 << w)
    }
}

// --------------------------------------------------------------------------
// Part A. Clause 1. Rounding a value already on the grid returns it unchanged.
//
// A grid point at fraction width F, embedded into the ambient scale F + s, is
// k * 2^s. Rounding back must give k, for every mode and every s.
// --------------------------------------------------------------------------

fn clause_a_counterexamples(w: u32, s: u32, signed: bool, m: Mode) -> u64 {
    let (lo, hi) = domain(w, signed);
    let mut bad = 0u64;
    for k in lo..hi {
        let embedded = k << s;
        if rnd(embedded, s, m) != k {
            bad += 1;
        }
    }
    bad
}

// --------------------------------------------------------------------------
// Part B / D. Clause 2, the property the instrument named "retracts".
//
//   eager    = rnd_F( rnd_F(a*b) * c )
//   deferred = rnd_2F( a*b*c )
//
// Under `Exact` neither step discards anything, so both sides are the exact
// product at scale 3F and the comparison is between two identical expressions.
// That is the point: it is a value of the axis at which the law holds.
// --------------------------------------------------------------------------

fn chain_differ(w: u32, f: u32, signed: bool, m: Mode) -> (u64, u64) {
    let (lo, hi) = domain(w, signed);
    let mut differ = 0u64;
    let mut total = 0u64;
    for a in lo..hi {
        for b in lo..hi {
            let ab = a * b;
            let ab_q = rnd(ab, f, m);
            for c in lo..hi {
                total += 1;
                let eager = rnd(ab_q * c, f, m);
                let deferred = rnd(ab * c, 2 * f, m);
                if eager != deferred {
                    differ += 1;
                }
            }
        }
    }
    (differ, total)
}

fn main() {
    let mut sound = true;

    println!("q1. the statement is two clauses, and they disagree");
    println!();
    println!("== PART A. clause 1: rounding an on-grid value returns it unchanged ==");
    println!("counterexamples, over every grid point of the declared domain");
    print!("{:<18}", "mode");
    for s in 0..=6u32 {
        print!("{:>8}", format!("s={s}"));
    }
    println!("   signedness");
    for signed in [false, true] {
        for m in CLAUSE_A_MODES {
            print!("{:<18}", name(m));
            let mut tot = 0u64;
            for s in 0..=6u32 {
                let bad = clause_a_counterexamples(8, s, signed, m);
                tot += bad;
                print!("{:>8}", bad);
            }
            println!("   {}", if signed { "signed" } else { "unsigned" });
            if tot != 0 {
                println!("  !! clause A failed for a real mode, which contradicts the theorem");
                sound = false;
            }
        }
    }
    println!();
    println!("  C1 must-fail: the planted non-identity map, same checker");
    print!("{:<18}", name(Shifted));
    let mut c1 = 0u64;
    for s in 0..=6u32 {
        let bad = clause_a_counterexamples(8, s, false, Shifted);
        c1 += bad;
        print!("{:>8}", bad);
    }
    println!("   unsigned");
    if c1 == 0 {
        println!("  C1 FAILED: the checker cannot detect a non-retraction, so part A's zeros mean nothing");
        sound = false;
    } else {
        println!("  C1 ok: {c1} counterexamples, so part A's zeros are earned");
    }

    println!();
    println!("== PART B. clause 2: the property the instrument calls retraction ==");
    println!("differing triples out of the whole cube, unsigned, W in {{4, 6}}");
    println!(
        "{:<18} {:>3} {:>3} {:>14} {:>14}   {}",
        "mode", "W", "F", "triples", "differ", "verdict"
    );
    let mut b_zero_at_f0 = true;
    let mut b_nonzero_above = true;
    for m in [
        Floor,
        Ceil,
        TowardZero,
        HalfUp,
        HalfUpAway,
        HalfEven,
        AwayFromZero,
    ] {
        for w in [4u32, 6] {
            for f in 0..=w {
                let (differ, total) = chain_differ(w, f, false, m);
                println!(
                    "{:<18} {:>3} {:>3} {:>14} {:>14}   {}",
                    name(m),
                    w,
                    f,
                    total,
                    differ,
                    if differ == 0 { "holds" } else { "FAILS" }
                );
                if f == 0 && differ != 0 {
                    b_zero_at_f0 = false;
                }
                if f >= 1 && differ == 0 {
                    b_nonzero_above = false;
                }
            }
        }
    }
    println!();
    println!(
        "  clause 2 holds at every F = 0 cell: {b_zero_at_f0}\n  clause 2 fails at every F >= 1 cell, every listed mode: {b_nonzero_above}"
    );
    println!(
        "  So clause 1 is a theorem and clause 2 is not, and the row's `fails`\n  field is about clause 2. The `so` joining them is the defect."
    );

    println!();
    println!("== PART D. the declared value `rounding = exact` ==");
    println!("`dimension::rounding`: \"`rounding = exact` names the case where nothing");
    println!("is discarded, which is a value of the axis rather than its absence.\"");
    println!(
        "{:<18} {:>3} {:>3} {:>14} {:>14}   {}",
        "mode", "W", "F", "triples", "differ", "verdict"
    );
    let mut exact_all_hold = true;
    for signed in [false, true] {
        for w in [4u32, 6] {
            for f in 0..=w {
                let (differ, total) = chain_differ(w, f, signed, Exact);
                if differ != 0 {
                    exact_all_hold = false;
                }
                println!(
                    "{:<18} {:>3} {:>3} {:>14} {:>14}   {}  {}",
                    name(Exact),
                    w,
                    f,
                    total,
                    differ,
                    if differ == 0 { "holds" } else { "FAILS" },
                    if signed { "signed" } else { "unsigned" }
                );
            }
        }
    }
    if !exact_all_hold {
        println!("  !! `exact` failed somewhere, which would defeat the finding");
        sound = false;
    }

    println!();
    println!("  C2 must-fail: the same comparison with an arm that discards one bit");
    let mut c2_seen = 0u64;
    for w in [4u32, 6] {
        for f in 0..=w {
            let (differ, _) = chain_differ(w, f, false, OneBit);
            c2_seen += differ;
            println!("    W={w} F={f}  differ={differ}");
        }
    }
    if c2_seen == 0 {
        println!("  C2 FAILED: the harness reports zero for an arm that does discard, so");
        println!("             part D's zeros are structural rather than measured");
        sound = false;
    } else {
        println!("  C2 ok: {c2_seen} differing triples on the planted discarding arm,");
        println!("         so part D's zeros come from `exact` and not from the harness");
    }

    println!();
    println!("  C3 must-fail: part B's F = 0 zeros, against the planted shifted map");
    let (c3, _) = chain_differ(4, 0, false, Shifted);
    if c3 == 0 {
        println!("  C3 FAILED: the F = 0 column cannot detect a broken quantiser");
        sound = false;
    } else {
        println!("  C3 ok: {c3} differing triples at W=4 F=0 for the planted map");
    }

    println!();
    println!("== WHAT THIS REFUTES ==");
    println!("1. `228` section 3.3: \"That sentence is true of every mode at every");
    println!("   fraction width.\" Part B: the sentence's second clause is false at");
    println!("   every F >= 1 for every mode. Only clause 1 is a theorem, and");
    println!("   `228`'s control C2 checks clause 1 alone.");
    println!("2. `228` finding F4, `fails: rounding: rounding any`. Part D: at the");
    println!("   declared value `rounding = exact` the law HOLDS at every cell of");
    println!("   both signednesses, so `any` on the `fails` side is false.");
    println!();
    println!("instrument: {}", if sound { "sound" } else { "UNSOUND" });
    if !sound {
        std::process::exit(1);
    }
}
