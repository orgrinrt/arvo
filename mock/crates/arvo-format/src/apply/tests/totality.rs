//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The map is total over every format shape the design admits.
//!
//! The design states totality for every mode, every policy and every format
//! shape, and the arm that stood in `apply/tests` asserted it at `Integer<5>`
//! with a few edge arms at `Integer<3>`. That is a law sampled at one point of
//! the width axis, and the widths it skipped are where the arithmetic comes
//! nearest the ends of the carrier. This walks every admitted width of both slot
//! families, and at each one feeds the edges of the range and the ends of the
//! coordinate, so the completion is entered from just outside and from as far
//! outside as a slot can be.
//!
//! The member's adapted door is the same map returned as a `Member`, so the walk
//! reads it beside `adapt` at every fed position and requires the two to agree,
//! and places every slot of the fed edges that lies in range on itself through
//! it, at every mode and policy.

use crate::adapt::DeclaredSignature;
use crate::apply::{Dither, Exact, Fraction, adapt};
use crate::format::Format;
use crate::member::Member;
use crate::overflow::SHIPPED_POLICIES;
use crate::rounding::ALL_MODES;
use crate::slots::{ADMITTED_WIDTHS, Slot, Slots};
use crate::tests::dispatch::{self, PerFormat, PerSignature};

/// The slots an arm feeds each signature.
///
/// Both ends of the coordinate and one in from each, the two slots either side
/// of each end of the range and the ends themselves, and zero.
fn edges(min: Slot, max: Slot) -> impl Iterator<Item = i128> {
    let (lo, hi) = (min.index(), max.index());
    [
        i128::MIN,
        i128::MIN + 1,
        lo - 2,
        lo - 1,
        lo,
        lo + 1,
        0,
        hi - 1,
        hi,
        hi + 1,
        hi + 2,
        i128::MAX - 1,
        i128::MAX,
    ]
    .into_iter()
}

/// The residues: on the grid, both sides of the midpoint and on it, a third,
/// and one part short of a whole slot, which rounds up at the top of the
/// coordinate to a position past it.
fn residues() -> impl Iterator<Item = (i64, i64)> {
    [(0, 1), (1, 4), (1, 2), (3, 4), (1, 3), (i64::MAX - 1, i64::MAX)].into_iter()
}

/// The dithers, so the stochastic mode is entered on both sides of a residue.
fn dithers() -> impl Iterator<Item = Dither> {
    [Dither::UNUSED, Dither::at(Fraction::of(1, 4)), Dither::at(Fraction::of(3, 4))].into_iter()
}

/// Adapt every fed position at one signature and require the answer to be
/// admitted. Answers how many positions it checked.
struct Total;

impl PerSignature for Total {
    type Out = u64;

    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> u64 {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let mut checked = 0u64;
        for slot in edges(min, max) {
            for (num, den) in residues() {
                for d in dithers() {
                    let e = Exact::between(Slot::at(slot), Fraction::of(num, den));
                    let got = adapt::<S>(e, d);
                    assert_eq!(
                        Member::<S::Format>::adapted::<S>(e, d).slot(),
                        got,
                        "the member door and the map disagree adapting {e:?} with {d:?}"
                    );
                    assert!(
                        got.is_within(min, max).get(),
                        "{got:?} is outside [{min:?}, {max:?}] adapting {e:?} with {d:?}"
                    );
                    checked += 1;
                }
            }
        }
        for slot in edges(min, max).filter(|&s| Slot::at(s).is_within(min, max).get()) {
            for d in dithers() {
                let on = Member::<S::Format>::adapted::<S>(Exact::on_grid(Slot::at(slot)), d);
                assert_eq!(
                    on.slot(),
                    Slot::at(slot),
                    "a member on the grid moved under {d:?}"
                );
                checked += 1;
            }
        }
        checked
    }
}

/// Every mode and policy at one format, counted.
struct EveryCell {
    cells:       usize,
    positions:   u64,
    widths_seen: [u32; 65],
    signed:      u32,
    unsigned:    u32,
}

impl PerFormat for EveryCell {
    #[inline(never)]
    fn run<F: Format>(&mut self) {
        let min = <F::Slots as Slots>::MIN;
        let max = <F::Slots as Slots>::MAX;
        // The control that the ends of the coordinate are outside the range, so
        // feeding them enters the completion rather than the in-range path.
        assert!(i128::MIN < min.index() && i128::MAX > max.index());
        self.widths_seen[<F::Slots as Slots>::WIDTH.count() as usize] += 1;
        if min.index() < 0 {
            self.signed += 1;
        } else {
            self.unsigned += 1;
        }
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                self.positions += dispatch::at::<F, Total>(mode, policy, &Total);
                self.cells += 1;
            }
        }
    }
}

#[test]
fn the_map_is_total_over_every_admitted_width_mode_and_policy() {
    // Every cell feeds 13 slots at 6 residues under 3 dithers through the map,
    // and the 5 of those slots that lie in range on the grid under the 3 dithers
    // through the member door.
    let mut walk = EveryCell {
        cells:       0,
        positions:   0,
        widths_seen: [0; 65],
        signed:      0,
        unsigned:    0,
    };
    dispatch::every_width(&mut walk);

    assert_eq!(
        walk.cells,
        64 * 2 * 6 * 3,
        "the walk is not every width of both families under every mode and policy"
    );
    // The walk names its widths from a literal list, so its coverage is checked
    // against the set the slot ladder admits rather than trusted.
    for width in ADMITTED_WIDTHS {
        assert_eq!(
            walk.widths_seen[width.count() as usize],
            2,
            "width {} was not walked once in each family",
            width.count()
        );
    }
    assert_eq!(
        walk.widths_seen.iter().sum::<u32>() as usize,
        2 * ADMITTED_WIDTHS.len(),
        "the walk reached a width the ladder does not admit"
    );
    assert_eq!(walk.signed, 64);
    assert_eq!(walk.unsigned, 64);
    assert_eq!(
        walk.positions,
        (walk.cells as u64) * (13 * 6 * 3 + 5 * 3),
        "a cell checked fewer positions than it was fed"
    );
}
