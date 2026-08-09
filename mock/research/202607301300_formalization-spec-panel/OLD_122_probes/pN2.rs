#![no_std]
#![allow(dead_code)]
pub trait Lowering {
    type Layout;
}
pub trait Carrier: Lowering {
    type Store;
} // UNSAFE direction
pub trait NeedsLayout {}
pub fn probe<S: Carrier>()
where
    S::Layout: NeedsLayout,
{
}
