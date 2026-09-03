//! The escape the refusal's own diagnostic names, taken and found closed.
//!
//! The `E0277` on the 64-bit arm says in as many words: "The trait is open, so
//! another crate may implement it at a range of its own; what such an implementor
//! owes is the `ADMITTED` obligation." This file is that other crate. It takes
//! the escape by the shortest route available, a **signed** 64-bit range, whose
//! slots run `-2^63 ..= 2^63 - 1` and are therefore exactly the `i64` the slot
//! coordinate carries. No constant in it overflows anything.
//!
//! **The case that must fail, stated before the run**: I predicted this would
//! compile, and that a signed-against-unsigned asymmetry would be the finding. It
//! does not compile, and the reason is better than the prediction.
//!
//! `slots.rs:219` caps `Slots::ADMITTED` at 62 for **every** implementor, so the
//! escape the diagnostic names is closed by the obligation the same diagnostic
//! tells the implementor it owes. The `E0080` is committed beside this file.
//!
//! The control, `the_outside_signed_sixty_four_control.rs`, is this declaration
//! verbatim with the obligation not forced. It builds. That is what says the
//! refusal is the obligation and nothing else, and it carries the arithmetic arm
//! establishing what the coordinate can name at all.

use arvo_format::{Slot, Slots, Width};

/// A 64-bit signed slot range, declared outside the crate that ships the family.
///
/// Every constant here is inside `i64`. Nothing about this declaration is
/// arithmetically impossible, which is what makes the refusal it draws a
/// statement about the obligation rather than about the numbers.
pub struct PlatformSigned64;

impl Slots for PlatformSigned64 {
    const MAX: Slot = Slot::at(i64::MAX);
    const MIN: Slot = Slot::at(i64::MIN);
    const WIDTH: Width = Width::bits(64);
}

/// Forcing the obligation, which is what refuses.
pub const THE_ESCAPE_IS_CLOSED_BY_THE_OBLIGATION: () = {
    let () = <PlatformSigned64 as Slots>::ADMITTED;
    assert!(<PlatformSigned64 as Slots>::WIDTH.count() == 64);
};
