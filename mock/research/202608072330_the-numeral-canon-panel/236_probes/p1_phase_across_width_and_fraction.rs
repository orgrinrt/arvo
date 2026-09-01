//! Does the half-step-phase result move with the width or the fraction width,
//! and does its adaptation-law half read the tie rule at all.
//!
//! `proposal::a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity`
//! carries `total_width: W = 4`, `fraction_width: F = 0` and
//! `rounding = nearest`. Its instrument, `56_probes/q2_affine_membership.rs`,
//! runs one grid: `STEP = 8`, `BIAS = 4`, `SCALE = 32`, sixteen points. It
//! varies neither width nor fraction width, so nothing in the corpus supports a
//! `construction` warrant on either axis, which
//! `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` obliges an
//! instrument to supply. And `question::which_tie_direction_an_unqualified_nearest_names`
//! names that row as one of the two whose rounding axis names no member of the
//! ratified six.
//!
//! THE CASES THAT MUST FAIL, stated before the run:
//!
//!   C1. At phase zero the closure claim MUST be false: exact sums of two grid
//!       points DO land on the grid. If C1 reports closure broken at phase zero
//!       the instrument is not reading the phase and every other number here is
//!       worthless.
//!
//!   C2. At a one-third phase the closure claim MUST still hold and the
//!       half-step distance claim MUST be false. This separates "no sum lands
//!       on the grid", which any nonzero phase gives, from "every sum sits
//!       exactly half a step away", which only the half phase gives. The
//!       original instrument ran one phase and could not tell them apart.
//!
//!   C3. A rounder targeting the phase-zero grid MUST fail retraction on the
//!       biased grid. Carried over from `q2` so this file reproduces its
//!       control rather than trusting it.
//!
//!   C4. The monotonicity arm MUST be able to report false. A deliberately
//!       non-monotone rounder is run through the same arm and has to be caught.
//!
//! Arithmetic is exact in integer units of `Q/6`, where `Q` is the quantum, so
//! phases 0, 1/6, 1/3 and 1/2 are all expressible with no rational type and no
//! rounding of the model itself. A grid point of slot `k` at phase `p/6` sits at
//! `6k + p`. Nothing here depends on `Q`'s value, and that is the point: `F`
//! enters only through `Q = 2^-F`, and the width `W` only through the slot
//! count `N = 2^W`, so both are swept and both are reported per cell rather
//! than asserted away.
//!
//! Run: `rustc -O p1_phase_across_width_and_fraction.rs -o /tmp/p1 && /tmp/p1`

use std::collections::BTreeSet;

const L: i64 = 6; // units per quantum; 6 so that 0, 1/6, 1/3, 1/2 are integers

/// The grid of `n` slots at phase `p/L`, in units of `Q/L`.
fn grid(n: i64, p: i64) -> Vec<i64> {
    (0..n).map(|k| L * k + p).collect()
}

/// Does any exact sum of two grid points land on the grid.
///
/// Quantified over the whole grid rather than over the in-range part, because
/// the claim is about the set being closed and a sum leaving the range is a
/// different failure from a sum landing between points.
fn sums_on_grid(g: &[i64]) -> usize {
    let set: BTreeSet<i64> = g.iter().copied().collect();
    let mut hits = 0;
    for &a in g {
        for &b in g {
            if set.contains(&(a + b)) {
                hits += 1;
            }
        }
    }
    hits
}

/// Is every exact sum exactly half a quantum from the nearest grid point.
///
/// Distance is computed against the unbounded grid `{L*k + p}` rather than
/// against the stored slots, so a sum past the top of the range is measured by
/// its offset from the lattice rather than reported as far away.
fn every_sum_is_a_half_step(g: &[i64], p: i64) -> (bool, BTreeSet<i64>) {
    let mut all = true;
    let mut seen = BTreeSet::new();
    for &a in g {
        for &b in g {
            let s = a + b;
            let r = ((s - p) % L + L) % L;
            let d = r.min(L - r);
            seen.insert(d);
            if d != L / 2 {
                all = false;
            }
        }
    }
    (all, seen)
}

/// Zero, and one, in units of `Q/L`, at fraction width `f`.
///
/// One is `2^f` quanta, so `L * 2^f` units. Zero is the origin.
fn contains_zero_and_one(g: &[i64], f: u32) -> (bool, bool) {
    let set: BTreeSet<i64> = g.iter().copied().collect();
    (set.contains(&0), set.contains(&(L * (1i64 << f))))
}

/// Round to nearest onto `g`, breaking a tie by `tie`.
///
/// `tie` sees the two candidate slot indices and returns the one to take, so a
/// tie rule is a parameter rather than a rewrite of the rounder.
fn round_nearest(x: i64, g: &[i64], tie: fn(usize, usize) -> usize) -> i64 {
    let mut best = 0usize;
    for i in 1..g.len() {
        let di = (g[i] - x).abs();
        let db = (g[best] - x).abs();
        if di < db {
            best = i;
        } else if di == db {
            best = tie(best, i);
        }
    }
    g[best]
}

fn tie_up(_lo: usize, hi: usize) -> usize {
    hi
}

fn tie_even(lo: usize, hi: usize) -> usize {
    if lo % 2 == 0 { lo } else { hi }
}

/// C4's mutant: a tie rule is not what breaks monotonicity, so the control has
/// to break it somewhere a tie rule cannot reach. This one takes the top slot
/// whenever the input is odd, which is non-monotone by construction.
fn round_non_monotone(x: i64, g: &[i64]) -> i64 {
    if x % 2 != 0 { g[g.len() - 1] } else { round_nearest(x, g, tie_up) }
}

/// The four adaptation laws, over the window the grid spans.
///
/// `total` and `retraction` are over the grid; `monotone` and
/// `distance_minimising` are over every integer in the window, which is every
/// value the rounder can be handed.
fn adaptation_laws(g: &[i64], r: &dyn Fn(i64) -> i64) -> (bool, bool, bool, bool) {
    let set: BTreeSet<i64> = g.iter().copied().collect();
    let lo = *g.first().unwrap();
    let hi = *g.last().unwrap();
    let total = (lo..=hi).all(|x| set.contains(&r(x)));
    let retraction = g.iter().all(|&x| r(x) == x);
    let mut monotone = true;
    let mut prev = r(lo);
    for x in lo..=hi {
        let cur = r(x);
        if cur < prev {
            monotone = false;
        }
        prev = cur;
    }
    let distance = (lo..=hi).all(|x| g.iter().all(|&c| (r(x) - x).abs() <= (c - x).abs()));
    (total, retraction, monotone, distance)
}

fn main() {
    let widths: Vec<u32> = (2..=8).collect();
    let fractions: Vec<u32> = (0..=6).collect();
    let mut ok = true;

    // ---- part 1: the closure claim across width and fraction width ----------
    //
    // Three phases per cell: half, zero (C1) and a third (C2).
    println!("## part 1: closure, zero and one, across W and F");
    println!("W  F  phase   sums_on_grid  every_sum_half_step  has_zero  has_one");
    let mut half_never_closes = true;
    let mut half_always_half_step = true;
    let mut half_never_has_zero_or_one = true;
    let mut zero_phase_closes_somewhere = false;
    let mut third_phase_ever_half_step = false;
    let mut third_phase_closes_somewhere = false;
    for &w in &widths {
        for &f in &fractions {
            let n = 1i64 << w;
            for (label, p) in [("1/2", L / 2), ("0", 0), ("1/3", L / 3)] {
                let g = grid(n, p);
                let hits = sums_on_grid(&g);
                let (half, _) = every_sum_is_a_half_step(&g, p);
                let (z, o) = contains_zero_and_one(&g, f);
                if p == L / 2 {
                    if hits != 0 {
                        half_never_closes = false;
                    }
                    if !half {
                        half_always_half_step = false;
                    }
                    if z || o {
                        half_never_has_zero_or_one = false;
                    }
                } else if p == 0 {
                    if hits > 0 {
                        zero_phase_closes_somewhere = true;
                    }
                } else {
                    if half {
                        third_phase_ever_half_step = true;
                    }
                    if hits > 0 {
                        third_phase_closes_somewhere = true;
                    }
                }
                if w <= 3 && f <= 1 {
                    println!(
                        "{w:<2} {f:<2} {label:<7} {hits:<13} {half:<20} {z:<9} {o}"
                    );
                }
            }
        }
    }
    println!("... (full sweep run, sample rows printed)");
    println!();
    println!(
        "half phase, W in 2..=8, F in 0..=6: no sum lands on the grid anywhere: {half_never_closes}"
    );
    println!(
        "half phase, same cube: every sum sits exactly half a quantum away: {half_always_half_step}"
    );
    println!(
        "half phase, same cube: the grid contains neither zero nor one: {half_never_has_zero_or_one}"
    );
    ok &= half_never_closes && half_always_half_step && half_never_has_zero_or_one;

    println!();
    println!("### controls on part 1");
    println!("C1 zero phase: sums DO land on the grid somewhere: {zero_phase_closes_somewhere}");
    println!(
        "C2 third phase: sums still never land on the grid: {}",
        !third_phase_closes_somewhere
    );
    println!(
        "C2 third phase: the half-step distance claim is FALSE: {}",
        !third_phase_ever_half_step
    );
    ok &= zero_phase_closes_somewhere && !third_phase_closes_somewhere && !third_phase_ever_half_step;

    // ---- part 2: does the adaptation-law half read the tie rule -------------
    //
    // At the half phase every exact sum is a tie, so this is the cell where a
    // tie rule is maximally observable rather than one where it never fires.
    println!();
    println!("## part 2: the four adaptation laws under two tie rules");
    println!("W  F  tie        total  retraction  monotone  distance_minimising");
    let mut up_all = true;
    let mut even_all = true;
    let mut mutant_target_fails_retraction_everywhere = true;
    let mut non_monotone_control_caught_everywhere = true;
    for &w in &widths {
        for &f in &fractions {
            let n = 1i64 << w;
            let hub = grid(n, L / 2);
            let plain = grid(n, 0);
            for (label, tie) in [
                ("half_up", tie_up as fn(usize, usize) -> usize),
                ("half_even", tie_even as fn(usize, usize) -> usize),
            ] {
                let r = |x: i64| round_nearest(x, &hub, tie);
                let (t, re, m, d) = adaptation_laws(&hub, &r);
                if label == "half_up" {
                    up_all &= t && re && m && d;
                } else {
                    even_all &= t && re && m && d;
                }
                if w <= 3 && f == 0 {
                    println!("{w:<2} {f:<2} {label:<10} {t:<6} {re:<11} {m:<9} {d}");
                }
            }
            // C3: a rounder aimed at the phase-zero grid must fail retraction.
            let wrong = |x: i64| round_nearest(x, &plain, tie_up);
            let (_, re_wrong, _, _) = adaptation_laws(&hub, &wrong);
            if re_wrong {
                mutant_target_fails_retraction_everywhere = false;
            }
            // C4: the monotonicity arm must be able to say false.
            let bad = |x: i64| round_non_monotone(x, &hub);
            let (_, _, m_bad, _) = adaptation_laws(&hub, &bad);
            if m_bad {
                non_monotone_control_caught_everywhere = false;
            }
        }
    }
    println!("... (full sweep run, sample rows printed)");
    println!();
    println!("half_up keeps all four laws over the whole cube:   {up_all}");
    println!("half_even keeps all four laws over the whole cube: {even_all}");
    println!(
        "so the adaptation-law half does not read the tie rule: {}",
        up_all && even_all
    );
    ok &= up_all && even_all;

    println!();
    println!("### controls on part 2");
    println!(
        "C3 phase-zero-target rounder fails retraction on the biased grid, every cell: {mutant_target_fails_retraction_everywhere}"
    );
    println!(
        "C4 non-monotone control is caught by the monotonicity arm, every cell: {non_monotone_control_caught_everywhere}"
    );
    ok &= mutant_target_fails_retraction_everywhere && non_monotone_control_caught_everywhere;

    println!();
    println!("{}", if ok { "P1 WORKS" } else { "P1 FAILED" });
    if !ok {
        std::process::exit(1);
    }
}
