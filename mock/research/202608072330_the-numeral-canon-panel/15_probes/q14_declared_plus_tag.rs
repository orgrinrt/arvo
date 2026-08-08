// q14. q11 found the diagnostic tag's one defect: a computed value's type does
// not unify with a written alias of the same shape, because a type parameter is
// part of type identity however unused it is. This probe asks whether 13's
// arrangement D dissolves that rather than tolerating it.
//
// D's move (13:377-411) is that a product site DECLARES its output width and the
// type system checks the declaration is wide enough. If the consumer is already
// naming the output, then the tag on the output is the consumer's own numbers,
// supplied at the same site, and there is nothing to retag. The defect and the
// arrangement cancel.
//
// Four checks:
//   d1  a declared-output product with an adequate declaration          compiles
//   d2  a declared-output product with a too-narrow declaration         refuses
//   d3  what the refusal reads like                                     inspected
//   d4  does the declaration reintroduce a reverse table                counted
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. Edition 2024.
// Build (d2 expected to fail; that failure is the result):
//   rustc +nightly-2026-05-28 --edition 2024 --crate-type lib q14_declared_plus_tag.rs \
//     --out-dir build

#![no_std]
#![allow(dead_code)]

include!("q04_core_inc.rs");
include!("q04_literals_inc.rs");

pub struct L<const K: u32>;
pub trait Lit {
    type N;
}
macro_rules! lits { ($($k:literal => $t:ty),* $(,)?) => { $(
    impl Lit for L<$k> { type N = $t; }
)* } }
// The ENTIRE bridge for this program. Six rows, and they are exactly the six
// numbers written below in consumer position. Nothing computed appears here.
lits!(0 => N0, 3 => N3, 6 => N6, 13 => N13, 26 => N26, 27 => N27);
pub type NatOf<const K: u32> = <L<K> as Lit>::N;

pub struct Tag<const I: u32, const F: u32>;
pub struct Numeral<W, F, T>(core::marker::PhantomData<(W, F, T)>);

// (W, F) keying, per section 1: W = I + F is computed in the nat algebra, not in
// a const argument, so no forbidden feature is anywhere near this.
pub type UFixed<const I: u32, const F: u32> = Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Tag<I, F>>;

// The declared-output product. `Out` is written by the consumer; the obligation
// is that its shape contains the product's. There is no reverse table because
// nothing is computed BACK into a literal: the check compares nats.
pub trait MulInto<R, Out> {}

#[diagnostic::on_unimplemented(
    message = "the declared output numeral is narrower than the product needs",
    label = "widen the declared output, or state the rounding explicitly",
    note = "a product occupies the sum of the total widths and the sum of the fraction widths; the `Tag<I, F>` in the note below carries the widths as the consumer wrote them"
)]
pub trait IsLe {}
impl IsLe for Lt {}
impl IsLe for Eqq {}

impl<W1, F1, T1, W2, F2, T2, WO, FO, TO> MulInto<Numeral<W2, F2, T2>, Numeral<WO, FO, TO>>
    for Numeral<W1, F1, T1>
where
    W1: Add<W2>,
    F1: Add<F2>,
    Sum<W1, W2>: Cmp<WO>,
    Sum<F1, F2>: Cmp<FO>,
    Ord2<Sum<W1, W2>, WO>: IsLe,
    Ord2<Sum<F1, F2>, FO>: IsLe,
{
}

type Money = UFixed<13, 3>; // W = 16, F = 3
type Wide = UFixed<26, 6>; // W = 32, F = 6, wide enough
type TooNarrow = UFixed<27, 3>; // W = 30, F = 3, fraction too narrow

// d1: the adequate declaration
fn d1()
where
    Money: MulInto<Money, Wide>,
{
}

// d2: the too-narrow declaration. EXPECTED TO FAIL.
#[cfg(not(no_d2))]
fn d2()
where
    Money: MulInto<Money, TooNarrow>,
{
}

// d4: the tag is on the declared output, supplied by the consumer at the site
// where they already write the numbers, so nothing retags and nothing crosses
// back. Count of impls keyed on a computed width in this file: zero, checkable
// with `grep -c 'impl.*Lit for L<' q14_declared_plus_tag.rs` against the list of
// literals written in consumer position.
