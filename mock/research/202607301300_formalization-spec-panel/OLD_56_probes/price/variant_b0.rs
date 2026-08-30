#![feature(adt_const_params)]
#![allow(dead_code)]
use core::marker::ConstParamTy;
use core::marker::PhantomData;
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Spec {
    pub precision: u16,
    pub bias_num: u64,
    pub bias_den: u64,
}
impl Spec {
    const fn is_reduced(&self) -> bool {
        gcd(self.bias_num, self.bias_den) == 1
    }
}
pub struct NFace<const S: Spec>(PhantomData<()>);
impl<const S: Spec> NFace<S> {
    pub const REDUCED: () = assert!(S.is_reduced());
    pub const fn checked() -> Self {
        let () = Self::REDUCED;
        NFace(PhantomData)
    }
}
