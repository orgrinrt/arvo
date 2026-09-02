// The name the workspace register carries a row for, in its Allowed tier.
#![feature(adt_const_params)]

#[repr(transparent)]
#[derive(PartialEq, Eq, std::marker::ConstParamTy)]
pub struct Width(u32);

pub struct Signed<const BITS: Width>;

fn main() {}
