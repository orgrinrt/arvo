// The name the compiler's own help text on this pin tells you to add.
#![feature(min_adt_const_params)]

#[repr(transparent)]
#[derive(PartialEq, Eq, std::marker::ConstParamTy)]
pub struct Width(u32);

pub struct Signed<const BITS: Width>;

fn main() {}
