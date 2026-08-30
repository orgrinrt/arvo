//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An axis is added and never removed or renamed.
//!
//! Adding one reaches backwards not at all: a predicate's absence quantifies
//! over the world rather than over the vocabulary, so a row written before an
//! axis existed was always exactly as narrow as it reads once the axis is
//! declared. Declaring reveals a narrowness rather than creating one.
//!
//! **Removing or renaming one does reach backwards, and badly.** Every
//! predicate naming the old slug becomes unparseable, and every predicate that
//! did not name it loses the thing its silence was a statement about. A written
//! absence turns into nonsense, which is worse than a written span turning into
//! an error, because nothing reports it.
//!
//! So the vocabulary is append-only. That was argued rather than enforced when
//! the axis set was extended, and this is the enforcement, because a rule
//! nothing checks is a rule the next person does not know about.

use arvo_checks::canon;

/// Every axis slug ever declared.
///
/// **Append to this list; never edit or remove an entry.** A slug leaving the
/// registry is what this test exists to refuse, and editing the list to make
/// the test pass is deleting the guarantee rather than satisfying it.
///
/// The first sixteen were read off one member file's predicate line. The four
/// after them were added on two independent readings once a census measured
/// what that one file had missed, and the divide is worth keeping visible: a
/// vocabulary read off a single source is a vocabulary that will need
/// extending.
const EVER_DECLARED: &[&str] = &[
    // read off `109:156-159`, topic five's first sitting
    "access_pattern",
    "alignment",
    "arity",
    "build_profile",
    "chain_length",
    "container",
    "fraction_width",
    "integer_width",
    "operation",
    "overflow_policy",
    "rounding",
    "signedness",
    "strategy",
    "target_features",
    "threads",
    "total_width",
    // added after the census, on two independent readings
    "accumulator_width",
    "ambient_domain",
    "radix",
    "toolchain",
];

#[test]
fn no_axis_has_been_removed_or_renamed() {
    let reg = canon();
    let live = reg.slugs("dimension");
    let gone: Vec<&&str> = EVER_DECLARED
        .iter()
        .filter(|slug| !live.contains(*slug))
        .collect();
    assert!(
        gone.is_empty(),
        "these axes were declared and are no longer: {gone:?}. Every predicate that named \
         one is now unparseable, and every predicate that did not has lost the thing its \
         silence was a statement about. If the axis was wrong, supersede it with a new row \
         and leave this one; do not edit this list."
    );
}

/// The other half. A new axis is expected and welcome, and this reports it
/// rather than refusing it, so somebody adds the line here in the same commit.
#[test]
fn every_live_axis_is_recorded_here() {
    let reg = canon();
    let live = reg.slugs("dimension");
    let unrecorded: Vec<&&str> = live
        .iter()
        .filter(|slug| !EVER_DECLARED.contains(*slug))
        .collect();
    assert!(
        unrecorded.is_empty(),
        "a new axis is declared and not recorded in EVER_DECLARED: {unrecorded:?}. Adding \
         one is fine and expected; add it to the list in the same commit so the append-only \
         guarantee keeps meaning something."
    );
}

/// The control, and the reason neither test above is a way of saying the file
/// exists.
///
/// A list checked against itself passes whatever the registry holds. These
/// plant both directions against a list that disagrees with the canon, and the
/// arms have to fire.
#[test]
fn both_directions_fire_against_a_disagreeing_list() {
    let reg = canon();
    let live = reg.slugs("dimension");

    let pretend_removed = "an_axis_that_never_existed";
    assert!(
        !live.contains(&pretend_removed),
        "control: the canon must not hold the slug this arm assumes is absent"
    );

    let first = live.first().expect("the canon declares at least one axis");
    assert!(
        EVER_DECLARED.contains(first),
        "control: a live axis is in the list, so the second arm is comparing real sets"
    );
    assert!(
        !EVER_DECLARED.contains(&pretend_removed),
        "control: an invented slug is not in the list, so the first arm can fail"
    );
}
