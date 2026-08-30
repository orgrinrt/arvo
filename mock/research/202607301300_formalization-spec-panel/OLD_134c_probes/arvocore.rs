#![no_std]
// the bridge trait, generic over a marker the CONSUMER supplies
pub struct Idx<const N: u32>;
pub trait ToNat<M> {
    type Out;
    const VAL: u32;
}
// arvo's own marker and its own populated range
pub struct ArvoWidths;
pub struct Nat<const V: u32>;
impl ToNat<ArvoWidths> for Idx<8> {
    type Out = Nat<8>;
    const VAL: u32 = 8;
}
impl ToNat<ArvoWidths> for Idx<16> {
    type Out = Nat<16>;
    const VAL: u32 = 16;
}
