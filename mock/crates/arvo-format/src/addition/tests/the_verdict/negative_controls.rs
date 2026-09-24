//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Two narrower rules, each disagreeing with brute force where this verdict
//! does not.

use super::super::{Table, Window};
use super::Sat;
use crate::addition::addition_is_associative;
use crate::ambient::BinaryRationals;
use crate::apply::Dither;
use crate::format::Format;
use crate::points::{Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Slot, Slots};
use crate::symmetry::{Reach, completion_is_translation_homomorphic};
use crate::tests::grid::Grid;

/// The rule the additive row was first measured as: under saturation, the sum
/// is associative exactly when the range has no negative slot.
fn the_signedness_rule<F: Format>() -> bool {
    <F::Slots as Slots>::MIN.index() >= 0
}

#[test]
fn a_rule_keyed_on_signedness_disagrees_with_brute_force_where_the_verdict_does_not() {
    // A signed range whose sums leave it on one side only, and whose stored
    // operands never point back up: associative, and the rule says it is not.
    type F = Grid<BinaryRationals, Constant<0>, Window<-4, 0>, 0, 1>;
    assert_eq!(Table::of::<Sat<F>>(Dither::UNUSED).divergent(), 0);
    assert!(addition_is_associative::<Sat<F>>().get());
    assert!(!the_signedness_rule::<F>());
    // The control: where the rule is right it agrees, so the disagreement above
    // is about the range and not about the rule never answering yes.
    assert!(Table::of::<Sat<Integer<4>>>(Dither::UNUSED).divergent() > 0);
    assert!(!the_signedness_rule::<Integer<4>>());
    assert!(the_signedness_rule::<UFixed<4, 0>>());
}

#[test]
fn quantifying_over_the_ambient_domain_refuses_a_format_that_is_associative() {
    type S = Sat<UFixed<4, 0>>;
    assert_eq!(Table::of::<S>(Dither::UNUSED).divergent(), 0);
    assert!(addition_is_associative::<S>().get());
    // The same positions, with every translation the coordinate carries rather
    // than the stored operands: a negative translation points back into the
    // range from above, so the law is refused over a format where it holds.
    let positions = Reach::of(Slot::at(0), Slot::at(30));
    let over_the_ambient = positions
        .translated_by(Slot::at(i128::MIN), Slot::at(i128::MAX))
        .on_grid();
    assert!(!completion_is_translation_homomorphic::<S>(over_the_ambient).get());
    let over_stored = positions.translated_by(Slot::at(0), Slot::at(15)).on_grid();
    assert!(completion_is_translation_homomorphic::<S>(over_stored).get());
}
