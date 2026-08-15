// p2: is the cost-driven half of a strategy answer-invisible?
//
// The claim under test, which I am calling the OBSERVABILITY FIREWALL:
//   two lowerings of the SAME policy agree on every input, so which one a cost
//   model picks cannot be observed in an answer;
//   and a lowering that changes the answer is not a lowering at all, it is a
//   different policy, and it has to be declared as one.
//
// Part A takes one policy (saturating fixed-point multiply) and implements it
// twice by genuinely different routes, then compares exhaustively.
//
// Part B takes the optimisation a backend most wants to make on this shape,
// fusing multiply-add so the intermediate is never rounded, and asks whether
// it agrees. If it does not, fusing crosses the firewall: it is a policy
// change wearing a codegen choice's clothes.
//
// PREDICTIONS, recorded before the first run:
//   Q1 Part A: zero disagreements at every shape. Both arms compute the same
//      specified function; only the route differs.
//   Q2 Part B: nonzero disagreements at every F > 0, and ZERO at F = 0. At
//      F = 0 there is no intermediate rounding to skip, so the only thing
//      fusion removes is the intermediate saturation, which still bites, so
//      I actually expect nonzero at F = 0 too under Sat. Recorded as stated:
//      nonzero at F = 0 under Sat, zero at F = 0 under Wrap.
//   Q3 The wrong-clamp control arm disagrees at every shape where saturation
//      can fire at all.
//
// CONTROLS:
//   C1 the deliberately wrong arm (clamps one short of the maximum) MUST be
//      reported as differing. A comparator that cannot see that is not
//      measuring anything in part A.
//   C2 part A's agreement must be non-vacuous: report how many results are
//      nonzero and how many saturate. Two arms that both return zero agree
//      perfectly and prove nothing.
//   C3 part B's sample must REACH the case that can differ. Added after the
//      first run produced a false zero: a stride of 4 makes every sampled
//      product a multiple of 16, so at F <= 4 the shift is exact, truncation
//      never fires, and the fused and unfused arms agree for a reason that has
//      nothing to do with the question. The control counts sampled pairs whose
//      shift is INEXACT and fails when that is zero at F > 0. The strides are
//      now odd so a product is not systematically divisible by a power of two.

#[derive(Clone, Copy, Debug)]
struct Shape {
    w: u32,
    f: u32,
    signed: bool,
}

fn range(s: Shape) -> (i128, i128) {
    let m = 1i128 << s.w;
    if s.signed {
        (-(m >> 1), (m >> 1) - 1)
    } else {
        (0, m - 1)
    }
}

fn saturate(v: i128, s: Shape) -> i128 {
    let (lo, hi) = range(s);
    v.clamp(lo, hi)
}

fn wrap(v: i128, s: Shape) -> i128 {
    let m = 1i128 << s.w;
    let r = v.rem_euclid(m);
    if s.signed && r >= (m >> 1) {
        r - m
    } else {
        r
    }
}

// truncate toward zero after dividing by 2^sh
fn rshift_trunc(p: i128, sh: u32) -> i128 {
    if sh == 0 {
        p
    } else {
        p / (1i128 << sh)
    }
}

// ---------------------------------------------------------------- part A arms

// Arm 1: the obvious route. Widen, multiply, shift, clamp.
fn mul_arm_widen(a: i128, b: i128, s: Shape) -> i128 {
    saturate(rshift_trunc(a * b, s.f), s)
}

// Arm 2: a genuinely different route. Split each operand at the halfway bit
// and accumulate the four partial products separately, the way a machine
// without a full-width multiplier would have to, then shift and clamp.
// Nothing here reuses arm 1's expression.
fn mul_arm_partials(a: i128, b: i128, s: Shape) -> i128 {
    let h = (s.w / 2).max(1);
    let base = 1i128 << h;
    // signed-safe split: sign is carried on the high part
    let (asig, au) = if a < 0 { (-1i128, -a) } else { (1i128, a) };
    let (bsig, bu) = if b < 0 { (-1i128, -b) } else { (1i128, b) };
    let (a1, a0) = (au / base, au % base);
    let (b1, b0) = (bu / base, bu % base);
    let p = a1 * b1 * base * base + a1 * b0 * base + a0 * b1 * base + a0 * b0;
    let p = asig * bsig * p;
    saturate(rshift_trunc(p, s.f), s)
}

// Control arm: identical to arm 1 except the upper clamp is one short. This is
// what an off-by-one in a saturation bound looks like.
fn mul_arm_wrongclamp(a: i128, b: i128, s: Shape) -> i128 {
    let (lo, hi) = range(s);
    rshift_trunc(a * b, s.f).clamp(lo, hi - 1)
}

// ---------------------------------------------------------------- part B arms

// Unfused: round the product to the declared scale, reduce it, then add.
fn madd_unfused(a: i128, b: i128, c: i128, s: Shape, sat: bool) -> i128 {
    let red = |v: i128| if sat { saturate(v, s) } else { wrap(v, s) };
    let t = red(rshift_trunc(a * b, s.f));
    red(t + c)
}

// Fused: keep the product exact, add c at the product's scale, round once,
// reduce once. This is the shape a fused multiply-add takes.
fn madd_fused(a: i128, b: i128, c: i128, s: Shape, sat: bool) -> i128 {
    let red = |v: i128| if sat { saturate(v, s) } else { wrap(v, s) };
    red(rshift_trunc(a * b + (c << s.f), s.f))
}

fn domain(s: Shape) -> Vec<i128> {
    let (lo, hi) = range(s);
    (lo..=hi).collect()
}

fn main() {
    let shapes = [
        Shape {
            w: 8,
            f: 0,
            signed: false,
        },
        Shape {
            w: 8,
            f: 0,
            signed: true,
        },
        Shape {
            w: 8,
            f: 3,
            signed: false,
        },
        Shape {
            w: 8,
            f: 3,
            signed: true,
        },
        Shape {
            w: 8,
            f: 7,
            signed: true,
        },
        Shape {
            w: 10,
            f: 5,
            signed: true,
        },
    ];
    let mut failures = 0usize;

    println!("PART A: two routes, one policy (saturating multiply, truncating)");
    for s in shapes {
        let d = domain(s);
        let (mut n, mut diff_ab, mut diff_ac) = (0u64, 0u64, 0u64);
        let (mut nonzero, mut saturated) = (0u64, 0u64);
        let (_, hi) = range(s);
        let (lo, _) = range(s);
        for &a in &d {
            for &b in &d {
                n += 1;
                let x = mul_arm_widen(a, b, s);
                let y = mul_arm_partials(a, b, s);
                let z = mul_arm_wrongclamp(a, b, s);
                if x != y {
                    diff_ab += 1;
                }
                if x != z {
                    diff_ac += 1;
                }
                if x != 0 {
                    nonzero += 1;
                }
                let exact = rshift_trunc(a * b, s.f);
                if exact > hi || exact < lo {
                    saturated += 1;
                }
            }
        }
        println!(
            "  W={} F={} signed={}: pairs={} arm1-vs-arm2 differ={} | nonzero results={} saturating inputs={}",
            s.w, s.f, s.signed, n, diff_ab, nonzero, saturated
        );
        println!(
            "    C1 wrong-clamp control detected as differing at {} of {} pairs",
            diff_ac, n
        );
        if diff_ab != 0 {
            println!("    Q1 FAILED: two routes to one policy disagree");
            failures += 1;
        }
        // C1: the control must be detectable wherever saturation can fire.
        if saturated > 0 && diff_ac == 0 {
            println!("    C1 FAILED: comparator cannot see a known-wrong arm");
            failures += 1;
        }
        // C2: agreement must be non-vacuous.
        if nonzero == 0 {
            println!("    C2 FAILED: every result is zero, agreement is vacuous");
            failures += 1;
        }
    }

    println!();
    println!("PART B: fusing the multiply-add, same declared policy");
    let b_shapes = [
        (Shape { w: 6, f: 0, signed: false }, 1usize),
        (Shape { w: 6, f: 0, signed: true }, 1),
        (Shape { w: 6, f: 2, signed: false }, 1),
        (Shape { w: 6, f: 2, signed: true }, 1),
        (Shape { w: 6, f: 4, signed: true }, 1),
        (Shape { w: 8, f: 3, signed: false }, 5),
        (Shape { w: 8, f: 3, signed: true }, 5),
        (Shape { w: 8, f: 7, signed: true }, 5),
    ];
    for (s, st) in b_shapes {
        let d = domain(s);
        // C3: does the sampled sub-domain ever produce an inexact shift?
        let mut inexact = 0u64;
        let mut sampled_pairs = 0u64;
        for a in d.iter().step_by(st) {
            for b in d.iter().step_by(st) {
                sampled_pairs += 1;
                if s.f > 0 && (a * b).rem_euclid(1i128 << s.f) != 0 {
                    inexact += 1;
                }
            }
        }
        if s.f > 0 && inexact == 0 {
            println!(
                "  W={} F={} signed={}: C3 FAILED, stride={} makes every shift exact",
                s.w, s.f, s.signed, st
            );
            failures += 1;
        }
        for sat in [false, true] {
            let (mut n, mut diff) = (0u64, 0u64);
            let mut first: Option<(i128, i128, i128, i128, i128)> = None;
            for a in d.iter().step_by(st) {
                for b in d.iter().step_by(st) {
                    for c in d.iter().step_by(st) {
                        n += 1;
                        let u = madd_unfused(*a, *b, *c, s, sat);
                        let f = madd_fused(*a, *b, *c, s, sat);
                        if u != f {
                            diff += 1;
                            if first.is_none() {
                                first = Some((*a, *b, *c, u, f));
                            }
                        }
                    }
                }
            }
            let pol = if sat { "Sat " } else { "Wrap" };
            print!(
                "  W={} F={} signed={:5} {} stride={}: triples={:>9} differ={:>9} ({:>6.2}%) inexact-shift pairs={}/{}",
                s.w, s.f, s.signed, pol, st, n, diff,
                100.0 * diff as f64 / n as f64, inexact, sampled_pairs
            );
            match first {
                Some((a, b, c, u, f)) => {
                    println!("  witness a={a} b={b} c={c} unfused={u} fused={f}")
                }
                None => println!("  no witness"),
            }
        }
    }

    println!();
    println!("PART C: fusion difference rate over the WHOLE fraction axis, W=6 exhaustive");
    println!("  (the predicate wants every F, not three sampled values)");
    for signed in [false, true] {
        for sat in [false, true] {
            let mut row = String::new();
            for f in 0..6u32 {
                let s = Shape { w: 6, f, signed };
                let d = domain(s);
                let (mut n, mut diff) = (0u64, 0u64);
                for &a in &d {
                    for &b in &d {
                        for &c in &d {
                            n += 1;
                            if madd_unfused(a, b, c, s, sat) != madd_fused(a, b, c, s, sat) {
                                diff += 1;
                            }
                        }
                    }
                }
                row.push_str(&format!(
                    " F={f}:{:>6.2}%",
                    100.0 * diff as f64 / n as f64
                ));
            }
            println!(
                "  signed={:5} {}: {}",
                signed,
                if sat { "Sat " } else { "Wrap" },
                row
            );
        }
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
