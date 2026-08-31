//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The overflow policy: what happens outside the representable range.
//!
//! Open, unlike the rounding vocabulary. Three members ship and another joins by
//! supplying the obligations, because nothing ratified closed this one.
//!
//! **A panic is not a member and cannot be.** The ratified factoring requires the
//! adaptation to be total, and a panic diverges, so it is not a total map onto the
//! representable set. The canon reaches the same bound from the other side: the
//! panic is a debug-build behaviour, and what stands in its place on release is a
//! declared mode that lowers and behaves accordingly. So it belongs to the build
//! profile axis and not to this one.
//!
//! The dimension row for this axis still lists a panic among its values. That
//! disagreement is recorded rather than repaired here, because the dimension set
//! is append-only and the repair is a note on that row.

/// What a value outside the representable range becomes.
///
/// Open: an implementor outside this crate is a policy this crate does not know
/// about, and that is the intended shape.
pub trait Overflow {
    /// Which policy, as a value a const predicate can gate on.
    const POLICY: Policy;
}

/// The policies the corpus carries.
///
/// Not closed. The enumeration exists so an arm has something to compare, and a
/// member arriving later extends it rather than being refused by it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Policy {
    /// Reduce modulo the range, so the value re-enters at the other end.
    Wrap,
    /// Pin to the nearer end of the range.
    Saturate,
    /// Pin to a declared bound that need not be the range's own end.
    Clamp,
}

/// Modulo the range.
pub struct Wrap;
/// To the nearer end.
pub struct Saturate;
/// To a declared bound.
pub struct Clamp;

impl Overflow for Wrap {
    const POLICY: Policy = Policy::Wrap;
}
impl Overflow for Saturate {
    const POLICY: Policy = Policy::Saturate;
}
impl Overflow for Clamp {
    const POLICY: Policy = Policy::Clamp;
}

/// The policies this crate ships, for a test that wants all of them.
///
/// Named as what ships rather than as what exists, because the inventory is open
/// and a later member would not be here.
pub const SHIPPED_POLICIES: [Policy; 3] = [Policy::Wrap, Policy::Saturate, Policy::Clamp];

// Two predicates stood here, `is_monotone` and `is_identity_inside_range`, and
// they were deleted rather than improved. Each was a `matches!` over this
// enumeration that no code read, so their tests reached a declaration and
// stopped. Both properties are now asserted against the applied map in `apply`,
// where being wrong about arithmetic is possible.
