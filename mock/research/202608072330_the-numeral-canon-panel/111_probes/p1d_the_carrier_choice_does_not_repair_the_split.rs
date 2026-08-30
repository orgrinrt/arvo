// p1d. The carrier choice is not a design lever for the naming wall.
//
// p1b and p1c establish that a completion can be carried as a const generic
// value or as macro syntax rather than as a type, so `109`'s "the operation has
// to be a type" does not follow from the const-eval wall.
//
// The question that matters next is whether the carrier choice buys anything.
// `110` F8 shows two spellings of one primitive are a compile error with no
// in-language repair, using a const generic value (the radix) as its example.
// This probe checks the other carrier: a completion carried as a TYPE splits
// exactly the same way. So the two carriers are equivalent on the hazard, and
// what decides whether the axis may appear at all is `110`'s read-test, not the
// form the axis takes.
//
// Expected: FAILS to compile, at the marked line, with E0308.

use core::marker::PhantomData;

pub trait Completion {
    const LO: i32;
    const HI: i32;
}

pub struct SatBoth;
impl Completion for SatBoth {
    const LO: i32 = -8;
    const HI: i32 = 7;
}

pub struct Wrap;
impl Completion for Wrap {
    const LO: i32 = -8;
    const HI: i32 = 7;
}

#[repr(transparent)]
pub struct Fx<C: Completion>(i32, PhantomData<C>);

fn takes_sat(_x: Fx<SatBoth>) {}

fn main() {
    let w: Fx<Wrap> = Fx(0, PhantomData);
    // The two completions agree on every value in -8..=7 that no operation
    // leaves, which is the whole box under a proved range. The type system
    // cannot say so.
    takes_sat(w);
}
