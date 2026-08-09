//! P11: does GCA work in a GENERIC function body, the GCE ICE-prone zone?
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub trait Container {
    type Repr: Copy + Default;
}
pub struct Slot<const B: u16>;
impl Container for Slot<8> {
    type Repr = u8;
}
impl Container for Slot<16> {
    type Repr = u16;
}
impl Container for Slot<32> {
    type Repr = u32;
}
impl Container for Slot<64> {
    type Repr = u64;
}
pub trait Ladder<const W: u16> {
    type const RUNG: u16;
}
pub struct L;
impl<const W: u16> Ladder<W> for L {
    type const RUNG: u16 = const {
        if W <= 8 {
            8
        } else if W <= 16 {
            16
        } else if W <= 32 {
            32
        } else {
            64
        }
    };
}
pub type ReprOf<const W: u16> = <Slot<{ <L as Ladder<W>>::RUNG }> as Container>::Repr;

// generic over ALL widths, no per-width row, carrying the bound
pub fn zero<const W: u16>() -> ReprOf<W>
where
    Slot<{ <L as Ladder<W>>::RUNG }>: Container,
{
    Default::default()
}
pub fn call() {
    let _a: u8 = zero::<3>();
    let _b: u64 = zero::<47>();
}
