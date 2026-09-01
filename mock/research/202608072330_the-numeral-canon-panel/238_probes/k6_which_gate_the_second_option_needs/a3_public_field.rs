// The same, with the field made public, which is what the diagnostic on `a1`
// asks for. It answers whether the gate alone is the cost or whether the door
// type's encapsulation is part of it.
#![feature(min_adt_const_params)]

#[repr(transparent)]
#[derive(PartialEq, Eq, std::marker::ConstParamTy)]
pub struct Width(pub u32);

pub struct Signed<const BITS: Width>;

pub const EIGHT: Signed<{ Width(8) }> = Signed;

fn main() {
    let _ = EIGHT;
}
