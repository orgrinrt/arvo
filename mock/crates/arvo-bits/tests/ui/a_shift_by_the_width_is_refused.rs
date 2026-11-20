// A shift by the whole width is no move within it, and is refused at build
// rather than defined as the empty value.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).shift_up::<8>();
};

fn main() {}
