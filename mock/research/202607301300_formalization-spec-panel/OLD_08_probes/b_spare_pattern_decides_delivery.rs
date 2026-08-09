//! UNION PROBE stage 4: whether the absorbing-bottom delivery's byte saving
//! (05 sec 5) survives the graded aggregate (07 sec 2).
//!
//! The aggregate's carrier is `Answer<T>` computed from (grade, delivery).
//! The bottom's saving depends on the NUMERAL having a spare pattern, which
//! the aggregate cannot see. This file indexes the delivery by a spare-pattern
//! marker derived from (numeral, lowering) and measures the result.
use crate::*;

/// Does this (numeral, lowering) pair leave a bit pattern unused?
/// Keyed on the PROJECTED members, not on the lowering's name, because a
/// modifier (06 sec 5) is a distinct type that no name-keyed table covers.
pub trait SpareRule {
    type Spare: TruthMarker;
}
impl SpareRule for (DoubleLogical, Dense) {
    type Spare = True;
}
impl SpareRule for (DoubleLogical, Bitpacked) {
    type Spare = True;
}
impl SpareRule for (Minimum, Dense) {
    type Spare = False;
}
impl SpareRule for (Minimum, Bitpacked) {
    type Spare = False;
}
pub type SpareOf<L> = <(<L as Lowering>::StoredWidth, <L as Lowering>::Layout) as SpareRule>::Spare;

/// The bottom carried IN the payload, repr-transparent, one sentinel value.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct InPayload<T>(pub T);
impl<T: Copy> Carrier<T> for InPayload<T> {
    fn from_output(v: T) -> Self {
        InPayload(v)
    }
}

/// Delivery indexed by grade AND by whether a spare pattern exists.
pub trait Deliver2<G, Sp> {
    type C<T: Copy>: Carrier<T>;
    fn refuse<T: Copy>(nearest: T) -> Self::C<T>;
}
impl<Sp> Deliver2<False, Sp> for AsBottom {
    type C<T: Copy> = Total<T>;
    fn refuse<T: Copy>(n: T) -> Total<T> {
        Total(n)
    }
}
impl Deliver2<True, True> for AsBottom {
    type C<T: Copy> = InPayload<T>; // free: bottom lives in the spare pattern
    fn refuse<T: Copy>(n: T) -> InPayload<T> {
        InPayload(n)
    }
}
impl Deliver2<True, False> for AsBottom {
    type C<T: Copy> = Poison<T>; // no spare pattern: pay the flag byte
    fn refuse<T: Copy>(n: T) -> Poison<T> {
        Poison { v: n, bottom: true }
    }
}

/// The aggregate, now needing the numeral in the carrier computation.
pub trait Arith2 {
    type Answer<T: Copy>: Carrier<T>;
    fn over<T: Copy>(nearest: T) -> Self::Answer<T>;
}
impl<N: Numeral, P: Policy, L: Lowering> Arith2 for Number<N, P, L>
where
    (L::StoredWidth, L::Layout): SpareRule,
    L::Delivery: Deliver2<JoinG<P>, SpareOf<L>>,
    OverG<P>: Or<UnderG<P>>,
{
    type Answer<T: Copy> = <L::Delivery as Deliver2<JoinG<P>, SpareOf<L>>>::C<T>;
    fn over<T: Copy>(nearest: T) -> Self::Answer<T> {
        <L::Delivery as Deliver2<JoinG<P>, SpareOf<L>>>::refuse(nearest)
    }
}
