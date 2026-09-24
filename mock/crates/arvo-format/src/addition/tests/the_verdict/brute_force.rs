//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The verdict crossed against the triple walk directly: every interval with
//! both ends between minus four and four, under the shared phase list, beside
//! the shipped points at the widths the walk affords.
//!
//! A licensed cell never diverges. At a whole phase over two slots or more the
//! verdict is exact both ways. Where the phase is fractional the verdict refuses
//! some cells that turn out associative anyway, and that count is pinned so a
//! change to it is seen rather than absorbed.

use super::super::{DITHERS, Table, Window, phases, signatures};
use crate::adapt::DeclaredSignature;
use crate::ambient::BinaryRationals;
use crate::format::Format;
use crate::overflow::SHIPPED_POLICIES;
use crate::points::{Biased, Integer, UFixed};
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::Slots;
use crate::tests::dispatch::{self, PerFormat, PerSignature};
use crate::tests::grid::Grid;

/// One cell: whether the verdict licenses it, and whether brute force finds it
/// associative at every dither.
struct Cell;

impl PerSignature for Cell {
    type Out = (bool, bool);

    fn run<S: DeclaredSignature>(&self) -> (bool, bool) {
        let licensed = crate::addition::addition_is_associative::<S>().get();
        let associative = DITHERS.iter().all(|&d| Table::of::<S>(d).divergent() == 0);
        (licensed, associative)
    }
}

/// The cross, tallied by phase class.
#[derive(Default)]
struct Cross {
    cells: usize,
    licensed_divergent: u64,
    whole_licensed: u64,
    whole_refused: u64,
    whole_refused_associative: u64,
    single_slot_refused_associative: u64,
    fractional_licensed: u64,
    fractional_refused: u64,
    fractional_refused_associative: [[u64; 3]; 6],
    fractional_refused_by_cell: [[u64; 3]; 6],
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
    // verdict or the map is seen and so is where it landed. Columns are the
    // policies in the order `SHIPPED_POLICIES` lists them, wrap, saturate and
    // clamp. A row is looked up by its mode rather than written out in the order
    // `ALL_MODES` happens to list them, so each assertion below says which mode
    // it is about and a change to that order moves none of them.
    let index = |mode: Mode| {
        ALL_MODES
            .iter()
            .position(|&m| m == mode)
            .expect("the mode is one of the six")
    };
    let refused = |mode: Mode| cross.fractional_refused_by_cell[index(mode)];
    let refused_associative = |mode: Mode| cross.fractional_refused_associative[index(mode)];

    // Half up refuses exactly the cells ceil does, since both add a fixed offset
    // and the offset is all the verdict reads. That is the relation, and it is
    // asserted as one rather than as a second copy of ceil's numbers, which is
    // the form that would go on agreeing after the two stopped being one rule.
    assert_eq!(
        refused(Mode::HalfUp),
        refused(Mode::Ceil),
        "half up and ceil add the same fixed offset, so the verdict reads them alike"
    );
    // And the relation says something, because the neighbouring row differs.
    assert_ne!(refused(Mode::HalfUp), refused(Mode::Floor));

    // The four modes reading nothing besides the residue have no refusal that is
    // associative anyway, so over this cross their verdict is exact both ways at
    // a fractional phase too. Asserted over the group, since it is a property of
    // what they read rather than four separate coincidences.
    for mode in [Mode::Floor, Mode::Ceil, Mode::HalfUp, Mode::Stochastic] {
        assert_eq!(
            refused_associative(mode),
            [0, 0, 0],
            "{mode:?} reads nothing besides the residue, so a refusal of its cells \
             should name a divergent triple"
        );
    }
    // The two reading the sign or the parity do have them, which is what that
    // group claim is against: the offset moves across the reach and the verdict
    // refuses without asking whether the moving offset composes anyway. Pinned as
    // measured, so a change to the verdict or the map is seen.
    assert_eq!(refused_associative(Mode::TowardZero), [38, 38, 38]);
    assert_eq!(refused_associative(Mode::HalfEven), [17, 22, 22]);

    // Every fractional refusal, by mode, as measured.
    assert_eq!(refused(Mode::TowardZero), [101, 101, 101]);
    assert_eq!(refused(Mode::Floor), [0, 53, 53]);
    assert_eq!(refused(Mode::Ceil), [0, 52, 52]);
    assert_eq!(refused(Mode::HalfEven), [51, 82, 82]);
    assert_eq!(refused(Mode::Stochastic), [0, 63, 63]);
    // A range of one slot is associative whatever the map does, and the verdict
    // licenses every such cell, so the exactness claim above is not resting on
    // the span bound it is stated with.
    assert_eq!(cross.single_slot_refused_associative, 0);
}
