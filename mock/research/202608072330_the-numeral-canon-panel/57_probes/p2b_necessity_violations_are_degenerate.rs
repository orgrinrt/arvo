//! Probe 2b: characterising `p2`'s 153 necessity violations rather than
//! waving at them.
//!
//! `p2` found absorption to be an exact biconditional for clamped ADDITION
//! (0 violations in either direction across 4248 configurations) and merely
//! sufficient for clamped MULTIPLICATION, with 153 configurations that are
//! associative without being absorbing. A finding that names a count and not a
//! mechanism is half a finding, so this probe asks what those 153 are.
//!
//! Hypothesis: they are degenerate collapses. When the clamp maps every
//! reachable product to a single value, the induced operation is CONSTANT on
//! the box, and a constant binary operation is associative for free, with no
//! absorption anywhere in sight. If that accounts for all 153, then the
//! qualification on the criterion is exactly "modulo operations the clamp has
//! collapsed", which is a boundary worth stating and not a hole.
//!
//! Reported: of the necessity violations, how many have a constant induced
//! operation, and any that do not are printed in full since those would be the
//! genuinely interesting residue.
//!
//! Instrument validation: the constancy checker is also run over the whole
//! sweep, and must report both constant and non-constant induced operations,
//! so a checker that answered "constant" unconditionally would be visible.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p2b p2b_necessity_violations_are_degenerate.rs && ./p2b

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
}

fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn assoc_ok(cfg: &Cfg) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            for c in cfg.blo..=cfg.bhi {
                let l = cfg.rho(mul(cfg.rho(mul(a, b)), c));
                let r = cfg.rho(mul(a, cfg.rho(mul(b, c))));
                if l != r {
                    return false;
                }
            }
        }
    }
    true
}

fn absorbing(cfg: &Cfg) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            let x = mul(a, b);
            for y in cfg.blo..=cfg.bhi {
                if cfg.rho(mul(cfg.rho(x), y)) != cfg.rho(mul(x, y)) {
                    return false;
                }
            }
        }
    }
    true
}

/// is the induced operation a # b = rho(a*b) constant over the box
fn induced_is_constant(cfg: &Cfg) -> bool {
    let first = cfg.rho(mul(cfg.blo, cfg.blo));
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            if cfg.rho(mul(a, b)) != first {
                return false;
            }
        }
    }
    true
}

fn main() {
    let faces: Vec<Option<i64>> = core::iter::once(None)
        .chain((-6i64..=6).map(Some))
        .collect();

    let mut violations = 0u64;
    let mut violations_constant = 0u64;
    let mut residue: Vec<Cfg> = Vec::new();

    let mut saw_constant = false;
    let mut saw_nonconstant = false;

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
                    let k = induced_is_constant(&cfg);
                    saw_constant |= k;
                    saw_nonconstant |= !k;

                    if assoc_ok(&cfg) && !absorbing(&cfg) {
                        violations += 1;
                        if k {
                            violations_constant += 1;
                        } else {
                            residue.push(cfg);
                        }
                    }
                }
            }
        }
    }

    println!(
        "necessity violations for clamped multiplication: {}",
        violations
    );
    println!(
        "  of which the induced operation is CONSTANT:    {}",
        violations_constant
    );
    println!(
        "  residue (associative, not absorbing, not constant): {}",
        residue.len()
    );
    for c in residue.iter().take(20) {
        println!("     lo={:?} hi={:?} box=[{},{}]", c.lo, c.hi, c.blo, c.bhi);
    }

    println!();
    println!(
        "instrument: constancy checker saw constant {} and non-constant {}",
        saw_constant, saw_nonconstant
    );

    let ok = saw_constant && saw_nonconstant && violations > 0;
    println!();
    println!(
        "verdict: absorption is necessary for clamped multiplication EXCEPT on collapsed \n\
         operations: {} of {} violations are constant induced operations.",
        violations_constant, violations
    );
    println!("{}", if ok { "P2B WORKS" } else { "P2B FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
