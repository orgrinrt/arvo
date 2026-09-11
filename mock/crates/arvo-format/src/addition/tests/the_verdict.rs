//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The associativity verdict against brute force, and the counts the canon
//! records.
//!
//! Brute force here is the triple walk over every member of the range, under one
//! signature and one dither, and a cell is associative when no triple diverges
//! at any dither the sweep carries. The same dither is handed to every addition
//! in a triple, which is the reading the verdict is stated for: the stochastic
//! mode's offset is the dither's, and the verdict holds for every dither at once.
//!
//! The cross runs every interval with both ends between minus four and four, so
//! ranges without zero in them are crossed as often as ranges with it, under the
//! shared phase list, beside the shipped points at the widths the walk affords.
//! A licensed cell never diverges. At a whole phase over two slots or more the
//! verdict is exact both ways. Where the phase is fractional the verdict refuses
//! some cells that turn out associative anyway, and that count is pinned so a
//! change to it is seen rather than absorbed.

use super::{DITHERS, phases, signatures, Table, Window};
use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::addition::addition_is_associative;
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::Dither;
use crate::format::Format;
use crate::overflow::{SHIPPED_POLICIES, Saturate, Wrap};
use crate::points::{Biased, Integer, UFixed};
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Floor};
use crate::slots::{Signed, Slot, Slots};
use crate::symmetry::{Reach, completion_is_translation_homomorphic};
use crate::tests::dispatch::{self, PerFormat, PerSignature};
use crate::tests::grid::Grid;

/// The saturating signature over `F`, under the floor.
type Sat<F> = Signature<F, Adapt<Floor, Saturate>>;

/// The wrapping signature over `F`, under the floor.
type Wr<F> = Signature<F, Adapt<Floor, Wrap>>;

// --- the counts ----------------------------------------------------------------

#[test]
fn the_counts_the_canon_records_are_reproduced() {
    assert_eq!(Table::of::<Sat<Integer<4>>>(Dither::UNUSED).divergent(), 952);
    assert_eq!(Table::of::<Wr<Integer<4>>>(Dither::UNUSED).divergent(), 0);
    assert_eq!(Table::of::<Sat<UFixed<4, 0>>>(Dither::UNUSED).divergent(), 0);
    assert_eq!(Table::of::<Wr<UFixed<4, 0>>>(Dither::UNUSED).divergent(), 0);
    assert_eq!(Table::of::<Sat<Integer<8>>>(Dither::UNUSED).divergent(), 4_177_792);

    assert!(!addition_is_associative::<Sat<Integer<4>>>().get());
    assert!(addition_is_associative::<Wr<Integer<4>>>().get());
    assert!(addition_is_associative::<Sat<UFixed<4, 0>>>().get());
    assert!(!addition_is_associative::<Sat<Integer<8>>>().get());
}

/// The count and the verdict at one format, saturating, recorded per format.
struct AtEveryScale {
    counts:   [u64; 64],
    verdicts: [bool; 64],
    formats:  usize,
}

impl AtEveryScale {
    /// Nothing recorded yet.
    fn new() -> Self {
        Self {
            counts:   [0; 64],
            verdicts: [false; 64],
            formats:  0,
        }
    }
}

impl PerFormat for AtEveryScale {
    fn run<F: Format>(&mut self) {
        self.counts[self.formats] = Table::of::<Sat<F>>(Dither::UNUSED).divergent();
        self.verdicts[self.formats] = addition_is_associative::<Sat<F>>().get();
        self.formats += 1;
    }
}

/// Every exponent from minus eight to eight, at one ambient and phase, over four
/// signed bits.
macro_rules! every_scale {
    ($walk:ident; $ambient:ty, $pn:literal, $pd:literal) => {
        every_scale!(@ $walk; $ambient, $pn, $pd;
            -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8);
    };
    (@ $walk:ident; $ambient:ty, $pn:literal, $pd:literal; $($e:literal),+) => {
        $( $walk.run::<Grid<$ambient, Constant<{ $e }>, Signed<4>, { $pn }, { $pd }>>(); )+
    };
}

#[test]
fn the_count_and_the_verdict_do_not_move_with_the_fraction_width_or_the_radix() {
    let mut whole = AtEveryScale::new();
    every_scale!(whole; BinaryRationals, 0, 1);
    every_scale!(whole; DecimalRationals, 0, 1);
    assert_eq!(whole.formats, 34);
    assert!(whole.counts[.. 34].iter().all(|&c| c == 952), "{:?}", &whole.counts[.. 34]);
    assert!(whole.verdicts[.. 34].iter().all(|&v| !v));

    // The same at a fractional phase, where the count is whatever it is at one
    // scale and has to be that at every other.
    let mut third = AtEveryScale::new();
    every_scale!(third; BinaryRationals, 1, 3);
    every_scale!(third; DecimalRationals, 1, 3);
    assert_eq!(third.formats, 34);
    let first = third.counts[0];
    assert!(third.counts[.. 34].iter().all(|&c| c == first), "{:?}", &third.counts[.. 34]);
    assert!(third.verdicts[.. 34].iter().all(|&v| v == third.verdicts[0]));
}

// --- the verdict against brute force -------------------------------------------

/// One cell: whether the verdict licenses it, and whether brute force finds it
/// associative at every dither.
struct Cell;

impl PerSignature for Cell {
    type Out = (bool, bool);

    fn run<S: DeclaredSignature>(&self) -> (bool, bool) {
        let licensed = addition_is_associative::<S>().get();
        let associative = DITHERS.iter().all(|&d| Table::of::<S>(d).divergent() == 0);
        (licensed, associative)
    }
}

/// The cross, tallied by phase class.
#[derive(Default)]
struct Cross {
    cells:                          usize,
    licensed_divergent:             u64,
    whole_licensed:                 u64,
    whole_refused:                  u64,
    whole_refused_associative:      u64,
    single_slot_refused_associative: u64,
    fractional_licensed:            u64,
    fractional_refused:             u64,
    fractional_refused_associative: [[u64; 3]; 6],
    fractional_refused_by_cell:     [[u64; 3]; 6],
}

impl PerFormat for Cross {
    fn run<F: Format>(&mut self) {
        let whole = F::PHASE.is_whole_multiple().get();
        let span = <F::Slots as Slots>::MAX.index() - <F::Slots as Slots>::MIN.index();
        for (m, mode) in ALL_MODES.into_iter().enumerate() {
            for (p, policy) in SHIPPED_POLICIES.into_iter().enumerate() {
                let (licensed, associative) = dispatch::at::<F, Cell>(mode, policy, &Cell);
                self.cells += 1;
                if licensed && !associative {
                    self.licensed_divergent += 1;
                }
                match (whole, licensed) {
                    (true, true) => self.whole_licensed += 1,
                    (true, false) => {
                        self.whole_refused += 1;
                        if associative && span >= 1 {
                            self.whole_refused_associative += 1;
                        } else if associative {
                            self.single_slot_refused_associative += 1;
                        }
                    },
                    (false, true) => self.fractional_licensed += 1,
                    (false, false) => {
                        self.fractional_refused += 1;
                        self.fractional_refused_by_cell[m][p] += 1;
                        if associative {
                            self.fractional_refused_associative[m][p] += 1;
                        }
                    },
                }
            }
        }
    }
}

/// Every interval named, each under every phase.
macro_rules! every_window {
    ($walk:ident; $( ($lo:literal, $hi:literal) )+) => {
        $( every_phase!($walk; Window<{ $lo }, { $hi }>); )+
    };
}

#[test]
fn a_licensed_cell_never_diverges_and_a_whole_phase_verdict_is_exact_both_ways() {
    let mut cross = Cross::default();
    every_window!(cross;
        (-4, -4) (-4, -3) (-4, -2) (-4, -1) (-4, 0) (-4, 1) (-4, 2) (-4, 3) (-4, 4)
        (-3, -3) (-3, -2) (-3, -1) (-3, 0) (-3, 1) (-3, 2) (-3, 3) (-3, 4)
        (-2, -2) (-2, -1) (-2, 0) (-2, 1) (-2, 2) (-2, 3) (-2, 4)
        (-1, -1) (-1, 0) (-1, 1) (-1, 2) (-1, 3) (-1, 4)
        (0, 0) (0, 1) (0, 2) (0, 3) (0, 4)
        (1, 1) (1, 2) (1, 3) (1, 4)
        (2, 2) (2, 3) (2, 4)
        (3, 3) (3, 4)
        (4, 4)
    );
    cross.run::<Integer<1>>();
    cross.run::<Integer<2>>();
    cross.run::<Integer<3>>();
    cross.run::<Integer<4>>();
    cross.run::<Integer<5>>();
    cross.run::<Integer<6>>();
    cross.run::<UFixed<1, 0>>();
    cross.run::<UFixed<2, 0>>();
    cross.run::<UFixed<3, 0>>();
    cross.run::<UFixed<4, 0>>();
    cross.run::<UFixed<5, 0>>();
    cross.run::<UFixed<6, 0>>();
    cross.run::<Biased<2, 0, 1>>();
    cross.run::<Biased<3, 0, 1>>();
    cross.run::<Biased<4, 0, 1>>();
    cross.run::<Biased<5, 0, 1>>();
    cross.run::<Biased<6, 0, 1>>();
    cross.run::<Biased<3, 0, 2>>();
    cross.run::<Biased<4, -2, -3>>();
    cross.run::<Biased<5, 3, 4>>();

    assert_eq!(cross.cells, (45 * phases() + 20) * signatures());
    assert_eq!(cross.licensed_divergent, 0, "a licensed cell diverged");
    assert_eq!(
        cross.whole_refused_associative, 0,
        "a whole-phase refusal over two slots or more named no divergent triple"
    );
    // Both answers are reached in both classes, so neither half of the verdict
    // is compared against nothing.
    assert!(cross.whole_licensed > 0 && cross.whole_refused > 0);
    assert!(cross.fractional_licensed > 0 && cross.fractional_refused > 0);
    // The cost of the fractional verdict, pinned per cell so a change in the
    // verdict or the map is seen and so is where it landed. Rows are the modes in
    // the order `ALL_MODES` lists them, toward zero, floor, ceil, half up, half
    // even and stochastic, and columns the policies in the order
    // `SHIPPED_POLICIES` lists them, wrap, saturate and clamp.
    //
    // The first table is every fractional refusal, the second the refusals that
    // are associative anyway. The three modes that read nothing besides the
    // residue have none of the second kind, so over this cross their verdict is
    // exact both ways at a fractional phase too. Every associative refusal is in
    // a mode reading the sign or the parity, where the offset moves across the
    // reach and the verdict refuses without asking whether the moving offset
    // composes anyway.
    assert_eq!(
        cross.fractional_refused_by_cell,
        [[101, 101, 101], [0, 53, 53], [0, 52, 52], [36, 67, 67], [51, 82, 82], [0, 63, 63]]
    );
    assert_eq!(
        cross.fractional_refused_associative,
        [[38, 38, 38], [0, 0, 0], [0, 0, 0], [10, 14, 14], [17, 22, 22], [0, 0, 0]]
    );
    // A range of one slot is associative whatever the map does, and the verdict
    // licenses every such cell, so the exactness claim above is not resting on
    // the span bound it is stated with.
    assert_eq!(cross.single_slot_refused_associative, 0);
}

// --- the negative controls -----------------------------------------------------

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
    let over_the_ambient = positions.translated_by(Slot::at(i64::MIN), Slot::at(i64::MAX)).on_grid();
    assert!(!completion_is_translation_homomorphic::<S>(over_the_ambient).get());
    let over_stored = positions.translated_by(Slot::at(0), Slot::at(15)).on_grid();
    assert!(completion_is_translation_homomorphic::<S>(over_stored).get());
}
