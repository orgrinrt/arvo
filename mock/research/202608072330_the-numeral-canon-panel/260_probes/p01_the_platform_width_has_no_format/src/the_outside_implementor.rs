//! The escape the refusal's own diagnostic names, taken and found closed.
//!
//! The `E0277` on the 64-bit arm says the trait is open and another crate may
//! implement it at a range of its own. This is that crate. It declares a slot
//! range for a 64-bit unsigned platform-width numeral and tries to state its own
//! upper bound.
//!
//! **The case that must fail, stated before the run**: `Slot` carries an `i64`,
//! and the largest slot of a 64-bit unsigned declaration is `2^64 - 1`, which is
//! not an `i64`. So the bound cannot be written, and the obstruction is the
//! coordinate's own type rather than the shipped impl list. If this compiled,
//! the finding beside it would be void and the answer would be that a downstream
//! crate can simply supply the width.
//!
//! The control is `the_outside_implementor_control.rs`, the identical outside
//! impl at 62, which does compile and is what says the refusal is the arithmetic
//! rather than the orphan rules or the obligation.

use arvo_format::{Slot, Slots, Width};

/// A 64-bit unsigned slot range, declared outside the crate that ships the family.
pub struct PlatformUnsigned64;

impl Slots for PlatformUnsigned64 {
    // 2^64 - 1. Written as the shipped macro writes it, one domain up, because
    // there is no domain up from the one the coordinate is in.
    const MAX: Slot = Slot::at((1i64 << 64) - 1);
    const MIN: Slot = Slot::at(0);
    const WIDTH: Width = Width::bits(64);
}
