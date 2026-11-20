// Two containers related by subtyping, one 128 bits wide and one 8: a proof of
// index 100 made for the wide one, bound as a proof for the narrow one, is a
// type mismatch, since the proof is invariant in its container.

use core::marker::PhantomData;

use arvo_bits::{BitWidth, Within};
use arvo_format::Width;

struct Wrap<T>(PhantomData<T>);

impl BitWidth for Wrap<for<'a> fn(&'a ())> {
    const WIDTH: Width = Width::bits(128);
}

#[allow(coherence_leak_check)]
impl BitWidth for Wrap<fn(&'static ())> {
    const WIDTH: Width = Width::bits(8);
}

fn main() {
    let wide: Within<Wrap<for<'a> fn(&'a ())>, 100> = Within::NEW;
    let _narrow: Within<Wrap<fn(&'static ())>, 100> = wide;
}
