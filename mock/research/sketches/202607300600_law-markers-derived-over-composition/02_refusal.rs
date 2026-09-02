// Probe 02: the refusal. This file MUST NOT COMPILE.
//
// Probe 01 showed the laws derive and the positive cases resolve. That proves
// half of it. The half that matters is whether the composition the mathematics
// forbids is actually rejected, because a derivation that admits everything has
// derived nothing.
//
// Signed saturating addition is not associative:
//   (127 + 1) + (-1) = 127 + (-1) = 126
//   127 + (1 + (-1)) = 127 + 0    = 127
// so a fold over it returns a grouping-dependent answer. This asks for that fold
// and expects the compiler to refuse.
//
// Run: rustc --edition 2021 02_refusal.rs -o /tmp/q02
// Expected: error[E0277], quoted in FINDINGS.md.

use core::marker::PhantomData;

pub struct Wrap;
pub struct Saturate;
pub struct Unsigned;
pub struct Signed;
pub struct Trunc;
pub struct FullPrecision;
pub struct Fixed<const F: i32>;

pub trait AddAssoc {}

impl AddAssoc for (Wrap, Unsigned) {}
impl AddAssoc for (Wrap, Signed) {}
impl AddAssoc for (Saturate, Unsigned) {}
// Deliberately absent: impl AddAssoc for (Saturate, Signed)

pub struct Num<Fmt, Sign, Round, Over, Grow>(PhantomData<(Fmt, Sign, Round, Over, Grow)>);

pub struct Add;

pub trait Semigroup<Op> {}

impl<Fmt, Sign, Round, Over, Grow> Semigroup<Add> for Num<Fmt, Sign, Round, Over, Grow> where
    (Over, Sign): AddAssoc
{
}

fn fold_requires_assoc<T: Semigroup<Add>>() -> &'static str {
    "folded"
}

fn main() {
    // Fine: unsigned saturating is truncated addition, a commutative monoid.
    type SatU = Num<Fixed<8>, Unsigned, Trunc, Saturate, FullPrecision>;
    let _ = fold_requires_assoc::<SatU>();

    // THE REFUSAL. Signed two-sided saturating addition is not associative, so
    // folding it is a question with no single right answer. This line is the
    // reason the whole derivation exists.
    type SatI = Num<Fixed<8>, Signed, Trunc, Saturate, FullPrecision>;
    let _ = fold_requires_assoc::<SatI>();
}
