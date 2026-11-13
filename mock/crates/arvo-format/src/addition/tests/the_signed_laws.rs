//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws negation and subtraction owe beside addition's, and the composed
//! count the registry records.
//!
//! The identity is the slot that cancels the phase, and it has to be neutral on
//! the right of a difference and fixed by negation. A member less itself lands
//! at minus the phase, negation undoes itself wherever it lands on a member, and
//! a difference is the sum with the negation wherever the negation lands on a
//! slot at all. Each is stated at the exact step, which is where it holds at
//! every phase, and the identity also through the adapted step, which is where a
//! consumer meets it.
//!
//! The count is `law::associativity_of_a_composed_saturating_add_and_subtract`,
//! measured at eight unsigned bits under saturation, and the same tables under
//! wrapping are its control: there the composed law holds everywhere, because
//! wrapping is a ring homomorphism onto the residues.

use notko::Maybe;

use super::steps::{Difference, Sum};
use super::{DITHERS, Ratio, Window, at_every_signature, coordinates, members, stepped};
use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::addition::{difference_position, neg, negation_position, sub, sum_position};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Dither, Exact};
use crate::format::{Format, cancelling_slot, has_additive_identity};
use crate::overflow::{Saturate, Wrap};
use crate::points::{Biased, Integer, UFixed};
use crate::quantum::{Constant, Magnitude};
use crate::rounding::Floor;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::{PerFormat, PerSignature};
use crate::tests::grid::Grid;

// --- the identity --------------------------------------------------------------

/// The identity's slot on the right of a difference, and under negation, at one
/// signature.
struct Neutral(Slot);

impl PerSignature for Neutral {
    type Out = u64;

    fn run<S: DeclaredSignature>(&self) -> u64 {
        let mut checked = 0u64;
        for dither in DITHERS {
            assert_eq!(neg::<S>(self.0, dither), self.0, "-identity");
            for a in members::<S::Format>() {
                assert_eq!(sub::<S>(a, self.0, dither), a, "{a:?} - identity");
                checked += 1;
            }
        }
        checked
    }
}

/// The identity laws at one format that has an identity.
fn the_identity_is_neutral<F: Format>() -> u64 {
    assert!(has_additive_identity::<F>().get());
    let Maybe::Is(identity) = cancelling_slot::<F>(Magnitude::SMALLEST) else {
        panic!(
            "{:?} has an identity and no cancelling slot",
            coordinates::<F>()
        );
    };
    at_every_signature::<F, Neutral>(&Neutral(identity))
}

#[test]
fn the_identity_is_neutral_on_the_right_of_a_difference_and_fixed_by_negation() {
    let checked = the_identity_is_neutral::<Integer<4>>()
        + the_identity_is_neutral::<UFixed<4, 0>>()
        + the_identity_is_neutral::<Biased<4, 0, 2>>()
        + the_identity_is_neutral::<Grid<BinaryRationals, Constant<0>, Signed<4>, -2, 1>>()
        + the_identity_is_neutral::<Grid<DecimalRationals, Constant<3>, Signed<4>, 3, 1>>()
        + the_identity_is_neutral::<Grid<BinaryRationals, Constant<0>, Window<-3, 2>, 1, 1>>();
    let owed = super::signatures() as u64 * DITHERS.len() as u64 * (16 + 16 + 16 + 16 + 16 + 6);
    assert_eq!(checked, owed);
}

// --- the exact laws ------------------------------------------------------------

/// The three exact laws at one format, counted by how often each had a case.
#[derive(Default)]
struct ExactLaws {
    formats:      u32,
    self_less:    u64,
    involutions:  u64,
    as_sums:      u64,
    off_the_grid: u64,
}

impl PerFormat for ExactLaws {
    fn run<F: Format>(&mut self) {
        let min = <F::Slots as Slots>::MIN;
        let max = <F::Slots as Slots>::MAX;
        let minus_phase = Ratio::whole(0).minus(Ratio::phase_of::<F>()).position();
        let name = coordinates::<F>();
        self.formats += 1;
        for a in members::<F>() {
            // A member less itself is the value zero, which sits at minus the
            // phase whether or not that position is a member.
            assert_eq!(
                difference_position::<F>(a, a),
                minus_phase,
                "{a:?} - {a:?} at {name:?}"
            );
            self.self_less += 1;

            let negated = negation_position::<F>(a);
            if !negated.is_on_grid().get() {
                self.off_the_grid += 1;
                continue;
            }
            let n = negated.slot();
            if n.is_within(min, max).get() {
                assert_eq!(
                    negation_position::<F>(n),
                    Exact::on_grid(a),
                    "-(-{a:?}) at {name:?}"
                );
                self.involutions += 1;
            }
            for b in members::<F>() {
                assert_eq!(
                    difference_position::<F>(b, a),
                    sum_position::<F>(b, n),
                    "{b:?} - {a:?} against {b:?} + {n:?} at {name:?}"
                );
                self.as_sums += 1;
            }
        }
    }
}

#[test]
fn a_difference_is_a_sum_with_the_negation_and_negation_undoes_itself() {
    let mut walk = ExactLaws::default();
    every_phase!(walk; Signed<4>);
    every_phase!(walk; Unsigned<4>);
    every_phase!(walk; Window<-3, 2>);
    walk.run::<Biased<4, 0, 1>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 5, 4>>();
    walk.run::<Grid<DecimalRationals, Constant<-2>, Signed<4>, 7, 2>>();

    assert_eq!(walk.formats, 3 * super::phases() as u32 + 3);
    assert_eq!(
        walk.self_less,
        (2 * 16 + 6) * super::phases() as u64 + 3 * 16
    );
    // Every law had cases, and a phase whose double is not whole put some
    // negations off the grid, where neither of the last two laws has a slot to
    // be stated at.
    assert!(walk.involutions > 0 && walk.as_sums > 0 && walk.off_the_grid > 0);
}

// --- the composed count --------------------------------------------------------

/// How many triples `(a + b) - c` and `a + (b - c)` disagree on under `S`.
fn composed_divergent<S: DeclaredSignature>() -> u64 {
    let sums = stepped::<Sum, S>(Dither::UNUSED);
    let differences = stepped::<Difference, S>(Dither::UNUSED);
    let mut count = 0u64;
    for a in 0 .. sums.members() {
        for b in 0 .. sums.members() {
            let ab = sums.at(a, b);
            for c in 0 .. sums.members() {
                if differences.at(ab, c) != sums.at(a, differences.at(b, c)) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[test]
fn the_composed_count_the_canon_records_is_reproduced() {
    type Sat<F> = Signature<F, Adapt<Floor, Saturate>>;
    type Wr<F> = Signature<F, Adapt<Floor, Wrap>>;
    // 13,882,880 of the 16,777,216 triples at eight unsigned bits, the 82.7
    // percent the registry's evidence row is named for.
    assert_eq!(composed_divergent::<Sat<UFixed<8, 0>>>(), 13_882_880);
    // The control: the same tables under wrapping keep the law everywhere, so
    // the count above is the saturation and not the instrument.
    assert_eq!(composed_divergent::<Wr<UFixed<8, 0>>>(), 0);
    assert_eq!(composed_divergent::<Wr<Integer<8>>>(), 0);
}
