//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! This file must NOT compile. The refusal is the finding.
//!
//! Run: `rustc --edition 2021 --crate-type lib p2_branch_a_cannot_express_two_carriers.rs`
//! Expected: `error[E0080]` (or the equivalent const-eval failure) on the
//! assertion below. A successful compile refutes the claim in the header.
//!
//! The claim: under branch A of `question::the_container_premise`, where the
//! carrier is not part of the type, one declared width is one type and therefore
//! one footprint. So the sentence "two carriers at one declared width" has no
//! expressible form in the branch-A shape, and the compiler is what says so
//! rather than a paragraph.
//!
//! This matters to the base-primitive design round because
//! `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
//! is an adequacy test on any answer to the premise, and a design that cannot
//! name a footprint cannot state one clause of `strategy::cold`'s intent either:
//! `ruling::cold_is_for_cold_paths_and_cold_storage` is ratified and its subject
//! is the footprint. An intent whose subject no signature can distinguish is not
//! a weak intent.

use core::mem::size_of;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FixedA<const I: usize, const F: usize>(u32);

// Branch A gives one type per declared width. Both aliases name the same type,
// because there is no second thing to name: the carrier is not a coordinate.
pub type Narrow13 = FixedA<13, 0>;
pub type Wide13 = FixedA<13, 0>;

// The sentence branch B can write and branch A cannot. If branch A could express
// two carriers at one declared width, this assertion would hold.
const _: () = assert!(
    size_of::<Narrow13>() != size_of::<Wide13>(),
    "if this compiles, the branch-A shape distinguishes two footprints at one \
     declared width, and the claim in this file's header is refuted"
);
