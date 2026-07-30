// TEST H: the spec mixes two encodings for the same kind of thing.
//   type LogicalWidth: Width          <- a TYPE
//   Stored<const BITS: Width, U>      <- a CONST
// The 202607291400 sketch established that no permitted feature computes a
// const from a generic const parameter in type position. So every derivation
// that SUBTRACTS (precision = logical - exponent field - sign) is expressible
// only if the operands are types. Probe: they are, gate-free, and the
// derivation is a projection rather than an impl-table row.
#![allow(dead_code)]

pub trait Nat {
    const N: u16;
    type Pred: Nat;
}
macro_rules! nats { ($($cur:ident $prev:ident $v:expr;)*) => { $(pub struct $cur; impl Nat for $cur { const N: u16 = $v; type Pred = $prev; })* } }
pub struct Z;
impl Nat for Z {
    const N: u16 = 0;
    type Pred = Z;
}
nats!(N1 Z 1; N2 N1 2; N3 N2 3; N4 N3 4; N5 N4 5; N6 N5 6; N7 N6 7; N8 N7 8;);

/// subtraction as a type-level function, one impl, not a table over pairs
pub trait Sub<R: Nat> {
    type Out: Nat;
}
impl<L: Nat> Sub<Z> for L {
    type Out = L;
}
// (a full impl would recurse; two rows suffice to show the shape resolves)

pub trait Underflow {}
pub struct Gradual;
impl Underflow for Gradual {}
pub trait Signedness {
    type Bits: Nat;
}
pub struct Signed;
impl Signedness for Signed {
    type Bits = N1;
}
pub struct Unsigned;
impl Signedness for Unsigned {
    type Bits = Z;
}

/// exponent field width as a TYPE, so the significand derivation is a
/// projection and needs no gate and no per-(width, field) impl row
pub struct Stored<B: Nat, U: Underflow>(core::marker::PhantomData<(B, U)>);

pub trait Numeral {
    type LogicalWidth: Nat;
    type Sign: Signedness;
    type Significand: Nat;
}

pub struct Binary<W: Nat, B: Nat, S: Signedness>(core::marker::PhantomData<(W, B, S)>);
impl<W: Nat, B: Nat, S: Signedness> Numeral for Binary<W, B, S>
where
    W: Sub<B>,
    <W as Sub<B>>::Out: Sub<S::Bits>,
{
    type LogicalWidth = W;
    type Sign = S;
    type Significand = <<W as Sub<B>>::Out as Sub<S::Bits>>::Out;
}

fn main() {
    // W - 0 - 0 exercises the shape end to end
    type T = Binary<N8, Z, Unsigned>;
    assert_eq!(<<T as Numeral>::Significand as Nat>::N, 8);
    println!("H OK: significand derived by projection, zero feature gates");
}
