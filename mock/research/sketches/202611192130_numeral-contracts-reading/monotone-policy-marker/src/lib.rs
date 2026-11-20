//! The construction this reading proposes in place of the shape's selection:
//! the property belongs to the completion, so the marker is a trait on the
//! overflow policy, and the numeral's marker follows from it. A policy of
//! another crate opts in by implementing it, and the generic law suite is what
//! checks that it told the truth. Both the shipped `Saturate` and the outside
//! `Pinned` reach the marker here; `monotone-policy-marker-wrap` is the twin
//! where `Wrap` must not.

use arvo_format::overflow::{Overflow, Policy, Saturate};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Adaptation, DeclaredSignature};
use arvo_numeral::{DebugCheck, Int, Num, Quiet, Sole};

/// A completion under which `a <= b` implies `a + c <= b + c`.
pub trait MonotoneCompletion: Overflow {}
impl MonotoneCompletion for Saturate {}

pub trait MonotoneSum {}
impl<S: Sole, C: DebugCheck> MonotoneSum for Num<S, C> where
    <<S as DeclaredSignature>::Adaptation as Adaptation>::Overflow: MonotoneCompletion
{
}

pub struct Pinned;
impl Overflow for Pinned {
    const POLICY: Policy = Policy::Saturate;
}
impl MonotoneCompletion for Pinned {}

pub type Shipped = Int<8, Adapt<TowardZero, Saturate>, Quiet>;
pub type Outside = Int<8, Adapt<TowardZero, Pinned>, Quiet>;

pub fn needs_monotone<T: MonotoneSum>(t: T) -> T {
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_saturating_policies_reach_the_marker() {
        let a = needs_monotone(Shipped::constant::<100>());
        let b = needs_monotone(Outside::constant::<100>());
        assert_eq!(a.add(a).slot().index(), 127);
        assert_eq!(b.add(b).slot().index(), 127);
    }
}
