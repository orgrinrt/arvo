// q1: `139` called one pair of expressions two things, forty lines apart, in the
// same dispatch. This establishes that mechanically rather than by eye.
//
// In `139_probes/p1_policy_classes.rs` the pair appears as two positions on an
// OBSERVABLE POLICY AXIS named `Intermediate`:
//
//     Intermediate::Exact    => reduce(rshift(a * b + (c << f), f, rd), s, ov)
//     Intermediate::Stepwise => { let t = reduce(rshift(a*b, f, rd), s, ov);
//                                 reduce(t + c, s, ov) }
//
// In `139_probes/p2_firewall.rs` the same two expressions appear as two
// LOWERINGS OF ONE POLICY, named `madd_fused` and `madd_unfused`, and the
// difference between them is what `139` section 4 proposes a slack mechanism to
// license.
//
// Those two readings are incompatible. If the pair is an axis, the difference is
// a declared policy difference and no mechanism is needed to permit it. If the
// pair is one policy lowered twice, the difference is a firewall violation. It
// cannot be both, and `139` asserted both.
//
// PREDICTIONS, recorded before the first run:
//   A1 p1's Exact and p2's fused agree on every input, bit for bit.
//   A2 p1's Stepwise and p2's unfused agree on every input, bit for bit.
//   A3 therefore the slack mechanism in `139` section 4 was proposed to buy a
//      capability `139`'s own previous probe had already modelled as an axis
//      position.
//
// CONTROLS:
//   C1 the CROSS pairing (p1's Exact against p2's unfused) must DIFFER
//      somewhere. If all four expressions agree everywhere, A1 and A2 are
//      vacuous and this probe proves nothing.
//   C2 the sweep must be non-vacuous: count nonzero results.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overflow {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rounding {
    Trunc,
    NearestEven,
    Floor,
}

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

fn wrap(v: i128, s: Shape) -> i128 {
    let m = 1i128 << s.w;
    let r = v.rem_euclid(m);
    if s.signed && r >= (m >> 1) {
        r - m
    } else {
        r
    }
}

fn saturate(v: i128, s: Shape) -> i128 {
    let (lo, hi) = range(s);
    v.clamp(lo, hi)
}

fn reduce(v: i128, s: Shape, ov: Overflow) -> i128 {
    match ov {
        Overflow::Wrap => wrap(v, s),
        Overflow::Sat => saturate(v, s),
    }
}

// p1's rounding, transcribed verbatim from 139_probes/p1_policy_classes.rs
fn rshift(p: i128, sh: u32, rd: Rounding) -> i128 {
    if sh == 0 {
        return p;
    }
    let d = 1i128 << sh;
    match rd {
        Rounding::Floor => p.div_euclid(d),
        Rounding::Trunc => p / d,
        Rounding::NearestEven => {
            let q = p.div_euclid(d);
            let r = p.rem_euclid(d);
            let half = d >> 1;
            if r > half {
                q + 1
            } else if r < half {
                q
            } else if q.rem_euclid(2) == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

// p2's rounding, transcribed verbatim from 139_probes/p2_firewall.rs
fn rshift_trunc(p: i128, sh: u32) -> i128 {
    if sh == 0 {
        p
    } else {
        p / (1i128 << sh)
    }
}

// ---- the four expressions, each transcribed from the file it appears in ----

// 139_probes/p1_policy_classes.rs, Op::Madd, Intermediate::Exact
fn p1_exact(a: i128, b: i128, c: i128, s: Shape, rd: Rounding, ov: Overflow) -> i128 {
    let acc = a * b + (c << s.f);
    reduce(rshift(acc, s.f, rd), s, ov)
}

// 139_probes/p1_policy_classes.rs, Op::Madd, Intermediate::Stepwise
fn p1_stepwise(a: i128, b: i128, c: i128, s: Shape, rd: Rounding, ov: Overflow) -> i128 {
    let t = reduce(rshift(a * b, s.f, rd), s, ov);
    reduce(t + c, s, ov)
}

// 139_probes/p2_firewall.rs, madd_fused
fn p2_fused(a: i128, b: i128, c: i128, s: Shape, sat: bool) -> i128 {
    let red = |v: i128| if sat { saturate(v, s) } else { wrap(v, s) };
    red(rshift_trunc(a * b + (c << s.f), s.f))
}

// 139_probes/p2_firewall.rs, madd_unfused
fn p2_unfused(a: i128, b: i128, c: i128, s: Shape, sat: bool) -> i128 {
    let red = |v: i128| if sat { saturate(v, s) } else { wrap(v, s) };
    let t = red(rshift_trunc(a * b, s.f));
    red(t + c)
}

fn domain(s: Shape) -> Vec<i128> {
    let (lo, hi) = range(s);
    (lo..=hi).collect()
}

fn main() {
    let mut failures = 0usize;
    println!("p1's Intermediate axis positions against p2's two lowerings");
    println!("p2 fixes rounding at toward-zero, so p1 is read at Rounding::Trunc to match");
    println!();

    let mut total = 0u64;
    let (mut d_exact_fused, mut d_step_unfused, mut d_cross) = (0u64, 0u64, 0u64);
    let mut nonzero = 0u64;

    for w in [4u32, 6] {
        for f in 0..w {
            for signed in [false, true] {
                let s = Shape { w, f, signed };
                let d = domain(s);
                for ov in [Overflow::Wrap, Overflow::Sat] {
                    let sat = ov == Overflow::Sat;
                    let (mut n, mut a1, mut a2, mut cx) = (0u64, 0u64, 0u64, 0u64);
                    for &a in &d {
                        for &b in &d {
                            for &c in &d {
                                n += 1;
                                let pe = p1_exact(a, b, c, s, Rounding::Trunc, ov);
                                let ps = p1_stepwise(a, b, c, s, Rounding::Trunc, ov);
                                let pf = p2_fused(a, b, c, s, sat);
                                let pu = p2_unfused(a, b, c, s, sat);
                                if pe != pf {
                                    a1 += 1;
                                }
                                if ps != pu {
                                    a2 += 1;
                                }
                                if pe != pu {
                                    cx += 1;
                                }
                                if pe != 0 {
                                    nonzero += 1;
                                }
                            }
                        }
                    }
                    total += n;
                    d_exact_fused += a1;
                    d_step_unfused += a2;
                    d_cross += cx;
                    if a1 != 0 || a2 != 0 {
                        println!(
                            "  W={w} F={f} signed={signed} {:?}: A1 diffs={a1} A2 diffs={a2}  <-- MISMATCH",
                            ov
                        );
                        failures += 1;
                    }
                }
            }
        }
    }

    println!("  swept {total} input triples across W in {{4,6}}, every F, both signednesses,");
    println!("  both overflow positions.");
    println!();
    println!("  A1  p1 Intermediate::Exact    vs p2 madd_fused    : {d_exact_fused} differences");
    println!("  A2  p1 Intermediate::Stepwise vs p2 madd_unfused  : {d_step_unfused} differences");
    println!("  C1  p1 Intermediate::Exact    vs p2 madd_unfused  : {d_cross} differences (control)");
    println!("  C2  nonzero results: {nonzero}");
    println!();

    if d_exact_fused == 0 && d_step_unfused == 0 {
        println!("  A1 and A2 CONFIRMED: the two files compute the same two functions.");
    } else {
        println!("  A1/A2 FAILED");
        failures += 1;
    }
    if d_cross > 0 {
        println!("  C1 PASS: the cross pairing differs at {d_cross} inputs, so the identity");
        println!("     above is a fact about the pairing rather than about all four agreeing.");
    } else {
        println!("  C1 FAIL: everything agrees, so A1 and A2 are vacuous.");
        failures += 1;
    }
    if nonzero == 0 {
        println!("  C2 FAIL: every result zero.");
        failures += 1;
    }

    println!();
    println!("A3, which is the consequence and not a measurement:");
    println!("  `139` section 2 presents this pair as an axis whose two positions are");
    println!("  observationally distinct, and counts them in its class table. `139`");
    println!("  section 4 then presents the same pair as one policy lowered two ways and");
    println!("  proposes a slack field on every policy to license the difference. The");
    println!("  capability the mechanism was buying is the axis position the same");
    println!("  dispatch had already modelled two probes earlier.");

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
