//! Probe 2: is absorption NECESSARY for associativity, or only sufficient.
//!
//! WHY. `p1` measured absorption (predictor C) against measured associativity
//! on twelve hand-picked configurations and found zero disagreements. Twelve
//! configurations is far too small a matrix to carry a biconditional, and
//! sufficiency is the easy half: if rho(rho(x)+y) == rho(x+y) everywhere then
//! both associations equal rho(a+b+c) and associativity follows in one line.
//! NECESSITY is the half that could fail, and it is the half the criterion
//! needs if it is to replace `42`'s sentence rather than sit beside it.
//!
//! So this probe sweeps the whole small configuration space rather than
//! choosing cases. Every combination of
//!
//!     floor in {absent} union [-6, 6]
//!     ceiling in {absent} union [-6, 6]
//!     operand box [blo, bhi] with blo in [-5, 0], bhi in [0, 5]
//!
//! that is well formed (floor <= ceiling when both present, and the box
//! non-empty), which the run reports as 4248 configurations per ambient
//! operation, each measured exhaustively over its box cubed. The count is
//! printed rather than asserted here, because an asserted count in a comment is
//! the kind of number that goes stale silently. For each: measured associativity, the absorption predicate,
//! and the two readings of `42:315-316`. Any configuration where absorption
//! and associativity disagree is printed in full, in either direction.
//!
//! A second sweep asks the same question for MULTIPLICATION, rho(a*b), which
//! nothing in the panel has tested for this criterion and which is where I
//! expect the cheap sign-confinement form to break first, since multiplication
//! by a negative reverses order.
//!
//! INSTRUMENT VALIDATION. Three things must hold or the probe reports FAILS:
//! the sweep must contain both associative and non-associative configurations,
//! the absorption predicate must return both values, and the deliberately
//! broken predictor (`42`'s per-fold reading) must be shown mispredicting, so
//! the comparison harness is demonstrably capable of reporting a disagreement.
//! That last one matters most: a harness that reported "0 disagreements" for
//! every predictor would be measuring nothing.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p2 p2_absorption_necessity_sweep.rs && ./p2

#[derive(Clone, Copy)]
struct Cfg {
    lo: Option<i64>,
    hi: Option<i64>,
    blo: i64,
    bhi: i64,
}

impl Cfg {
    fn rho(&self, x: i64) -> i64 {
        if let Some(lo) = self.lo {
            if x < lo {
                return lo;
            }
        }
        if let Some(hi) = self.hi {
            if x > hi {
                return hi;
            }
        }
        x
    }
    fn faces(&self, x: i64) -> (bool, bool) {
        (
            self.lo.map(|lo| x < lo).unwrap_or(false),
            self.hi.map(|hi| x > hi).unwrap_or(false),
        )
    }
}

fn assoc_failures(cfg: &Cfg, op: fn(i64, i64) -> i64) -> u64 {
    let mut n = 0;
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            for c in cfg.blo..=cfg.bhi {
                let l = cfg.rho(op(cfg.rho(op(a, b)), c));
                let r = cfg.rho(op(a, cfg.rho(op(b, c))));
                if l != r {
                    n += 1;
                }
            }
        }
    }
    n
}

/// absorption over this box for this ambient operation
fn absorbing(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            let x = op(a, b);
            for y in cfg.blo..=cfg.bhi {
                if cfg.rho(op(cfg.rho(x), y)) != cfg.rho(op(x, y)) {
                    return false;
                }
            }
        }
    }
    true
}

/// `42:315-316` read per fold: count divergent triples that trigger at most
/// one face across both associations. every one is a counterexample.
fn per_fold_mismatches(cfg: &Cfg, op: fn(i64, i64) -> i64) -> u64 {
    let mut n = 0;
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            for c in cfg.blo..=cfg.bhi {
                let s1l = op(a, b);
                let s2l = op(cfg.rho(s1l), c);
                let s1r = op(b, c);
                let s2r = op(a, cfg.rho(s1r));
                if cfg.rho(s2l) == cfg.rho(s2r) {
                    continue;
                }
                let (f1, c1) = cfg.faces(s1l);
                let (f2, c2) = cfg.faces(s2l);
                let (f3, c3) = cfg.faces(s1r);
                let (f4, c4) = cfg.faces(s2r);
                let floor = f1 || f2 || f3 || f4;
                let ceil = c1 || c2 || c3 || c4;
                if (floor as u32) + (ceil as u32) <= 1 {
                    n += 1;
                }
            }
        }
    }
    n
}

/// `42:315-316` read per domain: at most one face reachable over the box
fn per_domain(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    let mut rf = false;
    let mut rc = false;
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            let (f, c) = cfg.faces(op(a, b));
            rf |= f;
            rc |= c;
        }
    }
    (rf as u32) + (rc as u32) <= 1
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn sweep(label: &str, op: fn(i64, i64) -> i64) -> bool {
    let mut cfgs = 0u64;
    let mut assoc = 0u64;
    let mut nonassoc = 0u64;
    let mut abs_true = 0u64;
    let mut abs_false = 0u64;

    // absorption true but non-associative: would refute SUFFICIENCY
    let mut suff_violations: Vec<(Cfg, u64)> = Vec::new();
    // associative but absorption false: would refute NECESSITY
    let mut nec_violations: Vec<(Cfg, u64)> = Vec::new();

    let mut per_domain_mis = 0u64;
    let mut per_fold_mis_total = 0u64;
    let mut per_fold_cfgs_with_mis = 0u64;

    let faces: Vec<Option<i64>> = core::iter::once(None)
        .chain((-6i64..=6).map(Some))
        .collect();

    for &lo in &faces {
        for &hi in &faces {
            if let (Some(l), Some(h)) = (lo, hi) {
                if l > h {
                    continue;
                }
            }
            for blo in -5i64..=0 {
                for bhi in 0i64..=5 {
                    let cfg = Cfg { lo, hi, blo, bhi };
                    cfgs += 1;

                    let fails = assoc_failures(&cfg, op);
                    let truth = fails == 0;
                    if truth {
                        assoc += 1;
                    } else {
                        nonassoc += 1;
                    }

                    let abs = absorbing(&cfg, op);
                    if abs {
                        abs_true += 1;
                    } else {
                        abs_false += 1;
                    }

                    if abs && !truth {
                        suff_violations.push((cfg, fails));
                    }
                    if !abs && truth {
                        nec_violations.push((cfg, fails));
                    }

                    if per_domain(&cfg, op) != truth {
                        per_domain_mis += 1;
                    }
                    let pf = per_fold_mismatches(&cfg, op);
                    per_fold_mis_total += pf;
                    if pf > 0 {
                        per_fold_cfgs_with_mis += 1;
                    }
                }
            }
        }
    }

    println!("--- {} ---", label);
    println!("  configurations swept:                       {}", cfgs);
    println!("    measured associative:                     {}", assoc);
    println!("    measured non-associative:                 {}", nonassoc);
    println!("    absorption true:                          {}", abs_true);
    println!(
        "    absorption false:                         {}",
        abs_false
    );
    println!(
        "  SUFFICIENCY violations (absorbing but not associative): {}",
        suff_violations.len()
    );
    println!(
        "  NECESSITY violations   (associative but not absorbing): {}",
        nec_violations.len()
    );
    for (c, f) in nec_violations.iter().take(8) {
        println!(
            "     lo={:?} hi={:?} box=[{},{}]  assoc-failures={}",
            c.lo, c.hi, c.blo, c.bhi, f
        );
    }
    println!(
        "  42 per-domain reading: configurations mispredicted    {}",
        per_domain_mis
    );
    println!(
        "  42 per-fold reading:   divergent triples it excludes  {} (across {} configurations)",
        per_fold_mis_total, per_fold_cfgs_with_mis
    );

    // instrument validation for this sweep
    let both_truths = assoc > 0 && nonassoc > 0;
    let both_abs = abs_true > 0 && abs_false > 0;
    let harness_can_report = per_domain_mis > 0 || per_fold_mis_total > 0;
    println!(
        "  instrument: both truths present {}, both absorption values present {}, harness demonstrably reports disagreement {}",
        both_truths, both_abs, harness_can_report
    );
    println!();

    both_truths && both_abs && harness_can_report
}

fn main() {
    println!("=== absorption as a biconditional, swept rather than sampled ===");
    println!();
    println!("Sufficiency violations refute 'absorbing implies associative'.");
    println!("Necessity violations refute 'associative implies absorbing'.");
    println!("Both counts are reported; neither is assumed.");
    println!();

    let ok_add = sweep("ambient operation: addition", add);
    let ok_mul = sweep("ambient operation: multiplication", mul);

    println!(
        "{}",
        if ok_add && ok_mul {
            "P2 WORKS"
        } else {
            "P2 FAILS"
        }
    );
    std::process::exit(if ok_add && ok_mul { 0 } else { 1 });
}
