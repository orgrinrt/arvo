#![no_std]
use arvocore::*;
// The one lossless total conversion: same exponent, wider integer part.
// It is the only relation that may be implicit.
impl<const I: u32, const F: u32, const J: u32, G: Sign, S> Widen<Fixed<J, F, G, S>>
    for Fixed<I, F, G, S>
where
    S: Store<I, F, G> + Store<J, F, G>,
{
}
pub trait Widen<T> {}
pub fn implicit(a: UFixed<13, 3, Warm>) {
    let _w: UFixed<20, 3, Warm> = widen(a); // written, lossless, total
    let _r: UFixed<8, 8, Warm> = rescale(a); // written, scale-changing
}
// what a consumer sees when they mix two that do not relate
pub fn mixed(a: UFixed<13, 3, Warm>) {
    let _bad: UFixed<8, 8, Warm> = a;
}
