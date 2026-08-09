//! Probe 3. The adopted two-arity fallback does not state whether a
//! consumer's directional fallback OVERRIDES the preset's own resolution row
//! where that row answers, or COMPLETES it only where the row is silent. Both
//! readings implement cleanly, and they diverge observably at a ratified
//! cell, so the ratifying text must pick one; this probe compiles both and
//! executes the divergence.
//!
//! Separation statement per 86b: the two readings separate at a preset whose
//! OverRange row answers the directional cell (clamp -> far point). At Hot
//! fixed point (row silent at x/0) they coincide, which is exactly the
//! preset the fork spent its files on, so a model instantiated only at Hot
//! could not have seen this.
//!
//! Build: rustc --edition 2021 -O probe_3_fallback_slot_domain.rs -o out/probe_3
//! Run:   ./out/probe_3

const FAR_POINT: i64 = i64::MAX; // model far point for the clamp row

#[derive(Clone, Copy, PartialEq, Debug)]
enum Row {
    ClampAnswers,
    Silent, // Hot's ReduceModulo: no answer for an unbounded exact result
}

/// Reading A: the fallback slots always apply at their cells.
fn div_total_shadowing(x: i64, d: i64, f_dir: i64, f_ind: i64, _row: Row) -> i64 {
    if d != 0 {
        x / d
    } else if x != 0 {
        f_dir // consumer value, even where the row answers
    } else {
        f_ind
    }
}

/// Reading B: the row governs where it answers; the slots fill only holes.
fn div_total_completing(x: i64, d: i64, f_dir: i64, f_ind: i64, row: Row) -> i64 {
    if d != 0 {
        x / d
    } else if x != 0 {
        match row {
            Row::ClampAnswers => {
                if x > 0 {
                    FAR_POINT
                } else {
                    -FAR_POINT
                }
            }
            Row::Silent => f_dir,
        }
    } else {
        f_ind
    }
}

fn main() {
    let f_dir = 7i64;
    let f_ind = -1i64;

    // The divergent instantiation: a clamp preset, whose ratified OverRange
    // row answers the directional cell with the far point.
    let a = div_total_shadowing(5, 0, f_dir, f_ind, Row::ClampAnswers);
    let b = div_total_completing(5, 0, f_dir, f_ind, Row::ClampAnswers);
    println!(
        "clamp preset, 5/0: shadowing={a}, completing={b}, agree={}",
        a == b
    );

    // The coincident instantiation: Hot, row silent. Both readings hand the
    // cell to the consumer, which is the case the fork argued about.
    let c = div_total_shadowing(5, 0, f_dir, f_ind, Row::Silent);
    let d = div_total_completing(5, 0, f_dir, f_ind, Row::Silent);
    println!(
        "hot preset,   5/0: shadowing={c}, completing={d}, agree={}",
        c == d
    );

    // The indeterminate cell: both readings defer to the consumer at 0/0,
    // since no row anywhere answers it (file 93 clause 3, adopted).
    let e = div_total_shadowing(0, 0, f_dir, f_ind, Row::ClampAnswers);
    let f = div_total_completing(0, 0, f_dir, f_ind, Row::ClampAnswers);
    println!(
        "clamp preset, 0/0: shadowing={e}, completing={f}, agree={}",
        e == f
    );

    // The collapse the adopted sentence names: one value for both slots is
    // expressible under either reading; nothing below distinguishes them.
    let g = div_total_shadowing(5, 0, 0, 0, Row::Silent);
    let h = div_total_completing(0, 0, 0, 0, Row::Silent);
    println!("collapsed slots at hot: dir-cell={g}, ind-cell={h}");
}
