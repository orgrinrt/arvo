#![no_std]
#![allow(dead_code)]
pub trait Carrier {
    type Store;
}
pub trait Lowering: Carrier {
    type Layout;
} // SAFE direction: Lowering: Carrier
pub trait NeedsLayout {}
// Bounding on the LOWER trait must not reach the upper one's member.
pub fn probe<S: Carrier>()
where
    S::Layout: NeedsLayout,
{
}
