// Probe 4, half one: the arvo-side crate. Declares Numeral, Number<N,S>,
// and a fact trait `Associative<Op>` keyed on the operation but with the
// composition (Number<N,S>) as the Self / impl-target type, matching
// Dolan's "reading two" shape (14_dolan_which_algebra_is_this.md) and
// Move A of the main file. Compiled as a real separate crate so the
// orphan-rule question in probe 5 is a real cross-crate question, not a
// same-crate stand-in.
//
// rustc +nightly-2026-05-28 --crate-type lib 04_orphan_rule_arvo_crate.rs
//   (expect: compiles clean; this is the library, nothing to run)

pub trait Numeral {}
#[derive(Clone, Copy)]
pub struct Fixed3;
impl Numeral for Fixed3 {}

#[derive(Clone, Copy)]
pub struct Number<N: Numeral>(core::marker::PhantomData<N>);
impl<N: Numeral> Number<N> {
    pub fn new() -> Self {
        Number(core::marker::PhantomData)
    }
}

pub struct Add;

// the fact trait: Self = the numeral, parameter = the operation. This is
// the shape the current draft already has (Combine<Op>/Magma<Op>, section
// 3.7) and the shape Dolan's atomic facts extend without changing which
// side is Self.
pub trait Associative<Op> {}
impl<N: Numeral> Associative<Add> for Number<N> {}
