#![allow(dead_code)]
pub trait Nat {
    const VAL: u64;
}
pub struct Z;
impl Nat for Z {
    const VAL: u64 = 0;
}
pub struct Idx<const N: u16>;
pub trait NatIndex {
    type Out: Nat;
}
impl NatIndex for Idx<0> {
    type Out = Z;
}
