// p2: does the inclusion order on representable sets have joins? does it have
// meets? and where exactly does whichever one fails, fail?
//
// WHY THIS RUNS. `question::one_numeral_family_or_several` asks whether the
// numerals form one family, and its options are phrased over "a unique least
// upper bound". `question::is_the_cross_kind_join_closed_or_priced` asks
// whether to close the shape space under intersection. Those are two questions
// about two different operations on one order, and no instrument in this
// corpus has computed either one over a catalogue containing both named kinds.
//
// The order is set inclusion on representable sets, which is what
// `proposal::membership_of_the_representable_set_is_one_affine_predicate`
// (ratified through `ruling::the_format_spine_is_canon`) makes the primary
// object: "an affine slot function, a quantum per magnitude and a phase, of
// which integers, fixed point, scaled integers and floats are points". So the
// catalogue below is points of that parameterisation and the order is computed
// on their denotations rather than on their declarations, which is also the
// amendment `question::inclusion_order_singleton_amendment` is about.
//
// CATALOGUE. Everything is scaled by 16 so a value is an i64 and exact.
//   CONSTANT-QUANTUM points (fixed point / integers / scaled integers):
//     step in {1, 2, 4, 8, 16} sixteenths, half-width in {16, 32, 64, 128},
//     phase in {0, half a step}.
//   EXPONENTIAL-QUANTUM points (floats): precision p in {2, 3}, exponent range
//     inside what stays a multiple of a sixteenth, with subnormals.
//
// WHAT MUST FAIL, declared before the run. The run is void if any control
// reports other than its required verdict.
//   C1  Inside the constant-quantum family alone, every pair must have a
//       unique meet in the catalogue. Without this the instrument reports "no
//       meet" everywhere and has measured its own catalogue's poverty rather
//       than a property of the order.
//   C2  At least one pair's set intersection must be a set the catalogue does
//       NOT contain. Without this the closure question is empty and there is
//       nothing to price.
//   C3  Every pair must have at least one upper bound in the catalogue.
//       Without a top the join question is about the catalogue's ceiling
//       rather than about the order, so the catalogue carries the finest,
//       widest constant-quantum point on purpose.
//   C4  The two named kinds must be distinguishable: at least one
//       exponential-quantum point must be absent from the constant-quantum
//       family. Without this both families denote the same sets and the word
//       "cross-kind" names nothing.
//   C5  With the degenerate points REMOVED, C1 must fail. This is the arm that
//       says the degenerate points are load-bearing rather than decoration; if
//       C1 passes without them then admitting them buys nothing and the
//       singleton amendment is a matter of taste after all.
//
// C1 FAILED ON THE FIRST RUN AND THE RUN IS KEPT. `p2_v1_c1_failed_meets_
// inside_the_constant_family.out` is that run: 256 constant-quantum pairs with
// no meet. `p2d_diagnose_c1.rs` split them and every one of the 256 has an
// EMPTY intersection, zero have several maximal lower bounds. The catalogue was
// filtering out the degenerate points, so the failure was the instrument
// telling the truth about a catalogue that had excluded the bottom of its own
// order. The repair is to admit them, which is what C5 now measures the value
// of, and it is the substance of
// `question::inclusion_order_singleton_amendment`.
//
// SCOPE. radix 2, one bounded domain of 257 grid points, the enumerated
// catalogue only. This establishes the structure over that catalogue and does
// not establish it over the whole parameterisation.

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

fn main() {
    let c = catalogue();
    println!("### p2. joins and meets of representable sets under inclusion");
    println!("### catalogue: {} shapes, distinct by denotation", c.len());
    let nc = c.iter().filter(|s| s.kind == Kind::Constant).count();
    let ne = c.len() - nc;
    println!("###   constant quantum {nc}, exponential quantum {ne}");
    println!();

    let all: Vec<usize> = (0..c.len()).collect();
    let consts: Vec<usize> = all
        .iter()
        .copied()
        .filter(|&i| c[i].kind == Kind::Constant)
        .collect();

    let mut void = false;
    println!("CONTROLS");

    // C1: meets inside the constant family are unique.
    let c1_bad = constant_meet_failures(&c);
    let c1 = c1_bad == 0;
    println!(
        "  C1  meet is unique inside the constant-quantum family        {:>11}  required=0 bad, got {c1_bad}",
        if c1 { "as required" } else { "*** VOID ***" }
    );
    void |= !c1;

    // C2: some intersection is not in the catalogue.
    let cat: BTreeSet<&Set> = c.iter().map(|s| &s.vals).collect();
    let mut c2_hits = 0;
    let mut c2_example = String::new();
    for (a, b) in pairs(&all) {
        let inter: Set = c[a].vals.intersection(&c[b].vals).copied().collect();
        if inter.len() > 1 && !cat.contains(&inter) {
            if c2_hits == 0 {
                c2_example = format!("{}  /\\  {}", c[a].name, c[b].name);
            }
            c2_hits += 1;
        }
    }
    let c2 = c2_hits > 0;
    println!(
        "  C2  some pair's intersection is outside the catalogue        {:>11}  required=>0, got {c2_hits}",
        if c2 { "as required" } else { "*** VOID ***" }
    );
    void |= !c2;

    // C3: every pair has an upper bound.
    let mut c3_bad = 0;
    for (a, b) in pairs(&all) {
        let ub = all
            .iter()
            .copied()
            .filter(|&i| c[a].vals.is_subset(&c[i].vals) && c[b].vals.is_subset(&c[i].vals))
            .count();
        if ub == 0 {
            c3_bad += 1;
        }
    }
    let c3 = c3_bad == 0;
    println!(
        "  C3  every pair has at least one upper bound                  {:>11}  required=0 bad, got {c3_bad}",
        if c3 { "as required" } else { "*** VOID ***" }
    );
    void |= !c3;

    // C4: the kinds are distinguishable by denotation.
    let const_sets: BTreeSet<&Set> = c
        .iter()
        .filter(|s| s.kind == Kind::Constant)
        .map(|s| &s.vals)
        .collect();
    let c4_hits = c
        .iter()
        .filter(|s| s.kind == Kind::Exponential && !const_sets.contains(&s.vals))
        .count();
    let c4 = c4_hits > 0;
    println!(
        "  C4  some exponential point denotes no constant point's set   {:>11}  required=>0, got {c4_hits}",
        if c4 { "as required" } else { "*** VOID ***" }
    );
    void |= !c4;

    // C5: the degenerate points are load-bearing.
    let without = catalogue_with(false);
    let c5_bad = constant_meet_failures(&without);
    let c5 = c5_bad > 0;
    println!(
        "  C5  removing the degenerate points breaks C1                 {:>11}  required=>0, got {c5_bad}",
        if c5 { "as required" } else { "*** VOID ***" }
    );
    void |= !c5;
    println!();

    if void {
        println!("*** A CONTROL DID NOT REPORT ITS REQUIRED VERDICT. NOTHING BELOW COUNTS. ***");
        std::process::exit(1);
    }

    // --- the measurement ---------------------------------------------------
    let mut join_unique = 0usize;
    let mut join_many = 0usize;
    let mut meet_unique = 0usize;
    let mut meet_many = 0usize;
    let mut meet_none = 0usize;
    let mut cross_meet_many = 0usize;
    let mut same_meet_many = 0usize;
    let mut cross_join_many = 0usize;
    let mut same_join_many = 0usize;
    let mut join_fail_same_phase = 0usize;
    let mut join_fail_diff_phase = 0usize;
    let mut meet_fail_same_phase = 0usize;
    let mut meet_fail_diff_phase = 0usize;
    let mut first_meet_failure = String::new();
    let mut first_join_failure = String::new();

    for (a, b) in pairs(&all) {
        let cross = c[a].kind != c[b].kind;
        let ub: Vec<usize> = all
            .iter()
            .copied()
            .filter(|&i| c[a].vals.is_subset(&c[i].vals) && c[b].vals.is_subset(&c[i].vals))
            .collect();
        let lb: Vec<usize> = all
            .iter()
            .copied()
            .filter(|&i| c[i].vals.is_subset(&c[a].vals) && c[i].vals.is_subset(&c[b].vals))
            .collect();
        let least = minimal(&c, &ub);
        let greatest = maximal(&c, &lb);
        let same_phase = c[a].phase == c[b].phase;
        if least.len() == 1 {
            join_unique += 1;
        } else {
            join_many += 1;
            if cross {
                cross_join_many += 1;
            } else {
                same_join_many += 1;
            }
            if same_phase {
                join_fail_same_phase += 1;
            } else {
                join_fail_diff_phase += 1;
            }
            if first_join_failure.is_empty() {
                first_join_failure = format!(
                    "{}\n              {}\n           minimal upper bounds: {}",
                    c[a].name,
                    c[b].name,
                    least
                        .iter()
                        .map(|&i| c[i].name.clone())
                        .collect::<Vec<_>>()
                        .join(" | ")
                );
            }
        }
        match greatest.len() {
            1 => meet_unique += 1,
            0 => meet_none += 1,
            _ => {
                meet_many += 1;
                if cross {
                    cross_meet_many += 1;
                } else {
                    same_meet_many += 1;
                }
                if c[a].phase == c[b].phase {
                    meet_fail_same_phase += 1;
                } else {
                    meet_fail_diff_phase += 1;
                }
                if first_meet_failure.is_empty() {
                    first_meet_failure = format!(
                        "{}\n              {}\n           maximal lower bounds: {}",
                        c[a].name,
                        c[b].name,
                        greatest
                            .iter()
                            .map(|&i| c[i].name.clone())
                            .collect::<Vec<_>>()
                            .join(" | ")
                    );
                }
            }
        }
    }

    let total = join_unique + join_many;
    println!("MEASUREMENT over all {total} unordered pairs");
    println!("  join    unique least upper bound      {join_unique}");
    println!("  join    several minimal upper bounds  {join_many}");
    println!("  meet    unique greatest lower bound   {meet_unique}");
    println!("  meet    several maximal lower bounds  {meet_many}");
    println!("  meet    no lower bound at all         {meet_none}");
    println!();
    println!("  of the {join_many} pairs with no unique JOIN:");
    println!("    both points of one kind    {same_join_many}");
    println!("    one of each kind           {cross_join_many}");
    println!("    phases agree               {join_fail_same_phase}");
    println!("    phases differ              {join_fail_diff_phase}");
    println!();
    println!("  of the {meet_many} pairs with no unique MEET:");
    println!("    both points of one kind    {same_meet_many}");
    println!("    one of each kind           {cross_meet_many}");
    println!("    phases agree               {meet_fail_same_phase}");
    println!("    phases differ              {meet_fail_diff_phase}");
    println!();
    if !first_join_failure.is_empty() {
        println!("  first join failure:  {first_join_failure}");
    } else {
        println!("  no join failure: the order is a join-semilattice over this catalogue");
    }
    println!();
    if !first_meet_failure.is_empty() {
        println!("  first meet failure:  {first_meet_failure}");
    } else {
        println!("  no meet failure over this catalogue");
    }
    println!();
    // Is a failure a property of the pair, or of the space it sits in?
    let (n_all, j_all, m_all) = failures(&c, |_| true, true);
    let (n_con, j_con, m_con) = failures(&c, |s| s.kind == Kind::Constant, true);
    let (n_solo, j_solo, m_solo) = failures(&c, |s| s.kind == Kind::Constant, false);
    println!("IS THE FAILURE A PROPERTY OF THE PAIR OR OF THE SPACE?");
    println!("  the same constant-quantum pairs are measured twice, and the only thing that");
    println!("  changes is whether the exponential-quantum points are in the space at all.");
    println!("  every pair, bounds drawn from the whole space         pairs {n_all:>5}  join fails {j_all:>4}  meet fails {m_all:>4}");
    println!("  constant pairs, bounds from the whole space           pairs {n_con:>5}  join fails {j_con:>4}  meet fails {m_con:>4}");
    println!("  constant pairs, bounds from the constant space only   pairs {n_solo:>5}  join fails {j_solo:>4}  meet fails {m_solo:>4}");
    println!();

    println!("  intersections falling outside the catalogue: {c2_hits}");
    println!("  first such:  {c2_example}");
}

/// Join and meet failure counts over an arbitrary catalogue, restricted to
/// pairs both of whose members satisfy `keep`.
///
/// This is what says whether a failure is a property of the PAIR or of the
/// SPACE the pair sits in: the same pairs are re-measured with the other kind
/// removed from the catalogue, and nothing else changes.
fn failures(
    c: &[Shape],
    keep: impl Fn(&Shape) -> bool + Copy,
    bounds_from_all: bool,
) -> (usize, usize, usize) {
    let live: Vec<usize> = (0..c.len()).filter(|&i| keep(&c[i])).collect();
    let cand: Vec<usize> = if bounds_from_all {
        (0..c.len()).collect()
    } else {
        live.clone()
    };
    let mut jm = 0;
    let mut mm = 0;
    let n = pairs(&live).len();
    for (a, b) in pairs(&live) {
        let ub: Vec<usize> = cand
            .iter()
            .copied()
            .filter(|&i| c[a].vals.is_subset(&c[i].vals) && c[b].vals.is_subset(&c[i].vals))
            .collect();
        let lb: Vec<usize> = cand
            .iter()
            .copied()
            .filter(|&i| c[i].vals.is_subset(&c[a].vals) && c[i].vals.is_subset(&c[b].vals))
            .collect();
        if minimal(c, &ub).len() != 1 {
            jm += 1;
        }
        if maximal(c, &lb).len() != 1 {
            mm += 1;
        }
    }
    (n, jm, mm)
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
