//! The second route to the same place, taken because the first refusal is weak.
//!
//! `the_block_floating_point_element.rs` draws an `E0424` saying `self` is not
//! available in an impl, and a reader is entitled to answer that this is a fact
//! about where `self` may be written rather than about value-dependence. So this
//! arm removes `self` entirely: the block is in hand, as an ordinary parameter,
//! at the moment the format is named.
//!
//! **The case that must fail, stated before the run**: `E0435`, a non-constant
//! value in a constant. The block's exponent is a value, a const generic argument
//! is not, and no amount of having the block available changes that.
//!
//! The control is `the_block_exponent_in_hand_control.rs`, the same function with
//! the exponent taken from a const instead of from the parameter it is handed.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::{Constant, Quantum};
use arvo_format::slots::Signed;

/// A block of eight elements sharing one exponent.
pub struct Block {
    pub shared_exponent: i32,
    pub slots:           [i64; 8],
}

/// The element's format at a given block's exponent.
///
/// Generic over the exponent, which is the only shape a format can have: the
/// coordinate is a parameter of the type.
pub struct BlockElement<const EXP: i32>;

impl<const EXP: i32> Format for BlockElement<EXP> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<EXP>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::ZERO;
}

/// Reading the quantum of the element a given block holds.
///
/// The block is right here. Its exponent is right here. The format is still not
/// nameable, and the compiler says why.
pub fn the_quantum_of_this_blocks_elements(block: &Block) -> i32 {
    <<BlockElement<{ block.shared_exponent }> as Format>::Quantum as Quantum>::BASE.power()
}
