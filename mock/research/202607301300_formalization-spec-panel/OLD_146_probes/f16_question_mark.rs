//! Is the by-reference `From` reachable through `?`?
//! `?` desugars to `From::from` applied to the value, so a by-reference impl should
//! not be found. `145:663` uses `?` reachability as an argument about which
//! conversions read as free, so the answer bears on that argument.
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub struct Unsigned;
pub struct Warm;
pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}
impl<const I: u32, const F: u32, G, S> core::fmt::Debug for Fixed<I, F, G, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Fixed")
    }
}
pub const fn tag_embeds(i1: u32, f1: u32, i2: u32, f2: u32) -> usize {
    if i1 <= i2 && f1 <= f2 {
        0
    } else {
        1
    }
}
pub struct Picker;
pub struct Pair<const I1: u32, const F1: u32, const I2: u32, const F2: u32>;
pub trait Tagged {
    type const TAG: usize;
}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> Tagged for Pair<I1, F1, I2, F2> {
    type const TAG: usize = const { tag_embeds(I1, F1, I2, F2) };
}
pub trait EmbedWitness<const TAG: usize> {}
impl EmbedWitness<0> for Picker {}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<&Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
{
    fn from(_: &Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}

fn inner() -> Result<(), UFixed<13, 3, Warm>> {
    Err(Fixed(PhantomData))
}
pub fn via_question() -> Result<(), UFixed<20, 8, Warm>> {
    inner()?;
    Ok(())
}
