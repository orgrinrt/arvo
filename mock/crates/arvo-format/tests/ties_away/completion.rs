//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where a rounded position lands once the format's own range is applied, said
//! twice.
//!
//! The rounding half of the alias sweep is double-stated: `oracle_step` is the
//! rule and `brute_step` is the same rule rearranged, and an arm agrees them. The
//! completion is the other half of every expected value the sweep forms, and a
//! rearrangement error in it would move every cell at once, with nothing left to
//! disagree. What is here is the second statement of it, the machinery that
//! agrees the two, and wrong rearrangements planted so the agreement is known to
//! be capable of failing.
//!
//! The two are arranged differently on purpose. `complete` decides the saturating
//! case by comparing the slot against each end before the step is added, and
//! wraps by counting forward from `lo`. `brute` forms the position once and
//! clamps it, and wraps by counting back from `hi`. Neither is derived from the
//! other, which is the property that makes the agreement worth anything.
//!
//! Both are stated for a step of zero or one. That is what the rounding half
//! produces and nothing else, and a step of minus one is not a case they agree
//! on: `complete` carries a slot at `lo` down to `lo - 1` while `brute` clamps it
//! back. Rather than pick a reading nothing asks for, the domain is asserted at
//! the door of each.

/// One disagreement between two statements of the completion, kept as its parts
/// so a failure names the cell rather than a sentence somebody parses back.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Disagreement {
    /// The slot the position stood at.
    pub(crate) slot: i128,
    /// The step the rounding half produced there.
    pub(crate) step: i128,
    /// The bottom of the format's index.
    pub(crate) lo:   i128,
    /// The top of the format's index.
    pub(crate) hi:   i128,
    /// Whether the format's overflow policy wraps.
    pub(crate) wrap: bool,
    /// What `complete` said.
    pub(crate) want: i128,
    /// What the spelling said.
    pub(crate) got:  i128,
}

/// A spelling of the completion, which is what the planted set below holds.
pub(crate) type Spelling = fn(i128, i128, i128, i128, bool) -> i128;

/// The domain both statements are written for, asserted at the door of each.
fn stated_for(step: i128) {
    assert!(
        step == 0 || step == 1,
        "the completion is stated for a step of zero or one, not {step}"
    );
}

/// Where `slot + step` lands after the completion the format declares.
///
/// Saturating compares the slot against each end before the step is added, so
/// nothing wider than the index is formed at either end of the `i128`. Wrapping
/// counts forward from `lo`, reducing the slot and `lo` separately so the
/// difference stays inside the span.
pub(crate) fn complete(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    stated_for(step);
    if wrap {
        let span = hi - lo + 1;
        (slot.rem_euclid(span) - lo.rem_euclid(span) + step).rem_euclid(span) + lo
    } else if slot > hi || (slot == hi && step == 1) {
        hi
    } else if slot < lo && !(slot == lo - 1 && step == 1) {
        lo
    } else {
        slot + step
    }
}

/// The same completion, stated from the top of the index rather than the bottom.
///
/// Wrapping counts back from `hi`, and saturating forms the position once with a
/// checked add and clamps it. The only add that can leave the `i128` is a step of
/// one at `i128::MAX`, which runs off the top and so lands at `hi`.
pub(crate) fn brute(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    stated_for(step);
    if wrap {
        let span = hi - lo + 1;
        hi - (hi.rem_euclid(span) - slot.rem_euclid(span) - step).rem_euclid(span)
    } else {
        match slot.checked_add(step) {
            Some(at) => at.clamp(lo, hi),
            None => hi,
        }
    }
}

/// Every position the two are asked about, given the ranges the sweep runs over
/// as `(lo, hi, wrap)`.
///
/// Both ends of the `i128` are in it, because that is where the saturating
/// statement's checked add differs from the comparing statement's branches if
/// either is wrong, and both ends of each format's own index with margin,
/// because that is where those branches meet.
///
/// The band around zero is in it as well, because a signed index has its sign
/// change there and neither statement mentions the sign, and each end carries
/// three cells of margin rather than two so that a spelling off by two is not
/// sitting just outside what is asked.
pub(crate) fn positions(ranges: &[(i128, i128, bool)]) -> Vec<(i128, i128, i128, i128, bool)> {
    let mut out = Vec::new();
    for &(lo, hi, wrap) in ranges {
        let mut at = vec![i128::MIN, i128::MIN + 1, i128::MIN + 2];
        at.extend([i128::MAX - 2, i128::MAX - 1, i128::MAX]);
        at.extend(-4i128 ..= 4);
        for end in [lo, hi] {
            at.extend((-3i128 ..= 3).filter_map(|d| end.checked_add(d)));
        }
        at.sort_unstable();
        at.dedup();
        for slot in at {
            out.push((slot, 0, lo, hi, wrap));
            out.push((slot, 1, lo, hi, wrap));
        }
    }
    out
}

/// Whether a spelling agrees with `complete` at every position, as the number of
/// cells agreed or the first disagreement.
///
/// Returns rather than asserts, so an arm can require it to reject a wrong
/// spelling as readily as it requires it to accept the right one.
pub(crate) fn agrees(
    spell: Spelling,
    ranges: &[(i128, i128, bool)],
) -> Result<usize, Disagreement> {
    let mut cells = 0;
    for (slot, step, lo, hi, wrap) in positions(ranges) {
        let want = complete(slot, step, lo, hi, wrap);
        let got = spell(slot, step, lo, hi, wrap);
        if want != got {
            return Err(Disagreement {
                slot,
                step,
                lo,
                hi,
                wrap,
                want,
                got,
            });
        }
        cells += 1;
    }
    Ok(cells)
}

// --- the wrong rearrangements ----------------------------------------------------

/// Clamped before the step rather than after, which lands one past the top of the
/// index at the last cell.
fn clamped_first(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    if wrap {
        brute(slot, step, lo, hi, wrap)
    } else {
        slot.clamp(lo, hi).saturating_add(step)
    }
}

/// The span taken as `hi - lo`, one cell short, which is the off-by-one a
/// half-open reading of an inclusive index produces.
fn span_short(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    if !wrap {
        return brute(slot, step, lo, hi, wrap);
    }
    let span = hi - lo;
    hi - (hi.rem_euclid(span) - slot.rem_euclid(span) - step).rem_euclid(span)
}

/// Wrapped without the step, which drops the carry into the next cell and so
/// rounds nothing.
fn stepless(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    if !wrap {
        return brute(slot, step, lo, hi, wrap);
    }
    let span = hi - lo + 1;
    hi - (hi.rem_euclid(span) - slot.rem_euclid(span)).rem_euclid(span)
}

/// An add that leaves the `i128` taken to the bottom of the index rather than the
/// top, which is the wrong end of the only overflow that can happen.
fn overflow_to_the_floor(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    if wrap {
        return brute(slot, step, lo, hi, wrap);
    }
    match slot.checked_add(step) {
        Some(at) => at.clamp(lo, hi),
        None => lo,
    }
}

/// A saturating statement that forgets the step, which agrees wherever no tie
/// rounds up and is why the planted set needs a cell where one does.
fn saturate_stepless(slot: i128, step: i128, lo: i128, hi: i128, wrap: bool) -> i128 {
    if wrap { brute(slot, step, lo, hi, wrap) } else { slot.clamp(lo, hi) }
}

/// Every wrong rearrangement planted against the agreement, with its name.
pub(crate) const PLANTED: &[(&str, Spelling)] = &[
    ("clamped before the step", clamped_first),
    ("the span one cell short", span_short),
    ("wrapped without the step", stepless),
    ("the overflow taken to the floor", overflow_to_the_floor),
    ("saturated without the step", saturate_stepless),
];
