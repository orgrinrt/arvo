// `bit` at the width, called at run time through a function generic over `BitAccess`, is refused at build.

use arvo_bits::{BitAccess, Bits};

fn at_the_width<B: BitAccess>(b: B) {
    let _ = b.bit::<8>();
}

fn main() {
    at_the_width(Bits::<8>::masked(1));
}
