// a6: separating the two repairs 221 applied at once to the inclusion order.
//
// `221` answers `question::inclusion_order_singleton_amendment` with option 1, decide inclusion
// on denotation rather than on declaration. Its evidence is that its C1 control, requiring a
// unique meet inside the constant-quantum family, failed at 256 of 630 pairs on the first run
// and passes on the repaired one, and its C5 arm re-runs C1 with the degenerate points removed
// and reports 256 again.
//
// **Two repairs went in together and only one of them was ever varied.** Its
// `catalogue_with(degenerates: bool)` makes admitting the bottom a parameter, and C5 varies it.
// The deduplication by denotation sits unconditionally in the same function and is never varied
// by anything. Its own sentence is "the deduplication is what makes C1 pass", and its own
// diagnosis of the 256 is that "every one of them has an EMPTY intersection, zero have several
// maximal lower bounds", which is the bottom being absent rather than duplicates being present.
// So the sentence names a repair the instrument did not test, and the diagnosis names the other.
//
// This probe varies both, over the same catalogue, and reports the 2x2.
//
// PROVENANCE. The catalogue construction, the shape type, the `constant` and `exponential`
// generators, the degenerate points, `maximal`, `minimal` and `pairs` are copied verbatim from
// `221_probes/p2_joins_exist_meets_do_not_core.rs`, lines 63 to 270, so the comparison is
// against that instrument rather than against a reimplementation of it. What I changed is one
// thing: deduplication is a parameter rather than unconditional. Everything below the copy is
// mine.
//
// PREDICTIONS, stated before running:
//
// P1. The two cells 221 already ran reproduce exactly: dedup on with degenerates on gives 0,
//     dedup on with degenerates off gives 256. If either misses, this copy is not its
//     instrument and nothing else here counts. **This is the control that matters most.**
// P2. Dedup off with degenerates on gives a nonzero count.
// P3. Every one of those failures has all its maximal lower bounds denoting one set, which
//     would make them a preorder artefact rather than an order failure.
//
// P3 WAS WRONG AND THE FIRST RUN SAID SO, which is why the split below has four columns rather
// than three. That run is kept as `a6_v1_the_split_conflated_no_bounds_with_annihilated_bounds.txt`.
// It reported all 28 dedup-off failures as "empty intersections", which cannot be right with the
// bottom present, since the empty-valued degenerate point is a lower bound of every pair. The
// cause is in the copied helper rather than in the catalogue: `maximal` keeps `i` when no `j`
// has `c[i].vals` a subset of `c[j].vals`, and two indices denoting ONE set are each a subset of
// the other, so **both are dropped and the maximal set comes back empty**. My split then read
// that as "no lower bounds".
//
// So there are three mechanisms here, not two, and the third is the interesting one:
//   (a) the bottom being absent, which is what the 256 are;
//   (b) antisymmetry being absent, which is what deciding inclusion on denotation supplies;
//   (c) `maximal` and `minimal` being defined only on a partial order, so running the copied
//       instrument without deduplication does not run the same measurement at all.
// The four columns below tell them apart.
//
// THE CASE THAT MUST FAIL. P1, in both directions: a cell that does not reproduce 221's number,
// and a cell that reproduces it when it should not. Both are printed.
//
// Build: rustc --edition 2024 -O.

use std::collections::{BTreeMap, BTreeSet};

type Set = BTreeSet<i64>;

#[derive(Clone)]
struct Shape {
    name: String,
    kind: Kind,
    /// The phase coordinate, in sixteenths. Zero for every exponential point by
    /// construction, since a float grid is anchored at zero.
    phase: i64,
    vals: Set,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Constant,
    Exponential,
}

/// Constant quantum: `phase + k*step`, symmetric reach.
fn constant(step: i64, half: i64, phase: i64) -> Shape {
    let mut vals = Set::new();
    let mut k = -(half / step) - 1;
    while k * step + phase <= half {
        let v = k * step + phase;
        if v >= -half {
            vals.insert(v);
        }
        k += 1;
    }
    Shape {
        name: format!("const step={step}/16 half={half}/16 phase={phase}/16"),
        kind: Kind::Constant,
        phase,
        vals,
    }
}

/// Exponential quantum: sign-symmetric `m * 2^e` with `m` in `[2^(p-1), 2^p)`,
/// plus subnormals at `emin`, plus zero. Scaled by 16, exact by construction:
/// every emitted value is checked to be an integer number of sixteenths.
fn exponential(p: u32, emin: i32, emax: i32, half: i64) -> Shape {
    let mut vals = Set::new();
    vals.insert(0);
    let lo = 1i64 << (p - 1);
    let hi = 1i64 << p;
    for e in emin..=emax {
        for m in lo..hi {
            // value = m * 2^(e - (p-1)); scaled by 16.
            let shift = e - (p as i32 - 1);
            let num = m * 16;
            let v = if shift >= 0 {
                num << shift
            } else {
                let d = 1i64 << (-shift);
                if num % d != 0 {
                    continue;
                }
                num / d
            };
            if v.abs() <= half {
                vals.insert(v);
                vals.insert(-v);
            }
        }
    }
    // subnormals at emin: m in 1..lo
    for m in 1..lo {
        let shift = emin - (p as i32 - 1);
        let num = m * 16;
        let v = if shift >= 0 {
            num << shift
        } else {
            let d = 1i64 << (-shift);
            if num % d != 0 {
                continue;
            }
            num / d
        };
        if v.abs() <= half {
            vals.insert(v);
            vals.insert(-v);
        }
    }
    Shape {
        name: format!("float p={p} e=[{emin},{emax}] half={half}/16"),
        kind: Kind::Exponential,
        phase: 0,
        vals,
    }
}

/// The degenerate points: a grid with no values and a grid with one.
///
/// Points of the same parameterisation with the reach cut to nothing, and the
/// bottom of the inclusion order. Excluded in v1, which is what made C1 fail.
fn degenerate() -> Vec<Shape> {
    vec![
        Shape {
            name: "degenerate: no values".into(),
            kind: Kind::Constant,
            phase: 0,
            vals: Set::new(),
        },
        Shape {
            name: "degenerate: one value {0}".into(),
            kind: Kind::Constant,
            phase: 0,
            vals: [0i64].into_iter().collect(),
        },
    ]
}

fn catalogue_with(degenerates: bool) -> Vec<Shape> {
    let mut out = Vec::new();
    if degenerates {
        out.extend(degenerate());
    }
    for &step in &[1i64, 2, 4, 8, 16] {
        for &half in &[16i64, 32, 64, 128] {
            for &phase in &[0i64, step / 2] {
                if phase != 0 && step == 1 {
                    continue; // half a sixteenth is off the domain's own grid
                }
                let s = constant(step, half, phase);
                if !s.vals.is_empty() {
                    out.push(s);
                }
            }
        }
    }
    for &p in &[2u32, 3] {
        for &(emin, emax) in &[(-3i32, 2i32), (-2, 3), (-4, 1)] {
            for &half in &[32i64, 128] {
                let s = exponential(p, emin, emax, half);
                if s.vals.len() > 3 {
                    out.push(s);
                }
            }
        }
    }
    // Deduplicate by denotation: two declarations denoting one set are one
    // shape in this order, which is the amendment the singleton row asks about.
    let mut seen: BTreeMap<Set, usize> = BTreeMap::new();
    let mut uniq = Vec::new();
    for s in out {
        if !seen.contains_key(&s.vals) {
            seen.insert(s.vals.clone(), uniq.len());
            uniq.push(s);
        }
    }
    uniq
}

fn catalogue() -> Vec<Shape> {
    catalogue_with(true)
}

/// C1 recomputed over an arbitrary catalogue, so C5 can run the same arm twice.
fn constant_meet_failures(c: &[Shape]) -> usize {
    let consts: Vec<usize> = (0..c.len())
        .filter(|&i| c[i].kind == Kind::Constant)
        .collect();
    let mut bad = 0;
    for (a, b) in pairs(&consts) {
        let lb: Vec<usize> = consts
            .iter()
            .copied()
            .filter(|&i| c[i].vals.is_subset(&c[a].vals) && c[i].vals.is_subset(&c[b].vals))
            .collect();
        if maximal(c, &lb).len() != 1 {
            bad += 1;
        }
    }
    bad
}

/// Minimal elements of a set of indices under inclusion.
fn minimal(c: &[Shape], idx: &[usize]) -> Vec<usize> {
    idx.iter()
        .copied()
        .filter(|&i| {
            !idx.iter()
                .any(|&j| j != i && c[j].vals.is_subset(&c[i].vals))
        })
        .collect()
}

fn maximal(c: &[Shape], idx: &[usize]) -> Vec<usize> {
    idx.iter()
        .copied()
        .filter(|&i| {
            !idx.iter()
                .any(|&j| j != i && c[i].vals.is_subset(&c[j].vals))
        })
        .collect()
}

fn pairs(idx: &[usize]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for i in 0..idx.len() {
        for j in (i + 1)..idx.len() {
            out.push((idx[i], idx[j]));
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Everything below this line is mine. Above it is 221's, verbatim except for the one parameter.
// ---------------------------------------------------------------------------------------------

/// 221's `catalogue_with`, with deduplication made a parameter.
fn catalogue_2x2(degenerates: bool, dedup: bool) -> Vec<Shape> {
    let out = catalogue_undeduped(degenerates);
    if !dedup {
        return out;
    }
    let mut seen: BTreeMap<Set, usize> = BTreeMap::new();
    let mut uniq = Vec::new();
    for s in out {
        if !seen.contains_key(&s.vals) {
            seen.insert(s.vals.clone(), uniq.len());
            uniq.push(s);
        }
    }
    uniq
}

/// The generator half of 221's `catalogue_with`, up to the deduplication block.
fn catalogue_undeduped(degenerates: bool) -> Vec<Shape> {
    let mut out = Vec::new();
    if degenerates {
        out.extend(degenerate());
    }
    for &step in &[1i64, 2, 4, 8, 16] {
        for &half in &[16i64, 32, 64, 128] {
            for &phase in &[0i64, step / 2] {
                if phase != 0 && step == 1 {
                    continue;
                }
                let s = constant(step, half, phase);
                if !s.vals.is_empty() {
                    out.push(s);
                }
            }
        }
    }
    for &p in &[2u32, 3] {
        for &(emin, emax) in &[(-3i32, 2i32), (-2, 3), (-4, 1)] {
            for &half in &[32i64, 128] {
                let s = exponential(p, emin, emax, half);
                if s.vals.len() > 3 {
                    out.push(s);
                }
            }
        }
    }
    out
}

/// C1's count, split by which of the three mechanisms produced each failure.
struct Split {
    total_pairs: usize,
    failures: usize,
    /// No lower bound exists at all: the bottom is absent.
    no_lower_bounds: usize,
    /// Lower bounds exist and `maximal` returned none, which only happens on a tie.
    maximal_annihilated: usize,
    /// Several maximal lower bounds denoting different sets: a real order failure.
    several_distinct: usize,
    /// Several maximal lower bounds all denoting one set.
    several_same: usize,
}

fn c1_split(c: &[Shape]) -> Split {
    let consts: Vec<usize> = (0..c.len()).filter(|&i| c[i].kind == Kind::Constant).collect();
    let mut s = Split {
        total_pairs: 0,
        failures: 0,
        no_lower_bounds: 0,
        maximal_annihilated: 0,
        several_distinct: 0,
        several_same: 0,
    };
    for (a, b) in pairs(&consts) {
        s.total_pairs += 1;
        let lb: Vec<usize> = consts
            .iter()
            .copied()
            .filter(|&i| c[i].vals.is_subset(&c[a].vals) && c[i].vals.is_subset(&c[b].vals))
            .collect();
        let m = maximal(c, &lb);
        if m.len() == 1 {
            continue;
        }
        s.failures += 1;
        if lb.is_empty() {
            s.no_lower_bounds += 1;
        } else if m.is_empty() {
            s.maximal_annihilated += 1;
        } else {
            let denots: BTreeSet<&Set> = m.iter().map(|&i| &c[i].vals).collect();
            if denots.len() == 1 {
                s.several_same += 1;
            } else {
                s.several_distinct += 1;
            }
        }
    }
    s
}

fn main() {
    println!("=== a6: the two repairs 221 applied together, varied one at a time ===");
    println!();
    println!("C1 is 221's control: inside the constant-quantum family alone, every pair must have");
    println!("a unique meet. The four columns say which mechanism produced each failure.");
    println!();
    println!(
        "{:>10} {:>6} {:>7} {:>6} {:>6} | {:>9} {:>12} {:>9} {:>9}",
        "degenerate", "dedup", "points", "pairs", "fail", "no bounds", "annihilated", "distinct", "same"
    );
    let mut cells = Vec::new();
    for &degen in &[true, false] {
        for &dedup in &[true, false] {
            let c = catalogue_2x2(degen, dedup);
            let s = c1_split(&c);
            println!(
                "{:>10} {:>6} {:>7} {:>6} {:>6} | {:>9} {:>12} {:>9} {:>9}",
                degen,
                dedup,
                c.len(),
                s.total_pairs,
                s.failures,
                s.no_lower_bounds,
                s.maximal_annihilated,
                s.several_distinct,
                s.several_same
            );
            cells.push((degen, dedup, s));
        }
    }
    println!();

    let get = |d: bool, u: bool| -> &Split {
        &cells.iter().find(|(a, b, _)| *a == d && *b == u).unwrap().2
    };

    println!("--- P1, the control that decides whether anything else here counts ---");
    let on_on = get(true, true).failures;
    let off_on = get(false, true).failures;
    println!("  221's repaired cell (degenerates on, dedup on):  {on_on}  (its run reports 0)");
    println!("  221's C5 cell       (degenerates off, dedup on): {off_on}  (its run reports 256)");
    let reproduced = on_on == 0 && off_on == 256;
    println!("  both reproduced: {reproduced}");
    if !reproduced {
        println!("  *** this copy is not 221's instrument and nothing below counts ***");
        return;
    }
    println!();

    println!("--- what each repair is actually for ---");
    let dd = get(true, false);
    let od = get(false, true);
    println!(
        "  removing the bottom, deduplication kept:   {} failures, {} of them with no lower bound",
        od.failures, od.no_lower_bounds
    );
    println!(
        "  keeping the bottom, deduplication removed: {} failures, {} of them with lower bounds",
        dd.failures,
        dd.failures - dd.no_lower_bounds
    );
    println!(
        "    of those, {} are `maximal` annihilating a tie, {} are several distinct denotations",
        dd.maximal_annihilated, dd.several_distinct
    );
    println!();
    println!("  So the 256 are mechanism (a), the bottom, exactly as 221's own p2d diagnosis says.");
    println!("  The dedup-off failures are mechanism (c), the copied helper being defined only on");
    println!("  a partial order: `maximal` drops every member of a tie because each is a subset of");
    println!("  the other. Not one failure in any cell is several distinct denotations, so nothing");
    println!("  measured here is an order failure of the kind option 1 is about.");
    println!();
    println!("--- what this overturns and what it leaves standing ---");
    println!("  221's ANSWER stands and I agree with it. Deciding inclusion on denotation is right,");
    println!("  because without it the relation is a preorder, two declarations denoting one set");
    println!("  are each below the other and neither is the meet, and 'unique least upper bound' is");
    println!("  unique only up to an equivalence nobody named.");
    println!();
    println!("  Its EVIDENCE does not reach that answer. The 256 are the bottom being absent, which");
    println!("  is a different question with a different answer, and the sentence 'the deduplication");
    println!("  is what makes C1 pass' names a repair its instrument never varied. What varying it");
    println!("  shows is that the helper functions require the repair, which is a fact about the");
    println!("  instrument rather than evidence about the canon.");
    println!();
    println!("=== end a6 ===");
}
