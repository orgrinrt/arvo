//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The platform-width points land on the target's pointer-width carrier.
//!
//! In a file of its own because the parent had passed 500 lines. The pointer
//! width is read from the alias's own declared slot width, which `arvo-format`'s
//! suite pins against the host's pointer-sized integers, so nothing here selects
//! an arm by target.

use arvo_format::Format;
use arvo_format::slots::declared_slot_width;

use super::Sig;
use crate::{Occupancy, derive_sole, objective};

#[test]
fn the_platform_width_points_are_placed_alone_on_the_pointer_width_carrier() {
    // Placement reads the declared width and nothing else, so a platform-width
    // point lands on the carrier its declared width names, on every target the
    // ladder has a rung for.
    use arvo_format::points::{ISize, USize};
    let unsigned = declared_slot_width::<<USize as Format>::Slots>();
    let signed = declared_slot_width::<<ISize as Format>::Slots>();
    assert_eq!(
        unsigned, signed,
        "the two points disagree on the pointer width"
    );
    for p in [
        derive_sole::<Sig<USize>, objective::Footprint>(),
        derive_sole::<Sig<USize>, objective::Access>(),
        derive_sole::<Sig<ISize>, objective::Footprint>(),
        derive_sole::<Sig<ISize>, objective::Access>(),
    ] {
        assert_eq!(p.carrier, unsigned);
        assert_eq!(p.access, unsigned);
        assert_eq!(p.stride, unsigned);
        assert_eq!(p.occupancy, Occupancy::Sole);
    }
}
