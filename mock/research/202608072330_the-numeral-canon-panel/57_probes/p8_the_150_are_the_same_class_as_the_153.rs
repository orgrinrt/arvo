//! Probe 8: are `61`'s 150 the same class as `57`'s 153.
//!
//! WHY THIS EXISTS. The resumption brief carried a correction: "I reported to op
//! that absorption never mispredicts. On the widened sweep it mispredicts 150
//! times, against 356 and 587 for the two coherence readings." I re-ran
//! `61_probes/q1` byte-identically before taking any number from it, and read
//! its output: the 150 is `q1_output.txt:109`, which is the **multiplication**
//! row of the widened sweep. The addition rows are `q1_output.txt:29` and
//! `:83`, and both read `absorption 0`.
//!
//! So the correction is real and it is about multiplication, and `57` already
//! reported multiplication's mispredictions (153 on the 4248 sweep) and already
//! characterised them: `57_probes/p2b` found all 153 to be configurations whose
//! induced operation is CONSTANT, where associativity is free and absorption has
//! nothing to be necessary for. Residue zero.
//!
//! What was never checked is whether the WIDENED sweep's 150 are the same class.
//! `57_probes/p2b` only ever ran over `p2`'s own configuration space. If the 150
//! contain anything that is not a collapsed operation, then absorption has an
//! off-domain failure mode nobody has named, and the honest sentence about the
//! criterion changes. If they are all collapses, the characterisation transfers
//! and the criterion's boundary is exactly where `57` said it was.
//!
//! This probe reproduces `61_probes/q1`'s widened sweep parameters exactly,
//! read from its source rather than from its prose:
//!   faces: {absent} union [-4, 4]        (`q1:364-366`)
//!   blo:   -20..=0 step 2                (`q1:367`)
//!   bhi:   0..=20 step 2                 (`q1:368`)
//! and partitions absorption's mispredictions by whether the induced operation
//! is constant over the box.
//!
//! INSTRUMENT VALIDATION. The constancy checker must report both values over the
//! sweep, and the total misprediction count must reproduce `61`'s 150 exactly.
//! If it does not, this probe is measuring a different sweep than the one the
//! correction is about, and it says so and fails rather than reporting a number
//! that looks like an answer.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p8 p8_the_150_are_the_same_class_as_the_153.rs && ./p8

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
fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn assoc_ok(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            for c in cfg.blo..=cfg.bhi {
                if cfg.rho(op(cfg.rho(op(a, b)), c)) != cfg.rho(op(a, cfg.rho(op(b, c)))) {
                    return false;
                }
            }
        }
    }
    true
}

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

fn induced_is_constant(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    let first = cfg.rho(op(cfg.blo, cfg.blo));
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            if cfg.rho(op(a, b)) != first {
                return false;
            }
        }
    }
    true
}

fn run(label: &str, op: fn(i64, i64) -> i64, expected: Option<u64>) -> bool {
    let faces: Vec<Option<i64>> = core::iter::once(None)
        .chain((-4i64..=4).map(Some))
        .collect();
    let blos: Vec<i64> = (-20i64..=0).step_by(2).collect();
    let bhis: Vec<i64> = (0i64..=20).step_by(2).collect();

    let mut cfgs = 0u64;
    let mut mispredict = 0u64;
    let mut mispredict_constant = 0u64;
    let mut residue: Vec<Cfg> = Vec::new();
    let mut saw_const = false;
    let mut saw_nonconst = false;

    for &lo in &faces {
        for &hi in &faces {
            if let (Some(l), Some(h)) = (lo, hi) {
                if l > h {
                    continue;
                }
            }
            for &blo in &blos {
                for &bhi in &bhis {
                    let cfg = Cfg { lo, hi, blo, bhi };
                    cfgs += 1;
                    let k = induced_is_constant(&cfg, op);
                    saw_const |= k;
                    saw_nonconst |= !k;

                    let truth = assoc_ok(&cfg, op);
                    let abs = absorbing(&cfg, op);
                    if abs != truth {
                        mispredict += 1;
                        if k {
                            mispredict_constant += 1;
                        } else {
                            residue.push(cfg);
                        }
                    }
                }
            }
        }
    }

    println!("--- {} ---", label);
    println!(
        "  configurations swept:                             {}",
        cfgs
    );
    println!(
        "  absorption mispredictions:                        {}",
        mispredict
    );
    println!(
        "    of which the induced operation is CONSTANT:     {}",
        mispredict_constant
    );
    println!(
        "    residue (mispredicted and NOT constant):        {}",
        residue.len()
    );
    for c in residue.iter().take(10) {
        println!(
            "       lo={:?} hi={:?} box=[{},{}]",
            c.lo, c.hi, c.blo, c.bhi
        );
    }
    let matches_expected = match expected {
        None => true,
        Some(e) => {
            let m = mispredict == e;
            println!(
                "  reproduces `61_probes/q1`'s reported count of {}: {}",
                e, m
            );
            m
        }
    };
    println!(
        "  instrument: constancy checker saw constant {} and non-constant {}",
        saw_const, saw_nonconst
    );
    println!();
    matches_expected && saw_const && saw_nonconst && residue.is_empty()
}

fn main() {
    println!("=== absorption's off-domain mispredictions on `61`'s widened sweep ===");
    println!();
    println!("  Sweep parameters read from `61_probes/q1_absorption_versus_coherence.rs`");
    println!("  lines 364 to 368, not from its prose.");
    println!();

    // 61_probes/q1_output.txt:109 reports 150 for multiplication on the widened
    // sweep; :83 reports 0 for addition on the same sweep.
    let ok_mul = run("multiplication, widened box", mul, Some(150));
    let ok_add = run("addition, widened box", add, Some(0));

    println!("=== what this settles ===");
    println!();
    println!("  If the residue is zero for multiplication, the widened sweep's mispredictions");
    println!("  are the same collapsed-operation class `57_probes/p2b` isolated on the narrower");
    println!("  sweep, and the criterion's stated boundary needs no change. If addition's count");
    println!("  is zero here as it was there, absorption's addition result is exact on both");
    println!("  sweeps and the correction does not touch it.");

    let ok = ok_mul && ok_add;
    println!();
    println!("{}", if ok { "P8 WORKS" } else { "P8 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
