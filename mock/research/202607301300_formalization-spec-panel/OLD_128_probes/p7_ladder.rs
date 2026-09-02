//! P7: the hardware ladder. Width -> container, blanket impls, no per-width rows.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

pub trait Container {
    type Repr;
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
impl Container for Slot<128> {
    type Repr = u128;
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
        } else if W <= 64 {
            64
        } else {
            128
        }
    };
}
pub type ReprOf<const W: u16> = <Slot<{ <L as Ladder<W>>::RUNG }> as Container>::Repr;

// three blanket uses over ALL widths, zero per-width rows
pub fn store<const W: u16>(v: ReprOf<W>) -> ReprOf<W>
where
    Slot<{ <L as Ladder<W>>::RUNG }>: Container,
{
    v
}
pub fn check() {
    let _: ReprOf<3> = 0u8;
    let _: ReprOf<13> = 0u16;
    let _: ReprOf<47> = 0u64;
    let _: ReprOf<64> = 0u64;
    let _: ReprOf<100> = 0u128;
}
