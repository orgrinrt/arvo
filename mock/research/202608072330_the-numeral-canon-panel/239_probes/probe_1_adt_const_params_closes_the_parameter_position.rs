#![feature(adt_const_params)]

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, std::marker::ConstParamTy)]
#[repr(transparent)]
pub struct Width(u32);

impl Width {
    pub const fn bits(n: u32) -> Self {
        Self(n)
    }
    pub const fn count(self) -> u32 {
        self.0
    }
}

pub struct Signed<const BITS: Width>;

impl<const BITS: Width> Signed<BITS> {
    pub const MIN: i64 = -(1i64 << (BITS.count() - 1));
}

fn main() {
    // external-crate-style construction: no direct field access, only via ::bits
    let _s: Signed<{ Width::bits(8) }> = Signed;
    assert_eq!(Signed::<{ Width::bits(8) }>::MIN, -128);
    println!("ok");
}
