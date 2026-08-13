// Probe C. WHICH per-operation policies may be decided per operation?
//
// A strategy that selects a policy at each operation implements its stated
// objective only if that objective is a property of ONE operation. Where the
// objective is a property of a CHAIN, per-operation selection implements it
// exactly when the policy RETRACTS: applying it at every step gives the same
// answer as applying it once at the end.
//
//   retracts(q, op1, op2)  :=  forall a b c.
//       q(q(a op1 b) op2 c)  ==  q(a op1 b op2 c)
//
// with the right-hand chain evaluated exactly in a domain wide enough that
// nothing else intervenes. Three separate law questions are decided here, each
// exhaustively over its whole small domain, and each yields a REGION rather
// than a verdict:
//
//   part 1  retraction of an overflow policy, over the full 3x3 matrix of
//           chain shapes, because a homogeneous chain answers a different and
//           much easier question than a mixed one.
//   part 2  retraction of a rounding policy in fixed point, swept over F.
//   part 3  associativity, which is a different law from retraction and is
//           what licenses splitting a fold into lanes.
//
// An earlier revision of part 1 swept only homogeneous chains (add-then-add,
// sub-then-sub). Every policy "retracted" under it. That was setup that helps:
// a chain monotone in one direction can never escape a clamp, so the clamp's
// position cannot matter. The matrix below is the whole thing.
//
// Build and run:
//   rustc --edition 2024 -O -o c_retraction c_retraction.rs && ./c_retraction

/// The model widths swept. Every law below is decided exhaustively at each of
/// these and the verdict is reported per width, so a region that depends on the
/// width is visible rather than hidden behind one convenient choice.
const WIDTHS: [u32; 6] = [2, 3, 4, 5, 6, 8];

#[derive(Copy, Clone, PartialEq)]
enum Policy {
    Saturate,
    Wrap,
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
        }
    }
    fn apply(self, a: i128, b: i128) -> i128 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
        }
    }
}

/// Quantise a wide exact value back into the declared unsigned W-bit domain.
fn q(p: Policy, n: i128, x: i128) -> i128 {
    match p {
        Policy::Saturate => {
            let limit = n - 1;
            if x < 0 {
                0
            } else if x > limit {
                limit
            } else {
                x
            }
        }
        Policy::Wrap => x.rem_euclid(n),
    }
}

// --------------------------------------------------------------------------
// Part 1. Retraction of an overflow policy, over every chain shape.
// --------------------------------------------------------------------------

fn part1() {
    println!("part 1: does an overflow policy retract over a two-operation chain?");
    println!("        exhaustive over a, b, c in 0..2^W; every (op1, op2) pair; W swept");
    print!("{:<10} {:<10}", "policy", "chain");
    for w in WIDTHS {
        print!("{:>12}", format!("W={w}"));
    }
    println!("   {}", "region where it retracts");

    for p in [Policy::Saturate, Policy::Wrap] {
        let pname = if p == Policy::Saturate {
            "saturate"
        } else {
            "wrap"
        };
        for op1 in [Op::Add, Op::Sub, Op::Mul] {
            for op2 in [Op::Add, Op::Sub, Op::Mul] {
                print!(
                    "{:<10} {:<10}",
                    pname,
                    format!("{}>{}", op1.name(), op2.name())
                );
                let mut holds: Vec<u32> = Vec::new();
                for w in WIDTHS {
                    let n: i128 = 1 << w;
                    let mut differ: u64 = 0;
                    let mut total: u64 = 0;
                    for a in 0..n {
                        for b in 0..n {
                            let ab_exact = op1.apply(a, b);
                            let ab_q = q(p, n, ab_exact);
                            for c in 0..n {
                                total += 1;
                                if q(p, n, op2.apply(ab_q, c)) != q(p, n, op2.apply(ab_exact, c)) {
                                    differ += 1;
                                }
                            }
                        }
                    }
                    if differ == 0 {
                        holds.push(w);
                        print!("{:>12}", "-");
                    } else {
                        print!("{:>11.2}%", 100.0 * differ as f64 / total as f64);
                    }
                }
                if holds.len() == WIDTHS.len() {
                    println!("   RETRACTS at every swept W");
                } else if holds.is_empty() {
                    println!("   retracts at no swept W");
                } else {
                    println!("   retracts only at W in {holds:?}");
                }
            }
        }
    }
}

// --------------------------------------------------------------------------
// Part 2. Rounding policy on a two-multiply chain in fixed point, swept over
// the fraction width. Values are integers scaled by 2^F. A fixed-point
// multiply of two such produces a value scaled by 2^(2F), so returning to the
// declared scale costs a shift, and the shift is where the policy lives.
// --------------------------------------------------------------------------

fn part2() {
    println!();
    println!("part 2: does a rounding policy retract over a two-multiply chain in fixed point?");
    println!("        exhaustive over a, b, c in 0..2^W; swept over W and over F in 0..=W");
    println!(
        "{:<10} {:>3} {:>3} {:>14} {:>14} {:>9} {:>12}   {}",
        "policy", "W", "F", "triples", "differ", "pct", "max |diff|", "verdict"
    );

    for &(name, nearest) in [("truncate", false), ("nearest", true)].iter() {
        for w in [4u32, 6, 8] {
            let n: u128 = 1 << w;
            for f in 0..=w {
                let half: u128 = if f == 0 { 0 } else { 1 << (f - 1) };
                let half2: u128 = if f == 0 { 0 } else { 1 << (2 * f - 1) };
                let qf = |x: u128| -> u128 {
                    if nearest {
                        (x + half) >> f
                    } else {
                        x >> f
                    }
                };
                let mut differ: u64 = 0;
                let mut total: u64 = 0;
                let mut worst: u128 = 0;
                for a in 0..n {
                    for b in 0..n {
                        let ab_exact = a * b;
                        let ab_q = qf(ab_exact);
                        for c in 0..n {
                            total += 1;
                            let eager = qf(ab_q * c);
                            let deferred = if nearest {
                                (ab_exact * c + half2) >> (2 * f)
                            } else {
                                (ab_exact * c) >> (2 * f)
                            };
                            if eager != deferred {
                                differ += 1;
                                let d = eager.abs_diff(deferred);
                                if d > worst {
                                    worst = d;
                                }
                            }
                        }
                    }
                }
                println!(
                    "{:<10} {:>3} {:>3} {:>14} {:>14} {:>8.2}% {:>12}   {}",
                    name,
                    w,
                    f,
                    total,
                    differ,
                    100.0 * differ as f64 / total as f64,
                    worst,
                    if differ == 0 {
                        "RETRACTS"
                    } else {
                        "does not retract"
                    }
                );
            }
        }
    }
}

// --------------------------------------------------------------------------
// Part 3. Associativity, which is a DIFFERENT law from retraction. Retraction
// says the policy may be applied per step. Associativity says the fold may be
// re-bracketed, which is what licenses splitting it into independent lanes.
// A policy can retract and not associate, and the two questions are answered
// separately because they license different arms.
// --------------------------------------------------------------------------

fn part3() {
    println!();
    println!("part 3: is the per-operation policy associative? (the lane-splitting licence)");
    println!("        exhaustive over a, b, c in 0..2^W; W swept");
    print!("{:<10} {:<6}", "policy", "op");
    for w in WIDTHS {
        print!("{:>12}", format!("W={w}"));
    }
    println!("   {}", "region where it associates");

    for p in [Policy::Saturate, Policy::Wrap] {
        let pname = if p == Policy::Saturate {
            "saturate"
        } else {
            "wrap"
        };
        for op in [Op::Add, Op::Sub, Op::Mul] {
            print!("{:<10} {:<6}", pname, op.name());
            let mut holds: Vec<u32> = Vec::new();
            for w in WIDTHS {
                let n: i128 = 1 << w;
                let step = |x: i128, y: i128| q(p, n, op.apply(x, y));
                let mut differ: u64 = 0;
                let mut total: u64 = 0;
                for a in 0..n {
                    for b in 0..n {
                        let ab = step(a, b);
                        for c in 0..n {
                            total += 1;
                            if step(ab, c) != step(a, step(b, c)) {
                                differ += 1;
                            }
                        }
                    }
                }
                if differ == 0 {
                    holds.push(w);
                    print!("{:>12}", "-");
                } else {
                    print!("{:>11.2}%", 100.0 * differ as f64 / total as f64);
                }
            }
            if holds.len() == WIDTHS.len() {
                println!("   ASSOCIATES at every swept W");
            } else if holds.is_empty() {
                println!("   associates at no swept W");
            } else {
                println!("   associates only at W in {holds:?}");
            }
        }
    }
}

fn main() {
    part1();
    part2();
    part3();
}
