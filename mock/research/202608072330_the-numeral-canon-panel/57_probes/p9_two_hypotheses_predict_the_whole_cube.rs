//! Probe 9: two hypotheses, evaluated mechanically, against every cell of
//! `62`'s cube.
//!
//! WHAT IS BEING CLAIMED. `57`'s sufficiency argument has two hypotheses, and
//! `57` stated the second and under-emphasised the first:
//!
//!   H1  the AMBIENT operation is associative on the reachable set;
//!   H2  the reduction's kernel relation is a CONGRUENCE for that operation
//!       (equivalently, on the domain `61` delimited, the reduction is
//!       absorbing).
//!
//! Given both, the induced operation on `Q` is associative, because `Q` is then
//! the quotient and inherits associativity from the ambient. The claim this
//! probe tests is that H1 and H2 together are not merely sufficient but
//! **explain the whole of `62` section 2's cube**: every cell's measured verdict
//! is predicted by evaluating the two hypotheses for that cell, with no cell
//! needing a mechanism of its own.
//!
//! If it holds, the unit's findings are one frame rather than a table, and the
//! two failure modes are separable:
//!   H2 fails: the reduction is not a congruence. Decided by the range geometry,
//!             per operation (`57_probes/p7`). FIXABLE by choosing the range,
//!             which is `62`'s symmetric-clamp result.
//!   H1 fails: the ambient operation is itself non-associative. That is what the
//!             rescale does at `F > 0` (`57_probes/p4` section 1 and
//!             `62_probes/p2` section 3, both with no reduction present). NOT
//!             fixable by any property of the reduction, because there is
//!             nothing associative left to inherit from.
//!
//! That predicts, without measuring the cell first, that `62`'s symmetric clamp
//! restores multiplicative associativity at `F = 0` and buys nothing at `F > 0`,
//! which is what `62:42-44` reports.
//!
//! THE NINE CELLS, from `62:162-172`, each evaluated here rather than quoted.
//! Policies: saturation into `Q`, and two's-complement wrap. Sign domains:
//! unsigned `[0, M]`, signed two's complement, signed symmetric. Operations:
//! addition and multiplication. Scales: `F = 0` and `F = 2`.
//!
//! HOW EACH HYPOTHESIS IS EVALUATED, mechanically, with no per-cell special
//! casing:
//!   H1: is the ambient operation associative over the reachable ambient, with
//!       NO reduction applied anywhere.
//!   H2: is the kernel of the reduction a congruence for the ambient operation
//!       over that same reachable ambient.
//!   truth: is the induced operation on `Q` associative, exhaustively over
//!       `Q` cubed.
//! The prediction is `H1 && H2 => truth`. Both directions are reported: a cell
//! where both hold and the induced operation is NOT associative would refute the
//! sufficiency argument outright, and a cell where the induced operation is
//! associative without both holding is a necessity gap, which is reported and
//! characterised rather than hidden.
//!
//! INSTRUMENT VALIDATION. Each of H1, H2 and truth must take both values across
//! the cube, or the corresponding checker is not discriminating and the probe
//! says so and fails.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p9 p9_two_hypotheses_predict_the_whole_cube.rs && ./p9

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Saturate,
    Wrap,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Copy)]
struct Cell {
    name: &'static str,
    policy: Policy,
    op: Op,
    lo: i64,
    hi: i64,
    f: u32,
}

impl Cell {
    /// the ambient (exact) operation at this scale, before any reduction
    fn ambient(&self, a: i64, b: i64) -> i64 {
        match self.op {
            Op::Add => a + b,
            Op::Mul => (a * b) >> self.f,
        }
    }
    fn reduce(&self, x: i64) -> i64 {
        match self.policy {
            Policy::Saturate => x.clamp(self.lo, self.hi),
            Policy::Wrap => {
                let m = self.hi - self.lo + 1;
                (x - self.lo).rem_euclid(m) + self.lo
            }
        }
    }
    /// Q together with the image of Q x Q under the ambient operation, which is
    /// what one exact step actually reaches. Same domain discipline `p7` had to
    /// be corrected into.
    fn reachable(&self) -> (i64, i64) {
        let (mut alo, mut ahi) = (self.lo, self.hi);
        for a in self.lo..=self.hi {
            for b in self.lo..=self.hi {
                let v = self.ambient(a, b);
                alo = alo.min(v);
                ahi = ahi.max(v);
            }
        }
        (alo, ahi)
    }
}

/// H1: the ambient operation is associative over the reachable set, no
/// reduction applied anywhere
fn h1_ambient_associative(c: &Cell) -> bool {
    let (alo, ahi) = c.reachable();
    for a in alo..=ahi {
        for b in alo..=ahi {
            for x in alo..=ahi {
                if c.ambient(c.ambient(a, b), x) != c.ambient(a, c.ambient(b, x)) {
                    return false;
                }
            }
        }
    }
    true
}

/// H2: the kernel of the reduction is a congruence for the ambient operation
fn h2_kernel_is_congruence(c: &Cell) -> bool {
    let (alo, ahi) = c.reachable();
    let span = (c.hi - c.lo + 1) as usize;
    let idx = |v: i64| (v - c.lo) as usize;
    let mut seen: Vec<Option<i64>> = vec![None; span * span];
    for x in alo..=ahi {
        let cx = c.reduce(x);
        for y in alo..=ahi {
            let cy = c.reduce(y);
            let r = c.reduce(c.ambient(x, y));
            let slot = &mut seen[idx(cx) * span + idx(cy)];
            match slot {
                None => *slot = Some(r),
                Some(r0) => {
                    if *r0 != r {
                        return false;
                    }
                }
            }
        }
    }
    true
}

/// the measured truth: is the induced operation on Q associative
fn induced_associative(c: &Cell) -> bool {
    for a in c.lo..=c.hi {
        for b in c.lo..=c.hi {
            for x in c.lo..=c.hi {
                let l = c.reduce(c.ambient(c.reduce(c.ambient(a, b)), x));
                let r = c.reduce(c.ambient(a, c.reduce(c.ambient(b, x))));
                if l != r {
                    return false;
                }
            }
        }
    }
    true
}

fn induced_is_constant(c: &Cell) -> bool {
    let first = c.reduce(c.ambient(c.lo, c.lo));
    for a in c.lo..=c.hi {
        for b in c.lo..=c.hi {
            if c.reduce(c.ambient(a, b)) != first {
                return false;
            }
        }
    }
    true
}

fn cube() -> Vec<Cell> {
    // width 4: unsigned [0,15], two's complement [-8,7], symmetric [-7,7]
    let mut v = Vec::new();
    for &f in &[0u32, 2] {
        for &(dom, lo, hi) in &[
            ("unsigned    ", 0i64, 15i64),
            ("signed 2c   ", -8, 7),
            ("signed sym  ", -7, 7),
        ] {
            for &(pn, policy) in &[("sat ", Policy::Saturate), ("wrap", Policy::Wrap)] {
                for &(on, op) in &[("add", Op::Add), ("mul", Op::Mul)] {
                    let name: &'static str =
                        Box::leak(format!("{} {} {} F={}", dom, pn, on, f).into_boxed_str());
                    v.push(Cell {
                        name,
                        policy,
                        op,
                        lo,
                        hi,
                        f,
                    });
                }
            }
        }
    }
    v
}

fn main() {
    let mut ok = true;
    let mut saw_h1 = (false, false);
    let mut saw_h2 = (false, false);
    let mut saw_truth = (false, false);

    let mut suff_violations = 0u64;
    let mut nec_violations = 0u64;
    let mut nec_constant = 0u64;
    let mut nec_residue: Vec<&'static str> = Vec::new();

    println!("=== every cell of the cube, with both hypotheses evaluated mechanically ===");
    println!();
    println!(
        "{:<28} {:>4} {:>4} {:>8} {:>10} {:>10}",
        "cell", "H1", "H2", "induced", "predicted", "verdict"
    );

    for c in cube() {
        let h1 = h1_ambient_associative(&c);
        let h2 = h2_kernel_is_congruence(&c);
        let truth = induced_associative(&c);
        let predicted = h1 && h2;

        if h1 {
            saw_h1.0 = true
        } else {
            saw_h1.1 = true
        }
        if h2 {
            saw_h2.0 = true
        } else {
            saw_h2.1 = true
        }
        if truth {
            saw_truth.0 = true
        } else {
            saw_truth.1 = true
        }

        let verdict = if predicted && !truth {
            suff_violations += 1;
            "REFUTES"
        } else if !predicted && truth {
            nec_violations += 1;
            if induced_is_constant(&c) {
                nec_constant += 1;
                "assoc, collapsed"
            } else {
                nec_residue.push(c.name);
                "assoc, RESIDUE"
            }
        } else {
            "predicted"
        };

        println!(
            "{:<28} {:>4} {:>4} {:>8} {:>10} {:>10}",
            c.name,
            if h1 { "yes" } else { "no" },
            if h2 { "yes" } else { "no" },
            if truth { "assoc" } else { "broken" },
            if predicted { "assoc" } else { "broken" },
            verdict
        );
    }

    println!();
    println!("=== what the cube says ===");
    println!();
    println!(
        "  SUFFICIENCY violations (H1 and H2 hold, induced not associative): {}",
        suff_violations
    );
    println!(
        "  cells associative without both hypotheses:                        {}",
        nec_violations
    );
    println!(
        "    of which the induced operation is collapsed to a constant:      {}",
        nec_constant
    );
    println!(
        "    residue:                                                        {}",
        nec_residue.len()
    );
    for n in &nec_residue {
        println!("       {}", n);
    }
    ok &= suff_violations == 0;
    ok &= nec_residue.is_empty();

    println!();
    println!("  Reading. H2 is the fixable one: it is decided by the range geometry per");
    println!("  operation, which is why a symmetric range restores signed multiplication at");
    println!("  F = 0. H1 is the one nothing about the reduction can repair, which is why the");
    println!("  same symmetric range buys nothing at F > 0: the rescaled ambient is not");
    println!("  associative, so there is no associativity left to inherit.");

    println!();
    println!("=== instrument validation ===");
    println!();
    println!(
        "  H1 observed both true and false:    {}",
        saw_h1.0 && saw_h1.1
    );
    println!(
        "  H2 observed both true and false:    {}",
        saw_h2.0 && saw_h2.1
    );
    println!(
        "  induced observed both true and false: {}",
        saw_truth.0 && saw_truth.1
    );
    println!("  (a hypothesis that never varied would predict by accident)");
    ok &= saw_h1.0 && saw_h1.1 && saw_h2.0 && saw_h2.1 && saw_truth.0 && saw_truth.1;

    println!();
    println!("{}", if ok { "P9 WORKS" } else { "P9 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
