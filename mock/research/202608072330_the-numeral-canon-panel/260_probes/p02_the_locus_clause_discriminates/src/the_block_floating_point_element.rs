//! Arm three: the class the ratified locus clause names, kept out of the build.
//!
//! `08_knuth_what_the_one_format_concept_covers.md:278` heads its section "The
//! locus" and lists block floating point first among the excluded, with the
//! reason at `08:283`: "no per-datum type can express a constraint that holds
//! between data. The per-element value set of a block floating point element is
//! the union over shared exponents, which is a float and is expressible; what is
//! not expressible is that the block shares one exponent."
//!
//! So this writes a block floating point element the way somebody actually would:
//! a block carries one exponent, every element in it is a slot multiplied by the
//! quantum at that exponent, and the element's representable set is therefore
//! determined by a value sitting in the block header.
//!
//! Build it with `./build_the_refusal.sh`. Its stderr is committed beside this
//! directory, and the control that says the refusal is the value rather than the
//! route is `THE_CONST_ROUTE_WORKS` in `lib.rs`, which puts the identical
//! `Constant<EXP>` behind a const and builds.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

/// A block of eight elements sharing one exponent, which is what makes it block
/// floating point rather than eight independent floats.
pub struct Block {
    /// The shared exponent, read off the block at runtime. This is the "other
    /// data" the clause is about, and there is no spelling of it that is a const,
    /// because it is a different number for each block the program handles.
    pub shared_exponent: i32,
    pub slots:           [i64; 8],
}

/// One element of that block, declared as a format.
///
/// The representable set of this element is the slots of a signed eight-bit range
/// scaled by two to the block's shared exponent. Every coordinate below is exact
/// and known, and one of them is known only once a block is in hand.
pub struct BlockElement<'a> {
    pub block: &'a Block,
}

impl<'a> Format for BlockElement<'a> {
    type Ambient = BinaryRationals;
    // The quantum's exponent is the block's. There is no way to write it: a
    // field of a value reached through a reference is not a const generic
    // argument, and that is the clause's criterion firing.
    type Quantum = Constant<{ self.block.shared_exponent }>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::ZERO;
}
