// Probe 01: does `+` distribute over MIN the way file 13's probe 2 checked
// distribution over MAX? arvo-comb's matrix_chain_dp (dp.rs:100) computes
// `dp[i][k] + dp[k+1][j]` and combines candidates by `total_cmp ... Less`,
// i.e. it is a min-plus recurrence: dp[lo][hi] = min over k of
// (dp[lo][k] + dp[k+1][hi]), dual to arvo-graph's max-plus rank/path.
//
// Hypothesis under test: the min-plus dual sorts the presets the SAME way
// file 13's max-plus probe found (saturate satisfies distributivity over
// the reduction, wrap does not), because both are instances of one fact
// (translation is order-preserving) rather than two independent facts.
//
// Model: representable range [-4, 3] (matches file 13's probe 2 exactly,
// for direct comparability). Recovery maps: Wrap (mod 8, mapped back into
// signed range), Saturate (clamp), SubstituteZero, Exact (no recovery,
// checked against the literal integers with no wraparound at all).
//
// Run: rustc -O 01_min_plus_dual.rs -o /tmp/min_plus_dual && /tmp/min_plus_dual

const LO: i64 = -4;
const HI: i64 = 3;
const RANGE: i64 = HI - LO + 1; // 8

fn wrap(x: i64) -> i64 {
    // map x into [LO, HI] modulo RANGE, matching two's-complement wraparound
    // over an 8-value signed range with LO = -RANGE/2.
    let m = ((x - LO) % RANGE + RANGE) % RANGE;
    m + LO
}

fn saturate(x: i64) -> i64 {
    if x < LO {
        LO
    } else if x > HI {
        HI
    } else {
        x
    }
}

fn sub_zero(x: i64) -> i64 {
    if x < LO || x > HI {
        0
    } else {
        x
    }
}

fn add(recover: fn(i64) -> i64, a: i64, b: i64) -> i64 {
    recover(a + b)
}

fn check_distributes_over_min(name: &str, recover: fn(i64) -> i64) {
    let mut ok = true;
    let mut counterexample: Option<(i64, i64, i64)> = None;
    for w in LO..=HI {
        for a in LO..=HI {
            for b in LO..=HI {
                let lhs = add(recover, w, if a < b { a } else { b }); // w + min(a,b)
                let m1 = add(recover, w, a);
                let m2 = add(recover, w, b);
                let rhs = if m1 < m2 { m1 } else { m2 }; // min(w+a, w+b)
                if lhs != rhs {
                    ok = false;
                    counterexample = Some((w, a, b));
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if !ok {
            break;
        }
    }
    match counterexample {
        None => println!("{name}: distributes over MIN: yes"),
        Some((w, a, b)) => println!(
            "{name}: distributes over MIN: NO at (w={w}, a={a}, b={b}): w+min(a,b)={}, min(w+a,w+b)={}",
            add(recover, w, if a < b { a } else { b }),
            {
                let m1 = add(recover, w, a);
                let m2 = add(recover, w, b);
                if m1 < m2 { m1 } else { m2 }
            }
        ),
    }
}

fn check_associative(name: &str, recover: fn(i64) -> i64) {
    let mut ok = true;
    let mut counterexample: Option<(i64, i64, i64)> = None;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                let lhs = add(recover, add(recover, a, b), c);
                let rhs = add(recover, a, add(recover, b, c));
                if lhs != rhs {
                    ok = false;
                    counterexample = Some((a, b, c));
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if !ok {
            break;
        }
    }
    match counterexample {
        None => println!("{name}: + associative: yes"),
        Some((a, b, c)) => println!("{name}: + associative: NO at ({a},{b},{c})"),
    }
}

// arvo-comb's actual recurrence, at a tiny fixed instance, to see whether
// the DP's own answer (not just the abstract law) disagrees between the
// two nestings under Wrap. Three leaves with costs c0, c1, c2; the DP
// picks min(cost(0,0)+cost(1,2)_best, cost(0,1)_best+cost(2,2)) where each
// sub-cost is itself a min over its own splits. For N=3 there is only one
// non-trivial split choice at the root: split after 0, or split after 1.
fn dp3(recover: fn(i64) -> i64, c: [i64; 3]) -> i64 {
    // leaves
    let leaf = |i: usize| c[i];
    // pair costs: only one way to combine two adjacent leaves
    let pair = |i: usize, j: usize| add(recover, leaf(i), leaf(j));
    // root: min( leaf(0)+pair(1,2), pair(0,1)+leaf(2) )
    let split_after_0 = add(recover, leaf(0), pair(1, 2));
    let split_after_1 = add(recover, pair(0, 1), leaf(2));
    if split_after_0 < split_after_1 {
        split_after_0
    } else {
        split_after_1
    }
}

fn main() {
    println!("model: representables [{LO}, {HI}], min-plus recurrence (arvo-comb shape)\n");

    let variants: [(&str, fn(i64) -> i64); 3] = [
        ("Wrap", wrap as fn(i64) -> i64),
        ("Saturate", saturate as fn(i64) -> i64),
        ("SubstituteZero", sub_zero as fn(i64) -> i64),
    ];

    println!("| arith | + assoc | + distributes over min |");
    println!("|---|---|---|");
    for (name, recover) in variants {
        check_associative(name, recover);
        check_distributes_over_min(name, recover);
    }
    // exact: no wraparound recovery at all, model as identity (values stay
    // in a wide enough range that recovery never fires in this probe).
    let exact = |x: i64| x;
    check_associative("Exact", exact);
    check_distributes_over_min("Exact", exact);

    println!("\nthree-leaf DP disagreement check (dp3), costs enumerated exhaustively over [{LO},{HI}]^3:");
    for (name, recover) in variants {
        let mut disagreements = 0u64;
        let mut total = 0u64;
        let mut example: Option<[i64; 3]> = None;
        for c0 in LO..=HI {
            for c1 in LO..=HI {
                for c2 in LO..=HI {
                    total += 1;
                    let c = [c0, c1, c2];
                    // reference: exact arithmetic, no recovery, then compare
                    // sign of which split the recovered version picked
                    // against which split exact arithmetic would pick.
                    let exact_split0 = c0 + (c1 + c2);
                    let exact_split1 = (c0 + c1) + c2;
                    let exact_best = if exact_split0 < exact_split1 {
                        exact_split0
                    } else {
                        exact_split1
                    };
                    let got = dp3(recover, c);
                    if got != exact_best && example.is_none() {
                        example = Some(c);
                    }
                    if got != exact_best {
                        disagreements += 1;
                    }
                }
            }
        }
        println!(
            "  {name}: {disagreements}/{total} triples where dp3 != exact best split{}",
            match example {
                Some(c) => format!(
                    ", e.g. costs={:?} dp3={} exact_best={}",
                    c,
                    dp3(recover, c),
                    {
                        let s0 = c[0] + (c[1] + c[2]);
                        let s1 = (c[0] + c[1]) + c[2];
                        if s0 < s1 {
                            s0
                        } else {
                            s1
                        }
                    }
                ),
                None => String::new(),
            }
        );
    }
}
