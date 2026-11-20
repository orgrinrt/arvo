//! The control for `monotone-outside-policy`: the marker as the shape selects
//! it, by the overflow type being `Saturate`, and an outside policy whose
//! declared `POLICY` is `Saturate`. This crate shows that the value admits the
//! outside policy, that its sums are exactly the shipped policy's sums over
//! every pair at eight bits, and that the marker reaches the shipped one.

use arvo_format::overflow::{Overflow, Policy, Saturate};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Adaptation, DeclaredSignature};
#[cfg(test)]
use arvo_format::{Exact, Slot};
use arvo_numeral::{DebugCheck, Int, Num, Quiet, Sole};

/// The marker as the shape writes it: implemented only where the signature's
/// adaptation names `Saturate` as its overflow.
pub trait MonotoneSum {}

impl<S: Sole, C: DebugCheck> MonotoneSum for Num<S, C> where
    <S as DeclaredSignature>::Adaptation: Adaptation<Overflow = Saturate>
{
}

/// A policy of another crate, which `arvo_format::overflow::Overflow` is open
/// to and `Sole` holds on the same terms, by its declared `POLICY`.
pub struct Pinned;
impl Overflow for Pinned {
    const POLICY: Policy = Policy::Saturate;
}

pub type Shipped = Int<8, Adapt<TowardZero, Saturate>, Quiet>;
pub type Outside = Int<8, Adapt<TowardZero, Pinned>, Quiet>;

pub fn needs_monotone<T: MonotoneSum>(t: T) -> T {
    t
}

pub fn shipped_is_marked(x: Shipped) -> Shipped {
    needs_monotone(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_outside_policy_sums_exactly_as_saturate_does() {
        let mut pairs = 0u32;
        for a in -128i128 ..= 127 {
            for b in -128i128 ..= 127 {
                let s = Shipped::adapted(Exact::on_grid(Slot::at(a)))
                    .add(Shipped::adapted(Exact::on_grid(Slot::at(b))));
                let o = Outside::adapted(Exact::on_grid(Slot::at(a)))
                    .add(Outside::adapted(Exact::on_grid(Slot::at(b))));
                assert_eq!(s.slot().index(), o.slot().index(), "a={a} b={b}");
                pairs += 1;
            }
        }
        assert_eq!(pairs, 65536);
        // A sum that saturates, so the agreement is not only over in-range sums.
        let top = Outside::adapted(Exact::on_grid(Slot::at(100)));
        assert_eq!(top.add(top).slot().index(), 127);
    }
}
