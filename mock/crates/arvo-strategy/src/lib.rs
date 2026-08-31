//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A name bound to what you are optimising for.
//!
//! A strategy is a binding and not a switch. It maps a name onto a placement
//! objective, which is what the derivation ladder is keyed on, and onto an
//! adaptation. It names no implementation and owns none.
//!
//! The set is open. Four presets ship and they are implementors of a concept
//! rather than members of an enumeration, so a fifth needs no edit to anything
//! here. What each is for is carried as an intent and not as a table of
//! behaviours, because the canon says the per-strategy statements are not to be
//! written down as clear cut and settled.
//!
//! Which overflow mode each preset names is not written here. That question is
//! open in the registry, and filling it inside a design is how an open question
//! gets closed where nobody can see it happen.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use arvo_format::Adaptation;
use arvo_placement::Objective;

/// A name bound to an objective and an adaptation selection.
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

/// The intent behind a preset, in the words the canon has for it.
///
/// Carried as prose on purpose. The canon says these are not to be written down
/// as clear cut and settled, so there is no table of behaviours keyed on a name
/// and this is what stands in its place: something a reader can read and nothing
/// a compiler can gate on.
///
/// Debug builds only. A static string is not part of this stack's release
/// surface, and an intent is documentation rather than something the lowered
/// program needs, so gating it is what the shape asks for rather than a
/// concession to a lint.
#[cfg(debug_assertions)]
pub trait StatedIntent {
    /// What this preset is for, as an intent rather than a rule.
    const INTENT: &'static str;
}

/// The presets the corpus carries.
///
/// Kept because a prior design can name the parts well and go wrong in execution,
/// and nothing here has found these four wrong. They are instances, not the
/// inventory.
pub mod presets {
    #[cfg(debug_assertions)]
    use super::StatedIntent;
    use super::Strategy;
    use arvo_format::overflow::{Saturate, Wrap};
    use arvo_format::rounding::{HalfEven, TowardZero};
    use arvo_format::Adapt;
    use arvo_placement::Objective;

    /// The speed-first binding.
    pub struct Hot;

    impl Strategy for Hot {
        const OBJECTIVE: Objective = Objective::Access;
        type Adaptation = Adapt<TowardZero, Wrap>;
    }

    #[cfg(debug_assertions)]
    impl StatedIntent for Hot {
        const INTENT: &'static str = "Performance and efficiency, even at the cost of accuracy or \
             soundness. Sacrificing soundness is its explicit purpose rather than a tolerated \
             defect, but it should not lose soundness for nothing: the price is a provable \
             meaningful gain. What counts as meaningful is unset and nobody has set it.";
    }

    /// The storage-minimising binding.
    pub struct Cold;

    impl Strategy for Cold {
        const OBJECTIVE: Objective = Objective::Footprint;
        type Adaptation = Adapt<TowardZero, Wrap>;
    }

    #[cfg(debug_assertions)]
    impl StatedIntent for Cold {
        const INTENT: &'static str = "Cold paths and cold storage. It aggressively minimises and \
             bitpacks and stays small for memory or disk. Because the path is cold it has leeway \
             to be inefficient, and it is not obliged to take it: it may use the same paths the \
             speed-first binding uses wherever nothing in its intent fights them. It is not \
             deprioritised, and that survives the set being reshaped, renamed or resized.";
    }

    /// The accuracy-first binding.
    pub struct Precise;

    impl Strategy for Precise {
        const OBJECTIVE: Objective = Objective::Access;
        type Adaptation = Adapt<HalfEven, Saturate>;
    }

    #[cfg(debug_assertions)]
    impl StatedIntent for Precise {
        const INTENT: &'static str =
            "Sacrifices as much performance and efficiency as makes sense \
             to reach the most precise answer, throwing out both the speed and the footprint \
             optimisations, and especially within chains rather than only per operation. Its \
             objective had no measurement in the panel and that gap is recorded rather than \
             papered over.";
    }

    /// The compromise binding, meant as the sensible default.
    pub struct Warm;

    impl Strategy for Warm {
        const OBJECTIVE: Objective = Objective::Access;
        type Adaptation = Adapt<HalfEven, Wrap>;
    }

    #[cfg(debug_assertions)]
    impl StatedIntent for Warm {
        const INTENT: &'static str = "The intuitive best choice for most every use case. The \
             intuitive part demands it mimics, and being a Rust crate makes Rust's way the \
             baseline for what a reader finds intuitive, but that is a baseline and not a \
             definition: mimicry is dropped where following it is consistently the worse choice.";
    }
}

#[cfg(test)]
mod tests;
