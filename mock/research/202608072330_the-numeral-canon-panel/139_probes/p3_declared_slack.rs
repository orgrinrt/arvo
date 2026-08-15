// p3: can a policy declare SLACK, so that a lowering which changes the answer
// is legal without breaking the firewall?
//
// p2 established that fusing a multiply-add changes the answer for signed
// types. Under the firewall that makes fusion a policy change, which would
// forbid a real and wanted optimisation. The way out, if there is one, is for
// a policy to specify a SET of acceptable answers rather than one: then fusion
// is legal exactly when its result stays inside the declared set, and the
// weighting picks freely within it.
//
// That is only useful if the required slack is SMALL. A policy that has to
// declare "any value in the whole range is acceptable" has specified nothing.
// So the question this probe answers is a magnitude: how much slack does
// admitting fusion actually cost, per shape?
//
// PREDICTIONS, recorded before the first run:
//   R1 under Wrap, the maximum fused-vs-unfused gap is 1 unit in the last
//      place at every F, because the only thing fusion removes is one
//      truncation, and truncation moves a value by less than one ulp.
//   R2 under Sat, the maximum gap is on the order of the whole range, because
//      an intermediate that saturated has thrown away magnitude the fused form
//      still has. p2's witness at W=6 F=0 signed already showed -1 against 31
//      on a range of -32..31, so I expect roughly half the range or more.
//   R3 therefore a 1-ulp slack declaration legalises fusion under Wrap and
//      does not legalise it under Sat at any useful slack.
//
// CONTROLS:
//   C1 the conformance checker must ACCEPT the arm that meets the spec. A
//      checker that rejects everything reports a large slack for free.
//   C2 the conformance checker must REJECT an arm deliberately placed one unit
//      outside the declared slack. A checker that accepts everything reports a
//      zero slack for free. Both directions are needed; either alone is
//      satisfiable by a broken checker.

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

fn rshift_trunc(p: i128, sh: u32) -> i128 {
    if sh == 0 {
        p
    } else {
        p / (1i128 << sh)
    }
}

fn red(v: i128, s: Shape, sat: bool) -> i128 {
    if sat {
        saturate(v, s)
    } else {
        wrap(v, s)
    }
}

fn madd_unfused(a: i128, b: i128, c: i128, s: Shape, sat: bool) -> i128 {
    let t = red(rshift_trunc(a * b, s.f), s, sat);
    red(t + c, s, sat)
}

fn madd_fused(a: i128, b: i128, c: i128, s: Shape, sat: bool) -> i128 {
    red(rshift_trunc(a * b + (c << s.f), s.f), s, sat)
}

// The spec: a policy pins the unfused result and declares a slack of `k` raw
// units around it. Conformance is membership, not equality.
fn conforms(result: i128, pinned: i128, k: i128) -> bool {
    (result - pinned).abs() <= k
}

fn domain(s: Shape) -> Vec<i128> {
    let (lo, hi) = range(s);
    (lo..=hi).collect()
}

fn main() {
    let mut failures = 0usize;
    println!("maximum fused-vs-unfused gap in raw units, W=6 exhaustive over all triples");
    println!("range at W=6: unsigned 0..63, signed -32..31, so 63 units spans the whole type");
    println!();

    for signed in [false, true] {
        for sat in [false, true] {
            let mut row = String::new();
            for f in 0..6u32 {
                let s = Shape { w: 6, f, signed };
                let d = domain(s);
                let mut maxgap = 0i128;
                for &a in &d {
                    for &b in &d {
                        for &c in &d {
                            let u = madd_unfused(a, b, c, s, sat);
                            let fu = madd_fused(a, b, c, s, sat);
                            let g = (u - fu).abs();
                            if g > maxgap {
                                maxgap = g;
                            }
                        }
                    }
                }
                row.push_str(&format!(" F={f}:{maxgap:>3}"));
            }
            println!(
                "  signed={:5} {}: max gap {}",
                signed,
                if sat { "Sat " } else { "Wrap" },
                row
            );
        }
    }

    println!();
    println!("conformance, with both controls, at the smallest slack that admits fusion");
    for signed in [false, true] {
        for sat in [false, true] {
            for f in [0u32, 2, 4] {
                let s = Shape { w: 6, f, signed };
                let d = domain(s);
                // find the smallest k that admits fusion everywhere
                let mut needed = 0i128;
                for &a in &d {
                    for &b in &d {
                        for &c in &d {
                            let g = (madd_unfused(a, b, c, s, sat)
                                - madd_fused(a, b, c, s, sat))
                            .abs();
                            if g > needed {
                                needed = g;
                            }
                        }
                    }
                }

                // C1: at slack = needed, the fused arm must be accepted everywhere.
                // C2: at slack = needed, an arm placed one unit further out must
                //     be rejected somewhere.
                let (mut c1_reject, mut c2_reject) = (0u64, 0u64);
                for &a in &d {
                    for &b in &d {
                        for &c in &d {
                            let pin = madd_unfused(a, b, c, s, sat);
                            let fu = madd_fused(a, b, c, s, sat);
                            if !conforms(fu, pin, needed) {
                                c1_reject += 1;
                            }
                            // the violator: one unit beyond whichever side the
                            // fused arm already sits on, so it is genuinely
                            // outside the declared set.
                            let dir = if fu >= pin { 1 } else { -1 };
                            let viol = pin + dir * (needed + 1);
                            if !conforms(viol, pin, needed) {
                                c2_reject += 1;
                            }
                        }
                    }
                }
                let total = (d.len() as u64).pow(3);
                let verdict = if c1_reject == 0 && c2_reject == total {
                    "PASS"
                } else {
                    failures += 1;
                    "FAIL"
                };
                println!(
                    "  signed={:5} {} F={}: slack needed={:>3} raw units ({:>6.2}% of range) | C1 rejects of conforming arm={} C2 rejects of violator={}/{} {}",
                    signed,
                    if sat { "Sat " } else { "Wrap" },
                    f,
                    needed,
                    100.0 * needed as f64 / ((1u64 << s.w) - 1) as f64,
                    c1_reject,
                    c2_reject,
                    total,
                    verdict
                );
            }
        }
    }

    println!();
    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
