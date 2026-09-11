//! p5a: a capacity as a const generic parameter, spelled with the host's usize.
//! This is the one position op's exception admits a bare primitive at.
#![no_std]
pub struct Buf<const N: usize>([u8; N]);
pub type Four = Buf<4>;
