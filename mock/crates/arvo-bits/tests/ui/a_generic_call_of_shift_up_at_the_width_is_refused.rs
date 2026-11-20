// `shift_up` at the width, called at run time through a function generic over `BitSequence`, is refused at build.

use arvo_bits::{BitSequence, Bits};

fn at_the_width<B: BitSequence>(b: B) {
    let _ = b.shift_up::<8>();
}

fn main() {
    at_the_width(Bits::<8>::masked(1));
}
