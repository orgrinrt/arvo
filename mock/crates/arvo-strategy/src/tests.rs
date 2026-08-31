//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws the design names, over every preset rather than the one that was
//! convenient.

use crate::presets::{Cold, Hot, Precise, Warm};
use crate::{objective_of, Strategy};
use arvo_format::overflow::{Policy, Wrap};
use arvo_format::rounding::{Mode, TowardZero};
use arvo_format::{overflow_of, rounding_of, Adapt};
use arvo_placement::{derive_sole, Objective};

#[cfg(debug_assertions)]
use crate::StatedIntent;

// --- the control -------------------------------------------------------------

#[test]
fn the_control_the_objectives_are_distinguishable() {
    assert_ne!(
        Objective::Footprint,
        Objective::Access,
        "if the objectives are the same value, every binding below binds to one \
         thing and the tests are vacuous"
    );
}

// --- a strategy is a binding, and reaches the ladder through the objective ----

#[test]
fn every_preset_binds_to_an_objective() {
    assert_eq!(objective_of::<Hot>(), Objective::Access);
    assert_eq!(objective_of::<Cold>(), Objective::Footprint);
    assert_eq!(objective_of::<Warm>(), Objective::Access);
    assert_eq!(objective_of::<Precise>(), Objective::Access);
}

#[test]
fn the_storage_binding_is_the_one_that_binds_to_footprint() {
    // The ratified intent is that the storage-minimising path is not
    // deprioritised, and the binding is where that shows up: it is the preset that
    // reaches the footprint objective, and no other does.
    let to_footprint = [
        objective_of::<Hot>(),
        objective_of::<Cold>(),
        objective_of::<Warm>(),
        objective_of::<Precise>(),
    ]
    .into_iter()
    .filter(|o| *o == Objective::Footprint)
    .count();
    assert_eq!(to_footprint, 1);
    assert_eq!(objective_of::<Cold>(), Objective::Footprint);
}

#[test]
fn every_preset_selects_an_adaptation() {
    // Reached through the binding, so a consumer never has to know which preset it
    // holds to know what adaptation it gets.
    assert_eq!(
        rounding_of::<<Hot as Strategy>::Adaptation>(),
        Mode::TowardZero
    );
    assert_eq!(overflow_of::<<Hot as Strategy>::Adaptation>(), Policy::Wrap);
    assert_eq!(
        rounding_of::<<Warm as Strategy>::Adaptation>(),
        Mode::HalfEven
    );
    assert_eq!(
        rounding_of::<<Precise as Strategy>::Adaptation>(),
        Mode::HalfEven
    );
    assert_eq!(
        overflow_of::<<Precise as Strategy>::Adaptation>(),
        Policy::Saturate
    );
}

// --- the set is open, and this is what that means --------------------------

/// A strategy declared outside the shipped presets.
///
/// If this compiles, adding one needed no edit to any existing item, which is
/// the whole content of "the set is not closed".
struct Bespoke;

impl Strategy for Bespoke {
    const OBJECTIVE: Objective = Objective::Footprint;
    type Adaptation = Adapt<TowardZero, Wrap>;
}

#[cfg(debug_assertions)]
impl StatedIntent for Bespoke {
    const INTENT: &'static str = "A consumer's own binding, admitted by supplying the obligations.";
}

#[test]
fn the_strategy_inventory_admits_a_member_this_crate_does_not_know_about() {
    assert_eq!(objective_of::<Bespoke>(), Objective::Footprint);
    assert_eq!(
        rounding_of::<<Bespoke as Strategy>::Adaptation>(),
        Mode::TowardZero
    );
}

#[test]
fn a_new_strategy_reaches_the_ladder_the_same_way_a_preset_does() {
    // The binding is the whole interface, so a strategy this crate never heard of
    // derives a placement exactly as a shipped one does. If a preset had a
    // privileged route, this would not hold.
    use arvo_format::points::Integer;
    use arvo_format::rounding::Floor;
    use arvo_format::{Adapt as A, Signature};

    type Sig = Signature<Integer<13>, A<Floor, Wrap>>;

    let from_preset = derive_sole::<Sig>(objective_of::<Cold>());
    let from_bespoke = derive_sole::<Sig>(objective_of::<Bespoke>());
    assert_eq!(from_preset, from_bespoke);
}

// --- the presets carry intents and not behaviour tables ----------------------

#[cfg(debug_assertions)]
#[test]
fn every_preset_states_an_intent() {
    for intent in [
        <Hot as StatedIntent>::INTENT,
        <Cold as StatedIntent>::INTENT,
        <Warm as StatedIntent>::INTENT,
        <Precise as StatedIntent>::INTENT,
    ] {
        assert!(!intent.is_empty());
        assert!(
            intent.len() > 80,
            "an intent that fits in a label is a behaviour table with the columns \
             removed, which is the shape the canon says not to write"
        );
    }
}

#[cfg(debug_assertions)]
#[test]
fn the_intents_are_four_distinct_statements() {
    let intents = [
        <Hot as StatedIntent>::INTENT,
        <Cold as StatedIntent>::INTENT,
        <Warm as StatedIntent>::INTENT,
        <Precise as StatedIntent>::INTENT,
    ];
    for i in 0..intents.len() {
        for j in 0..intents.len() {
            if i != j {
                assert_ne!(intents[i], intents[j]);
            }
        }
    }
}

#[test]
fn no_preset_carries_a_behaviour_keyed_on_its_own_name() {
    // Structural. `Strategy` has exactly two items, an objective and an
    // adaptation, and neither is keyed on which preset this is. A third item
    // naming a per-strategy behaviour would break this file.
    //
    // Asserted by exercising both items on every preset through the same generic
    // path: if a preset had behaviour of its own, this function could not be
    // written once and applied to all four.
    fn both_items<S: Strategy>() -> (Objective, Mode) {
        (objective_of::<S>(), rounding_of::<S::Adaptation>())
    }

    let _ = both_items::<Hot>();
    let _ = both_items::<Cold>();
    let _ = both_items::<Warm>();
    let _ = both_items::<Precise>();
    let _ = both_items::<Bespoke>();
}

// --- the selection is const --------------------------------------------------

const _HOT_OBJECTIVE: Objective = objective_of::<Hot>();
const _COLD_OBJECTIVE: Objective = objective_of::<Cold>();

#[test]
fn the_binding_is_readable_at_const_time() {
    // The const bindings above are the assertion: an arm selects at
    // monomorphisation, so the objective has to be reachable there. A runtime-only
    // binding would not compile as a const, and this body only checks the values
    // are also right.
    assert_eq!(_HOT_OBJECTIVE, Objective::Access);
    assert_eq!(_COLD_OBJECTIVE, Objective::Footprint);
}
