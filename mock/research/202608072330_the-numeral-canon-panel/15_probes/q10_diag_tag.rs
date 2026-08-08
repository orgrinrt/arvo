// q10. What a consumer reads when a width mismatch happens under (W,F) keying,
// and an attack on it.
//
// 13's arrangement A pays the worst diagnostic in the set: two structural digit
// towers with elisions and none of the consumer's four numbers (13:279-300).
// 13's arrangements B and D fix it by making the width type the literal, and B
// then pays the ceiling because naming an operation's output means crossing back
// (13:338-376). Under (W,F) keying the consumer's I is not even a stored
// parameter, so B's fix does not straightforwardly apply.
//
// THE MOVE TESTED HERE. Carry the consumer's literal pair as a parameter used
// for NOTHING except the diagnostic. It is never computed, only propagated when
// an operation preserves the shape and dropped to `Anon` when an operation
// derives a new one. So there is no reverse crossing: the tag is not a function
// of the output, it is a label that either survives or does not.
//
// Three arms, so the messages can be read side by side:
//   arm A   bare structural widths, no tag                   (13's arrangement A)
//   arm B   structural widths plus a const-generic literal tag
//   arm C   arm B, with the equality routed through an annotated trait so the
//           failure is E0277 rather than E0308
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. Edition 2024.
// Build: rustc +nightly-2026-05-28 --edition 2024 --crate-type lib q10_diag_tag.rs \
//          --out-dir build     (EXPECTED TO FAIL; the failures are the result)

#![no_std]
#![allow(dead_code)]

include!("q04_core_inc.rs");
include!("q04_literals_inc.rs");

pub struct L<const K: u32>;
pub trait Lit {
    type N;
}
impl Lit for L<3> {
    type N = N3;
}
impl Lit for L<4> {
    type N = N4;
}
impl Lit for L<12> {
    type N = N12;
}
impl Lit for L<13> {
    type N = N13;
}
pub type NatOf<const K: u32> = <L<K> as Lit>::N;

// ---------------------------------------------------------------- arm A -----
pub mod a {
    use super::*;
    pub struct Numeral<W, F>(core::marker::PhantomData<(W, F)>);
    pub type UFixed<const I: u32, const F: u32> = Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>>;

    pub fn takes(_: UFixed<13, 3>) {}
    pub fn caller(x: UFixed<12, 4>) {
        takes(x);
    }
}

// ---------------------------------------------------------------- arm B -----
// The tag. Two const parameters and nothing else. It is PhantomData, so it is
// gone at codegen; q11 checks that rather than asserting it.
pub mod b {
    use super::*;
    pub struct Tag<const I: u32, const F: u32>;
    pub struct Anon;

    pub struct Numeral<W, F, T>(core::marker::PhantomData<(W, F, T)>);
    pub type UFixed<const I: u32, const F: u32> =
        Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Tag<I, F>>;

    pub fn takes(_: UFixed<13, 3>) {}
    pub fn caller(x: UFixed<12, 4>) {
        takes(x);
    }
}

// ---------------------------------------------------------------- arm C -----
// Same tag, but the shape equality is a trait obligation rather than a type
// identity, so `#[diagnostic::on_unimplemented]` is reachable. 13:290-292 found
// the annotation does not reach E0308 and rustc offers no other hook; this asks
// whether moving the check off E0308 is that hook.
pub mod c {
    pub use super::b::{Anon, Tag};
    use super::*;

    pub struct Numeral<W, F, T>(core::marker::PhantomData<(W, F, T)>);
    pub type UFixed<const I: u32, const F: u32> =
        Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Tag<I, F>>;

    #[diagnostic::on_unimplemented(
        message = "this numeral is not the width the operation expects",
        label = "the two numerals must have the same integer and fraction widths",
        note = "the expected and found `Tag<I, F>` in the note below carry the two widths as written"
    )]
    pub trait SameShape<R> {}
    impl<W, F, T> SameShape<Numeral<W, F, T>> for Numeral<W, F, T> {}

    pub fn takes<X>(_: X)
    where
        X: SameShape<UFixed<13, 3>>,
    {
    }
    pub fn caller(x: UFixed<12, 4>) {
        takes(x);
    }
}
