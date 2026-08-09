#![no_std]
// Does a defaulted TYPE parameter accept a projection whose const arguments
// are standalone parameters of the same item? If so, surface arity and the
// projection mechanism are independent questions.
pub trait Store<const I: usize, const F: usize> {
    type T: Copy;
}
pub struct Hot;
impl Store<3, 0> for Hot {
    type T = u8;
}
impl Store<13, 3> for Hot {
    type T = u16;
}

pub struct Fixed<const I: usize, const F: usize, S, C = <S as Store<I, F>>::T> {
    raw: C,
    _m: core::marker::PhantomData<S>,
}
pub type UFixed<const I: usize, const F: usize, S> = Fixed<I, F, S>;

pub fn three_params(_: UFixed<13, 3, Hot>) {}
const _: () = {
    let _: <Hot as Store<13, 3>>::T = 0u16;
};
