//! Must fail: the policy-side marker of `monotone-policy-marker`, and a
//! wrapping value handed to a function bounding on the numeral's marker.

use arvo_format::overflow::{Overflow, Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Adaptation, DeclaredSignature};
use arvo_numeral::{DebugCheck, Int, Num, Quiet, Sole};

pub trait MonotoneCompletion: Overflow {}
impl MonotoneCompletion for Saturate {}

pub trait MonotoneSum {}
impl<S: Sole, C: DebugCheck> MonotoneSum for Num<S, C> where
    <<S as DeclaredSignature>::Adaptation as Adaptation>::Overflow: MonotoneCompletion
{
}

pub type Wrapping = Int<8, Adapt<TowardZero, Wrap>, Quiet>;

pub fn needs_monotone<T: MonotoneSum>(t: T) -> T {
    t
}

pub fn wrapping_is_marked(x: Wrapping) -> Wrapping {
    needs_monotone(x)
}
