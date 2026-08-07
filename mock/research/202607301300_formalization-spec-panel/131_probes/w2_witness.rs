#![no_std]
use arvocore::*;
// A generic wrapper claiming the product of two P-shaped numerals is P-shaped.
// Wrong for every non-degenerate instantiation. Uninstantiated, it compiles.
pub fn square_wrong<const I: u32, const F: u32, G: Sign, S>(
    x: Fixed<I, F, G, S>,
) -> Fixed<I, F, G, S>
where
    S: Store<I, F, G>,
{
    mul::<I, F, I, F, I, F, G, S>(x, x)
}

// A wrapper claiming a constant output format.
pub fn square_lit<const I: u32, const F: u32, G: Sign, S>(
    x: Fixed<I, F, G, S>,
) -> Fixed<26, 6, G, S>
where
    S: Store<I, F, G> + Store<26, 6, G>,
{
    mul::<I, F, I, F, 26, 6, G, S>(x, x)
}

/// Two witnesses discharge the product law: any wrapper claiming a literal
/// output is refused by a pair differing in I, and by a pair differing in F.
#[doc(hidden)]
pub mod witnesses {
    use super::*;
    pub fn p_a(x: UFixed<13, 3, Warm>) -> UFixed<26, 6, Warm> {
        square_lit(x)
    }
    pub fn p_b(x: UFixed<7, 2, Warm>) -> UFixed<26, 6, Warm> {
        square_lit(x)
    }
}
