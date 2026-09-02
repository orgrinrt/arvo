//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where the bits sit. Nothing about what they mean.
//!
//! Given a declared signature, this derives the placement. The derivation is a
//! placement rather than a semantics: it chooses where the value goes and never
//! what the value is. Two placements of one declared signature compute the same
//! function, and they are distinguishable only through an observation this design
//! does not own, the host language's layout observation on a sized type.
//!
//! The output count moves with occupancy, and that is two arms over a declared
//! axis rather than one number with a special case. At sole occupancy there is
//! one output, the carrier. At shared occupancy there are three, the carrier and
//! the access width and the stride, and there is no per-element footprint to
//! observe at all: the element is reached through its carrier and no size-bounded
//! contract ranges over it.
//!
//! **One packing rule ships here and the design names two.** Under the rule below
//! the access width is not a function of the carrier, and the converse does not
//! hold: every access width reaches exactly one carrier. The design states the
//! independence in both directions, so that gap is real and is carried as a
//! catalogued red test rather than as a narrowed claim.
//!
//! Every width here is `arvo_format::Width` rather than a host integer. This
//! crate does not introduce the numeric category and is checked for that.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use arvo_format::{Bool, DeclaredSignature, Format, Slots, Width};

/// Whether a value has its carrier allocation to itself.
///
/// A declared axis rather than a detail, because the footprint is observable on
/// one side of it and does not exist on the other.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Occupancy {
    /// The value is the sole logical occupant of its allocation. Padding is
    /// permitted; sharing is not.
    Sole,
    /// The value shares its allocation with others.
    Shared,
}

/// A machine container a placement can put bits in.
///
/// The inventory is what the host offers and is not this crate's to invent.
pub trait Carrier {
    /// How many bits the container holds.
    const BITS: Width;
}

/// An eight-bit container.
pub struct Carrier8;
/// A sixteen-bit container.
pub struct Carrier16;
/// A thirty-two-bit container.
pub struct Carrier32;
/// A sixty-four-bit container.
pub struct Carrier64;

impl Carrier for Carrier8 {
    const BITS: Width = Width::bits(8);
}
impl Carrier for Carrier16 {
    const BITS: Width = Width::bits(16);
}
impl Carrier for Carrier32 {
    const BITS: Width = Width::bits(32);
}
impl Carrier for Carrier64 {
    const BITS: Width = Width::bits(64);
}

/// The containers this crate knows about, narrowest first.
///
/// The ladder runs out before an arbitrary declared width does, which is a real
/// bound rather than an oversight: a declared width past the widest entry has no
/// placement here, and closing that needs a decision about arbitrary widths that
/// the canon holds open.
pub const LADDER: [Width; 4] = [
    Width::bits(8),
    Width::bits(16),
    Width::bits(32),
    Width::bits(64),
];

/// What a placement is optimising for.
///
/// The derivation ladder is keyed on this rather than on a strategy name. A
/// strategy is a binding onto it, which is `arvo-strategy`'s subject, and keeping
/// the key here is what lets the strategy set be reshaped without the ladder
/// moving.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Objective {
    /// Smallest footprint. Bitpacks and shares allocations where it can.
    Footprint,
    /// Widest native access. Prefers a container the host handles directly.
    Access,
}

/// An objective as a type, which is how it reaches the derivation.
///
/// A const generic cannot carry it: a const argument may not depend on a generic
/// type parameter, so a consumer generic over the strategy could not write the
/// call at all, and the feature rustc suggests for that is forbidden here. A
/// marker composes as an ordinary type argument and its associated const is still
/// a constant after monomorphisation, so the branch erases either way.
pub trait ObjectiveKind {
    /// Which objective this marker names.
    const OBJECTIVE: Objective;
}

/// The markers, one per objective.
pub mod objective {
    use super::{Objective, ObjectiveKind};

    /// Smallest footprint. Packs at shared occupancy.
    pub struct Footprint;

    /// Widest native access. Does not pack.
    pub struct Access;

    impl ObjectiveKind for Footprint {
        const OBJECTIVE: Objective = Objective::Footprint;
    }

    impl ObjectiveKind for Access {
        const OBJECTIVE: Objective = Objective::Access;
    }
}

/// The placement of a declared signature.
///
/// One output at sole occupancy and three at shared.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Placement {
    /// Where the bits live.
    pub carrier: Width,
    /// How wide a read reaches. Equal to the carrier at sole occupancy.
    pub access: Width,
    /// How far to the next element. Equal to the carrier at sole occupancy.
    pub stride: Width,
    /// Which side of the axis this placement is on.
    pub occupancy: Occupancy,
}

impl Placement {
    /// How many outputs the derivation produced, which is the count that moves
    /// with occupancy.
    ///
    /// One at sole, three at shared. Stated as a function of the placement rather
    /// than as a comment, so a reader can check it.
    #[must_use]
    pub const fn output_count(&self) -> Width {
        match self.occupancy {
            Occupancy::Sole => Width::bits(1),
            Occupancy::Shared => Width::bits(3),
        }
    }

    /// Whether a per-element footprint observation exists.
    ///
    /// True at sole occupancy and false at shared. At a shared placement what is
    /// observable is the allocation's stride, which is a property of the column
    /// rather than of the element.
    #[must_use]
    pub const fn footprint_is_observable(&self) -> Bool {
        Bool::of(matches!(self.occupancy, Occupancy::Sole))
    }
}

/// How many bits a declared signature's slot range needs.
///
/// Reads the declared signature and nothing else, which is what makes the
/// derivation a function of the declaration.
#[must_use]
pub const fn declared_width<S: DeclaredSignature>() -> Width {
    // Read, not recovered. The declaration stated the width; deriving it back
    // from the slot bounds by counting and halving is a computation over a
    // quantity nothing needs, at the one place in the chain where the arithmetic
    // can leave the range it is carried in.
    <<S::Format as Format>::Slots as Slots>::WIDTH
}

/// The narrowest container on the ladder that holds a declared width.
///
/// Returns the absent width where the ladder runs out, which is the bound named
/// above rather than a silent widening.
#[must_use]
pub const fn narrowest_carrier(declared: Width) -> Width {
    let mut i = 0;
    while i < LADDER.len() {
        if LADDER[i].covers(declared).get() {
            return LADDER[i];
        }
        i += 1;
    }
    Width::NONE
}

/// Derive a placement at sole occupancy.
///
/// One output, and **both objectives give it**. The value is the sole logical
/// occupant of its allocation, so there is no other element to trade against:
/// the narrowest carrier that holds the declared width is at once the smallest
/// footprint and the widest native access. That is a result rather than a
/// collapse, and the tests assert the equality rather than leaving it to look
/// like an oversight.
///
/// The objective is a const parameter because the typestate holds it as a
/// compile-time fact. The branch resolves at monomorphisation and is gone before
/// the backend sees it.
#[must_use]
pub const fn derive_sole<S: DeclaredSignature, O: ObjectiveKind>() -> Placement {
    let declared = declared_width::<S>();
    let carrier = narrowest_carrier(declared);
    // Read so the parameter is not decorative, and so a marker whose associated
    // const disagreed with its own identity would still resolve here.
    let _ = O::OBJECTIVE;
    Placement {
        carrier,
        access: carrier,
        stride: carrier,
        occupancy: Occupancy::Sole,
    }
}

/// Derive a placement at shared occupancy, where the objective decides.
///
/// Three outputs, and this is where the two objectives differ, because three
/// outputs are what they have to differ in.
///
/// `Footprint` packs. The stride is the declared width, elements share carriers,
/// and a read has to reach wider because an element can straddle a boundary. That
/// is `ruling::cold_is_for_cold_paths_and_cold_storage`, which says the
/// storage-minimising objective aggressively minimises and bitpacks.
///
/// `Access` does not pack. One element per carrier, stride and access both the
/// carrier, padding rather than straddling, so every read is a single native
/// access and no shift pair assembles a value.
#[must_use]
pub const fn derive_shared<S: DeclaredSignature, O: ObjectiveKind>() -> Placement {
    let declared = declared_width::<S>();
    let carrier = narrowest_carrier(declared);
    // An associated const of a generic parameter, so this is a constant once the
    // call is monomorphised and the unselected arm is gone before the backend.
    match O::OBJECTIVE {
        Objective::Footprint => Placement {
            carrier,
            // An element of `declared` bits at an arbitrary offset can span one
            // more boundary than its width alone suggests.
            access: narrowest_carrier(declared.add(declared).less_one()),
            stride: declared,
            occupancy: Occupancy::Shared,
        },
        Objective::Access => Placement {
            carrier,
            access: carrier,
            stride: carrier,
            occupancy: Occupancy::Shared,
        },
    }
}

#[cfg(test)]
mod tests;
