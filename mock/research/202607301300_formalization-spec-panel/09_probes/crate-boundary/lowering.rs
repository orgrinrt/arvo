//! crate `lowering`: minimal stand-in for arvo-lowering (D72's crate table).
#![crate_type = "rlib"]
#![crate_name = "lowering"]

pub trait StorageLayout {}

pub struct Dense;
impl StorageLayout for Dense {}

pub struct Bitpacked;
impl StorageLayout for Bitpacked {}

pub trait Lowering {
    type Layout: StorageLayout;
}

pub struct MinWidth;
impl Lowering for MinWidth {
    type Layout = Dense;
}

pub struct DoubleWidth;
impl Lowering for DoubleWidth {
    type Layout = Bitpacked;
}
