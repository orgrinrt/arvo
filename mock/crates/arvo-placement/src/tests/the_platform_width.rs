//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The platform-width points land on the target's pointer-width carrier.
//!
//! In a file of its own because the parent had passed 500 lines, and because
//! these arms are the ones selected by the target rather than by the matrix.

use super::Sig;
use crate::{Occupancy, declared_width, derive_shared, derive_sole, objective};

/// The literal points the platform-width aliases name on this target, one arm
/// per pointer width, selected by the target rather than read from it, so the
/// comparison below is against a width written down and not the alias's own.
#[cfg(target_pointer_width = "64")]
mod pointer {
    pub(super) type Unsigned = arvo_format::points::UFixed<64, 0>;
    pub(super) type Signed = arvo_format::points::Integer<64>;
    pub(super) const WIDTH: arvo_format::Width = arvo_format::Width::bits(64);
}

// FIXME: compiled only by `cargo check --tests --target i686-unknown-linux-gnu` run
// by hand; no gate builds the suite at a 32-bit target.
#[cfg(target_pointer_width = "32")]
mod pointer {
    pub(super) type Unsigned = arvo_format::points::UFixed<32, 0>;
    pub(super) type Signed = arvo_format::points::Integer<32>;
    pub(super) const WIDTH: arvo_format::Width = arvo_format::Width::bits(32);
}

// FIXME: never compiled. A test harness needs `std` and no 16-bit target has one,
// so this arm stays unbuilt until a harness exists there.
#[cfg(target_pointer_width = "16")]
mod pointer {
    pub(super) type Unsigned = arvo_format::points::UFixed<16, 0>;
    pub(super) type Signed = arvo_format::points::Integer<16>;
    pub(super) const WIDTH: arvo_format::Width = arvo_format::Width::bits(16);
}

#[test]
fn the_platform_width_points_are_placed_alone_on_the_pointer_width_carrier() {
    // Placement reads the declared width and nothing else, so a platform-width
    // point lands on the carrier the target's own pointer width names, on every
    // target the ladder has a rung for.
    use arvo_format::points::{ISize, USize};
    let pointer = pointer::WIDTH;
    for p in [
        derive_sole::<Sig<USize>, objective::Footprint>(),
        derive_sole::<Sig<USize>, objective::Access>(),
        derive_sole::<Sig<ISize>, objective::Footprint>(),
        derive_sole::<Sig<ISize>, objective::Access>(),
    ] {
        assert_eq!(p.carrier, pointer);
        assert_eq!(p.access, pointer);
        assert_eq!(p.stride, pointer);
        assert_eq!(p.occupancy, Occupancy::Sole);
    }
    assert_eq!(declared_width::<Sig<USize>>(), pointer);
    assert_eq!(declared_width::<Sig<ISize>>(), pointer);
    // The same placement the explicit width derives, so nothing about the alias
    // is special to the ladder.
    assert_eq!(
        derive_shared::<Sig<USize>, objective::Footprint>(),
        derive_shared::<Sig<pointer::Unsigned>, objective::Footprint>()
    );
    assert_eq!(
        derive_shared::<Sig<ISize>, objective::Access>(),
        derive_shared::<Sig<pointer::Signed>, objective::Access>()
    );
}
