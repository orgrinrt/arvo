//! The control for `the_sixty_four_bit_width.rs`, identical but for the number.
//!
//! Sixty-two is the widest the shipped slot family admits. If this compiled and
//! the other did not, the refusal is the width bound. If both refused, the
//! refusal would be about the alias spelling and the finding beside it would be
//! void.

use arvo_format::points::{Integer, UFixed};
use arvo_format::{Format, Slots};

/// The unsigned numeral at the ceiling of the shipped family.
pub type USizeAtCeiling = UFixed<62, 0>;

/// The signed one.
pub type ISizeAtCeiling = Integer<62>;

/// The same const, forcing the same bound, at the width that is admitted.
pub const THE_DECLARED_WIDTH: () = {
    assert!(<USizeAtCeiling as Format>::Slots::WIDTH.count() == 62);
    assert!(<ISizeAtCeiling as Format>::Slots::WIDTH.count() == 62);
};
