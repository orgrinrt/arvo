//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The adaptation's two vocabularies, and the open inventory reaching the map.
//!
//! Nothing here reads a declaration back: a mode's constant against the literal
//! its own impl set, a policy's through one accessor, or a counter against the
//! number of times the test itself incremented it could not fail. Everything
//! here can.
//!
//! The two vocabulary facts are stated as an exhaustive `match` with no wildcard,
//! so a seventh mode or a fourth policy stops this file compiling, and the sweep
//! lists the rest of the crate walks are checked against that match rather than
//! against their own length. The open-inventory arm runs its foreign policy
//! through the map. Whether the two coordinates move independently is measured in
//! the applied map's suite, which adapts one position under two signatures that
//! differ in one coordinate.

use crate::adapt::{Adapt, Signature};
use crate::apply::{Dither, Exact, adapt};
use crate::overflow::{Clamp, Overflow, Policy, SHIPPED_POLICIES};
use crate::points::Integer;
use crate::rounding::{ALL_MODES, Floor, Mode};
use crate::slots::Slot;

// --- the two vocabularies, and the sweeps over them --------------------------

/// Where a mode sits among the six, by an exhaustive match.
///
/// No wildcard, on purpose: a seventh name is a compile error here before it is
/// anything else, which is the closed vocabulary held by the compiler rather than
/// by a count somebody remembered.
fn place_of_mode(mode: Mode) -> usize {
    match mode {
        Mode::TowardZero => 0,
        Mode::Floor => 1,
        Mode::Ceil => 2,
        Mode::HalfUp => 3,
        Mode::HalfEven => 4,
        Mode::Stochastic => 5,
    }
}

/// Where a policy sits among the shipped three, by the same kind of match.
fn place_of_policy(policy: Policy) -> usize {
    match policy {
        Policy::Wrap => 0,
        Policy::Saturate => 1,
        Policy::Clamp => 2,
    }
}

#[test]
fn the_mode_sweep_holds_every_name_exactly_once() {
    // `ALL_MODES` is what every law over the rounding region walks, so a name
    // missing from it is a region no law was asserted over, and a name in it
    // twice is a sweep that counted one mode for two. The length of the array
    // says neither.
    let mut seen = [0u32; 6];
    for mode in ALL_MODES {
        seen[place_of_mode(mode)] += 1;
    }
    assert_eq!(
        seen, [1; 6],
        "the sweep does not hold each of the six exactly once"
    );
}

#[test]
fn the_policy_sweep_holds_every_shipped_policy_exactly_once() {
    let mut seen = [0u32; 3];
    for policy in SHIPPED_POLICIES {
        seen[place_of_policy(policy)] += 1;
    }
    assert_eq!(
        seen, [1; 3],
        "the sweep does not hold each shipped policy exactly once"
    );
}

// --- the open inventory reaches the map ---------------------------------------

/// A user-defined overflow policy, which is what an open inventory means.
struct DeclaredBound;

impl Overflow for DeclaredBound {
    const POLICY: Policy = Policy::Clamp;
}

/// A second one, deciding the other way.
///
/// One foreign member establishes that the map does not refuse an unknown type.
/// Two deciding differently establish that it reads what the declaration says,
/// which is the claim the name makes and the thing one member cannot separate.
struct DeclaredRing;

impl Overflow for DeclaredRing {
    const POLICY: Policy = Policy::Wrap;
}

/// The window `Integer<5>` declares: slots -16 through 15, so a span of 32.
const MIN5: Slot = Slot::at(-16);
const MAX5: Slot = Slot::at(15);

#[test]
fn the_overflow_inventory_admits_a_member_this_crate_does_not_know_about() {
    // What the name claims is that a policy declared outside this crate reaches
    // the crate's machinery, so every assertion runs it through the map. A
    // read-back of the declared constant would say only that a constant holds
    // what its own impl set.
    type Outside = Signature<Integer<5>, Adapt<Floor, DeclaredBound>>;
    type Ring = Signature<Integer<5>, Adapt<Floor, DeclaredRing>>;
    type Shipped = Signature<Integer<5>, Adapt<Floor, Clamp>>;

    // Positions well outside the window on both sides, so the completion region
    // answers and the rounding region has nothing to do.
    let above = Exact::on_grid(Slot::at(MAX5.index() + 25));
    let below = Exact::on_grid(Slot::at(MIN5.index() - 25));

    for (foreign, shipped) in [
        (
            adapt::<Outside>(above, Dither::UNUSED),
            adapt::<Shipped>(above, Dither::UNUSED),
        ),
        (
            adapt::<Outside>(below, Dither::UNUSED),
            adapt::<Shipped>(below, Dither::UNUSED),
        ),
    ] {
        assert_eq!(
            foreign, shipped,
            "a foreign policy naming the same value did not land where the shipped one does"
        );
    }
    assert_eq!(adapt::<Outside>(above, Dither::UNUSED), MAX5);
    assert_eq!(adapt::<Outside>(below, Dither::UNUSED), MIN5);

    // The two foreign members decide differently, which is what says the map
    // reads the declaration rather than answering one way for anything it does
    // not recognise.
    assert_ne!(
        adapt::<Outside>(above, Dither::UNUSED),
        adapt::<Ring>(above, Dither::UNUSED),
        "two foreign policies naming different values adapted the same way"
    );

    // Worked out by hand rather than by the expression the crate uses. The span
    // is 32, so 40 lands at ((40 + 16) mod 32) - 16, which is 24 - 16 = 8, and
    // -41 lands at ((-41 + 16) mod 32) - 16, which is 7 - 16 = -9.
    assert_eq!(adapt::<Ring>(above, Dither::UNUSED), Slot::at(8));
    assert_eq!(adapt::<Ring>(below, Dither::UNUSED), Slot::at(-9));

    // And the foreign path is total, like every other: both answers are in the
    // declared window rather than merely different from each other.
    for got in [
        adapt::<Outside>(above, Dither::UNUSED),
        adapt::<Outside>(below, Dither::UNUSED),
        adapt::<Ring>(above, Dither::UNUSED),
        adapt::<Ring>(below, Dither::UNUSED),
    ] {
        assert!(
            got.is_within(MIN5, MAX5).get(),
            "{got:?} left the declared window under a foreign policy"
        );
    }

    // The control: inside the window the completion does nothing, so both foreign
    // members agree there and the disagreement above is about the region the
    // policy governs rather than about the policy being read at all.
    let inside = Exact::on_grid(Slot::at(3));
    assert_eq!(adapt::<Outside>(inside, Dither::UNUSED), Slot::at(3));
    assert_eq!(adapt::<Ring>(inside, Dither::UNUSED), Slot::at(3));
}
