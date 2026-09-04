//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 02. Does the completion region commute with translation, and where.
//!
//! The question is `complete(y + c) == complete(complete(y) + c)` for a value `y`
//! that may sit outside the range and a translation `c` that is representable.
//! The inner completion throws away how far outside `y` was, and the law asks
//! whether a later translation ever needed that.
//!
//! It never does under wrapping, because reduction modulo the span is a
//! homomorphism of the additive group. Under saturation it does exactly when a
//! translation can point back at the range from the side the excursion left on.
//! So the region is not "unsigned" and is not "non-negative": it is a statement
//! about which excursion sides the values reach crossed with which signs the
//! translations carry, and this step measures those separately.
//!
//! Completion is isolated by handing the map a position already on the grid,
//! where no mode has anything to decide. `C1` measures that the answer does not
//! move with the mode rather than assuming it.

use the_adaptation_contract_under_translation::{
    MODES,
    POLICIES,
    Range,
    complete_only,
    completion_is_mode_blind,
    verdict,
};

/// Which excursions a walk lets its values make.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Excursions {
    /// Values on both sides of the range and inside it.
    Both,
    /// Nothing below the lowest admitted index.
    NoneBelow,
    /// Nothing above the highest.
    NoneAbove,
    /// Values inside the range only.
    Neither,
}

impl Excursions {
    const ALL: [Excursions; 4] = [
        Excursions::Both,
        Excursions::NoneBelow,
        Excursions::NoneAbove,
        Excursions::Neither,
    ];

    fn admits(self, y: i64, lo: i64, hi: i64) -> bool {
        match self {
            Excursions::Both => true,
            Excursions::NoneBelow => y >= lo,
            Excursions::NoneAbove => y <= hi,
            Excursions::Neither => y >= lo && y <= hi,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Excursions::Both => "both-sides",
            Excursions::NoneBelow => "none-below",
            Excursions::NoneAbove => "none-above",
            Excursions::Neither => "inside-only",
        }
    }
}

/// What a walk over one range, one policy and one excursion set found.
struct Walk {
    homomorphic:           bool,
    witness:               Option<(i64, i64, i64, i64)>,
    pairs:                 u64,
    reached_below:         u64,
    reached_above:         u64,
    negative_translations: u64,
    positive_translations: u64,
}

fn walk(range: Range, policy: arvo_format::overflow::Policy, excursions: Excursions) -> Walk {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let mut out = Walk {
        homomorphic:           true,
        witness:               None,
        pairs:                 0,
        reached_below:         0,
        reached_above:         0,
        negative_translations: 0,
        positive_translations: 0,
    };
    // Two spans past each end, so an excursion can be further out than one
    // translation could ever pull it back and also near enough that one could.
    for y in (lo - 2 * span) ..= (hi + 2 * span) {
        if !excursions.admits(y, lo, hi) {
            continue;
        }
        if y < lo {
            out.reached_below += 1;
        }
        if y > hi {
            out.reached_above += 1;
        }
        // The translation is a representable value, which is what a slot in the
        // range is.
        for c in lo ..= hi {
            if c < 0 {
                out.negative_translations += 1;
            }
            if c > 0 {
                out.positive_translations += 1;
            }
            out.pairs += 1;
            let direct = complete_only(range, policy, y + c);
            let staged = complete_only(range, policy, complete_only(range, policy, y) + c);
            if direct != staged {
                out.homomorphic = false;
                if out.witness.is_none() {
                    out.witness = Some((y, c, direct, staged));
                }
            }
        }
    }
    out
}

fn main() {
    println!("step 02: does the completion region commute with translation");
    println!("values two spans past each end, translations over the whole representable range");
    println!();

    let mut mode_blind_failures = 0u32;
    let mut any_homomorphic = false;
    let mut any_broken = false;

    for range in Range::SMALL {
        let (lo, hi) = range.bounds();
        println!(
            "range {range:?}: {lo} to {hi}, reaches negatives = {}, symmetric = {}",
            range.reaches_negatives(),
            range.is_symmetric()
        );
        for &policy in &POLICIES {
            for excursions in Excursions::ALL {
                let w = walk(range, policy, excursions);
                println!(
                    "  cell policy={policy:?} excursions={} homomorphic={} pairs={} below={} above={} neg_c={} pos_c={} witness={}",
                    excursions.name(),
                    w.homomorphic,
                    w.pairs,
                    w.reached_below,
                    w.reached_above,
                    w.negative_translations,
                    w.positive_translations,
                    match w.witness {
                        None => "none".to_string(),
                        Some((y, c, direct, staged)) =>
                            format!("y={y} c={c} direct={direct} staged={staged}"),
                    }
                );
                if excursions == Excursions::Both {
                    if w.homomorphic {
                        any_homomorphic = true;
                    } else {
                        any_broken = true;
                    }
                }
            }
        }
        // The isolation control, per range: the completion's answer on a grid
        // position does not move with the rounding mode.
        for &policy in &POLICIES {
            for y in (lo - 2) ..= (hi + 2) {
                if !completion_is_mode_blind(range, policy, y) {
                    mode_blind_failures += 1;
                }
            }
        }
        println!();
    }

    println!("--- the controls ---");

    verdict(
        "C1",
        "the completion's answer on a grid position does not move with the rounding mode, so \
         the completion region is what was measured",
        mode_blind_failures == 0,
    );
    verdict(
        "C2",
        "the instrument separates: some policy commutes over both-sided excursions and some \
         does not",
        any_homomorphic && any_broken,
    );

    // C3. The vacuous walk. With the values restricted to the range the inner
    // completion is the identity, so every policy commutes and the verdict says
    // nothing about the policy. Reported so a `homomorphic=true` in the
    // `inside-only` column is read as what it is.
    let vacuous_all_yes = Range::SMALL.iter().all(|&range| {
        POLICIES
            .iter()
            .all(|&policy| walk(range, policy, Excursions::Neither).homomorphic)
    });
    verdict(
        "C3",
        "with no excursion at all every policy commutes, which is what a vacuous restriction \
         looks like",
        vacuous_all_yes,
    );

    // C4. The witness on the signed range under saturation, worked out by hand
    // and asserted rather than read off the sweep.
    //
    // `Integer<5>` admits -16 to 15. Take y = 31, which is 16 above the top, and
    // c = -16, which is representable. Directly: 31 - 16 = 15, in range, so 15.
    // Staged: 31 saturates to 15, then 15 - 16 = -1, in range, so -1. The clamp
    // threw away the 16 the translation was about to give back.
    let direct = complete_only(
        Range::SmallSigned,
        arvo_format::overflow::Policy::Saturate,
        31 - 16,
    );
    let staged = complete_only(
        Range::SmallSigned,
        arvo_format::overflow::Policy::Saturate,
        complete_only(
            Range::SmallSigned,
            arvo_format::overflow::Policy::Saturate,
            31,
        ) - 16,
    );
    verdict(
        "C4",
        &format!(
            "the hand-worked signed witness is direct = 15 and staged = -1, got {direct} and {staged}"
        ),
        direct == 15 && staged == -1,
    );

    // C5. The witness on the unsigned range, which is the one an argument from
    // "the translations are all non-negative" misses.
    //
    // `UFixed<5, 0>` admits 0 to 31, so every translation is non-negative and no
    // high excursion can be undone. A low excursion can: y = -5, c = 10.
    // Directly: 5, in range. Staged: -5 clamps to 0, then 0 + 10 = 10.
    let u_direct = complete_only(
        Range::SmallUnsigned,
        arvo_format::overflow::Policy::Saturate,
        -5 + 10,
    );
    let u_staged = complete_only(
        Range::SmallUnsigned,
        arvo_format::overflow::Policy::Saturate,
        complete_only(
            Range::SmallUnsigned,
            arvo_format::overflow::Policy::Saturate,
            -5,
        ) + 10,
    );
    verdict(
        "C5",
        &format!(
            "the unsigned range still breaks on a low excursion: direct = 5 and staged = 10, \
             got {u_direct} and {u_staged}"
        ),
        u_direct == 5 && u_staged == 10,
    );

    // C6. And the restriction that repairs it, stated as the claim it is: on the
    // unsigned range with nothing below the bottom, saturation commutes.
    let repaired = walk(
        Range::SmallUnsigned,
        arvo_format::overflow::Policy::Saturate,
        Excursions::NoneBelow,
    );
    verdict(
        "C6",
        &format!(
            "unsigned saturation commutes once nothing goes below the bottom, over {} pairs \
             of which {} were above the top",
            repaired.pairs, repaired.reached_above
        ),
        repaired.homomorphic && repaired.reached_above > 0,
    );

    // C7. The same restriction does not repair the signed range, which is what
    // says C6 is about the sign of the translations rather than about the
    // restriction.
    let not_repaired = walk(
        Range::SmallSigned,
        arvo_format::overflow::Policy::Saturate,
        Excursions::NoneBelow,
    );
    verdict(
        "C7",
        "the same restriction leaves the signed range broken, so C6 is about which signs the \
         translations carry",
        !not_repaired.homomorphic,
    );

    // C8. Wrapping commutes in every cell measured, which is the homomorphism
    // and is the arm a design would gate on.
    let wrap_everywhere = Range::SMALL.iter().all(|&range| {
        Excursions::ALL
            .iter()
            .all(|&e| walk(range, arvo_format::overflow::Policy::Wrap, e).homomorphic)
    });
    verdict(
        "C8",
        "wrapping commutes over every range and every excursion set measured",
        wrap_everywhere,
    );

    // C9. And every mode agrees with that, since the walk above ran one mode.
    let modes_seen = MODES.len();
    verdict(
        "C9",
        &format!("the mode-blindness control ran all {modes_seen} modes"),
        modes_seen == 6,
    );
}
