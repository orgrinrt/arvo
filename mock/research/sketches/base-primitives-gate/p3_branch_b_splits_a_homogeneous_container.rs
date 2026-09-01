//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! This file must NOT compile. The refusal is the finding.
//!
//! Run: `rustc --edition 2021 --crate-type lib p3_branch_b_splits_a_homogeneous_container.rs`
//! Expected: `error[E0308]` mismatched types on the array literal.
//!
//! The claim: under branch B, where the carrier is part of the type, two
//! selections at one declared width are not substitutable inside a homogeneous
//! container. That is the third arm of the cost
//! `proposal::an_axis_the_realisation_map_does_not_read_is_not_a_type_parameter`
//! prices, in its own words "no repair at a homogeneous container", and it is
//! the arm that lands on the storage path `ruling::cold_is_for_cold_paths_and_cold_storage`
//! exists for.
//!
//! Why it belongs to the gate rather than to a design: the consequence is
//! consumer-visible and it differs between the branches, so a base-primitive
//! design cannot be written without choosing which consequence its consumers
//! live with. Choosing is answering `question::the_container_premise`, which is
//! open, whose `decider` is the panel, and whose six answering rows are one
//! persona's by that persona's own statement.

pub trait Carrier {
    type Repr: Copy;
}

#[derive(Clone, Copy)]
pub struct Narrow;
#[derive(Clone, Copy)]
pub struct Wide;

impl Carrier for Narrow {
    type Repr = u16;
}
impl Carrier for Wide {
    type Repr = u32;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FixedB<const I: usize, const F: usize, S: Carrier>(S::Repr);

// One declared width, 13 bits, two selections. A homogeneous container over the
// declared width is what a storage column is, and branch B refuses to build one
// from both selections. If this compiles, the claim in the header is refuted.
pub fn a_column_over_one_declared_width() -> [FixedB<13, 0, Narrow>; 2] {
    [FixedB::<13, 0, Narrow>(0u16), FixedB::<13, 0, Wide>(0u32)]
}
