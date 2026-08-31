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
        Shape { name: "degenerate: no values".into(), kind: Kind::Constant, phase: 0, vals: Set::new() },
        Shape { name: "degenerate: one value {0}".into(), kind: Kind::Constant, phase: 0, vals: [0i64].into_iter().collect() },
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
    let consts: Vec<usize> = (0..c.len()).filter(|&i| c[i].kind == Kind::Constant).collect();
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
