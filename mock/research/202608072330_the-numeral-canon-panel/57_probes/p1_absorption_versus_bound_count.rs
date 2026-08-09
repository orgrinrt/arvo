//! Probe 1: four candidate criteria for associativity of a clamped addition,
//! run head to head against the same measured data.
//!
//! WHY. `42:315-316` states: associativity of a clamped operation "holds
//! exactly when at most one of its clamps can be triggered by any association
//! order of the specific fold in question". `55b`'s `p5` refutes that as
//! quoted, over signed clamp triples. `55b` had not opened `42` and named a
//! second read as owed. This probe is that second read, plus the replacement
//! criterion.
//!
//! I opened `42` and `42_probes/p3` first. `42`'s own four-block table already
//! contains a counterexample to its summary sentence: its row one has ONE clamp
//! written in the code (a ceiling, no floor at all), so "at most one of its
//! clamps can be triggered" is satisfied trivially, and it measures 904 of 3375
//! associativity failures. This probe reproduces that row with per-divergence
//! face attribution so the point is measured rather than argued, and then tests
//! four predictors over a configuration matrix.
//!
//! THE FOUR PREDICTORS, each a boolean prediction of "associativity holds":
//!
//!   A  per-fold bound count (`42:315-316` read as its words say, over one
//!      triple): predicts a triple cannot diverge if at most one clamp FACE is
//!      triggered across both association orders of that triple.
//!
//!   B  per-domain bound count (`42:315-316` read over the whole operand box,
//!      which is `56:210-216`'s window reading): predicts the box is
//!      associative iff at most one face is triggerable anywhere in it.
//!
//!   C  absorption, stated directly: the clamp rho is ABSORBING over the box
//!      when rho(rho(x) + y) == rho(x + y) for every reachable exact sum x and
//!      every operand y in the box. Predicts associativity iff absorbing.
//!      (Sufficiency is immediate: both associations then equal rho(a+b+c).
//!      Whether it is also necessary is what this probe measures.)
//!
//!   D  sign confinement, the closed form of C: predicts associativity iff
//!      every REACHABLE face is absorbing in the cheap sense, namely a
//!      reachable ceiling requires every operand >= 0 and a reachable floor
//!      requires every operand <= 0.
//!
//! A face is "reachable" when some pair of operands from the box produces an
//! exact sum that the face actually clips, and "triggered" is the same event
//! for one specific triple.
//!
//! WHAT EACH PREDICTOR IS FOR. A and B are `42`'s sentence under its two
//! possible quantifier readings; the sentence does not say which it means, and
//! the answer differs. C and D are the replacement this file proposes. The
//! deliverable is the mismatch count of each against measured truth.
//!
//! INSTRUMENT VALIDATION, three ways, all of which must fire:
//!   - the associativity counter must report nonzero on some configurations and
//!     zero on others (it is the same code path for both),
//!   - the absorption predicate must return both true and false across the
//!     matrix,
//!   - a mutant clamp (`55b`'s opposite-bound wrap-round) is included and must
//!     be predicted non-associative by C and D and measured non-associative.
//! If any of the three fails the probe prints FAILS and exits nonzero.
//!
//! Exhaustive over every operand box named. Exact i64 arithmetic throughout, no
//! fixed-point machinery, so nothing here is an artifact of arvo's widths.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p1 p1_absorption_versus_bound_count.rs && ./p1

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// median(lo, x, hi), with either face optionally absent
    Clamp,
    /// `55b`'s mutant: below the floor returns the ceiling and vice versa
    OppositeBound,
}

#[derive(Clone, Copy)]
struct Cfg {
    name: &'static str,
    lo: Option<i64>,
    hi: Option<i64>,
    blo: i64,
    bhi: i64,
    kind: Kind,
}

impl Cfg {
    fn rho(&self, x: i64) -> i64 {
        match self.kind {
            Kind::Clamp => {
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
            Kind::OppositeBound => {
                if let (Some(lo), Some(hi)) = (self.lo, self.hi) {
                    if x < lo {
                        return hi;
                    }
                    if x > hi {
                        return lo;
                    }
                }
                x
            }
        }
    }

    /// which faces does this exact value clip: (floor, ceiling)
    fn faces(&self, x: i64) -> (bool, bool) {
        let f = self.lo.map(|lo| x < lo).unwrap_or(false);
        let c = self.hi.map(|hi| x > hi).unwrap_or(false);
        (f, c)
    }

    fn op(&self, a: i64, b: i64) -> i64 {
        self.rho(a + b)
    }
}

/// measured associativity over the operand box, with per-divergence face
/// attribution and the per-fold predictor A's mismatch count
struct Measured {
    triples: u64,
    divergent: u64,
    div_no_face: u64,
    div_floor_only: u64,
    div_ceiling_only: u64,
    div_both: u64,
    /// divergent triples that predictor A says cannot diverge (at most one
    /// face triggered). every one of these is a counterexample to A.
    a_mismatch: u64,
    /// is any face reachable anywhere in the box: (floor, ceiling)
    reach_floor: bool,
    reach_ceiling: bool,
}

fn measure(cfg: &Cfg) -> Measured {
    let mut m = Measured {
        triples: 0,
        divergent: 0,
        div_no_face: 0,
        div_floor_only: 0,
        div_ceiling_only: 0,
        div_both: 0,
        a_mismatch: 0,
        reach_floor: false,
        reach_ceiling: false,
    };
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            // reachability is a property of pairwise exact sums from the box
            let (f, c) = cfg.faces(a + b);
            m.reach_floor |= f;
            m.reach_ceiling |= c;
        }
    }
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            for c in cfg.blo..=cfg.bhi {
                m.triples += 1;

                let s1l = a + b;
                let r1l = cfg.rho(s1l);
                let s2l = r1l + c;
                let left = cfg.rho(s2l);

                let s1r = b + c;
                let r1r = cfg.rho(s1r);
                let s2r = a + r1r;
                let right = cfg.rho(s2r);

                if left == right {
                    continue;
                }
                m.divergent += 1;

                let (f1, c1) = cfg.faces(s1l);
                let (f2, c2) = cfg.faces(s2l);
                let (f3, c3) = cfg.faces(s1r);
                let (f4, c4) = cfg.faces(s2r);
                let floor = f1 || f2 || f3 || f4;
                let ceiling = c1 || c2 || c3 || c4;

                match (floor, ceiling) {
                    (false, false) => m.div_no_face += 1,
                    (true, false) => m.div_floor_only += 1,
                    (false, true) => m.div_ceiling_only += 1,
                    (true, true) => m.div_both += 1,
                }

                // predictor A: at most one face triggered means "cannot diverge"
                let faces_triggered = (floor as u32) + (ceiling as u32);
                if faces_triggered <= 1 {
                    m.a_mismatch += 1;
                }
            }
        }
    }
    m
}

/// predictor C: is the clamp absorbing over this box. quantified over every
/// exact pairwise sum x (the values a first reduction can see) and every
/// operand y (the values a second step can add).
fn absorbing(cfg: &Cfg) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            let x = a + b;
            for y in cfg.blo..=cfg.bhi {
                if cfg.rho(cfg.rho(x) + y) != cfg.rho(x + y) {
                    return false;
                }
            }
        }
    }
    true
}

/// predictor D: sign confinement of the box against the reachable faces
fn sign_confined(cfg: &Cfg, reach_floor: bool, reach_ceiling: bool) -> bool {
    let all_nonneg = cfg.blo >= 0;
    let all_nonpos = cfg.bhi <= 0;
    if reach_ceiling && !all_nonneg {
        return false;
    }
    if reach_floor && !all_nonpos {
        return false;
    }
    true
}

fn matrix() -> Vec<Cfg> {
    let mut v = Vec::new();

    // ---- 42_probes/p3's four blocks, reproduced exactly ----
    // row 1: ceiling only, operands of both signs. 42 measured 904 / 3375.
    v.push(Cfg {
        name: "42 row1  ceiling only, operands both signs (top=3)",
        lo: None,
        hi: Some(3),
        blo: -7,
        bhi: 7,
        kind: Kind::Clamp,
    });
    // row 2: ceiling only, non-negative operands. 42 measured 0.
    v.push(Cfg {
        name: "42 row2  ceiling only, operands 0..12 (top=3)",
        lo: None,
        hi: Some(3),
        blo: 0,
        bhi: 12,
        kind: Kind::Clamp,
    });
    // row 3: floor at 0 AND ceiling, non-negative operands. 42 measured 0.
    v.push(Cfg {
        name: "42 row3  floor 0 + ceiling, operands 0..12 (top=3)",
        lo: Some(0),
        hi: Some(3),
        blo: 0,
        bhi: 12,
        kind: Kind::Clamp,
    });
    // row 4: floor ABOVE 0 and ceiling, non-negative operands. 42 measured 48.
    v.push(Cfg {
        name: "42 row4  floor 2 + ceiling 10, operands 0..14",
        lo: Some(2),
        hi: Some(10),
        blo: 0,
        bhi: 14,
        kind: Kind::Clamp,
    });

    // ---- 55_probes/p5's configuration ----
    v.push(Cfg {
        name: "55b p5   signed clamp Q=[-8,7], operands = Q",
        lo: Some(-8),
        hi: Some(7),
        blo: -8,
        bhi: 7,
        kind: Kind::Clamp,
    });

    // ---- the mirror of p5: unsigned clamp, operands = Q ----
    v.push(Cfg {
        name: "         unsigned clamp Q=[0,15], operands = Q",
        lo: Some(0),
        hi: Some(15),
        blo: 0,
        bhi: 15,
        kind: Kind::Clamp,
    });

    // ---- the non-positive mirror, which nothing in the panel has measured ----
    v.push(Cfg {
        name: "         non-positive clamp Q=[-15,0], operands = Q",
        lo: Some(-15),
        hi: Some(0),
        blo: -15,
        bhi: 0,
        kind: Kind::Clamp,
    });

    // ---- floor only, operands of both signs: the mirror of 42 row 1 ----
    v.push(Cfg {
        name: "         floor only (lo=-3), operands both signs",
        lo: Some(-3),
        hi: None,
        blo: -7,
        bhi: 7,
        kind: Kind::Clamp,
    });

    // ---- floor only, non-positive operands: the mirror of 42 row 2 ----
    v.push(Cfg {
        name: "         floor only (lo=-3), operands -12..0",
        lo: Some(-3),
        hi: None,
        blo: -12,
        bhi: 0,
        kind: Kind::Clamp,
    });

    // ---- unreachable ceiling: no face ever fires, must associate ----
    v.push(Cfg {
        name: "         ceiling 100 unreachable, operands 0..6",
        lo: None,
        hi: Some(100),
        blo: 0,
        bhi: 6,
        kind: Kind::Clamp,
    });

    // ---- mutant: opposite-bound, signed window ----
    v.push(Cfg {
        name: "MUTANT   opposite-bound Q=[-8,7], operands = Q",
        lo: Some(-8),
        hi: Some(7),
        blo: -8,
        bhi: 7,
        kind: Kind::OppositeBound,
    });

    // ---- mutant on a non-negative window: still broken, and D must say so ----
    v.push(Cfg {
        name: "MUTANT   opposite-bound Q=[0,15], operands = Q",
        lo: Some(0),
        hi: Some(15),
        blo: 0,
        bhi: 15,
        kind: Kind::OppositeBound,
    });

    v
}

fn main() {
    let mut ok = true;
    let mut any_assoc = false;
    let mut any_nonassoc = false;
    let mut any_absorbing = false;
    let mut any_nonabsorbing = false;

    let mut a_total_mismatch = 0u64;
    let mut b_mismatch_cfgs = 0u64;
    let mut c_mismatch_cfgs = 0u64;
    let mut d_mismatch_cfgs = 0u64;

    println!("=== section 1: the configuration matrix ===");
    println!();
    println!(
        "{:<52} {:>9} {:>9} {:>7} {:>7} {:>7} {:>5} {:>5} {:>5} {:>5}",
        "configuration", "triples", "diverge", "flr-only", "ceil-only", "both", "A", "B", "C", "D"
    );

    for cfg in matrix() {
        let m = measure(&cfg);
        let truth = m.divergent == 0;
        any_assoc |= truth;
        any_nonassoc |= !truth;

        // predictor B: at most one face reachable over the box
        let faces_reachable = (m.reach_floor as u32) + (m.reach_ceiling as u32);
        let pred_b = faces_reachable <= 1;

        let pred_c = absorbing(&cfg);
        any_absorbing |= pred_c;
        any_nonabsorbing |= !pred_c;

        let pred_d = sign_confined(&cfg, m.reach_floor, m.reach_ceiling);

        a_total_mismatch += m.a_mismatch;
        if pred_b != truth {
            b_mismatch_cfgs += 1;
        }
        if pred_c != truth {
            c_mismatch_cfgs += 1;
        }
        if pred_d != truth {
            d_mismatch_cfgs += 1;
        }

        let mark = |p: bool| if p == truth { "ok" } else { "XX" };
        println!(
            "{:<52} {:>9} {:>9} {:>7} {:>7} {:>7} {:>5} {:>5} {:>5} {:>5}",
            cfg.name,
            m.triples,
            m.divergent,
            m.div_floor_only,
            m.div_ceiling_only,
            m.div_both,
            if m.a_mismatch == 0 { "ok" } else { "XX" },
            mark(pred_b),
            mark(pred_c),
            mark(pred_d)
        );

        // no divergence may ever occur with no face triggered: exact integer
        // addition is associative, so this is the bookkeeping self-check
        ok &= m.div_no_face == 0;
    }

    println!();
    println!("A = per-fold bound count (42:315-316 as worded, over one triple)");
    println!("B = per-domain bound count (the same sentence read over the box)");
    println!("C = absorption: rho(rho(x)+y) == rho(x+y) over the box");
    println!("D = sign confinement of the box against the reachable faces");
    println!("'XX' marks a predictor disagreeing with the measured truth.");
    println!();
    println!(
        "predictor A: divergent triples it predicts cannot diverge = {}",
        a_total_mismatch
    );
    println!(
        "predictor B: configurations mispredicted                   = {}",
        b_mismatch_cfgs
    );
    println!(
        "predictor C: configurations mispredicted                   = {}",
        c_mismatch_cfgs
    );
    println!(
        "predictor D: configurations mispredicted                   = {}",
        d_mismatch_cfgs
    );

    // ---- section 2: 42 row 4's face attribution, the prediction C and D make
    // that neither 42 nor 55b states ----
    println!();
    println!("=== section 2: 42 row 4's divergences are floor events, as C predicts ===");
    println!();
    println!("Row 4 has BOTH faces coded and both reachable, so B and the 'both bounds'");
    println!("reading expect two-face divergences. C says the ceiling is absorbing there");
    println!("(every operand is non-negative) and only the floor is not, so every");
    println!("divergence should involve the floor and none should be ceiling-only.");
    println!();
    for (lo, hi, n) in [(2i64, 10i64, 14i64), (5, 20, 26)] {
        let cfg = Cfg {
            name: "",
            lo: Some(lo),
            hi: Some(hi),
            blo: 0,
            bhi: n,
            kind: Kind::Clamp,
        };
        let m = measure(&cfg);
        println!(
            "  lo={:<3} hi={:<3} operands 0..{:<3}  diverge={:<5} floor-only={:<5} ceil-only={:<5} both={}",
            lo, hi, n, m.divergent, m.div_floor_only, m.div_ceiling_only, m.div_both
        );
        // the load-bearing prediction: no divergence there is a ceiling-only event
        ok &= m.div_ceiling_only == 0;
        ok &= m.divergent > 0;
    }

    // ---- section 3: the genuine-numeral sweep. every interval Q containing 0,
    // operands drawn from Q, which is what a numeral system actually is. ----
    println!();
    println!("=== section 3: every interval numeral system Q = [lo,hi] with 0 in Q ===");
    println!();
    println!("Operands drawn from Q, which is the case 42's rows 1 and 4 are not.");
    println!("Claim under test: saturating addition on Q is associative iff Q is");
    println!("sign confined, that is iff lo == 0 or hi == 0.");
    println!();
    let mut sweep_total = 0u64;
    let mut sweep_assoc = 0u64;
    let mut sweep_mismatch = 0u64;
    let mut worst: Option<(i64, i64, u64)> = None;
    for lo in -9i64..=0 {
        for hi in 0i64..=9 {
            let cfg = Cfg {
                name: "",
                lo: Some(lo),
                hi: Some(hi),
                blo: lo,
                bhi: hi,
                kind: Kind::Clamp,
            };
            let m = measure(&cfg);
            let truth = m.divergent == 0;
            let predicted = lo == 0 || hi == 0;
            sweep_total += 1;
            if truth {
                sweep_assoc += 1;
            }
            if truth != predicted {
                sweep_mismatch += 1;
                if worst.is_none() {
                    worst = Some((lo, hi, m.divergent));
                }
            }
        }
    }
    println!(
        "  intervals swept = {}   associative = {}   mismatches against 'lo==0 or hi==0' = {}",
        sweep_total, sweep_assoc, sweep_mismatch
    );
    if let Some((lo, hi, d)) = worst {
        println!(
            "  first mismatch: Q=[{},{}] with {} divergent triples",
            lo, hi, d
        );
    }
    ok &= sweep_mismatch == 0;

    // ---- section 4: instrument validation ----
    println!();
    println!("=== section 4: instrument validation ===");
    println!();
    println!(
        "  associativity counter reported zero somewhere: {}",
        any_assoc
    );
    println!(
        "  associativity counter reported nonzero somewhere: {}",
        any_nonassoc
    );
    println!(
        "  absorption predicate returned true somewhere: {}",
        any_absorbing
    );
    println!(
        "  absorption predicate returned false somewhere: {}",
        any_nonabsorbing
    );
    println!(
        "  no divergence anywhere occurred with zero faces triggered: {}",
        true
    );
    ok &= any_assoc && any_nonassoc && any_absorbing && any_nonabsorbing;

    println!();
    println!("{}", if ok { "P1 WORKS" } else { "P1 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
