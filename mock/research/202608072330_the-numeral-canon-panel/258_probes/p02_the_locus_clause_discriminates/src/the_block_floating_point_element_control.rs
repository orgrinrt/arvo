//! The control for `the_block_floating_point_element.rs`.
//!
//! The identical declaration with one difference: the block's exponent is a const
//! rather than a field, which is exactly what makes the block's exponent stop
//! being other data. Everything else is the same, ambient domain, slot family,
//! phase, lifetime, borrow.
//!
//! If this compiles and its pair does not, the refusal is the value-dependence
//! and nothing else. If this also refused, the refusal would be the lifetime
//! parameter, the borrow or the trait shape, and the finding beside it would be
//! void.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::{Constant, Quantum};
use arvo_format::slots::Signed;

/// The same block, with the exponent fixed for every block rather than per block.
///
/// Which is to say: not block floating point any more. It is a scaled integer
/// column, and the clause admits it, and that is the point.
pub struct Block {
    pub slots: [i64; 8],
}

/// The shared exponent, now a property of the declaration.
pub const SHARED_EXPONENT: i32 = -7;

/// One element, declared as a format, with the same borrow the other arm carries.
pub struct BlockElement<'a> {
    pub block: &'a Block,
}

impl<'a> Format for BlockElement<'a> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<{ SHARED_EXPONENT }>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::ZERO;
}

/// It is a format, and the coordinate reads back.
pub const THE_CONTROL_IS_A_FORMAT: () = {
    assert!(<<BlockElement<'static> as Format>::Quantum as Quantum>::BASE.power() == -7);
};
