//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where the placement's carrier meets a machine integer.
//!
//! Which ladder container a slot family sits in alone is the placement's
//! answer, `arvo_placement::SoleCarrier`, read here and not restated. What this
//! crate adds is the host's integer of that container's size, one row per
//! container, and it is always the unsigned one: the sign is the slot family's,
//! read from its range where an operation dispatches.

use arvo_placement::{Carrier, Carrier8, Carrier16, Carrier32, Carrier64};

use crate::machine::Machine;

/// A ladder container, and the machine integer of its size.
///
/// Public in a module the crate does not export, so `Sole` can name it in its
/// supertrait list while no consumer can name it at all. The four rows are the
/// containers the ladder offers, which are what the host offers.
pub trait Held: Carrier {
    /// The unsigned machine integer of the container's size.
    type Repr: Machine;
}

impl Held for Carrier8 {
    type Repr = u8;
}

impl Held for Carrier16 {
    type Repr = u16;
}

impl Held for Carrier32 {
    type Repr = u32;
}

impl Held for Carrier64 {
    type Repr = u64;
}

/// Each row's integer is as wide as the container it holds.
const _: () = {
    const fn agrees<C: Held>() -> bool {
        core::mem::size_of::<C::Repr>() as u32 * 8 == C::BITS.count()
    }
    assert!(
        agrees::<Carrier8>()
            && agrees::<Carrier16>()
            && agrees::<Carrier32>()
            && agrees::<Carrier64>(),
        "a machine integer is not the size of the container it holds"
    );
};
