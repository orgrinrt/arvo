//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The rounding axis moves the sum exactly where the phase is not whole.
//!
//! The last of the laws `the_operation` states over small formats and every
//! phase the shared list names, in a file of its own because that one had passed
//! 500 lines.

use super::{DITHERS, Sums, Window, coordinates};
use crate::ambient::BinaryRationals;
use crate::apply::Dither;
use crate::format::Format;
use crate::overflow::SHIPPED_POLICIES;
use crate::points::{Biased, Integer};
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Signed, Unsigned};
use crate::tests::dispatch::{self, PerFormat};
use crate::tests::grid::Grid;

/// Every mode's table against the floor's, at one format and every policy.
#[derive(Default)]
struct Axis {
    whole:      u32,
    fractional: u32,
}

impl PerFormat for Axis {
    fn run<F: Format>(&mut self) {
        let name = coordinates::<F>();
        let whole = F::PHASE.is_whole_multiple().get();
        for policy in SHIPPED_POLICIES {
            let floor = dispatch::at::<F, Sums>(Mode::Floor, policy, &Sums(Dither::UNUSED));
            if whole {
                for mode in ALL_MODES {
                    for dither in DITHERS {
                        let other = dispatch::at::<F, Sums>(mode, policy, &Sums(dither));
                        assert!(
                            other == floor,
                            "{mode:?} moved a sum at {name:?}, {policy:?}"
                        );
                    }
                }
            } else {
                let ceil = dispatch::at::<F, Sums>(Mode::Ceil, policy, &Sums(Dither::UNUSED));
                assert!(
                    ceil != floor,
                    "floor and ceil agree at {name:?}, {policy:?}"
                );
            }
        }
        if whole {
            self.whole += 1;
        } else {
            self.fractional += 1;
        }
    }
}

#[test]
fn the_rounding_mode_moves_the_sum_exactly_where_the_phase_is_not_whole() {
    let mut walk = Axis::default();
    every_phase!(walk; Signed<4>);
    every_phase!(walk; Unsigned<4>);
    every_phase!(walk; Window<-3, 2>);
    walk.run::<Integer<4>>();
    walk.run::<Biased<4, 0, 1>>();
    assert_eq!(walk.whole, 3 * 3 + 1);
    assert_eq!(walk.fractional, 3 * 3 + 1);
}
