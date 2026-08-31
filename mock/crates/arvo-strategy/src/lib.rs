//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! A name bound to what you are optimising for.
//!
//! A strategy is a binding and not a switch. It maps a name onto a placement
//! objective, which is what the derivation ladder is keyed on, and onto an
//! adaptation. It names no implementation and owns none.
//!
//! The set is open. Four presets ship and they are implementors of a concept
//! rather than members of an enumeration, so a fifth needs no edit to anything
//! here.
//!
//! **What each preset is for is written as rustdoc on the preset and is not a
//! trait item.** An intent is something a reader reads and nothing a compiler
//! gates on, so it is documentation; a `const` in a trait is the opposite of
//! that, and carrying one made a per-preset compile-time item keyed on the
//! preset, which is nearer the behaviour table the canon forbids than prose is.
//!
//! Which overflow mode each preset names is not written here. That question is
//! open in the registry, and filling it inside a design is how an open question
//! gets closed where nobody can see it happen.

use arvo_format::Adaptation;
use arvo_placement::Objective;

/// A name bound to an objective and an adaptation selection.
///
/// Two items and no third. A third keyed on which preset this is would be the
/// behaviour table again, and the compiler is what enforces the count: adding an
/// item breaks every impl with `E0046` before any test runs.
///
/// Open: an implementor outside this crate is a strategy this crate does not know
/// about, which is the intended shape and not a gap.
pub trait Strategy {
    /// What the placement ladder is keyed on for this binding.
    const OBJECTIVE: Objective;

    /// The adaptation this binding selects.
    ///
    /// An associated type rather than a const, because an adaptation is a pair of
    /// types and a strategy selects it at monomorphisation.
    type Adaptation: Adaptation;
}

/// The objective a strategy binds to.
///
/// Reached through the binding rather than around it, so a consumer never has to
/// know which preset it holds to know what the ladder will do.
#[must_use]
pub const fn objective_of<S: Strategy>() -> Objective {
    S::OBJECTIVE
}

/// The presets the corpus carries.
///
/// Kept because a prior design can name the parts well and go wrong in execution,
/// and nothing here has found these four wrong. They are instances, not the
/// inventory.
pub mod presets {
    use super::Strategy;
    use arvo_format::overflow::{Saturate, Wrap};
    use arvo_format::rounding::{HalfEven, TowardZero};
    use arvo_format::Adapt;
    use arvo_placement::Objective;

    /// The speed-first binding.
    ///
    /// Performance and efficiency, even at the cost of accuracy or soundness.
    /// Sacrificing soundness is its explicit purpose rather than a tolerated
    /// defect, but it should not lose soundness for nothing: the price is a
    /// provable meaningful gain. What counts as meaningful is unset and nobody
    /// has set it.
    pub struct Hot;

    impl Strategy for Hot {
        const OBJECTIVE: Objective = Objective::Access;
        type Adaptation = Adapt<TowardZero, Wrap>;
    }

    /// The storage-minimising binding.
    ///
    /// Cold paths and cold storage. It aggressively minimises and bitpacks and
    /// stays small for memory or disk. Because the path is cold it has leeway to
    /// be inefficient, and it is not obliged to take it: it may use the same
    /// paths the speed-first binding uses wherever nothing in its intent fights
    /// them. It is not deprioritised, and that survives the set being reshaped,
    /// renamed or resized.
    pub struct Cold;

    impl Strategy for Cold {
        const OBJECTIVE: Objective = Objective::Footprint;
        type Adaptation = Adapt<TowardZero, Wrap>;
    }

    /// The accuracy-first binding.
    ///
    /// Sacrifices as much performance and efficiency as makes sense to reach the
    /// most precise answer, throwing out both the speed and the footprint
    /// optimisations, and especially within chains rather than only per
    /// operation. Its objective had no measurement in the panel and that gap is
    /// recorded rather than papered over.
    pub struct Precise;

    impl Strategy for Precise {
        const OBJECTIVE: Objective = Objective::Access;
        type Adaptation = Adapt<HalfEven, Saturate>;
    }

    /// The compromise binding, meant as the sensible default.
    ///
    /// The intuitive best choice for most every use case. The intuitive part
    /// demands it mimics, and being a Rust crate makes Rust's way the baseline
    /// for what a reader finds intuitive, but that is a baseline and not a
    /// definition: mimicry is dropped where following it is consistently the
    /// worse choice.
    pub struct Warm;

    impl Strategy for Warm {
        const OBJECTIVE: Objective = Objective::Access;
        type Adaptation = Adapt<HalfEven, Wrap>;
    }
}

#[cfg(test)]
mod tests;
