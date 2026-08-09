//! Probe 4b: COMMITTED REFUSING, on purpose.
//!
//! A saturating composition (`Clamp` at both ends, the shipped `Warm`/`Cold`
//! shape) has no associativity law at any view: the delivered values diverge
//! across groupings, which probe 1 measures and which the consolidation
//! already records from an independent model (`26:126-137`). No published
//! grade makes that regrouping honest, so the mechanism refuses it outright
//! rather than letting it be published away.
//!
//! This is the half of the mechanism that has to be a hard refusal, and it is
//! the half my first draft got wrong by trying to make everything publishable.
//!
//! Verbatim diagnostic recorded in `OUTCOMES.md`.

#[path = "probe_4_view_as_a_return_type_and_the_transfer.rs"]
mod mechanism;

use mechanism::{regroup_fold, Folded};

// Clamp at both ends (resolution code 1), signed, arity 4, no headroom.
pub const SATURATING_REGROUPED: Folded<3> = regroup_fold::<1, 1, 1, 4, 0, 3>([1, 2, 3, 4]);
