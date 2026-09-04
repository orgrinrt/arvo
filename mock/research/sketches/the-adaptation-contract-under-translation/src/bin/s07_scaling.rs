//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 07. A third law, and it is not a third symmetry.
//!
//! Step 06 measures that translation and reflection generate every isometry of
//! the grid, so nothing else in that family is left. Multiplying by a whole
//! number is not an isometry: it maps the grid into itself and not onto itself,
//! so it is an endomorphism and sits outside the group. That makes it a separate
//! question rather than a third generator, and the question is whether it has a
//! region worth an arm.
//!
//! ```text
//! scaling:  adapt(m * position) == adapt(m * adapt(position))
//! ```
//!
//! The prediction before running: the rounding region cannot commute with it off
//! the grid, because rounding a scaled residue is not scaling a rounded one, and
//! the completion region commutes with it under every policy at a non-negative
//! `m`, because scaling carries an excursion further out on the side it already
//! left on and wrapping is an endomorphism for multiplication as well as for
//! addition. So the region should be the positions that are all on the grid, at
//! `m` at least zero, under any policy. If that is what comes back it is a real
//! arm: a chain of whole-number scalings of an on-grid value adapts once at the
//! end rather than at every step.

use the_adaptation_contract_under_translation::{
    MODES,
    POLICIES,
    Range,
    adapt_at,
    dither,
    position,
    residues,
    verdict,
};

const DITHER: (i64, i64) = (1, 2);

/// The scalings measured. Zero and one are the degenerate ones and are in the
/// walk so a region that included only them would be visible as such.
const FACTORS: [i64; 7] = [-2, -1, 0, 1, 2, 3, 5];

/// What one cell's walk found.
struct Cell {
    held:       bool,
    walked:     u64,
    excursions: u64,
    witness:    Option<(i64, i64, i64, i64, i64)>,
}

/// The scaling law over one cell.
///
/// `on_grid_only` is the restriction the prediction turns on: with it the walk
/// visits positions with a zero residue and nothing else.
fn scaling(
    range: Range,
    mode: arvo_format::rounding::Mode,
    policy: arvo_format::overflow::Policy,
    factor: i64,
    on_grid_only: bool,
) -> Cell {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);
    let mut out = Cell {
        held:       true,
        walked:     0,
        excursions: 0,
        witness:    None,
    };
    for (num, den) in residues() {
        if on_grid_only && num != 0 {
            continue;
        }
        for n in (lo - span) ..= (hi + span) {
            out.walked += 1;
            if n < lo || n > hi {
                out.excursions += 1;
            }
            let here = adapt_at(range, mode, policy, position(n, num, den), d);
            let direct = adapt_at(
                range,
                mode,
                policy,
                position(factor * n, factor * num, den),
                d,
            );
            let staged = adapt_at(range, mode, policy, position(factor * here, 0, 1), d);
            if direct != staged {
                out.held = false;
                if out.witness.is_none() {
                    out.witness = Some((n, num, den, direct, staged));
                }
            }
        }
    }
    out
}

fn main() {
    println!("step 07: does the map commute with a whole-number scaling, and where");
    println!("law: adapt(m * position) == adapt(m * adapt(position))");
    println!();

    let mut cells = 0u32;
    let mut walked = 0u64;
    let mut excursions = 0u64;
    let mut held_on_grid_non_negative = 0u32;
    let mut cells_on_grid_non_negative = 0u32;
    let mut held_off_grid_scaling_up = 0u32;
    let mut cells_off_grid_scaling_up = 0u32;
    let mut held_on_grid_negative = 0u32;
    let mut cells_on_grid_negative = 0u32;

    for range in Range::SMALL {
        for &mode in &MODES {
            for &policy in &POLICIES {
                for factor in FACTORS {
                    for on_grid_only in [true, false] {
                        let cell = scaling(range, mode, policy, factor, on_grid_only);
                        cells += 1;
                        walked += cell.walked;
                        excursions += cell.excursions;
                        if on_grid_only && factor >= 0 {
                            cells_on_grid_non_negative += 1;
                            if cell.held {
                                held_on_grid_non_negative += 1;
                            }
                        }
                        if on_grid_only && factor < 0 {
                            cells_on_grid_negative += 1;
                            if cell.held {
                                held_on_grid_negative += 1;
                            }
                        }
                        if !on_grid_only && factor >= 2 {
                            cells_off_grid_scaling_up += 1;
                            if cell.held {
                                held_off_grid_scaling_up += 1;
                            }
                        }
                        println!(
                            "cell range={range:?} mode={mode:?} policy={policy:?} m={factor} on_grid_only={on_grid_only} held={} walked={} excursions={} witness={}",
                            cell.held,
                            cell.walked,
                            cell.excursions,
                            match cell.witness {
                                None => "none".to_string(),
                                Some((n, num, den, direct, staged)) =>
                                    format!(
                                        "position {n}+{num}/{den}: direct={direct} staged={staged}"
                                    ),
                            }
                        );
                    }
                }
            }
        }
    }

    println!();
    println!("--- the controls ---");
    println!("cells={cells} walked={walked} excursions={excursions}");

    verdict(
        "S1",
        &format!(
            "on the grid at a non-negative factor the law holds in {held_on_grid_non_negative} \
             of {cells_on_grid_non_negative} cells"
        ),
        held_on_grid_non_negative == cells_on_grid_non_negative,
    );
    verdict(
        "S2",
        &format!(
            "off the grid at a factor of two or more the law fails in {} of \
             {cells_off_grid_scaling_up} cells",
            cells_off_grid_scaling_up - held_off_grid_scaling_up
        ),
        held_off_grid_scaling_up == 0,
    );
    verdict(
        "S3",
        &format!(
            "on the grid at a negative factor the law holds in {held_on_grid_negative} of \
             {cells_on_grid_negative} cells, which is the reflection question wearing a factor"
        ),
        held_on_grid_negative > 0 && held_on_grid_negative < cells_on_grid_negative,
    );
    verdict(
        "S4",
        "the walk left the range, so the completion region was asked something",
        excursions > 0,
    );

    // S5. The hand-worked case for the region, and it took two goes.
    //
    // The first version of this arm was written for `[-4, 3]`, which is what the
    // crate's own suite uses, while `Range::SmallSigned` here is `Integer<5>` and
    // admits -16 to 15. Both numbers in it were wrong and the arm read BROKEN
    // against correct code, which is the right way round for a paper answer to
    // fail and is why the arm is written down rather than checked by eye.
    //
    // The case worth pinning is one where the two schedules are different
    // integers before the completion sees them. Take the slot 20, four above the
    // top, and a factor of three. Directly: 60, which is `(60 + 16) mod 32 = 12`
    // and lands on `-16 + 12`. Staged: 20 wraps to `(20 + 16) mod 32 = 4`, which
    // is `-16 + 4`, and three times that is -36, which is `(-36 + 16) mod 32 = 12`
    // and lands on the same slot. Sixty and minus thirty-six are the two integers,
    // and wrapping being an endomorphism for multiplication is what makes them one
    // answer.
    let d = dither(DITHER.0, DITHER.1);
    let here = adapt_at(
        Range::SmallSigned,
        arvo_format::rounding::Mode::Floor,
        arvo_format::overflow::Policy::Wrap,
        position(20, 0, 1),
        d,
    );
    let direct = adapt_at(
        Range::SmallSigned,
        arvo_format::rounding::Mode::Floor,
        arvo_format::overflow::Policy::Wrap,
        position(60, 0, 1),
        d,
    );
    let staged = adapt_at(
        Range::SmallSigned,
        arvo_format::rounding::Mode::Floor,
        arvo_format::overflow::Policy::Wrap,
        position(3 * here, 0, 1),
        d,
    );
    verdict(
        "S5",
        &format!(
            "the slot 20 wraps to {here}, sixty wraps to {direct}, and three times the wrapped \
             answer wraps to {staged}"
        ),
        here == -12 && direct == -4 && staged == -4,
    );

    // S6. And the hand-worked failure off the grid, which is what bounds the
    // region. The position one and a half floors to 1. Three times it is four and
    // a half, which the coordinate normalises to the slot 4 at a half and floors
    // to 4. Three times the floored answer is 3. Both are inside the range, so
    // nothing here is the completion's doing.
    let off_here = adapt_at(
        Range::SmallSigned,
        arvo_format::rounding::Mode::Floor,
        arvo_format::overflow::Policy::Wrap,
        position(1, 1, 2),
        d,
    );
    let off_direct = adapt_at(
        Range::SmallSigned,
        arvo_format::rounding::Mode::Floor,
        arvo_format::overflow::Policy::Wrap,
        position(3, 3, 2),
        d,
    );
    let off_staged = adapt_at(
        Range::SmallSigned,
        arvo_format::rounding::Mode::Floor,
        arvo_format::overflow::Policy::Wrap,
        position(3 * off_here, 0, 1),
        d,
    );
    verdict(
        "S6",
        &format!(
            "off the grid the two schedules differ: direct={off_direct} staged={off_staged} \
             from the floored answer {off_here}"
        ),
        off_here == 1 && off_direct == 4 && off_staged == 3,
    );
}
