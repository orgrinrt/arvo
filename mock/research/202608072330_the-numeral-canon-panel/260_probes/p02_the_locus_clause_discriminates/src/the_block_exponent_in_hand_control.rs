//! The control for `the_block_exponent_in_hand.rs`.
//!
//! Identical, including the parameter the function is handed, except that the
//! exponent comes from a const rather than from that parameter. The block is
//! still borrowed and still unused-but-present, so the difference between the two
//! arms is exactly which of the two the const generic argument reads.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::{Constant, Quantum};
use arvo_format::slots::Signed;

/// The same block.
pub struct Block {
    pub shared_exponent: i32,
    pub slots:           [i64; 8],
}

/// The exponent as a property of the declaration rather than of a block.
pub const SHARED_EXPONENT: i32 = -7;

/// The same element, generic over the exponent.
pub struct BlockElement<const EXP: i32>;

impl<const EXP: i32> Format for BlockElement<EXP> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<EXP>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::ZERO;
}

/// The same function, taking the same block, reading the const instead.
pub fn the_quantum_of_this_blocks_elements(block: &Block) -> i32 {
    let _ = block.shared_exponent;
    <<BlockElement<{ SHARED_EXPONENT }> as Format>::Quantum as Quantum>::BASE.power()
}
