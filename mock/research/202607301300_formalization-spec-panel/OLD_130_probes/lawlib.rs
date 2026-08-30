#![no_std]
#![allow(dead_code)]
include!("surface_core.rs");

/// The law is a named item, so rustc's own error names the law and prints its
/// coordinates in the order the law states them.
pub struct MulLaw<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
>;
impl<const I: u32, const F: u32, const J: u32, const K: u32, const M: u32, const N: u32>
    MulLaw<I, F, J, K, M, N>
{
    pub const HOLDS: () = assert!(
        M == I + J && N == F + K,
        "arvo: the product's format does not follow from its inputs.
  The law is: UFixed<I, F> * UFixed<J, K> has format UFixed<I + J, F + K>.
  The failing instantiation is printed above as MulLaw::<I, F, J, K, M, N>.
  If you wrote this call, name the output UFixed<I + J, F + K, ..>.
  If this fired inside a function you did not write, that function's signature
  claims a format relation that does not hold for any input, and the note below
  names the function and its line. It is that function that is wrong, not yours."
    );
}

pub fn mul2<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    a: UFixed<I, F, C, S>,
    b: UFixed<J, K, C, S>,
) -> UFixed<M, N, D, S> {
    let () = MulLaw::<I, F, J, K, M, N>::HOLDS;
    let _ = (a, b);
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}

pub fn square_wrong<const I: u32, const F: u32, C: Container, S: Policy + Lowering>(
    x: UFixed<I, F, C, S>,
) -> UFixed<I, F, C, S> {
    mul2::<I, F, I, F, I, F, C, C, S>(x, x)
}
