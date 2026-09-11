//! p6a: the ladder over the pointer width, with no format in the crate.
//!
//! The half of p6 that touches no `Slots` impl, so it builds at every pointer
//! width including 64, where p6 inherits p1's refusal. What it asserts: the
//! narrowest shipped carrier covering the pointer width is the machine word, at
//! every target, and the ladder itself is the same list at every target.
#![no_std]

use arvo_format::width::Width;
use arvo_placement::{LADDER, narrowest_carrier};

/// The narrowest shipped carrier covering the pointer width.
pub const CARRIER: Width = narrowest_carrier(Width::bits(usize::BITS));

const _: () = assert!(
    CARRIER.count() == usize::BITS,
    "the narrowest carrier covering the pointer width is the machine word"
);

/// The ladder names what the host language offers, not what the target's word is.
const _: () = assert!(LADDER[0].count() == 8 && LADDER[3].count() == 64);
