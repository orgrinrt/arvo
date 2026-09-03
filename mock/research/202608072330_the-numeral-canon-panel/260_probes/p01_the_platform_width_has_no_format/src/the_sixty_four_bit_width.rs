//! The arm that does not compile, kept out of the build on purpose.
//!
//! A platform-width numeral on a 64-bit target is the shape family at 64 bits.
//! This writes exactly that, both signed and unsigned, and asks the shipped
//! format contract to accept it.
//!
//! Build it with `./build_the_refusal.sh`, which puts the stderr beside this
//! directory. The control that says the refusal is the width rather than the
//! spelling is `the_sixty_two_bit_width.rs`, identical but for the number.

use arvo_format::points::{Integer, UFixed};
use arvo_format::{Format, Slots};

/// The unsigned platform-width numeral a 64-bit target needs.
pub type USize = UFixed<64, 0>;

/// The signed one.
pub type ISize = Integer<64>;

/// Reading the declared width is what forces the `Format` bound.
pub const THE_DECLARED_WIDTH: () = {
    assert!(<USize as Format>::Slots::WIDTH.count() == 64);
    assert!(<ISize as Format>::Slots::WIDTH.count() == 64);
};
