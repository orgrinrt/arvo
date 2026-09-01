//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Does `has_additive_identity` agree with the design's membership predicate?
//!
//! An ad-hoc quick spike. Not a bench, nothing here is timed, and it decides no
//! fork: it establishes existence claims and refutes one universal.
//!
//! The design at `mock/crates/arvo-format/DESIGN.md.tmpl:29` says a value `v` is
//! representable exactly when there is a magnitude `m` and a slot `i` within the
//! slot range such that `v = phase + i * quantum(m)`. So zero is in the set
//! exactly when some admitted pair cancels the phase. The shipped
//! `has_additive_identity` evaluates that at `m = 0` and reads no coordinate of
//! the quantum at all.
//!
//! The arm is an enumerator over the real grid that never mentions the function
//! under test, so agreement between the two cannot be either one agreeing with
//! itself. That is the same control the canon's own phase probe used
//! (`probe::a_half_step_biased_grid_is_not_closed_under_addition`): a predicate
//! checked against an enumeration written independently of it.
//!
//! Run: `rustc --edition 2021 -O enumerate_against_the_predicate.rs -o p && ./p`

/// A format's coordinates, flattened out of the trait so the spike is standalone.
///
/// Spike scaffolding. The field names and the struct shape are chosen to reach
/// the check and are not a proposal about how anything should be spelled.
///
/// `base` is carried and deliberately never read, which the compiler reports and
/// which is itself a result: `BASE` cancels out of the cancellation equation
/// below, so the answer does not depend on it at any magnitude.
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
struct Coords {
    name: &'static str,
    radix: i128,
    base: i32,
    slope: i32,
    magnitudes: u32,
    slot_min: i64,
    slot_max: i64,
    phase_num: i64,
    phase_den: i64,
}

/// The shipped function, transcribed verbatim from
/// `mock/crates/arvo-format/src/format.rs:67`.
///
/// Transcribed rather than linked because the crate is a workspace member and a
/// standalone spike cannot resolve `notko.workspace = true`. The permanent form
/// of this check is a test inside the crate, against the real function.
fn shipped_has_additive_identity(c: &Coords) -> bool {
    if c.phase_den == 0 {
        return false;
    }
    c.phase_num % c.phase_den == 0 && slot_in_range(c, -(c.phase_num / c.phase_den))
}

fn slot_in_range(c: &Coords, slot: i64) -> bool {
    slot >= c.slot_min && slot <= c.slot_max
}

/// Whether `phase + i * quantum(m) == 0`, decided in exact integers.
///
/// The phase is `PHASE_NUM / PHASE_DEN` in units of the quantum at magnitude
/// zero, so with `q(m) = r^(BASE + SLOPE*m)` the equation
///
/// ```text
///   (PN/PD) * r^BASE + i * r^(BASE + SLOPE*m) = 0
/// ```
///
/// divides through by `r^BASE`, which is where `BASE` leaves the problem, and
/// clears the denominator to
///
/// ```text
///   i * PD * r^(SLOPE*m) = -PN
/// ```
///
/// which mentions no exponent offset and no rational arithmetic. Both sides are
/// evaluated in `i128` and any overflow returns `None` rather than wrapping, so
/// a `false` here is always "checked and not equal" and never "the arithmetic
/// gave up quietly".
fn cancels(c: &Coords, slot: i64, magnitude: u32) -> Option<bool> {
    let k = (c.slope as i64) * (magnitude as i64);
    let mut lhs = i128::from(slot).checked_mul(i128::from(c.phase_den))?;
    if k >= 0 {
        for _ in 0..k {
            lhs = lhs.checked_mul(c.radix)?;
        }
        Some(lhs == -i128::from(c.phase_num))
    } else {
        let mut rhs = -i128::from(c.phase_num);
        for _ in 0..(-k) {
            rhs = rhs.checked_mul(c.radix)?;
        }
        Some(lhs == rhs)
    }
}

/// The design's predicate, enumerated over every admitted pair.
///
/// Returns the witness so a disagreement can be printed rather than merely
/// counted. Nothing in here calls the function under test.
fn enumerated_contains_zero(c: &Coords) -> Option<(i64, u32)> {
    for m in 0..c.magnitudes {
        let mut i = c.slot_min;
        loop {
            if cancels(c, i, m) == Some(true) {
                return Some((i, m));
            }
            if i == c.slot_max {
                break;
            }
            i += 1;
        }
    }
    None
}

fn main() {
    // Widths kept small so the enumeration is exhaustive rather than sampled.
    let cases = [
        // --- the four shipped points, which is the control ------------------
        Coords {
            name: "Integer<4>",
            radix: 2,
            base: 0,
            slope: 0,
            magnitudes: 1,
            slot_min: -8,
            slot_max: 7,
            phase_num: 0,
            phase_den: 1,
        },
        Coords {
            name: "UFixed<4,-2>",
            radix: 2,
            base: -2,
            slope: 0,
            magnitudes: 1,
            slot_min: 0,
            slot_max: 15,
            phase_num: 0,
            phase_den: 1,
        },
        Coords {
            name: "Biased<4,0,1> (1/2)",
            radix: 2,
            base: 0,
            slope: 0,
            magnitudes: 1,
            slot_min: -8,
            slot_max: 7,
            phase_num: 1,
            phase_den: 2,
        },
        Coords {
            name: "Biased<4,0,2> (1)",
            radix: 2,
            base: 0,
            slope: 0,
            magnitudes: 1,
            slot_min: -8,
            slot_max: 7,
            phase_num: 2,
            phase_den: 2,
        },
        Coords {
            name: "Floating<4,-2,3>",
            radix: 2,
            base: -2,
            slope: 1,
            magnitudes: 3,
            slot_min: -8,
            slot_max: 7,
            phase_num: 0,
            phase_den: 1,
        },
        // --- a growing quantum with a whole phase out of reach at m = 0 ------
        // Every coordinate here is one the crate ships: `Indexed` is slope one
        // and `Signed<2>` is exactly this range, so reaching this needs an
        // outside `Format` and no outside `Quantum` at all. The phase is four
        // quanta. Slot -4 cancels it at m = 0 and `Signed<2>` stops at -2; at
        // m = 1 the quantum has doubled, so slot -2 cancels the same absolute
        // phase and is the range's own lowest index.
        Coords {
            name: "Indexed + Signed<2>, phase 4",
            radix: 2,
            base: 0,
            slope: 1,
            magnitudes: 2,
            slot_min: -2,
            slot_max: 1,
            phase_num: 4,
            phase_den: 1,
        },
        // --- a shrinking quantum, which is what SLOPE < 0 means --------------
        // Needs an outside `Quantum` too, since the crate ships slope 0 and 1
        // only. Phase one half: at m = 0 the cancelling slot is -1/2 and is not
        // a slot at all, and at m = 1 the quantum has halved, so the same
        // absolute phase is one whole step and slot -1 cancels it. This is the
        // case where a FRACTIONAL phase still leaves the identity on the grid,
        // which the design's gloss says cannot happen.
        Coords {
            name: "shrinking quantum, phase 1/2",
            radix: 2,
            base: 0,
            slope: -1,
            magnitudes: 2,
            slot_min: -8,
            slot_max: 7,
            phase_num: 1,
            phase_den: 2,
        },
        // --- no magnitudes at all, so the set is empty -----------------------
        Coords {
            name: "no magnitudes, phase 0",
            radix: 2,
            base: 0,
            slope: 0,
            magnitudes: 0,
            slot_min: -8,
            slot_max: 7,
            phase_num: 0,
            phase_den: 1,
        },
    ];

    let mut disagreements = 0;
    println!(
        "{:<34} {:>8} {:>12}  {}",
        "format", "shipped", "enumerated", "witness"
    );
    for c in &cases {
        let shipped = shipped_has_additive_identity(c);
        let witness = enumerated_contains_zero(c);
        let enumerated = witness.is_some();
        let mark = if shipped == enumerated {
            ""
        } else {
            "<- disagree"
        };
        println!(
            "{:<34} {:>8} {:>12}  {:<16?} {}",
            c.name, shipped, enumerated, witness, mark
        );
        if shipped != enumerated {
            disagreements += 1;
        }
    }

    println!();
    println!("disagreements: {disagreements}");

    // The negative control. If the enumerator answered `true` for everything, or
    // `false` for everything, the disagreements above would be an artifact of the
    // instrument rather than a fact about the function. It has to split the
    // shipped points, and it does: a zero phase is on the grid and a half-step
    // phase is not.
    assert!(
        enumerated_contains_zero(&cases[0]).is_some(),
        "the enumerator lost zero on an unbiased grid"
    );
    assert!(
        enumerated_contains_zero(&cases[2]).is_none(),
        "the enumerator found zero on a half-step grid"
    );
    println!("control: the enumerator separates a zero phase from a half-step phase");

    // And the second control: the enumerator has to agree with the shipped
    // function everywhere the shipped function is right, or "it disagrees" says
    // nothing about which one is wrong.
    for c in &cases[0..5] {
        assert_eq!(
            shipped_has_additive_identity(c),
            enumerated_contains_zero(c).is_some(),
            "the two disagree on {}, which is a shipped point and not a case under test",
            c.name
        );
    }
    println!("control: the two agree on all five shipped points");

    // The third control, for the arithmetic rather than for the verdict. `BASE`
    // cancels, so moving it alone must not move any answer. If it does, the
    // derivation above is wrong and every witness is suspect.
    for c in &cases {
        let mut shifted = *c;
        shifted.base = c.base + 7;
        assert_eq!(
            enumerated_contains_zero(c).is_some(),
            enumerated_contains_zero(&shifted).is_some(),
            "moving BASE moved the answer on {}, so BASE does not cancel",
            c.name
        );
    }
    println!("control: moving BASE by seven moves no answer, so BASE cancels");
}
