#![feature(adt_const_params)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, std::marker::ConstParamTy)]
#[repr(transparent)]
pub struct Width(u32);
impl Width {
    pub const fn bits(n: u32) -> Self {
        Self(n)
    }
}
pub struct Signed<const BITS: Width>;
fn main() {}
