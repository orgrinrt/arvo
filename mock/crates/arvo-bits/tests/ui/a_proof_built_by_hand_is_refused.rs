// A `Within` built by hand, to read past the width without `Within::NEW`, does
// not build: its field is private.

use core::marker::PhantomData;

use arvo_bits::{BitAccessAt, Bits, Within};

fn main() {
    let _ = Bits::<8>::masked(1).bit_at::<9>(Within(PhantomData));
}
