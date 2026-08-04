// Does min_generic_const_args admit the BitsContainerFor shape once the const
// expression sits inside a const block, as its own error message suggests?
#![feature(min_generic_const_args)]
#![crate_type = "lib"]

pub const fn tag(n: u16) -> u8 {
    if n <= 64 {
        0
    } else {
        1
    }
}

pub trait Project<const TAG: u8> {
    type T;
}
pub struct Picker;
impl Project<0> for Picker {
    type T = u64;
}
impl Project<1> for Picker {
    type T = u128;
}

pub trait ContainerFor<const N: u16> {
    type T;
}
pub struct Hot;
impl<const N: u16> ContainerFor<N> for Hot
where
    Picker: Project<{ const { tag(N) } }>,
{
    type T = <Picker as Project<{ const { tag(N) } }>>::T;
}
