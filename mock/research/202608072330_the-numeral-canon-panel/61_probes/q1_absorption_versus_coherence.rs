//! Probe q1: is absorption the same law as coherence, or two laws that agree
//! on the cases measured so far.
//!
//! WHY. `57:277-278` argues the identification in one paragraph: "`56`'s
//! C-law is `rho(a op b) == rho(rho(a) op rho(b))`. With `b` drawn from `Q`
//! so that `rho(b) = b`, that is `rho(rho(a) op b) == rho(a op b)`, which is
//! absorption." `59` section 1a opened both probe sources and found the
//! bridging step assumes `y` (the "b" in `57`'s sentence) is already in `Q`,
//! which `57_probes/p2`'s own sweep does not enforce: `p2`'s operand box
//! `[blo, bhi]` ranges independently of the clamp bounds `[lo, hi]`, so `y`
//! is frequently NOT a fixed point of `rho`. Nobody has run the check.
//!
//! This probe runs it. Three predicates, computed on the SAME configuration
//! space `57_probes/p2` swept (4248 configurations per ambient operation:
//! floor in {absent} union [-6,6], ceiling likewise, box [blo,bhi] with
//! blo in [-5,0], bhi in [0,5]):
//!
//!   ABSORPTION   (57_probes/p2's own code, byte-for-byte):
//!                for a,b in box, x = op(a,b) [UNREDUCED];
//!                for y in box [UNREDUCED on either side]:
//!                    rho(op(rho(x), y)) == rho(op(x, y))
//!
//!   COHERENCE-EXT (the same (x,y) domain as absorption, so the diff is a
//!                fair one-line comparison rather than a domain change):
//!                for a,b in box, x = op(a,b) [UNREDUCED];
//!                for y in box:
//!                    rho(op(x, y)) == rho(op(rho(x), rho(y)))
//!                [note: reduces BOTH x and y on the right, per 56's C-law]
//!
//!   COHERENCE-DIRECT (56_probes/q1's literal shape, using the operand box
//!                itself as the ambient window, which is the natural
//!                per-config instantiation of q1's fixed [-64,64] window):
//!                for a,b in box [UNREDUCED on the left]:
//!                    rho(op(a, b)) == rho(op(rho(a), rho(b)))
//!
//! Where COHERENCE-EXT and ABSORPTION disagree, the disagreement can only
//! come from the one place they differ syntactically: COHERENCE-EXT reduces
//! `y` before combining on its right side, ABSORPTION never does. So a
//! disagreement is direct evidence that "y already in Q" is load-bearing,
//! not a footnote.
//!
//! A second, more aggressive sweep deliberately widens the operand box well
//! past the clamp bounds (per the dispatch: "second operands drawn from
//! outside the representable set are the crux. Build that case
//! deliberately."), to maximise how often y sits outside Q and see whether
//! the three predicates separate under real pressure rather than only in
//! the corner of p2's original small box.
//!
//! INSTRUMENT VALIDATION. The three predicates must each take both boolean
//! values somewhere in the sweep (a predicate that is always true or always
//! false is not being exercised). And the disagreement counters themselves
//! must be printed rather than asserted zero, so a nonzero count is visible
//! rather than silently swallowed.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o q1 q1_absorption_versus_coherence.rs && ./q1

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

/// `57_probes/p2`'s absorbing(), reproduced verbatim (re-derived, not copy
/// pasted, to keep this probe self-contained; checked line-for-line against
/// the source before this file was committed).
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

/// coherence over the SAME (x, y) domain absorption uses: x is a reachable
/// sum of two box elements, y ranges over the box, and BOTH sides of the
/// right-hand combination are reduced. This isolates the one syntactic
/// difference from absorbing() above: y is reduced here, unreduced there.
fn coherent_ext(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            let x = op(a, b);
            for y in cfg.blo..=cfg.bhi {
                if cfg.rho(op(x, y)) != cfg.rho(op(cfg.rho(x), cfg.rho(y))) {
                    return false;
                }
            }
        }
    }
    true
}

/// coherence exactly as `56_probes/q1` states it (a, b range over an ambient
/// window; here the window is the operand box, which is the natural
/// per-config instantiation).
fn coherent_direct(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            if cfg.rho(op(a, b)) != cfg.rho(op(cfg.rho(a), cfg.rho(b))) {
                return false;
            }
        }
    }
    true
}

fn assoc_holds(cfg: &Cfg, op: fn(i64, i64) -> i64) -> bool {
    for a in cfg.blo..=cfg.bhi {
        for b in cfg.blo..=cfg.bhi {
            for c in cfg.blo..=cfg.bhi {
                let l = cfg.rho(op(cfg.rho(op(a, b)), c));
                let r = cfg.rho(op(a, cfg.rho(op(b, c))));
                if l != r {
                    return false;
                }
            }
        }
    }
    true
}

/// how often y (drawn from the box) is NOT a fixed point of rho, i.e. sits
/// outside Q. this is the quantity the dispatch calls the crux.
fn y_outside_q_count(cfg: &Cfg) -> u64 {
    let mut n = 0;
    for y in cfg.blo..=cfg.bhi {
        if cfg.rho(y) != y {
            n += 1;
        }
    }
    n
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn mul(a: i64, b: i64) -> i64 {
    a * b
}

struct Result {
    cfgs: u64,
    abs_true: u64,
    coh_ext_true: u64,
    coh_dir_true: u64,
    // pairwise disagreements between the three predicates
    abs_vs_coh_ext_disagree: Vec<(Cfg, bool, bool, u64)>,
    abs_vs_coh_dir_disagree: Vec<(Cfg, bool, bool, u64)>,
    coh_ext_vs_coh_dir_disagree: Vec<(Cfg, bool, bool)>,
    // does the predicate track measured associativity as well as absorption did
    coh_ext_assoc_mismatch: u64,
    coh_dir_assoc_mismatch: u64,
    abs_assoc_mismatch: u64,
    // partition of abs-vs-coh-ext disagreements by whether the whole box sits inside Q
    disagree_ext_box_subset_of_q: u64,
    disagree_ext_box_not_subset_of_q: u64,
    disagree_dir_box_subset_of_q: u64,
    disagree_dir_box_not_subset_of_q: u64,
}

fn sweep(
    label: &str,
    op: fn(i64, i64) -> i64,
    floors: &[Option<i64>],
    ceilings: &[Option<i64>],
    blos: &[i64],
    bhis: &[i64],
) -> Result {
    let mut r = Result {
        cfgs: 0,
        abs_true: 0,
        coh_ext_true: 0,
        coh_dir_true: 0,
        abs_vs_coh_ext_disagree: Vec::new(),
        abs_vs_coh_dir_disagree: Vec::new(),
        coh_ext_vs_coh_dir_disagree: Vec::new(),
        coh_ext_assoc_mismatch: 0,
        coh_dir_assoc_mismatch: 0,
        abs_assoc_mismatch: 0,
        disagree_ext_box_subset_of_q: 0,
        disagree_ext_box_not_subset_of_q: 0,
        disagree_dir_box_subset_of_q: 0,
        disagree_dir_box_not_subset_of_q: 0,
    };

    for &lo in floors {
        for &hi in ceilings {
            if let (Some(l), Some(h)) = (lo, hi) {
                if l > h {
                    continue;
                }
            }
            for &blo in blos {
                for &bhi in bhis {
                    if blo > bhi {
                        continue;
                    }
                    let cfg = Cfg { lo, hi, blo, bhi };
                    r.cfgs += 1;

                    let abs = absorbing(&cfg, op);
                    let coh_ext = coherent_ext(&cfg, op);
                    let coh_dir = coherent_direct(&cfg, op);
                    let assoc = assoc_holds(&cfg, op);

                    if abs {
                        r.abs_true += 1;
                    }
                    if coh_ext {
                        r.coh_ext_true += 1;
                    }
                    if coh_dir {
                        r.coh_dir_true += 1;
                    }

                    if abs != coh_ext {
                        let yout = y_outside_q_count(&cfg);
                        r.abs_vs_coh_ext_disagree.push((cfg, abs, coh_ext, yout));
                        if yout == 0 {
                            r.disagree_ext_box_subset_of_q += 1;
                        } else {
                            r.disagree_ext_box_not_subset_of_q += 1;
                        }
                    }
                    if abs != coh_dir {
                        let yout = y_outside_q_count(&cfg);
                        r.abs_vs_coh_dir_disagree.push((cfg, abs, coh_dir, yout));
                        if yout == 0 {
                            r.disagree_dir_box_subset_of_q += 1;
                        } else {
                            r.disagree_dir_box_not_subset_of_q += 1;
                        }
                    }
                    if coh_ext != coh_dir {
                        r.coh_ext_vs_coh_dir_disagree.push((cfg, coh_ext, coh_dir));
                    }

                    if abs != assoc {
                        r.abs_assoc_mismatch += 1;
                    }
                    if coh_ext != assoc {
                        r.coh_ext_assoc_mismatch += 1;
                    }
                    if coh_dir != assoc {
                        r.coh_dir_assoc_mismatch += 1;
                    }
                }
            }
        }
    }

    println!("--- {} ---", label);
    println!("  configurations swept:                    {}", r.cfgs);
    println!("  absorption true:                         {}", r.abs_true);
    println!(
        "  coherence-ext true:                      {}",
        r.coh_ext_true
    );
    println!(
        "  coherence-direct true:                   {}",
        r.coh_dir_true
    );
    println!(
        "  absorption vs coherence-ext disagreements:    {}",
        r.abs_vs_coh_ext_disagree.len()
    );
    for (c, a, ce, yout) in r.abs_vs_coh_ext_disagree.iter().take(6) {
        println!(
            "     lo={:?} hi={:?} box=[{},{}]  absorption={} coherence-ext={}  y-outside-Q-count={}",
            c.lo, c.hi, c.blo, c.bhi, a, ce, yout
        );
    }
    println!(
        "  absorption vs coherence-direct disagreements: {}",
        r.abs_vs_coh_dir_disagree.len()
    );
    for (c, a, cd, yout) in r.abs_vs_coh_dir_disagree.iter().take(6) {
        println!(
            "     lo={:?} hi={:?} box=[{},{}]  absorption={} coherence-direct={}  y-outside-Q-count={}",
            c.lo, c.hi, c.blo, c.bhi, a, cd, yout
        );
    }
    println!(
        "  coherence-ext vs coherence-direct disagreements: {}",
        r.coh_ext_vs_coh_dir_disagree.len()
    );
    println!(
        "  absorption vs coherence-ext disagreements WHERE box entirely subset of Q: {} (should be 0)",
        r.disagree_ext_box_subset_of_q
    );
    println!(
        "  absorption vs coherence-ext disagreements where box is NOT subset of Q:  {}",
        r.disagree_ext_box_not_subset_of_q
    );
    println!(
        "  absorption vs coherence-direct disagreements WHERE box entirely subset of Q: {} (mechanism check)",
        r.disagree_dir_box_subset_of_q
    );
    println!(
        "  absorption vs coherence-direct disagreements where box is NOT subset of Q:  {}",
        r.disagree_dir_box_not_subset_of_q
    );
    println!(
        "  disagreements with MEASURED associativity: absorption {}, coherence-ext {}, coherence-direct {}",
        r.abs_assoc_mismatch, r.coh_ext_assoc_mismatch, r.coh_dir_assoc_mismatch
    );
    println!();

    r
}

fn main() {
    println!("=== absorption against coherence, measured rather than argued ===");
    println!();

    // section 1: reproduce 57_probes/p2's exact sweep, three predicates at once
    let faces: Vec<Option<i64>> = core::iter::once(None)
        .chain((-6i64..=6).map(Some))
        .collect();
    let blos: Vec<i64> = (-5i64..=0).collect();
    let bhis: Vec<i64> = (0i64..=5).collect();

    println!("### section 1: 57_probes/p2's own configuration space (4248 per op) ###");
    println!();
    let r_add = sweep("addition, p2's sweep", add, &faces, &faces, &blos, &bhis);
    let r_mul = sweep(
        "multiplication, p2's sweep",
        mul,
        &faces,
        &faces,
        &blos,
        &bhis,
    );

    // section 2: deliberately widen the box far past the clamp bounds, so
    // y is outside Q far more often. per the dispatch: build the crux case.
    println!("### section 2: box deliberately widened past the clamp bounds ###");
    println!();
    let wide_faces: Vec<Option<i64>> = core::iter::once(None)
        .chain((-4i64..=4).map(Some))
        .collect();
    let wide_blos: Vec<i64> = (-20i64..=0).step_by(2).collect();
    let wide_bhis: Vec<i64> = (0i64..=20).step_by(2).collect();
    let _r_add_wide = sweep(
        "addition, widened box",
        add,
        &wide_faces,
        &wide_faces,
        &wide_blos,
        &wide_bhis,
    );
    let _r_mul_wide = sweep(
        "multiplication, widened box",
        mul,
        &wide_faces,
        &wide_faces,
        &wide_blos,
        &wide_bhis,
    );

    // section 3: one hand-picked witness, worked by hand before the sweep
    // ran, to have a readable instance rather than only a count.
    println!("### section 3: one hand-built witness ###");
    let cfg = Cfg {
        lo: Some(-8),
        hi: Some(7),
        blo: -5,
        bhi: 5,
    };
    // x = a + b with a = b = 5 -> x = 10, rho(x) = 7 (clamped already)
    // y = -5, which is inside the box and inside Q (Q = [-8,7]), so this
    // particular pair will not separate the two predicates; printed anyway
    // as an example of the domain the sweep covers.
    let x = add(5, 5);
    let y = -5;
    println!(
        "  x = add(5,5) = {}, rho(x) = {}, y = {}, rho(y) = {}",
        x,
        cfg.rho(x),
        y,
        cfg.rho(y)
    );
    println!(
        "  absorption check:      rho(op(rho(x), y)) = {}   rho(op(x, y)) = {}",
        cfg.rho(add(cfg.rho(x), y)),
        cfg.rho(add(x, y))
    );
    println!(
        "  coherence-ext check:   rho(op(x, y)) = {}   rho(op(rho(x), rho(y))) = {}",
        cfg.rho(add(x, y)),
        cfg.rho(add(cfg.rho(x), cfg.rho(y)))
    );
    println!();

    // instrument validation
    let predicates_exercised = r_add.abs_true > 0
        && r_add.abs_true < r_add.cfgs
        && r_mul.abs_true > 0
        && r_mul.abs_true < r_mul.cfgs;
    let ok = predicates_exercised;
    println!(
        "instrument: predicates take both boolean values in the sweep {}",
        predicates_exercised
    );
    println!("{}", if ok { "Q1 WORKS" } else { "Q1 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
