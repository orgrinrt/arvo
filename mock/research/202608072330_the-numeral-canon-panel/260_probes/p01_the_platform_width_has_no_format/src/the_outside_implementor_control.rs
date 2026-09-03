//! The control for `the_outside_implementor.rs`: the same outside impl at 62.
//!
//! If this compiles and the 64-bit one does not, the obstruction is the
//! arithmetic of the slot coordinate. If this also refused, the obstruction
//! would be the orphan rules or the `ADMITTED` obligation, and the finding beside
//! it would be about something else entirely.

use arvo_format::{Slot, Slots, Width};

/// A 62-bit unsigned slot range, declared outside the crate that ships the family.
pub struct OutsideUnsigned62;

impl Slots for OutsideUnsigned62 {
    const MAX: Slot = Slot::at((1i64 << 62) - 1);
    const MIN: Slot = Slot::at(0);
    const WIDTH: Width = Width::bits(62);
}

/// Forcing the obligation, so the control exercises the same path the other arm
/// would have reached.
pub const THE_OUTSIDE_RANGE_IS_ADMISSIBLE: () = {
    let () = <OutsideUnsigned62 as Slots>::ADMITTED;
    assert!(<OutsideUnsigned62 as Slots>::WIDTH.count() == 62);
};
