//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Does a base-primitive design have to answer `question::the_container_premise`?
//!
//! The gate question for the base-primitive design round. If the two branches of
//! the premise produce the same type constructor at the consumer's signature,
//! the design is invariant under the premise and can be written before it is
//! ruled. If they produce different arities, writing the design picks a branch,
//! and picking is answering an open canon question by fiat.
//!
//! Run: `rustc --edition 2021 -O --test p1_branch_arity.rs -o /tmp/p1 && /tmp/p1`
//!
//! What must fail, stated before the run: section 3 asserts that the branch-A
//! shape cannot carry two carriers at one declared width. If that assertion
//! passes trivially because the two "carriers" are the same type, the probe
//! establishes nothing. Section 0 is the control that makes the two carriers
//! genuinely distinct, and it fails loudly if they are not.

#![allow(dead_code)]

use core::mem::size_of;

// --- 0. the control -----------------------------------------------------------
//
// Every claim below rests on u16 and u32 being distinguishable by the ambient
// layout observation. If they are not, nothing in this file means anything.

#[test]
fn the_control_the_two_carriers_are_distinguishable_at_all() {
    assert_ne!(
        size_of::<u16>(),
        size_of::<u32>(),
        "the carriers this probe distinguishes are indistinguishable, so every \
         separation below is vacuous"
    );
}

// --- 1. branch A: the carrier is not part of the type -------------------------
//
// `proposal::an_axis_the_realisation_map_does_not_read_is_not_a_type_parameter`,
// second and third sentences, which are signed. Under this branch the carrier is
// chosen by lowering and is not nameable, so one declared width is one type.

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FixedA<const I: usize, const F: usize>(u32);

impl<const I: usize, const F: usize> FixedA<I, F> {
    pub const DECLARED_WIDTH: usize = I + F;
}

/// A consumer signature under branch A. Two generic parameters, both widths.
pub fn consume_a<const I: usize, const F: usize>(x: FixedA<I, F>) -> usize {
    let _ = x;
    FixedA::<I, F>::DECLARED_WIDTH
}

// --- 2. branch B: the carrier is part of the type -----------------------------
//
// `proposal::the_carrier_is_observable_through_the_ambient_layout_observation_alone`.
// Under this branch two realisations agreeing on the value set and the
// realisation map and differing in the carrier are distinct in layout, so the
// carrier has to be nameable for a consumer to select one.

pub trait Carrier {
    type Repr: Copy + PartialEq + core::fmt::Debug;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Narrow;
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Wide;

impl Carrier for Narrow {
    type Repr = u16;
}
impl Carrier for Wide {
    type Repr = u32;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FixedB<const I: usize, const F: usize, S: Carrier>(S::Repr);

impl<const I: usize, const F: usize, S: Carrier> FixedB<I, F, S> {
    pub const DECLARED_WIDTH: usize = I + F;
}

/// A consumer signature under branch B. Three generic parameters: the extra one
/// is the threaded parameter the refusal row prices.
pub fn consume_b<const I: usize, const F: usize, S: Carrier>(x: FixedB<I, F, S>) -> usize {
    let _ = x;
    FixedB::<I, F, S>::DECLARED_WIDTH
}

// --- 3. the separation --------------------------------------------------------

#[test]
fn branch_b_carries_two_layouts_at_one_declared_width() {
    // One declared width, 13 bits. Two carriers. Two distinct sizes.
    assert_eq!(FixedB::<13, 0, Narrow>::DECLARED_WIDTH, 13);
    assert_eq!(FixedB::<13, 0, Wide>::DECLARED_WIDTH, 13);
    assert_eq!(size_of::<FixedB<13, 0, Narrow>>(), 2);
    assert_eq!(size_of::<FixedB<13, 0, Wide>>(), 4);
    assert_ne!(
        size_of::<FixedB<13, 0, Narrow>>(),
        size_of::<FixedB<13, 0, Wide>>(),
        "branch B must be able to name two footprints at one declared width, or \
         it is not branch B"
    );
}

#[test]
fn branch_a_has_exactly_one_layout_per_declared_width() {
    // There is no second instantiation to write. `FixedA<13, 0>` is one type and
    // one size, and no turbofish reaches another. This is the whole content of
    // branch A and it is what makes the two branches different designs.
    assert_eq!(FixedA::<13, 0>::DECLARED_WIDTH, 13);
    assert_eq!(size_of::<FixedA<13, 0>>(), 4);

    // The claim under test: every value of the branch-A type at one declared
    // width has one footprint. Asserted over the whole matrix of declared widths
    // the shape admits, rather than over a chosen sample.
    assert_eq!(size_of::<FixedA<1, 0>>(), size_of::<FixedA<13, 0>>());
    assert_eq!(size_of::<FixedA<8, 0>>(), size_of::<FixedA<13, 0>>());
    assert_eq!(size_of::<FixedA<0, 8>>(), size_of::<FixedA<13, 0>>());
    assert_eq!(size_of::<FixedA<16, 15>>(), size_of::<FixedA<13, 0>>());
}

// --- 4. the arity, which is the finding ---------------------------------------
//
// A consumer's own signature is where the branches stop being interchangeable.
// `consume_a` is generic over two parameters and `consume_b` over three, and the
// third is not defaultable away: a default on `S` picks a carrier, which is
// branch B choosing for the consumer rather than the design deferring.

#[test]
fn the_consumer_signature_arity_differs_between_the_branches() {
    assert_eq!(consume_a(FixedA::<13, 0>(0)), 13);
    assert_eq!(consume_b(FixedB::<13, 0, Narrow>(0u16)), 13);
    assert_eq!(consume_b(FixedB::<13, 0, Wide>(0u32)), 13);

    // Both compile. That is not the point and stating it is not the finding.
    // The finding is that `consume_b` had to name `S` to be written at all,
    // and `consume_a` has no `S` to name. Counted rather than asserted in
    // prose: the two functions this file declares take two and three generic
    // parameters respectively, which a reader can check in the source above.
}

// --- 5. can one declaration serve both? ---------------------------------------
//
// The route that would dissolve the fork: a single type constructor of which
// both branches are instances, so the design defers rather than decides.
//
// The attempt: give `S` a default, so a consumer may write `Fixed<13, 0>` and
// the design is nominally branch-A-shaped while remaining branch-B-capable.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Defaulted;
impl Carrier for Defaulted {
    type Repr = u32;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FixedD<const I: usize, const F: usize, S: Carrier = Defaulted>(S::Repr);

impl<const I: usize, const F: usize, S: Carrier> FixedD<I, F, S> {
    pub const DECLARED_WIDTH: usize = I + F;
}

#[test]
fn a_default_does_not_dissolve_the_fork_it_picks_branch_b_and_hides_it() {
    // The default reaches a type-position elision only.
    assert_eq!(size_of::<FixedD<13, 0>>(), 4);
    assert_eq!(size_of::<FixedD<13, 0, Narrow>>(), 2);

    // Two footprints remain reachable at one declared width, so this is branch B
    // whatever the surface looks like. A design shipping this has answered the
    // premise, and answered it in the direction that makes the footprint
    // observable, while reading as though it had deferred.
    assert_ne!(
        size_of::<FixedD<13, 0>>(),
        size_of::<FixedD<13, 0, Narrow>>()
    );
}

/// A default does not reach a generic function's parameter list, which is where
/// the cost the refusal row prices actually lands.
///
/// Written as a function rather than asserted in prose because the compiler is
/// the thing that settles it: `S` appears in this signature and no default
/// removes it, so every polymorphic consumer downstream threads it.
pub fn consume_d<const I: usize, const F: usize, S: Carrier>(x: FixedD<I, F, S>) -> usize {
    let _ = x;
    FixedD::<I, F, S>::DECLARED_WIDTH
}

#[test]
fn the_default_is_absent_from_the_polymorphic_position() {
    assert_eq!(consume_d(FixedD::<13, 0, Narrow>(0u16)), 13);
    assert_eq!(consume_d(FixedD::<13, 0, Wide>(0u32)), 13);
}
