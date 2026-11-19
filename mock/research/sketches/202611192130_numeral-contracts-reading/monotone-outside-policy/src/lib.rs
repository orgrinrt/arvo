//! Must fail: the same marker and the same outside policy as the control, and
//! a function bounding on the marker handed the outside-policy value. The
//! value saturates (the control proves its sums equal `Saturate`'s at every
//! pair), and the marker refuses it, because the shape selects the impl on the
//! type `Saturate` while `Sole` admits a policy by its declared value.

use arvo_format::overflow::{Overflow, Policy, Saturate};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Adaptation, DeclaredSignature};
use arvo_numeral::{DebugCheck, Int, Num, Quiet, Sole};

pub trait MonotoneSum {}

impl<S: Sole, C: DebugCheck> MonotoneSum for Num<S, C> where
    <S as DeclaredSignature>::Adaptation: Adaptation<Overflow = Saturate>
{
}

pub struct Pinned;
impl Overflow for Pinned {
    const POLICY: Policy = Policy::Saturate;
}

pub type Outside = Int<8, Adapt<TowardZero, Pinned>, Quiet>;

pub fn needs_monotone<T: MonotoneSum>(t: T) -> T {
    t
}

pub fn outside_is_marked(x: Outside) -> Outside {
    needs_monotone(x)
}
