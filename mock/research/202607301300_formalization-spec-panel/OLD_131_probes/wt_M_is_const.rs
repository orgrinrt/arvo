#![no_std]
use arvocore::*;
pub fn w<const I: u32, const J: u32, const F: u32, G: Sign, S>(
    a: Fixed<I, F, G, S>,
    b: Fixed<J, F, G, S>,
) -> Fixed<6, F, G, S>
where
    S: Store<I, F, G> + Store<J, F, G> + Store<6, F, G>,
{
    add::<I, J, F, 6, G, S>(a, b)
}
// the two witnesses: I>J with max 5, and J>I with max 9
pub mod witnesses {
    use super::*;
    pub fn s_a(a: UFixed<5, 2, Warm>, b: UFixed<2, 2, Warm>) -> UFixed<6, 2, Warm> {
        w(a, b)
    }
    pub fn s_b(a: UFixed<3, 2, Warm>, b: UFixed<9, 2, Warm>) -> UFixed<10, 2, Warm> {
        w(a, b)
    }
}
